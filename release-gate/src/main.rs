use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use tl_release_gate::{
    check, compare_legacy_goldens, consumer_lock_differences, consumer_manifest,
    duplicated_owner_blobs, test_target_has_executed_cases, BuildFact, Candidate, CandidateFact,
    CandidateInput, CandidateSet, CorpusLaneFact, CorpusOwnershipFact, CrateWireFact,
    LegacyDecoderFact, SmokeFact, WireFact,
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

const CORPUS_LANES: [(&str, &str); 3] = [
    ("tl-parse", "owner_infinite_corpus"),
    ("tl-mltl", "infinite_corpus"),
    ("tl-rewrite", "infinite_owner_corpus"),
];

fn golden_files(
    candidate: &Candidate,
    revision: &str,
) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut args = vec!["ls-tree", "-r", "--name-only", revision, "--"];
    if candidate.name == "tl-syntax" {
        args.extend(GOLDEN_PATHS);
    } else {
        args.push("corpus");
        if candidate.name == "tl-mltl" {
            args.push("schemas");
        }
    }
    let listing = git(&candidate.path, &args)?;
    let mut files = BTreeMap::new();
    for path in listing.lines() {
        if path.ends_with(".md") {
            continue;
        }
        files.insert(
            path.to_owned(),
            git_bytes(&candidate.path, &["show", &format!("{revision}:{path}")])?,
        );
    }
    Ok(files)
}

