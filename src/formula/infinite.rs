//! Canonical infinite-trace formula edition, separate from bounded v1/v2 bytes.

use core::{fmt, str::FromStr};

use crate::{FutureKind, Interval, NodeId, PropositionId, SemanticProfile, SourceSpan};

#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{
            array_field_population, read_strict_document, StrictDocument, StrictDocumentReadError,
        },
    },
    SyntaxArtifactLimits,
};
#[cfg(feature = "serde")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// The only profile admitted by the unbounded formula edition.
pub const INFINITE_TRACE_PROFILE: &str = "mltl.infinite-trace/v1";
/// The only clock admitted by infinite-trace documents.
pub const EVENT_POSITION_CLOCK: &str = "event_position";
/// The sibling formula wire edition, distinct from bounded v1 and v2.
pub const FORMULA_UNBOUNDED_V1: &str = "tl-syntax.formula-unbounded/v1";
/// The capability identity used for infinite-trace settlement routing.
pub const LIVENESS_CAPABILITY_V1: &str = "tl-syntax.liveness/v1";
/// Exact checked-in Draft 7 schema text for the unbounded formula edition.
#[cfg(feature = "serde")]
pub const FORMULA_UNBOUNDED_V1_SCHEMA: &str =
    include_str!("../../corpus/schema/formula-unbounded-v1.schema.json");
/// Exact checked-in Draft 7 schema bytes for the unbounded formula edition.
#[cfg(feature = "serde")]
pub const FORMULA_UNBOUNDED_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../corpus/schema/formula-unbounded-v1.schema.json");
/// SHA-256 of [`FORMULA_UNBOUNDED_V1_SCHEMA_BYTES`].
#[cfg(feature = "serde")]
pub const FORMULA_UNBOUNDED_V1_SCHEMA_SHA256: &str =
    "eae3e9d733fe288ad53fd321c67ad0da1cae3bbecad67df16bcd266ddcc10224";

/// Wire schema of the infinite formula edition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InfiniteFormulaSchemaVersion {
    /// First unbounded formula edition.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.formula-unbounded/v1"))]
    V1,
}

/// An inclusive interval with no upper bound, `[start,)`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct UnboundedInterval {
    start: u32,
}

impl UnboundedInterval {
    /// Constructs `[start,)`. Every `u32` lower bound is valid.
    pub const fn new(start: u32) -> Self {
        Self { start }
    }
    /// Returns the inclusive lower bound.
    pub const fn start(self) -> u32 {
        self.start
    }
}

/// Closed or unbounded temporal interval in the infinite formula edition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum TemporalInterval {
    /// Existing inclusive closed interval `[a,b]`.
    Closed(Interval),
    /// Open-upper interval `[a,)`.
    Unbounded(UnboundedInterval),
}

impl TemporalInterval {
    /// Returns the inclusive lower bound of either form.
    pub const fn start(self) -> u32 {
        match self {
            Self::Closed(interval) => interval.start(),
            Self::Unbounded(interval) => interval.start(),
        }
    }
}

/// Refusal for a non-canonical temporal interval spelling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TemporalIntervalParseError {
    /// The input is not `[a,b]` or `[a,)`.
    Malformed,
    /// A bound is not an unsigned 32-bit decimal integer.
    InvalidBound,
    /// A closed interval has its start after its end.
    Inverted,
}

impl fmt::Display for TemporalIntervalParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid temporal interval: {self:?}")
    }
}

impl FromStr for TemporalInterval {
    type Err = TemporalIntervalParseError;

    fn from_str(spelling: &str) -> Result<Self, Self::Err> {
        let (body, unbounded) = match (spelling.strip_prefix('['), spelling.ends_with(')')) {
            (Some(body), true) => (body.strip_suffix(')').ok_or(Self::Err::Malformed)?, true),
            (Some(body), false) => (body.strip_suffix(']').ok_or(Self::Err::Malformed)?, false),
            _ => return Err(Self::Err::Malformed),
        };
        let (start, end) = body.split_once(',').ok_or(Self::Err::Malformed)?;
        let parse_bound = |value: &str| {
            if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(Self::Err::InvalidBound);
            }
            value.parse::<u32>().map_err(|_| Self::Err::InvalidBound)
        };
        let start = parse_bound(start)?;
        if unbounded {
            if !end.is_empty() {
                return Err(Self::Err::Malformed);
            }
            Ok(Self::Unbounded(UnboundedInterval::new(start)))
        } else {
            let end = parse_bound(end)?;
            Interval::new(start, end)
                .map(Self::Closed)
                .map_err(|_| Self::Err::Inverted)
        }
    }
}

