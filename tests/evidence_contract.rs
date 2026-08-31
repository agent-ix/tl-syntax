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
        "cargo deny check advisories",
        "cargo deny check bans",
        "cargo deny check licenses",
        "cargo deny check sources",
        "test_evidence_tool.py",
        "test_failure_propagation.py",
        "test_json_schema_gate.py",
        "test_traceability_gate.py",
        "quire validate --scope . 'spec/**/*.md' --strict --summary",
        "check_traceability_coverage.py",
        "verify_evidence.sh",
    ] {
        assert!(ci_commands.contains(command), "make ci omits {command}");
    }
    assert!(ci_commands.contains("scripts/check_failure_propagation.py"));
    let exact_cargo_test = ci_commands.lines().any(|line| {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        fields.len() == 3
            && PathBuf::from(fields[0])
                .file_name()
                .and_then(|name| name.to_str())
                == Some("cargo")
            && fields[1..] == ["test", "--all-features"]
    });
    assert!(
        exact_cargo_test,
        "make ci changes or weakens required command cargo test --all-features"
    );
    assert!(
        ci_commands
            .lines()
            .any(|line| { line.trim() == "python3 scripts/check_traceability_coverage.py" }),
        "make ci changes or weakens the strict traceability policy gate"
    );
    for command in [
        "make ci",
        "make spec",
        "python3 scripts/check_traceability_coverage.py",
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

// Trace: TC-018, NFR-002-AC-4
#[test]
fn mandatory_policy_gates_observe_failure_states() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for (program, args) in [
        (
            "make",
            vec!["--no-print-directory", "check-failure-propagation"],
        ),
        ("python3", vec!["scripts/test_failure_propagation.py"]),
        ("python3", vec!["scripts/test_corpus_gate.py"]),
        ("python3", vec!["scripts/test_json_schema_gate.py"]),
        ("python3", vec!["scripts/test_traceability_gate.py"]),
    ] {
        let output = Command::new(program)
            .args(args)
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "policy behavior gate failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let disabled_guard = Command::new("make")
        .args([
            "--no-print-directory",
            "check-failure-propagation",
            "PYTHON=false",
        ])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        !disabled_guard.status.success(),
        "check-failure-propagation target no longer invokes its executable policy"
    );
}
