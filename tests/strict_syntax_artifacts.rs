#![cfg(feature = "serde")]

use sha2::{Digest, Sha256};
use tl_syntax::{
    FormulaDocument, FormulaError, Interval, Node, NodeId, NodeKind, OwnedSignalDeclaration,
    PropositionBinding, PropositionEntry, PropositionId, PropositionMapDocument, SemanticProfile,
    SignalCatalogDocument, SignalDomain, SignalId, StrictDocumentReadError, SyntaxArtifactLimits,
    FORMULA_V1_SCHEMA_BYTES, FORMULA_V1_SCHEMA_SHA256, FORMULA_V2_SCHEMA_BYTES,
    FORMULA_V2_SCHEMA_SHA256, PROPOSITION_MAP_V1_SCHEMA_BYTES, PROPOSITION_MAP_V1_SCHEMA_SHA256,
    SIGNAL_CATALOG_V1_SCHEMA_BYTES, SIGNAL_CATALOG_V1_SCHEMA_SHA256,
};

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn formula_v1() -> FormulaDocument {
    FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(0),
        vec![Node::new(NodeKind::True)],
    )
    .unwrap()
}

fn formula_v2(kind: NodeKind) -> FormulaDocument {
    FormulaDocument::new_v2(
        SemanticProfile::OriginCompleteHistoryV1,
        NodeId(2),
        vec![
            Node::new(NodeKind::True),
            Node::new(NodeKind::False),
            Node::new(kind),
        ],
    )
    .unwrap()
}

fn signal_catalog() -> SignalCatalogDocument {
    SignalCatalogDocument::new(
        vec![OwnedSignalDeclaration::new(
            SignalId(7),
            "ready".into(),
            SignalDomain::Boolean,
        )],
        vec![PropositionBinding::new(PropositionId(3), SignalId(7))],
    )
    .unwrap()
}

fn proposition_map() -> PropositionMapDocument {
    PropositionMapDocument::new(vec![PropositionEntry {
        id: PropositionId(3),
        name: "ready".into(),
    }])
    .unwrap()
}

// Trace: TC-075, FR-014-AC-1
#[test]
fn schema_bytes_and_digests_are_exact_immutable_owner_artifacts() {
    assert_eq!(sha256(FORMULA_V1_SCHEMA_BYTES), FORMULA_V1_SCHEMA_SHA256);
    assert_eq!(sha256(FORMULA_V2_SCHEMA_BYTES), FORMULA_V2_SCHEMA_SHA256);
    assert_eq!(
        sha256(SIGNAL_CATALOG_V1_SCHEMA_BYTES),
        SIGNAL_CATALOG_V1_SCHEMA_SHA256
    );
    assert_eq!(
        sha256(PROPOSITION_MAP_V1_SCHEMA_BYTES),
        PROPOSITION_MAP_V1_SCHEMA_SHA256
    );

    for bytes in [FORMULA_V1_SCHEMA_BYTES, FORMULA_V2_SCHEMA_BYTES] {
        let schema: serde_json::Value = serde_json::from_slice(bytes).unwrap();
        assert_eq!(schema["additionalProperties"], false);
    }
}

