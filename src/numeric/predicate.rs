//! Predicate tables (FR-303, borrowed core) and predicate binding (FR-304).
//!
//! A predicate defines one `PropositionId` as a comparison of two typed
//! expressions. The formula graph, node kinds and profiles are unchanged.

use core::fmt;

use super::expr::{type_arena, ExprId, ExprNode, ExprSlot, ExpressionError};
use crate::{
    Formula, NodeId, NodeKind, PropositionId, SignalCatalog, SignalDeclaration, SignalDomain,
    SignalId,
};

/// Maximum predicates in one table.
pub const MAX_PREDICATES: usize = 100_000;

/// Closed comparison vocabulary. A comparison is Boolean.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ComparisonOperator {
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `==`
    Eq,
    /// `!=`
    Ne,
}

impl ComparisonOperator {
    /// Every operator.
    pub const ALL: [Self; 6] = [Self::Lt, Self::Le, Self::Gt, Self::Ge, Self::Eq, Self::Ne];

    /// Stable wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::Eq => "eq",
            Self::Ne => "ne",
        }
    }
}

/// One predicate: `proposition := left <operator> right`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Predicate {
    /// The defined proposition.
    pub proposition: PropositionId,
    /// The comparison.
    pub operator: ComparisonOperator,
    /// Left expression root.
    pub left: ExprId,
    /// Right expression root.
    pub right: ExprId,
}

/// Refusal of a predicate table. No table is returned.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PredicateTableError {
    /// The shared expression arena is invalid (FR-302).
    Expression(ExpressionError),
    /// More than [`MAX_PREDICATES`] predicates.
    PredicateLimitExceeded {
        /// Supplied count.
        count: usize,
        /// Stable ceiling.
        limit: usize,
    },
    /// Predicate propositions are duplicated or not strictly increasing.
    PropositionNotIncreasing {
        /// Previous proposition.
        previous: PropositionId,
        /// Current proposition.
        current: PropositionId,
    },
    /// A comparison root names no expression node.
    RootOutOfRange {
        /// Predicate proposition.
        proposition: PropositionId,
        /// Rejected root.
        root: ExprId,
    },
    /// The two comparison operands have different scales.
    ScaleMismatch {
        /// Predicate proposition.
        proposition: PropositionId,
        /// Left scale.
        left: u8,
        /// Right scale.
        right: u8,
    },
}

impl From<ExpressionError> for PredicateTableError {
    fn from(error: ExpressionError) -> Self {
        Self::Expression(error)
    }
}

impl fmt::Display for PredicateTableError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(error) => error.fmt(formatter),
            Self::PredicateLimitExceeded { count, limit } => write!(
                formatter,
                "predicate count {count} exceeds the {limit}-predicate limit"
            ),
            Self::PropositionNotIncreasing { previous, current } => write!(
                formatter,
                "predicate proposition {} does not follow {}",
                current.0, previous.0
            ),
            Self::RootOutOfRange { proposition, root } => write!(
                formatter,
                "predicate {} root {} names no expression",
                proposition.0, root.0
            ),
            Self::ScaleMismatch {
                proposition,
                left,
                right,
            } => write!(
                formatter,
                "predicate {} compares scale {left} with scale {right}",
                proposition.0
            ),
        }
    }
}

/// Validates predicates against an already typed arena.
pub(crate) fn check_predicates(
    predicates: &[Predicate],
    slots: &[ExprSlot],
    arena_len: usize,
) -> Result<(), PredicateTableError> {
    if predicates.len() > MAX_PREDICATES {
        return Err(PredicateTableError::PredicateLimitExceeded {
            count: predicates.len(),
            limit: MAX_PREDICATES,
        });
    }
    for (index, predicate) in predicates.iter().enumerate() {
        if let Some(previous) = index.checked_sub(1).and_then(|item| predicates.get(item)) {
            if previous.proposition >= predicate.proposition {
                return Err(PredicateTableError::PropositionNotIncreasing {
                    previous: previous.proposition,
                    current: predicate.proposition,
                });
            }
        }
        let mut scales = [0_u8; 2];
        for (side, root) in [predicate.left, predicate.right].into_iter().enumerate() {
            let slot = root
                .index()
                .filter(|position| *position < arena_len)
                .and_then(|position| slots.get(position))
                .ok_or(PredicateTableError::RootOutOfRange {
                    proposition: predicate.proposition,
                    root,
                })?;
            scales[side] = slot.ty().scale;
        }
        if scales[0] != scales[1] {
            return Err(PredicateTableError::ScaleMismatch {
                proposition: predicate.proposition,
                left: scales[0],
                right: scales[1],
            });
        }
    }
    Ok(())
}

/// Allocation-free validated predicate table view.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PredicateTable<'a> {
    predicates: &'a [Predicate],
    expressions: &'a [ExprNode],
}

