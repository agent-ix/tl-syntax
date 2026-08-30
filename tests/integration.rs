#![cfg(feature = "serde")]

use tl_syntax::{
    FormulaDocument, FormulaSchemaVersion, Interval, Node, NodeId, NodeKind, PropositionEntry,
    PropositionId, PropositionMapDocument, SemanticProfile, SourceSpan,
};

#[derive(serde::Deserialize)]
struct CorpusManifest {
    corpus_revision: String,
    formula_schema: String,
    proposition_map_schema: String,
    proposition_map: String,
    semantic_profiles: Vec<String>,
    fixtures: Vec<CorpusFixture>,
}

#[derive(serde::Deserialize)]
struct CorpusFixture {
    id: String,
    class: String,
    formula: String,
    expected_validation: String,
    #[serde(default)]
    trace: Option<Vec<Vec<u32>>>,
    #[serde(default)]
    expected_horizon: Option<u32>,
    #[serde(default)]
    expected_closed_trace: Option<bool>,
}

// Trace: TC-008, TC-009, FR-003-AC-3, FR-004-AC-1
#[test]
fn formula_document_round_trips_with_required_profile() {
    let document = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(1),
        vec![
            Node::with_span(
                NodeKind::Proposition {
                    proposition: PropositionId(4),
                },
                SourceSpan::new(0, 5).unwrap(),
            ),
            Node::new(NodeKind::Future {
                interval: Interval::new(1, 2).unwrap(),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();

    let json = serde_json::to_string(&document).unwrap();
    assert!(json.contains("mltl.closed-trace/v1"));
    let decoded: FormulaDocument = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, document);
    decoded.validate().unwrap();

    let missing_profile = json.replace("\"semantic_profile\":\"mltl.closed-trace/v1\",", "");
    assert!(serde_json::from_str::<FormulaDocument>(&missing_profile).is_err());
}

// Trace: TC-010, FR-004-AC-2
#[test]
fn unknown_schema_and_profile_versions_are_rejected() {
    let formula = r#"{
        "schema_version":"tl-syntax.formula/v2",
        "semantic_profile":"mltl.closed-trace/v1",
        "root":0,
        "nodes":[{"kind":"true"}]
    }"#;
    assert!(serde_json::from_str::<FormulaDocument>(formula).is_err());

    let formula = r#"{
        "schema_version":"tl-syntax.formula/v1",
        "semantic_profile":"mltl.closed-trace/v2",
        "root":0,
        "nodes":[{"kind":"true"}]
    }"#;
    assert!(serde_json::from_str::<FormulaDocument>(formula).is_err());

    let proposition_map = r#"{
        "schema_version":"tl-syntax.proposition-map/v2",
        "propositions":[]
    }"#;
    assert!(serde_json::from_str::<PropositionMapDocument>(proposition_map).is_err());
}

// Trace: TC-011, FR-004-AC-3
#[test]
fn checked_values_and_graphs_reject_malformed_wire_data() {
    let inverted_interval = r#"{
        "schema_version":"tl-syntax.formula/v1",
        "semantic_profile":"mltl.closed-trace/v1",
        "root":1,
        "nodes":[
          {"kind":"true"},
          {"kind":"future","interval":{"start":2,"end":1},"operand":0}
        ]
    }"#;
    assert!(serde_json::from_str::<FormulaDocument>(inverted_interval).is_err());

    let forward_reference = r#"{
        "schema_version":"tl-syntax.formula/v1",
        "semantic_profile":"mltl.closed-trace/v1",
        "root":0,
        "nodes":[{"kind":"not","operand":1}]
    }"#;
    let document: FormulaDocument = serde_json::from_str(forward_reference).unwrap();
    assert!(document.validate().is_err());
}

// Trace: TC-009, FR-004-AC-1
#[test]
fn proposition_map_round_trips_in_stable_order() {
    let map = PropositionMapDocument::new(vec![
        PropositionEntry {
            id: PropositionId(0),
            name: "request".into(),
        },
        PropositionEntry {
            id: PropositionId(1),
            name: "response".into(),
        },
    ])
    .unwrap();
    let json = serde_json::to_string(&map).unwrap();
    let decoded: PropositionMapDocument = serde_json::from_str(&json).unwrap();
    decoded.validate().unwrap();
    assert_eq!(decoded, map);
    assert_eq!(FormulaSchemaVersion::V1.as_str(), "tl-syntax.formula/v1");
}

// Trace: TC-012, TC-013, TC-014, FR-005-AC-1, FR-005-AC-2, FR-005-AC-3, NFR-002-AC-2
#[test]
fn shared_corpus_is_complete_stable_and_self_consistent() {
    use std::{collections::BTreeSet, fs, path::PathBuf};

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let manifest_bytes = fs::read(root.join("manifest.json")).unwrap();
    let manifest: CorpusManifest = serde_json::from_slice(&manifest_bytes).unwrap();
    assert_eq!(manifest.corpus_revision, tl_syntax::CORPUS_REVISION);
    assert_eq!(manifest.formula_schema, "tl-syntax.formula/v1");
    assert_eq!(
        manifest.proposition_map_schema,
        "tl-syntax.proposition-map/v1"
    );
    assert_eq!(
        manifest.semantic_profiles,
        ["mltl.closed-trace/v1", "mltl.online-prefix/v1"]
    );

    let proposition_map_bytes = fs::read(root.join(&manifest.proposition_map)).unwrap();
    let proposition_map: PropositionMapDocument =
        serde_json::from_slice(&proposition_map_bytes).unwrap();
    proposition_map.validate().unwrap();

    let mut identities = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for fixture in &manifest.fixtures {
        assert!(identities.insert(&fixture.id), "duplicate fixture identity");
        assert!(paths.insert(&fixture.formula), "duplicate fixture path");
        classes.insert(fixture.class.as_str());

        let bytes = fs::read(root.join(&fixture.formula)).unwrap();
        let document = serde_json::from_slice::<FormulaDocument>(&bytes);
        let observed_valid = document
            .as_ref()
            .map(|document| document.validate().is_ok())
            .unwrap_or(false);
        match fixture.expected_validation.as_str() {
            "valid" => {
                assert!(observed_valid, "{} should be valid", fixture.id);
                assert!(fixture.trace.is_some(), "{} has no trace", fixture.id);
                assert!(
                    fixture.expected_horizon.is_some(),
                    "{} has no expected horizon",
                    fixture.id
                );
                assert!(
                    fixture.expected_closed_trace.is_some(),
                    "{} has no expected closed-trace result",
                    fixture.id
                );
            }
            "invalid" => assert!(!observed_valid, "{} should be invalid", fixture.id),
            other => panic!("unknown expected_validation value {other}"),
        }
    }

    assert_eq!(
        classes,
        BTreeSet::from([
            "boundary",
            "large-bound",
            "malformed",
            "nested",
            "primitive",
            "short-trace",
        ])
    );
}
