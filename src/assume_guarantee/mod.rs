//! Assume-guarantee contracts (P-TL-150, TL-425 syntax half; FR-309..FR-311).
//!
//! A contract names an assumption root and a guarantee root in one existing
//! formula graph. This module owns the one status vocabulary and truth table
//! shared by the evaluator, the exporter and the solver (FR-310), the
//! obligation lowering, and the prefix-conditional reading of a per-position
//! outcome series that realizability consumes (ADR-017 option C, decided
//! 2026-11-23). The core is allocation-free; the document is under `serde`.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{Node, NodeId, NodeKind};
#[cfg(feature = "alloc")]
use crate::{Formula, FormulaBindingError, SignalCatalog, SignalDirection};

#[cfg(feature = "serde")]
pub mod document;

/// Maximum contracts in one contract set.
pub const MAX_CONTRACTS: usize = 10_000;

/// A Boolean-or-pending result of one root at one position.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContractTruth {
    /// The root is false here.
    False,
    /// The root is true here.
    True,
    /// The profile leaves the root undecided here.
    Pending,
}

impl ContractTruth {
    /// Every value, `False`, `True`, `Pending`.
    pub const ALL: [Self; 3] = [Self::False, Self::True, Self::Pending];

    /// Whether `self` is `other` or a decided refinement of a `Pending` other.
    pub const fn refines(self, other: Self) -> bool {
        matches!(
            (self, other),
            (_, Self::Pending) | (Self::True, Self::True) | (Self::False, Self::False)
        )
    }
}

/// The three contract statuses (ADR-017 decision 2).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContractStatus {
    /// The assumption does not hold; the guarantee is not consulted.
    Inactive,
    /// The assumption holds and the guarantee does not.
    Invalid,
    /// The assumption and the guarantee hold.
    Verified,
}

impl ContractStatus {
    /// Stable wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Invalid => "invalid",
            Self::Verified => "verified",
        }
    }
}

/// A status or the non-verdict `Pending`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContractOutcome {
    /// A decided status.
    Status(ContractStatus),
    /// Not decided at this position.
    Pending,
}

impl ContractOutcome {
    /// Stable wire spelling: `inactive`, `invalid`, `verified` or `pending`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Status(status) => status.as_str(),
            Self::Pending => "pending",
        }
    }
}

/// The FR-310 truth table: total, allocation-free and monotone in
/// information.
///
/// | a | g | outcome |
/// |---|---|---|
/// | False | any | Inactive |
/// | True | False | Invalid |
/// | True | True | Verified |
/// | True | Pending | Pending |
/// | Pending | any | Pending |
pub const fn contract_outcome(
    assumption: ContractTruth,
    guarantee: ContractTruth,
) -> ContractOutcome {
    match (assumption, guarantee) {
        (ContractTruth::False, _) => ContractOutcome::Status(ContractStatus::Inactive),
        (ContractTruth::True, ContractTruth::False) => {
            ContractOutcome::Status(ContractStatus::Invalid)
        }
        (ContractTruth::True, ContractTruth::True) => {
            ContractOutcome::Status(ContractStatus::Verified)
        }
        (ContractTruth::True, ContractTruth::Pending) | (ContractTruth::Pending, _) => {
            ContractOutcome::Pending
        }
    }
}

// SPEC-GAP: FR-311 and StR-016-VC-2 leave the realizability reading and the
// Input-only check to P-TL-149 (tl-solve FR-100). Decision 1 (2026-11-23)
// adopts option C and requires the typed refusal, so the shared, total pieces
// live here for tl-solve and tl-mltl to call; FR-311 text to amend ("the
// prefix verdict and Input-only check are offered by tl-syntax and applied
// by tl-solve"). tl-syntax still states no assumption scope on any document.

/// The settled-positions reading of a per-position outcome series (decision
/// 1, option C, with tl-mltl ADR-004 decision 5): "the guarantee holds at
/// every settled position at which the assumption has held so far".
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AssumptionPrefixVerdict {
    /// No settled position is `invalid` before the first `inactive` one.
    Holds,
    /// The first settled `invalid` position with no earlier `inactive` one:
    /// the system loses the play.
    Violated {
        /// The violating position.
        position: usize,
    },
    /// A settled position reports `pending`: a tool fault (`Failed`), never
    /// a verdict.
    Fault {
        /// The first settled pending position.
        position: usize,
    },
}

/// The number of settled positions of a complete play of `play_length`
/// positions for a contract of lookahead `look = max(lookahead(A),
/// lookahead(G))`: position `p` is settled when `p + look <= H`, where
/// `H = play_length - 1` is the last position, so that no read at `p`
/// reaches past the play.
pub const fn settled_positions(play_length: usize, look: usize) -> usize {
    play_length.saturating_sub(look)
}

