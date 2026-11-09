//! One boundary test per row of the NFR-006 limit contract table: a value equal
//! to the limit is accepted; a value one past the limit is refused with the typed
//! error that names that limit.
//!
//! Each test names its table row. Rows with a caller-lowered `SyntaxArtifactLimits`
//! input run at the lowered value everywhere and at the owner value natively. Rows
//! with no caller input need up to 100,000 elements and run natively only.
//!
//! For a gate-style limit (document bytes, JSON depth) no canonical document
//! reaches the owner value, so "accepted" means the gate passes and a later check
//! refuses the input instead.
#![cfg(all(feature = "alloc", feature = "serde"))]

use tl_syntax::{
    FairnessPremisesDocument, FairnessPremisesError, Formula, FormulaDocument, FormulaError,
    FutureLoweringRefusal, FutureLoweringRequest, InfiniteClock, InfiniteFormulaDocument,
    InfiniteFormulaError, InfiniteNode, InfiniteNodeKind, LassoTraceDocument, LassoTraceError,
    Node, NodeId, NodeKind, OwnedSignalDeclaration, PartialValuation, PartialValuationError,
    PartialValue, PropositionBinding, PropositionId, PropositionMapDocument, RawBounds,
    RequirementContext, RequirementContextError, SemanticProfile, SignalCatalog,
    SignalCatalogDocument, SignalCatalogError, SignalDeclaration, SignalDomain, SignalId,
    SourceSpan, StrictDocumentReadError, SyntaxArtifactLimits, TraceObservation, ValuationEntry,
    FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1, MAX_FORMULA_DOCUMENT_DEPTH,
    MAX_FORMULA_DOCUMENT_NODES, MAX_FUTURE_LOWERING_IDENTITY_BYTES, MAX_FUTURE_LOWERING_KIND_BYTES,
    MAX_PROPOSITION_MAP_ENTRIES, MAX_REQUIREMENT_CONTEXT_FIELD_BYTES, MAX_SIGNAL_CATALOG_BINDINGS,
    MAX_SIGNAL_CATALOG_SIGNALS, MAX_SIGNAL_NAME_BYTES, MAX_TL_DOCUMENT_BYTES,
    MAX_TL_DOCUMENT_DEPTH, PAST_PROFILE_MANIFEST_MAX_BYTES,
};

const OWNER: SyntaxArtifactLimits = SyntaxArtifactLimits::OWNER_MAXIMA;

fn infinite_node_chain(length: usize) -> Vec<InfiniteNode> {
    let mut nodes = vec![InfiniteNode::new(InfiniteNodeKind::True)];
    for index in 1..length {
        nodes.push(InfiniteNode::new(InfiniteNodeKind::Not {
            operand: NodeId((index - 1) as u32),
        }));
    }
    nodes
}

fn node_chain(length: usize) -> Vec<Node> {
    let mut nodes = vec![Node::new(NodeKind::True)];
    for index in 1..length {
        nodes.push(Node::new(NodeKind::Not {
            operand: NodeId((index - 1) as u32),
        }));
    }
    nodes
}

