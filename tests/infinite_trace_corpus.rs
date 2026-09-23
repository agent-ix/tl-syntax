#![cfg(feature = "serde")]

use std::{collections::BTreeSet, fs, path::Path};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use tl_syntax::CORPUS_DIR;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    corpus: String,
    revision: u64,
    role: String,
    formula_schema: String,
    lasso_schema: String,
    valuation_schema: String,
    fairness_schema: String,
    semantic_profile: String,
    clock: String,
    case_count: usize,
    files: Vec<Pin>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pin {
    path: String,
    sha256: String,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

// Trace: TC-160, FR-024-AC-1 (manifest and case-input portion)
#[test]
fn tc_160_owner_corpus_manifest_is_pinned_and_cases_are_distinct() {
    let root = Path::new(CORPUS_DIR).join("infinite-trace");
    let manifest_bytes = fs::read(root.join("manifest.json")).expect("manifest");
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).expect("typed manifest");
    assert_eq!(manifest.corpus, "tl-syntax.infinite-trace-corpus/v1");
    assert_eq!(manifest.revision, 1);
    assert_eq!(manifest.role, "evidence-input");
    assert_eq!(manifest.formula_schema, "tl-syntax.formula-unbounded/v1");
    assert_eq!(manifest.lasso_schema, "tl-syntax.lasso-trace/v1");
    assert_eq!(manifest.valuation_schema, "tl-syntax.partial-valuation/v1");
    assert_eq!(manifest.fairness_schema, "tl-syntax.fairness-premises/v1");
    assert_eq!(manifest.semantic_profile, "mltl.infinite-trace/v1");
    assert_eq!(manifest.clock, "event_position");

    let expected_files = BTreeSet::from(["README.md", "cases.json", "schema.json"]);
    let mut actual_files = BTreeSet::new();
    for pin in &manifest.files {
        assert!(actual_files.insert(pin.path.as_str()), "duplicate file pin");
        assert_eq!(
            digest(&fs::read(root.join(&pin.path)).expect("pinned file")),
            pin.sha256
        );
    }
    assert_eq!(actual_files, expected_files);

    let sums = fs::read_to_string(root.join("SHA256SUMS")).expect("checksum list");
    let listed: BTreeSet<_> = sums
        .lines()
        .map(|line| line.split_once("  ").expect("checksum row"))
        .map(|(hash, path)| (path.to_owned(), hash.to_owned()))
        .collect();
    assert_eq!(listed.len(), 4);
    for name in ["README.md", "cases.json", "manifest.json", "schema.json"] {
        let hash = digest(&fs::read(root.join(name)).expect("checksum member"));
        assert!(
            listed.contains(&(name.to_owned(), hash)),
            "{name} digest mismatch"
        );
    }

    let cases: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("cases.json")).expect("cases"))
            .expect("case JSON");
    let cases = cases["cases"].as_array().expect("case array");
    assert_eq!(cases.len(), manifest.case_count);
    assert_eq!(cases.len(), 13);
    let mut ids = BTreeSet::new();
    let mut families = BTreeSet::new();
    let mut axes = BTreeSet::new();
    for case in cases {
        assert!(ids.insert(case["id"].as_str().expect("case id")));
        families.insert(case["family"].as_str().expect("family"));
        assert!(case["derivation"]
            .as_str()
            .is_some_and(|text| text.len() >= 20));
        assert_eq!(case["formula"]["schema_version"], manifest.formula_schema);
        assert_eq!(case["trace"]["schema_version"], manifest.lasso_schema);
        assert_eq!(case["fairness"]["schema_version"], manifest.fairness_schema);
        if case["expected"]["kind"] == "refusal" {
            axes.insert(case["expected"]["axis"].as_str().expect("refusal axis"));
        }
    }
    assert_eq!(
        families,
        BTreeSet::from(["fairness", "lasso", "negative", "partial", "past"])
    );
    assert!(axes.contains("profile"));
    assert!(axes.contains("clock"));
    assert!(axes.contains("fairness"));
    assert!(axes.contains("operator"));
}
