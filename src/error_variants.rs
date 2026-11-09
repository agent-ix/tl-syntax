//! Produces every variant of every public error enum (NFR-006-AC-4).
//!
//! Each enum lists its variants once. That list drives an exhaustive `match`, so
//! adding a variant breaks the build until it is listed, and the same list is the
//! set a test must produce. The enums are `#[non_exhaustive]` for other crates,
//! which is why this check lives inside the crate.

use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::{
    contracts::limits::OWNER_PROPOSITIONS, lower_infinite_future, select_infinite_profile,
    FairnessPremisesDocument, FairnessPremisesError, FairnessReadError, FixedDecimalSignalDomain,
    Formula, FormulaBindingError, FormulaConversionError, FormulaDocument, FormulaError,
    FutureKind, FutureLoweringRefusal, FutureLoweringRequest, InfiniteClock, InfiniteClockError,
    InfiniteFormulaDocument, InfiniteFormulaError, InfiniteNode, InfiniteNodeKind,
    InfiniteProfileError, IntegerSignalDomain, Interval, LassoTraceDocument, LassoTraceError, Node,
    NodeId, NodeKind, PartialValuation, PartialValuationError, PartialValue, PropositionBinding,
    PropositionEntry, PropositionId, PropositionMapDocument, PropositionMapError, RawBounds,
    RequirementContext, RequirementContextError, SemanticProfile, SignalCatalog,
    SignalCatalogError, SignalDeclaration, SignalDomain, SignalDomainError, SignalId, SourceSpan,
    StrictDocumentReadError, SyntaxArtifactLimits, TemporalInterval, TemporalIntervalParseError,
    TraceObservation, UnboundedInterval, ValuationEntry, FUTURE_LOWERING_REQUEST_V1,
    FUTURE_OPERATORS_V1, MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES,
    MAX_FUTURE_LOWERING_IDENTITY_BYTES, MAX_FUTURE_LOWERING_KIND_BYTES,
    MAX_REQUIREMENT_CONTEXT_FIELD_BYTES, MAX_SIGNAL_CATALOG_BINDINGS, MAX_SIGNAL_CATALOG_SIGNALS,
    MAX_SIGNAL_NAME_BYTES,
};

trait Named {
    fn variant(&self) -> &'static str;
}

/// Declares an enum's variant list: the exhaustive name match and the full set.
macro_rules! variants {
    ($ty:ident, $all:ident: $($variant:ident),+ $(,)?) => {
        impl Named for $ty {
            fn variant(&self) -> &'static str {
                match self {
                    $($ty::$variant { .. } => stringify!($variant),)+
                }
            }
        }
        const $all: &[&str] = &[$(stringify!($variant)),+];
    };
}