// Trace: TC-075, FR-014-AC-1, FR-014-AC-3
#[test]
fn all_owner_documents_round_trip_only_their_canonical_bytes() {
    let limits = SyntaxArtifactLimits::default();
    let _: tl_syntax::formula::profile::SemanticProfile = SemanticProfile::ClosedTraceV1;
    let _: tl_syntax::signal::domain::SignalDomain = SignalDomain::Boolean;
    let _: tl_syntax::signal::binding::PropositionBinding =
        PropositionBinding::new(PropositionId(3), SignalId(7));

    let formula = formula_v1();
    let formula_bytes = formula.canonical_json_bytes().unwrap();
    assert_eq!(
        FormulaDocument::from_json_bytes(&formula_bytes, limits).unwrap(),
        formula
    );

    let signal = signal_catalog();
    let signal_bytes = signal.canonical_json_bytes().unwrap();
    assert_eq!(
        SignalCatalogDocument::from_json_bytes(&signal_bytes, limits).unwrap(),
        signal
    );

    let map = proposition_map();
    let map_bytes = map.canonical_json_bytes().unwrap();
    assert_eq!(
        PropositionMapDocument::from_json_bytes(&map_bytes, limits).unwrap(),
        map
    );

    let mut noncanonical = formula_bytes.clone();
    noncanonical.push(b'\n');
    assert!(matches!(
        FormulaDocument::from_json_bytes(&noncanonical, limits),
        Err(StrictDocumentReadError::NonCanonicalDocument)
    ));
    let reordered = br#"{"semantic_profile":"mltl.closed-trace/v1","schema_version":"tl-syntax.formula/v1","root":0,"nodes":[{"kind":"true"}]}"#;
    assert!(matches!(
        FormulaDocument::from_json_bytes(reordered, limits),
        Err(StrictDocumentReadError::NonCanonicalDocument)
    ));
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn strict_formula_reader_refuses_each_closed_wire_failure_class() {
    let limits = SyntaxArtifactLimits::default();
    for bytes in [
        br#"{"schema_version":"tl-syntax.formula/v1","schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"true"}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"true"}],"unknown":0}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","nodes":[{"kind":"true"}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.formula/v9","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"true"}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"},{"kind":"not","operand":1}]}"#.as_slice(),
        b"\xff".as_slice(),
    ] {
        assert!(FormulaDocument::from_json_bytes(bytes, limits).is_err());
    }
}

// Trace: TC-075, FR-014-AC-2, FR-014-AC-3
#[test]
fn strict_signal_and_map_readers_refuse_wire_and_semantic_mutations() {
    let limits = SyntaxArtifactLimits::default();
    for bytes in [
        br#"{"signals":[],"schema_version":"tl-syntax.signal-catalog/v1","bindings":[]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{"id":1,"name":"count","domain":{"kind":"integer","minimum":2,"maximum":1}}],"bindings":[]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[],"bindings":[{"proposition":0,"signal":9}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[],"bindings":[],"extra":0}"#.as_slice(),
    ] {
        assert!(SignalCatalogDocument::from_json_bytes(bytes, limits).is_err());
    }

    for bytes in [
        br#"{"propositions":[],"schema_version":"tl-syntax.proposition-map/v1"}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{"name":"ready","id":3}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{"id":3,"name":"ready"},{"id":3,"name":"other"}]}"#.as_slice(),
        br#"{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{"id":3,"name":"ready"},{"id":4,"name":"ready"}]}"#.as_slice(),
    ] {
        assert!(PropositionMapDocument::from_json_bytes(bytes, limits).is_err());
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn caller_limits_lower_but_never_raise_owner_maxima() {
    let formula = formula_v1();
    let formula_bytes = formula.canonical_json_bytes().unwrap();

    let exact = SyntaxArtifactLimits {
        document_bytes: formula_bytes.len(),
        formula_nodes: 1,
        formula_depth: 1,
        work: formula_bytes.len() * 3 + 1,
        ..SyntaxArtifactLimits::default()
    };
    assert!(FormulaDocument::from_json_bytes(&formula_bytes, exact).is_ok());

    let exact_shape = SyntaxArtifactLimits {
        json_depth: 3,
        string_bytes: 20,
        ..SyntaxArtifactLimits::default()
    };
    assert!(FormulaDocument::from_json_bytes(&formula_bytes, exact_shape).is_ok());
    let mut one_under_shape = exact_shape;
    one_under_shape.json_depth -= 1;
    assert!(matches!(
        FormulaDocument::from_json_bytes(&formula_bytes, one_under_shape),
        Err(StrictDocumentReadError::DepthLimitExceeded { .. })
    ));
    one_under_shape = exact_shape;
    one_under_shape.string_bytes -= 1;
    assert!(matches!(
        FormulaDocument::from_json_bytes(&formula_bytes, one_under_shape),
        Err(StrictDocumentReadError::StringTooLarge { .. })
    ));

    let mut one_under = exact;
    one_under.document_bytes -= 1;
    assert!(matches!(
        FormulaDocument::from_json_bytes(&formula_bytes, one_under),
        Err(StrictDocumentReadError::DocumentTooLarge { .. })
    ));
    one_under = exact;
    one_under.formula_nodes = 0;
    assert!(matches!(
        FormulaDocument::from_json_bytes(&formula_bytes, one_under),
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "formula nodes",
            actual: 1,
            limit: 0
        })
    ));
    one_under = exact;
    one_under.work -= 1;
    assert!(matches!(
        FormulaDocument::from_json_bytes(&formula_bytes, one_under),
        Err(StrictDocumentReadError::WorkLimitExceeded { .. })
    ));

    let map = proposition_map();
    let map_bytes = map.canonical_json_bytes().unwrap();
    let map_limit = SyntaxArtifactLimits {
        propositions: 0,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        PropositionMapDocument::from_json_bytes(&map_bytes, map_limit),
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "propositions",
            actual: 1,
            limit: 0
        })
    ));

    let signal = signal_catalog();
    let signal_bytes = signal.canonical_json_bytes().unwrap();
    let signal_limit = SyntaxArtifactLimits {
        signals: 0,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        SignalCatalogDocument::from_json_bytes(&signal_bytes, signal_limit),
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "signals",
            actual: 1,
            limit: 0
        })
    ));
    let binding_limit = SyntaxArtifactLimits {
        bindings: 0,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        SignalCatalogDocument::from_json_bytes(&signal_bytes, binding_limit),
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "bindings",
            actual: 1,
            limit: 0
        })
    ));

    let cannot_raise = SyntaxArtifactLimits {
        document_bytes: usize::MAX,
        ..SyntaxArtifactLimits::default()
    };
    let oversized = vec![b' '; SyntaxArtifactLimits::OWNER_MAXIMA.document_bytes + 1];
    assert!(matches!(
        FormulaDocument::from_json_bytes(&oversized, cannot_raise),
        Err(StrictDocumentReadError::DocumentTooLarge {
            limit,
            ..
        }) if limit == SyntaxArtifactLimits::OWNER_MAXIMA.document_bytes
    ));
}

