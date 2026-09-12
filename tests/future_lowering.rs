use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

use proptest::prelude::*;
use tl_syntax::{
    Formula, FutureKind, FutureLowering, FutureLoweringAxis, FutureLoweringOperand,
    FutureLoweringRefusal, FutureLoweringRequest, FutureLoweringSpanRole, Interval, Node, NodeId,
    NodeKind, PropositionId, RawBounds, SemanticProfile, SourceSpan, UnsupportedFutureKind,
    FUTURE_LOWERING_NODE_CHARGE, FUTURE_LOWERING_REFUSAL_V1, FUTURE_LOWERING_REPORT_V1,
    FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1, MAX_FORMULA_DOCUMENT_NODES,
    MAX_FUTURE_LOWERING_IDENTITY_BYTES, MAX_FUTURE_LOWERING_KIND_BYTES,
};

/// Counts heap allocations on the current thread so lowering can prove it makes none.
struct CountingAllocator;

thread_local! {
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

// SAFETY: every method forwards the caller's layout and pointer unchanged to `System`.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
        // SAFETY: the layout is the caller's, forwarded under the same contract.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the pointer came from `System.alloc` with this layout.
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn lower_counting_allocations(
    request: FutureLoweringRequest<'_>,
) -> (Result<FutureLowering, FutureLoweringRefusal>, usize) {
    let before = ALLOCATIONS.with(Cell::get);
    let result = request.lower();
    (result, ALLOCATIONS.with(Cell::get) - before)
}

const UNSUPPORTED: [(&[u8], UnsupportedFutureKind); 6] = [
    (b"X", UnsupportedFutureKind::Next),
    (b"Y", UnsupportedFutureKind::Previous),
    (b"O", UnsupportedFutureKind::Once),
    (b"H", UnsupportedFutureKind::Historically),
    (b"S", UnsupportedFutureKind::Since),
    (b"T", UnsupportedFutureKind::Triggered),
];

const PROFILES: [SemanticProfile; 2] = [
    SemanticProfile::ClosedTraceV1,
    SemanticProfile::OnlinePrefixV1,
];

fn propositions(count: usize) -> Vec<Node> {
    (0..count)
        .map(|index| {
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(u32::try_from(index).unwrap()),
            })
        })
        .collect()
}

fn request<'a>(
    formula: Formula<'a>,
    kind: &'a [u8],
    left: u32,
    right: u32,
) -> FutureLoweringRequest<'a> {
    FutureLoweringRequest {
        request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
        operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
        kind,
        semantic_profile: formula.profile().as_str().as_bytes(),
        formula,
        left: u64::from(left),
        right: u64::from(right),
        interval: Some(RawBounds::new(2, 5)),
        operator_span: None,
        expression_span: None,
    }
}

fn raw_span(span: SourceSpan) -> RawBounds {
    RawBounds::new(u64::from(span.start()), u64::from(span.end()))
}

fn id(index: usize) -> NodeId {
    NodeId(u32::try_from(index).unwrap())
}

/// Transcribes the FR-008 lowering table for one derived form.
fn direct_expansion(
    kind: FutureKind,
    interval: Interval,
    p: NodeId,
    q: NodeId,
    base: usize,
    span: Option<SourceSpan>,
) -> [Node; 3] {
    let (first, second) = (id(base), id(base + 1));
    let kinds = match kind {
        FutureKind::WeakUntil => [
            NodeKind::Until {
                interval,
                left: p,
                right: q,
            },
            NodeKind::Globally {
                interval,
                operand: p,
            },
            NodeKind::Or {
                left: first,
                right: second,
            },
        ],
        FutureKind::StrongRelease => [
            NodeKind::Release {
                interval,
                left: p,
                right: q,
            },
            NodeKind::Future {
                interval,
                operand: p,
            },
            NodeKind::And {
                left: first,
                right: second,
            },
        ],
    };
    kinds.map(|kind| Node { kind, span })
}

fn node_table() -> impl Strategy<Value = Vec<Node>> {
    prop::collection::vec(
        (0_u8..12, any::<u32>(), any::<u32>(), any::<(u32, u32)>()),
        1..24,
    )
    .prop_map(|entries| {
        entries
            .into_iter()
            .enumerate()
            .map(|(index, (selector, first, second, (start, end)))| {
                let interval = Interval::new(start.min(end), start.max(end)).unwrap();
                let bound = u32::try_from(index).unwrap().max(1);
                let (left, right) = (NodeId(first % bound), NodeId(second % bound));
                let kind = match (index, selector) {
                    (0, _) | (_, 0) => NodeKind::Proposition {
                        proposition: PropositionId(first),
                    },
                    (_, 1) => NodeKind::True,
                    (_, 2) => NodeKind::False,
                    (_, 3) => NodeKind::Not { operand: left },
                    (_, 4) => NodeKind::And { left, right },
                    (_, 5) => NodeKind::Or { left, right },
                    (_, 6) => NodeKind::Implies { left, right },
                    (_, 7) => NodeKind::Equivalent { left, right },
                    (_, 8) => NodeKind::Future {
                        interval,
                        operand: left,
                    },
                    (_, 9) => NodeKind::Globally {
                        interval,
                        operand: left,
                    },
                    (_, 10) => NodeKind::Until {
                        interval,
                        left,
                        right,
                    },
                    _ => NodeKind::Release {
                        interval,
                        left,
                        right,
                    },
                };
                Node::new(kind)
            })
            .collect()
    })
}