impl<'a> PredicateTable<'a> {
    /// Types the shared arena against `catalog` (one scratch slot per node) and
    /// validates every predicate. Any FR-302 or FR-303 error refuses the table.
    pub fn new(
        catalog: SignalCatalog<'_>,
        predicates: &'a [Predicate],
        expressions: &'a [ExprNode],
        scratch: &mut [ExprSlot],
    ) -> Result<Self, PredicateTableError> {
        type_arena(catalog, expressions, scratch, None)?;
        check_predicates(predicates, scratch, expressions.len())?;
        Ok(Self {
            predicates,
            expressions,
        })
    }

    /// Predicates in strictly increasing proposition order.
    pub const fn predicates(self) -> &'a [Predicate] {
        self.predicates
    }

    /// The shared expression arena.
    pub const fn expressions(self) -> &'a [ExprNode] {
        self.expressions
    }

    /// Looks up the predicate defining `proposition`.
    pub fn predicate(self, proposition: PropositionId) -> Option<&'a Predicate> {
        self.predicates
            .binary_search_by_key(&proposition, |predicate| predicate.proposition)
            .ok()
            .and_then(|index| self.predicates.get(index))
    }

    /// Exact set of catalog signals reachable from `predicate`, ascending.
    #[cfg(feature = "alloc")]
    pub fn predicate_signals(self, predicate: &Predicate) -> alloc::vec::Vec<SignalId> {
        let mut reachable = alloc::vec![false; self.expressions.len()];
        for root in [predicate.left, predicate.right] {
            if let Some(slot) = root.index().and_then(|index| reachable.get_mut(index)) {
                *slot = true;
            }
        }
        let mut signals = alloc::vec::Vec::new();
        for index in (0..self.expressions.len()).rev() {
            if !reachable[index] {
                continue;
            }
            let node = self.expressions[index];
            if let ExprNode::Signal { signal } = node {
                signals.push(signal);
            }
            for operand in node.operands().into_iter().flatten() {
                if let Some(slot) = operand.index().and_then(|item| reachable.get_mut(item)) {
                    *slot = true;
                }
            }
        }
        signals.sort_unstable();
        signals.dedup();
        signals
    }
}

/// Records "reaches non-numeric `signal`" in a scratch slot: depth 1 marks
/// it, and the minimum bound carries the signal identity.
fn mark(signal: Option<SignalId>) -> ExprSlot {
    match signal {
        Some(signal) => ExprSlot::marked(i64::from(signal.0)),
        None => ExprSlot::EMPTY,
    }
}

fn slot_signal(slot: ExprSlot) -> Option<SignalId> {
    (slot.depth() == 1).then(|| SignalId(u32::try_from(slot.ty().minimum).unwrap_or(u32::MAX)))
}

/// How one proposition occurrence is defined.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropositionSource<'c, 't> {
    /// A direct Boolean catalog binding.
    Signal(SignalDeclaration<'c>),
    /// A predicate-table definition.
    Predicate(&'t Predicate),
}

/// Refusal to bind a formula through a catalog and a predicate table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PredicateBindingError {
    /// The first occurring proposition with neither a binding nor a predicate.
    PropositionUnbound {
        /// Proposition.
        proposition: PropositionId,
    },
    /// The first occurring proposition with both a binding and a predicate.
    PropositionDefinedTwice {
        /// Proposition.
        proposition: PropositionId,
    },
    /// The first occurring predicate reading a signal that is not numeric here.
    NonNumericSignal {
        /// Predicate proposition.
        proposition: PropositionId,
        /// Signal that is Boolean or absent in this catalog.
        signal: SignalId,
    },
    /// The table, re-typed against the binding catalog, is ill-typed there
    /// (a scale or range that differs from the typing catalog's).
    IllTyped {
        /// The FR-302/FR-303 refusal under the binding catalog.
        error: PredicateTableError,
    },
}

impl fmt::Display for PredicateBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PropositionUnbound { proposition } => write!(
                formatter,
                "proposition {} has neither a signal binding nor a predicate",
                proposition.0
            ),
            Self::PropositionDefinedTwice { proposition } => write!(
                formatter,
                "proposition {} has both a signal binding and a predicate",
                proposition.0
            ),
            Self::NonNumericSignal {
                proposition,
                signal,
            } => write!(
                formatter,
                "predicate {} reads signal {} which is not numeric in this catalog",
                proposition.0, signal.0
            ),
            Self::IllTyped { error } => write!(
                formatter,
                "predicate table is ill-typed under the binding catalog: {error}"
            ),
        }
    }
}

/// A formula whose every proposition occurrence resolves to exactly one
/// source: a Boolean catalog binding or a predicate. Allocation-free.
#[derive(Clone, Copy, Debug)]
pub struct BoundPredicateFormula<'f, 'c, 't> {
    formula: Formula<'f>,
    catalog: SignalCatalog<'c>,
    table: PredicateTable<'t>,
}