/// Decides a complete play from its per-position outcome series. Only the
/// first `settled` positions decide; the unsettled tail is ignored, so a
/// winning play's pending tail still reads `Holds`. In order: `inactive` ends
/// the obligation (the assumption stopped holding), `invalid` violates it,
/// `verified` continues, and `pending` at a settled position is a `Fault`.
pub fn assumption_prefix_verdict(
    series: &[ContractOutcome],
    settled: usize,
) -> AssumptionPrefixVerdict {
    let settled = settled.min(series.len());
    for (position, outcome) in series[..settled].iter().enumerate() {
        match outcome {
            ContractOutcome::Status(ContractStatus::Inactive) => {
                return AssumptionPrefixVerdict::Holds
            }
            ContractOutcome::Status(ContractStatus::Invalid) => {
                return AssumptionPrefixVerdict::Violated { position }
            }
            ContractOutcome::Status(ContractStatus::Verified) => {}
            ContractOutcome::Pending => return AssumptionPrefixVerdict::Fault { position },
        }
    }
    AssumptionPrefixVerdict::Holds
}

/// Refusal to lower a contract obligation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ContractObligationError {
    /// A root names no node of the graph.
    RootOutOfRange {
        /// Rejected root.
        root: NodeId,
    },
    /// The graph is already at the document node limit.
    NodeLimit {
        /// Stable ceiling.
        limit: usize,
    },
}

impl fmt::Display for ContractObligationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootOutOfRange { root } => {
                write!(formatter, "contract root {} names no node", root.0)
            }
            Self::NodeLimit { limit } => {
                write!(formatter, "obligation lowering would exceed {limit} nodes")
            }
        }
    }
}

/// Builds the obligation node `Implies(assumption, guarantee)` for a graph of
/// `node_count` nodes. The caller appends it; its identity is `node_count`.
/// It is true exactly when the outcome is inactive or verified.
pub fn contract_obligation_node(
    node_count: usize,
    assumption: NodeId,
    guarantee: NodeId,
) -> Result<(NodeId, Node), ContractObligationError> {
    for root in [assumption, guarantee] {
        if usize::try_from(root.0).map_or(true, |index| index >= node_count) {
            return Err(ContractObligationError::RootOutOfRange { root });
        }
    }
    if node_count >= crate::MAX_FORMULA_DOCUMENT_NODES {
        return Err(ContractObligationError::NodeLimit {
            limit: crate::MAX_FORMULA_DOCUMENT_NODES,
        });
    }
    // `node_count` is below the document node limit, which fits `u32`.
    let id = NodeId(node_count as u32);
    Ok((
        id,
        Node::new(NodeKind::Implies {
            left: assumption,
            right: guarantee,
        }),
    ))
}

/// Appends the obligation `Implies(assumption, guarantee)` to `nodes`, using
/// one node of the budget and keeping every existing node and identity.
#[cfg(feature = "alloc")]
pub fn lower_contract_obligation(
    nodes: &mut Vec<Node>,
    assumption: NodeId,
    guarantee: NodeId,
) -> Result<NodeId, ContractObligationError> {
    let (id, node) = contract_obligation_node(nodes.len(), assumption, guarantee)?;
    nodes.push(node);
    Ok(id)
}

/// Refusal of a contract assumption as a realizability assumption.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ContractAssumptionError {
    /// The assumption root names no node.
    RootOutOfRange {
        /// Rejected root.
        root: NodeId,
    },
    /// A proposition of the assumption has no direct Boolean binding.
    Binding(crate::FormulaBindingError),
    /// A proposition of the assumption is not bound to an `Input` signal.
    NotInput {
        /// First such proposition in node order.
        proposition: crate::PropositionId,
        /// Its direction (`Output` or `Undeclared`).
        direction: crate::SignalDirection,
    },
}

impl fmt::Display for ContractAssumptionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootOutOfRange { root } => {
                write!(formatter, "assumption root {} names no node", root.0)
            }
            Self::Binding(error) => error.fmt(formatter),
            Self::NotInput {
                proposition,
                direction,
            } => write!(
                formatter,
                "assumption proposition {} has direction {direction:?}, not Input",
                proposition.0
            ),
        }
    }
}

/// Requires every proposition reachable from `assumption` to be bound to an
/// `Input` signal (decision 1: otherwise the system could win by falsifying
/// its own assumption). The first offending proposition in node order is
/// reported.
#[cfg(feature = "alloc")]
pub fn check_assumption_inputs(
    catalog: SignalCatalog<'_>,
    formula: Formula<'_>,
    assumption: NodeId,
) -> Result<(), ContractAssumptionError> {
    let nodes = formula.nodes();
    let root = usize::try_from(assumption.0)
        .ok()
        .filter(|index| *index < nodes.len())
        .ok_or(ContractAssumptionError::RootOutOfRange { root: assumption })?;
    let mut reachable = alloc::vec![false; nodes.len()];
    reachable[root] = true;
    for index in (0..=root).rev() {
        if !reachable[index] {
            continue;
        }
        for operand in nodes[index].kind.operands().into_iter().flatten() {
            if let Some(slot) = usize::try_from(operand.0)
                .ok()
                .and_then(|item| reachable.get_mut(item))
            {
                *slot = true;
            }
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        if !reachable[index] {
            continue;
        }
        let NodeKind::Proposition { proposition } = node.kind else {
            continue;
        };
        let direction = catalog.proposition_direction(proposition).ok_or(
            ContractAssumptionError::Binding(FormulaBindingError::MissingPropositionBinding {
                proposition,
            }),
        )?;
        if direction != SignalDirection::Input {
            return Err(ContractAssumptionError::NotInput {
                proposition,
                direction,
            });
        }
    }
    Ok(())
}