// Trace: TC-075, FR-014-AC-4
#[test]
fn formula_v2_is_exhaustive_for_profile_partitioned_temporal_graphs() {
    let interval = Interval::new(0, 3).unwrap();
    let operators = [
        NodeKind::Once {
            interval,
            operand: NodeId(0),
        },
        NodeKind::Historically {
            interval,
            operand: NodeId(0),
        },
        NodeKind::StrongPrevious { operand: NodeId(0) },
        NodeKind::Since {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        NodeKind::Triggered {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
    ];
    for operator in operators {
        let document = formula_v2(operator);
        let bytes = document.canonical_json_bytes().unwrap();
        assert_eq!(
            FormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).unwrap(),
            document
        );
    }

    for profile in [
        SemanticProfile::ClosedTraceV1,
        SemanticProfile::OnlinePrefixV1,
    ] {
        let future = FormulaDocument::new_v2(
            profile,
            NodeId(1),
            vec![
                Node::new(NodeKind::True),
                Node::new(NodeKind::Future {
                    interval,
                    operand: NodeId(0),
                }),
            ],
        )
        .unwrap();
        let bytes = future.canonical_json_bytes().unwrap();
        assert_eq!(
            FormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).unwrap(),
            future
        );
    }

    assert!(matches!(
        FormulaDocument::new_v2(
            SemanticProfile::OriginCompleteHistoryV1,
            NodeId(1),
            vec![
                Node::new(NodeKind::True),
                Node::new(NodeKind::Future {
                    interval,
                    operand: NodeId(0),
                }),
            ],
        ),
        Err(FormulaError::ProfileIncompatibleNode { .. })
    ));
    assert!(matches!(
        FormulaDocument::new_v2(
            SemanticProfile::ClosedTraceV1,
            NodeId(1),
            vec![
                Node::new(NodeKind::True),
                Node::new(NodeKind::Once {
                    interval,
                    operand: NodeId(0),
                }),
            ],
        ),
        Err(FormulaError::ProfileIncompatibleNode { .. })
    ));
}

// Trace: TC-075, FR-014-AC-1, FR-014-AC-2
#[test]
fn content_identity_is_separate_domain_bound_and_stable() {
    let document = formula_v1();
    let bytes = document.canonical_json_bytes().unwrap();
    let mut hash = Sha256::new();
    hash.update(b"tl-syntax.content-identity/v1\0");
    hash.update(b"tl-syntax.formula/v1");
    hash.update([0]);
    hash.update(&bytes);
    assert_eq!(
        document.content_identity().unwrap(),
        format!("{:x}", hash.finalize())
    );
    assert_ne!(
        document.content_identity().unwrap(),
        FORMULA_V1_SCHEMA_SHA256
    );
}
