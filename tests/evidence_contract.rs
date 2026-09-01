use std::{fs, path::PathBuf, process::Command};

fn make_prerequisites(makefile: &str, target: &str) -> Vec<String> {
    let prefix = format!("{target}:");
    let definition = makefile
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("missing Make target {target}"));
    definition[prefix.len()..]
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

fn make_recipe(makefile: &str, target: &str) -> String {
    let prefix = format!("{target}:");
    let mut lines = makefile
        .lines()
        .skip_while(|line| !line.starts_with(&prefix));
    assert!(lines.next().is_some(), "missing Make target {target}");
    lines
        .take_while(|line| line.starts_with('\t') || line.trim().is_empty())
        .filter(|line| line.starts_with('\t'))
        .collect::<Vec<_>>()
        .join("\n")
}

// Trace: SUITE-001, SUITE-002, SUITE-003, SUITE-004, SUITE-005, SUITE-006, SUITE-007
#[test]
fn evidence_suite_registry_is_wired_to_executable_gates() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    let collector = fs::read_to_string(root.join("scripts/collect_evidence.sh")).unwrap();
    let suites = fs::read_to_string(root.join("spec/evidence/suites.md")).unwrap();

    let ci_targets = make_prerequisites(&makefile, "ci");
    let candidate_targets = make_prerequisites(&makefile, "ci-for-evidence");

    for target in ["ci:", "spec:", "check-corpus:", "evidence-tool:"] {
        assert!(makefile.contains(target), "missing Make target {target}");
    }
    for (target, command) in [
        (
            "check-failure-propagation",
            "scripts/check_failure_propagation.py",
        ),
        ("fmt-check", "cargo fmt --all -- --check"),
        ("check-features", "cargo check --lib --no-default-features"),
        (
            "check-default-dependencies",
            "check_default_dependencies.py",
        ),
        (
            "lint",
            "cargo clippy --all-targets --all-features -- -D warnings",
        ),
        ("test", "cargo test --all-features"),
        ("check-corpus", "validate_corpus.py"),
        ("deny", "cargo deny check advisories"),
        ("deny", "cargo deny check bans"),
        ("deny", "cargo deny check licenses"),
        ("deny", "cargo deny check sources"),
        ("audit-unsafe", "scripts/check_unsafe_comments.sh"),
        ("evidence-tool", "scripts/run_policy_tests.py"),
        (
            "spec",
            "quire validate --scope . 'spec/**/*.md' --strict --summary",
        ),
        ("spec", "check_traceability_coverage.py"),
        (
            "verify-evidence",
            "scripts/check_evidence_shell_contract.py",
        ),
        (
            "verify-evidence",
            "/usr/bin/bash scripts/verify_evidence.sh",
        ),
        ("msrv", "rustup run 1.75.0 cargo test --all-features"),
        (
            "rustdoc",
            "RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --all-features",
        ),
    ] {
        assert!(
            ci_targets.iter().any(|item| item == target),
            "make ci omits {target}"
        );
        assert!(
            make_recipe(&makefile, target).contains(command),
            "Make target {target} omits {command}"
        );
    }
    assert!(
        candidate_targets
            .iter()
            .any(|item| item == "check-tool-identities")
            && make_recipe(&makefile, "check-tool-identities")
                .contains("/usr/bin/python3 scripts/tool_identity.py --verify-live"),
        "candidate CI omits live tool-identity verification"
    );
    assert!(
        !ci_targets
            .iter()
            .any(|item| item == "check-tool-identities")
            && !candidate_targets
                .iter()
                .any(|item| item == "verify-evidence"),
        "portable and host-qualified Make graphs are not separated"
    );
    let test_recipe = make_recipe(&makefile, "test");
    let exact_cargo_test = test_recipe.lines().any(|line| {
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
        make_recipe(&makefile, "spec").lines().any(|line| line
            .trim()
            .ends_with("python3 scripts/check_traceability_coverage.py")),
        "make ci changes or weakens the strict traceability policy gate"
    );
    for command in [
        "make ci-for-evidence",
        "make spec",
        "python3 scripts/check_traceability_coverage.py",
        "cargo doc --no-deps --all-features",
        "PGM01_SCHEMA",
        "PGM01_VALIDATOR",
        "run_and_retain input-schema",
        "run_and_retain manifest-schema",
        "run_and_retain pgm01-schema",
        "run_and_retain pgm01-validator",
        "run_and_retain sealed-pgm01-schema",
        "run_and_retain sealed-pgm01-validator",
        "finalize_collection.py",
    ] {
        assert!(collector.contains(command), "collector omits {command}");
    }
    assert!(
        collector.contains("jsonschema-format-checkers.txt"),
        "collector does not retain the installed JSON format checker set"
    );
    assert!(
        collector.contains("clean_env=(/usr/bin/env -i PATH="),
        "collector does not replace the ambient PATH with its trusted tool path"
    );
    assert!(
        collector.contains("for tool in bash cargo git make python3 quire rustc rustup sha256sum")
            && collector.contains("tool-${tool}-path.txt")
            && collector.contains("tool-${tool}-sha256.txt"),
        "collector does not retain resolved mandatory-tool identities"
    );
    assert!(
        collector.contains("verify_pinned_external")
            && collector
                .contains("0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256")
            && collector
                .contains("1c2881d5f8800dab031f6afa26d5ad11f88a5ab42a942bc9fe0c2853b58df2f1"),
        "collector does not recheck the reviewed PGM-01 artifact digests"
    );
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
            "/usr/bin/python3",
            vec!["scripts/check_failure_propagation.py"],
        ),
        ("python3", vec!["scripts/test_failure_propagation.py"]),
        ("python3", vec!["scripts/test_corpus_gate.py"]),
        ("python3", vec!["scripts/test_json_schema_gate.py"]),
        ("python3", vec!["scripts/test_traceability_gate.py"]),
    ] {
        let mut command = Command::new(program);
        command.args(args).current_dir(&root);
        command.env_remove("MAKEFLAGS");
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "policy behavior gate failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let mut probe_dir = std::env::temp_dir();
    probe_dir.push(format!("tl-syntax-policy-probe-{}", std::process::id()));
    let _ = fs::remove_dir_all(&probe_dir);
    fs::create_dir(&probe_dir).unwrap();
    fs::write(probe_dir.join("test_fails.py"), "raise SystemExit(7)\n").unwrap();
    let disabled_guard = Command::new("python3")
        .args([
            "scripts/run_policy_tests.py",
            "--directory",
            probe_dir.to_str().unwrap(),
        ])
        .current_dir(&root)
        .output()
        .unwrap();
    fs::remove_dir_all(&probe_dir).unwrap();
    assert!(
        !disabled_guard.status.success(),
        "policy runner swallowed a discovered failing test"
    );
}