fn formula_bytes(nodes: usize) -> Vec<u8> {
    format!(
        r#"{{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{}]}}"#,
        vec![r#"{"kind":"true"}"#; nodes].join(",")
    )
    .into_bytes()
}

fn proposition_map_bytes(entries: usize, name: &str) -> Vec<u8> {
    let items: Vec<String> = (0..entries)
        .map(|index| format!(r#"{{"id":{index},"name":"{name}{index}"}}"#))
        .collect();
    format!(
        r#"{{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{}]}}"#,
        items.join(",")
    )
    .into_bytes()
}

fn catalog_bytes(signals: usize, bindings: usize) -> Vec<u8> {
    let declared: Vec<String> = (1..=signals)
        .map(|id| format!(r#"{{"id":{id},"name":"s{id}","domain":{{"kind":"boolean"}}}}"#))
        .collect();
    let bound: Vec<String> = (0..bindings)
        .map(|index| format!(r#"{{"proposition":{index},"signal":1}}"#))
        .collect();
    format!(
        r#"{{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{}],"bindings":[{}]}}"#,
        declared.join(","),
        bound.join(",")
    )
    .into_bytes()
}

fn resource<T: std::fmt::Debug>(
    result: Result<T, StrictDocumentReadError>,
    expected: &str,
    actual: usize,
    limit: usize,
) {
    match result {
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource,
            actual: seen,
            limit: bound,
        }) => assert_eq!((resource, seen, bound), (expected, actual, limit)),
        other => panic!("expected {expected} resource refusal, got {other:?}"),
    }
}

// Row 1. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_01_document_bytes_at_a_lowered_limit() {
    let bytes = formula_bytes(1);
    let at = SyntaxArtifactLimits {
        document_bytes: bytes.len(),
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        document_bytes: bytes.len() - 1,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&bytes, at).is_ok());
    assert_eq!(
        FormulaDocument::from_json_bytes(&bytes, past)
            .unwrap_err()
            .to_string(),
        StrictDocumentReadError::DocumentTooLarge {
            actual: bytes.len(),
            limit: bytes.len() - 1
        }
        .to_string()
    );
    assert!(matches!(
        FormulaDocument::from_json_bytes(&bytes, past),
        Err(StrictDocumentReadError::DocumentTooLarge { actual, limit })
            if actual == bytes.len() && limit == bytes.len() - 1
    ));
}

// Row 1. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_01_document_bytes_at_the_owner_limit() {
    // A canonical proposition map of exactly the limit: 1,100 entries with
    // fixed-width unique names, the remainder spent on the last name.
    const ENTRIES: usize = 1100;
    let build = |filler: usize, extra: usize| {
        let items: Vec<String> = (0..ENTRIES)
            .map(|index| {
                let extra = if index == ENTRIES - 1 { extra } else { 0 };
                format!(
                    r#"{{"id":{index},"name":"{index:04}{}"}}"#,
                    "n".repeat(filler + extra)
                )
            })
            .collect();
        format!(
            r#"{{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{}]}}"#,
            items.join(",")
        )
        .into_bytes()
    };
    let filler = (MAX_TL_DOCUMENT_BYTES - build(0, 0).len()) / ENTRIES;
    let extra = MAX_TL_DOCUMENT_BYTES - build(filler, 0).len();
    let exact = build(filler, extra);
    assert_eq!(exact.len(), MAX_TL_DOCUMENT_BYTES);
    let document = PropositionMapDocument::from_json_bytes(&exact, OWNER);
    assert!(document.is_ok(), "{:?}", document.err());

    let past = vec![b' '; MAX_TL_DOCUMENT_BYTES + 1];
    assert!(matches!(
        PropositionMapDocument::from_json_bytes(&past, OWNER),
        Err(StrictDocumentReadError::DocumentTooLarge { actual, limit })
            if actual == MAX_TL_DOCUMENT_BYTES + 1 && limit == MAX_TL_DOCUMENT_BYTES
    ));
}

// Row 2. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_02_json_depth_at_a_lowered_limit() {
    let bytes = formula_bytes(1);
    let at = SyntaxArtifactLimits {
        json_depth: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        json_depth: 2,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&bytes, at).is_ok());
    assert!(matches!(
        FormulaDocument::from_json_bytes(&bytes, past),
        Err(StrictDocumentReadError::DepthLimitExceeded {
            actual: 3,
            limit: 2
        })
    ));
}

// Row 2. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_02_json_depth_at_the_owner_limit() {
    let nested = |depth: usize| format!("{}0{}", "[".repeat(depth), "]".repeat(depth)).into_bytes();
    let at = FormulaDocument::from_json_bytes(&nested(MAX_TL_DOCUMENT_DEPTH), OWNER);
    assert!(
        !matches!(at, Err(StrictDocumentReadError::DepthLimitExceeded { .. })),
        "{at:?}"
    );
    assert!(matches!(
        FormulaDocument::from_json_bytes(&nested(MAX_TL_DOCUMENT_DEPTH + 1), OWNER),
        Err(StrictDocumentReadError::DepthLimitExceeded { actual, limit })
            if actual == MAX_TL_DOCUMENT_DEPTH + 1 && limit == MAX_TL_DOCUMENT_DEPTH
    ));
}