fn profile() -> impl Strategy<Value = SemanticProfile> {
    prop::sample::select(PROFILES.to_vec())
}

fn boundary_interval() -> impl Strategy<Value = Interval> {
    prop_oneof![
        Just(Interval::new(0, 0).unwrap()),
        Just(Interval::new(u32::MAX, u32::MAX).unwrap()),
        Just(Interval::new(0, u32::MAX).unwrap()),
        any::<(u32, u32)>().prop_map(|(a, b)| Interval::new(a.min(b), a.max(b)).unwrap()),
    ]
}

fn span_pair() -> impl Strategy<Value = Option<(SourceSpan, SourceSpan)>> {
    prop_oneof![
        Just(None),
        Just(Some((
            SourceSpan::new(u32::MAX, u32::MAX).unwrap(),
            SourceSpan::new(0, u32::MAX).unwrap(),
        ))),
        any::<[u32; 4]>().prop_map(|mut offsets| {
            offsets.sort_unstable();
            let [outer_start, token_start, token_end, outer_end] = offsets;
            Some((
                SourceSpan::new(token_start, token_end).unwrap(),
                SourceSpan::new(outer_start, outer_end).unwrap(),
            ))
        }),
    ]
}

#[allow(clippy::too_many_arguments)]
fn assert_exact_lowering(
    nodes: &[Node],
    profile: SemanticProfile,
    kind: FutureKind,
    p_seed: u32,
    q_seed: u32,
    interval: Interval,
    spans: Option<(SourceSpan, SourceSpan)>,
) {
    let snapshot = nodes.to_vec();
    let base = nodes.len();
    let root = id(base - 1);
    let formula = Formula::new(profile, root, nodes).unwrap();
    let bound = u32::try_from(base).unwrap();
    let (p, q) = (NodeId(p_seed % bound), NodeId(q_seed % bound));
    let mut raw = request(formula, kind.as_str().as_bytes(), p.0, q.0);
    raw.interval = Some(RawBounds::new(
        u64::from(interval.start()),
        u64::from(interval.end()),
    ));
    raw.operator_span = spans.map(|(operator, _)| raw_span(operator));
    raw.expression_span = spans.map(|(_, expression)| raw_span(expression));

    let (result, allocations) = lower_counting_allocations(raw);
    assert_eq!(allocations, 0, "lowering must not allocate");
    let lowering = result.unwrap();
    let expression = spans.map(|(_, expression)| expression);
    assert_eq!(
        lowering.nodes(),
        &direct_expansion(kind, interval, p, q, base, expression)
    );
    assert_eq!(lowering.node_ids(), [id(base), id(base + 1), id(base + 2)]);
    assert_eq!(lowering.root(), id(base + 2));

    let report = lowering.report();
    assert_eq!(report.identity(), FUTURE_LOWERING_REPORT_V1);
    assert_eq!(report.request_identity(), FUTURE_LOWERING_REQUEST_V1);
    assert_eq!(report.operator_profile(), FUTURE_OPERATORS_V1);
    assert_eq!(report.kind(), kind);
    assert_eq!(report.semantic_profile(), profile);
    assert_eq!((report.left(), report.right()), (p, q));
    assert_eq!(report.root(), id(base + 2));
    assert_eq!(report.first_generated(), id(base));
    assert_eq!(report.generated_count(), FUTURE_LOWERING_NODE_CHARGE);
    assert_eq!(report.operator_span(), spans.map(|(operator, _)| operator));
    assert_eq!(report.expression_span(), expression);

    assert_eq!(raw.lower(), Ok(lowering), "equal inputs give equal outputs");
    assert_eq!(nodes, snapshot.as_slice(), "caller graph is not mutated");

    let mut appended = snapshot;
    appended.extend_from_slice(lowering.nodes());
    let lowered = Formula::new(profile, lowering.root(), &appended).unwrap();
    assert_eq!(lowered.profile(), profile);
    assert_eq!(&lowered.nodes()[..base], nodes, "operand graphs are reused");
}

