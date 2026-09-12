use core::fmt;

use crate::syntax::{
    Formula, Interval, Node, NodeId, NodeKind, SemanticProfile, SourceSpan,
    MAX_FORMULA_DOCUMENT_NODES,
};

/// Closed derived future-time operator-profile identity.
pub const FUTURE_OPERATORS_V1: &str = "tl-syntax.future-operators/v1";

/// Identity of the raw [`FutureLoweringRequest`] admission contract.
pub const FUTURE_LOWERING_REQUEST_V1: &str = "tl-syntax.future-lowering-request/v1";

/// Identity of the non-wire [`FutureLoweringReport`] contract.
pub const FUTURE_LOWERING_REPORT_V1: &str = "tl-syntax.future-lowering-report/v1";

/// Identity of the [`FutureLoweringRefusal`] contract.
pub const FUTURE_LOWERING_REFUSAL_V1: &str = "tl-syntax.future-lowering-refusal/v1";

/// Maximum byte length of a request, operator-profile, or semantic-profile identity.
pub const MAX_FUTURE_LOWERING_IDENTITY_BYTES: usize = 128;

/// Maximum byte length of a derived-kind spelling.
pub const MAX_FUTURE_LOWERING_KIND_BYTES: usize = 16;

/// Number of canonical nodes every admitted lowering appends.
pub const FUTURE_LOWERING_NODE_CHARGE: usize = 3;

/// Derived future-time operator admitted by `tl-syntax.future-operators/v1`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FutureKind {
    /// Bounded weak until, `p W[a,b] q`, lowered to `(p U[a,b] q) | G[a,b] p`.
    WeakUntil,
    /// Bounded strong release, `p M[a,b] q`, lowered to `(p R[a,b] q) & F[a,b] p`.
    StrongRelease,
}

impl FutureKind {
    /// Returns the decoded kind spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WeakUntil => "W",
            Self::StrongRelease => "M",
        }
    }
}

/// Recognized kind that `tl-syntax.future-operators/v1` refuses.
///
/// Next needs a closure-aware successor-existence profile; past-time operators
/// belong to a separately versioned past/history profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnsupportedFutureKind {
    /// Strong or weak next, `X`.
    Next,
    /// Strong or weak previous, `Y`.
    Previous,
    /// Past once, `O`.
    Once,
    /// Past historically, `H`.
    Historically,
    /// Past since, `S`.
    Since,
    /// Past triggered, `T`.
    Triggered,
}

impl UnsupportedFutureKind {
    const ALL: [Self; 6] = [
        Self::Next,
        Self::Previous,
        Self::Once,
        Self::Historically,
        Self::Since,
        Self::Triggered,
    ];

    /// Returns the decoded kind spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Next => "X",
            Self::Previous => "Y",
            Self::Once => "O",
            Self::Historically => "H",
            Self::Since => "S",
            Self::Triggered => "T",
        }
    }
}

/// A raw pair of `u64` endpoints whose range and order are not yet checked.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RawBounds {
    /// Raw start value.
    pub start: u64,
    /// Raw end value.
    pub end: u64,
}

impl RawBounds {
    /// Pairs two raw endpoints.
    pub const fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

/// The borrowed, unclassified `tl-syntax.future-lowering-request/v1` boundary.
///
/// Every field keeps invalid states representable. [`Self::lower`] classifies
/// them in the stable refusal precedence documented on
/// [`FutureLoweringRefusal`] and never allocates or mutates the formula.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FutureLoweringRequest<'a> {
    /// Request contract identity; must be [`FUTURE_LOWERING_REQUEST_V1`].
    pub request_identity: &'a [u8],
    /// Operator-profile identity; must be [`FUTURE_OPERATORS_V1`].
    pub operator_profile: &'a [u8],
    /// Decoded derived-kind spelling.
    pub kind: &'a [u8],
    /// Semantic-profile identity; must name the formula's profile.
    pub semantic_profile: &'a [u8],
    /// The caller's validated graph; its length is the base for generated identities.
    pub formula: Formula<'a>,
    /// Raw identity of the already-lowered left operand `p`.
    pub left: u64,
    /// Raw identity of the already-lowered right operand `q`.
    pub right: u64,
    /// Raw inclusive interval bounds `[a,b]`.
    pub interval: Option<RawBounds>,
    /// Raw half-open span of the derived operator token.
    pub operator_span: Option<RawBounds>,
    /// Raw half-open span of the full derived expression.
    pub expression_span: Option<RawBounds>,
}

impl FutureLoweringRequest<'_> {
    /// Admits this request and returns its exact canonical expansion.
    ///
    /// # Errors
    ///
    /// Returns the single highest-precedence [`FutureLoweringRefusal`]; no
    /// partial result exists.
    pub fn lower(self) -> Result<FutureLowering, FutureLoweringRefusal> {
        admit(self).map(AdmittedLowering::lower)
    }
}

