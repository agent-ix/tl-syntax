//! FR-034 signal direction on the signal catalog (TM-008, TC-180..TC-184).
#![cfg(feature = "serde")]

use std::collections::BTreeSet;

use jsonschema::{Draft, JSONSchema};
use proptest::prelude::*;
use tl_syntax::{
    Formula, FormulaBindingError, IntegerSignalDomain, Node, NodeId, NodeKind,
    OwnedSignalDeclaration, PropositionBinding, PropositionId, SemanticProfile, SignalCatalog,
    SignalCatalogDocument, SignalCatalogError, SignalCatalogSchemaVersion, SignalDeclaration,
    SignalDirection, SignalDomain, SignalId, SignalPartitionError, SyntaxArtifactLimits,
    SIGNAL_CATALOG_V1_SCHEMA, SIGNAL_CATALOG_V1_SCHEMA_BYTES, SIGNAL_CATALOG_V1_SCHEMA_SHA256,
    SIGNAL_CATALOG_V2_SCHEMA, SIGNAL_CATALOG_V2_SCHEMA_BYTES,
};

const V1_FIXTURE: &[u8] = include_bytes!("fixtures/valid-signal-catalog.json");

fn read(bytes: &[u8]) -> Result<SignalCatalogDocument, tl_syntax::StrictDocumentReadError> {
    SignalCatalogDocument::from_json_bytes(bytes, SyntaxArtifactLimits::default())
}

fn proposition(id: u32) -> Node {
    Node::new(NodeKind::Proposition {
        proposition: PropositionId(id),
    })
}

fn schema(text: &str) -> JSONSchema {
    let value: serde_json::Value = serde_json::from_str(text).unwrap();
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&value)
        .unwrap()
}

/// TC-180: every checked-in v1 catalog still validates, keeps its bytes and
/// identity, and reads every signal as undeclared.
// Trace: TC-180, FR-034-AC-1
#[test]
fn v1_catalog_fixtures_are_unchanged_and_read_as_undeclared() {
    let document: SignalCatalogDocument = serde_json::from_slice(V1_FIXTURE).unwrap();
    let canonical = document.canonical_json_bytes().unwrap();
    let reread = read(&canonical).unwrap();
    assert_eq!(reread, document);
    assert_eq!(reread.schema_version(), SignalCatalogSchemaVersion::V1);
    assert_eq!(reread.canonical_json_bytes().unwrap(), canonical);
    assert_eq!(
        reread.content_identity().unwrap(),
        document.content_identity().unwrap()
    );
    // The canonical v1 bytes are exactly the pre-FR-034 owner encoding.
    assert_eq!(
        String::from_utf8(canonical).unwrap(),
        r#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{"id":1,"name":"request_ready","domain":{"kind":"boolean"}},{"id":2,"name":"retry_count","domain":{"kind":"integer","minimum":0,"maximum":8}},{"id":3,"name":"temperature","domain":{"kind":"fixed_decimal","minimum_coefficient":-400,"maximum_coefficient":1250,"scale":1}}],"bindings":[{"proposition":7,"signal":1}]}"#
    );
    for signal in reread.signals() {
        assert_eq!(signal.direction(), SignalDirection::Undeclared);
    }
    let catalog = reread.validate().unwrap();
    for signal in catalog.signals() {
        assert_eq!(signal.direction(), SignalDirection::Undeclared);
    }
    assert_eq!(
        catalog.proposition_direction(PropositionId(7)),
        Some(SignalDirection::Undeclared)
    );
    assert_eq!(catalog.proposition_direction(PropositionId(8)), None);
}

fn v2_signals() -> Vec<OwnedSignalDeclaration> {
    vec![
        OwnedSignalDeclaration::new(SignalId(1), "door".into(), SignalDomain::Boolean)
            .with_direction(SignalDirection::Input),
        OwnedSignalDeclaration::new(SignalId(2), "alarm".into(), SignalDomain::Boolean)
            .with_direction(SignalDirection::Output),
        OwnedSignalDeclaration::new(SignalId(3), "spare".into(), SignalDomain::Boolean),
        OwnedSignalDeclaration::new(
            SignalId(4),
            "level".into(),
            SignalDomain::Integer(IntegerSignalDomain::new(0, 9).unwrap()),
        )
        .with_direction(SignalDirection::Input),
    ]
}

