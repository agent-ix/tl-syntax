use std::{fs, path::PathBuf};

// Trace: SUITE-001, SUITE-002, SUITE-003, SUITE-004, SUITE-005, SUITE-006, SUITE-007
#[test]
fn evidence_suite_registry_is_wired_to_executable_gates() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    let collector = fs::read_to_string(root.join("scripts/collect_evidence.sh")).unwrap();
    let suites = fs::read_to_string(root.join("spec/evidence/suites.md")).unwrap();

    for target in ["ci:", "spec:", "check-corpus:", "evidence-tool:"] {
        assert!(makefile.contains(target), "missing Make target {target}");
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