variants!(RequirementContextError, CONTEXT: EmptyField, FieldTooLong);
variants!(
    FutureLoweringRefusal, LOWERING:
    RequestIdentityTooLong, UnknownRequestIdentity, OperatorProfileTooLong,
    UnknownOperatorProfile, KindTooLong, UnsupportedKind, UnknownKind, SemanticProfileTooLong,
    UnknownSemanticProfile, SemanticProfileMismatch, MissingInterval, IntervalBoundOutOfRange,
    InvertedInterval, OperandIdOutOfRange, OperandAbsent, SpanHalfMissing,
    SpanEndpointOutOfRange, InvertedSpan, OperatorSpanOutsideExpression, NodeCountOverflow,
    GeneratedIdOutOfRange, DocumentNodeLimitExceeded,
);
variants!(
    StrictDocumentReadError, READER:
    DocumentTooLarge, DepthLimitExceeded, StringTooLarge, WorkLimitExceeded,
    ResourceLimitExceeded, NonCanonicalDocument, InvalidDocument,
);
variants!(TemporalIntervalParseError, INTERVAL: Malformed, InvalidBound, Inverted);
variants!(InfiniteClockError, CLOCK: Missing, Unsupported);
variants!(InfiniteProfileError, PROFILE: Missing, Mismatched, Unknown);
variants!(
    InfiniteFormulaError, INFINITE:
    Profile, Root, Operand, NodeLimit, DepthLimit, GeneratedIdentityOverflow,
);
variants!(
    FormulaError, FORMULA:
    RootOutOfRange, TooManyNodes, DocumentNodeLimitExceeded, DocumentDepthLimitExceeded,
    OperandNotPreceding, ProfileIncompatibleNode, FormulaV1ProfileUnsupported,
    InfiniteProfileRequiresUnboundedEdition, FormulaV1NodeUnsupported,
);
variants!(PropositionMapError, MAP: EmptyName, IdentityNotIncreasing, DuplicateName);
variants!(
    SignalDomainError, DOMAIN: IntegerBoundsInverted, DecimalBoundsInverted, DecimalScaleOutOfRange,
);
variants!(FormulaConversionError, CONVERSION: UnsupportedSemanticProfile);
variants!(
    PartialValuationError, VALUATION:
    MissingMapIdentity, MapOrder, ForeignProposition, DuplicateProposition,
    UnorderedProposition, OmittedProposition, ResourceLimit,
);
variants!(
    LassoTraceError, LASSO:
    Profile, ProfileIdentity, Clock, EmptyLoop, Position, MapIdentityMismatch, Valuation,
    ResourceLimit,
);
variants!(
    FairnessPremisesError, FAIRNESS:
    ForeignRoot, DuplicateRoot, UnorderedRoot, Profile, Clock, MissingGraphIdentity,
    GraphIdentityMismatch, ResourceLimit,
);
variants!(FairnessReadError, FAIRNESS_READ: Document, Binding);
variants!(
    SignalCatalogError, CATALOG:
    SignalLimitExceeded, BindingLimitExceeded, NameOrderScratchTooSmall,
    SignalIdentityNotIncreasing, EmptySignalName, SignalNameTooLong, DuplicateSignalName,
    BindingIdentityNotIncreasing, BindingTargetMissing, NonBooleanPropositionBinding,
);
variants!(FormulaBindingError, BINDING: MissingPropositionBinding);

/// The variants produced by one test, checked against the enum's full list.
#[derive(Default)]
struct Seen(BTreeSet<&'static str>);

impl Seen {
    /// Asserts that `error` is the variant `expected` and records it.
    #[track_caller]
    fn is<E: Named + core::fmt::Debug>(&mut self, expected: &'static str, error: E) {
        assert_eq!(error.variant(), expected, "{error:?}");
        self.0.insert(expected);
    }

    /// Asserts every listed variant was produced. Variants in `heavy` need
    /// allocations far too large for Miri; the native run produces them.
    #[track_caller]
    fn complete(self, all: &[&'static str], heavy: &[&'static str]) {
        let expected: BTreeSet<&str> = all
            .iter()
            .copied()
            .filter(|name| !(cfg!(miri) && heavy.contains(name)))
            .collect();
        assert_eq!(self.0, expected);
    }
}

fn err<T: core::fmt::Debug, E>(result: Result<T, E>) -> E {
    match result {
        Ok(value) => panic!("expected a refusal, got {value:?}"),
        Err(error) => error,
    }
}

fn span(start: u32, end: u32) -> SourceSpan {
    SourceSpan::new(start, end).unwrap()
}

fn proposition_nodes(count: usize) -> Vec<Node> {
    (0..count)
        .map(|index| {
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(index as u32),
            })
        })
        .collect()
}

