use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use tl_release_gate::{
    check, compare_legacy_goldens, BuildFact, Candidate, CandidateFact, CandidateInput,
    CandidateSet, WireFact,
};

fn git_bytes(path: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .map_err(|error| format!("git invocation at {}: {error}", path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git at {} {:?}: {}",
            path.display(),
            args,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output.stdout)
}

fn git(path: &Path, args: &[&str]) -> Result<String, String> {
    String::from_utf8(git_bytes(path, args)?)
        .map(|text| text.trim().to_owned())
        .map_err(|error| error.to_string())
}

const GOLDEN_PATHS: [&str; 6] = [
    "corpus/schema",
    "corpus/formulas",
    "corpus/propositions.json",
    "corpus/malformed",
    "corpus/future-operators/expected",
    "corpus/past-history/cases.json",
];

fn golden_files(
    candidate: &Candidate,
    revision: &str,
) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut args = vec!["ls-tree", "-r", "--name-only", revision, "--"];
    args.extend(GOLDEN_PATHS);
    let listing = git(&candidate.path, &args)?;
    let mut files = BTreeMap::new();
    for path in listing.lines() {
        files.insert(
            path.to_owned(),
            git_bytes(&candidate.path, &["show", &format!("{revision}:{path}")])?,
        );
    }
    Ok(files)
}

fn tracked(candidate: &Candidate, file: &str) -> Result<String, String> {
    git(
        &candidate.path,
        &["show", &format!("{}:{file}", candidate.commit)],
    )
}

fn observe(candidate: &Candidate) -> Result<CandidateInput, String> {
    if !tl_release_gate::valid_sha(&candidate.commit) {
        return Err(format!(
            "{}: commit must be a full 40-character SHA-1",
            candidate.name
        ));
    }
    let manifest: toml::Value = tracked(candidate, "Cargo.toml")?
        .parse()
        .map_err(|error| format!("{}: Cargo.toml: {error}", candidate.name))?;
    let lockfile: toml::Value = tracked(candidate, "Cargo.lock")?
        .parse()
        .map_err(|error| format!("{}: Cargo.lock: {error}", candidate.name))?;
    let toolchain: toml::Value = tracked(candidate, "rust-toolchain.toml")?
        .parse()
        .map_err(|error| format!("{}: rust-toolchain.toml: {error}", candidate.name))?;
    let package = manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("{}: missing package table", candidate.name))?;
    let get = |field| {
        package
            .get(field)
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| format!("{}: missing package.{field}", candidate.name))
    };
    let tag_commit = candidate.proposed_tag.as_ref().and_then(|tag| {
        git(
            &candidate.path,
            &[
                "rev-parse",
                "--verify",
                &format!("refs/tags/{tag}^{{commit}}"),
            ],
        )
        .ok()
    });
    Ok(CandidateInput {
        fact: CandidateFact {
            name: candidate.name.clone(),
            commit: candidate.commit.clone(),
            version: get("version")?,
            msrv: get("rust-version")?,
            proposed_tag: candidate.proposed_tag.clone(),
            tag_commit,
        },
        manifest,
        lockfile,
        toolchain,
    })
}