// Row 3. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_03_string_bytes_at_the_owner_limit() {
    let name = |length: usize| {
        format!(
            r#"{{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{{"id":1,"name":"{}"}}]}}"#,
            "n".repeat(length)
        )
        .into_bytes()
    };
    let at = PropositionMapDocument::from_json_bytes(&name(OWNER.string_bytes), OWNER);
    assert!(at.is_ok(), "{:?}", at.err());
    assert!(matches!(
        PropositionMapDocument::from_json_bytes(&name(OWNER.string_bytes + 1), OWNER),
        Err(StrictDocumentReadError::StringTooLarge { actual, limit })
            if actual == OWNER.string_bytes + 1 && limit == OWNER.string_bytes
    ));
}

// Row 4. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_04_work_at_a_lowered_limit() {
    // No document within the other owner limits costs the 256 Mi owner ceiling
    // (three work units per byte of a 64 MiB document is below it), so only the
    // caller-lowered limit can sit exactly on the boundary.
    let bytes = formula_bytes(1);
    let exact = bytes.len() * 3 + 1;
    let at = SyntaxArtifactLimits {
        work: exact,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        work: exact - 1,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&bytes, at).is_ok());
    assert!(matches!(
        FormulaDocument::from_json_bytes(&bytes, past),
        Err(StrictDocumentReadError::WorkLimitExceeded { actual, limit })
            if actual == exact && limit == exact - 1
    ));
    const { assert!(3 * OWNER.document_bytes + OWNER.formula_nodes < OWNER.work) };
}

// Row 5. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_05_formula_nodes_at_a_lowered_limit() {
    let bytes = formula_bytes(3);
    let at = SyntaxArtifactLimits {
        formula_nodes: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        formula_nodes: 2,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&bytes, at).is_ok());
    resource(
        FormulaDocument::from_json_bytes(&bytes, past),
        "formula nodes",
        3,
        2,
    );
}

// Row 5. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_05_formula_nodes_at_the_owner_limit() {
    let at = formula_bytes(MAX_FORMULA_DOCUMENT_NODES);
    assert!(FormulaDocument::from_json_bytes(&at, OWNER).is_ok());
    let past = formula_bytes(MAX_FORMULA_DOCUMENT_NODES + 1);
    resource(
        FormulaDocument::from_json_bytes(&past, OWNER),
        "formula nodes",
        MAX_FORMULA_DOCUMENT_NODES + 1,
        MAX_FORMULA_DOCUMENT_NODES,
    );
}

// Row 6. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_06_formula_document_nodes() {
    let nodes = |count: usize| vec![Node::new(NodeKind::True); count];
    assert!(FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(0),
        nodes(MAX_FORMULA_DOCUMENT_NODES)
    )
    .is_ok());
    assert_eq!(
        FormulaDocument::new(
            SemanticProfile::ClosedTraceV1,
            NodeId(0),
            nodes(MAX_FORMULA_DOCUMENT_NODES + 1)
        ),
        Err(FormulaError::DocumentNodeLimitExceeded {
            node_count: MAX_FORMULA_DOCUMENT_NODES + 1,
            limit: MAX_FORMULA_DOCUMENT_NODES,
        })
    );
}

// Row 7. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_07_future_lowering_charges_three_nodes_against_the_limit() {
    let lower = |nodes: &[Node]| {
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), nodes).unwrap();
        FutureLoweringRequest {
            request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
            operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
            kind: b"W",
            semantic_profile: formula.profile().as_str().as_bytes(),
            formula,
            left: 0,
            right: 0,
            interval: Some(RawBounds::new(0, 1)),
            operator_span: None,
            expression_span: None,
        }
        .lower()
        .map(|_| ())
    };
    let fits = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES - 3];
    assert_eq!(lower(&fits), Ok(()));
    let over = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES - 2];
    assert_eq!(
        lower(&over),
        Err(FutureLoweringRefusal::DocumentNodeLimitExceeded {
            node_count: MAX_FORMULA_DOCUMENT_NODES + 1,
            limit: MAX_FORMULA_DOCUMENT_NODES,
        })
    );
}