/// One fixed-size canonical expansion and its report.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FutureLowering {
    nodes: [Node; FUTURE_LOWERING_NODE_CHARGE],
    node_ids: [NodeId; FUTURE_LOWERING_NODE_CHARGE],
    report: FutureLoweringReport,
}

impl FutureLowering {
    /// Returns the generated nodes in the order the caller must append them.
    pub const fn nodes(&self) -> &[Node; FUTURE_LOWERING_NODE_CHARGE] {
        &self.nodes
    }

    /// Returns the absolute identity each generated node takes when appended.
    pub const fn node_ids(&self) -> [NodeId; FUTURE_LOWERING_NODE_CHARGE] {
        self.node_ids
    }

    /// Returns the identity of the expansion root.
    pub const fn root(&self) -> NodeId {
        self.report.root
    }

    /// Returns the diagnostic lowering report.
    pub const fn report(&self) -> &FutureLoweringReport {
        &self.report
    }
}

/// The non-wire `tl-syntax.future-lowering-report/v1` attribution value.
///
/// It is diagnostic attribution, not formula semantic identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FutureLoweringReport {
    kind: FutureKind,
    semantic_profile: SemanticProfile,
    left: NodeId,
    right: NodeId,
    root: NodeId,
    first_generated: NodeId,
    operator_span: Option<SourceSpan>,
    expression_span: Option<SourceSpan>,
}

impl FutureLoweringReport {
    /// Returns [`FUTURE_LOWERING_REPORT_V1`].
    pub const fn identity(&self) -> &'static str {
        FUTURE_LOWERING_REPORT_V1
    }

    /// Returns the admitted request identity.
    pub const fn request_identity(&self) -> &'static str {
        FUTURE_LOWERING_REQUEST_V1
    }

    /// Returns the admitted operator-profile identity.
    pub const fn operator_profile(&self) -> &'static str {
        FUTURE_OPERATORS_V1
    }

    /// Returns the lowered derived kind.
    pub const fn kind(&self) -> FutureKind {
        self.kind
    }

    /// Returns the preserved semantic profile.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        self.semantic_profile
    }

    /// Returns the referenced left operand root.
    pub const fn left(&self) -> NodeId {
        self.left
    }

    /// Returns the referenced right operand root.
    pub const fn right(&self) -> NodeId {
        self.right
    }

    /// Returns the expansion root.
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// Returns the first generated identity.
    pub const fn first_generated(&self) -> NodeId {
        self.first_generated
    }

    /// Returns the number of generated nodes.
    pub const fn generated_count(&self) -> usize {
        FUTURE_LOWERING_NODE_CHARGE
    }

    /// Returns the supplied operator-token span.
    pub const fn operator_span(&self) -> Option<SourceSpan> {
        self.operator_span
    }

    /// Returns the supplied full-expression span.
    pub const fn expression_span(&self) -> Option<SourceSpan> {
        self.expression_span
    }
}

/// Admission axis a refusal identifies, in stable precedence order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FutureLoweringAxis {
    /// Request contract identity.
    RequestIdentity,
    /// Operator-profile identity.
    OperatorProfile,
    /// Derived kind.
    Kind,
    /// Semantic-profile identity.
    SemanticProfile,
    /// Agreement between the semantic profile and the borrowed formula.
    ProfileAgreement,
    /// Interval presence, range, and order.
    Interval,
    /// Operand identity conversion and membership.
    Operand,
    /// Span pairing, range, order, and containment.
    Span,
    /// Generated-node identity and document budget.
    NodeBudget,
}

/// Which operand a refusal names.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FutureLoweringOperand {
    /// The left operand `p`.
    Left,
    /// The right operand `q`.
    Right,
}

impl FutureLoweringOperand {
    /// Returns the stable operand name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Which span a refusal names.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FutureLoweringSpanRole {
    /// The derived operator token.
    Operator,
    /// The full derived expression.
    Expression,
}

impl FutureLoweringSpanRole {
    /// Returns the stable span-role name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::Expression => "expression",
        }
    }
}