proptest! {
    // Trace: TC-041, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4
    #[test]
    fn weak_until_lowers_to_until_globally_or(
        nodes in node_table(),
        profile in profile(),
        p in any::<u32>(),
        q in any::<u32>(),
        interval in boundary_interval(),
        spans in span_pair(),
    ) {
        assert_exact_lowering(&nodes, profile, FutureKind::WeakUntil, p, q, interval, spans);
    }

    // Trace: TC-042, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4
    #[test]
    fn strong_release_lowers_to_release_future_and(
        nodes in node_table(),
        profile in profile(),
        p in any::<u32>(),
        q in any::<u32>(),
        interval in boundary_interval(),
        spans in span_pair(),
    ) {
        assert_exact_lowering(&nodes, profile, FutureKind::StrongRelease, p, q, interval, spans);
    }
}

// Trace: TC-040, FR-008-AC-1, FR-009-AC-2, FR-009-AC-3
#[test]
fn contract_identities_are_exact_and_orthogonal() {
    let identities = [
        FUTURE_OPERATORS_V1,
        FUTURE_LOWERING_REQUEST_V1,
        FUTURE_LOWERING_REPORT_V1,
        FUTURE_LOWERING_REFUSAL_V1,
        "tl-syntax.formula/v1",
        "tl-parse.clean-ascii/v1",
        "tl-parse.clean-ascii/v2",
        SemanticProfile::ClosedTraceV1.as_str(),
        SemanticProfile::OnlinePrefixV1.as_str(),
    ];
    assert_eq!(
        identities[..4],
        [
            "tl-syntax.future-operators/v1",
            "tl-syntax.future-lowering-request/v1",
            "tl-syntax.future-lowering-report/v1",
            "tl-syntax.future-lowering-refusal/v1",
        ]
    );
    for (index, identity) in identities.iter().enumerate() {
        assert!(identity.len() <= MAX_FUTURE_LOWERING_IDENTITY_BYTES);
        assert!(!identities[index + 1..].contains(identity));
    }
    assert_eq!(
        FutureLoweringRefusal::UnknownKind.identity(),
        FUTURE_LOWERING_REFUSAL_V1
    );

    let nodes = propositions(2);
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
    let valid = request(formula, b"W", 0, 1);
    let successors = [
        (
            FutureLoweringRequest {
                request_identity: b"tl-syntax.future-lowering-request/v2",
                ..valid
            },
            FutureLoweringRefusal::UnknownRequestIdentity,
        ),
        (
            FutureLoweringRequest {
                operator_profile: b"tl-syntax.future-operators/v2",
                ..valid
            },
            FutureLoweringRefusal::UnknownOperatorProfile,
        ),
        (
            FutureLoweringRequest {
                operator_profile: b"tl-parse.clean-ascii/v2",
                ..valid
            },
            FutureLoweringRefusal::UnknownOperatorProfile,
        ),
        (
            FutureLoweringRequest {
                semantic_profile: b"mltl.closed-trace/v2",
                ..valid
            },
            FutureLoweringRefusal::UnknownSemanticProfile,
        ),
    ];
    for (successor, refusal) in successors {
        assert_eq!(successor.lower(), Err(refusal));
    }
    assert!(valid.lower().is_ok());
}

// Trace: TC-040, FR-008-AC-1, FR-009-AC-4
#[test]
fn operator_catalog_admits_only_w_and_m() {
    let nodes = propositions(2);
    let formula = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(1), &nodes).unwrap();
    let mut spellings: Vec<Vec<u8>> = (0..=u8::MAX).map(|byte| vec![byte]).collect();
    spellings.extend((0..=u16::MAX).map(|pair| pair.to_be_bytes().to_vec()));
    spellings.extend(
        [
            &b"WX"[..],
            b"w",
            b"m",
            b"U",
            b"R",
            b"F",
            b"G",
            b"Weak",
            b"WeakUntil",
            b"StrongRelease",
            b"Once",
            b"Since",
            b"",
        ]
        .map(<[u8]>::to_vec),
    );
    for spelling in &spellings {
        let result = request(formula, spelling, 0, 1).lower();
        match spelling.as_slice() {
            b"W" => assert_eq!(result.unwrap().report().kind(), FutureKind::WeakUntil),
            b"M" => assert_eq!(result.unwrap().report().kind(), FutureKind::StrongRelease),
            other => {
                let expected = UNSUPPORTED
                    .iter()
                    .find(|(unsupported, _)| *unsupported == other)
                    .map_or(FutureLoweringRefusal::UnknownKind, |(_, kind)| {
                        FutureLoweringRefusal::UnsupportedKind { kind: *kind }
                    });
                assert_eq!(result, Err(expected), "kind {other:?}");
            }
        }
    }
    for (spelling, kind) in UNSUPPORTED {
        assert_eq!(kind.as_str().as_bytes(), spelling);
        let refusal = request(formula, spelling, 0, 1).lower().unwrap_err();
        assert_eq!(refusal.code(), "unsupported_kind");
        assert_eq!(refusal.axis(), FutureLoweringAxis::Kind);
    }
    assert_eq!(FutureLoweringRefusal::UnknownKind.code(), "unknown_kind");
    let over_limit = [b'W'; MAX_FUTURE_LOWERING_KIND_BYTES + 1];
    assert_eq!(
        request(formula, &over_limit, 0, 1).lower(),
        Err(FutureLoweringRefusal::KindTooLong {
            len: MAX_FUTURE_LOWERING_KIND_BYTES + 1
        })
    );
}

