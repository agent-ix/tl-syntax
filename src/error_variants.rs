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
    AggregationRefusal, ContractAssumptionError, ContractObligationError, ContractSetError,
    ExpressionError, LtlfFormulaError, LtlfIdentityError, PredicateBindingError,
    PredicateTableError, SetAggregationError,
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
    SignalCatalogError, SignalDeclaration, SignalDomain, SignalDomainError, SignalId,
    SignalPartitionError, SourceSpan, StrictDocumentReadError, SyntaxArtifactLimits,
    TemporalInterval, TemporalIntervalParseError,
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
    InfiniteProfileRequiresUnboundedEdition, FormulaV1NodeUnsupported, LtlfNodeRefused,
    LtlfProfileRequiresLtlfEdition,
);
variants!(
    LtlfIdentityError, LTLF_IDENTITY:
    MissingEdition, UnknownEdition, MismatchedEdition, MissingProfile, UnknownProfile,
    MismatchedProfile,
);
variants!(LtlfFormulaError, LTLF_FORMULA: Identity, Refused, Graph);
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
    DirectionInV1Document,
);
variants!(FormulaBindingError, BINDING: MissingPropositionBinding);
variants!(SignalPartitionError, PARTITION: Binding, UndeclaredDirection);
variants!(
    ExpressionError, EXPRESSION:
    RangeOverflow, ScaleMismatch, ScaleOverflow, RescaleNarrowing, NonNumericSignal,
    UnknownSignal, ExpressionLimitExceeded, OperandNotPreceding, ScratchTooSmall,
    SlotOutsideTemplate,
);
variants!(
    PredicateTableError, PREDICATE_TABLE:
    Expression, PredicateLimitExceeded, PropositionNotIncreasing, RootOutOfRange, ScaleMismatch,
);
variants!(
    SetAggregationError, AGGREGATION:
    EmptySetName, SetNameTooLong, DuplicateSetName, UndeclaredMember, DuplicateMember,
    UnknownSet, QuantifierOutOfRange, AggregationTooDeep, InnerLeafMismatch, TemplateStructure,
    SlotOutsideTemplate, TemplatePredicate, TemplateIllTypedForMember,
    MemberHasNoBooleanBinding, FreshIdentityCollision, AggregationTooLarge,
    PredicateDefinedTwice, PlaceholderOutOfScope,
);
variants!(AggregationRefusal, REFUSAL: AggregationTooLarge, ExpansionTooSmall, Invalid);
variants!(
    ContractSetError, CONTRACT_SET:
    RootOutOfRange, ContractIdNotIncreasing, EmptyName, NameTooLong, DuplicateName,
    ContractLimitExceeded, GraphIdentityMismatch, ProfileMismatch, ClockMismatch,
);
variants!(ContractObligationError, OBLIGATION: RootOutOfRange, NodeLimit);
variants!(ContractAssumptionError, ASSUMPTION: RootOutOfRange, Binding, NotInput);
variants!(
    PredicateBindingError, PREDICATE_BINDING:
    PropositionUnbound, PropositionDefinedTwice, NonNumericSignal, IllTyped,
);

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
    seen.is(
        "LtlfNodeRefused",
        err(Formula::new(
            SemanticProfile::LtlfFiniteTraceV1,
            NodeId(1),
            &past(),
        )),
    );
    seen.is(
        "LtlfProfileRequiresLtlfEdition",
        err(FormulaDocument::new_v2(
            SemanticProfile::LtlfFiniteTraceV1,
            NodeId(0),
            truth(),
        )),
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
    seen.is(
        "DirectionInV1Document",
        err(crate::SignalCatalogDocument::new(
            vec![
                crate::OwnedSignalDeclaration::new(SignalId(1), "a".into(), SignalDomain::Boolean)
                    .with_direction(crate::SignalDirection::Input),
            ],
            vec![],
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

    let mut seen = Seen::default();
    seen.is("Binding", err(catalog.partition(formula)));
    let undeclared_nodes = [Node::new(NodeKind::Proposition {
        proposition: PropositionId(1),
    })];
    let undeclared =
        Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &undeclared_nodes).unwrap();
    seen.is("UndeclaredDirection", err(catalog.partition(undeclared)));
    seen.complete(PARTITION, &[]);
}

// Trace: TC-166, TC-1502, NFR-006-AC-4, FR-302-AC-2
#[test]
fn numeric_expression_table_and_binding_errors() {
    use crate::{
        BoundPredicateFormula, ComparisonOperator, ExprId, ExprNode, ExprSlot, ExpressionArena,
        Predicate, PredicateTable, MAX_EXPRESSION_DEPTH, MAX_EXPRESSION_NODES, MAX_PREDICATES,
    };
    let signals = [
        SignalDeclaration::new(SignalId(1), "flag", SignalDomain::Boolean),
        SignalDeclaration::new(
            SignalId(2),
            "count",
            SignalDomain::Integer(IntegerSignalDomain::new(i64::MIN, i64::MAX).unwrap()),
        ),
        SignalDeclaration::new(
            SignalId(3),
            "temp",
            SignalDomain::FixedDecimal(FixedDecimalSignalDomain::new(0, 9, 1).unwrap()),
        ),
    ];
    let bindings = [PropositionBinding::new(PropositionId(1), SignalId(1))];
    let mut order = [0_u32; 3];
    let catalog = SignalCatalog::new(&signals, &bindings, &mut order).unwrap();
    let typed = |nodes: &[ExprNode]| {
        let mut scratch = vec![ExprSlot::EMPTY; nodes.len()];
        ExpressionArena::new(catalog, nodes, &mut scratch).map(|_| ())
    };
    let read = |signal: u32| ExprNode::Signal {
        signal: SignalId(signal),
    };
    let pair = |kind: fn(ExprId, ExprId) -> ExprNode, left: ExprNode, right: ExprNode| {
        vec![left, right, kind(ExprId(0), ExprId(1))]
    };
    let mut seen = Seen::default();
    seen.is(
        "RangeOverflow",
        err(typed(&pair(
            |left, right| ExprNode::Mul { left, right },
            read(2),
            read(2),
        ))),
    );
    seen.is(
        "ScaleMismatch",
        err(typed(&pair(
            |left, right| ExprNode::Add { left, right },
            read(2),
            read(3),
        ))),
    );
    seen.is(
        "ScaleOverflow",
        err(typed(&[ExprNode::Decimal {
            coefficient: 1,
            scale: 19,
        }])),
    );
    seen.is(
        "RescaleNarrowing",
        err(typed(&[
            read(3),
            ExprNode::Rescale {
                operand: ExprId(0),
                scale: 0,
            },
        ])),
    );
    seen.is("NonNumericSignal", err(typed(&[read(1)])));
    seen.is("UnknownSignal", err(typed(&[read(9)])));
    let mut chain = vec![ExprNode::Integer { value: 0 }];
    for index in 0..MAX_EXPRESSION_DEPTH as u32 {
        chain.push(ExprNode::Neg {
            operand: ExprId(index),
        });
    }
    seen.is("ExpressionLimitExceeded", err(typed(&chain)));
    #[cfg(not(miri))]
    {
        let wide = vec![ExprNode::Integer { value: 0 }; MAX_EXPRESSION_NODES + 1];
        assert!(typed(&wide).is_err());
    }
    seen.is(
        "OperandNotPreceding",
        err(typed(&[ExprNode::Neg {
            operand: ExprId(0),
        }])),
    );
    let mut short = [ExprSlot::EMPTY; 0];
    seen.is(
        "ScratchTooSmall",
        err(ExpressionArena::new(catalog, &[read(2)], &mut short)),
    );
    seen.is(
        "SlotOutsideTemplate",
        err(typed(&[ExprNode::SlotSignal { depth: 0 }])),
    );
    seen.complete(EXPRESSION, &[]);

    let expressions = [read(2), read(3)];
    let predicate = |proposition: u32, left: u32, right: u32| Predicate {
        proposition: PropositionId(proposition),
        operator: ComparisonOperator::Lt,
        left: ExprId(left),
        right: ExprId(right),
    };
    let table = |predicates: &[Predicate], expressions: &[ExprNode]| {
        let mut scratch = vec![ExprSlot::EMPTY; expressions.len()];
        PredicateTable::new(catalog, predicates, expressions, &mut scratch).map(|_| ())
    };
    let mut seen = Seen::default();
    seen.is("Expression", err(table(&[], &[read(1)])));
    #[cfg(not(miri))]
    {
        let many: Vec<Predicate> = (0..=MAX_PREDICATES as u32)
            .map(|index| predicate(index + 2, 0, 0))
            .collect();
        seen.is("PredicateLimitExceeded", err(table(&many, &expressions)));
    }
    seen.is(
        "PropositionNotIncreasing",
        err(table(&[predicate(5, 0, 0), predicate(5, 0, 0)], &expressions)),
    );
    seen.is(
        "RootOutOfRange",
        err(table(&[predicate(5, 0, 7)], &expressions)),
    );
    seen.is(
        "ScaleMismatch",
        err(table(&[predicate(5, 0, 1)], &expressions)),
    );
    seen.complete(PREDICATE_TABLE, &["PredicateLimitExceeded"]);

    let predicates = [predicate(5, 0, 0)];
    let mut scratch = [ExprSlot::EMPTY; 2];
    let valid = PredicateTable::new(catalog, &predicates, &expressions, &mut scratch).unwrap();
    let twice = [predicate(1, 0, 0)];
    let mut scratch = [ExprSlot::EMPTY; 2];
    let doubled = PredicateTable::new(catalog, &twice, &expressions, &mut scratch).unwrap();
    let atom = |id: u32| {
        [Node::new(NodeKind::Proposition {
            proposition: PropositionId(id),
        })]
    };
    let bind = |nodes: &[Node], table: PredicateTable<'_>| {
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), nodes).unwrap();
        let mut slots = [ExprSlot::EMPTY; 8];
        BoundPredicateFormula::new(formula, catalog, table, &mut slots).map(|_| ())
    };
    let mut seen = Seen::default();
    seen.is("PropositionUnbound", err(bind(&atom(9), valid)));
    seen.is("PropositionDefinedTwice", err(bind(&atom(1), doubled)));
    let boolean = [SignalDeclaration::new(SignalId(2), "count", SignalDomain::Boolean)];
    let mut other_order = [0_u32; 1];
    let other = SignalCatalog::new(&boolean, &[], &mut other_order).unwrap();
    let nodes = atom(5);
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes).unwrap();
    let mut slots = [ExprSlot::EMPTY; 8];
    seen.is(
        "NonNumericSignal",
        err(BoundPredicateFormula::new(formula, other, valid, &mut slots).map(|_| ())),
    );
    // Typed with signal 2 at scale 0; bound where signal 2 has scale 1.
    let literal = [read(2), ExprNode::Integer { value: 1 }];
    let compare = [predicate(5, 0, 1)];
    let mut scratch = [ExprSlot::EMPTY; 2];
    let scaled_table = PredicateTable::new(catalog, &compare, &literal, &mut scratch).unwrap();
    let scaled = [SignalDeclaration::new(
        SignalId(2),
        "count",
        SignalDomain::FixedDecimal(FixedDecimalSignalDomain::new(0, 9, 1).unwrap()),
    )];
    let mut scaled_order = [0_u32; 1];
    let scaled_catalog = SignalCatalog::new(&scaled, &[], &mut scaled_order).unwrap();
    let mut slots = [ExprSlot::EMPTY; 8];
    seen.is(
        "IllTyped",
        err(BoundPredicateFormula::new(formula, scaled_catalog, scaled_table, &mut slots)
            .map(|_| ())),
    );
    seen.complete(PREDICATE_BINDING, &[]);
}