/// One `tl-syntax.future-lowering-refusal/v1` value.
///
/// Admission checks, in this order: request identity; operator profile; kind
/// (over-limit, then recognized-unsupported, then unknown); semantic-profile
/// identity; its agreement with the formula; interval presence, `u32` range,
/// and order; left then right operand `NodeId` conversion, then left then right
/// membership; span pairing, then operator then expression endpoint range, then
/// operator then expression order, then containment; checked addition of the
/// node charge; conversion of every generated identity; and finally
/// [`MAX_FORMULA_DOCUMENT_NODES`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FutureLoweringRefusal {
    /// The request identity exceeds [`MAX_FUTURE_LOWERING_IDENTITY_BYTES`].
    RequestIdentityTooLong {
        /// Supplied byte length.
        len: usize,
    },
    /// The request identity is not [`FUTURE_LOWERING_REQUEST_V1`].
    UnknownRequestIdentity,
    /// The operator-profile identity exceeds [`MAX_FUTURE_LOWERING_IDENTITY_BYTES`].
    OperatorProfileTooLong {
        /// Supplied byte length.
        len: usize,
    },
    /// The operator-profile identity is not [`FUTURE_OPERATORS_V1`].
    UnknownOperatorProfile,
    /// The kind spelling exceeds [`MAX_FUTURE_LOWERING_KIND_BYTES`].
    KindTooLong {
        /// Supplied byte length.
        len: usize,
    },
    /// The kind is recognized but outside this profile.
    UnsupportedKind {
        /// Recognized unsupported kind.
        kind: UnsupportedFutureKind,
    },
    /// The kind is not recognized.
    UnknownKind,
    /// The semantic-profile identity exceeds [`MAX_FUTURE_LOWERING_IDENTITY_BYTES`].
    SemanticProfileTooLong {
        /// Supplied byte length.
        len: usize,
    },
    /// The semantic-profile identity names no existing profile.
    UnknownSemanticProfile,
    /// The semantic profile differs from the borrowed formula's profile.
    SemanticProfileMismatch {
        /// Profile named by the request.
        requested: SemanticProfile,
        /// Profile of the borrowed formula.
        formula: SemanticProfile,
    },
    /// No interval was supplied.
    MissingInterval,
    /// An interval bound exceeds `u32::MAX`.
    IntervalBoundOutOfRange {
        /// Raw start bound.
        start: u64,
        /// Raw end bound.
        end: u64,
    },
    /// The interval start exceeds its end.
    InvertedInterval {
        /// Rejected start bound.
        start: u32,
        /// Rejected end bound.
        end: u32,
    },
    /// An operand identity exceeds the `NodeId` range.
    OperandIdOutOfRange {
        /// Rejected operand.
        operand: FutureLoweringOperand,
        /// Raw identity.
        id: u64,
    },
    /// An operand identity is absent from the borrowed formula.
    OperandAbsent {
        /// Rejected operand.
        operand: FutureLoweringOperand,
        /// Rejected identity.
        id: NodeId,
        /// Length of the borrowed node table.
        node_count: usize,
    },
    /// Exactly one of the operator and expression spans was supplied.
    SpanHalfMissing {
        /// The absent span.
        missing: FutureLoweringSpanRole,
    },
    /// A span endpoint exceeds `u32::MAX`.
    SpanEndpointOutOfRange {
        /// Rejected span.
        role: FutureLoweringSpanRole,
        /// Raw start offset.
        start: u64,
        /// Raw end offset.
        end: u64,
    },
    /// A span start exceeds its end.
    InvertedSpan {
        /// Rejected span.
        role: FutureLoweringSpanRole,
        /// Rejected start offset.
        start: u32,
        /// Rejected end offset.
        end: u32,
    },
    /// The operator-token span is not contained in the expression span.
    OperatorSpanOutsideExpression {
        /// Checked operator-token span.
        operator: SourceSpan,
        /// Checked expression span.
        expression: SourceSpan,
    },
    /// Adding the node charge to the formula length overflows `usize`.
    NodeCountOverflow {
        /// Length of the borrowed node table.
        node_count: usize,
    },
    /// A generated identity exceeds the `NodeId` range.
    GeneratedIdOutOfRange {
        /// Length of the borrowed node table.
        node_count: usize,
    },
    /// The lowered table would exceed [`MAX_FORMULA_DOCUMENT_NODES`].
    DocumentNodeLimitExceeded {
        /// Node count after appending the expansion.
        node_count: usize,
        /// Formula-v1 document node limit.
        limit: usize,
    },
}