#[cfg(feature = "serde")]
// Trace: TC-040, TC-044, FR-008-AC-2, FR-009-AC-2
#[test]
fn lowered_graphs_use_the_unchanged_primitive_formula_wire() {
    use tl_syntax::{FormulaDocument, FormulaSchemaVersion};

    const PRIMITIVE_TAGS: [&str; 12] = [
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
    ];

    for profile in PROFILES {
        let interval = Interval::new(1, 4).unwrap();
        let expression = SourceSpan::new(0, 9).unwrap();
        let operator = SourceSpan::new(2, 3).unwrap();
        let mut lowered = propositions(2);

        let formula = Formula::new(profile, NodeId(1), &lowered).unwrap();
        let mut weak = request(formula, b"W", 0, 1);
        weak.interval = Some(RawBounds::new(1, 4));
        weak.operator_span = Some(raw_span(operator));
        weak.expression_span = Some(raw_span(expression));
        let weak = weak.lower().unwrap();
        lowered.extend_from_slice(weak.nodes());

        let formula = Formula::new(profile, weak.root(), &lowered).unwrap();
        let mut nested = request(formula, b"M", weak.root().0, 0);
        nested.interval = Some(RawBounds::new(1, 4));
        let nested = nested.lower().unwrap();
        lowered.extend_from_slice(nested.nodes());

        let mut direct = propositions(2);
        direct.extend(direct_expansion(
            FutureKind::WeakUntil,
            interval,
            NodeId(0),
            NodeId(1),
            2,
            Some(expression),
        ));
        direct.extend(direct_expansion(
            FutureKind::StrongRelease,
            interval,
            NodeId(4),
            NodeId(0),
            5,
            None,
        ));
        let lowered = FormulaDocument::new(profile, nested.root(), lowered).unwrap();
        let direct = FormulaDocument::new(profile, NodeId(7), direct).unwrap();
        assert_eq!(lowered, direct);
        assert_eq!(lowered.schema_version(), FormulaSchemaVersion::V1);
        assert_eq!(lowered.semantic_profile(), profile);
        let wire = serde_json::to_string(&lowered).unwrap();
        assert_eq!(wire, serde_json::to_string(&direct).unwrap());
        assert_eq!(
            serde_json::from_str::<FormulaDocument>(&wire).unwrap(),
            direct
        );
        let value: serde_json::Value = serde_json::from_str(&wire).unwrap();
        for node in value["nodes"].as_array().unwrap() {
            assert!(PRIMITIVE_TAGS.contains(&node["kind"].as_str().unwrap()));
        }

        let spanless = FormulaDocument::new(
            profile,
            NodeId(7),
            lowered
                .nodes()
                .iter()
                .map(|node| Node::new(node.kind))
                .collect(),
        )
        .unwrap();
        assert_ne!(lowered, spanless, "wire keeps diagnostic spans");
        assert_eq!(lowered.semantic_view(), spanless.semantic_view());
    }
}

#[cfg(feature = "serde")]
// Trace: TC-046, FR-009-AC-4
#[test]
fn derived_nodes_are_refused_by_the_formula_wire() {
    for kind in [
        "weak_until",
        "strong_release",
        "next",
        "weak_next",
        "since",
        "once",
    ] {
        let document = format!(
            r#"{{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":2,"nodes":[{{"kind":"true"}},{{"kind":"false"}},{{"kind":"{kind}","interval":{{"start":0,"end":1}},"left":0,"right":1}}]}}"#
        );
        assert!(serde_json::from_str::<tl_syntax::FormulaDocument>(&document).is_err());
    }
}