/// One canonical node in a formula-unbounded graph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InfiniteNode {
    /// Operator and operands.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: InfiniteNodeKind,
    /// Diagnostic source span, excluded from semantic identity.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub span: Option<SourceSpan>,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum InfiniteNodeWire {
    False {
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    True {
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Proposition {
        proposition: PropositionId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Not {
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    And {
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Or {
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Implies {
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Equivalent {
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Future {
        interval: TemporalInterval,
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Globally {
        interval: TemporalInterval,
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Until {
        interval: TemporalInterval,
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Release {
        interval: TemporalInterval,
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Once {
        interval: TemporalInterval,
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Historically {
        interval: TemporalInterval,
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    StrongPrevious {
        operand: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Since {
        interval: TemporalInterval,
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
    Triggered {
        interval: TemporalInterval,
        left: NodeId,
        right: NodeId,
        #[serde(default)]
        span: Option<SourceSpan>,
    },
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InfiniteNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use InfiniteNodeKind as Kind;
        let (kind, span) = match InfiniteNodeWire::deserialize(deserializer)? {
            InfiniteNodeWire::False { span } => (Kind::False, span),
            InfiniteNodeWire::True { span } => (Kind::True, span),
            InfiniteNodeWire::Proposition { proposition, span } => {
                (Kind::Proposition { proposition }, span)
            }
            InfiniteNodeWire::Not { operand, span } => (Kind::Not { operand }, span),
            InfiniteNodeWire::And { left, right, span } => (Kind::And { left, right }, span),
            InfiniteNodeWire::Or { left, right, span } => (Kind::Or { left, right }, span),
            InfiniteNodeWire::Implies { left, right, span } => {
                (Kind::Implies { left, right }, span)
            }
            InfiniteNodeWire::Equivalent { left, right, span } => {
                (Kind::Equivalent { left, right }, span)
            }
            InfiniteNodeWire::Future {
                interval,
                operand,
                span,
            } => (Kind::Future { interval, operand }, span),
            InfiniteNodeWire::Globally {
                interval,
                operand,
                span,
            } => (Kind::Globally { interval, operand }, span),
            InfiniteNodeWire::Until {
                interval,
                left,
                right,
                span,
            } => (
                Kind::Until {
                    interval,
                    left,
                    right,
                },
                span,
            ),
            InfiniteNodeWire::Release {
                interval,
                left,
                right,
                span,
            } => (
                Kind::Release {
                    interval,
                    left,
                    right,
                },
                span,
            ),
            InfiniteNodeWire::Once {
                interval,
                operand,
                span,
            } => (Kind::Once { interval, operand }, span),
            InfiniteNodeWire::Historically {
                interval,
                operand,
                span,
            } => (Kind::Historically { interval, operand }, span),
            InfiniteNodeWire::StrongPrevious { operand, span } => {
                (Kind::StrongPrevious { operand }, span)
            }
            InfiniteNodeWire::Since {
                interval,
                left,
                right,
                span,
            } => (
                Kind::Since {
                    interval,
                    left,
                    right,
                },
                span,
            ),
            InfiniteNodeWire::Triggered {
                interval,
                left,
                right,
                span,
            } => (
                Kind::Triggered {
                    interval,
                    left,
                    right,
                },
                span,
            ),
        };
        Ok(Self { kind, span })
    }
}

impl InfiniteNode {
    /// Constructs a node without a diagnostic span.
    pub const fn new(kind: InfiniteNodeKind) -> Self {
        Self { kind, span: None }
    }
    /// Constructs a node with a checked source span.
    pub const fn with_span(kind: InfiniteNodeKind, span: SourceSpan) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }
}

/// Closed operator vocabulary of the infinite-trace formula edition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)
)]
pub enum InfiniteNodeKind {
    /// Boolean false.
    False,
    /// Boolean true.
    True,
    /// Atomic proposition.
    Proposition {
        /// Proposition identity.
        proposition: PropositionId,
    },
    /// Negation.
    Not {
        /// Operand identity.
        operand: NodeId,
    },
    /// Conjunction.
    And {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Disjunction.
    Or {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Implication.
    Implies {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Equivalence.
    Equivalent {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Future.
    Future {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Operand.
        operand: NodeId,
    },
    /// Globally.
    Globally {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Operand.
        operand: NodeId,
    },
    /// Until.
    Until {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Release.
    Release {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Once.
    Once {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Operand.
        operand: NodeId,
    },
    /// Historically.
    Historically {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Operand.
        operand: NodeId,
    },
    /// Strong previous.
    StrongPrevious {
        /// Operand.
        operand: NodeId,
    },
    /// Since.
    Since {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Triggered.
    Triggered {
        /// Temporal interval.
        interval: TemporalInterval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
}

impl InfiniteNodeKind {
    /// Returns the node's predecessor operands.
    pub const fn operands(self) -> [Option<NodeId>; 2] {
        match self {
            Self::False | Self::True | Self::Proposition { .. } => [None, None],
            Self::Not { operand }
            | Self::Future { operand, .. }
            | Self::Globally { operand, .. }
            | Self::Once { operand, .. }
            | Self::Historically { operand, .. }
            | Self::StrongPrevious { operand } => [Some(operand), None],
            Self::And { left, right }
            | Self::Or { left, right }
            | Self::Implies { left, right }
            | Self::Equivalent { left, right }
            | Self::Until { left, right, .. }
            | Self::Release { left, right, .. }
            | Self::Since { left, right, .. }
            | Self::Triggered { left, right, .. } => [Some(left), Some(right)],
        }
    }
}

/// Exact discrete event-position clock selection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InfiniteClock {
    /// One tick per event position, including each loop repetition.
    #[cfg_attr(feature = "serde", serde(rename = "event_position"))]
    EventPosition,
}

impl InfiniteClock {
    /// Returns the stable identity.
    pub const fn as_str(self) -> &'static str {
        EVENT_POSITION_CLOCK
    }
}

/// Refusal selecting the infinite-trace clock identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfiniteClockError {
    /// No clock identity was supplied.
    Missing,
    /// A clock other than exact event positions was selected.
    Unsupported,
}

impl fmt::Display for InfiniteClockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => formatter.write_str("infinite-trace clock identity is missing"),
            Self::Unsupported => {
                formatter.write_str("infinite-trace clock requires event_position")
            }
        }
    }
}

impl TryFrom<&str> for InfiniteClock {
    type Error = InfiniteClockError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Missing)
        } else if value == EVENT_POSITION_CLOCK {
            Ok(Self::EventPosition)
        } else {
            Err(Self::Error::Unsupported)
        }
    }
}

/// Refusal selecting the exact TL infinite-trace profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfiniteProfileError {
    /// No TL profile identity was supplied.
    Missing,
    /// A different known TL profile was supplied.
    Mismatched(SemanticProfile),
    /// The spelling names no known TL profile.
    Unknown,
}

impl fmt::Display for InfiniteProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => formatter.write_str("infinite-trace TL profile identity is missing"),
            Self::Mismatched(profile) => write!(
                formatter,
                "TL profile {} is not infinite trace",
                profile.as_str()
            ),
            Self::Unknown => formatter.write_str("unknown TL profile identity"),
        }
    }
}

/// Selects the exact TL infinite-trace profile without inferring one from syntax.
pub fn select_infinite_profile(
    identity: Option<&str>,
) -> Result<SemanticProfile, InfiniteProfileError> {
    let identity = identity.ok_or(InfiniteProfileError::Missing)?;
    if identity.is_empty() {
        return Err(InfiniteProfileError::Missing);
    }
    match SemanticProfile::ALL
        .into_iter()
        .find(|profile| profile.as_str() == identity)
    {
        Some(SemanticProfile::InfiniteTraceV1) => Ok(SemanticProfile::InfiniteTraceV1),
        Some(profile) => Err(InfiniteProfileError::Mismatched(profile)),
        None => Err(InfiniteProfileError::Unknown),
    }
}

/// Typed admission error for an infinite formula graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InfiniteFormulaError {
    /// A different semantic profile was selected.
    Profile {
        /// Supplied profile.
        actual: SemanticProfile,
    },
    /// The root is outside the graph.
    Root {
        /// Supplied root.
        root: NodeId,
    },
    /// An operand does not precede its node.
    Operand {
        /// Node containing the operand.
        node: NodeId,
        /// Invalid operand.
        operand: NodeId,
    },
    /// The node budget is exceeded.
    NodeLimit {
        /// Actual node population.
        actual: usize,
    },
    /// The maximum graph depth is exceeded.
    DepthLimit {
        /// First offending node.
        node: NodeId,
    },
    /// Generated node identities would overflow the 32-bit range.
    GeneratedIdentityOverflow,
}

impl fmt::Display for InfiniteFormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "infinite formula admission: {self:?}")
    }
}

/// Borrowed, validated infinite formula graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InfiniteFormula<'a> {
    root: NodeId,
    nodes: &'a [InfiniteNode],
}

impl<'a> InfiniteFormula<'a> {
    /// Returns the root identity.
    pub const fn root(self) -> NodeId {
        self.root
    }
    /// Returns nodes in canonical topological order.
    pub const fn nodes(self) -> &'a [InfiniteNode] {
        self.nodes
    }
    /// Looks up one node.
    pub fn node(self, id: NodeId) -> Option<&'a InfiniteNode> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.nodes.get(index))
    }
    /// Returns the only admitted profile.
    pub const fn semantic_profile(self) -> SemanticProfile {
        SemanticProfile::InfiniteTraceV1
    }
}