fn tracked_blobs(
    candidate: &Candidate,
    revision: &str,
) -> Result<BTreeMap<String, String>, String> {
    let listing = git(&candidate.path, &["ls-tree", "-r", revision])?;
    let mut blobs = BTreeMap::new();
    for row in listing.lines() {
        let (metadata, path) = row
            .split_once('\t')
            .ok_or_else(|| format!("{}: malformed git tree row", candidate.name))?;
        let mut parts = metadata.split_ascii_whitespace();
        let _mode = parts.next();
        if parts.next() == Some("blob") {
            let oid = parts
                .next()
                .ok_or_else(|| format!("{}: missing blob identity", candidate.name))?;
            blobs.insert(path.to_owned(), oid.to_owned());
        }
    }
    Ok(blobs)
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

fn toolchain_binary(toolchain: &str, binary: &str) -> Result<String, String> {
    let output = Command::new("rustup")
        .args(["which", binary, "--toolchain", toolchain])
        .output()
        .map_err(|error| format!("cannot resolve {toolchain} {binary}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cannot resolve {toolchain} {binary}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|path| path.trim().to_owned())
        .map_err(|error| error.to_string())
}

fn rustup_output(path: &Path, toolchain: &str, cargo_args: &[&str]) -> Result<String, String> {
    let cargo = toolchain_binary(toolchain, "cargo")?;
    let rustc = toolchain_binary(toolchain, "rustc")?;
    let output = Command::new(cargo)
        .env("RUSTC", rustc)
        .args(cargo_args)
        .current_dir(path)
        .output()
        .map_err(|error| format!("rustup invocation at {}: {error}", path.display()))?;
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|error| error.to_string())
    } else {
        Err(format!(
            "{}: {toolchain} cargo {:?} failed: {}",
            path.display(),
            cargo_args,
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn rustup_run(path: &Path, toolchain: &str, cargo_args: &[&str]) -> Result<(), String> {
    rustup_output(path, toolchain, cargo_args).map(|_| ())
}

fn run_consumer_smoke(candidates: &[CandidateFact], msrv: &str) -> Result<SmokeFact, String> {
    let manifest = consumer_manifest(candidates)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let project =
        std::env::temp_dir().join(format!("tl-release-smoke-{}-{nonce}", std::process::id()));
    fs::create_dir(&project).map_err(|error| format!("{}: {error}", project.display()))?;
    fs::create_dir(project.join("src"))
        .map_err(|error| format!("{}: {error}", project.display()))?;
    fs::write(project.join("Cargo.toml"), manifest)
        .map_err(|error| format!("{}: {error}", project.display()))?;
    fs::write(project.join("src/main.rs"), include_str!("../smoke.rs"))
        .map_err(|error| format!("{}: {error}", project.display()))?;
    rustup_run(&project, msrv, &["generate-lockfile"])?;
    let lock_text = fs::read_to_string(project.join("Cargo.lock"))
        .map_err(|error| format!("{}: {error}", project.display()))?;
    let lock: toml::Value = lock_text.parse().map_err(|error| {
        format!(
            "{}: invalid external Cargo.lock: {error}",
            project.display()
        )
    })?;
    let differences = consumer_lock_differences(&lock, candidates);
    if !differences.is_empty() {
        return Err(format!(
            "external consumer lock: {}",
            differences.join("; ")
        ));
    }
    rustup_run(&project, msrv, &["run", "--locked", "--quiet"])?;
    rustup_run(&project, "stable", &["run", "--locked", "--quiet"])?;
    Ok(SmokeFact {
        candidate_revisions: candidates
            .iter()
            .map(|candidate| candidate.commit.clone())
            .collect(),
        lock_result: "passed".to_owned(),
        msrv_result: "passed".to_owned(),
        stable_result: "passed".to_owned(),
    })
}

fn run_legacy_decoder_replay(set: &CandidateSet, msrv: &str) -> Result<LegacyDecoderFact, String> {
    let mut previous = Vec::new();
    for candidate in &set.candidates {
        let revision = git(
            &candidate.path,
            &[
                "rev-parse",
                "--verify",
                &format!("refs/tags/{}^{{commit}}", candidate.previous_tag),
            ],
        )?;
        let prior = Candidate {
            commit: revision,
            proposed_tag: None,
            ..candidate.clone()
        };
        previous.push(observe(&prior)?.fact);
    }
    let current: Vec<_> = set
        .candidates
        .iter()
        .map(observe)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|input| input.fact)
        .collect();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("tl-legacy-replay-{}-{nonce}", std::process::id()));
    let fixtures = root.join("fixtures");
    fs::create_dir_all(&fixtures).map_err(|error| format!("{}: {error}", fixtures.display()))?;
    for (name, facts, source) in [
        ("baseline", &previous, include_str!("../legacy-baseline.rs")),
        (
            "candidate",
            &current,
            include_str!("../legacy-candidate.rs"),
        ),
    ] {
        let project = root.join(name);
        fs::create_dir_all(project.join("src"))
            .map_err(|error| format!("{}: {error}", project.display()))?;
        fs::write(
            project.join("Cargo.toml"),
            format!("{}serde_json = \"1\"\n", consumer_manifest(facts)?),
        )
        .map_err(|error| format!("{}: {error}", project.display()))?;
        fs::write(project.join("src/main.rs"), source)
            .map_err(|error| format!("{}: {error}", project.display()))?;
        rustup_run(&project, msrv, &["generate-lockfile"])?;
        let lock = fs::read_to_string(project.join("Cargo.lock"))
            .map_err(|error| format!("{}: {error}", project.display()))?;
        let lock: toml::Value = lock
            .parse()
            .map_err(|error| format!("{}: {error}", project.display()))?;
        let differences = consumer_lock_differences(&lock, facts);
        if !differences.is_empty() {
            return Err(format!("{name} legacy lock: {}", differences.join("; ")));
        }
        let cargo = toolchain_binary(msrv, "cargo")?;
        let rustc = toolchain_binary(msrv, "rustc")?;
        let output = Command::new(cargo)
            .env("RUSTC", rustc)
            .env("TL_LEGACY_DIR", &fixtures)
            .args(["run", "--locked", "--quiet"])
            .current_dir(&project)
            .output()
            .map_err(|error| format!("{name} legacy replay: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "{name} legacy replay failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(LegacyDecoderFact {
        previous_revisions: previous.into_iter().map(|fact| fact.commit).collect(),
        candidate_revisions: current.into_iter().map(|fact| fact.commit).collect(),
        result: "passed".to_owned(),
    })
}

fn breaking_version_advanced(previous: &str, current: &str) -> bool {
    let parse = |text: &str| -> Option<(u64, u64, u64)> {
        let mut parts = text.split('.').map(str::parse::<u64>);
        let value = (
            parts.next()?.ok()?,
            parts.next()?.ok()?,
            parts.next()?.ok()?,
        );
        parts.next().is_none().then_some(value)
    };
    let (Some(old), Some(new)) = (parse(previous), parse(current)) else {
        return false;
    };
    if old.0 == 0 {
        new.0 > 0 || (new.0 == 0 && new.1 > old.1)
    } else {
        new.0 > old.0
    }
}

fn run_api_compatibility(
    candidate: &Candidate,
    fact: &CandidateFact,
) -> Result<tl_release_gate::ApiCompatibilityFact, String> {
    let previous_manifest: toml::Value = git(
        &candidate.path,
        &["show", &format!("{}:Cargo.toml", candidate.previous_tag)],
    )?
    .parse()
    .map_err(|error| format!("{} previous Cargo.toml: {error}", candidate.name))?;
    let previous_version = previous_manifest
        .get("package")
        .and_then(|table| table.get("version"))
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("{}: previous package version absent", candidate.name))?;
    let binary =
        std::env::var_os("TL_SEMVER_CHECKS").unwrap_or_else(|| "cargo-semver-checks".into());
    let version = Command::new(&binary)
        .arg("--version")
        .output()
        .map_err(|error| {
            format!(
                "{}: cargo-semver-checks unavailable: {error}",
                candidate.name
            )
        })?;
    if !version.status.success() {
        return Err(format!(
            "{}: cargo-semver-checks --version failed",
            candidate.name
        ));
    }
    let tool_version = String::from_utf8_lossy(&version.stdout).trim().to_owned();
    if tool_version != "cargo-semver-checks 0.50.0" {
        return Err(format!(
            "{}: expected cargo-semver-checks 0.50.0, got {tool_version}",
            candidate.name
        ));
    }
    let rustc = toolchain_binary(&fact.msrv, "rustc")?;
    let tool_dir = Path::new(&rustc)
        .parent()
        .ok_or_else(|| format!("{}: rustc has no bin directory", candidate.name))?;
    let mut paths = vec![tool_dir.to_path_buf()];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(paths).map_err(|error| error.to_string())?;
    let output = Command::new(binary)
        .env("PATH", path)
        .env("RUSTC", rustc)
        .args([
            "check-release",
            "--manifest-path",
            "Cargo.toml",
            "--baseline-rev",
        ])
        .arg(&candidate.previous_tag)
        .args([
            "--release-type",
            "minor",
            "--all-features",
            "--color",
            "never",
        ])
        .current_dir(&candidate.path)
        .output()
        .map_err(|error| format!("{}: cargo-semver-checks: {error}", candidate.name))?;
    if !matches!(output.status.code(), Some(0 | 100)) {
        return Err(format!(
            "{}: cargo-semver-checks could not compare APIs (exit {:?}): {}",
            candidate.name,
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let stdout = String::from_utf8(output.stdout)
        .map_err(|error| format!("{}: semver output is not UTF-8: {error}", candidate.name))?;
    let findings = tl_release_gate::parse_semver_findings(&stdout)?;
    if output.status.code() == Some(100) && findings.is_empty() {
        return Err(format!(
            "{}: semver found breaks but emitted no parseable findings",
            candidate.name
        ));
    }
    let changelog = tracked(candidate, "CHANGELOG.md")?;
    let mut failures =
        tl_release_gate::reconcile_api_migrations(&changelog, &fact.version, &findings);
    if !findings.is_empty() && !breaking_version_advanced(previous_version, &fact.version) {
        failures.push(format!(
            "{}: API breaks require a new breaking release version after {previous_version}",
            candidate.name
        ));
    }
    if !failures.is_empty() {
        return Err(format!("{}: {}", candidate.name, failures.join("; ")));
    }
    Ok(tl_release_gate::ApiCompatibilityFact {
        name: candidate.name.clone(),
        previous_tag: candidate.previous_tag.clone(),
        tool_version,
        findings,
        result: "passed".to_owned(),
    })
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
        match tracked_blobs(syntax, &syntax.commit) {
            Ok(blobs) => {
                let owner: BTreeMap<_, _> = blobs
                    .into_iter()
                    .filter(|(path, _)| {
                        path.starts_with("corpus/infinite-trace/") && !path.ends_with("README.md")
                    })
                    .collect();
                if owner.is_empty() {
                    report
                        .failures
                        .push("tl-syntax: owner infinite corpus is absent".to_owned());
                }
                for consumer in set
                    .candidates
                    .iter()
                    .filter(|candidate| candidate.name != "tl-syntax")
                {
                    match tracked_blobs(consumer, &consumer.commit) {
                        Ok(blobs) => {
                            let duplicates = duplicated_owner_blobs(&owner, &blobs);
                            for path in &duplicates {
                                report.failures.push(format!(
                                    "{}: vendored syntax owner corpus bytes at {path}",
                                    consumer.name
                                ));
                            }
                            report.corpus_ownership.push(CorpusOwnershipFact {
                                name: consumer.name.clone(),
                                owner_files: owner.len(),
                                duplicated_owner_blobs: duplicates,
                            });
                        }
                        Err(error) => report.failures.push(error),
                    }
                }
            }
            Err(error) => report.failures.push(error),
        }
    }
    for candidate in &set.candidates {
        match (
            golden_files(candidate, &candidate.previous_tag),
            golden_files(candidate, &candidate.commit),
        ) {
            (Ok(previous), Ok(current)) => {
                let differences = compare_legacy_goldens(&previous, &current);
                for difference in &differences {
                    report
                        .failures
                        .push(format!("{} wire golden: {difference}", candidate.name));
                }
                if candidate.name == "tl-syntax" {
                    report.syntax_wire_golden = Some(WireFact {
                        previous_tag: candidate.previous_tag.clone(),
                        checked_files: previous.len(),
                        differences: differences.clone(),
                    });
                }
                report.wire_goldens.push(CrateWireFact {
                    name: candidate.name.clone(),
                    previous_tag: candidate.previous_tag.clone(),
                    checked_files: previous.len(),
                    differences,
                });
            }
            (Err(error), _) | (_, Err(error)) => report.failures.push(format!(
                "{} wire golden unavailable: {error}",
                candidate.name
            )),
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
            let Some(fact) = report
                .candidates
                .iter()
                .find(|fact| fact.name == candidate.name)
            else {
                report
                    .failures
                    .push(format!("{}: API candidate fact absent", candidate.name));
                continue;
            };
            match run_api_compatibility(candidate, fact) {
                Ok(compatibility) => report.api_compatibility.push(compatibility),
                Err(error) => report.failures.push(format!("API compatibility: {error}")),
            }
        }
        if !report.failures.is_empty() {
            report.failures.sort();
            report.accepted = false;
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
            );
            return Ok(false);
        }
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
        for (name, target) in CORPUS_LANES {
            let Some(candidate) = set
                .candidates
                .iter()
                .find(|candidate| candidate.name == name)
            else {
                report
                    .failures
                    .push(format!("{name}: corpus replay candidate absent"));
                continue;
            };
            let source = tracked(candidate, &format!("tests/{target}.rs"));
            if source
                .as_ref()
                .map(|source| !source.contains("CORPUS_DIR"))
                .unwrap_or(true)
            {
                report.failures.push(format!(
                    "{name}: {target} does not read the compiled syntax owner corpus"
                ));
                continue;
            }
            let result = rustup_output(
                &candidate.path,
                msrv,
                &["test", "--locked", "--all-features", "--test", target],
            )
            .and_then(|output| {
                if test_target_has_executed_cases(&output) {
                    Ok(())
                } else {
                    Err(format!(
                        "{name}:{target}: no complete executing test population"
                    ))
                }
            });
            report.corpus_lanes.push(CorpusLaneFact {
                name: name.to_owned(),
                target,
                result: if result.is_ok() { "passed" } else { "failed" }.to_owned(),
            });
            if let Err(error) = result {
                report
                    .failures
                    .push(format!("{name} owner corpus replay: {error}"));
            }
        }
        if report.failures.is_empty() {
            match run_consumer_smoke(&report.candidates, msrv) {
                Ok(smoke) => report.consumer_smoke = Some(smoke),
                Err(error) => report
                    .failures
                    .push(format!("external consumer smoke: {error}")),
            }
        }
        if report.failures.is_empty() {
            match run_legacy_decoder_replay(&set, msrv) {
                Ok(replay) => report.legacy_decoder_replay = Some(replay),
                Err(error) => report
                    .failures
                    .push(format!("legacy decoder replay: {error}")),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-176 manual preflight lane: final admission executes this unconditionally
    /// after the exact candidate graph is fixed.
    #[test]
    #[ignore = "requires four immutable Git revisions and Cargo network access"]
    fn tagged_legacy_decoder_replay_probe() {
        let path = std::env::var("TL_LEGACY_CANDIDATE_SET").expect("candidate set path");
        let bytes = fs::read(path).unwrap();
        let set: CandidateSet = serde_json::from_slice(&bytes).unwrap();
        let fact = run_legacy_decoder_replay(&set, "1.98.1").unwrap();
        assert_eq!(fact.previous_revisions.len(), 4);
        assert_eq!(fact.candidate_revisions.len(), 4);
        assert_eq!(fact.result, "passed");
    }

    #[test]
    fn breaking_api_release_requires_new_breaking_version() {
        assert!(!breaking_version_advanced("0.3.0", "0.3.0"));
        assert!(!breaking_version_advanced("0.3.0", "0.3.1"));
        assert!(breaking_version_advanced("0.3.0", "0.4.0"));
        assert!(!breaking_version_advanced("1.2.0", "1.3.0"));
        assert!(breaking_version_advanced("1.2.0", "2.0.0"));
    }
}