// Trace: TC-046, FR-008-AC-1, FR-008-AC-3, FR-009-AC-4
#[test]
fn refused_profile_combinations_identify_their_axis() {
    let nodes = propositions(2);
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
    let valid = request(formula, b"M", 1, 0);
    let cases = [
        (
            FutureLoweringRequest {
                kind: b"X",
                ..valid
            },
            FutureLoweringRefusal::UnsupportedKind {
                kind: UnsupportedFutureKind::Next,
            },
        ),
        (
            FutureLoweringRequest {
                kind: b"S",
                ..valid
            },
            FutureLoweringRefusal::UnsupportedKind {
                kind: UnsupportedFutureKind::Since,
            },
        ),
        (
            FutureLoweringRequest {
                interval: None,
                ..valid
            },
            FutureLoweringRefusal::MissingInterval,
        ),
        (
            FutureLoweringRequest {
                interval: Some(RawBounds::new(0, u64::MAX)),
                ..valid
            },
            FutureLoweringRefusal::IntervalBoundOutOfRange {
                start: 0,
                end: u64::MAX,
            },
        ),
        (
            FutureLoweringRequest {
                semantic_profile: b"mltl.dense-time/v1",
                ..valid
            },
            FutureLoweringRefusal::UnknownSemanticProfile,
        ),
        (
            FutureLoweringRequest {
                semantic_profile: b"mltl.timestamped/v1",
                ..valid
            },
            FutureLoweringRefusal::UnknownSemanticProfile,
        ),
        (
            FutureLoweringRequest {
                semantic_profile: b"mltl.closed-trace/v1;unit=ms",
                ..valid
            },
            FutureLoweringRefusal::UnknownSemanticProfile,
        ),
        (
            FutureLoweringRequest {
                semantic_profile: b"mltl.window-closure/v1",
                ..valid
            },
            FutureLoweringRefusal::UnknownSemanticProfile,
        ),
        (
            FutureLoweringRequest {
                semantic_profile: SemanticProfile::OnlinePrefixV1.as_str().as_bytes(),
                ..valid
            },
            FutureLoweringRefusal::SemanticProfileMismatch {
                requested: SemanticProfile::OnlinePrefixV1,
                formula: SemanticProfile::ClosedTraceV1,
            },
        ),
        (
            FutureLoweringRequest {
                request_identity: &[0xff; 4],
                ..valid
            },
            FutureLoweringRefusal::UnknownRequestIdentity,
        ),
        (
            FutureLoweringRequest {
                operator_profile: &[b'x'; MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1],
                ..valid
            },
            FutureLoweringRefusal::OperatorProfileTooLong {
                len: MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1,
            },
        ),
    ];
    for (case, refusal) in cases {
        let (result, allocations) = lower_counting_allocations(case);
        assert_eq!(allocations, 0);
        assert_eq!(result, Err(refusal));
        assert_eq!(result.unwrap_err().identity(), FUTURE_LOWERING_REFUSAL_V1);
    }
}

// Trace: TC-044, TC-046, FR-008-AC-1, FR-008-AC-3
#[test]
fn document_node_limit_is_fixed_and_charged_three_nodes() {
    let fits = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES - 3];
    let formula = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(0), &fits).unwrap();
    let lowering = request(formula, b"W", 0, 0).lower().unwrap();
    assert_eq!(lowering.root(), id(MAX_FORMULA_DOCUMENT_NODES - 1));

    let over = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES - 2];
    let formula = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(0), &over).unwrap();
    let limit = FutureLoweringRefusal::DocumentNodeLimitExceeded {
        node_count: MAX_FORMULA_DOCUMENT_NODES + 1,
        limit: MAX_FORMULA_DOCUMENT_NODES,
    };
    let (result, allocations) = lower_counting_allocations(request(formula, b"M", 0, 0));
    assert_eq!(allocations, 0);
    assert_eq!(result, Err(limit));
    assert_eq!(limit.axis(), FutureLoweringAxis::NodeBudget);

    let mut uncontained = request(formula, b"M", 0, 0);
    uncontained.operator_span = Some(RawBounds::new(8, 9));
    uncontained.expression_span = Some(RawBounds::new(0, 4));
    assert_eq!(
        uncontained.lower().unwrap_err().code(),
        "operator_span_outside_expression"
    );
    let absent = request(formula, b"M", 0, u32::try_from(over.len()).unwrap());
    assert_eq!(absent.lower().unwrap_err().code(), "operand_absent");
}

// Trace: TC-041, TC-042, TC-044, TC-046, FR-010-AC-4
#[test]
fn lowering_mutants_change_the_expected_graph_or_report() {
    let nodes = propositions(3);
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(2), &nodes).unwrap();
    let interval = Interval::new(2, 5).unwrap();
    let operator = SourceSpan::new(4, 5).unwrap();
    let expression = SourceSpan::new(1, 9).unwrap();
    let mut raw = request(formula, b"W", 0, 2);
    raw.operator_span = Some(raw_span(operator));
    raw.expression_span = Some(raw_span(expression));
    let actual = raw.lower().unwrap();
    let (p, q) = (NodeId(0), NodeId(2));
    let span = Some(expression);
    let expected = direct_expansion(FutureKind::WeakUntil, interval, p, q, 3, span);
    assert_eq!(actual.nodes(), &expected);

    let endpoint = |start, end| Interval::new(start, end).unwrap();
    let [until, globally, or] = expected;
    let graph_mutants = [
        // lowering branch
        direct_expansion(FutureKind::StrongRelease, interval, p, q, 3, span),
        // generated-node order
        [globally, until, or],
        // inclusive endpoints
        direct_expansion(FutureKind::WeakUntil, endpoint(3, 5), p, q, 3, span),
        direct_expansion(FutureKind::WeakUntil, endpoint(2, 4), p, q, 3, span),
        direct_expansion(FutureKind::WeakUntil, endpoint(2, 6), p, q, 3, span),
        // associativity and operand order
        [
            until,
            globally,
            Node {
                kind: NodeKind::Or {
                    left: NodeId(4),
                    right: NodeId(3),
                },
                span,
            },
        ],
        direct_expansion(FutureKind::WeakUntil, interval, q, p, 3, span),
        // node charge shifts every absolute identity
        direct_expansion(FutureKind::WeakUntil, interval, p, q, 4, span),
        // span attribution
        direct_expansion(FutureKind::WeakUntil, interval, p, q, 3, Some(operator)),
        direct_expansion(FutureKind::WeakUntil, interval, p, q, 3, None),
    ];
    for mutant in graph_mutants {
        assert_ne!(actual.nodes(), &mutant);
    }
    assert_ne!(actual.node_ids(), [3, 4, 5, 6].map(NodeId)[1..]);
    assert_eq!(actual.report().generated_count(), 3);

    let report = actual.report();
    assert_ne!(report.operator_span(), report.expression_span());
    assert_eq!(
        (report.operator_span(), report.expression_span()),
        (Some(operator), Some(expression))
    );

    let online = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(2), &nodes).unwrap();
    let mut other_profile = raw;
    other_profile.formula = online;
    other_profile.semantic_profile = SemanticProfile::OnlinePrefixV1.as_str().as_bytes();
    let other_profile = other_profile.lower().unwrap();
    assert_eq!(other_profile.nodes(), actual.nodes());
    assert_ne!(other_profile.report(), actual.report());

    let mut unspanned = raw;
    unspanned.operator_span = None;
    unspanned.expression_span = None;
    assert_ne!(unspanned.lower().unwrap().report(), actual.report());
}