/// Canonical three-node W/M lowering for the infinite formula edition.
///
/// The caller appends these nodes at `first_generated` and validates the
/// resulting document. `W = (p U q) | G p`; `M = (p R q) & F p`.
pub fn lower_infinite_future(
    kind: FutureKind,
    left: NodeId,
    right: NodeId,
    interval: TemporalInterval,
    first_generated: NodeId,
    span: Option<SourceSpan>,
) -> Result<[InfiniteNode; 3], InfiniteFormulaError> {
    let second = first_generated
        .0
        .checked_add(1)
        .map(NodeId)
        .ok_or(InfiniteFormulaError::GeneratedIdentityOverflow)?;
    first_generated
        .0
        .checked_add(2)
        .ok_or(InfiniteFormulaError::GeneratedIdentityOverflow)?;
    let kinds = match kind {
        FutureKind::WeakUntil => [
            InfiniteNodeKind::Until {
                interval,
                left,
                right,
            },
            InfiniteNodeKind::Globally {
                interval,
                operand: left,
            },
            InfiniteNodeKind::Or {
                left: first_generated,
                right: second,
            },
        ],
        FutureKind::StrongRelease => [
            InfiniteNodeKind::Release {
                interval,
                left,
                right,
            },
            InfiniteNodeKind::Future {
                interval,
                operand: left,
            },
            InfiniteNodeKind::And {
                left: first_generated,
                right: second,
            },
        ],
    };
    Ok(kinds.map(|kind| InfiniteNode { kind, span }))
}