// Row 8. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_08_infinite_graph_nodes() {
    let build = |count: usize| {
        InfiniteFormulaDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            NodeId(0),
            vec![InfiniteNode::new(InfiniteNodeKind::True); count],
        )
    };
    assert!(build(MAX_FORMULA_DOCUMENT_NODES).is_ok());
    assert_eq!(
        build(MAX_FORMULA_DOCUMENT_NODES + 1).unwrap_err(),
        InfiniteFormulaError::NodeLimit {
            actual: MAX_FORMULA_DOCUMENT_NODES + 1
        }
    );
}

fn observation(position: u32, map: &str) -> TraceObservation {
    TraceObservation {
        position,
        valuation: PartialValuation::new(
            map.into(),
            &[PropositionId(1)],
            vec![ValuationEntry {
                proposition: PropositionId(1),
                value: PartialValue::True,
            }],
        )
        .unwrap(),
    }
}

// Row 9. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_09_lasso_positions() {
    let build = |count: usize| {
        LassoTraceDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            "m".into(),
            vec![PropositionId(1)],
            Vec::new(),
            (0..count as u32)
                .map(|position| observation(position, "m"))
                .collect(),
        )
    };
    assert!(build(MAX_FORMULA_DOCUMENT_NODES).is_ok());
    assert_eq!(
        build(MAX_FORMULA_DOCUMENT_NODES + 1).unwrap_err(),
        LassoTraceError::ResourceLimit
    );
}

// Row 10. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_10_fairness_roots() {
    let formula = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True); MAX_FORMULA_DOCUMENT_NODES],
    )
    .unwrap();
    let identity = formula.content_identity().unwrap();
    let roots = |count: usize| (0..count as u32).map(NodeId).collect::<Vec<_>>();
    let build = |count: usize| {
        FairnessPremisesDocument::new(
            &formula,
            identity.clone(),
            InfiniteClock::EventPosition,
            roots(count),
        )
    };
    assert!(build(MAX_FORMULA_DOCUMENT_NODES).is_ok());
    assert_eq!(
        build(MAX_FORMULA_DOCUMENT_NODES + 1).unwrap_err(),
        FairnessPremisesError::ResourceLimit
    );
}

// Row 11. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_11_formula_depth() {
    let build = |depth: usize| {
        FormulaDocument::new_v2(
            SemanticProfile::ClosedTraceV1,
            NodeId((depth - 1) as u32),
            node_chain(depth),
        )
    };
    assert!(build(MAX_FORMULA_DOCUMENT_DEPTH).is_ok());
    assert_eq!(
        build(MAX_FORMULA_DOCUMENT_DEPTH + 1).unwrap_err(),
        FormulaError::DocumentDepthLimitExceeded {
            node: NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32),
            depth: MAX_FORMULA_DOCUMENT_DEPTH + 1,
            limit: MAX_FORMULA_DOCUMENT_DEPTH,
        }
    );
}

// Row 12. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_12_infinite_graph_depth() {
    let build = |depth: usize| {
        InfiniteFormulaDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            NodeId((depth - 1) as u32),
            infinite_node_chain(depth),
        )
    };
    assert!(build(MAX_FORMULA_DOCUMENT_DEPTH).is_ok());
    assert_eq!(
        build(MAX_FORMULA_DOCUMENT_DEPTH + 1).unwrap_err(),
        InfiniteFormulaError::DepthLimit {
            node: NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32)
        }
    );
}

// Row 13. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_13_formula_depth_at_a_lowered_limit() {
    let document = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(2),
        infinite_node_chain(3),
    )
    .unwrap();
    let bytes = document.canonical_json_bytes().unwrap();
    let at = SyntaxArtifactLimits {
        formula_depth: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        formula_depth: 2,
        ..OWNER
    };
    assert!(InfiniteFormulaDocument::from_json_bytes(&bytes, at).is_ok());
    resource(
        InfiniteFormulaDocument::from_json_bytes(&bytes, past),
        "formula depth",
        3,
        2,
    );
}

