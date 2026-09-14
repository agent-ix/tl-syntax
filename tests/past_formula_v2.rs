#![cfg(feature = "serde")]

use proptest::prelude::*;
use tl_syntax::{
    Formula, FormulaConversionError, FormulaDocument, FormulaError, FormulaSchemaVersion,
    FutureLoweringRefusal, FutureLoweringRequest, Interval, Node, NodeId, NodeKind, OperatorArity,
    PastOperatorKind, PropositionId, RawBounds, SemanticProfile, TemporalFamily,
    FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1, MAX_FORMULA_DOCUMENT_DEPTH, PAST_OPERATORS_V1,
};

fn past_nodes() -> Vec<Node> {
    let interval = Interval::new(0, u32::MAX).unwrap();
    vec![
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(7),
        }),
        Node::new(NodeKind::Once {
            interval,
            operand: NodeId(0),
        }),
        Node::new(NodeKind::Historically {
            interval,
            operand: NodeId(1),
        }),
        Node::new(NodeKind::StrongPrevious { operand: NodeId(2) }),
        Node::new(NodeKind::Since {
            interval,
            left: NodeId(2),
            right: NodeId(3),
        }),
        Node::new(NodeKind::Triggered {
            interval,
            left: NodeId(3),
            right: NodeId(4),
        }),
    ]
}

// Trace: TC-054, FR-011-AC-1, FR-011-AC-5, FR-013-AC-1
#[test]
fn formula_v2_round_trips_the_closed_past_profile_and_catalog() {
    assert_eq!(PAST_OPERATORS_V1, "tl-syntax.past-operators/v1");
    assert_eq!(
        SemanticProfile::OriginCompleteHistoryV1.as_str(),
        "mltl.origin-complete-history/v1"
    );
    assert_eq!(
        PastOperatorKind::ALL,
        [
            PastOperatorKind::Once,
            PastOperatorKind::Historically,
            PastOperatorKind::StrongPrevious,
            PastOperatorKind::Since,
            PastOperatorKind::Triggered,
        ]
    );
    assert_eq!(PastOperatorKind::Once.semantic_name(), "Once");
    assert_eq!(
        PastOperatorKind::Historically.semantic_name(),
        "Historically"
    );
    assert_eq!(
        PastOperatorKind::StrongPrevious.semantic_name(),
        "StrongPrevious"
    );
    assert_eq!(PastOperatorKind::Since.semantic_name(), "Since");
    assert_eq!(PastOperatorKind::Triggered.semantic_name(), "Triggered");
    assert_eq!(
        PastOperatorKind::StrongPrevious.arity(),
        OperatorArity::Unary
    );
    assert_eq!(PastOperatorKind::Since.arity(), OperatorArity::Binary);

    let document = FormulaDocument::new_v2(
        SemanticProfile::OriginCompleteHistoryV1,
        NodeId(5),
        past_nodes(),
    )
    .unwrap();
    let encoded = serde_json::to_string(&document).unwrap();
    for tag in [
        "once",
        "historically",
        "strong_previous",
        "since",
        "triggered",
    ] {
        assert!(encoded.contains(&format!(r#""kind":"{tag}""#)));
    }
    let decoded: FormulaDocument = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, document);
    assert_eq!(decoded.schema_version(), FormulaSchemaVersion::V2);
    assert_eq!(
        decoded.validate().unwrap().profile(),
        SemanticProfile::OriginCompleteHistoryV1
    );
}

// Trace: TC-054, FR-013-AC-1
#[test]
fn formula_profiles_fail_closed_without_changing_formula_v1() {
    let past = past_nodes();
    assert_eq!(
        Formula::new(SemanticProfile::ClosedTraceV1, NodeId(5), &past),
        Err(FormulaError::ProfileIncompatibleNode {
            profile: SemanticProfile::ClosedTraceV1,
            node: NodeId(1),
            family: TemporalFamily::Past,
        })
    );

    let future = vec![
        Node::new(NodeKind::True),
        Node::new(NodeKind::Future {
            interval: Interval::new(0, 1).unwrap(),
            operand: NodeId(0),
        }),
    ];
    assert_eq!(
        Formula::new(SemanticProfile::OriginCompleteHistoryV1, NodeId(1), &future),
        Err(FormulaError::ProfileIncompatibleNode {
            profile: SemanticProfile::OriginCompleteHistoryV1,
            node: NodeId(1),
            family: TemporalFamily::Future,
        })
    );

    assert_eq!(
        FormulaDocument::new(
            SemanticProfile::OriginCompleteHistoryV1,
            NodeId(5),
            past.clone(),
        ),
        Err(FormulaError::FormulaV1ProfileUnsupported {
            profile: SemanticProfile::OriginCompleteHistoryV1,
        })
    );
    assert_eq!(
        FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(5), past),
        Err(FormulaError::FormulaV1NodeUnsupported {
            node: NodeId(1),
            operator: PastOperatorKind::Once,
        })
    );

    let legacy = FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(1), future).unwrap();
    assert_eq!(
        serde_json::to_string(&legacy).unwrap(),
        r#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"},{"kind":"future","interval":{"start":0,"end":1},"operand":0}]}"#
    );
}