/// Owned sibling of bounded formula v1/v2 with an open-upper interval vocabulary.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "InfiniteFormulaWire"))]
pub struct InfiniteFormulaDocument {
    schema_version: InfiniteFormulaSchemaVersion,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    root: NodeId,
    nodes: Vec<InfiniteNode>,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct InfiniteFormulaWire {
    schema_version: InfiniteFormulaSchemaVersion,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    root: NodeId,
    #[serde(deserialize_with = "deserialize_nodes")]
    nodes: Vec<InfiniteNode>,
}

#[cfg(feature = "serde")]
fn deserialize_nodes<'de, D>(deserializer: D) -> Result<Vec<InfiniteNode>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error as _, SeqAccess, Visitor};
    struct Nodes;
    impl<'de> Visitor<'de> for Nodes {
        type Value = Vec<InfiniteNode>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("at most 100000 infinite formula nodes")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut nodes = Vec::new();
            while let Some(node) = seq.next_element()? {
                if nodes.len() == crate::MAX_FORMULA_DOCUMENT_NODES {
                    return Err(A::Error::custom("infinite formula node limit exceeded"));
                }
                nodes.push(node);
            }
            Ok(nodes)
        }
    }
    deserializer.deserialize_seq(Nodes)
}

#[cfg(feature = "serde")]
impl TryFrom<InfiniteFormulaWire> for InfiniteFormulaDocument {
    type Error = InfiniteFormulaError;
    fn try_from(wire: InfiniteFormulaWire) -> Result<Self, Self::Error> {
        let _ = wire.schema_version;
        Self::new(wire.semantic_profile, wire.clock, wire.root, wire.nodes)
    }
}

