//! Closed semantic profiles and temporal operator catalogs.

/// Closed past-time operator-profile identity.
pub const PAST_OPERATORS_V1: &str = "tl-syntax.past-operators/v1";

/// Finite-trace semantic profile attached to an exchanged formula.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SemanticProfile {
    /// Boolean semantics over a complete finite trace.
    #[cfg_attr(feature = "serde", serde(rename = "mltl.closed-trace/v1"))]
    ClosedTraceV1,
    /// Online prefix semantics that remain pending until the prefix decides the formula.
    #[cfg_attr(feature = "serde", serde(rename = "mltl.online-prefix/v1"))]
    OnlinePrefixV1,
    /// Origin-complete past-time semantics over a discrete position history.
    #[cfg_attr(feature = "serde", serde(rename = "mltl.origin-complete-history/v1"))]
    OriginCompleteHistoryV1,
    /// Discrete infinite trace semantics, admitting future and past operators.
    #[cfg_attr(feature = "serde", serde(rename = "mltl.infinite-trace/v1"))]
    InfiniteTraceV1,
}

impl SemanticProfile {
    /// Every profile; `as_str`'s exhaustive match names the same set.
    pub const ALL: [Self; 4] = [
        Self::ClosedTraceV1,
        Self::OnlinePrefixV1,
        Self::OriginCompleteHistoryV1,
        Self::InfiniteTraceV1,
    ];

    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClosedTraceV1 => "mltl.closed-trace/v1",
            Self::OnlinePrefixV1 => "mltl.online-prefix/v1",
            Self::OriginCompleteHistoryV1 => "mltl.origin-complete-history/v1",
            Self::InfiniteTraceV1 => "mltl.infinite-trace/v1",
        }
    }
}

/// Temporal direction owned by a primitive node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TemporalFamily {
    /// Future-time F/G/U/R primitive.
    Future,
    /// Past-time O/H/Y/S/T primitive.
    Past,
}

/// Operand count of a temporal operator.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperatorArity {
    /// One formula operand.
    Unary,
    /// Two formula operands.
    Binary,
}

/// Closed semantic catalog for [`PAST_OPERATORS_V1`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PastOperatorKind {
    /// Bounded Once.
    Once,
    /// Bounded Historically.
    Historically,
    /// Strong Previous, with the truth relation of `Once[1,1]`.
    StrongPrevious,
    /// Bounded Since.
    Since,
    /// Bounded Triggered.
    Triggered,
}

impl PastOperatorKind {
    /// Every operator in the closed v1 catalog.
    pub const ALL: [Self; 5] = [
        Self::Once,
        Self::Historically,
        Self::StrongPrevious,
        Self::Since,
        Self::Triggered,
    ];

    /// Returns the stable semantic node name, not a parser spelling.
    pub const fn semantic_name(self) -> &'static str {
        match self {
            Self::Once => "Once",
            Self::Historically => "Historically",
            Self::StrongPrevious => "StrongPrevious",
            Self::Since => "Since",
            Self::Triggered => "Triggered",
        }
    }

    /// Returns the operator's fixed operand count.
    pub const fn arity(self) -> OperatorArity {
        match self {
            Self::Once | Self::Historically | Self::StrongPrevious => OperatorArity::Unary,
            Self::Since | Self::Triggered => OperatorArity::Binary,
        }
    }
}