#[derive(Clone, Copy, Debug)]
enum IdentityChoice {
    Valid,
    TooLong(usize),
    Unknown(u8),
}

#[derive(Clone, Copy, Debug)]
enum KindChoice {
    Valid(bool),
    TooLong(usize),
    Unsupported(usize),
    Unknown(u8),
}

#[derive(Clone, Copy, Debug)]
enum SemanticChoice {
    Valid,
    TooLong(usize),
    Unknown(u8),
    Mismatch,
}

#[derive(Clone, Copy, Debug)]
enum IntervalChoice {
    Valid(u32, u32),
    Missing,
    OutOfRange(u64, u64),
    Inverted(u32, u32),
}

#[derive(Clone, Copy, Debug)]
enum OperandChoice {
    Valid(u32),
    OutOfRange(u64),
    Absent(u32),
}

#[derive(Clone, Copy, Debug)]
enum SpanChoice {
    Valid,
    Missing,
    OutOfRange(u64, u64),
    Inverted(u32, u32),
}

fn identity_choice() -> impl Strategy<Value = IdentityChoice> {
    prop_oneof![
        Just(IdentityChoice::Valid),
        (MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1..512).prop_map(IdentityChoice::TooLong),
        any::<u8>().prop_map(IdentityChoice::Unknown),
    ]
}

fn kind_choice() -> impl Strategy<Value = KindChoice> {
    prop_oneof![
        any::<bool>().prop_map(KindChoice::Valid),
        (MAX_FUTURE_LOWERING_KIND_BYTES + 1..64).prop_map(KindChoice::TooLong),
        (0..UNSUPPORTED.len()).prop_map(KindChoice::Unsupported),
        any::<u8>().prop_map(KindChoice::Unknown),
    ]
}

fn semantic_choice() -> impl Strategy<Value = SemanticChoice> {
    prop_oneof![
        Just(SemanticChoice::Valid),
        (MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1..512).prop_map(SemanticChoice::TooLong),
        any::<u8>().prop_map(SemanticChoice::Unknown),
        Just(SemanticChoice::Mismatch),
    ]
}

fn wide() -> impl Strategy<Value = u64> {
    u64::from(u32::MAX) + 1..=u64::MAX
}

fn interval_choice() -> impl Strategy<Value = IntervalChoice> {
    prop_oneof![
        any::<(u32, u32)>().prop_map(|(a, b)| IntervalChoice::Valid(a.min(b), a.max(b))),
        Just(IntervalChoice::Missing),
        (wide(), any::<u64>(), any::<bool>()).prop_map(|(wide, any, swap)| if swap {
            IntervalChoice::OutOfRange(any, wide)
        } else {
            IntervalChoice::OutOfRange(wide, any)
        }),
        (1..=u32::MAX, any::<u32>())
            .prop_map(|(start, gap)| IntervalChoice::Inverted(start, gap % start)),
    ]
}

fn operand_choice() -> impl Strategy<Value = OperandChoice> {
    prop_oneof![
        any::<u32>().prop_map(OperandChoice::Valid),
        wide().prop_map(OperandChoice::OutOfRange),
        any::<u32>().prop_map(OperandChoice::Absent),
    ]
}

fn span_choice() -> impl Strategy<Value = SpanChoice> {
    prop_oneof![
        Just(SpanChoice::Valid),
        Just(SpanChoice::Missing),
        (wide(), any::<u64>()).prop_map(|(wide, any)| SpanChoice::OutOfRange(any, wide)),
        (1..=u32::MAX, any::<u32>())
            .prop_map(|(start, gap)| SpanChoice::Inverted(start, gap % start)),
    ]
}