// Row 13. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_13_formula_depth_at_the_owner_limit() {
    let bytes = |depth: usize| {
        let document = InfiniteFormulaDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            NodeId(0),
            infinite_node_chain(depth),
        );
        // The constructor refuses a deeper chain, so build the past-limit bytes
        // from the at-limit document by appending one more Not node.
        document.map(|document| document.canonical_json_bytes().unwrap())
    };
    let at = bytes(MAX_FORMULA_DOCUMENT_DEPTH).unwrap();
    assert!(InfiniteFormulaDocument::from_json_bytes(&at, OWNER).is_ok());
    let text = String::from_utf8(at).unwrap();
    let extra = format!(
        r#"{{"kind":"not","operand":{}}}"#,
        MAX_FORMULA_DOCUMENT_DEPTH - 1
    );
    let past = text.replacen("]}", &format!(",{extra}]}}"), 1);
    // At the owner value the graph constructor behind the reader refuses first.
    match InfiniteFormulaDocument::from_json_bytes(past.as_bytes(), OWNER) {
        Err(StrictDocumentReadError::InvalidDocument(error)) => {
            let expected = InfiniteFormulaError::DepthLimit {
                node: NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32),
            };
            assert!(
                error.to_string().contains(&format!("{expected:?}")),
                "{error}"
            );
        }
        other => panic!("expected the graph depth refusal, got {other:?}"),
    }
}

fn boolean(id: u32, name: &str) -> SignalDeclaration<'_> {
    SignalDeclaration::new(SignalId(id), name, SignalDomain::Boolean)
}

// Row 14. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_14_borrowed_catalog_signals() {
    let names: Vec<String> = (0..=MAX_SIGNAL_CATALOG_SIGNALS)
        .map(|i| format!("s{i}"))
        .collect();
    let signals: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(i, n)| boolean(i as u32, n))
        .collect();
    let build = |count: usize| {
        let mut scratch = vec![0_u32; count];
        SignalCatalog::new(&signals[..count], &[], &mut scratch).map(|_| ())
    };
    assert_eq!(build(MAX_SIGNAL_CATALOG_SIGNALS), Ok(()));
    assert_eq!(
        build(MAX_SIGNAL_CATALOG_SIGNALS + 1),
        Err(SignalCatalogError::SignalLimitExceeded {
            count: MAX_SIGNAL_CATALOG_SIGNALS + 1,
            limit: MAX_SIGNAL_CATALOG_SIGNALS,
        })
    );
}

// Row 15. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_15_catalog_signals_at_a_lowered_limit() {
    let bytes = catalog_bytes(3, 0);
    let at = SyntaxArtifactLimits {
        signals: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        signals: 2,
        ..OWNER
    };
    assert!(SignalCatalogDocument::from_json_bytes(&bytes, at).is_ok());
    resource(
        SignalCatalogDocument::from_json_bytes(&bytes, past),
        "signals",
        3,
        2,
    );
}

// Row 15. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_15_catalog_signals_at_the_owner_limit() {
    let at = catalog_bytes(MAX_SIGNAL_CATALOG_SIGNALS, 0);
    assert!(SignalCatalogDocument::from_json_bytes(&at, OWNER).is_ok());
    resource(
        SignalCatalogDocument::from_json_bytes(
            &catalog_bytes(MAX_SIGNAL_CATALOG_SIGNALS + 1, 0),
            OWNER,
        ),
        "signals",
        MAX_SIGNAL_CATALOG_SIGNALS + 1,
        MAX_SIGNAL_CATALOG_SIGNALS,
    );
}

// Row 16. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_16_borrowed_catalog_bindings() {
    let signals = [boolean(1, "a")];
    let bindings: Vec<_> = (0..=MAX_SIGNAL_CATALOG_BINDINGS as u32)
        .map(|p| PropositionBinding::new(PropositionId(p), SignalId(1)))
        .collect();
    let build = |count: usize| {
        let mut scratch = [0_u32; 1];
        SignalCatalog::new(&signals, &bindings[..count], &mut scratch).map(|_| ())
    };
    assert_eq!(build(MAX_SIGNAL_CATALOG_BINDINGS), Ok(()));
    assert_eq!(
        build(MAX_SIGNAL_CATALOG_BINDINGS + 1),
        Err(SignalCatalogError::BindingLimitExceeded {
            count: MAX_SIGNAL_CATALOG_BINDINGS + 1,
            limit: MAX_SIGNAL_CATALOG_BINDINGS,
        })
    );
}

