//! Versioned infinite-trace observations and fairness premises.

use alloc::{string::String, vec::Vec};
use core::fmt;

#[cfg(feature = "serde")]
use crate::contracts::identity::canonical_json;
use crate::{InfiniteClock, InfiniteFormulaDocument, NodeId, PropositionId, SemanticProfile};

/// Wire identity for one four-state valuation.
pub const PARTIAL_VALUATION_V1: &str = "tl-syntax.partial-valuation/v1";
/// Wire identity for a finite-prefix and nonempty-loop trace.
pub const LASSO_TRACE_V1: &str = "tl-syntax.lasso-trace/v1";
/// Wire identity for unevaluated fairness premises.
pub const FAIRNESS_PREMISES_V1: &str = "tl-syntax.fairness-premises/v1";

/// Evidence state of one proposition at one position.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PartialValue {
    /// Exactly true.
    True,
    /// Exactly false.
    False,
    /// No evidence was supplied.
    Missing,
    /// Both true and false evidence were supplied.
    Conflicting,
}

/// One proposition's state in canonical proposition order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ValuationEntry {
    /// Proposition-map identity.
    pub proposition: PropositionId,
    /// Exact four-valued evidence state.
    pub value: PartialValue,
}

/// Admission refusal for a partial valuation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartialValuationError {
    /// The proposition-map identity is empty.
    MissingMapIdentity,
    /// A declared proposition identity repeats or is not increasing.
    MapOrder,
    /// An entry is absent from the declared map.
    ForeignProposition {
        /// Rejected identity.
        proposition: PropositionId,
    },
    /// Two entries identify the same proposition.
    DuplicateProposition {
        /// Repeated identity.
        proposition: PropositionId,
    },
    /// Entries are not in canonical proposition order.
    UnorderedProposition {
        /// Rejected identity.
        proposition: PropositionId,
    },
    /// A declared proposition has no entry.
    OmittedProposition {
        /// Missing identity.
        proposition: PropositionId,
    },
    /// The map identity differs from the enclosing trace.
    MapIdentityMismatch,
    /// The entry count exceeds the owner bound.
    ResourceLimit,
}

impl fmt::Display for PartialValuationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "partial valuation admission: {self:?}")
    }
}

