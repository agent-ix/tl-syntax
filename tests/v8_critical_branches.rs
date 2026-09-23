//! Feature-sliced admission cases for critical syntax branches (FR-050).

use tl_syntax::{
    select_infinite_profile, InfiniteClock, InfiniteClockError, InfiniteProfileError, Interval,
    SemanticProfile, TemporalInterval, TemporalIntervalParseError, UnboundedInterval,
};

// The infinite spelling and identity selectors are available without alloc or serde.
// Trace: TC-191, FR-050-AC-2
#[test]
fn core_infinite_spelling_and_identity_admission() {
    for (spelling, expected) in [
        (
            "[0,1]",
            Ok(TemporalInterval::Closed(Interval::new(0, 1).unwrap())),
        ),
        (
            "[1,)",
            Ok(TemporalInterval::Unbounded(UnboundedInterval::new(1))),
        ),
        ("[,1]", Err(TemporalIntervalParseError::InvalidBound)),
        ("[x,1]", Err(TemporalIntervalParseError::InvalidBound)),
        ("[1,]", Err(TemporalIntervalParseError::InvalidBound)),
        ("[1,x]", Err(TemporalIntervalParseError::InvalidBound)),
        ("[1,2)", Err(TemporalIntervalParseError::Malformed)),
        ("[2,1]", Err(TemporalIntervalParseError::Inverted)),
    ] {
        assert_eq!(spelling.parse::<TemporalInterval>(), expected, "{spelling}");
    }
    assert_eq!(
        InfiniteClock::try_from(""),
        Err(InfiniteClockError::Missing)
    );
    assert_eq!(
        InfiniteClock::try_from("event_position"),
        Ok(InfiniteClock::EventPosition)
    );
    assert_eq!(
        InfiniteClock::try_from("timestamp"),
        Err(InfiniteClockError::Unsupported)
    );
    assert_eq!(
        select_infinite_profile(None),
        Err(InfiniteProfileError::Missing)
    );
    assert_eq!(
        select_infinite_profile(Some("")),
        Err(InfiniteProfileError::Missing)
    );
    assert_eq!(
        select_infinite_profile(Some("mltl.infinite-trace/v1")),
        Ok(SemanticProfile::InfiniteTraceV1)
    );
    assert_eq!(
        select_infinite_profile(Some("mltl.closed-trace/v1")),
        Err(InfiniteProfileError::Mismatched(
            SemanticProfile::ClosedTraceV1
        ))
    );
    assert_eq!(
        select_infinite_profile(Some("unknown")),
        Err(InfiniteProfileError::Unknown)
    );
}

// These malformed inputs pass through the same strict preflight as owner documents.
// Trace: TC-191, FR-050-AC-2
#[cfg(feature = "serde")]
#[test]
fn strict_reader_rejects_malformed_scanning_boundaries() {
    use tl_syntax::{FormulaDocument, StrictDocumentReadError, SyntaxArtifactLimits};

    let malformed: &[&[u8]] = &[
        br#"{"nodes" []}"#,
        br#"{"nodes":{}}"#,
        br#"{"nodes":[}"#,
        br#"{"nodes":[[]]}"#,
        br#"{"nodes":[  ]}"#,
        br#"{"nodes" : []}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"unterminated"#,
        br#"{"nodes":[{"kind":"true"}],"x":"ends-with-\"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\u0041"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\u07FF"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\uD83D\uDE00"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"escaped \" quote"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\uD83D\n"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\uD83D\u0041"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\uZZZZ"}"#,
        br#"{"nodes":[{"kind":"true"}],"x":"\uD83D"}"#,
    ];
    for bytes in malformed {
        assert!(
            FormulaDocument::from_json_bytes(bytes, SyntaxArtifactLimits::default()).is_err(),
            "accepted malformed owner bytes: {bytes:?}"
        );
    }

    let bytes = br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"true"}]}"#;
    let work = SyntaxArtifactLimits {
        work: bytes.len() * 3 - 1,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        FormulaDocument::from_json_bytes(bytes, work),
        Err(StrictDocumentReadError::WorkLimitExceeded { .. })
    ));
    let string = SyntaxArtifactLimits {
        string_bytes: 2,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        FormulaDocument::from_json_bytes(bytes, string),
        Err(StrictDocumentReadError::StringTooLarge { .. })
    ));
}

