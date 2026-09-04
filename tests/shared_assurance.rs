//! Tests for the shared assurance intake path (FR-006).
//!
//! These follow this repository's own binding idiom: a `// Trace:` comment above
//! each `#[test]`, which is what Quire's census reads. They invoke the gates
//! rather than reimplementing them, because a test that recomputes what a gate
//! computes is a second implementation that can agree with itself while both are
//! wrong.
//!
//! A missing prerequisite is a failure here, never a skip. A gate that stands
//! down when its dependency is absent reports the same green as one that ran.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The interpreter `make assurance-env` builds. Its absence is an error.
fn assurance_python() -> PathBuf {
    let path = std::env::var_os("ASSURANCE_PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".venv-assurance/bin/python"));
    assert!(
        path.is_file(),
        "the pinned assurance interpreter is missing at {}. Run `make assurance-env`. \
         This is a failure and not a skip: a gate that stands down when its dependency \
         is absent reports the same green as one that ran.",
        path.display()
    );
    path
}

fn run(program: &Path, arguments: &[&str]) -> (i32, String, String) {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root())
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", program.display()));
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn json_gate(program: &Path, arguments: &[&str]) -> Value {
    let (code, stdout, stderr) = run(program, arguments);
    assert_eq!(code, 0, "{arguments:?} exited {code}\n{stdout}\n{stderr}");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|error| panic!("{arguments:?} did not emit JSON: {error}\n{stdout}"))
}

fn head_revision() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("git rev-parse failed");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// The chain is expensive and several tests read it. It runs once per test
/// binary, and every reader sees the same run rather than a different one.
static CHAIN: OnceLock<Value> = OnceLock::new();

fn chain_report() -> &'static Value {
    CHAIN.get_or_init(|| {
        // The chain runs under the system interpreter: it only shells out to
        // quoin and never imports engineering-assurance.
        let revision = head_revision();
        let (code, stdout, stderr) = run(
            Path::new("python3"),
            &[
                "scripts/assurance_chain.py",
                "--candidate-revision",
                &revision,
                "--json",
            ],
        );
        assert_eq!(code, 0, "the assurance chain exited {code}\n{stderr}");
        serde_json::from_str(&stdout).expect("the assurance chain did not emit JSON")
    })
}

// Trace: TC-021, FR-006-AC-1
#[test]
fn every_shared_pin_is_classified_by_the_packaged_matrix() {
    let python = assurance_python();
    let report = json_gate(&python, &["scripts/check_shared_pins.py", "--json"]);

    let components = report["components"].as_array().expect("components array");
    assert_eq!(
        components.len(),
        4,
        "the matrix pins four components; this run classified {}",
        components.len()
    );
    for component in components {
        assert_eq!(
            component["verdict"], "compatible",
            "{} is {} ({})",
            component["component"], component["verdict"], component["reason"]
        );
    }
    assert_eq!(report["accepted"], true);
    assert!(report["artifact_mismatches"].as_array().unwrap().is_empty());
    assert!(report["mirror_references"].as_array().unwrap().is_empty());

    // Acceptance is reported and never gated on: the pinned release records
    // `pending_human_acceptance` and ships no predicate for it
    // (agent-ix/engineering-assurance#20). Reading an absent field as approval,
    // in either direction, is the mistake this asserts against.
    assert_eq!(report["acceptance_recorded_here"], false);
    assert!(report["acceptance_state"].is_string());

    // The mirror check must be seen to refuse. Without this it is indistinguishable
    // from a check that matches nothing.
    let (code, stdout, stderr) = run(
        &python,
        &[
            "-c",
            "import json,sys;sys.path.insert(0,'scripts');\
             import check_shared_pins as m;\
             pins=json.load(open('assurance/pins.json'));\
             pins['engineering_assurance']['requirement']+=' --registry=https://npm.ix/';\
             print(json.dumps(m.mirror_references(pins)))",
        ],
    );
    assert_eq!(code, 0, "the mirror probe failed: {stderr}");
    let offenders: Vec<String> = serde_json::from_str(stdout.trim()).unwrap();
    assert!(
        !offenders.is_empty(),
        "a mirror registry reference was not detected; the check matches nothing"
    );
}