/// Immutable four-valued observation over one exact proposition map.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "PartialValuationWire"))]
pub struct PartialValuation {
    schema_version: PartialValuationSchema,
    proposition_map_identity: String,
    propositions: Vec<PropositionId>,
    entries: Vec<ValuationEntry>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum PartialValuationSchema {
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.partial-valuation/v1"))]
    V1,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PartialValuationWire {
    schema_version: PartialValuationSchema,
    proposition_map_identity: String,
    propositions: Vec<PropositionId>,
    entries: Vec<ValuationEntry>,
}

#[cfg(feature = "serde")]
impl TryFrom<PartialValuationWire> for PartialValuation {
    type Error = PartialValuationError;
    fn try_from(wire: PartialValuationWire) -> Result<Self, Self::Error> {
        let _ = wire.schema_version;
        Self::new(
            wire.proposition_map_identity,
            &wire.propositions,
            wire.entries,
        )
    }
}

impl PartialValuation {
    /// Constructs an exact, canonically ordered valuation over the declared map.
    pub fn new(
        map_identity: String,
        propositions: &[PropositionId],
        entries: Vec<ValuationEntry>,
    ) -> Result<Self, PartialValuationError> {
        let mut value = Self::admit_without_map(map_identity, entries)?;
        if propositions.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(PartialValuationError::MapOrder);
        }
        for (index, proposition) in propositions.iter().enumerate() {
            match value.entries.get(index) {
                None => {
                    return Err(PartialValuationError::OmittedProposition {
                        proposition: *proposition,
                    })
                }
                Some(entry) if entry.proposition < *proposition => {
                    return Err(PartialValuationError::ForeignProposition {
                        proposition: entry.proposition,
                    })
                }
                Some(entry) if entry.proposition > *proposition => {
                    return Err(PartialValuationError::OmittedProposition {
                        proposition: *proposition,
                    })
                }
                Some(_) => {}
            }
        }
        if let Some(extra) = value.entries.get(propositions.len()) {
            return Err(PartialValuationError::ForeignProposition {
                proposition: extra.proposition,
            });
        }
        value.propositions = propositions.to_vec();
        Ok(value)
    }
    fn admit_without_map(
        map_identity: String,
        entries: Vec<ValuationEntry>,
    ) -> Result<Self, PartialValuationError> {
        if map_identity.is_empty() {
            return Err(PartialValuationError::MissingMapIdentity);
        }
        if entries.len() > crate::contracts::limits::OWNER_PROPOSITIONS {
            return Err(PartialValuationError::ResourceLimit);
        }
        for pair in entries.windows(2) {
            if pair[0].proposition == pair[1].proposition {
                return Err(PartialValuationError::DuplicateProposition {
                    proposition: pair[1].proposition,
                });
            }
            if pair[0].proposition > pair[1].proposition {
                return Err(PartialValuationError::UnorderedProposition {
                    proposition: pair[1].proposition,
                });
            }
        }
        Ok(Self {
            schema_version: PartialValuationSchema::V1,
            proposition_map_identity: map_identity,
            propositions: Vec::new(),
            entries,
        })
    }
    /// Returns exact map identity.
    pub fn proposition_map_identity(&self) -> &str {
        &self.proposition_map_identity
    }
    /// Returns entries in canonical identity order.
    pub fn entries(&self) -> &[ValuationEntry] {
        &self.entries
    }
    /// Returns the declared proposition identities.
    pub fn propositions(&self) -> &[PropositionId] {
        &self.propositions
    }
    /// Looks up a proposition without Boolean coercion.
    pub fn value(&self, proposition: PropositionId) -> Option<PartialValue> {
        self.entries
            .binary_search_by_key(&proposition, |entry| entry.proposition)
            .ok()
            .map(|index| self.entries[index].value)
    }
    /// Returns canonical JSON bytes.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }
    /// Returns domain-separated content identity.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        self.canonical_json_bytes()
            .map(|bytes| crate::contracts::identity::content_identity(PARTIAL_VALUATION_V1, &bytes))
    }
}

/// One position of the materialized prefix or loop.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct TraceObservation {
    /// Absolute position in the materialized prefix plus one loop lap.
    pub position: u32,
    /// Exact proposition evidence at this position.
    pub valuation: PartialValuation,
}

/// Admission refusal for a lasso trace.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LassoTraceError {
    /// The selected profile is not infinite trace.
    Profile {
        /// Rejected profile.
        actual: SemanticProfile,
    },
    /// A non-event-position clock was selected.
    Clock,
    /// The loop is empty.
    EmptyLoop,
    /// An observation has the wrong materialized position.
    Position {
        /// Expected position.
        expected: u32,
        /// Supplied position.
        actual: u32,
    },
    /// Materialized positions exceed `u32`.
    PositionLimit,
    /// A valuation names another proposition map.
    MapIdentityMismatch,
    /// A valuation has the wrong proposition population.
    Valuation(PartialValuationError),
    /// Trace length exceeds the owner bound.
    ResourceLimit,
}

impl fmt::Display for LassoTraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lasso trace admission: {self:?}")
    }
}