// Row 17. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_17_catalog_bindings_at_a_lowered_limit() {
    let bytes = catalog_bytes(1, 3);
    let at = SyntaxArtifactLimits {
        bindings: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        bindings: 2,
        ..OWNER
    };
    assert!(SignalCatalogDocument::from_json_bytes(&bytes, at).is_ok());
    resource(
        SignalCatalogDocument::from_json_bytes(&bytes, past),
        "bindings",
        3,
        2,
    );
}

// Row 17. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_17_catalog_bindings_at_the_owner_limit() {
    let at = catalog_bytes(1, MAX_SIGNAL_CATALOG_BINDINGS);
    assert!(SignalCatalogDocument::from_json_bytes(&at, OWNER).is_ok());
    resource(
        SignalCatalogDocument::from_json_bytes(
            &catalog_bytes(1, MAX_SIGNAL_CATALOG_BINDINGS + 1),
            OWNER,
        ),
        "bindings",
        MAX_SIGNAL_CATALOG_BINDINGS + 1,
        MAX_SIGNAL_CATALOG_BINDINGS,
    );
}

// Row 18. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_18_proposition_entries_at_a_lowered_limit() {
    let bytes = proposition_map_bytes(3, "p");
    let at = SyntaxArtifactLimits {
        propositions: 3,
        ..OWNER
    };
    let past = SyntaxArtifactLimits {
        propositions: 2,
        ..OWNER
    };
    assert!(PropositionMapDocument::from_json_bytes(&bytes, at).is_ok());
    resource(
        PropositionMapDocument::from_json_bytes(&bytes, past),
        "propositions",
        3,
        2,
    );
}

// Row 18. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_18_proposition_entries_at_the_owner_limit() {
    let at = proposition_map_bytes(MAX_PROPOSITION_MAP_ENTRIES, "p");
    assert!(PropositionMapDocument::from_json_bytes(&at, OWNER).is_ok());
    resource(
        PropositionMapDocument::from_json_bytes(
            &proposition_map_bytes(MAX_PROPOSITION_MAP_ENTRIES + 1, "p"),
            OWNER,
        ),
        "propositions",
        MAX_PROPOSITION_MAP_ENTRIES + 1,
        MAX_PROPOSITION_MAP_ENTRIES,
    );
}

// Row 19. Trace: TC-165, NFR-006-AC-3
#[cfg(not(miri))]
#[test]
fn row_19_partial_valuation_entries() {
    let build = |count: usize| {
        let propositions: Vec<_> = (0..count as u32).map(PropositionId).collect();
        let entries = propositions
            .iter()
            .map(|&proposition| ValuationEntry {
                proposition,
                value: PartialValue::True,
            })
            .collect();
        PartialValuation::new("m".into(), &propositions, entries)
    };
    assert!(build(MAX_PROPOSITION_MAP_ENTRIES).is_ok());
    assert_eq!(
        build(MAX_PROPOSITION_MAP_ENTRIES + 1).unwrap_err(),
        PartialValuationError::ResourceLimit
    );
}

// Row 20. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_20_signal_name_bytes() {
    let build = |length: usize| {
        let name = "n".repeat(length);
        let signals = [boolean(1, &name)];
        let mut scratch = [0_u32; 1];
        SignalCatalog::new(&signals, &[], &mut scratch).map(|_| ())
    };
    assert_eq!(build(MAX_SIGNAL_NAME_BYTES), Ok(()));
    assert_eq!(
        build(MAX_SIGNAL_NAME_BYTES + 1),
        Err(SignalCatalogError::SignalNameTooLong {
            signal: SignalId(1),
            length: MAX_SIGNAL_NAME_BYTES + 1,
            limit: MAX_SIGNAL_NAME_BYTES,
        })
    );
    let wire = |length: usize| {
        let name = "n".repeat(length);
        SignalCatalogDocument::new(
            vec![OwnedSignalDeclaration::new(
                SignalId(1),
                name,
                SignalDomain::Boolean,
            )],
            Vec::new(),
        )
        .map(|_| ())
    };
    assert_eq!(wire(MAX_SIGNAL_NAME_BYTES), Ok(()));
    assert!(matches!(
        wire(MAX_SIGNAL_NAME_BYTES + 1),
        Err(SignalCatalogError::SignalNameTooLong { length, limit, .. })
            if length == MAX_SIGNAL_NAME_BYTES + 1 && limit == MAX_SIGNAL_NAME_BYTES
    ));
}

