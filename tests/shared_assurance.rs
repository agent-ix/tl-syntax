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

use std::collections::BTreeSet;
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

// Trace: TC-022, FR-006-AC-2
#[test]
fn the_chain_completes_with_every_producer_removed_from_the_path() {
    // The strongest available statement of "this driver never runs a producer" is
    // to take the producers away and watch the driver finish anyway. `cargo` and
    // `rustup` are replaced with stubs that fail on any invocation; `quire` and
    // `quoin` are left real, because the driver is supposed to run those.
    let shims = root().join("target/producer-shims");
    fs::create_dir_all(&shims).unwrap();
    for name in ["cargo", "rustup", "rustc"] {
        let path = shims.join(name);
        fs::write(
            &path,
            "#!/bin/sh\necho \"a producer was executed by the assurance driver: $0 $@\" >&2\nexit 97\n",
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    let inherited = std::env::var("PATH").unwrap_or_default();
    let revision = head_revision();
    let output = Command::new("python3")
        .args([
            "scripts/assurance_chain.py",
            "--candidate-revision",
            &revision,
        ])
        .current_dir(root())
        .env("PATH", format!("{}:{inherited}", shims.display()))
        .output()
        .expect("failed to run the assurance chain");
    assert!(
        output.status.success(),
        "the assurance chain failed once producers were removed from PATH, \
         which means it was running one:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // And the stubs must actually have been reachable, or the test proves nothing.
    let (code, _, _) = {
        let probe = Command::new(shims.join("cargo"))
            .output()
            .expect("the shim itself is not executable");
        (probe.status.code().unwrap_or(-1), (), ())
    };
    assert_eq!(
        code, 97,
        "the producer shim is not the failing stub it claims to be"
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
    assert!(!bytes.is_empty(), "the Quire export is empty");

    // The export is Quire's, not this repository's restatement of it.
    let export_value: Value = serde_json::from_slice(&bytes).expect("the Quire export is JSON");
    assert!(
        export_value.is_object() || export_value.is_array(),
        "the Quire export is not a structured document"
    );
}

// Trace: TC-024, FR-006-AC-4
#[test]
fn retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte() {
    let python = assurance_python();
    let census = json_gate(&python, &["scripts/legacy_evidence_view.py", "--json"]);

    assert!(census["evidence_bytes_moved"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(census["misattributed_records"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(census["matched"], true);

    let files = census["evidence_files_read"].as_u64().unwrap();
    let on_disk = walk(&root().join("evidence"));
    assert_eq!(
        files, on_disk,
        "the compatibility view read {files} evidence files but {on_disk} are present"
    );

    let retained = &census["retained"];
    assert!(retained["count"].as_u64().unwrap() > 0);
    // The honest answer for this repository. Its retained family is
    // quire.derivation-evidence/v1, which the pinned mapping does not cover, so
    // every envelope is refused. That refusal is reported as it stands and is
    // not converted into a pass. Filed as agent-ix/engineering-assurance#21.
    assert_eq!(
        retained["outcomes"],
        serde_json::json!(["incompatible"]),
        "the retained-evidence outcome changed; if the shared mapping gained a \
         derivation-evidence reader this assertion should be updated deliberately"
    );

    // The mapping must be seen to accept, or a refusal proves nothing.
    let cases = census["cases"].as_array().unwrap();
    assert!(
        cases
            .iter()
            .any(|case| case["kind"] == "positive_control" && case["outcome"] == "lossy"),
        "no positive control was accepted; a mapping only ever seen refusing is \
         indistinguishable from a step that never worked"
    );

    let (code, stdout, stderr) = run(
        &python,
        &["scripts/legacy_evidence_view.py", "--mutation-probes"],
    );
    assert_eq!(
        code, 0,
        "a load-bearing compatibility check was removed and the census did not \
         notice\n{stdout}\n{stderr}"
    );
}

fn walk(directory: &Path) -> u64 {
    let mut count = 0;
    for entry in fs::read_dir(directory).expect("evidence directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            count += walk(&path);
        } else {
            count += 1;
        }
    }
    count
}

// Trace: TC-025, TC-016, FR-006-AC-5, NFR-002-AC-3
#[test]
fn all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls() {
    // The twelve states this migration must keep distinguishable, and the gate
    // that owns each. A state nobody demonstrates is a state nobody would notice
    // the loss of.
    const REQUIRED: [(&str, &str); 12] = [
        ("pass", "chain"),
        ("fail", "chain"),
        ("unavailable", "chain"),
        ("unsupported", "chain"),
        ("inconclusive", "chain"),
        ("not-computed", "chain"),
        ("malformed", "compatibility"),
        ("partial", "chain"),
        ("stale", "chain"),
        ("suspect", "chain"),
        ("vacuous", "chain"),
        ("tampered", "chain"),
    ];

    let python = assurance_python();
    let report = chain_report();
    let census = json_gate(&python, &["scripts/legacy_evidence_view.py", "--json"]);

    let mut demonstrated: BTreeSet<String> = report["states_demonstrated"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect();
    for case in census["cases"].as_array().unwrap() {
        demonstrated.insert(case["kind"].as_str().unwrap().replace('_', "-"));
    }

    let missing: Vec<&str> = REQUIRED
        .iter()
        .filter(|(state, _)| !demonstrated.contains(*state))
        .map(|(state, _)| *state)
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
        "attested-failure",
    ] {
        assert!(
            negatives.contains(required),
            "the negative {required} has no positive control"
        );
    }
}

// Trace: TC-026, FR-006-AC-6
#[test]
fn no_local_evidence_framework_remains_and_the_frozen_schemas_are_referenced_by_nothing() {
    let root = root();

    // The generic machinery is gone, by name.
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
    ] {
        assert!(
            !root.join(removed).exists(),
            "{removed} is still present; the generic evidence machinery was not removed"
        );
    }

    // The two evidence schemas are frozen, not deleted: retained envelopes name
    // them by path and by SHA-256, and removing them would break a reference
    // inside bytes this migration is required to leave untouched.
    let frozen = [
        (
            "schemas/tl-syntax-evidence-input-v1.schema.json",
            "e6c1d95a3b8849ab37077e8789e5a297e14e037e9f8c528f20a88383c51cf8c0",
        ),
        (
            "schemas/tl-syntax-evidence-manifest-v1.schema.json",
            "3a3124a5a934272cb4a288909d46da2dccdfe600ab588ca3de89f38cb03059ef",
        ),
    ];
    for (path, expected) in frozen {
        let file = root.join(path);
        assert!(
            file.is_file(),
            "{path} was deleted; it is frozen, not removed"
        );
        let output = Command::new("sha256sum").arg(&file).output().unwrap();
        let digest = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned();
        assert_eq!(
            digest, expected,
            "{path} changed; a frozen schema is immutable"
        );
    }

    // Nothing validates against them any more, and a census this small would be
    // vacuous, so the census size is asserted too.
    let mut inspected = 0;
    for directory in ["scripts", "tests", "examples"] {
        for entry in fs::read_dir(root.join(directory)).unwrap() {
            let path = entry.unwrap().path();
            let extension = path.extension().and_then(|value| value.to_str());
            if !matches!(extension, Some("py" | "sh" | "rs" | "txt")) {
                continue;
            }
            inspected += 1;
            let source = fs::read_to_string(&path).unwrap();
            for (schema, _) in frozen {
                let name = Path::new(schema).file_name().unwrap().to_str().unwrap();
                if path.file_name().and_then(|value| value.to_str()) == Some("shared_assurance.rs")
                {
                    continue;
                }
                assert!(
                    !source.contains(name),
                    "{} references the frozen schema {name}; nothing may validate against it",
                    path.display()
                );
            }
        }
    }
    assert!(
        inspected > 5,
        "the source census is unexpectedly small ({inspected}) to make this claim"
    );

    // The Makefile is orchestration, not a trust root, and carries no gate that
    // polices its own execution.
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    for gone in [
        "check-failure-propagation",
        "ci-for-evidence",
        "verify-evidence",
        "evidence-tool",
        "check-tool-identities",
    ] {
        assert!(
            !makefile.contains(gone),
            "the Makefile still carries the {gone} self-attestation target"
        );
    }
}