/// Validated finite prefix plus nonempty repeating loop.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "LassoTraceWire"))]
pub struct LassoTraceDocument {
    schema_version: LassoTraceSchema,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    proposition_map_identity: String,
    propositions: Vec<PropositionId>,
    prefix: Vec<TraceObservation>,
    loop_observations: Vec<TraceObservation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum LassoTraceSchema {
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.lasso-trace/v1"))]
    V1,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct LassoTraceWire {
    schema_version: LassoTraceSchema,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    proposition_map_identity: String,
    propositions: Vec<PropositionId>,
    prefix: Vec<TraceObservation>,
    loop_observations: Vec<TraceObservation>,
}

#[cfg(feature = "serde")]
impl TryFrom<LassoTraceWire> for LassoTraceDocument {
    type Error = LassoTraceError;
    fn try_from(wire: LassoTraceWire) -> Result<Self, Self::Error> {
        let _ = wire.schema_version;
        Self::new(
            wire.semantic_profile,
            wire.clock,
            wire.proposition_map_identity,
            wire.propositions,
            wire.prefix,
            wire.loop_observations,
        )
    }
}

impl LassoTraceDocument {
    /// Constructs a lasso and validates every position and valuation.
    pub fn new(
        profile: SemanticProfile,
        clock: InfiniteClock,
        map_identity: String,
        propositions: Vec<PropositionId>,
        prefix: Vec<TraceObservation>,
        loop_observations: Vec<TraceObservation>,
    ) -> Result<Self, LassoTraceError> {
        if profile != SemanticProfile::InfiniteTraceV1 {
            return Err(LassoTraceError::Profile { actual: profile });
        }
        if loop_observations.is_empty() {
            return Err(LassoTraceError::EmptyLoop);
        }
        let total = prefix
            .len()
            .checked_add(loop_observations.len())
            .ok_or(LassoTraceError::PositionLimit)?;
        if total > crate::MAX_FORMULA_DOCUMENT_NODES {
            return Err(LassoTraceError::ResourceLimit);
        }
        let mut expected = 0_u32;
        for observation in prefix.iter().chain(&loop_observations) {
            if observation.position != expected {
                return Err(LassoTraceError::Position {
                    expected,
                    actual: observation.position,
                });
            }
            if observation.valuation.proposition_map_identity() != map_identity {
                return Err(LassoTraceError::MapIdentityMismatch);
            }
            PartialValuation::new(
                map_identity.clone(),
                &propositions,
                observation.valuation.entries().to_vec(),
            )
            .map_err(LassoTraceError::Valuation)?;
            expected = expected
                .checked_add(1)
                .ok_or(LassoTraceError::PositionLimit)?;
        }
        Ok(Self {
            schema_version: LassoTraceSchema::V1,
            semantic_profile: profile,
            clock,
            proposition_map_identity: map_identity,
            propositions,
            prefix,
            loop_observations,
        })
    }
    /// Returns the exact profile.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        self.semantic_profile
    }
    /// Returns the exact clock.
    pub const fn clock(&self) -> InfiniteClock {
        self.clock
    }
    /// Returns the map identity.
    pub fn proposition_map_identity(&self) -> &str {
        &self.proposition_map_identity
    }
    /// Returns declared propositions.
    pub fn propositions(&self) -> &[PropositionId] {
        &self.propositions
    }
    /// Returns finite prefix positions.
    pub fn prefix(&self) -> &[TraceObservation] {
        &self.prefix
    }
    /// Returns the nonempty loop positions.
    pub fn loop_observations(&self) -> &[TraceObservation] {
        &self.loop_observations
    }
    /// Returns first loop position.
    pub fn loop_entry(&self) -> usize {
        self.prefix.len()
    }
    /// Returns observation at an arbitrary infinite position.
    pub fn observation_at(&self, position: u64) -> &TraceObservation {
        let prefix_len = u64::try_from(self.prefix.len()).unwrap_or(u64::MAX);
        if position < prefix_len {
            return &self.prefix[usize::try_from(position).unwrap_or(0)];
        }
        let loop_len = u64::try_from(self.loop_observations.len()).unwrap_or(u64::MAX);
        let index = usize::try_from((position - prefix_len) % loop_len).unwrap_or(0);
        &self.loop_observations[index]
    }
    /// Returns canonical JSON bytes.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }
    /// Returns domain-separated content identity.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        self.canonical_json_bytes()
            .map(|bytes| crate::contracts::identity::content_identity(LASSO_TRACE_V1, &bytes))
    }
}

/// Admission refusal for fairness references.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FairnessPremisesError {
    /// A root is absent from the supplied formula graph.
    ForeignRoot {
        /// Rejected root.
        root: NodeId,
    },
    /// A root repeats.
    DuplicateRoot {
        /// Repeated root.
        root: NodeId,
    },
    /// Roots are not in increasing order.
    UnorderedRoot {
        /// Rejected root.
        root: NodeId,
    },
    /// The formula's semantic profile is not infinite trace.
    Profile,
    /// Clock differs from the formula's clock.
    Clock,
    /// The graph identity is absent.
    MissingGraphIdentity,
    /// The supplied identity does not identify the formula document.
    GraphIdentityMismatch,
    /// Premise population exceeds the owner limit.
    ResourceLimit,
}

