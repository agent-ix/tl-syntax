#![cfg(all(feature = "alloc", feature = "serde"))]

use tl_syntax::{
    FairnessPremisesDocument, FairnessPremisesError, InfiniteClock, InfiniteFormulaDocument,
    InfiniteNode, InfiniteNodeKind, LassoTraceDocument, LassoTraceError, NodeId, PartialValuation,
    PartialValuationError, PartialValue, PropositionId, SemanticProfile, TraceObservation,
    ValuationEntry,
};

fn propositions() -> Vec<PropositionId> {
    vec![
        PropositionId(1),
        PropositionId(3),
        PropositionId(5),
        PropositionId(7),
    ]
}
fn valuation(values: [PartialValue; 4]) -> PartialValuation {
    PartialValuation::new(
        "map/v1".into(),
        &propositions(),
        propositions()
            .into_iter()
            .zip(values)
            .map(|(proposition, value)| ValuationEntry { proposition, value })
            .collect(),
    )
    .unwrap()
}
fn observation(position: u32, value: PartialValuation) -> TraceObservation {
    TraceObservation {
        position,
        valuation: value,
    }
}
fn formula() -> InfiniteFormulaDocument {
    InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(1),
        vec![
            InfiniteNode::new(InfiniteNodeKind::Proposition {
                proposition: PropositionId(1),
            }),
            InfiniteNode::new(InfiniteNodeKind::Globally {
                interval: tl_syntax::TemporalInterval::Unbounded(
                    tl_syntax::UnboundedInterval::new(0),
                ),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap()
}

// Trace: TC-157, FR-023-AC-1
#[test]
fn four_states_round_trip_without_coercion() {
    let values = [
        PartialValue::True,
        PartialValue::False,
        PartialValue::Missing,
        PartialValue::Conflicting,
    ];
    let valuation = valuation(values);
    for (id, expected) in propositions().into_iter().zip(values) {
        assert_eq!(valuation.value(id), Some(expected));
    }
    let bytes = valuation.canonical_json_bytes().unwrap();
    let decoded: PartialValuation = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(decoded, valuation);
    assert_eq!(decoded.canonical_json_bytes().unwrap(), bytes);
}

// Trace: TC-158, FR-023-AC-2
#[test]
fn partial_valuation_refuses_each_bad_population() {
    let ids = propositions();
    let mk = |proposition| ValuationEntry {
        proposition,
        value: PartialValue::True,
    };
    assert_eq!(
        PartialValuation::new(
            "map/v1".into(),
            &ids,
            vec![mk(ids[0]), mk(ids[1]), mk(ids[1]), mk(ids[3])]
        ),
        Err(PartialValuationError::DuplicateProposition {
            proposition: ids[1]
        })
    );
    assert_eq!(
        PartialValuation::new(
            "map/v1".into(),
            &ids,
            vec![mk(ids[1]), mk(ids[0]), mk(ids[2]), mk(ids[3])]
        ),
        Err(PartialValuationError::UnorderedProposition {
            proposition: ids[0]
        })
    );
    assert_eq!(
        PartialValuation::new(
            "map/v1".into(),
            &ids,
            vec![mk(ids[0]), mk(ids[1]), mk(ids[2])]
        ),
        Err(PartialValuationError::OmittedProposition {
            proposition: ids[3]
        })
    );
    assert_eq!(
        PartialValuation::new(
            "map/v1".into(),
            &ids,
            vec![mk(ids[0]), mk(ids[1]), mk(ids[2]), mk(PropositionId(9))]
        ),
        Err(PartialValuationError::OmittedProposition {
            proposition: ids[3]
        })
    );
}

// Trace: TC-159, FR-023-AC-3
#[test]
fn missing_and_conflicting_have_different_semantic_identities() {
    let missing = valuation([
        PartialValue::True,
        PartialValue::False,
        PartialValue::Missing,
        PartialValue::True,
    ]);
    let conflict = valuation([
        PartialValue::True,
        PartialValue::False,
        PartialValue::Conflicting,
        PartialValue::True,
    ]);
    assert_ne!(
        missing.canonical_json_bytes().unwrap(),
        conflict.canonical_json_bytes().unwrap()
    );
    assert_ne!(
        missing.content_identity().unwrap(),
        conflict.content_identity().unwrap()
    );
}

// Trace: TC-154, FR-022-AC-1
#[test]
fn empty_and_nonempty_prefix_lassos_round_trip() {
    let v = valuation([
        PartialValue::True,
        PartialValue::False,
        PartialValue::Missing,
        PartialValue::Conflicting,
    ]);
    for prefix_len in [0, 1] {
        let prefix = (0..prefix_len)
            .map(|position| observation(position, v.clone()))
            .collect();
        let trace = LassoTraceDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            "map/v1".into(),
            propositions(),
            prefix,
            vec![observation(prefix_len, v.clone())],
        )
        .unwrap();
        assert_eq!(trace.loop_entry(), prefix_len as usize);
        let bytes = trace.canonical_json_bytes().unwrap();
        let decoded: LassoTraceDocument = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded, trace);
    }
}

// Trace: TC-155, FR-022-AC-2
#[test]
fn lasso_refuses_empty_loop_bad_position_and_map_identity() {
    let v = valuation([PartialValue::True; 4]);
    let make = |prefix, loop_observations| {
        LassoTraceDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            "map/v1".into(),
            propositions(),
            prefix,
            loop_observations,
        )
    };
    assert_eq!(make(vec![], vec![]), Err(LassoTraceError::EmptyLoop));
    assert_eq!(
        make(vec![], vec![observation(1, v.clone())]),
        Err(LassoTraceError::Position {
            expected: 0,
            actual: 1
        })
    );
    assert_eq!(
        make(
            vec![],
            vec![observation(
                0,
                PartialValuation::new("other".into(), &propositions(), v.entries().to_vec())
                    .unwrap()
            )]
        ),
        Err(LassoTraceError::MapIdentityMismatch)
    );
    assert!(LassoTraceDocument::new(
        SemanticProfile::ClosedTraceV1,
        InfiniteClock::EventPosition,
        "map/v1".into(),
        propositions(),
        vec![],
        vec![observation(0, v)]
    )
    .is_err());
}