// Trace: TC-166, TC-1515, NFR-006-AC-4, FR-306-AC-2
#[test]
fn set_aggregation_errors() {
    use crate::{
        expand_quantifier_into, Aggregation, AggregationTemplate, ComparisonOperator, ExprId,
        ExprNode, MemberSet, OwnedSignalDeclaration, Predicate, Quantifier,
        SetAggregationDocument, SignalCatalogDocument, TemplateNode,
    };
    let catalog_document = SignalCatalogDocument::new(
        vec![
            OwnedSignalDeclaration::new(SignalId(1), "b".into(), SignalDomain::Boolean),
            OwnedSignalDeclaration::new(
                SignalId(2),
                "i".into(),
                SignalDomain::Integer(IntegerSignalDomain::new(0, 9).unwrap()),
            ),
        ],
        vec![PropositionBinding::new(PropositionId(1), SignalId(1))],
    )
    .unwrap();
    let catalog = catalog_document.validate().unwrap();
    let set = |name: &str, members: &[u32]| MemberSet {
        name: name.into(),
        members: members.iter().copied().map(SignalId).collect(),
    };
    let slot = |depth: u8| AggregationTemplate {
        nodes: vec![TemplateNode::SlotProposition { depth }],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: None,
    };
    let quantified = |set: &str, quantifier, template| Aggregation {
        set: set.into(),
        quantifier,
        template,
    };
    let numeric = |left: ExprNode| AggregationTemplate {
        nodes: vec![TemplateNode::Node {
            node: Node::new(NodeKind::Proposition {
                proposition: PropositionId(50),
            }),
        }],
        root: NodeId(0),
        expressions: vec![left, ExprNode::Integer { value: 1 }],
        predicates: vec![Predicate {
            proposition: PropositionId(50),
            operator: ComparisonOperator::Lt,
            left: ExprId(0),
            right: ExprId(1),
        }],
        inner: None,
    };
    let nest = |inner: Aggregation, leaves: usize| AggregationTemplate {
        nodes: vec![TemplateNode::Inner; leaves],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: Some(alloc::boxed::Box::new(inner)),
    };
    let new = |sets: Vec<MemberSet>, first: u32, aggregations: Vec<Aggregation>| {
        SetAggregationDocument::new(catalog, sets, PropositionId(first), aggregations).map(|_| ())
    };
    let each = Quantifier::ForEach;
    let mut seen = Seen::default();
    seen.is("EmptySetName", err(new(vec![set("", &[])], 100, vec![])));
    let long = "x".repeat(MAX_SIGNAL_NAME_BYTES + 1);
    seen.is("SetNameTooLong", err(new(vec![set(&long, &[])], 100, vec![])));
    seen.is(
        "DuplicateSetName",
        err(new(vec![set("a", &[]), set("a", &[])], 100, vec![])),
    );
    seen.is("UndeclaredMember", err(new(vec![set("a", &[9])], 100, vec![])));
    seen.is("DuplicateMember", err(new(vec![set("a", &[1, 1])], 100, vec![])));
    seen.is(
        "UnknownSet",
        err(new(vec![], 100, vec![quantified("a", each, slot(0))])),
    );
    seen.is(
        "QuantifierOutOfRange",
        err(new(
            vec![set("a", &[1])],
            100,
            vec![quantified("a", Quantifier::AtLeast { n: 1_025 }, slot(0))],
        )),
    );
    let innermost = quantified("a", each, slot(0));
    let middle = quantified("a", each, nest(innermost, 1));
    let deep = quantified("a", each, nest(middle, 1));
    seen.is("AggregationTooDeep", err(new(vec![set("a", &[1])], 100, vec![deep])));
    seen.is(
        "InnerLeafMismatch",
        err(new(
            vec![set("a", &[1])],
            100,
            vec![quantified("a", each, nest(quantified("a", each, slot(1)), 2))],
        )),
    );
    let mut forward = slot(0);
    forward.nodes = vec![TemplateNode::Node {
        node: Node::new(NodeKind::Not { operand: NodeId(0) }),
    }];
    seen.is(
        "TemplateStructure",
        err(new(vec![set("a", &[1])], 100, vec![quantified("a", each, forward)])),
    );
    seen.is(
        "SlotOutsideTemplate",
        err(new(vec![set("a", &[1])], 100, vec![quantified("a", each, slot(1))])),
    );
    seen.is(
        "TemplatePredicate",
        err(new(
            vec![set("a", &[2])],
            100,
            vec![quantified(
                "a",
                each,
                numeric(ExprNode::Signal {
                    signal: SignalId(1),
                }),
            )],
        )),
    );
    seen.is(
        "TemplateIllTypedForMember",
        err(new(
            vec![set("a", &[1])],
            100,
            vec![quantified("a", each, numeric(ExprNode::SlotSignal { depth: 0 }))],
        )),
    );
    seen.is(
        "MemberHasNoBooleanBinding",
        err(new(vec![set("a", &[2])], 100, vec![quantified("a", each, slot(0))])),
    );
    seen.is(
        "FreshIdentityCollision",
        err(new(
            vec![set("a", &[2])],
            1,
            vec![quantified("a", each, numeric(ExprNode::SlotSignal { depth: 0 }))],
        )),
    );
    seen.is(
        "AggregationTooLarge",
        err(new(
            vec![set("a", &[1])],
            100,
            vec![quantified("a", Quantifier::AtLeast { n: 1 }, slot(0)); 50_001],
        )),
    );
    let shared_predicate = AggregationTemplate {
        nodes: vec![TemplateNode::Node {
            node: Node::new(NodeKind::Proposition {
                proposition: PropositionId(1),
            }),
        }],
        root: NodeId(0),
        expressions: vec![ExprNode::Integer { value: 1 }],
        predicates: vec![Predicate {
            proposition: PropositionId(1),
            operator: ComparisonOperator::Eq,
            left: ExprId(0),
            right: ExprId(0),
        }],
        inner: None,
    };
    seen.is(
        "PredicateDefinedTwice",
        err(new(vec![set("a", &[2])], 100, vec![quantified("a", each, shared_predicate)])),
    );
    let mut stray = slot(0);
    stray.nodes = vec![TemplateNode::Node {
        node: Node::new(NodeKind::Proposition {
            proposition: PropositionId(50),
        }),
    }];
    seen.is(
        "PlaceholderOutOfScope",
        err(new(
            vec![set("a", &[2])],
            100,
            vec![
                quantified("a", each, numeric(ExprNode::SlotSignal { depth: 0 })),
                quantified("a", each, stray),
            ],
        )),
    );
    seen.complete(AGGREGATION, &["AggregationTooLarge"]);

    let mut seen = Seen::default();
    let roots = [NodeId(0); 2];
    let mut out = [Node::new(NodeKind::True); 1];
    seen.is(
        "ExpansionTooSmall",
        err(expand_quantifier_into(Quantifier::AtLeast { n: 1 }, &roots, NodeId(2), &mut out)),
    );
    let many = vec![NodeId(0); crate::MAX_SET_MEMBERS + 1];
    seen.is(
        "AggregationTooLarge",
        err(expand_quantifier_into(Quantifier::ForEach, &many, NodeId(0), &mut out)),
    );
    // Validated against one catalog, expanded against one without the binding.
    let document = SetAggregationDocument::new(
        catalog,
        vec![set("a", &[1])],
        PropositionId(100),
        vec![quantified("a", each, slot(0))],
    )
    .unwrap();
    let unbound = SignalCatalogDocument::new(
        vec![OwnedSignalDeclaration::new(SignalId(1), "b".into(), SignalDomain::Boolean)],
        vec![],
    )
    .unwrap();
    seen.is("Invalid", err(document.expand(unbound.validate().unwrap())));
    seen.complete(REFUSAL, &[]);
}