fn v2_bindings() -> Vec<PropositionBinding> {
    vec![
        PropositionBinding::new(PropositionId(1), SignalId(1)),
        PropositionBinding::new(PropositionId(2), SignalId(2)),
        PropositionBinding::new(PropositionId(3), SignalId(3)),
    ]
}

/// TC-181: v2 round-trips mixed directions; every invalid form is refused.
// Trace: TC-181, FR-034-AC-2
#[test]
fn v2_round_trips_directions_and_refuses_invalid_forms() {
    let document = SignalCatalogDocument::new_v2(v2_signals(), v2_bindings()).unwrap();
    assert_eq!(document.schema_version(), SignalCatalogSchemaVersion::V2);
    assert_eq!(
        document.schema_version().as_str(),
        "tl-syntax.signal-catalog/v2"
    );
    let bytes = document.canonical_json_bytes().unwrap();
    assert_eq!(
        String::from_utf8(bytes.clone()).unwrap(),
        r#"{"schema_version":"tl-syntax.signal-catalog/v2","signals":[{"id":1,"name":"door","domain":{"kind":"boolean"},"direction":"input"},{"id":2,"name":"alarm","domain":{"kind":"boolean"},"direction":"output"},{"id":3,"name":"spare","domain":{"kind":"boolean"}},{"id":4,"name":"level","domain":{"kind":"integer","minimum":0,"maximum":9},"direction":"input"}],"bindings":[{"proposition":1,"signal":1},{"proposition":2,"signal":2},{"proposition":3,"signal":3}]}"#
    );
    let reread = read(&bytes).unwrap();
    assert_eq!(reread, document);
    assert_eq!(reread.canonical_json_bytes().unwrap(), bytes);
    let directions: Vec<_> = reread.signals().iter().map(|s| s.direction()).collect();
    assert_eq!(
        directions,
        [
            SignalDirection::Input,
            SignalDirection::Output,
            SignalDirection::Undeclared,
            SignalDirection::Input
        ]
    );
    // v1 and v2 identities of equal content are distinct.
    let plain = SignalCatalogDocument::new_v2(
        vec![OwnedSignalDeclaration::new(
            SignalId(1),
            "door".into(),
            SignalDomain::Boolean,
        )],
        vec![],
    )
    .unwrap();
    let plain_v1 = SignalCatalogDocument::new(
        vec![OwnedSignalDeclaration::new(
            SignalId(1),
            "door".into(),
            SignalDomain::Boolean,
        )],
        vec![],
    )
    .unwrap();
    assert_ne!(
        plain.content_identity().unwrap(),
        plain_v1.content_identity().unwrap()
    );

    let text = String::from_utf8(bytes).unwrap();
    for refused in [
        text.replacen("\"direction\":\"input\"", "\"direction\":\"sideways\"", 1),
        text.replacen("\"direction\":\"input\"", "\"direction\":\"undeclared\"", 1),
        text.replacen(
            "\"direction\":\"input\"",
            "\"direction\":\"input\",\"direction\":\"input\"",
            1,
        ),
        text.replacen("\"direction\":\"input\"", "\"direction\":\"Input\"", 1),
        text.replacen("\"direction\":\"input\"", "\"direction\":null", 1),
    ] {
        assert!(read(refused.as_bytes()).is_err(), "{refused}");
    }
    let v1_with_direction = r#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{"id":1,"name":"door","domain":{"kind":"boolean"},"direction":"input"}],"bindings":[]}"#;
    assert!(read(v1_with_direction.as_bytes()).is_err());
    assert_eq!(
        SignalCatalogDocument::new(v2_signals(), v2_bindings()),
        Err(SignalCatalogError::DirectionInV1Document {
            signal: SignalId(1)
        })
    );
    // A v2 document without any direction is valid and all undeclared.
    let bare = r#"{"schema_version":"tl-syntax.signal-catalog/v2","signals":[{"id":1,"name":"door","domain":{"kind":"boolean"}}],"bindings":[]}"#;
    let bare = read(bare.as_bytes()).unwrap();
    assert_eq!(bare.signals()[0].direction(), SignalDirection::Undeclared);
}

fn direction_strategy() -> impl Strategy<Value = SignalDirection> {
    prop_oneof![Just(SignalDirection::Input), Just(SignalDirection::Output)]
}