impl FutureLoweringRefusal {
    /// Returns [`FUTURE_LOWERING_REFUSAL_V1`].
    pub const fn identity(&self) -> &'static str {
        FUTURE_LOWERING_REFUSAL_V1
    }

    /// Returns the admission axis this refusal identifies.
    pub const fn axis(&self) -> FutureLoweringAxis {
        match self {
            Self::RequestIdentityTooLong { .. } | Self::UnknownRequestIdentity => {
                FutureLoweringAxis::RequestIdentity
            }
            Self::OperatorProfileTooLong { .. } | Self::UnknownOperatorProfile => {
                FutureLoweringAxis::OperatorProfile
            }
            Self::KindTooLong { .. } | Self::UnsupportedKind { .. } | Self::UnknownKind => {
                FutureLoweringAxis::Kind
            }
            Self::SemanticProfileTooLong { .. } | Self::UnknownSemanticProfile => {
                FutureLoweringAxis::SemanticProfile
            }
            Self::SemanticProfileMismatch { .. } => FutureLoweringAxis::ProfileAgreement,
            Self::MissingInterval
            | Self::IntervalBoundOutOfRange { .. }
            | Self::InvertedInterval { .. } => FutureLoweringAxis::Interval,
            Self::OperandIdOutOfRange { .. } | Self::OperandAbsent { .. } => {
                FutureLoweringAxis::Operand
            }
            Self::SpanHalfMissing { .. }
            | Self::SpanEndpointOutOfRange { .. }
            | Self::InvertedSpan { .. }
            | Self::OperatorSpanOutsideExpression { .. } => FutureLoweringAxis::Span,
            Self::NodeCountOverflow { .. }
            | Self::GeneratedIdOutOfRange { .. }
            | Self::DocumentNodeLimitExceeded { .. } => FutureLoweringAxis::NodeBudget,
        }
    }

    /// Returns the stable refusal code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::RequestIdentityTooLong { .. } => "request_identity_too_long",
            Self::UnknownRequestIdentity => "unknown_request_identity",
            Self::OperatorProfileTooLong { .. } => "operator_profile_too_long",
            Self::UnknownOperatorProfile => "unknown_operator_profile",
            Self::KindTooLong { .. } => "kind_too_long",
            Self::UnsupportedKind { .. } => "unsupported_kind",
            Self::UnknownKind => "unknown_kind",
            Self::SemanticProfileTooLong { .. } => "semantic_profile_too_long",
            Self::UnknownSemanticProfile => "unknown_semantic_profile",
            Self::SemanticProfileMismatch { .. } => "semantic_profile_mismatch",
            Self::MissingInterval => "missing_interval",
            Self::IntervalBoundOutOfRange { .. } => "interval_bound_out_of_range",
            Self::InvertedInterval { .. } => "inverted_interval",
            Self::OperandIdOutOfRange { .. } => "operand_id_out_of_range",
            Self::OperandAbsent { .. } => "operand_absent",
            Self::SpanHalfMissing { .. } => "span_half_missing",
            Self::SpanEndpointOutOfRange { .. } => "span_endpoint_out_of_range",
            Self::InvertedSpan { .. } => "inverted_span",
            Self::OperatorSpanOutsideExpression { .. } => "operator_span_outside_expression",
            Self::NodeCountOverflow { .. } => "node_count_overflow",
            Self::GeneratedIdOutOfRange { .. } => "generated_id_out_of_range",
            Self::DocumentNodeLimitExceeded { .. } => "document_node_limit_exceeded",
        }
    }
}

impl fmt::Display for FutureLoweringRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", FUTURE_LOWERING_REFUSAL_V1, self.code())?;
        match self {
            Self::RequestIdentityTooLong { len }
            | Self::OperatorProfileTooLong { len }
            | Self::SemanticProfileTooLong { len } => write!(
                formatter,
                " ({len} bytes exceeds {MAX_FUTURE_LOWERING_IDENTITY_BYTES})"
            ),
            Self::KindTooLong { len } => write!(
                formatter,
                " ({len} bytes exceeds {MAX_FUTURE_LOWERING_KIND_BYTES})"
            ),
            Self::UnsupportedKind { kind } => write!(formatter, " ({})", kind.as_str()),
            Self::SemanticProfileMismatch { requested, formula } => write!(
                formatter,
                " (requested {}, formula {})",
                requested.as_str(),
                formula.as_str()
            ),
            Self::IntervalBoundOutOfRange { start, end } => {
                write!(formatter, " ([{start},{end}])")
            }
            Self::InvertedInterval { start, end } => write!(formatter, " ([{start},{end}])"),
            Self::OperandIdOutOfRange { operand, id } => {
                write!(formatter, " ({} {id})", operand.as_str())
            }
            Self::OperandAbsent {
                operand,
                id,
                node_count,
            } => write!(
                formatter,
                " ({} {} of {node_count} nodes)",
                operand.as_str(),
                id.0
            ),
            Self::SpanHalfMissing { missing } => {
                write!(formatter, " ({} absent)", missing.as_str())
            }
            Self::SpanEndpointOutOfRange { role, start, end } => {
                write!(formatter, " ({} [{start},{end}))", role.as_str())
            }
            Self::InvertedSpan { role, start, end } => {
                write!(formatter, " ({} [{start},{end}))", role.as_str())
            }
            Self::OperatorSpanOutsideExpression {
                operator,
                expression,
            } => write!(
                formatter,
                " ([{},{}) outside [{},{}))",
                operator.start(),
                operator.end(),
                expression.start(),
                expression.end()
            ),
            Self::NodeCountOverflow { node_count } | Self::GeneratedIdOutOfRange { node_count } => {
                write!(formatter, " ({node_count} nodes)")
            }
            Self::DocumentNodeLimitExceeded { node_count, limit } => {
                write!(formatter, " ({node_count} exceeds {limit})")
            }
            Self::UnknownRequestIdentity
            | Self::UnknownOperatorProfile
            | Self::UnknownKind
            | Self::UnknownSemanticProfile
            | Self::MissingInterval => Ok(()),
        }
    }
}

/// The private typed request: the only input the total lowerer accepts.
struct AdmittedLowering {
    kind: FutureKind,
    semantic_profile: SemanticProfile,
    left: NodeId,
    right: NodeId,
    interval: Interval,
    spans: Option<SpanPair>,
    node_ids: [NodeId; FUTURE_LOWERING_NODE_CHARGE],
}

#[derive(Clone, Copy)]
struct SpanPair {
    operator: SourceSpan,
    expression: SourceSpan,
}

