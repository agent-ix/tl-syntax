//! Tests that kill the core-module mutants the first cargo-mutants run left
//! alive (TL-231). Each test names the mutant it targets.

use tl_syntax::{
    Formula, FormulaError, Interval, IntervalError, Node, NodeId, NodeKind, PastOperatorKind,
    SemanticProfile, SourceSpan, SourceSpanError,
};

/// Kills `Interval::cardinality`: replace `&&` with `||`.
#[test]
fn cardinality_is_none_only_for_the_full_u32_range() {
    assert_eq!(Interval::new(0, u32::MAX).unwrap().cardinality(), None);
    assert_eq!(Interval::new(0, 5).unwrap().cardinality(), Some(6));
    assert_eq!(
        Interval::new(3, u32::MAX).unwrap().cardinality(),
        Some(u32::MAX - 2)
    );
    assert_eq!(Interval::new(4, 4).unwrap().cardinality(), Some(1));
}

/// Kills `SourceSpan::is_empty -> false`.
#[test]
fn a_span_is_empty_exactly_when_it_has_zero_length() {
    assert!(SourceSpan::new(7, 7).unwrap().is_empty());
    assert!(!SourceSpan::new(7, 8).unwrap().is_empty());
}

/// Kills the `Display` mutants for `IntervalError`, `SourceSpanError` and
/// `FormulaError` (`Ok(Default::default())`).
#[test]
fn error_displays_name_the_rejected_values() {
    assert_eq!(
        Interval::new(5, 2).unwrap_err().to_string(),
        "inclusive interval start 5 exceeds end 2"
    );
    let _: IntervalError = Interval::new(5, 2).unwrap_err();
    assert_eq!(
        SourceSpan::new(9, 1).unwrap_err().to_string(),
        "source span start 9 exceeds end 1"
    );
    let _: SourceSpanError = SourceSpan::new(9, 1).unwrap_err();
    let nodes = [Node::new(NodeKind::True)];
    let error = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(5), &nodes).unwrap_err();
    assert!(matches!(error, FormulaError::RootOutOfRange { .. }));
    assert_eq!(
        error.to_string(),
        "formula root 5 is outside node table of length 1"
    );
}

/// Kills the deleted `past_operator` match arms: every past node reports its
/// own catalog value, every future node reports none.
#[test]
fn past_operator_maps_each_past_node_to_its_catalog_value() {
    let interval = Interval::new(0, 1).unwrap();
    let a = NodeId(0);
    let cases = [
        (
            NodeKind::Once {
                interval,
                operand: a,
            },
            PastOperatorKind::Once,
        ),
        (
            NodeKind::Historically {
                interval,
                operand: a,
            },
            PastOperatorKind::Historically,
        ),
        (
            NodeKind::StrongPrevious { operand: a },
            PastOperatorKind::StrongPrevious,
        ),
        (
            NodeKind::Since {
                interval,
                left: a,
                right: a,
            },
            PastOperatorKind::Since,
        ),
        (
            NodeKind::Triggered {
                interval,
                left: a,
                right: a,
            },
            PastOperatorKind::Triggered,
        ),
    ];
    for (kind, expected) in cases {
        assert_eq!(kind.past_operator(), Some(expected));
    }
    assert_eq!(NodeKind::True.past_operator(), None);
    assert_eq!(
        NodeKind::Future {
            interval,
            operand: a
        }
        .past_operator(),
        None
    );
}