/// A faulted field: its position in the FR-008 precedence and its refusal.
type Fault = Option<(u8, FutureLoweringRefusal)>;

fn identity_bytes(
    choice: IdentityChoice,
    valid: &str,
    slots: (u8, u8),
    too_long: fn(usize) -> FutureLoweringRefusal,
    unknown: FutureLoweringRefusal,
) -> (Vec<u8>, Fault) {
    match choice {
        IdentityChoice::Valid => (valid.as_bytes().to_vec(), None),
        IdentityChoice::TooLong(len) => (vec![b'a'; len], Some((slots.0, too_long(len)))),
        IdentityChoice::Unknown(byte) => {
            let mut bytes = valid.as_bytes().to_vec();
            let last = bytes.len() - 1;
            bytes[last] = if bytes[last] == byte { !byte } else { byte };
            (bytes, Some((slots.1, unknown)))
        }
    }
}

fn span_bounds(choice: SpanChoice, valid: SourceSpan) -> Option<RawBounds> {
    match choice {
        SpanChoice::Valid => Some(raw_span(valid)),
        SpanChoice::Missing => None,
        SpanChoice::OutOfRange(start, end) => Some(RawBounds::new(start, end)),
        SpanChoice::Inverted(start, end) => Some(RawBounds::new(u64::from(start), u64::from(end))),
    }
}

fn span_fault(
    operator: SpanChoice,
    expression: SpanChoice,
    uncontained: bool,
    operator_span: SourceSpan,
    expression_span: SourceSpan,
) -> Fault {
    use FutureLoweringSpanRole::{Expression, Operator};

    let range = |role, choice| match choice {
        SpanChoice::OutOfRange(start, end) => Some((
            18 + u8::from(role == Expression),
            FutureLoweringRefusal::SpanEndpointOutOfRange { role, start, end },
        )),
        _ => None,
    };
    let order = |role, choice| match choice {
        SpanChoice::Inverted(start, end) => Some((
            20 + u8::from(role == Expression),
            FutureLoweringRefusal::InvertedSpan { role, start, end },
        )),
        _ => None,
    };
    match (operator, expression) {
        (SpanChoice::Missing, SpanChoice::Missing) => None,
        (SpanChoice::Missing, _) => Some((
            17,
            FutureLoweringRefusal::SpanHalfMissing { missing: Operator },
        )),
        (_, SpanChoice::Missing) => Some((
            17,
            FutureLoweringRefusal::SpanHalfMissing {
                missing: Expression,
            },
        )),
        (SpanChoice::Valid, SpanChoice::Valid) if uncontained => Some((
            22,
            FutureLoweringRefusal::OperatorSpanOutsideExpression {
                operator: operator_span,
                expression: expression_span,
            },
        )),
        _ => range(Operator, operator)
            .or(range(Expression, expression))
            .or(order(Operator, operator))
            .or(order(Expression, expression)),
    }
}