// Trace: TC-166, TC-1523, TC-1527, NFR-006-AC-4, FR-309-AC-2, FR-310-AC-3
#[test]
fn contract_errors() {
    use crate::{
        check_assumption_inputs, lower_contract_obligation, Contract, ContractGraph,
        ContractSetDocument, InfiniteClock, OwnedSignalDeclaration, SignalCatalogDocument,
        SignalDirection, MAX_CONTRACTS,
    };
    let nodes = vec![
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(1),
        }),
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(2),
        }),
    ];
    let formula = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(1), nodes.clone())
        .unwrap();
    let graph = ContractGraph::Bounded(&formula);
    let identity = graph.content_identity().unwrap();
    let contract = |id: u32, name: &str, assumption: u32| Contract {
        id,
        name: name.into(),
        assumption: NodeId(assumption),
        guarantee: NodeId(1),
    };
    let new = |contracts: Vec<Contract>| {
        ContractSetDocument::new(graph, identity.clone(), contracts).map(|_| ())
    };
    let mut seen = Seen::default();
    seen.is("RootOutOfRange", err(new(vec![contract(1, "a", 9)])));
    seen.is(
        "ContractIdNotIncreasing",
        err(new(vec![contract(1, "a", 0), contract(1, "b", 0)])),
    );
    seen.is("EmptyName", err(new(vec![contract(1, "", 0)])));
    let long = "x".repeat(MAX_SIGNAL_NAME_BYTES + 1);
    seen.is("NameTooLong", err(new(vec![contract(1, &long, 0)])));
    seen.is(
        "DuplicateName",
        err(new(vec![contract(1, "a", 0), contract(2, "a", 0)])),
    );
    #[cfg(not(miri))]
    {
        let many = (0..=MAX_CONTRACTS as u32)
            .map(|id| contract(id, &id.to_string(), 0))
            .collect();
        seen.is("ContractLimitExceeded", err(new(many)));
    }
    seen.is(
        "GraphIdentityMismatch",
        err(ContractSetDocument::new(graph, "0".repeat(64), vec![]).map(|_| ())),
    );
    seen.is(
        "ProfileMismatch",
        err(ContractSetDocument::bind(
            graph,
            SemanticProfile::OnlinePrefixV1,
            None,
            identity.clone(),
            vec![],
        )
        .map(|_| ())),
    );
    seen.is(
        "ClockMismatch",
        err(ContractSetDocument::bind(
            graph,
            SemanticProfile::ClosedTraceV1,
            Some(InfiniteClock::EventPosition),
            identity.clone(),
            vec![],
        )
        .map(|_| ())),
    );
    seen.complete(CONTRACT_SET, &["ContractLimitExceeded"]);

    let mut seen = Seen::default();
    let mut grown = nodes.clone();
    seen.is(
        "RootOutOfRange",
        err(lower_contract_obligation(&mut grown, NodeId(5), NodeId(0))),
    );
    #[cfg(not(miri))]
    {
        let mut full = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES];
        seen.is(
            "NodeLimit",
            err(lower_contract_obligation(&mut full, NodeId(0), NodeId(0))),
        );
    }
    seen.complete(OBLIGATION, &["NodeLimit"]);

    let catalog_document = SignalCatalogDocument::new_v2(
        vec![
            OwnedSignalDeclaration::new(SignalId(1), "a".into(), SignalDomain::Boolean)
                .with_direction(SignalDirection::Output),
        ],
        vec![PropositionBinding::new(PropositionId(1), SignalId(1))],
    )
    .unwrap();
    let catalog = catalog_document.validate().unwrap();
    let view = formula.validate().unwrap();
    let mut seen = Seen::default();
    seen.is(
        "RootOutOfRange",
        err(check_assumption_inputs(catalog, view, NodeId(9))),
    );
    seen.is(
        "Binding",
        err(check_assumption_inputs(catalog, view, NodeId(1))),
    );
    seen.is(
        "NotInput",
        err(check_assumption_inputs(catalog, view, NodeId(0))),
    );
    seen.complete(ASSUMPTION, &[]);
}