impl fmt::Display for FairnessPremisesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fairness premises admission: {self:?}")
    }
}

/// Ordered unevaluated fairness roots bound to one canonical formula graph.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "FairnessPremisesWire"))]
pub struct FairnessPremisesDocument {
    schema_version: FairnessPremisesSchema,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    graph_identity: String,
    roots: Vec<NodeId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum FairnessPremisesSchema {
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.fairness-premises/v1"))]
    V1,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FairnessPremisesWire {
    schema_version: FairnessPremisesSchema,
    semantic_profile: SemanticProfile,
    clock: InfiniteClock,
    graph_identity: String,
    roots: Vec<NodeId>,
}

#[cfg(feature = "serde")]
impl TryFrom<FairnessPremisesWire> for FairnessPremisesDocument {
    type Error = FairnessPremisesError;
    fn try_from(wire: FairnessPremisesWire) -> Result<Self, Self::Error> {
        let _ = wire.schema_version;
        Self::new_unchecked_graph(
            wire.semantic_profile,
            wire.clock,
            wire.graph_identity,
            wire.roots,
        )
    }
}

impl FairnessPremisesDocument {
    /// Constructs premise roots and binds them to the exact formula document.
    pub fn new(
        formula: &InfiniteFormulaDocument,
        graph_identity: String,
        clock: InfiniteClock,
        roots: Vec<NodeId>,
    ) -> Result<Self, FairnessPremisesError> {
        if formula.semantic_profile() != SemanticProfile::InfiniteTraceV1 {
            return Err(FairnessPremisesError::Profile);
        }
        if formula.clock() != clock {
            return Err(FairnessPremisesError::Clock);
        }
        #[cfg(feature = "serde")]
        if formula
            .content_identity()
            .map_err(|_| FairnessPremisesError::GraphIdentityMismatch)?
            != graph_identity
        {
            return Err(FairnessPremisesError::GraphIdentityMismatch);
        }
        let value =
            Self::new_unchecked_graph(formula.semantic_profile(), clock, graph_identity, roots)?;
        for root in &value.roots {
            if formula.formula().node(*root).is_none() {
                return Err(FairnessPremisesError::ForeignRoot { root: *root });
            }
        }
        Ok(value)
    }
    fn new_unchecked_graph(
        profile: SemanticProfile,
        clock: InfiniteClock,
        graph_identity: String,
        roots: Vec<NodeId>,
    ) -> Result<Self, FairnessPremisesError> {
        if profile != SemanticProfile::InfiniteTraceV1 {
            return Err(FairnessPremisesError::Profile);
        }
        if graph_identity.is_empty() {
            return Err(FairnessPremisesError::MissingGraphIdentity);
        }
        if roots.len() > crate::MAX_FORMULA_DOCUMENT_NODES {
            return Err(FairnessPremisesError::ResourceLimit);
        }
        for pair in roots.windows(2) {
            if pair[0] == pair[1] {
                return Err(FairnessPremisesError::DuplicateRoot { root: pair[1] });
            }
            if pair[0] > pair[1] {
                return Err(FairnessPremisesError::UnorderedRoot { root: pair[1] });
            }
        }
        Ok(Self {
            schema_version: FairnessPremisesSchema::V1,
            semantic_profile: profile,
            clock,
            graph_identity,
            roots,
        })
    }
    /// Returns the exact formula graph identity.
    pub fn graph_identity(&self) -> &str {
        &self.graph_identity
    }
    /// Returns ordered premise roots.
    pub fn roots(&self) -> &[NodeId] {
        &self.roots
    }
    /// Returns the clock binding.
    pub const fn clock(&self) -> InfiniteClock {
        self.clock
    }
    /// Returns canonical JSON bytes.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }
    /// Returns domain-separated content identity.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        self.canonical_json_bytes()
            .map(|bytes| crate::contracts::identity::content_identity(FAIRNESS_PREMISES_V1, &bytes))
    }
}
