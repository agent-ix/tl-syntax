#![cfg(all(feature = "alloc", feature = "serde"))]

use tl_syntax::{
    FormulaDocument, InfiniteClock, InfiniteFormulaDocument, InfiniteNode, InfiniteNodeKind,
    Interval, Node, NodeId, NodeKind, SemanticProfile, SyntaxArtifactLimits, TemporalInterval,
    UnboundedInterval,
};

// Trace: TC-144, FR-289-AC-1, FR-289-AC-2
#[test]
fn unbounded_formula_admits_mixed_future_and_past() {
    let closed = TemporalInterval::Closed(Interval::new(0, 2).unwrap());
    let open = TemporalInterval::Unbounded(UnboundedInterval::new(u32::MAX));
    assert_eq!(open.start(), u32::MAX);
    assert_ne!(closed, open);
    let nodes = vec![
        InfiniteNode::new(InfiniteNodeKind::True),
        InfiniteNode::new(InfiniteNodeKind::Once {
            interval: closed,
            operand: NodeId(0),
        }),
        InfiniteNode::new(InfiniteNodeKind::Globally {
            interval: open,
            operand: NodeId(1),
        }),
    ];
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(2),
        nodes.clone(),
    )
    .unwrap();
    assert_eq!(doc.formula().nodes(), nodes);
    assert!(FormulaDocument::new_v2(
        SemanticProfile::InfiniteTraceV1,
        NodeId(0),
        vec![Node::new(NodeKind::True)]
    )
    .is_err());
}

// Trace: TC-145, FR-289-AC-3
#[test]
fn unbounded_wire_round_trips_and_legacy_reader_refuses() {
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(1),
        vec![
            InfiniteNode::new(InfiniteNodeKind::True),
            InfiniteNode::new(InfiniteNodeKind::Future {
                interval: TemporalInterval::Unbounded(UnboundedInterval::new(3)),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();
    let bytes = doc.canonical_json_bytes().unwrap();
    let decoded =
        InfiniteFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).unwrap();
    assert_eq!(decoded, doc);
    assert!(FormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).is_err());
}

struct Backend(std::cell::Cell<usize>);
impl tl_syntax::LivenessBackend for Backend {
    fn settle(
        &self,
        _: &InfiniteFormulaDocument,
        _: tl_syntax::LivenessSubject<'_>,
    ) -> tl_syntax::LivenessDisposition {
        self.0.set(self.0.get() + 1);
        tl_syntax::LivenessDisposition::Proved
    }
}

// Trace: TC-146, TC-153, FR-290-AC-1, FR-290-AC-2, FR-290-AC-3, FR-021-AC-3
#[test]
fn absent_liveness_backend_settles_unsupported_before_evaluation() {
    use tl_syntax::{
        settle_liveness, LivenessDisposition, LivenessSubject, LivenessSubjectKind,
        LIVENESS_CAPABILITY_V1,
    };
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True)],
    )
    .unwrap();
    let subject = LivenessSubject {
        kind: LivenessSubjectKind::LassoTrace,
        identity: "trace-42",
    };
    let absent = settle_liveness(&doc, subject, None);
    assert_eq!(absent.disposition, LivenessDisposition::Unsupported);
    assert_eq!(absent.warning, Some(LIVENESS_CAPABILITY_V1));
    assert_eq!(absent.subject, subject);
    let backend = Backend(std::cell::Cell::new(0));
    let present = settle_liveness(&doc, subject, Some(&backend));
    assert_eq!(backend.0.get(), 1);
    assert_eq!(present.disposition, LivenessDisposition::Proved);
    assert_eq!(present.warning, None);
    assert_eq!(present.subject, subject);
    let prefix = LivenessSubject {
        kind: LivenessSubjectKind::FinitePrefix,
        identity: "prefix-42",
    };
    let claim = settle_liveness(&doc, prefix, Some(&backend));
    assert_eq!(claim.disposition, LivenessDisposition::Failed);
    assert_eq!(claim.subject, prefix);
}

