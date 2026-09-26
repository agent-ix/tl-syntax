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
    assert_eq!(cases.len(), 15);
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
        if case["id"] == "finite-prefix-globally-inconclusive" {
            assert_eq!(case["subject_kind"], "finite_prefix");
            assert_eq!(case["trace"]["prefix"].as_array().unwrap().len(), 1);
            assert_eq!(case["expected"]["value"], "inconclusive");
        } else {
            assert!(case.get("subject_kind").is_none());
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

// Trace: TC-160, FR-024-AC-1 (owner formula reader over every corpus case)
#[test]
fn tc_160_formula_cases_reach_the_owner_unbounded_decoder() {
    let bytes = fs::read(Path::new(CORPUS_DIR).join("infinite-trace/cases.json")).unwrap();
    let corpus: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let result =
            serde_json::from_value::<tl_syntax::InfiniteFormulaDocument>(case["formula"].clone());
        let formula_refusal = matches!(
            id,
            "finite-profile-refuses-unbounded" | "previous-with-unbounded-interval-refuses"
        );
        assert_eq!(
            result.is_err(),
            formula_refusal,
            "owner formula admission differs for {id}: {result:?}"
        );
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusTrace {
    schema_version: String,
    semantic_profile: String,
    clock: String,
    proposition_map: Vec<tl_syntax::PropositionEntry>,
    prefix: Vec<CorpusObservation>,
    #[serde(rename = "loop")]
    loop_observations: Vec<CorpusObservation>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusObservation {
    position: u32,
    valuation: Vec<CorpusValue>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusValue {
    proposition: tl_syntax::PropositionId,
    state: tl_syntax::PartialValue,
}

fn expected_case(id: &str) -> Option<(&'static str, &'static str, Option<&'static str>)> {
    Some(match id {
        "globally-true-empty-prefix" => ("lasso", "proved", None),
        "eventual-witness-after-prefix" => ("lasso", "proved", None),
        "globally-refuted-by-loop" => ("lasso", "refuted", None),
        "unbounded-once-reaches-origin" => ("past", "proved", None),
        "missing-value-is-inconclusive" => ("partial", "inconclusive", None),
        "conflicting-value-is-inconclusive" => ("partial", "inconclusive", None),
        "fair-loop-satisfies-premise" => ("fairness", "proved", None),
        "unfair-loop-has-no-admitted-trace" => ("fairness", "inconclusive", None),
        "finite-prefix-globally-inconclusive" => ("partial", "inconclusive", None),
        "finite-profile-refuses-unbounded" => (
            "negative",
            "unbounded_requires_infinite_profile",
            Some("profile"),
        ),
        "clock-mismatch-refuses" => ("negative", "clock_identity_mismatch", Some("clock")),
        "bounded-globally-in-infinite-profile" => ("lasso", "proved", None),
        "bounded-historically-in-infinite-profile" => ("past", "proved", None),
        "fairness-on-non-lasso-refuses" => {
            ("negative", "fairness_requires_lasso", Some("fairness"))
        }
        "previous-with-unbounded-interval-refuses" => {
            ("negative", "previous_has_no_interval", Some("operator"))
        }
        _ => return None,
    })
}

fn admit_case(case: &serde_json::Value) -> Result<(), &'static str> {
    use tl_syntax::{
        FairnessPremisesDocument, InfiniteClock, InfiniteFormulaDocument, LassoTraceDocument,
        PartialValuation, TraceObservation, ValuationEntry,
    };
    let formula_json = &case["formula"];
    if formula_json["semantic_profile"] != "mltl.infinite-trace/v1" {
        return Err("profile");
    }
    if formula_json["nodes"].as_array().is_some_and(|nodes| {
        nodes
            .iter()
            .any(|node| node["kind"] == "strong_previous" && node.get("interval").is_some())
    }) {
        return Err("operator");
    }
    let formula: InfiniteFormulaDocument =
        serde_json::from_value(formula_json.clone()).map_err(|_| "schema")?;
    let trace: CorpusTrace = serde_json::from_value(case["trace"].clone()).map_err(|_| "schema")?;
    if trace.schema_version != tl_syntax::LASSO_TRACE_V1 {
        return Err("schema");
    }
    if trace.semantic_profile != "mltl.infinite-trace/v1" {
        return Err("profile");
    }
    if trace.clock != "event_position" {
        return Err("clock");
    }
    let fairness = &case["fairness"];
    if fairness["schema_version"] != tl_syntax::FAIRNESS_PREMISES_V1 {
        return Err("schema");
    }
    if fairness["semantic_profile"] != "mltl.infinite-trace/v1" {
        return Err("profile");
    }
    if fairness["clock"] != "event_position" {
        return Err("clock");
    }
    let roots: Vec<tl_syntax::NodeId> =
        serde_json::from_value(fairness["roots"].clone()).map_err(|_| "fairness")?;
    if trace.loop_observations.is_empty() && !roots.is_empty() {
        return Err("fairness");
    }
    let map =
        tl_syntax::PropositionMapDocument::new(trace.proposition_map).map_err(|_| "valuation")?;
    let map_identity = map.content_identity().map_err(|_| "valuation")?;
    let propositions: Vec<_> = map.propositions().iter().map(|entry| entry.id).collect();
    let convert =
        |observations: Vec<CorpusObservation>| -> Result<Vec<TraceObservation>, &'static str> {
            observations
                .into_iter()
                .map(|observation| {
                    let entries = observation
                        .valuation
                        .into_iter()
                        .map(|value| ValuationEntry {
                            proposition: value.proposition,
                            value: value.state,
                        })
                        .collect();
                    let valuation =
                        PartialValuation::new(map_identity.clone(), &propositions, entries)
                            .map_err(|_| "valuation")?;
                    Ok(TraceObservation {
                        position: observation.position,
                        valuation,
                    })
                })
                .collect()
        };
    let prefix = convert(trace.prefix)?;
    let loop_observations = convert(trace.loop_observations)?;
    let _trace = LassoTraceDocument::new(
        tl_syntax::SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        map_identity,
        propositions,
        prefix,
        loop_observations,
    )
    .map_err(|_| "lasso")?;
    let graph_id = formula.content_identity().map_err(|_| "schema")?;
    let _fairness =
        FairnessPremisesDocument::new(&formula, graph_id, InfiniteClock::EventPosition, roots)
            .map_err(|_| "fairness")?;
    Ok(())
}

fn verify_case(case: &serde_json::Value) -> Result<(), String> {
    let id = case["id"].as_str().ok_or("case id absent")?;
    let (family, value, axis) = expected_case(id).ok_or("unknown case id")?;
    match id {
        "unfair-loop-has-no-admitted-trace" => {
            if case.get("subject_kind").is_some()
                || case["fairness"]["roots"] != serde_json::json!([2])
                || case["trace"]["loop"][0]["valuation"][0]["state"] != "false"
            {
                return Err(format!("{id}: unfair-loop witness changed"));
            }
        }
        "finite-prefix-globally-inconclusive" => {
            if case["subject_kind"] != "finite_prefix"
                || case["trace"]["prefix"].as_array().map(Vec::len) != Some(1)
                || case["trace"]["prefix"][0]["valuation"][0]["state"] != "true"
                || case["trace"]["loop"][0]["valuation"][0]["state"] != "false"
            {
                return Err(format!("{id}: finite-prefix witness changed"));
            }
        }
        _ if case.get("subject_kind").is_some() => {
            return Err(format!("{id}: unexpected subject selection"));
        }
        _ => {}
    }
    if case["family"] != family || case["expected"]["value"] != value {
        return Err(format!("{id}: expected family/verdict changed"));
    }
    if let Some(axis) = axis {
        if case["expected"]["kind"] != "refusal" || case["expected"]["axis"] != axis {
            return Err(format!("{id}: expected refusal changed"));
        }
        if admit_case(case) != Err(axis) {
            return Err(format!("{id}: refusal axis not reproduced"));
        }
    } else {
        if case["expected"]["kind"] != "settlement" {
            return Err(format!("{id}: settlement kind changed"));
        }
        admit_case(case).map_err(|actual| format!("{id}: unexpected {actual} refusal"))?;
    }
    Ok(())
}

// Trace: TC-161, FR-024-AC-2
#[test]
fn tc_161_every_owner_case_reaches_typed_admission_and_frozen_oracle() {
    let bytes = fs::read(Path::new(CORPUS_DIR).join("infinite-trace/cases.json")).unwrap();
    let corpus: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        verify_case(case).unwrap();
    }
}

// Trace: TC-161, FR-024-AC-2
#[test]
fn tc_161_input_expected_axis_and_verdict_mutations_fail() {
    let bytes = fs::read(Path::new(CORPUS_DIR).join("infinite-trace/cases.json")).unwrap();
    let corpus: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let mut positive = corpus["cases"][0].clone();
    positive["trace"]["clock"] = "fixed_sample".into();
    assert!(verify_case(&positive).is_err());
    let mut positive = corpus["cases"][0].clone();
    positive["expected"]["value"] = "refuted".into();
    assert!(verify_case(&positive).is_err());
    let mut negative = corpus["cases"][7].clone();
    negative["expected"]["axis"] = "clock".into();
    assert!(verify_case(&negative).is_err());
    let mut unfair = corpus["cases"][13].clone();
    unfair["trace"]["loop"][0]["valuation"][0]["state"] = "true".into();
    assert!(verify_case(&unfair).is_err());
    let mut prefix = corpus["cases"][14].clone();
    prefix["subject_kind"] = "lasso".into();
    assert!(verify_case(&prefix).is_err());
}

fn verify_manifest_pins(root: &Path, manifest: &serde_json::Value) -> Result<(), String> {
    let pins = manifest["files"].as_array().ok_or("files absent")?;
    for pin in pins {
        let path = pin["path"].as_str().ok_or("pin path absent")?;
        let expected = pin["sha256"].as_str().ok_or("digest absent")?;
        let actual = digest(&fs::read(root.join(path)).map_err(|error| error.to_string())?);
        if actual != expected {
            return Err(format!("{path}: digest mismatch"));
        }
    }
    Ok(())
}

// Trace: TC-161, FR-024-AC-2
#[test]
fn tc_161_pinned_digest_mutation_fails() {
    let root = Path::new(CORPUS_DIR).join("infinite-trace");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("manifest.json")).unwrap()).unwrap();
    assert!(verify_manifest_pins(&root, &manifest).is_ok());
    manifest["files"][0]["sha256"] =
        "0000000000000000000000000000000000000000000000000000000000000000".into();
    assert!(verify_manifest_pins(&root, &manifest).is_err());
}