// Trace: TC-166, TC-221, TC-222, NFR-006-AC-4, FR-332-AC-2, FR-332-AC-3
#[test]
fn ltlf_errors() {
    use crate::{select_ltlf_identity, validate_ltlf, LtlfFormulaDocument, FORMULA_LTLF_V1};
    let ltlf = Some("mltl.ltlf-finite-trace/v1");
    let mut seen = Seen::default();
    seen.is("MissingEdition", err(select_ltlf_identity(None, ltlf)));
    seen.is("UnknownEdition", err(select_ltlf_identity(Some("x"), ltlf)));
    seen.is(
        "MismatchedEdition",
        err(select_ltlf_identity(Some("tl-syntax.formula/v2"), ltlf)),
    );
    seen.is(
        "MissingProfile",
        err(select_ltlf_identity(Some(FORMULA_LTLF_V1), None)),
    );
    seen.is(
        "UnknownProfile",
        err(select_ltlf_identity(Some(FORMULA_LTLF_V1), Some("x"))),
    );
    seen.is(
        "MismatchedProfile",
        err(select_ltlf_identity(
            Some(FORMULA_LTLF_V1),
            Some("mltl.closed-trace/v1"),
        )),
    );
    seen.complete(LTLF_IDENTITY, &[]);

    let mut seen = Seen::default();
    seen.is(
        "Identity",
        err(LtlfFormulaDocument::new(
            SemanticProfile::ClosedTraceV1,
            NodeId(0),
            vec![Node::new(NodeKind::True)],
        )),
    );
    let past = [
        Node::new(NodeKind::True),
        Node::new(NodeKind::StrongPrevious { operand: NodeId(0) }),
    ];
    seen.is("Refused", err(validate_ltlf(NodeId(1), &past)));
    seen.is("Graph", err(validate_ltlf(NodeId(9), &past)));
    seen.complete(LTLF_FORMULA, &[]);
}

#[test]
fn the_completeness_check_accepts_a_full_set() {
    let mut full = Seen::default();
    full.0.insert("A");
    full.0.insert("B");
    full.complete(&["A", "B"], &[]);
}

#[test]
#[should_panic]
fn the_completeness_check_panics_when_a_variant_was_not_produced() {
    let mut seen = Seen::default();
    seen.0.insert("A");
    seen.complete(&["A", "B"], &[]);
}