impl<'f, 'c, 't> BoundPredicateFormula<'f, 'c, 't> {
    /// Resolves every proposition occurrence in formula-node order and
    /// re-types the table against `catalog` (review M-5), using one
    /// caller-owned scratch slot per expression node; nothing is allocated.
    ///
    /// The non-numeric check is exact: a forward pass marks, per expression
    /// node, the first non-numeric signal it reaches.
    pub fn new(
        formula: Formula<'f>,
        catalog: SignalCatalog<'c>,
        table: PredicateTable<'t>,
        scratch: &mut [ExprSlot],
    ) -> Result<Self, PredicateBindingError> {
        let expressions = table.expressions;
        if scratch.len() < expressions.len() {
            return Err(PredicateBindingError::IllTyped {
                error: PredicateTableError::Expression(ExpressionError::ScratchTooSmall {
                    provided: scratch.len(),
                    required: expressions.len(),
                }),
            });
        }
        // Pass 1: the first non-numeric signal each node reaches, if any,
        // recorded in the scratch slot before pass 2 overwrites it.
        for (index, node) in expressions.iter().enumerate() {
            let own = match node {
                ExprNode::Signal { signal } => match catalog.signal(*signal).map(|s| s.domain()) {
                    Some(SignalDomain::Integer(_) | SignalDomain::FixedDecimal(_)) => None,
                    _ => Some(*signal),
                },
                _ => None,
            };
            let inherited = node.operands().into_iter().flatten().find_map(|operand| {
                operand
                    .index()
                    .filter(|position| *position < index)
                    .and_then(|position| slot_signal(scratch[position]))
            });
            scratch[index] = mark(own.or(inherited));
        }
        for node in formula.nodes() {
            let NodeKind::Proposition { proposition } = node.kind else {
                continue;
            };
            match (
                catalog.signal_for_proposition(proposition),
                table.predicate(proposition),
            ) {
                (None, None) => {
                    return Err(PredicateBindingError::PropositionUnbound { proposition })
                }
                (Some(_), Some(_)) => {
                    return Err(PredicateBindingError::PropositionDefinedTwice { proposition })
                }
                (Some(_), None) => {}
                (None, Some(predicate)) => {
                    let read = |root: ExprId| {
                        root.index()
                            .and_then(|index| scratch.get(index))
                            .and_then(|slot| slot_signal(*slot))
                    };
                    if let Some(signal) = read(predicate.left).or_else(|| read(predicate.right)) {
                        return Err(PredicateBindingError::NonNumericSignal {
                            proposition,
                            signal,
                        });
                    }
                }
            }
        }
        // Pass 2: re-type against the binding catalog.
        type_arena(catalog, expressions, scratch, None)
            .map_err(|error| PredicateBindingError::IllTyped {
                error: PredicateTableError::Expression(error),
            })?;
        check_predicates(table.predicates, scratch, expressions.len())
            .map_err(|error| PredicateBindingError::IllTyped { error })?;
        Ok(Self {
            formula,
            catalog,
            table,
        })
    }

    /// The bound formula.
    pub const fn formula(self) -> Formula<'f> {
        self.formula
    }

    /// The catalog.
    pub const fn catalog(self) -> SignalCatalog<'c> {
        self.catalog
    }

    /// The predicate table.
    pub const fn table(self) -> PredicateTable<'t> {
        self.table
    }

    /// The source defining `proposition`, if any.
    pub fn source(self, proposition: PropositionId) -> Option<PropositionSource<'c, 't>> {
        match self.catalog.signal_for_proposition(proposition) {
            Some(signal) => Some(PropositionSource::Signal(signal)),
            None => self
                .table
                .predicate(proposition)
                .map(PropositionSource::Predicate),
        }
    }

    /// Every proposition occurrence in formula-node order with its source.
    pub fn occurrences(
        self,
    ) -> impl Iterator<Item = (NodeId, PropositionId, PropositionSource<'c, 't>)> + 'f
    where
        'c: 'f,
        't: 'f,
    {
        self.formula
            .nodes()
            .iter()
            .enumerate()
            .filter_map(move |(index, node)| match node.kind {
                NodeKind::Proposition { proposition } => {
                    // Validated formulas have at most u32::MAX + 1 nodes.
                    let id = NodeId(index as u32);
                    self.source(proposition)
                        .map(|source| (id, proposition, source))
                }
                _ => None,
            })
    }

    /// The ascending set of numeric signals the formula reads through its
    /// predicates.
    #[cfg(feature = "alloc")]
    pub fn numeric_signals(self) -> alloc::vec::Vec<SignalId> {
        let mut signals = alloc::vec::Vec::new();
        for node in self.formula.nodes() {
            let NodeKind::Proposition { proposition } = node.kind else {
                continue;
            };
            if self.catalog.signal_for_proposition(proposition).is_some() {
                continue;
            }
            if let Some(predicate) = self.table.predicate(proposition) {
                signals.extend(self.table.predicate_signals(predicate));
            }
        }
        signals.sort_unstable();
        signals.dedup();
        signals
    }
}