impl AdmittedLowering {
    fn lower(self) -> FutureLowering {
        let [first, second, root] = self.node_ids;
        let Self {
            interval,
            left,
            right,
            ..
        } = self;
        let kinds = match self.kind {
            FutureKind::WeakUntil => [
                NodeKind::Until {
                    interval,
                    left,
                    right,
                },
                NodeKind::Globally {
                    interval,
                    operand: left,
                },
                NodeKind::Or {
                    left: first,
                    right: second,
                },
            ],
            FutureKind::StrongRelease => [
                NodeKind::Release {
                    interval,
                    left,
                    right,
                },
                NodeKind::Future {
                    interval,
                    operand: left,
                },
                NodeKind::And {
                    left: first,
                    right: second,
                },
            ],
        };
        let span = self.spans.map(|spans| spans.expression);
        FutureLowering {
            nodes: kinds.map(|kind| Node { kind, span }),
            node_ids: self.node_ids,
            report: FutureLoweringReport {
                kind: self.kind,
                semantic_profile: self.semantic_profile,
                left,
                right,
                root,
                first_generated: first,
                operator_span: self.spans.map(|spans| spans.operator),
                expression_span: span,
            },
        }
    }
}

fn admit(request: FutureLoweringRequest<'_>) -> Result<AdmittedLowering, FutureLoweringRefusal> {
    use FutureLoweringRefusal as Refusal;

    admit_identity(
        request.request_identity,
        FUTURE_LOWERING_REQUEST_V1,
        |len| Refusal::RequestIdentityTooLong { len },
        Refusal::UnknownRequestIdentity,
    )?;
    admit_identity(
        request.operator_profile,
        FUTURE_OPERATORS_V1,
        |len| Refusal::OperatorProfileTooLong { len },
        Refusal::UnknownOperatorProfile,
    )?;
    let kind = admit_kind(request.kind)?;
    let semantic_profile = admit_semantic_profile(request.semantic_profile)?;
    let formula = request.formula;
    if semantic_profile != formula.profile() {
        return Err(Refusal::SemanticProfileMismatch {
            requested: semantic_profile,
            formula: formula.profile(),
        });
    }
    let interval = admit_interval(request.interval)?;
    let left = admit_operand_id(FutureLoweringOperand::Left, request.left)?;
    let right = admit_operand_id(FutureLoweringOperand::Right, request.right)?;
    admit_membership(formula, FutureLoweringOperand::Left, left)?;
    admit_membership(formula, FutureLoweringOperand::Right, right)?;
    let spans = admit_spans(request.operator_span, request.expression_span)?;
    let node_ids = preflight_node_ids(formula.nodes().len())?;
    Ok(AdmittedLowering {
        kind,
        semantic_profile,
        left,
        right,
        interval,
        spans,
        node_ids,
    })
}

fn admit_identity(
    supplied: &[u8],
    expected: &str,
    too_long: impl FnOnce(usize) -> FutureLoweringRefusal,
    unknown: FutureLoweringRefusal,
) -> Result<(), FutureLoweringRefusal> {
    if supplied.len() > MAX_FUTURE_LOWERING_IDENTITY_BYTES {
        Err(too_long(supplied.len()))
    } else if supplied == expected.as_bytes() {
        Ok(())
    } else {
        Err(unknown)
    }
}

fn admit_kind(supplied: &[u8]) -> Result<FutureKind, FutureLoweringRefusal> {
    if supplied.len() > MAX_FUTURE_LOWERING_KIND_BYTES {
        return Err(FutureLoweringRefusal::KindTooLong {
            len: supplied.len(),
        });
    }
    for kind in [FutureKind::WeakUntil, FutureKind::StrongRelease] {
        if supplied == kind.as_str().as_bytes() {
            return Ok(kind);
        }
    }
    match UnsupportedFutureKind::ALL
        .into_iter()
        .find(|kind| supplied == kind.as_str().as_bytes())
    {
        Some(kind) => Err(FutureLoweringRefusal::UnsupportedKind { kind }),
        None => Err(FutureLoweringRefusal::UnknownKind),
    }
}

fn admit_semantic_profile(supplied: &[u8]) -> Result<SemanticProfile, FutureLoweringRefusal> {
    if supplied.len() > MAX_FUTURE_LOWERING_IDENTITY_BYTES {
        return Err(FutureLoweringRefusal::SemanticProfileTooLong {
            len: supplied.len(),
        });
    }
    SemanticProfile::ALL
        .into_iter()
        .find(|profile| supplied == profile.as_str().as_bytes())
        .ok_or(FutureLoweringRefusal::UnknownSemanticProfile)
}

fn admit_interval(supplied: Option<RawBounds>) -> Result<Interval, FutureLoweringRefusal> {
    let bounds = supplied.ok_or(FutureLoweringRefusal::MissingInterval)?;
    let (Ok(start), Ok(end)) = (u32::try_from(bounds.start), u32::try_from(bounds.end)) else {
        return Err(FutureLoweringRefusal::IntervalBoundOutOfRange {
            start: bounds.start,
            end: bounds.end,
        });
    };
    Interval::new(start, end).map_err(|error| FutureLoweringRefusal::InvertedInterval {
        start: error.start,
        end: error.end,
    })
}