#[cfg(feature = "alloc")]
impl InfiniteFormulaDocument {
    /// Constructs a canonical infinite formula after profile and graph checks.
    pub fn new(
        profile: SemanticProfile,
        clock: InfiniteClock,
        root: NodeId,
        nodes: Vec<InfiniteNode>,
    ) -> Result<Self, InfiniteFormulaError> {
        if profile != SemanticProfile::InfiniteTraceV1 {
            return Err(InfiniteFormulaError::Profile { actual: profile });
        }
        if nodes.len() > crate::MAX_FORMULA_DOCUMENT_NODES {
            return Err(InfiniteFormulaError::NodeLimit {
                actual: nodes.len(),
            });
        }
        let root_index =
            usize::try_from(root.0).map_err(|_| InfiniteFormulaError::Root { root })?;
        if root_index >= nodes.len() {
            return Err(InfiniteFormulaError::Root { root });
        }
        let mut depths = Vec::<usize>::with_capacity(nodes.len());
        for (index, node) in nodes.iter().enumerate() {
            let mut depth = 1_usize;
            for operand in node.kind.operands().into_iter().flatten() {
                let operand_index =
                    usize::try_from(operand.0).map_err(|_| InfiniteFormulaError::Operand {
                        node: NodeId(u32::try_from(index).unwrap_or(u32::MAX)),
                        operand,
                    })?;
                if operand_index >= index {
                    return Err(InfiniteFormulaError::Operand {
                        node: NodeId(u32::try_from(index).unwrap_or(u32::MAX)),
                        operand,
                    });
                }
                depth = depth.max(depths[operand_index].saturating_add(1));
            }
            if depth > crate::MAX_FORMULA_DOCUMENT_DEPTH {
                return Err(InfiniteFormulaError::DepthLimit {
                    node: NodeId(u32::try_from(index).unwrap_or(u32::MAX)),
                });
            }
            depths.push(depth);
        }
        Ok(Self {
            schema_version: InfiniteFormulaSchemaVersion::V1,
            semantic_profile: profile,
            clock,
            root,
            nodes,
        })
    }
    /// Returns a borrowed validated graph.
    pub fn formula(&self) -> InfiniteFormula<'_> {
        InfiniteFormula {
            root: self.root,
            nodes: &self.nodes,
        }
    }
    /// Returns the schema identity.
    pub const fn schema_version(&self) -> &'static str {
        FORMULA_UNBOUNDED_V1
    }
    /// Returns the exact TL profile identity.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        self.semantic_profile
    }
    /// Returns the clock binding.
    pub const fn clock(&self) -> InfiniteClock {
        self.clock
    }
    /// Returns the root.
    pub const fn root(&self) -> NodeId {
        self.root
    }
    /// Returns canonical graph nodes.
    pub fn nodes(&self) -> &[InfiniteNode] {
        &self.nodes
    }
    #[cfg(feature = "serde")]
    fn maximum_depth(&self) -> usize {
        let mut depths = Vec::<usize>::with_capacity(self.nodes.len());
        let mut maximum = 0;
        for node in &self.nodes {
            let mut depth = 1_usize;
            for operand in node.kind.operands().into_iter().flatten() {
                let index = usize::try_from(operand.0).unwrap_or(usize::MAX);
                let operand_depth = depths.get(index).copied().unwrap_or(0);
                depth = depth.max(operand_depth.saturating_add(1));
            }
            maximum = maximum.max(depth);
            depths.push(depth);
        }
        maximum
    }
    /// Serializes to canonical JSON.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }
    /// Reads one exact, bounded canonical document.
    #[cfg(feature = "serde")]
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<Self, StrictDocumentReadError> {
        read_strict_document(bytes, limits)
    }
    /// Returns the domain-separated SHA-256 content identity.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        self.canonical_json_bytes()
            .map(|bytes| crate::contracts::identity::content_identity(FORMULA_UNBOUNDED_V1, &bytes))
    }
}

#[cfg(feature = "serde")]
impl StrictDocument for InfiniteFormulaDocument {
    fn preflight_resource_limits(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError> {
        let nodes = array_field_population(bytes, b"nodes");
        if nodes > limits.formula_nodes {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "formula nodes",
                actual: nodes,
                limit: limits.formula_nodes,
            });
        }
        Ok(nodes)
    }
    fn validate_resource_limits(
        &self,
        limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError> {
        if self.nodes.len() > limits.formula_nodes {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "formula nodes",
                actual: self.nodes.len(),
                limit: limits.formula_nodes,
            });
        }
        let depth = self.maximum_depth();
        if depth > limits.formula_depth {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "formula depth",
                actual: depth,
                limit: limits.formula_depth,
            });
        }
        Ok(())
    }
}
