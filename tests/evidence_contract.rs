use std::{fs, path::PathBuf, process::Command};

// Trace: SUITE-001, SUITE-002, SUITE-003, SUITE-004, SUITE-005, SUITE-006, SUITE-007
#[test]
fn evidence_suite_registry_is_wired_to_executable_gates() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    let collector = fs::read_to_string(root.join("scripts/collect_evidence.sh")).unwrap();
    let suites = fs::read_to_string(root.join("spec/evidence/suites.md")).unwrap();

    let dry_run = Command::new("make")
        .args(["--no-print-directory", "-n", "ci"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        dry_run.status.success(),
        "make -n ci failed: {}",
        String::from_utf8_lossy(&dry_run.stderr)
    );
    let ci_commands = String::from_utf8(dry_run.stdout).unwrap();

    for target in ["ci:", "spec:", "check-corpus:", "evidence-tool:"] {
        assert!(makefile.contains(target), "missing Make target {target}");
    }
    for command in [
        "cargo fmt --all -- --check",
        "check_default_dependencies.py",
        "cargo clippy --all-targets --all-features -- -D warnings",
        "cargo test --all-features",
        "validate_corpus.py",
        "cargo deny check licenses",
        "cargo deny check sources",
        "test_evidence_tool.py",
        "quire validate",
        "quire coverage",
        "verify_evidence.sh",
    ] {
        assert!(ci_commands.contains(command), "make ci omits {command}");
    }
    for command in [
        "make ci",
        "make spec",
        "quire coverage --scope . --strict",
        "cargo doc --no-deps --all-features",
        "PGM01_SCHEMA",
        "PGM01_VALIDATOR",
    ] {
        assert!(collector.contains(command), "collector omits {command}");
    }
    for suite in [
        "SUITE-001",
        "SUITE-002",
        "SUITE-003",
        "SUITE-004",
        "SUITE-005",
        "SUITE-006",
        "SUITE-007",
    ] {
        assert!(suites.contains(suite), "registry omits {suite}");
    }

    for schema in [
        "schemas/tl-syntax-evidence-input-v1.schema.json",
        "schemas/tl-syntax-evidence-manifest-v1.schema.json",
    ] {
        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(root.join(schema)).unwrap()).unwrap();
        assert_eq!(value["$schema"], "http://json-schema.org/draft-07/schema#");
        assert_eq!(value["additionalProperties"], false);
    }
}

// Trace: TC-016, NFR-002-AC-3
#[test]
fn evidence_producer_rejects_false_success_classifications() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new("python3")
        .arg("scripts/test_evidence_tool.py")
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "evidence behavior test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