fn admit_operand_id(
    operand: FutureLoweringOperand,
    id: u64,
) -> Result<NodeId, FutureLoweringRefusal> {
    u32::try_from(id)
        .map(NodeId)
        .map_err(|_| FutureLoweringRefusal::OperandIdOutOfRange { operand, id })
}

fn admit_membership(
    formula: Formula<'_>,
    operand: FutureLoweringOperand,
    id: NodeId,
) -> Result<(), FutureLoweringRefusal> {
    match formula.node(id) {
        Some(_) => Ok(()),
        None => Err(FutureLoweringRefusal::OperandAbsent {
            operand,
            id,
            node_count: formula.nodes().len(),
        }),
    }
}

fn admit_spans(
    operator: Option<RawBounds>,
    expression: Option<RawBounds>,
) -> Result<Option<SpanPair>, FutureLoweringRefusal> {
    let (operator, expression) = match (operator, expression) {
        (None, None) => return Ok(None),
        (Some(_), None) => {
            return Err(FutureLoweringRefusal::SpanHalfMissing {
                missing: FutureLoweringSpanRole::Expression,
            });
        }
        (None, Some(_)) => {
            return Err(FutureLoweringRefusal::SpanHalfMissing {
                missing: FutureLoweringSpanRole::Operator,
            });
        }
        (Some(operator), Some(expression)) => (operator, expression),
    };
    let operator_offsets = span_offsets(FutureLoweringSpanRole::Operator, operator)?;
    let expression_offsets = span_offsets(FutureLoweringSpanRole::Expression, expression)?;
    let operator = checked_span(FutureLoweringSpanRole::Operator, operator_offsets)?;
    let expression = checked_span(FutureLoweringSpanRole::Expression, expression_offsets)?;
    if expression.start() <= operator.start() && operator.end() <= expression.end() {
        Ok(Some(SpanPair {
            operator,
            expression,
        }))
    } else {
        Err(FutureLoweringRefusal::OperatorSpanOutsideExpression {
            operator,
            expression,
        })
    }
}

fn span_offsets(
    role: FutureLoweringSpanRole,
    bounds: RawBounds,
) -> Result<(u32, u32), FutureLoweringRefusal> {
    match (u32::try_from(bounds.start), u32::try_from(bounds.end)) {
        (Ok(start), Ok(end)) => Ok((start, end)),
        _ => Err(FutureLoweringRefusal::SpanEndpointOutOfRange {
            role,
            start: bounds.start,
            end: bounds.end,
        }),
    }
}

fn checked_span(
    role: FutureLoweringSpanRole,
    (start, end): (u32, u32),
) -> Result<SourceSpan, FutureLoweringRefusal> {
    SourceSpan::new(start, end).map_err(|error| FutureLoweringRefusal::InvertedSpan {
        role,
        start: error.start,
        end: error.end,
    })
}