// Trace: TC-191, FR-050-AC-2
#[cfg(feature = "alloc")]
#[test]
fn infinite_graph_rejects_invalid_boundaries_before_exposing_a_document() {
    use tl_syntax::{
        InfiniteFormulaDocument, InfiniteFormulaError, InfiniteNode, InfiniteNodeKind, NodeId,
        MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES,
    };

    let profile = SemanticProfile::InfiniteTraceV1;
    let clock = InfiniteClock::EventPosition;
    let atom = InfiniteNode::new(InfiniteNodeKind::True);
    assert_eq!(
        InfiniteFormulaDocument::new(profile, clock, NodeId(1), vec![atom]),
        Err(InfiniteFormulaError::Root { root: NodeId(1) })
    );
    assert_eq!(
        InfiniteFormulaDocument::new(
            profile,
            clock,
            NodeId(1),
            vec![
                atom,
                InfiniteNode::new(InfiniteNodeKind::Not { operand: NodeId(1) })
            ],
        ),
        Err(InfiniteFormulaError::Operand {
            node: NodeId(1),
            operand: NodeId(1)
        })
    );

    assert_eq!(
        InfiniteFormulaDocument::new(
            profile,
            clock,
            NodeId(0),
            vec![atom; MAX_FORMULA_DOCUMENT_NODES + 1]
        ),
        Err(InfiniteFormulaError::NodeLimit {
            actual: MAX_FORMULA_DOCUMENT_NODES + 1
        })
    );

    let mut deep = vec![atom];
    for index in 1..=MAX_FORMULA_DOCUMENT_DEPTH {
        deep.push(InfiniteNode::new(InfiniteNodeKind::Not {
            operand: NodeId((index - 1) as u32),
        }));
    }
    assert_eq!(
        InfiniteFormulaDocument::new(
            profile,
            clock,
            NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32),
            deep
        ),
        Err(InfiniteFormulaError::DepthLimit {
            node: NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32)
        })
    );
}

// A JSON-escaped field key can evade the byte preflight's literal key scan. The
// typed post-deserialization check and hard serde visitor cap must still refuse it.
// Trace: TC-191, FR-050-AC-2
#[cfg(feature = "serde")]
#[test]
fn escaped_nodes_key_cannot_bypass_typed_population_or_owner_ceiling() {
    use tl_syntax::{
        InfiniteFormulaDocument, StrictDocumentReadError, SyntaxArtifactLimits,
        MAX_FORMULA_DOCUMENT_NODES,
    };

    let small = br#"{"schema_version":"tl-syntax.formula-unbounded/v1","semantic_profile":"mltl.infinite-trace/v1","clock":"event_position","root":1,"nod\u0065s":[{"kind":"true"},{"kind":"true"}]}"#;
    let lowered = SyntaxArtifactLimits {
        formula_nodes: 1,
        ..SyntaxArtifactLimits::default()
    };
    assert!(matches!(
        InfiniteFormulaDocument::from_json_bytes(small, lowered),
        Err(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "formula nodes",
            actual: 2,
            limit: 1
        })
    ));

    let nodes = vec![r#"{"kind":"true"}"#; MAX_FORMULA_DOCUMENT_NODES + 1].join(",");
    let excessive = format!(
        r#"{{"schema_version":"tl-syntax.formula-unbounded/v1","semantic_profile":"mltl.infinite-trace/v1","clock":"event_position","root":0,"nod\u0065s":[{nodes}]}}"#
    );
    assert!(matches!(
        InfiniteFormulaDocument::from_json_bytes(
            excessive.as_bytes(),
            SyntaxArtifactLimits::default()
        ),
        Err(StrictDocumentReadError::InvalidDocument(error))
            if error.to_string().contains("infinite formula node limit exceeded")
    ));
}