// Trace: TC-148, FR-020-AC-1
#[test]
fn infinite_profile_is_fourth_distinct_tl_identity() {
    use tl_syntax::{select_infinite_profile, InfiniteProfileError};
    let identities: std::collections::BTreeSet<_> = SemanticProfile::ALL
        .into_iter()
        .map(SemanticProfile::as_str)
        .collect();
    assert_eq!(identities.len(), 4);
    assert!(identities.contains("mltl.infinite-trace/v1"));
    assert!(!identities.contains("quire.temporal.infinite-trace/v1"));
    assert!(
        serde_json::from_str::<SemanticProfile>("\"quire.temporal.infinite-trace/v1\"").is_err()
    );
    assert_eq!(
        select_infinite_profile(None),
        Err(InfiniteProfileError::Missing)
    );
    assert_eq!(
        select_infinite_profile(Some("quire.temporal.infinite-trace/v1")),
        Err(InfiniteProfileError::Unknown)
    );
    assert_eq!(
        select_infinite_profile(Some("mltl.closed-trace/v1")),
        Err(InfiniteProfileError::Mismatched(
            SemanticProfile::ClosedTraceV1
        ))
    );
    assert_eq!(
        select_infinite_profile(Some("mltl.infinite-trace/v1")),
        Ok(SemanticProfile::InfiniteTraceV1)
    );
    let text = r#"{"schema_version":"tl-syntax.formula-unbounded/v1","clock":"event_position","root":0,"nodes":[{"kind":"true"}]}"#;
    assert!(InfiniteFormulaDocument::from_json_bytes(
        text.as_bytes(),
        SyntaxArtifactLimits::default()
    )
    .is_err());
}

// Trace: TC-149, FR-020-AC-2
#[test]
fn infinite_formula_refuses_foreign_clock_selection() {
    use tl_syntax::InfiniteClockError;
    assert_eq!(
        InfiniteClock::try_from(""),
        Err(InfiniteClockError::Missing)
    );
    assert_eq!(
        InfiniteClock::try_from("event_position"),
        Ok(InfiniteClock::EventPosition)
    );
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True)],
    )
    .unwrap();
    let bytes = doc.canonical_json_bytes().unwrap();
    for clock in ["fixed_sample", "dense", "timestamped"] {
        assert_eq!(
            InfiniteClock::try_from(clock),
            Err(InfiniteClockError::Unsupported)
        );
        let text = std::str::from_utf8(&bytes)
            .unwrap()
            .replace("event_position", clock);
        assert!(
            InfiniteFormulaDocument::from_json_bytes(
                text.as_bytes(),
                SyntaxArtifactLimits::default()
            )
            .is_err(),
            "{clock}"
        );
    }
}

