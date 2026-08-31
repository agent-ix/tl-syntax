#![cfg(feature = "serde")]

use tl_syntax::{
    FormulaDocument, FormulaSchemaVersion, Interval, Node, NodeId, NodeKind, PropositionEntry,
    PropositionId, PropositionMapDocument, SemanticProfile, SourceSpan,
};

use proptest::prelude::*;

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
    expected_horizon: Option<u64>,
    #[serde(default)]
    expected_closed_trace: Option<bool>,
}

fn derived_horizon(document: &FormulaDocument) -> u64 {
    let mut horizons = Vec::with_capacity(document.nodes().len());
    let prior = |values: &[u64], id: NodeId| values[id.0 as usize];
    for node in document.nodes() {
        let value = match node.kind {
            NodeKind::False | NodeKind::True | NodeKind::Proposition { .. } => 0,
            NodeKind::Not { operand } => prior(&horizons, operand),
            NodeKind::And { left, right }
            | NodeKind::Or { left, right }
            | NodeKind::Implies { left, right }
            | NodeKind::Equivalent { left, right } => {
                prior(&horizons, left).max(prior(&horizons, right))
            }
            NodeKind::Future { interval, operand } | NodeKind::Globally { interval, operand } => {
                u64::from(interval.end())
                    .checked_add(prior(&horizons, operand))
                    .unwrap()
            }
            NodeKind::Until {
                interval,
                left,
                right,
            }
            | NodeKind::Release {
                interval,
                left,
                right,
            } => u64::from(interval.end())
                .checked_add(prior(&horizons, left).max(prior(&horizons, right)))
                .unwrap(),
        };
        horizons.push(value);
    }
    horizons[document.root().0 as usize]
}

proptest! {
    // Trace: TC-009, FR-004-AC-1
    #[test]
    fn generated_owned_formulas_round_trip(
        proposition in any::<u32>(),
        first_bound in any::<u32>(),
        second_bound in any::<u32>(),
        closed_trace in any::<bool>(),
    ) {
        let start = first_bound.min(second_bound);
        let end = first_bound.max(second_bound);
        let profile = if closed_trace {
            SemanticProfile::ClosedTraceV1
        } else {
            SemanticProfile::OnlinePrefixV1
        };
        let document = FormulaDocument::new(
            profile,
            NodeId(1),
            vec![
                Node::new(NodeKind::Proposition {
                    proposition: PropositionId(proposition),
                }),
                Node::new(NodeKind::Future {
                    interval: Interval::new(start, end).unwrap(),
                    operand: NodeId(0),
                }),
            ],
        )
        .unwrap();
        let encoded = serde_json::to_vec(&document).unwrap();
        let decoded: FormulaDocument = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, document);
        decoded.validate().unwrap();
    }
}

// Trace: TC-008, TC-009, FR-003-AC-3, FR-004-AC-1, StR-002-VC-1
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

// Trace: TC-017, FR-004-AC-1
#[test]
fn every_supported_node_variant_round_trips_with_its_stable_wire_tag() {
    let interval = Interval::new(2, 5).unwrap();
    let document = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(11),
        vec![
            Node::new(NodeKind::False),
            Node::new(NodeKind::True),
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(17),
            }),
            Node::new(NodeKind::Not { operand: NodeId(2) }),
            Node::new(NodeKind::And {
                left: NodeId(1),
                right: NodeId(3),
            }),
            Node::new(NodeKind::Or {
                left: NodeId(0),
                right: NodeId(4),
            }),
            Node::new(NodeKind::Implies {
                left: NodeId(4),
                right: NodeId(5),
            }),
            Node::new(NodeKind::Equivalent {
                left: NodeId(5),
                right: NodeId(6),
            }),
            Node::new(NodeKind::Future {
                interval,
                operand: NodeId(7),
            }),
            Node::new(NodeKind::Globally {
                interval,
                operand: NodeId(8),
            }),
            Node::new(NodeKind::Until {
                interval,
                left: NodeId(8),
                right: NodeId(9),
            }),
            Node::new(NodeKind::Release {
                interval,
                left: NodeId(9),
                right: NodeId(10),
            }),
        ],
    )
    .unwrap();

    let encoded = serde_json::to_string(&document).unwrap();
    let decoded: FormulaDocument = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, document, "round-trip lost or reordered a node");
    decoded.validate().unwrap();

    for kind in [
        "false",
        "true",
        "proposition",
        "not",
        "and",
        "or",
        "implies",
        "equivalent",
        "future",
        "globally",
        "until",
        "release",
    ] {
        assert!(
            encoded.contains(&format!(r#""kind":"{kind}""#)),
            "missing stable wire tag {kind}"
        );
    }
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
    assert!(serde_json::from_str::<FormulaDocument>(forward_reference).is_err());
}

// Trace: TC-020, FR-004-AC-4
#[test]
fn formula_wire_decode_stops_at_the_document_node_limit() {
    let mut formula = String::from(
        r#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":["#,
    );
    for index in 0..=tl_syntax::MAX_FORMULA_DOCUMENT_NODES {
        if index != 0 {
            formula.push(',');
        }
        formula.push_str(r#"{"kind":"true"}"#);
    }
    formula.push_str("]}");
    let error = serde_json::from_str::<FormulaDocument>(&formula).unwrap_err();
    assert!(error.to_string().contains("100000-node wire limit"));
}

// Trace: TC-011, FR-004-AC-3
#[test]
fn unknown_node_fields_and_invalid_proposition_maps_are_rejected_on_decode() {
    let unknown_node_field = r#"{
        "schema_version":"tl-syntax.formula/v1",
        "semantic_profile":"mltl.closed-trace/v1",
        "root":0,
        "nodes":[{"kind":"true","unexpected":1}]
    }"#;
    assert!(serde_json::from_str::<FormulaDocument>(unknown_node_field).is_err());

    let duplicate_name = r#"{
        "schema_version":"tl-syntax.proposition-map/v1",
        "propositions":[
          {"id":0,"name":"same"},
          {"id":1,"name":"same"}
        ]
    }"#;
    assert!(serde_json::from_str::<PropositionMapDocument>(duplicate_name).is_err());
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

// Trace: TC-012, TC-013, TC-014, FR-005-AC-1, FR-005-AC-2, FR-005-AC-3, NFR-002-AC-2, StR-002-VC-2
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
                let document = document.unwrap();
                assert!(fixture.trace.is_some(), "{} has no trace", fixture.id);
                assert_eq!(
                    fixture.expected_horizon,
                    Some(derived_horizon(&document)),
                    "{} has a non-derived expected horizon",
                    fixture.id,
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