proptest! {
    /// TC-182: partitions of generated fully declared catalogs are disjoint,
    /// increasing, complete and faithful; `partition_all` is the union.
    // Trace: TC-182, FR-034-AC-3
    #[test]
    fn partition_is_disjoint_increasing_complete_and_faithful(
        directions in proptest::collection::vec(direction_strategy(), 1..8),
        first in proptest::collection::vec(0_usize..8, 1..10),
        second in proptest::collection::vec(0_usize..8, 1..10),
    ) {
        let names: Vec<String> = (0..directions.len()).map(|index| format!("s{index}")).collect();
        let signals: Vec<SignalDeclaration<'_>> = directions
            .iter()
            .enumerate()
            .map(|(index, direction)| {
                SignalDeclaration::new(SignalId(index as u32), &names[index], SignalDomain::Boolean)
                    .with_direction(*direction)
            })
            .collect();
        let bindings: Vec<PropositionBinding> = (0..directions.len())
            .map(|index| PropositionBinding::new(PropositionId(index as u32 * 2), SignalId(index as u32)))
            .collect();
        let mut scratch = vec![0; signals.len()];
        let catalog = SignalCatalog::new(&signals, &bindings, &mut scratch).unwrap();
        let build = |picks: &[usize]| -> Vec<Node> {
            picks
                .iter()
                .map(|pick| proposition((*pick % directions.len()) as u32 * 2))
                .collect()
        };
        let first_nodes = build(&first);
        let second_nodes = build(&second);
        let first_formula = Formula::new(
            SemanticProfile::ClosedTraceV1,
            NodeId(0),
            &first_nodes,
        )
        .unwrap();
        let second_formula = Formula::new(
            SemanticProfile::ClosedTraceV1,
            NodeId(0),
            &second_nodes,
        )
        .unwrap();
        let single = catalog.partition(first_formula).unwrap();
        let expected = |nodes: &[&[Node]]| {
            let mut occurring = BTreeSet::new();
            for list in nodes {
                for node in *list {
                    if let NodeKind::Proposition { proposition } = node.kind {
                        occurring.insert(proposition);
                    }
                }
            }
            occurring
        };
        for (partition, occurring) in [
            (single.clone(), expected(&[&first_nodes])),
            (
                catalog.partition_all(&[first_formula, second_formula]).unwrap(),
                expected(&[&first_nodes, &second_nodes]),
            ),
        ] {
            prop_assert!(partition.inputs().windows(2).all(|pair| pair[0] < pair[1]));
            prop_assert!(partition.outputs().windows(2).all(|pair| pair[0] < pair[1]));
            let inputs: BTreeSet<_> = partition.inputs().iter().copied().collect();
            let outputs: BTreeSet<_> = partition.outputs().iter().copied().collect();
            prop_assert!(inputs.is_disjoint(&outputs));
            let union: BTreeSet<_> = inputs.union(&outputs).copied().collect();
            prop_assert_eq!(&union, &occurring);
            for proposition in &inputs {
                prop_assert_eq!(catalog.proposition_direction(*proposition), Some(SignalDirection::Input));
            }
            for proposition in &outputs {
                prop_assert_eq!(catalog.proposition_direction(*proposition), Some(SignalDirection::Output));
            }
        }
    }
}