// Trace: TC-150, FR-020-AC-3
#[test]
fn legacy_formula_wire_bytes_stay_exact() {
    let v1 = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(1),
        vec![
            Node::new(NodeKind::True),
            Node::new(NodeKind::Future {
                interval: Interval::new(0, 1).unwrap(),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();
    let golden = r#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"true"},{"kind":"future","interval":{"start":0,"end":1},"operand":0}]}"#;
    assert_eq!(v1.canonical_json_bytes().unwrap(), golden.as_bytes());
    let v2 = v1.to_v2();
    let golden_v2 = golden.replace("tl-syntax.formula/v1", "tl-syntax.formula/v2");
    assert_eq!(v2.canonical_json_bytes().unwrap(), golden_v2.as_bytes());
}

// Trace: TC-144, FR-289-AC-1, FR-289-AC-4
#[test]
fn canonical_interval_spellings_and_malformed_forms() {
    use tl_syntax::TemporalIntervalParseError as Error;
    assert_eq!(
        "[0,)".parse::<TemporalInterval>(),
        Ok(TemporalInterval::Unbounded(UnboundedInterval::new(0)))
    );
    assert_eq!(
        "[4294967295,)".parse::<TemporalInterval>(),
        Ok(TemporalInterval::Unbounded(UnboundedInterval::new(
            u32::MAX
        )))
    );
    assert_eq!(
        "[1,3]".parse::<TemporalInterval>(),
        Ok(TemporalInterval::Closed(Interval::new(1, 3).unwrap()))
    );
    for bad in [
        "[0)", "[0,]", "[0,1)", "[0,1,2]", "[0 1]", "[0,1", "0,1]", "[ 0,)",
    ] {
        assert!(bad.parse::<TemporalInterval>().is_err(), "{bad}");
    }
    assert_eq!(
        "[4294967296,)".parse::<TemporalInterval>(),
        Err(Error::InvalidBound)
    );
    assert_eq!("[2,1]".parse::<TemporalInterval>(), Err(Error::Inverted));
}

// Trace: TC-147, FR-291-AC-1, FR-291-AC-2
#[test]
fn liveness_results_retain_exact_canonical_graph_and_subject() {
    use tl_syntax::{settle_liveness, LivenessDisposition, LivenessSubject, LivenessSubjectKind};
    struct Selected(LivenessDisposition);
    impl tl_syntax::LivenessBackend for Selected {
        fn settle(
            &self,
            _: &InfiniteFormulaDocument,
            _: LivenessSubject<'_>,
        ) -> LivenessDisposition {
            self.0
        }
    }
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True)],
    )
    .unwrap();
    let subject = LivenessSubject {
        kind: LivenessSubjectKind::Model,
        identity: "model-17",
    };
    for disposition in [
        LivenessDisposition::Proved,
        LivenessDisposition::Refuted,
        LivenessDisposition::Inconclusive,
        LivenessDisposition::Unsupported,
        LivenessDisposition::ResourceIncomplete,
        LivenessDisposition::Failed,
    ] {
        let result = settle_liveness(&doc, subject, Some(&Selected(disposition)));
        assert!(std::ptr::eq(result.formula, &doc));
        assert_eq!(
            result.formula.content_identity().unwrap(),
            doc.content_identity().unwrap()
        );
        assert_eq!(
            result.formula.semantic_profile(),
            SemanticProfile::InfiniteTraceV1
        );
        assert_eq!(result.formula.clock(), InfiniteClock::EventPosition);
        assert_eq!(result.subject, subject);
        assert_eq!(result.disposition, disposition);
    }
}