// Trace: TC-156, FR-022-AC-3
#[test]
fn lasso_unrolls_without_terminal_position() {
    let yes = valuation([PartialValue::True; 4]);
    let no = valuation([PartialValue::False; 4]);
    let trace = LassoTraceDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        "map/v1".into(),
        propositions(),
        vec![observation(0, yes.clone())],
        vec![observation(1, no.clone()), observation(2, yes.clone())],
    )
    .unwrap();
    for (position, expected) in [
        (0, &yes),
        (1, &no),
        (2, &yes),
        (3, &no),
        (4, &yes),
        (u64::MAX, &no),
    ] {
        assert_eq!(&trace.observation_at(position).valuation, expected);
    }
}

// Trace: TC-151, FR-021-AC-1
#[test]
fn fairness_round_trips_and_binds_graph() {
    let graph = formula();
    let graph_id = graph.content_identity().unwrap();
    for roots in [vec![], vec![NodeId(0), NodeId(1)]] {
        let fairness = FairnessPremisesDocument::new(
            &graph,
            graph_id.clone(),
            InfiniteClock::EventPosition,
            roots.clone(),
        )
        .unwrap();
        assert_eq!(fairness.roots(), roots);
        let bytes = fairness.canonical_json_bytes().unwrap();
        let decoded: FairnessPremisesDocument = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded, fairness);
    }
}

// Trace: TC-152, FR-021-AC-2
#[test]
fn fairness_refuses_graph_identity_duplicate_and_foreign_root() {
    let graph = formula();
    let id = graph.content_identity().unwrap();
    assert_eq!(
        FairnessPremisesDocument::new(
            &graph,
            "foreign".into(),
            InfiniteClock::EventPosition,
            vec![]
        ),
        Err(FairnessPremisesError::GraphIdentityMismatch)
    );
    assert_eq!(
        FairnessPremisesDocument::new(
            &graph,
            id.clone(),
            InfiniteClock::EventPosition,
            vec![NodeId(0), NodeId(0)]
        ),
        Err(FairnessPremisesError::DuplicateRoot { root: NodeId(0) })
    );
    assert_eq!(
        FairnessPremisesDocument::new(&graph, id, InfiniteClock::EventPosition, vec![NodeId(3)]),
        Err(FairnessPremisesError::ForeignRoot { root: NodeId(3) })
    );
}