/// TC-183: the first undeclared referenced signal is refused in node order,
/// unreferenced undeclared signals are ignored, binding errors are preserved.
// Trace: TC-183, FR-034-AC-4
#[test]
fn partition_refuses_first_undeclared_and_keeps_binding_errors() {
    let signals = [
        SignalDeclaration::new(SignalId(1), "door", SignalDomain::Boolean)
            .with_direction(SignalDirection::Input),
        SignalDeclaration::new(SignalId(2), "spare", SignalDomain::Boolean),
        SignalDeclaration::new(SignalId(3), "other", SignalDomain::Boolean),
        SignalDeclaration::new(SignalId(4), "alarm", SignalDomain::Boolean)
            .with_direction(SignalDirection::Output),
    ];
    let bindings = [
        PropositionBinding::new(PropositionId(1), SignalId(1)),
        PropositionBinding::new(PropositionId(2), SignalId(2)),
        PropositionBinding::new(PropositionId(3), SignalId(3)),
        PropositionBinding::new(PropositionId(4), SignalId(4)),
    ];
    let mut scratch = [0; 4];
    let catalog = SignalCatalog::new(&signals, &bindings, &mut scratch).unwrap();

    // Only declared signals referenced: accepted although 2 and 3 are undeclared.
    let declared = [
        proposition(1),
        proposition(4),
        Node::new(NodeKind::And {
            left: NodeId(0),
            right: NodeId(1),
        }),
    ];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(2), &declared).unwrap();
    let partition = catalog.partition(formula).unwrap();
    assert_eq!(partition.inputs(), &[PropositionId(1)]);
    assert_eq!(partition.outputs(), &[PropositionId(4)]);

    // Node order decides which undeclared proposition is named: 3 before 2.
    let undeclared = [
        proposition(1),
        proposition(3),
        proposition(2),
        Node::new(NodeKind::And {
            left: NodeId(1),
            right: NodeId(2),
        }),
    ];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(3), &undeclared).unwrap();
    let expected = SignalPartitionError::UndeclaredDirection {
        proposition: PropositionId(3),
        signal: SignalId(3),
    };
    assert_eq!(catalog.partition(formula), Err(expected));
    let fine = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(2), &declared).unwrap();
    assert_eq!(catalog.partition_all(&[fine, formula]), Err(expected));

    let missing = [proposition(1), proposition(9)];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &missing).unwrap();
    let binding_error = FormulaBindingError::MissingPropositionBinding {
        proposition: PropositionId(9),
    };
    assert_eq!(
        catalog.partition(formula),
        Err(SignalPartitionError::Binding(binding_error))
    );
    assert_eq!(catalog.bind_formula(formula).unwrap_err(), binding_error);
}

/// TC-184: v2 schema bytes are embedded, v1 schema bytes and digest unchanged,
/// and the v2 schema agrees with the semantic reader on structure.
// Trace: TC-184, FR-034-AC-5
#[test]
fn v2_schema_is_embedded_and_agrees_with_the_reader() {
    assert_eq!(
        SIGNAL_CATALOG_V2_SCHEMA_BYTES,
        include_bytes!("../spec/signal-catalog-v2.schema.json")
    );
    assert_eq!(SIGNAL_CATALOG_V2_SCHEMA.as_bytes(), SIGNAL_CATALOG_V2_SCHEMA_BYTES);
    assert_eq!(
        SIGNAL_CATALOG_V1_SCHEMA_BYTES,
        include_bytes!("../spec/signal-catalog-v1.schema.json")
    );
    assert_eq!(SIGNAL_CATALOG_V1_SCHEMA.as_bytes(), SIGNAL_CATALOG_V1_SCHEMA_BYTES);
    assert_eq!(
        SIGNAL_CATALOG_V1_SCHEMA_SHA256,
        "24fe7ccc26918e9a52dfa49a0e670ef932d0bfcd155d5d73dfd0be23dd276df8"
    );
    let v2 = schema(SIGNAL_CATALOG_V2_SCHEMA);
    let v1 = schema(SIGNAL_CATALOG_V1_SCHEMA);
    let accepted = SignalCatalogDocument::new_v2(v2_signals(), v2_bindings())
        .unwrap()
        .canonical_json_bytes()
        .unwrap();
    let accepted_value: serde_json::Value = serde_json::from_slice(&accepted).unwrap();
    assert!(v2.is_valid(&accepted_value));
    assert!(!v1.is_valid(&accepted_value));
    let text = String::from_utf8(accepted).unwrap();
    for structural in [
        text.replacen("\"direction\":\"input\"", "\"direction\":\"undeclared\"", 1),
        text.replacen("\"direction\":\"input\"", "\"direction\":\"sideways\"", 1),
        text.replacen("\"direction\":\"input\"", "\"orientation\":\"input\"", 1),
        text.replacen("signal-catalog/v2", "signal-catalog/v3", 1),
    ] {
        let value: serde_json::Value = serde_json::from_str(&structural).unwrap();
        assert!(!v2.is_valid(&value), "{structural}");
        assert!(read(structural.as_bytes()).is_err(), "{structural}");
    }
    let v1_value: serde_json::Value = serde_json::from_slice(V1_FIXTURE).unwrap();
    assert!(v1.is_valid(&v1_value));
}
