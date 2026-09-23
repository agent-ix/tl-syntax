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

// Trace: TC-146, FR-290-AC-1, FR-290-AC-2, FR-290-AC-3
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
    let doc = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True)],
    )
    .unwrap();
    let bytes = doc.canonical_json_bytes().unwrap();
    for clock in ["fixed_sample", "dense", "timestamped"] {
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