// Trace: TC-054, FR-013-AC-1
#[test]
fn v1_upgrade_is_lossless_and_v2_down_conversion_is_guarded() {
    let v1 = FormulaDocument::new(
        SemanticProfile::OnlinePrefixV1,
        NodeId(1),
        vec![
            Node::new(NodeKind::True),
            Node::new(NodeKind::Future {
                interval: Interval::new(2, 3).unwrap(),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();
    let v2 = v1.to_v2();
    assert_eq!(v2.schema_version(), FormulaSchemaVersion::V2);
    assert_eq!(v2.semantic_profile(), v1.semantic_profile());
    assert_eq!(v2.root(), v1.root());
    assert_eq!(v2.nodes(), v1.nodes());
    assert_eq!(v2.try_to_v1().unwrap(), v1);

    let past = FormulaDocument::new_v2(
        SemanticProfile::OriginCompleteHistoryV1,
        NodeId(5),
        past_nodes(),
    )
    .unwrap();
    assert_eq!(
        past.try_to_v1(),
        Err(FormulaConversionError::UnsupportedSemanticProfile {
            profile: SemanticProfile::OriginCompleteHistoryV1,
        })
    );
}

// Trace: TC-054, FR-013-AC-1
#[test]
fn existing_future_lowering_refuses_the_new_past_profile() {
    let nodes = [Node::new(NodeKind::True)];
    let formula =
        Formula::new(SemanticProfile::OriginCompleteHistoryV1, NodeId(0), &nodes).unwrap();
    let request = FutureLoweringRequest {
        request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
        operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
        kind: b"W",
        semantic_profile: SemanticProfile::OriginCompleteHistoryV1.as_str().as_bytes(),
        formula,
        left: 0,
        right: 0,
        interval: Some(RawBounds::new(0, 0)),
        operator_span: None,
        expression_span: None,
    };
    assert_eq!(
        request.lower(),
        Err(FutureLoweringRefusal::UnknownSemanticProfile)
    );
}

// Trace: TC-054, TC-057, FR-013-AC-1
#[test]
fn v2_wire_refuses_unknown_malformed_and_profile_incompatible_values() {
    let documents = [
        r#"{"schema_version":"tl-syntax.formula/v3","semantic_profile":"mltl.origin-complete-history/v1","root":0,"nodes":[{"kind":"true"}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v2","root":0,"nodes":[{"kind":"true"}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v1","root":0,"nodes":[{"kind":"previous","operand":0}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v1","root":1,"nodes":[{"kind":"true"},{"kind":"once","interval":{"start":2,"end":1},"operand":0}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v1","root":1,"nodes":[{"kind":"true"},{"kind":"once","interval":{"start":0,"end":1},"operand":1}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v1","root":1,"nodes":[{"kind":"true"},{"kind":"future","interval":{"start":0,"end":1},"operand":0}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"},{"kind":"once","interval":{"start":0,"end":1},"operand":0}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.origin-complete-history/v1","root":0,"nodes":[{"kind":"true"}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"},{"kind":"once","interval":{"start":0,"end":1},"operand":0}]}"#,
        r#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.origin-complete-history/v1","root":0,"nodes":[{"kind":"true","unexpected":1}]}"#,
    ];
    for document in documents {
        assert!(
            serde_json::from_str::<FormulaDocument>(document).is_err(),
            "accepted malformed document: {document}"
        );
    }
}

// Trace: TC-054, TC-057, FR-013-AC-1
#[test]
fn formula_v2_enforces_the_document_depth_limit() {
    let mut nodes = Vec::with_capacity(MAX_FORMULA_DOCUMENT_DEPTH + 1);
    nodes.push(Node::new(NodeKind::True));
    for index in 1..=MAX_FORMULA_DOCUMENT_DEPTH {
        nodes.push(Node::new(NodeKind::Not {
            operand: NodeId(u32::try_from(index - 1).unwrap()),
        }));
    }
    let legacy_nodes = nodes.clone();
    assert_eq!(
        FormulaDocument::new_v2(
            SemanticProfile::OriginCompleteHistoryV1,
            NodeId(u32::try_from(MAX_FORMULA_DOCUMENT_DEPTH).unwrap()),
            nodes,
        ),
        Err(FormulaError::DocumentDepthLimitExceeded {
            node: NodeId(u32::try_from(MAX_FORMULA_DOCUMENT_DEPTH).unwrap()),
            depth: MAX_FORMULA_DOCUMENT_DEPTH + 1,
            limit: MAX_FORMULA_DOCUMENT_DEPTH,
        })
    );
    assert!(FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(u32::try_from(MAX_FORMULA_DOCUMENT_DEPTH).unwrap()),
        legacy_nodes,
    )
    .is_ok());
}

proptest! {
    // Trace: TC-057, FR-013-AC-1
    #[test]
    fn arbitrary_bounded_input_never_unwinds_or_defaults_to_v2(bytes in prop::collection::vec(any::<u8>(), 0..8192)) {
        let decoded = serde_json::from_slice::<FormulaDocument>(&bytes);
        let accepted_document_is_valid = decoded
            .as_ref()
            .map_or(true, |document| document.validate().is_ok());
        let v1_never_defaults_to_past = decoded.as_ref().map_or(true, |document| {
            document.schema_version() != FormulaSchemaVersion::V1
                || (document.semantic_profile() != SemanticProfile::OriginCompleteHistoryV1
                    && document
                        .nodes()
                        .iter()
                        .all(|node| node.kind.temporal_family() != Some(TemporalFamily::Past)))
        });
        prop_assert!(accepted_document_is_valid);
        prop_assert!(v1_never_defaults_to_past);
    }
}