// Row 21. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_21_requirement_context_field_bytes() {
    let span = SourceSpan::new(0, 1).unwrap();
    let at = "x".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES);
    let past = "x".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1);
    let fields = |value: &str, slot: usize| {
        let mut parts = ["a"; 4];
        parts[slot] = value;
        RequirementContext::new(parts[0], parts[1], parts[2], parts[3], span).map(|_| ())
    };
    for slot in 0..4 {
        assert_eq!(fields(&at, slot), Ok(()));
        assert!(matches!(
            fields(&past, slot),
            Err(RequirementContextError::FieldTooLong { length, limit, .. })
                if length == MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1
                    && limit == MAX_REQUIREMENT_CONTEXT_FIELD_BYTES
        ));
    }
}

// Row 22. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_22_past_profile_manifest_bytes() {
    let spaces = |length: usize| vec![b' '; length];
    let at = tl_syntax::contracts::manifest::validate_past_profile_implementation(&spaces(
        PAST_PROFILE_MANIFEST_MAX_BYTES,
    ));
    assert_ne!(at.unwrap_err().field(), "size");
    let past = tl_syntax::contracts::manifest::validate_past_profile_implementation(&spaces(
        PAST_PROFILE_MANIFEST_MAX_BYTES + 1,
    ));
    assert_eq!(past.unwrap_err().field(), "size");
}

fn lowering<'a>(
    nodes: &'a [Node],
    edit: impl FnOnce(&mut FutureLoweringRequest<'a>),
) -> Result<(), FutureLoweringRefusal> {
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), nodes).unwrap();
    let mut request = FutureLoweringRequest {
        request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
        operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
        kind: b"W",
        semantic_profile: formula.profile().as_str().as_bytes(),
        formula,
        left: 0,
        right: 1,
        interval: Some(RawBounds::new(0, 1)),
        operator_span: None,
        expression_span: None,
    };
    edit(&mut request);
    request.lower().map(|_| ())
}

// Row 23. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_23_future_lowering_identity_bytes() {
    let nodes = [Node::new(NodeKind::True), Node::new(NodeKind::False)];
    let at = [b'x'; MAX_FUTURE_LOWERING_IDENTITY_BYTES];
    let past = [b'x'; MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1];
    // At the limit an unknown identity is refused as unknown, never as too long.
    assert_eq!(
        lowering(&nodes, |r| r.request_identity = &at),
        Err(FutureLoweringRefusal::UnknownRequestIdentity)
    );
    assert_eq!(
        lowering(&nodes, |r| r.request_identity = &past),
        Err(FutureLoweringRefusal::RequestIdentityTooLong { len: past.len() })
    );
    assert_eq!(
        lowering(&nodes, |r| r.operator_profile = &at),
        Err(FutureLoweringRefusal::UnknownOperatorProfile)
    );
    assert_eq!(
        lowering(&nodes, |r| r.operator_profile = &past),
        Err(FutureLoweringRefusal::OperatorProfileTooLong { len: past.len() })
    );
    assert_eq!(
        lowering(&nodes, |r| r.semantic_profile = &at),
        Err(FutureLoweringRefusal::UnknownSemanticProfile)
    );
    assert_eq!(
        lowering(&nodes, |r| r.semantic_profile = &past),
        Err(FutureLoweringRefusal::SemanticProfileTooLong { len: past.len() })
    );
}

// Row 24. Trace: TC-165, NFR-006-AC-3
#[test]
fn row_24_future_lowering_kind_bytes() {
    let nodes = [Node::new(NodeKind::True), Node::new(NodeKind::False)];
    let at = [b'W'; MAX_FUTURE_LOWERING_KIND_BYTES];
    let past = [b'W'; MAX_FUTURE_LOWERING_KIND_BYTES + 1];
    assert_eq!(
        lowering(&nodes, |r| r.kind = &at),
        Err(FutureLoweringRefusal::UnknownKind)
    );
    assert_eq!(
        lowering(&nodes, |r| r.kind = &past),
        Err(FutureLoweringRefusal::KindTooLong { len: past.len() })
    );
}