// Trace: TC-022, FR-006-AC-2
#[test]
fn the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer() {
    let report = chain_report();
    assert_eq!(report["matched"], true, "{report:#}");

    for group in ["scenarios", "controls", "adapter_probes"] {
        let items = report[group]
            .as_array()
            .unwrap_or_else(|| panic!("{group}"));
        assert!(!items.is_empty(), "{group} is empty");
        for item in items {
            assert_eq!(
                item["matched"], true,
                "{group} entry did not match: {item:#}"
            );
        }
    }

    // The adapter transcribes one named protocol and refuses another, rather than
    // guessing. A verdict recovered from an unrecognised stream is a verdict
    // recovered from nothing.
    let probes = report["adapter_probes"].as_array().unwrap();
    for required in ["refuses-a-foreign-protocol", "accepts-the-real-run"] {
        assert!(
            probes.iter().any(|probe| probe["probe"] == required),
            "adapter probe {required} is missing"
        );
    }
}

/// Write an executable shim for each name that records every invocation.
///
/// The log is the point. A shim that is never consulted and a producer that is
/// never run look identical from the outside, so the shims write down every call
/// and the test reads the file rather than assuming.
///
/// `--version` is answered rather than refused, and deliberately so. Asking a
/// tool its version is an observation — it is what the compatibility matrix's
/// own `observe` column does — and it is not the thing this test forbids. What
/// is forbidden is asking a tool to build, compile, test, or replay anything.
/// Every such invocation is logged and the log must be empty.
fn producer_shims(directory: &Path, names: &[&str]) -> PathBuf {
    fs::create_dir_all(directory).unwrap();
    let log = directory.join("invocations.log");
    let _ = fs::remove_file(&log);
    for name in names {
        let path = directory.join(name);
        fs::write(
            &path,
            format!(
                "#!/bin/sh\n\
                 case \"$1\" in\n\
                 --version|-V) echo \"{name} 9.9.9 (shim)\"; exit 0 ;;\n\
                 esac\n\
                 echo \"$0 $@\" >> {}\n\
                 exit 97\n",
                log.display()
            ),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    log
}

fn run_chain_with_path(shims: &Path) -> std::process::Output {
    let inherited = std::env::var("PATH").unwrap_or_default();
    let revision = head_revision();
    Command::new("python3")
        .args([
            "scripts/assurance_chain.py",
            "--candidate-revision",
            &revision,
        ])
        .current_dir(root())
        .env("PATH", format!("{}:{inherited}", shims.display()))
        .output()
        .expect("failed to run the assurance chain")
}

// Trace: TC-022, FR-006-AC-2
#[test]
fn the_chain_never_executes_a_producer_and_the_probe_can_prove_it() {
    // Two runs, because one proves nothing.
    //
    // Run A replaces every producer — cargo, rustup, rustc — with a stub that
    // logs and fails. The chain must finish, and the log must be empty: not one
    // producer was invoked.
    //
    // Run B is the control. It stubs `quoin`, which the chain is supposed to run,
    // and requires the chain to fail and the log to be non-empty. Without it, an
    // empty log in run A would be equally consistent with PATH never being
    // consulted at all, which is exactly how this test read before the fix.
    let producers = root().join("target/producer-shims");
    let producer_log = producer_shims(&producers, &["cargo", "rustup", "rustc"]);
    let output = run_chain_with_path(&producers);
    let logged = fs::read_to_string(&producer_log).unwrap_or_default();
    assert!(
        output.status.success(),
        "the assurance chain failed with producers stubbed, which means it ran one:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        logged.trim().is_empty(),
        "the assurance driver asked a producer to do work, not just to name its version:\n{logged}"
    );

    let tools = root().join("target/tool-shims");
    let tool_log = producer_shims(&tools, &["quoin"]);
    let control = run_chain_with_path(&tools);
    let tool_logged = fs::read_to_string(&tool_log).unwrap_or_default();
    assert!(
        !tool_logged.trim().is_empty(),
        "stubbing quoin produced no invocation, so PATH is not being consulted by \
         the subprocess and the run above proves nothing"
    );
    assert!(
        !control.status.success(),
        "the chain succeeded with quoin stubbed out, so it is not actually using it"
    );
}

// Trace: TC-023, FR-006-AC-3
#[test]
fn the_sealed_records_impact_snapshot_is_the_quire_export() {
    let report = chain_report();
    let export = root().join(report["quire_export"].as_str().expect("quire_export"));
    let bytes =
        fs::read(&export).unwrap_or_else(|error| panic!("{} is absent: {error}", export.display()));

    let digest = {
        let output = Command::new("sha256sum")
            .arg(&export)
            .output()
            .expect("sha256sum failed");
        String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .expect("sha256sum output")
            .to_owned()
    };
    assert_eq!(
        report["impact_snapshot_digest"], digest,
        "the sealed record's impact snapshot does not name the Quire export it claims"
    );
    // An empty object has a digest too. The snapshot is only worth its content,
    // so the export is required to actually carry the coverage facts the record
    // claims it snapshotted, and to name every requirement this repository has.
    let export: Value = serde_json::from_slice(&bytes).expect("the Quire export is JSON");
    let text = String::from_utf8_lossy(&bytes);
    for requirement in [
        "FR-001", "FR-002", "FR-003", "FR-004", "FR-005", "FR-006", "NFR-001", "NFR-002",
    ] {
        assert!(
            text.contains(requirement),
            "the Quire export does not mention {requirement}; it is not a coverage \
             export of this repository"
        );
    }
    assert!(
        export.is_object() && !export.as_object().unwrap().is_empty(),
        "the Quire export is not a populated document"
    );

    // And the chain must have read it as such rather than as a not-computed run.
    let report = chain_report();
    assert_eq!(
        report["attested_results"]["PROOF-quire-static-export"], "passed",
        "the Quire export was attested as {}",
        report["attested_results"]["PROOF-quire-static-export"]
    );
}

fn git_files(root: &Path, arguments: &[&str]) -> Vec<String> {
    let mut command = Command::new("git");
    command.args(arguments).current_dir(root);
    if let Some(parent) = root.parent() {
        // Do not let a negative-control directory inherit a repository from an
        // ancestor. A missing local Git boundary must remain a refusal.
        command.env("GIT_CEILING_DIRECTORIES", parent);
    }
    let output = command.output().expect("git ls-files failed");
    assert!(
        output.status.success(),
        "git ls-files {arguments:?} exited non-zero; the source census cannot enumerate \
         the repository and reporting it clean would be vacuous: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("the source census refuses non-UTF-8 Git paths")
        .split('\0')
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

const SOURCE_DIRECTORIES: [&str; 7] = [
    "scripts",
    "tests",
    "examples",
    "src",
    "spec",
    "assurance",
    ".github",
];
const REQUIRED_ROOT_FILES: [&str; 7] = [
    "Makefile",
    "Cargo.toml",
    "requirements-assurance.txt",
    "README.md",
    "CLAUDE.md",
    "CONTRIBUTING.md",
    "AGENTS.md",
];

fn source_in_scope(relative: &str) -> bool {
    if REQUIRED_ROOT_FILES.contains(&relative) {
        return true;
    }
    if relative == "tests/shared_assurance.rs"
        || relative.starts_with("spec/reviews/")
        || relative.starts_with("spec/plans/")
    {
        return false;
    }
    let path = Path::new(relative);
    let directory = path
        .components()
        .next()
        .and_then(|part| part.as_os_str().to_str());
    let extension = path.extension().and_then(|value| value.to_str());
    directory.is_some_and(|value| SOURCE_DIRECTORIES.contains(&value))
        && matches!(
            extension,
            Some("py" | "sh" | "rs" | "txt" | "toml" | "yml" | "md" | "json")
        )
}

fn source_sets(root: &Path) -> (Vec<String>, BTreeSet<String>) {
    let tracked: Vec<String> = git_files(root, &["ls-files", "-z"])
        .into_iter()
        .filter(|entry| source_in_scope(entry))
        .collect();
    let mut scanned: BTreeSet<String> = tracked.iter().cloned().collect();
    for entry in git_files(root, &["ls-files", "-z", "--others", "--exclude-standard"])
        .into_iter()
        .filter(|entry| source_in_scope(entry))
    {
        scanned.insert(entry);
    }
    (tracked, scanned)
}

fn source_area(relative: &str) -> String {
    relative
        .split_once('/')
        .map_or("<root>", |(area, _)| area)
        .to_owned()
}

// Trace: TC-025, TC-016, FR-006-AC-5, NFR-002-AC-3
#[test]
fn all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls() {
    // The twelve states this migration must keep distinguishable. Every one of
    // them is now demonstrated by the assurance chain over this repository's own
    // producer output. A state nobody demonstrates is a state nobody would
    // notice the loss of.
    //
    // `malformed` used to be demonstrated by the legacy-compatibility census
    // instead, over a PGM-01 record whose collector field had the wrong type.
    // That census went with the retained evidence under agent-ix/tl-syntax#12.
    // The state did not: the adapter refuses an undecodable row as malformed
    // and says so, and that is what is asserted here. Twelve remains twelve
    // because a demonstrator moved, not because a requirement was relaxed.
    const REQUIRED: [&str; 12] = [
        "pass",
        "fail",
        "unavailable",
        "unsupported",
        "inconclusive",
        "not-computed",
        "malformed",
        "partial",
        "stale",
        "suspect",
        "vacuous",
        "tampered",
    ];

    let report = chain_report();

    let demonstrated: BTreeSet<String> = report["states_demonstrated"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect();

    let missing: Vec<&str> = REQUIRED
        .iter()
        .filter(|state| !demonstrated.contains(**state))
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "these verification outcomes were never demonstrated: {missing:?}; \
         demonstrated: {demonstrated:?}"
    );

    // Every negative names the positive control that proves the step it refuses
    // is a step that works.
    let controls = report["controls"].as_array().unwrap();
    assert!(!controls.is_empty(), "no positive controls were run");
    let negatives: BTreeSet<&str> = controls
        .iter()
        .map(|control| control["pairs_with"].as_str().unwrap())
        .collect();
    for required in [
        "retained-bytes-changed-after-sealing",
        "refuse-an-edited-receipt",
        "stale-candidate-binding",
        "attested-failed",
    ] {
        assert!(
            negatives.contains(required),
            "the negative {required} has no positive control"
        );
    }

    // Four of the twelve states — malformed, unsupported, vacuous, and the
    // not-computed the adapter preserves — now rest on the adapter's own
    // refusals, where before they rested partly on a census the pinned upstream
    // mapping classified. An in-repository check with no external oracle has to
    // be shown capable of failing, so each refusal is switched off in turn and
    // the check that guards it is required to go red.
    //
    // This replaces the `--mutation-probes` gate that ran on every CI run
    // against the deleted compatibility view. Removing that gate and putting
    // nothing in its place would have left the twelve-state claim resting on
    // assertions nobody had seen fail.
    let (code, stdout, stderr) = run(
        Path::new("python3"),
        &["scripts/assurance_chain.py", "--mutation-probes"],
    );
    assert_eq!(
        code, 0,
        "an adapter refusal was switched off and the check that guards it did \
         not notice\n{stdout}\n{stderr}"
    );
    for probe in [
        "refuses-a-foreign-protocol",
        "refuses-an-empty-stream",
        "refuses-a-malformed-row",
        "refuses-an-unnamed-outcome",
    ] {
        assert!(
            stdout.contains(probe),
            "the mutation run did not exercise {probe}\n{stdout}"
        );
    }
}

// Trace: TC-026, FR-006-AC-6
#[test]
fn no_local_evidence_framework_remains_and_nothing_still_reads_the_dropped_tree() {
    let root = root();

    // The generic machinery is gone, by name. The first block went with the
    // migration (agent-ix/tl-syntax#9); the second went with the retained
    // records themselves (agent-ix/tl-syntax#12), under the preservation
    // constraint agent-ix/engineering-assurance#7 released for the pre-stable
    // phase. Deleted outright: nothing was rewritten, re-sealed, or backdated,
    // and no claim that argued from those records survives them.
    for removed in [
        "scripts/build_evidence_envelope.py",
        "scripts/collect_evidence.sh",
        "scripts/finalize_collection.py",
        "scripts/verify_evidence.sh",
        "scripts/verify_evidence_tree.py",
        "scripts/verify_evidence_manifest.py",
        "scripts/evidence_profile.py",
        "scripts/check_evidence_shell_contract.py",
        "scripts/check_failure_propagation.py",
        "scripts/check_traceability_coverage.py",
        "scripts/rust_test_census.py",
        "scripts/run_policy_tests.py",
        "scripts/tool_identity.py",
        "scripts/validate_json_schema.py",
        "tools.lock",
        "tests/evidence_contract.rs",
        "evidence",
        "schemas",
        "scripts/legacy_evidence_view.py",
        "tests/fixtures/legacy-compat",
    ] {
        assert!(
            !root.join(removed).exists(),
            "{removed} is still present; it was deleted and nothing recreates it"
        );
    }

    // And nothing still reaches for any of it. Git supplies the tracked source
    // population and the separate non-ignored untracked scan. Ignored generated
    // files therefore cannot redefine the reviewed population, while a new
    // untracked reader is still inspected before `git add`.
    //
    // `spec/reviews` and `spec/plans` are excluded on purpose. They are closed
    // records of reviews and plans that really did adjudicate this machinery,
    // and a record is not edited to stop naming what it examined. Everything
    // that is live -- code, gates, workflows, requirements, the matrix, the
    // suite registry, the assurance argument -- is in scope.
    const FORBIDDEN: [&str; 5] = [
        "legacy_evidence_view",
        "legacy-compat",
        "PROOF-legacy-compatibility",
        "tl-syntax-evidence-input-v1.schema.json",
        "tl-syntax-evidence-manifest-v1.schema.json",
    ];
    //
    // First pin the three Git population classes in an isolated repository.
    // This is the control for the live census below: changing the helper so an
    // ignored proptest seed enters either set, or so an ordinary untracked
    // source stays invisible, fails here before any live-tree number is read.
    let fixture = root.join("target/source-census-fixture");
    let clear = |path: &Path| match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to clear {}: {error}", path.display()),
    };
    clear(&fixture);
    fs::create_dir_all(fixture.join("src")).expect("create tracked fixture area");
    fs::create_dir_all(fixture.join("tests/proptest-regressions"))
        .expect("create ignored fixture area");
    fs::write(fixture.join(".gitignore"), "proptest-regressions/\n")
        .expect("write fixture ignore rule");
    fs::write(
        fixture.join("src/tracked.rs"),
        "pub const TRACKED: bool = true;\n",
    )
    .expect("write tracked fixture");
    fs::write(
        fixture.join("tests/untracked.rs"),
        "pub const UNTRACKED: bool = true;\n",
    )
    .expect("write untracked fixture");
    fs::write(
        fixture.join("tests/proptest-regressions/integration.txt"),
        "ignored generated seed\n",
    )
    .expect("write ignored fixture");
    let initialized = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(&fixture)
        .status()
        .expect("initialize source-census fixture repository");
    assert!(initialized.success(), "fixture git init failed");
    let staged = Command::new("git")
        .args(["add", ".gitignore", "src/tracked.rs"])
        .current_dir(&fixture)
        .status()
        .expect("stage source-census fixture");
    assert!(staged.success(), "fixture git add failed");
    let (fixture_tracked, fixture_scanned) = source_sets(&fixture);
    assert_eq!(fixture_tracked, vec!["src/tracked.rs"]);
    assert_eq!(
        fixture_scanned,
        BTreeSet::from(["src/tracked.rs".to_owned(), "tests/untracked.rs".to_owned(),]),
        "tracked and non-ignored untracked sources did not reach their declared sets, \
         or an ignored generated source perturbed the census"
    );

    let temporary_root = std::env::temp_dir();
    let non_repository = temporary_root.join(format!(
        "tl-syntax-source-census-non-repository-{}",
        std::process::id()
    ));
    assert_eq!(
        non_repository.parent(),
        Some(temporary_root.as_path()),
        "non-repository control escaped the system temporary directory"
    );
    clear(&non_repository);
    fs::create_dir(&non_repository).expect("create non-repository control");
    let refusal = std::panic::catch_unwind(|| git_files(&non_repository, &["ls-files", "-z"]))
        .expect_err("source enumeration outside a Git repository reported success");
    let refusal = refusal
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| refusal.downcast_ref::<&str>().copied())
        .unwrap_or("non-string panic");
    assert!(
        refusal.contains("source census cannot enumerate"),
        "the unable-to-enumerate control failed for the wrong reason: {refusal}"
    );
    clear(&fixture);
    clear(&non_repository);

    // `assurance/` is in scope and has to be: `change-assurance.json` held the
    // deleted proof obligation and `pins.json` held its digest pins, so it is
    // the likeliest place for a regression, and both are gate inputs --
    // `check_shared_pins.py` reads one and `assurance_chain.py` seals the other.
    let (tracked, scanned) = source_sets(&root);
    for directory in SOURCE_DIRECTORIES {
        assert!(
            root.join(directory).is_dir(),
            "declared source-census directory {directory} is missing"
        );
    }
    for file in REQUIRED_ROOT_FILES {
        assert!(
            root.join(file).is_file() && tracked.iter().any(|entry| entry == file),
            "required tracked source-census root file {file} is missing"
        );
    }

    let mut observed_areas = BTreeMap::new();
    for relative in &tracked {
        *observed_areas
            .entry(source_area(relative))
            .or_insert(0_usize) += 1;
    }
    let expected_areas: BTreeMap<String, usize> = [
        ("<root>", 7),
        (".github", 1),
        ("assurance", 3),
        ("examples", 1),
        ("scripts", 7),
        ("spec", 18),
        ("src", 3),
        ("tests", 2),
    ]
    .into_iter()
    .map(|(area, count)| (area.to_owned(), count))
    .collect();
    assert_eq!(
        observed_areas, expected_areas,
        "the tracked source-census area populations changed; update the expected \
         map only after reviewing the named area delta"
    );
    assert_eq!(
        tracked.len(),
        42,
        "the tracked source-census population changed from 42 to {}; expected and \
         observed areas: {expected_areas:?} / {observed_areas:?}",
        tracked.len()
    );

    for relative in &scanned {
        let path = root.join(relative);
        let source = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("cannot read selected source {}: {error}", path.display())
        });
        for name in FORBIDDEN {
            assert!(
                !source.contains(name),
                "{} still references {name}, which was deleted",
                path.display()
            );
        }
    }
    // The Makefile is orchestration, not a trust root, and carries no gate that
    // polices its own execution, nor the compatibility view it used to run.
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    for gone in [
        "check-failure-propagation",
        "ci-for-evidence",
        "verify-evidence",
        "evidence-tool",
        "check-tool-identities",
        "compat-view",
        "COMPAT_RESULT",
    ] {
        assert!(
            !makefile.contains(gone),
            "the Makefile still carries {gone}"
        );
    }
}