fn not_chain(length: usize) -> Vec<Node> {
    let mut nodes = vec![Node::new(NodeKind::True)];
    for index in 1..length {
        nodes.push(Node::new(NodeKind::Not {
            operand: NodeId((index - 1) as u32),
        }));
    }
    nodes
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn requirement_context_errors() {
    let mut seen = Seen::default();
    let ok = "x";
    let long = "x".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1);
    seen.is(
        "EmptyField",
        err(RequirementContext::new("", ok, ok, ok, span(0, 1))),
    );
    seen.is(
        "FieldTooLong",
        err(RequirementContext::new(ok, &long, ok, ok, span(0, 1))),
    );
    seen.complete(CONTEXT, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn future_lowering_refusals() {
    let nodes = proposition_nodes(3);
    let formula = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(2), &nodes).unwrap();
    let valid = FutureLoweringRequest {
        request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
        operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
        kind: b"M",
        semantic_profile: formula.profile().as_str().as_bytes(),
        formula,
        left: 0,
        right: 2,
        interval: Some(RawBounds::new(2, 5)),
        operator_span: None,
        expression_span: None,
    };
    let wide = u64::from(u32::MAX) + 1;
    let spans = |operator, expression| FutureLoweringRequest {
        operator_span: Some(operator),
        expression_span: Some(expression),
        ..valid
    };
    let long_identity = [b'x'; MAX_FUTURE_LOWERING_IDENTITY_BYTES + 1];
    let long_kind = [b'W'; MAX_FUTURE_LOWERING_KIND_BYTES + 1];
    let mut seen = Seen::default();
    let cases: [(&'static str, FutureLoweringRequest<'_>); 19] = [
        (
            "RequestIdentityTooLong",
            FutureLoweringRequest {
                request_identity: &long_identity,
                ..valid
            },
        ),
        (
            "UnknownRequestIdentity",
            FutureLoweringRequest {
                request_identity: b"other",
                ..valid
            },
        ),
        (
            "OperatorProfileTooLong",
            FutureLoweringRequest {
                operator_profile: &long_identity,
                ..valid
            },
        ),
        (
            "UnknownOperatorProfile",
            FutureLoweringRequest {
                operator_profile: b"other",
                ..valid
            },
        ),
        (
            "KindTooLong",
            FutureLoweringRequest {
                kind: &long_kind,
                ..valid
            },
        ),
        (
            "UnsupportedKind",
            FutureLoweringRequest {
                kind: b"X",
                ..valid
            },
        ),
        (
            "UnknownKind",
            FutureLoweringRequest {
                kind: b"Z",
                ..valid
            },
        ),
        (
            "SemanticProfileTooLong",
            FutureLoweringRequest {
                semantic_profile: &long_identity,
                ..valid
            },
        ),
        (
            "UnknownSemanticProfile",
            FutureLoweringRequest {
                semantic_profile: b"other",
                ..valid
            },
        ),
        (
            "SemanticProfileMismatch",
            FutureLoweringRequest {
                semantic_profile: SemanticProfile::ClosedTraceV1.as_str().as_bytes(),
                ..valid
            },
        ),
        (
            "MissingInterval",
            FutureLoweringRequest {
                interval: None,
                ..valid
            },
        ),
        (
            "IntervalBoundOutOfRange",
            FutureLoweringRequest {
                interval: Some(RawBounds::new(wide, 0)),
                ..valid
            },
        ),
        (
            "InvertedInterval",
            FutureLoweringRequest {
                interval: Some(RawBounds::new(5, 4)),
                ..valid
            },
        ),
        (
            "OperandIdOutOfRange",
            FutureLoweringRequest {
                left: wide,
                ..valid
            },
        ),
        ("OperandAbsent", FutureLoweringRequest { left: 3, ..valid }),
        (
            "SpanHalfMissing",
            FutureLoweringRequest {
                operator_span: Some(RawBounds::new(1, 2)),
                ..valid
            },
        ),
        (
            "SpanEndpointOutOfRange",
            spans(RawBounds::new(1, wide), RawBounds::new(0, 4)),
        ),
        (
            "InvertedSpan",
            spans(RawBounds::new(2, 1), RawBounds::new(0, 4)),
        ),
        (
            "OperatorSpanOutsideExpression",
            spans(RawBounds::new(3, 5), RawBounds::new(0, 4)),
        ),
    ];
    for (name, request) in cases {
        seen.is(name, err(request.lower()));
    }
    // The node-budget refusals come from the private preflight, whose length is a
    // parameter; real tables cannot reach the overflow and identity branches.
    seen.is(
        "NodeCountOverflow",
        err(crate::future::preflight_node_ids(usize::MAX - 2)),
    );
    if usize::BITS > u32::BITS {
        seen.is(
            "GeneratedIdOutOfRange",
            err(crate::future::preflight_node_ids(u32::MAX as usize)),
        );
    }
    seen.is(
        "DocumentNodeLimitExceeded",
        err(crate::future::preflight_node_ids(
            MAX_FORMULA_DOCUMENT_NODES - 2,
        )),
    );
    let on_32_bit = usize::BITS <= u32::BITS;
    let all: Vec<&str> = LOWERING
        .iter()
        .copied()
        .filter(|name| !(on_32_bit && *name == "GeneratedIdOutOfRange"))
        .collect();
    seen.complete(&all, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn strict_document_read_errors() {
    let document = FormulaDocument::new(
        SemanticProfile::ClosedTraceV1,
        NodeId(0),
        vec![Node::new(NodeKind::True)],
    )
    .unwrap();
    let bytes = document.canonical_json_bytes().unwrap();
    let read = |bytes: &[u8], limits| FormulaDocument::from_json_bytes(bytes, limits);
    let default = SyntaxArtifactLimits::default();
    let lowered = |edit: fn(&mut SyntaxArtifactLimits)| {
        let mut limits = default;
        edit(&mut limits);
        limits
    };
    let pretty =
        serde_json::to_vec_pretty(&serde_json::from_slice::<serde_json::Value>(&bytes).unwrap())
            .unwrap();
    let mut seen = Seen::default();
    seen.is(
        "DocumentTooLarge",
        err(read(
            &bytes,
            SyntaxArtifactLimits {
                document_bytes: bytes.len() - 1,
                ..default
            },
        )),
    );
    seen.is(
        "DepthLimitExceeded",
        err(read(&bytes, lowered(|l| l.json_depth = 1))),
    );
    seen.is(
        "StringTooLarge",
        err(read(&bytes, lowered(|l| l.string_bytes = 1))),
    );
    seen.is(
        "WorkLimitExceeded",
        err(read(&bytes, lowered(|l| l.work = 1))),
    );
    seen.is(
        "ResourceLimitExceeded",
        err(read(&bytes, lowered(|l| l.formula_nodes = 0))),
    );
    seen.is("NonCanonicalDocument", err(read(&pretty, default)));
    seen.is("InvalidDocument", err(read(b"{", default)));
    seen.complete(READER, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn infinite_profile_clock_and_interval_errors() {
    let mut seen = Seen::default();
    seen.is("Malformed", err("x".parse::<TemporalInterval>()));
    seen.is("InvalidBound", err("[a,5]".parse::<TemporalInterval>()));
    seen.is("Inverted", err("[5,2]".parse::<TemporalInterval>()));
    seen.complete(INTERVAL, &[]);

    let mut seen = Seen::default();
    seen.is("Missing", err(InfiniteClock::try_from("")));
    seen.is("Unsupported", err(InfiniteClock::try_from("wall_clock")));
    seen.complete(CLOCK, &[]);

    let mut seen = Seen::default();
    seen.is("Missing", err(select_infinite_profile(None)));
    seen.is(
        "Mismatched",
        err(select_infinite_profile(Some(
            SemanticProfile::ClosedTraceV1.as_str(),
        ))),
    );
    seen.is("Unknown", err(select_infinite_profile(Some("other"))));
    seen.complete(PROFILE, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn infinite_formula_errors() {
    let new = |profile, root, nodes: Vec<InfiniteNode>| {
        InfiniteFormulaDocument::new(profile, InfiniteClock::EventPosition, NodeId(root), nodes)
    };
    let infinite = SemanticProfile::InfiniteTraceV1;
    let truth = || InfiniteNode::new(InfiniteNodeKind::True);
    let mut seen = Seen::default();
    seen.is(
        "Profile",
        err(new(SemanticProfile::ClosedTraceV1, 0, vec![truth()])),
    );
    seen.is("Root", err(new(infinite, 5, vec![truth()])));
    seen.is(
        "Operand",
        err(new(
            infinite,
            0,
            vec![InfiniteNode::new(InfiniteNodeKind::Not {
                operand: NodeId(0),
            })],
        )),
    );
    #[cfg(not(miri))]
    {
        seen.is(
            "NodeLimit",
            err(new(
                infinite,
                0,
                vec![truth(); MAX_FORMULA_DOCUMENT_NODES + 1],
            )),
        );
        let mut chain = vec![truth()];
        for index in 1..=MAX_FORMULA_DOCUMENT_DEPTH {
            chain.push(InfiniteNode::new(InfiniteNodeKind::Not {
                operand: NodeId((index - 1) as u32),
            }));
        }
        seen.is("DepthLimit", err(new(infinite, 0, chain)));
    }
    seen.is(
        "GeneratedIdentityOverflow",
        err(lower_infinite_future(
            FutureKind::WeakUntil,
            NodeId(0),
            NodeId(1),
            TemporalInterval::Unbounded(UnboundedInterval::new(0)),
            NodeId(u32::MAX),
            None,
        )),
    );
    seen.complete(INFINITE, &["NodeLimit", "DepthLimit"]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn formula_graph_and_conversion_errors() {
    let closed = SemanticProfile::ClosedTraceV1;
    let history = SemanticProfile::OriginCompleteHistoryV1;
    let truth = || vec![Node::new(NodeKind::True)];
    let past = || {
        vec![
            Node::new(NodeKind::True),
            Node::new(NodeKind::Once {
                interval: Interval::new(0, 1).unwrap(),
                operand: NodeId(0),
            }),
        ]
    };
    let mut seen = Seen::default();
    seen.is(
        "RootOutOfRange",
        err(Formula::new(closed, NodeId(1), &truth())),
    );
    seen.is(
        "OperandNotPreceding",
        err(Formula::new(
            closed,
            NodeId(0),
            &[Node::new(NodeKind::Not { operand: NodeId(0) })],
        )),
    );
    seen.is(
        "ProfileIncompatibleNode",
        err(Formula::new(closed, NodeId(1), &past())),
    );
    seen.is(
        "TooManyNodes",
        err(crate::formula::graph::check_node_count(
            u32::MAX as usize + 2,
        )),
    );
    #[cfg(not(miri))]
    {
        seen.is(
            "DocumentNodeLimitExceeded",
            err(FormulaDocument::new(
                closed,
                NodeId(0),
                vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES + 1],
            )),
        );
        seen.is(
            "DocumentDepthLimitExceeded",
            err(FormulaDocument::new_v2(
                closed,
                NodeId(MAX_FORMULA_DOCUMENT_DEPTH as u32),
                not_chain(MAX_FORMULA_DOCUMENT_DEPTH + 1),
            )),
        );
    }
    seen.is(
        "FormulaV1ProfileUnsupported",
        err(FormulaDocument::new(history, NodeId(1), past())),
    );
    seen.is(
        "InfiniteProfileRequiresUnboundedEdition",
        err(FormulaDocument::new(
            SemanticProfile::InfiniteTraceV1,
            NodeId(0),
            truth(),
        )),
    );
    seen.is(
        "FormulaV1NodeUnsupported",
        err(FormulaDocument::new(closed, NodeId(1), past())),
    );
    seen.complete(
        FORMULA,
        &["DocumentNodeLimitExceeded", "DocumentDepthLimitExceeded"],
    );

    let mut seen = Seen::default();
    let v2_history = FormulaDocument::new_v2(history, NodeId(1), past()).unwrap();
    seen.is("UnsupportedSemanticProfile", err(v2_history.try_to_v1()));
    seen.complete(CONVERSION, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn proposition_map_and_signal_domain_errors() {
    let entry = |id: u32, name: &str| PropositionEntry {
        id: PropositionId(id),
        name: name.into(),
    };
    let mut seen = Seen::default();
    seen.is(
        "EmptyName",
        err(PropositionMapDocument::new(vec![entry(1, "")])),
    );
    seen.is(
        "IdentityNotIncreasing",
        err(PropositionMapDocument::new(vec![
            entry(2, "a"),
            entry(1, "b"),
        ])),
    );
    seen.is(
        "DuplicateName",
        err(PropositionMapDocument::new(vec![
            entry(1, "a"),
            entry(2, "a"),
        ])),
    );
    seen.complete(MAP, &[]);

    let mut seen = Seen::default();
    seen.is("IntegerBoundsInverted", err(IntegerSignalDomain::new(9, 8)));
    seen.is(
        "DecimalBoundsInverted",
        err(FixedDecimalSignalDomain::new(2, 1, 2)),
    );
    seen.is(
        "DecimalScaleOutOfRange",
        err(FixedDecimalSignalDomain::new(1, 2, 19)),
    );
    seen.complete(DOMAIN, &[]);
}

fn propositions() -> Vec<PropositionId> {
    vec![PropositionId(1), PropositionId(3)]
}

fn entries(values: &[(u32, PartialValue)]) -> Vec<ValuationEntry> {
    values
        .iter()
        .map(|(proposition, value)| ValuationEntry {
            proposition: PropositionId(*proposition),
            value: *value,
        })
        .collect()
}

fn valuation(map: &str) -> PartialValuation {
    PartialValuation::new(
        map.into(),
        &propositions(),
        entries(&[(1, PartialValue::True), (3, PartialValue::False)]),
    )
    .unwrap()
}

fn observation(position: u32, map: &str) -> TraceObservation {
    TraceObservation {
        position,
        valuation: valuation(map),
    }
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn partial_valuation_errors() {
    let make = |map: &str, props: &[PropositionId], values: &[(u32, PartialValue)]| {
        PartialValuation::new(map.into(), props, entries(values))
    };
    let both = [(1, PartialValue::True), (3, PartialValue::False)];
    let mut seen = Seen::default();
    seen.is("MissingMapIdentity", err(make("", &propositions(), &both)));
    seen.is(
        "MapOrder",
        err(make("m", &[PropositionId(3), PropositionId(1)], &both)),
    );
    seen.is(
        "ForeignProposition",
        err(make(
            "m",
            &propositions(),
            &[(1, PartialValue::True), (2, PartialValue::True)],
        )),
    );
    seen.is(
        "DuplicateProposition",
        err(make(
            "m",
            &propositions(),
            &[(1, PartialValue::True), (1, PartialValue::True)],
        )),
    );
    seen.is(
        "UnorderedProposition",
        err(make(
            "m",
            &propositions(),
            &[(3, PartialValue::True), (1, PartialValue::True)],
        )),
    );
    seen.is(
        "OmittedProposition",
        err(make("m", &propositions(), &[(1, PartialValue::True)])),
    );
    #[cfg(not(miri))]
    seen.is(
        "ResourceLimit",
        err(PartialValuation::new(
            "m".into(),
            &[],
            vec![
                ValuationEntry {
                    proposition: PropositionId(1),
                    value: PartialValue::True
                };
                OWNER_PROPOSITIONS + 1
            ],
        )),
    );
    seen.complete(VALUATION, &["ResourceLimit"]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn lasso_trace_errors() {
    let infinite = SemanticProfile::InfiniteTraceV1;
    let event = InfiniteClock::EventPosition;
    let make = |profile, prefix: Vec<TraceObservation>, loop_part: Vec<TraceObservation>| {
        LassoTraceDocument::new(
            profile,
            event,
            "m".into(),
            propositions(),
            prefix,
            loop_part,
        )
    };
    let mut seen = Seen::default();
    seen.is(
        "Profile",
        err(make(
            SemanticProfile::ClosedTraceV1,
            Vec::new(),
            vec![observation(0, "m")],
        )),
    );
    seen.is(
        "ProfileIdentity",
        err(LassoTraceDocument::from_selected_identities(
            None,
            "event_position",
            "m".into(),
            propositions(),
            Vec::new(),
            vec![observation(0, "m")],
        )),
    );
    seen.is(
        "Clock",
        err(LassoTraceDocument::from_selected_identities(
            Some(infinite.as_str()),
            "wall",
            "m".into(),
            propositions(),
            Vec::new(),
            vec![observation(0, "m")],
        )),
    );
    seen.is("EmptyLoop", err(make(infinite, Vec::new(), Vec::new())));
    seen.is(
        "Position",
        err(make(infinite, Vec::new(), vec![observation(1, "m")])),
    );
    seen.is(
        "MapIdentityMismatch",
        err(make(infinite, Vec::new(), vec![observation(0, "other")])),
    );
    let sparse = TraceObservation {
        position: 0,
        valuation: PartialValuation::new(
            "m".into(),
            &[PropositionId(1)],
            entries(&[(1, PartialValue::True)]),
        )
        .unwrap(),
    };
    seen.is("Valuation", err(make(infinite, Vec::new(), vec![sparse])));
    #[cfg(not(miri))]
    {
        let long: Vec<TraceObservation> = (0..=MAX_FORMULA_DOCUMENT_NODES as u32)
            .map(|position| observation(position, "m"))
            .collect();
        seen.is("ResourceLimit", err(make(infinite, Vec::new(), long)));
    }
    seen.complete(LASSO, &["ResourceLimit"]);
}

fn fairness_formula() -> InfiniteFormulaDocument {
    InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(1),
        vec![
            InfiniteNode::new(InfiniteNodeKind::Proposition {
                proposition: PropositionId(1),
            }),
            InfiniteNode::new(InfiniteNodeKind::Globally {
                interval: TemporalInterval::Unbounded(UnboundedInterval::new(0)),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap()
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn fairness_premise_errors() {
    let formula = fairness_formula();
    let identity = formula.content_identity().unwrap();
    let event = InfiniteClock::EventPosition;
    let make = |identity: &str, roots: Vec<NodeId>| {
        FairnessPremisesDocument::new(&formula, identity.into(), event, roots)
    };
    let mut seen = Seen::default();
    seen.is("ForeignRoot", err(make(&identity, vec![NodeId(9)])));
    seen.is(
        "DuplicateRoot",
        err(make(&identity, vec![NodeId(1), NodeId(1)])),
    );
    seen.is(
        "UnorderedRoot",
        err(make(&identity, vec![NodeId(1), NodeId(0)])),
    );
    seen.is("GraphIdentityMismatch", err(make("other", vec![NodeId(1)])));
    seen.is(
        "Profile",
        err(FairnessPremisesDocument::from_selected_identities(
            &formula,
            identity.clone(),
            Some(SemanticProfile::ClosedTraceV1.as_str()),
            "event_position",
            Vec::new(),
        )),
    );
    seen.is(
        "Clock",
        err(FairnessPremisesDocument::from_selected_identities(
            &formula,
            identity.clone(),
            Some(SemanticProfile::InfiniteTraceV1.as_str()),
            "wall",
            Vec::new(),
        )),
    );
    // An empty graph identity is only reachable through the wire conversion.
    let wire = r#"{"schema_version":"tl-syntax.fairness-premises/v1","semantic_profile":"mltl.infinite-trace/v1","clock":"event_position","graph_identity":"","roots":[]}"#;
    let message = serde_json::from_str::<FairnessPremisesDocument>(wire)
        .unwrap_err()
        .to_string();
    assert!(message.contains("MissingGraphIdentity"), "{message}");
    seen.is(
        "MissingGraphIdentity",
        err(FairnessPremisesDocument::new_unchecked_graph(
            SemanticProfile::InfiniteTraceV1,
            event,
            String::new(),
            Vec::new(),
        )),
    );
    #[cfg(not(miri))]
    seen.is(
        "ResourceLimit",
        err(make(
            &identity,
            vec![NodeId(0); MAX_FORMULA_DOCUMENT_NODES + 1],
        )),
    );
    seen.complete(FAIRNESS, &["ResourceLimit"]);

    let mut seen = Seen::default();
    let canonical = FairnessPremisesDocument::new(&formula, identity.clone(), event, Vec::new())
        .unwrap()
        .canonical_json_bytes()
        .unwrap();
    let limits = SyntaxArtifactLimits::default();
    seen.is(
        "Document",
        err(FairnessPremisesDocument::from_json_bytes(
            b"{", limits, &formula,
        )),
    );
    let foreign =
        FairnessPremisesDocument::new(&formula, identity, event, vec![NodeId(1)]).unwrap();
    let other = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        event,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True)],
    )
    .unwrap();
    seen.is(
        "Binding",
        err(FairnessPremisesDocument::from_json_bytes(
            &foreign.canonical_json_bytes().unwrap(),
            limits,
            &other,
        )),
    );
    assert!(!canonical.is_empty());
    seen.complete(FAIRNESS_READ, &[]);
}

// Trace: TC-166, NFR-006-AC-4
#[test]
fn signal_catalog_and_binding_errors() {
    let boolean = |id: u32, name| SignalDeclaration::new(SignalId(id), name, SignalDomain::Boolean);
    let new =
        |signals: &[SignalDeclaration<'_>], bindings: &[PropositionBinding], scratch: usize| {
            let mut order = vec![0_u32; scratch];
            SignalCatalog::new(signals, bindings, &mut order).map(|_| ())
        };
    let valid = [boolean(1, "a"), boolean(2, "b")];
    let bind = |proposition: u32, signal: u32| {
        PropositionBinding::new(PropositionId(proposition), SignalId(signal))
    };
    let integer = SignalDomain::Integer(IntegerSignalDomain::new(0, 9).unwrap());
    let mut seen = Seen::default();
    #[cfg(not(miri))]
    {
        seen.is(
            "SignalLimitExceeded",
            err(new(
                &vec![boolean(1, "a"); MAX_SIGNAL_CATALOG_SIGNALS + 1],
                &[],
                0,
            )),
        );
        seen.is(
            "BindingLimitExceeded",
            err(new(
                &valid,
                &vec![bind(1, 1); MAX_SIGNAL_CATALOG_BINDINGS + 1],
                2,
            )),
        );
    }
    seen.is("NameOrderScratchTooSmall", err(new(&valid, &[], 1)));
    seen.is(
        "SignalIdentityNotIncreasing",
        err(new(&[boolean(1, "a"), boolean(1, "b")], &[], 2)),
    );
    seen.is("EmptySignalName", err(new(&[boolean(1, "")], &[], 1)));
    let long = "x".repeat(MAX_SIGNAL_NAME_BYTES + 1);
    seen.is("SignalNameTooLong", err(new(&[boolean(1, &long)], &[], 1)));
    seen.is(
        "DuplicateSignalName",
        err(new(&[boolean(1, "a"), boolean(2, "a")], &[], 2)),
    );
    seen.is(
        "BindingIdentityNotIncreasing",
        err(new(&valid, &[bind(2, 1), bind(1, 1)], 2)),
    );
    seen.is("BindingTargetMissing", err(new(&valid, &[bind(1, 9)], 2)));
    seen.is(
        "NonBooleanPropositionBinding",
        err(new(
            &[
                boolean(1, "a"),
                SignalDeclaration::new(SignalId(2), "b", integer),
            ],
            &[bind(1, 2)],
            2,
        )),
    );
    seen.complete(CATALOG, &["SignalLimitExceeded", "BindingLimitExceeded"]);

    let mut order = [0_u32; 2];
    let bindings = [bind(1, 1)];
    let catalog = SignalCatalog::new(&valid, &bindings, &mut order).unwrap();
    let nodes = [Node::new(NodeKind::Proposition {
        proposition: PropositionId(7),
    })];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes).unwrap();
    let mut seen = Seen::default();
    seen.is(
        "MissingPropositionBinding",
        err(catalog.bind_formula(formula)),
    );
    seen.complete(BINDING, &[]);
}