/// Preflights the node charge against a node-table length.
///
/// The length is only ever `Formula::nodes().len()`; it is a parameter so every
/// branch stays testable. The overflow branch is unreachable with a real table,
/// and the identity-range branch needs a table longer than `u32::MAX - 2` nodes.
fn preflight_node_ids(
    node_count: usize,
) -> Result<[NodeId; FUTURE_LOWERING_NODE_CHARGE], FutureLoweringRefusal> {
    let Some(lowered_count) = node_count.checked_add(FUTURE_LOWERING_NODE_CHARGE) else {
        return Err(FutureLoweringRefusal::NodeCountOverflow { node_count });
    };
    let mut node_ids = [NodeId(0); FUTURE_LOWERING_NODE_CHARGE];
    for (offset, node_id) in node_ids.iter_mut().enumerate() {
        *node_id = u32::try_from(node_count + offset)
            .map(NodeId)
            .map_err(|_| FutureLoweringRefusal::GeneratedIdOutOfRange { node_count })?;
    }
    if lowered_count > MAX_FORMULA_DOCUMENT_NODES {
        return Err(FutureLoweringRefusal::DocumentNodeLimitExceeded {
            node_count: lowered_count,
            limit: MAX_FORMULA_DOCUMENT_NODES,
        });
    }
    Ok(node_ids)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::ToString;

    use super::*;

    /// Position in the FR-008 precedence. The match has no wildcard, so a new
    /// variant fails to compile until it is placed.
    fn precedence(refusal: FutureLoweringRefusal) -> u8 {
        use FutureLoweringOperand::{Left, Right};
        use FutureLoweringRefusal as Refusal;
        use FutureLoweringSpanRole::{Expression, Operator};

        match refusal {
            Refusal::RequestIdentityTooLong { .. } => 0,
            Refusal::UnknownRequestIdentity => 1,
            Refusal::OperatorProfileTooLong { .. } => 2,
            Refusal::UnknownOperatorProfile => 3,
            Refusal::KindTooLong { .. } => 4,
            Refusal::UnsupportedKind { .. } => 5,
            Refusal::UnknownKind => 6,
            Refusal::SemanticProfileTooLong { .. } => 7,
            Refusal::UnknownSemanticProfile => 8,
            Refusal::SemanticProfileMismatch { .. } => 9,
            Refusal::MissingInterval => 10,
            Refusal::IntervalBoundOutOfRange { .. } => 11,
            Refusal::InvertedInterval { .. } => 12,
            Refusal::OperandIdOutOfRange { operand: Left, .. } => 13,
            Refusal::OperandIdOutOfRange { operand: Right, .. } => 14,
            Refusal::OperandAbsent { operand: Left, .. } => 15,
            Refusal::OperandAbsent { operand: Right, .. } => 16,
            Refusal::SpanHalfMissing { .. } => 17,
            Refusal::SpanEndpointOutOfRange { role: Operator, .. } => 18,
            Refusal::SpanEndpointOutOfRange {
                role: Expression, ..
            } => 19,
            Refusal::InvertedSpan { role: Operator, .. } => 20,
            Refusal::InvertedSpan {
                role: Expression, ..
            } => 21,
            Refusal::OperatorSpanOutsideExpression { .. } => 22,
            Refusal::NodeCountOverflow { .. } => 23,
            Refusal::GeneratedIdOutOfRange { .. } => 24,
            Refusal::DocumentNodeLimitExceeded { .. } => 25,
        }
    }

    // Trace: TC-046, FR-008-AC-1, FR-008-AC-3
    #[test]
    fn every_refusal_has_its_code_axis_and_display() {
        use FutureLoweringAxis as Axis;
        use FutureLoweringOperand::{Left, Right};
        use FutureLoweringRefusal as Refusal;
        use FutureLoweringSpanRole::{Expression, Operator};

        let span = |start, end| SourceSpan::new(start, end).unwrap();
        let wide = u64::from(u32::MAX) + 1;
        let table = [
            (
                Refusal::RequestIdentityTooLong { len: 129 },
                Axis::RequestIdentity,
                "request_identity_too_long",
                " (129 bytes exceeds 128)",
            ),
            (
                Refusal::UnknownRequestIdentity,
                Axis::RequestIdentity,
                "unknown_request_identity",
                "",
            ),
            (
                Refusal::OperatorProfileTooLong { len: 130 },
                Axis::OperatorProfile,
                "operator_profile_too_long",
                " (130 bytes exceeds 128)",
            ),
            (
                Refusal::UnknownOperatorProfile,
                Axis::OperatorProfile,
                "unknown_operator_profile",
                "",
            ),
            (
                Refusal::KindTooLong { len: 17 },
                Axis::Kind,
                "kind_too_long",
                " (17 bytes exceeds 16)",
            ),
            (
                Refusal::UnsupportedKind {
                    kind: UnsupportedFutureKind::Next,
                },
                Axis::Kind,
                "unsupported_kind",
                " (X)",
            ),
            (Refusal::UnknownKind, Axis::Kind, "unknown_kind", ""),
            (
                Refusal::SemanticProfileTooLong { len: 131 },
                Axis::SemanticProfile,
                "semantic_profile_too_long",
                " (131 bytes exceeds 128)",
            ),
            (
                Refusal::UnknownSemanticProfile,
                Axis::SemanticProfile,
                "unknown_semantic_profile",
                "",
            ),
            (
                Refusal::SemanticProfileMismatch {
                    requested: SemanticProfile::OnlinePrefixV1,
                    formula: SemanticProfile::ClosedTraceV1,
                },
                Axis::ProfileAgreement,
                "semantic_profile_mismatch",
                " (requested mltl.online-prefix/v1, formula mltl.closed-trace/v1)",
            ),
            (
                Refusal::MissingInterval,
                Axis::Interval,
                "missing_interval",
                "",
            ),
            (
                Refusal::IntervalBoundOutOfRange {
                    start: 1,
                    end: wide,
                },
                Axis::Interval,
                "interval_bound_out_of_range",
                " ([1,4294967296])",
            ),
            (
                Refusal::InvertedInterval { start: 5, end: 2 },
                Axis::Interval,
                "inverted_interval",
                " ([5,2])",
            ),
            (
                Refusal::OperandIdOutOfRange {
                    operand: Left,
                    id: wide,
                },
                Axis::Operand,
                "operand_id_out_of_range",
                " (left 4294967296)",
            ),
            (
                Refusal::OperandIdOutOfRange {
                    operand: Right,
                    id: u64::MAX,
                },
                Axis::Operand,
                "operand_id_out_of_range",
                " (right 18446744073709551615)",
            ),
            (
                Refusal::OperandAbsent {
                    operand: Left,
                    id: NodeId(3),
                    node_count: 3,
                },
                Axis::Operand,
                "operand_absent",
                " (left 3 of 3 nodes)",
            ),
            (
                Refusal::OperandAbsent {
                    operand: Right,
                    id: NodeId(7),
                    node_count: 3,
                },
                Axis::Operand,
                "operand_absent",
                " (right 7 of 3 nodes)",
            ),
            (
                Refusal::SpanHalfMissing { missing: Operator },
                Axis::Span,
                "span_half_missing",
                " (operator absent)",
            ),
            (
                Refusal::SpanHalfMissing {
                    missing: Expression,
                },
                Axis::Span,
                "span_half_missing",
                " (expression absent)",
            ),
            (
                Refusal::SpanEndpointOutOfRange {
                    role: Operator,
                    start: wide,
                    end: 0,
                },
                Axis::Span,
                "span_endpoint_out_of_range",
                " (operator [4294967296,0))",
            ),
            (
                Refusal::SpanEndpointOutOfRange {
                    role: Expression,
                    start: 0,
                    end: wide,
                },
                Axis::Span,
                "span_endpoint_out_of_range",
                " (expression [0,4294967296))",
            ),
            (
                Refusal::InvertedSpan {
                    role: Operator,
                    start: 9,
                    end: 8,
                },
                Axis::Span,
                "inverted_span",
                " (operator [9,8))",
            ),
            (
                Refusal::InvertedSpan {
                    role: Expression,
                    start: 4,
                    end: 1,
                },
                Axis::Span,
                "inverted_span",
                " (expression [4,1))",
            ),
            (
                Refusal::OperatorSpanOutsideExpression {
                    operator: span(99, 150),
                    expression: span(100, 200),
                },
                Axis::Span,
                "operator_span_outside_expression",
                " ([99,150) outside [100,200))",
            ),
            (
                Refusal::NodeCountOverflow { node_count: 7 },
                Axis::NodeBudget,
                "node_count_overflow",
                " (7 nodes)",
            ),
            (
                Refusal::GeneratedIdOutOfRange {
                    node_count: 4_294_967_294,
                },
                Axis::NodeBudget,
                "generated_id_out_of_range",
                " (4294967294 nodes)",
            ),
            (
                Refusal::DocumentNodeLimitExceeded {
                    node_count: 100_001,
                    limit: MAX_FORMULA_DOCUMENT_NODES,
                },
                Axis::NodeBudget,
                "document_node_limit_exceeded",
                " (100001 exceeds 100000)",
            ),
        ];

        let mut placed = [false; 26];
        let mut previous = (0, Axis::RequestIdentity);
        for (refusal, axis, code, detail) in table {
            let slot = precedence(refusal);
            placed[usize::from(slot)] = true;
            assert!(slot >= previous.0, "{code} is out of precedence order");
            assert!(axis >= previous.1, "{code} lowers the axis");
            previous = (slot, axis);
            assert_eq!(refusal.axis(), axis, "{code}");
            assert_eq!(refusal.code(), code);
            assert_eq!(refusal.identity(), FUTURE_LOWERING_REFUSAL_V1);
            assert_eq!(
                refusal.to_string(),
                std::format!("{FUTURE_LOWERING_REFUSAL_V1}: {code}{detail}")
            );
        }
        assert_eq!(placed, [true; 26], "every precedence slot has a row");
    }

    // Trace: TC-046, FR-008-AC-1, FR-008-AC-3
    #[test]
    fn node_budget_refusals_follow_their_precedence() {
        assert_eq!(
            preflight_node_ids(usize::MAX - 2),
            Err(FutureLoweringRefusal::NodeCountOverflow {
                node_count: usize::MAX - 2
            })
        );
        assert_eq!(
            preflight_node_ids(usize::MAX - 3).map_err(|refusal| refusal.code()),
            Err(if usize::BITS > u32::BITS {
                "generated_id_out_of_range"
            } else {
                "document_node_limit_exceeded"
            })
        );
        assert_eq!(
            preflight_node_ids(MAX_FORMULA_DOCUMENT_NODES - 2),
            Err(FutureLoweringRefusal::DocumentNodeLimitExceeded {
                node_count: MAX_FORMULA_DOCUMENT_NODES + 1,
                limit: MAX_FORMULA_DOCUMENT_NODES
            })
        );
        assert_eq!(
            preflight_node_ids(MAX_FORMULA_DOCUMENT_NODES - 3),
            Ok([99_997, 99_998, 99_999].map(NodeId))
        );
    }

    // Trace: TC-046, FR-008-AC-1, FR-008-AC-3
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn every_generated_identity_is_converted() {
        let last_id = usize::try_from(u32::MAX).unwrap();
        for node_count in [last_id - 1, last_id, last_id + 1] {
            assert_eq!(
                preflight_node_ids(node_count),
                Err(FutureLoweringRefusal::GeneratedIdOutOfRange { node_count })
            );
        }
        assert_eq!(
            preflight_node_ids(last_id - 2).map_err(|refusal| refusal.code()),
            Err("document_node_limit_exceeded")
        );
    }
}