proptest! {
    // Trace: TC-046, FR-008-AC-1, FR-008-AC-3, FR-009-AC-4, FR-010-AC-4
    #[test]
    fn every_raw_field_fault_refuses_in_stable_precedence(
        (node_count, profile) in (1_usize..8, profile()),
        request_choice in identity_choice(),
        operator_choice in identity_choice(),
        kind in kind_choice(),
        semantic in semantic_choice(),
        interval in interval_choice(),
        operands in (operand_choice(), operand_choice()),
        spans in (span_choice(), span_choice(), any::<bool>()),
    ) {
        let nodes = propositions(node_count);
        let formula = Formula::new(profile, id(node_count - 1), &nodes).unwrap();
        let node_count_u32 = u32::try_from(node_count).unwrap();
        let mut faults: Vec<Fault> = Vec::new();

        let (request_identity, fault) = identity_bytes(
            request_choice,
            FUTURE_LOWERING_REQUEST_V1,
            (0, 1),
            |len| FutureLoweringRefusal::RequestIdentityTooLong { len },
            FutureLoweringRefusal::UnknownRequestIdentity,
        );
        faults.push(fault);
        let (operator_profile, fault) = identity_bytes(
            operator_choice,
            FUTURE_OPERATORS_V1,
            (2, 3),
            |len| FutureLoweringRefusal::OperatorProfileTooLong { len },
            FutureLoweringRefusal::UnknownOperatorProfile,
        );
        faults.push(fault);

        let (kind_bytes, fault) = match kind {
            KindChoice::Valid(weak) => ((if weak { "W" } else { "M" }).as_bytes().to_vec(), None),
            KindChoice::TooLong(len) => {
                (vec![b'W'; len], Some((4, FutureLoweringRefusal::KindTooLong { len })))
            }
            KindChoice::Unsupported(index) => {
                let (spelling, kind) = UNSUPPORTED[index];
                (spelling.to_vec(), Some((5, FutureLoweringRefusal::UnsupportedKind { kind })))
            }
            KindChoice::Unknown(byte) => {
                let mut spelling = vec![b'W', byte];
                spelling.truncate(if byte.is_ascii_uppercase() { 2 } else { 1 + usize::from(byte % 2) });
                if spelling == b"W" {
                    spelling = b"WW".to_vec();
                }
                (spelling, Some((6, FutureLoweringRefusal::UnknownKind)))
            }
        };
        faults.push(fault);

        let other = PROFILES.into_iter().find(|candidate| *candidate != profile).unwrap();
        let (semantic_bytes, fault) = match semantic {
            SemanticChoice::Valid => (profile.as_str().as_bytes().to_vec(), None),
            SemanticChoice::TooLong(len) => (
                vec![b'm'; len],
                Some((7, FutureLoweringRefusal::SemanticProfileTooLong { len })),
            ),
            SemanticChoice::Unknown(byte) => {
                let (bytes, fault) = identity_bytes(
                    IdentityChoice::Unknown(byte),
                    profile.as_str(),
                    (8, 8),
                    |len| FutureLoweringRefusal::SemanticProfileTooLong { len },
                    FutureLoweringRefusal::UnknownSemanticProfile,
                );
                let fault = if bytes == other.as_str().as_bytes() {
                    Some((9, FutureLoweringRefusal::SemanticProfileMismatch {
                        requested: other,
                        formula: profile,
                    }))
                } else {
                    fault
                };
                (bytes, fault)
            }
            SemanticChoice::Mismatch => (
                other.as_str().as_bytes().to_vec(),
                Some((9, FutureLoweringRefusal::SemanticProfileMismatch {
                    requested: other,
                    formula: profile,
                })),
            ),
        };
        faults.push(fault);

        let (raw_interval, fault) = match interval {
            IntervalChoice::Valid(start, end) => {
                (Some(RawBounds::new(u64::from(start), u64::from(end))), None)
            }
            IntervalChoice::Missing => (None, Some((10, FutureLoweringRefusal::MissingInterval))),
            IntervalChoice::OutOfRange(start, end) => (
                Some(RawBounds::new(start, end)),
                Some((11, FutureLoweringRefusal::IntervalBoundOutOfRange { start, end })),
            ),
            IntervalChoice::Inverted(start, end) => (
                Some(RawBounds::new(u64::from(start), u64::from(end))),
                Some((12, FutureLoweringRefusal::InvertedInterval { start, end })),
            ),
        };
        faults.push(fault);

        let mut raw_operands = [0_u64; 2];
        for (index, (choice, operand)) in [
            (operands.0, FutureLoweringOperand::Left),
            (operands.1, FutureLoweringOperand::Right),
        ]
        .into_iter()
        .enumerate()
        {
            let slot_offset = u8::try_from(index).unwrap();
            let (raw, fault) = match choice {
                OperandChoice::Valid(seed) => (u64::from(seed % node_count_u32), None),
                OperandChoice::OutOfRange(id) => (
                    id,
                    Some((13 + slot_offset, FutureLoweringRefusal::OperandIdOutOfRange { operand, id })),
                ),
                OperandChoice::Absent(seed) => {
                    let absent = node_count_u32 + seed % (u32::MAX - node_count_u32 + 1);
                    (
                        u64::from(absent),
                        Some((15 + slot_offset, FutureLoweringRefusal::OperandAbsent {
                            operand,
                            id: NodeId(absent),
                            node_count,
                        })),
                    )
                }
            };
            raw_operands[index] = raw;
            faults.push(fault);
        }

        let (operator_choice, expression_choice, uncontained) = spans;
        let expression_span = SourceSpan::new(100, 200).unwrap();
        let operator_span = if uncontained {
            SourceSpan::new(150, 201).unwrap()
        } else {
            SourceSpan::new(120, 130).unwrap()
        };
        faults.push(span_fault(
            operator_choice,
            expression_choice,
            uncontained,
            operator_span,
            expression_span,
        ));

        let raw = FutureLoweringRequest {
            request_identity: &request_identity,
            operator_profile: &operator_profile,
            kind: &kind_bytes,
            semantic_profile: &semantic_bytes,
            formula,
            left: raw_operands[0],
            right: raw_operands[1],
            interval: raw_interval,
            operator_span: span_bounds(operator_choice, operator_span),
            expression_span: span_bounds(expression_choice, expression_span),
        };
        let expected = faults.into_iter().flatten().min_by_key(|(slot, _)| *slot);
        let (result, allocations) = lower_counting_allocations(raw);
        prop_assert_eq!(allocations, 0);
        match expected {
            Some((_, refusal)) => prop_assert_eq!(result, Err(refusal)),
            None => prop_assert!(result.is_ok()),
        }
        prop_assert_eq!(raw.lower(), result);
    }
}