// Trace: TC-144, TC-145, FR-289-AC-2, FR-289-AC-3
#[test]
fn every_unbounded_temporal_primitive_round_trips() {
    use InfiniteNodeKind as Kind;
    let interval = TemporalInterval::Unbounded(UnboundedInterval::new(2));
    let unary = [
        Kind::Future {
            interval,
            operand: NodeId(0),
        },
        Kind::Globally {
            interval,
            operand: NodeId(0),
        },
        Kind::Once {
            interval,
            operand: NodeId(0),
        },
        Kind::Historically {
            interval,
            operand: NodeId(0),
        },
    ];
    let binary = [
        Kind::Until {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        Kind::Release {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        Kind::Since {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        Kind::Triggered {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
    ];
    for kind in unary.into_iter().chain(binary) {
        let nodes = vec![
            InfiniteNode::new(Kind::True),
            InfiniteNode::new(Kind::False),
            InfiniteNode::new(kind),
        ];
        let doc = InfiniteFormulaDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            NodeId(2),
            nodes,
        )
        .unwrap();
        let bytes = doc.canonical_json_bytes().unwrap();
        assert_eq!(
            InfiniteFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default())
                .unwrap(),
            doc
        );
        assert!(std::str::from_utf8(&bytes)
            .unwrap()
            .contains("\"kind\":\"unbounded\""));
    }
}

// Trace: TC-144, FR-289-AC-2
#[test]
fn weak_and_strong_derived_operators_use_the_same_equations() {
    use tl_syntax::{lower_infinite_future, FutureKind};
    let interval = TemporalInterval::Unbounded(UnboundedInterval::new(1));
    let weak = lower_infinite_future(
        FutureKind::WeakUntil,
        NodeId(0),
        NodeId(1),
        interval,
        NodeId(2),
        None,
    )
    .unwrap();
    assert!(matches!(weak[0].kind, InfiniteNodeKind::Until { .. }));
    assert!(matches!(weak[1].kind, InfiniteNodeKind::Globally { .. }));
    assert_eq!(
        weak[2].kind,
        InfiniteNodeKind::Or {
            left: NodeId(2),
            right: NodeId(3)
        }
    );
    let strong = lower_infinite_future(
        FutureKind::StrongRelease,
        NodeId(0),
        NodeId(1),
        interval,
        NodeId(2),
        None,
    )
    .unwrap();
    assert!(matches!(strong[0].kind, InfiniteNodeKind::Release { .. }));
    assert!(matches!(strong[1].kind, InfiniteNodeKind::Future { .. }));
    assert_eq!(
        strong[2].kind,
        InfiniteNodeKind::And {
            left: NodeId(2),
            right: NodeId(3)
        }
    );
}

// Trace: TC-145, FR-289-AC-3
#[test]
fn strict_unbounded_reader_honors_lowered_node_and_depth_limits() {
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(1),
        vec![
            InfiniteNode::new(InfiniteNodeKind::True),
            InfiniteNode::new(InfiniteNodeKind::Future {
                interval: TemporalInterval::Unbounded(UnboundedInterval::new(0)),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap();
    let bytes = doc.canonical_json_bytes().unwrap();
    let limits = SyntaxArtifactLimits {
        formula_nodes: 1,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        InfiniteFormulaDocument::from_json_bytes(&bytes, limits),
        Err(tl_syntax::StrictDocumentReadError::ResourceLimitExceeded {
            resource: "formula nodes",
            ..
        })
    ));
    let limits = SyntaxArtifactLimits {
        formula_nodes: 2,
        formula_depth: 1,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        InfiniteFormulaDocument::from_json_bytes(&bytes, limits),
        Err(tl_syntax::StrictDocumentReadError::ResourceLimitExceeded {
            resource: "formula depth",
            ..
        })
    ));
}

proptest::proptest! {
    // Trace: TC-144, FR-289-AC-1
    #[test]
    fn every_u32_lower_bound_constructs_a_distinct_unbounded_interval(start: u32) {
        let interval = UnboundedInterval::new(start);
        proptest::prop_assert_eq!(interval.start(), start);
        let open = TemporalInterval::Unbounded(interval);
        let closed = TemporalInterval::Closed(Interval::new(start, start).unwrap());
        proptest::prop_assert_ne!(open, closed);
    }
}

// Trace: TC-145, FR-289-AC-3
#[test]
fn every_infinite_node_wire_branch_is_decoded_and_unknown_fields_refuse() {
    use InfiniteNodeKind as K;
    let span = tl_syntax::SourceSpan::new(0, 2).unwrap();
    let interval = TemporalInterval::Closed(Interval::new(0, 1).unwrap());
    let kinds = [
        K::False,
        K::True,
        K::Proposition {
            proposition: tl_syntax::PropositionId(7),
        },
        K::Not { operand: NodeId(0) },
        K::And {
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Or {
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Implies {
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Equivalent {
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Future {
            interval,
            operand: NodeId(0),
        },
        K::Globally {
            interval,
            operand: NodeId(0),
        },
        K::Until {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Release {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Once {
            interval,
            operand: NodeId(0),
        },
        K::Historically {
            interval,
            operand: NodeId(0),
        },
        K::StrongPrevious { operand: NodeId(0) },
        K::Since {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
        K::Triggered {
            interval,
            left: NodeId(0),
            right: NodeId(1),
        },
    ];
    for kind in kinds {
        let node = InfiniteNode::with_span(kind, span);
        let bytes = serde_json::to_vec(&node).unwrap();
        assert_eq!(
            serde_json::from_slice::<InfiniteNode>(&bytes).unwrap(),
            node
        );
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        value["unexpected"] = 1.into();
        assert!(serde_json::from_value::<InfiniteNode>(value).is_err());
    }
}

// Trace: TC-145, TC-160, FR-289-AC-3, FR-024-AC-1
#[test]
fn unbounded_owner_schema_bytes_match_published_digest() {
    use sha2::{Digest, Sha256};
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(tl_syntax::FORMULA_UNBOUNDED_V1_SCHEMA_BYTES)
        ),
        tl_syntax::FORMULA_UNBOUNDED_V1_SCHEMA_SHA256
    );
    let schema: serde_json::Value =
        serde_json::from_str(tl_syntax::FORMULA_UNBOUNDED_V1_SCHEMA).unwrap();
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        tl_syntax::FORMULA_UNBOUNDED_V1
    );
    assert_eq!(schema["properties"]["clock"]["const"], "event_position");
    assert_eq!(
        schema["$defs"]["interval"]["oneOf"][1]["properties"]["kind"]["const"],
        "unbounded"
    );
}