fn rustup_run(path: &Path, msrv: &str, cargo_args: &[&str]) -> Result<(), String> {
    let output = Command::new("rustup")
        .args(["run", msrv, "cargo"])
        .args(cargo_args)
        .current_dir(path)
        .output()
        .map_err(|error| format!("rustup invocation at {}: {error}", path.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{}: MSRV cargo {:?} failed: {}",
            path.display(),
            cargo_args,
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run(path: &Path) -> Result<bool, String> {
    let raw = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let set: CandidateSet =
        serde_json::from_str(&raw).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut inputs = Vec::new();
    for candidate in &set.candidates {
        inputs.push(observe(candidate)?);
    }
    let mut report = check(&set, &inputs);
    if let Some(syntax) = set
        .candidates
        .iter()
        .find(|candidate| candidate.name == "tl-syntax")
    {
        match (
            golden_files(syntax, &syntax.previous_tag),
            golden_files(syntax, &syntax.commit),
        ) {
            (Ok(previous), Ok(candidate)) => {
                let differences = compare_legacy_goldens(&previous, &candidate);
                for difference in &differences {
                    report
                        .failures
                        .push(format!("tl-syntax wire golden: {difference}"));
                }
                report.syntax_wire_golden = Some(WireFact {
                    previous_tag: syntax.previous_tag.clone(),
                    checked_files: previous.len(),
                    differences,
                });
            }
            (Err(error), _) | (_, Err(error)) => {
                report
                    .failures
                    .push(format!("tl-syntax wire golden unavailable: {error}"));
            }
        }
    }
    for lane in &set.historical_lanes {
        let Some(target) = set
            .candidates
            .iter()
            .find(|candidate| candidate.name == lane.package)
        else {
            report.failures.push(format!(
                "{}:{}: historical package has no candidate repository",
                lane.owner, lane.alias
            ));
            continue;
        };
        if !tl_release_gate::valid_sha(&lane.revision) {
            report.failures.push(format!(
                "{}:{}: historical revision is not a full SHA-1",
                lane.owner, lane.alias
            ));
            continue;
        }
        match git(
            &target.path,
            &["show", &format!("{}:Cargo.toml", lane.revision)],
        ) {
            Ok(raw) => {
                match raw.parse::<toml::Value>() {
                    Ok(manifest) => {
                        let package = manifest.get("package");
                        let name = package
                            .and_then(|value| value.get("name"))
                            .and_then(toml::Value::as_str);
                        let version = package
                            .and_then(|value| value.get("version"))
                            .and_then(toml::Value::as_str);
                        if name != Some(lane.package.as_str())
                            || version != Some(lane.version.as_str())
                        {
                            report.failures.push(format!("{}:{}: historical revision package/version differs from declaration", lane.owner, lane.alias));
                        }
                    }
                    Err(error) => report.failures.push(format!(
                        "{}:{}: historical manifest is invalid: {error}",
                        lane.owner, lane.alias
                    )),
                }
            }
            Err(error) => report
                .failures
                .push(format!("{}:{}: {error}", lane.owner, lane.alias)),
        }
    }
    if report.accepted {
        for candidate in &set.candidates {
            match git(&candidate.path, &["rev-parse", "HEAD"]) {
                Ok(head) if head == candidate.commit => {}
                Ok(head) => report.failures.push(format!(
                    "{}: worktree HEAD {head} differs from candidate {}",
                    candidate.name, candidate.commit
                )),
                Err(error) => report.failures.push(error),
            }
            match git(&candidate.path, &["status", "--porcelain=v1", "--untracked-files=all"]) {
                Ok(status) if status.is_empty() => {}
                Ok(_) => report.failures.push(format!(
                    "{}: candidate worktree is dirty; MSRV build would not use exact committed source",
                    candidate.name
                )),
                Err(error) => report.failures.push(error),
            }
        }
    }
    if report.failures.is_empty() {
        let msrv = &report.candidates[0].msrv;
        for candidate in &set.candidates {
            match rustup_run(
                &candidate.path,
                msrv,
                &["test", "--locked", "--all-targets", "--all-features"],
            ) {
                Ok(()) => report.msrv_builds.push(BuildFact {
                    name: candidate.name.clone(),
                    result: "passed".to_owned(),
                }),
                Err(error) => {
                    report.msrv_builds.push(BuildFact {
                        name: candidate.name.clone(),
                        result: "failed".to_owned(),
                    });
                    report.failures.push(error);
                }
            }
        }
        for lane in &set.historical_lanes {
            if let Some(owner) = set
                .candidates
                .iter()
                .find(|candidate| candidate.name == lane.owner)
            {
                if let Err(error) = rustup_run(
                    &owner.path,
                    msrv,
                    &["test", "--locked", "--test", &lane.test_target],
                ) {
                    report.failures.push(error);
                }
            }
        }
    } else {
        for candidate in &set.candidates {
            report.msrv_builds.push(BuildFact {
                name: candidate.name.clone(),
                result: "not-run".to_owned(),
            });
        }
    }
    report.failures.sort();
    report.accepted = report.failures.is_empty();
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
    );
    Ok(report.accepted)
}

fn main() -> ExitCode {
    let mut args = std::env::args_os();
    let _program = args.next();
    let Some(path) = args.next() else {
        eprintln!("usage: tl-release-gate <candidate-set.json>");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("usage: tl-release-gate <candidate-set.json>");
        return ExitCode::from(2);
    }
    match run(Path::new(&path)) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
