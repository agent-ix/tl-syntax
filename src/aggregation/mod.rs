//! Compile-time set aggregation (P-TL-150, TL-424; FR-306..FR-308).
//!
//! A quantified template over an ordered member set expands to ordinary
//! formula nodes, predicates and expressions. Nothing downstream sees a
//! quantifier. This module is the allocation-free core: quantifier shapes,
//! their exact node counts and their construction into caller-owned slices.
//! The owned document, validation and template instantiation are in
//! [`document`] under `alloc`.

use core::fmt;

use crate::{Node, NodeId, NodeKind};

#[cfg(feature = "alloc")]
pub mod document;

/// Maximum members of one set.
pub const MAX_SET_MEMBERS: usize = 1_024;
/// Maximum formula nodes of one expansion (never above the document limit).
pub const MAX_AGGREGATION_NODES: usize = crate::MAX_FORMULA_DOCUMENT_NODES;
/// Maximum predicates of one expansion.
pub const MAX_AGGREGATION_PREDICATES: usize = crate::MAX_PREDICATES;
/// Maximum expression nodes of one expansion (the FR-302 table limit).
pub const MAX_AGGREGATION_EXPRESSIONS: usize = crate::MAX_EXPRESSION_NODES;
/// Maximum quantifier nesting depth.
pub const MAX_AGGREGATION_DEPTH: usize = 2;

const _: () = assert!(MAX_AGGREGATION_NODES <= crate::MAX_FORMULA_DOCUMENT_NODES);

/// Closed v1 quantifier vocabulary (ADR-016 decision 2).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)
)]
pub enum Quantifier {
    /// Conjunction over the members; `true` over an empty set.
    ForEach,
    /// Disjunction over the members; `false` over an empty set.
    ForSome,
    /// At least `n` members; `at_least(0)` is `true`, `n` above the member
    /// count is `false`. `n` is at most [`MAX_SET_MEMBERS`].
    AtLeast {
        /// Required count.
        n: u32,
    },
}

/// One template formula node. Ordinary nodes index the template's own node
/// arena; the slot and inner leaves exist only in templates (FR-306).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)
)]
pub enum TemplateNode {
    /// An ordinary formula node.
    Node {
        /// The node; operands index the template node arena.
        node: Node,
    },
    /// The Boolean proposition the catalog binds to the current member of the
    /// quantifier at `depth` (0 outer, 1 inner).
    SlotProposition {
        /// Quantifier depth.
        depth: u8,
    },
    /// The expansion of the nested quantified template.
    Inner,
}

/// Exact output populations of an expansion.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExpansionCounts {
    /// Formula nodes.
    pub nodes: usize,
    /// Predicates.
    pub predicates: usize,
    /// Expression nodes.
    pub expressions: usize,
    /// Members of the top-level sets, summed.
    pub members: usize,
}

/// Resource named by an [`AggregationRefusal::AggregationTooLarge`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AggregationLimit {
    /// Members of one set, or the `n` of `at_least`.
    Members,
    /// Output formula nodes.
    Nodes,
    /// Output predicates.
    Predicates,
    /// Output expression nodes.
    Expressions,
    /// A count overflowed `usize` or a fresh identity overflowed `u32`.
    Arithmetic,
}

/// Refusal of an expansion. No output is written.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AggregationRefusal {
    /// The expansion would exceed a declared bound.
    AggregationTooLarge {
        /// The limited resource.
        resource: AggregationLimit,
        /// Required population (saturated on overflow).
        actual: usize,
        /// Stable ceiling.
        limit: usize,
    },
    /// Caller-owned output slices are smaller than the exact requirement.
    ExpansionTooSmall {
        /// Exact required populations.
        required: ExpansionCounts,
    },
}

impl fmt::Display for AggregationRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AggregationTooLarge {
                resource,
                actual,
                limit,
            } => write!(
                formatter,
                "aggregation needs {actual} {resource:?}, above the limit {limit}"
            ),
            Self::ExpansionTooSmall { required } => write!(
                formatter,
                "expansion needs {} nodes, {} predicates and {} expression nodes",
                required.nodes, required.predicates, required.expressions
            ),
        }
    }
}

/// Exact nodes a quantifier adds over `members` instance roots (FR-307):
/// `for_each`/`for_some`: `k - 1`, or 1 for `k = 0`; `at_least(0)` and
/// `at_least(n > k)`: 1; otherwise `2 n (k - n + 1) + 2`.
pub fn quantifier_node_count(
    quantifier: Quantifier,
    members: usize,
) -> Result<usize, AggregationRefusal> {
    if members > MAX_SET_MEMBERS {
        return Err(AggregationRefusal::AggregationTooLarge {
            resource: AggregationLimit::Members,
            actual: members,
            limit: MAX_SET_MEMBERS,
        });
    }
    let overflow = AggregationRefusal::AggregationTooLarge {
        resource: AggregationLimit::Arithmetic,
        actual: usize::MAX,
        limit: MAX_AGGREGATION_NODES,
    };
    Ok(match quantifier {
        Quantifier::ForEach | Quantifier::ForSome => members.saturating_sub(1).max(
            // An empty set is one constant node.
            usize::from(members == 0),
        ),
        Quantifier::AtLeast { n } => {
            let n = usize::try_from(n).map_err(|_| overflow)?;
            if n > MAX_SET_MEMBERS {
                return Err(AggregationRefusal::AggregationTooLarge {
                    resource: AggregationLimit::Members,
                    actual: n,
                    limit: MAX_SET_MEMBERS,
                });
            }
            if n == 0 || n > members {
                1
            } else {
                n.checked_mul(members - n + 1)
                    .and_then(|cells| cells.checked_mul(2))
                    .and_then(|nodes| nodes.checked_add(2))
                    .ok_or(overflow)?
            }
        }
    })
}

/// Node writer over a caller slice; identities start at `first`.
struct Writer<'o> {
    out: &'o mut [Node],
    written: usize,
    first: u32,
}

impl Writer<'_> {
    /// Appends a node and returns its identity. The caller sized `out` from
    /// [`quantifier_node_count`], so the slot exists and the id fits.
    fn push(&mut self, kind: NodeKind) -> NodeId {
        self.out[self.written] = Node::new(kind);
        // `written` is below the checked node count, which the caller checked
        // to fit `u32` after `first`.
        let id = NodeId(self.first + self.written as u32);
        self.written += 1;
        id
    }

    fn balanced(&mut self, roots: &[NodeId], conjunction: bool) -> NodeId {
        if roots.len() == 1 {
            return roots[0];
        }
        let split = roots.len().div_ceil(2);
        let left = self.balanced(&roots[..split], conjunction);
        let right = self.balanced(&roots[split..], conjunction);
        self.push(if conjunction {
            NodeKind::And { left, right }
        } else {
            NodeKind::Or { left, right }
        })
    }

    /// The `at_least` counting table. Rows are built from `i = k` down to 1
    /// so every operand precedes its user; within a row `j` increases and each
    /// cell writes its `And` then its `Or`.
    fn at_least(&mut self, roots: &[NodeId], n: usize) -> NodeId {
        let k = roots.len();
        let true_id = self.push(NodeKind::True);
        let false_id = self.push(NodeKind::False);
        // Row i+1's first Or identity and lowest j; row k+1 has no cells.
        let mut next_row: Option<(NodeId, usize)> = None;
        let mut root = false_id;
        for i in (1..=k).rev() {
            let j_low = 1_usize.max((n + 1).saturating_sub(i));
            let j_high = n.min(k - i + 1);
            let mut first_or = None;
            for j in j_low..=j_high {
                let cell = |j: usize| -> NodeId {
                    if j == 0 {
                        return true_id;
                    }
                    if j > k - i {
                        // T(i+1, j) with j > k - (i+1) + 1 is the False constant.
                        return false_id;
                    }
                    match next_row {
                        Some((start, low)) => NodeId(start.0 + 2 * (j - low) as u32),
                        None => false_id,
                    }
                };
                let with = cell(j - 1);
                let without = cell(j);
                let and = self.push(NodeKind::And {
                    left: roots[i - 1],
                    right: with,
                });
                let or = self.push(NodeKind::Or {
                    left: and,
                    right: without,
                });
                if first_or.is_none() {
                    first_or = Some(or);
                }
                root = or;
            }
            next_row = first_or.map(|start| (start, j_low));
        }
        root
    }
}

/// Writes the quantifier over `roots` (already written instance roots) into
/// `out`, numbering new nodes from `first`. Returns the root and the count
/// written. Refuses before writing when `out` is too small or a bound is hit.
pub fn expand_quantifier_into(
    quantifier: Quantifier,
    roots: &[NodeId],
    first: NodeId,
    out: &mut [Node],
) -> Result<(NodeId, usize), AggregationRefusal> {
    let required = quantifier_node_count(quantifier, roots.len())?;
    if required > MAX_AGGREGATION_NODES {
        return Err(AggregationRefusal::AggregationTooLarge {
            resource: AggregationLimit::Nodes,
            actual: required,
            limit: MAX_AGGREGATION_NODES,
        });
    }
    if out.len() < required {
        return Err(AggregationRefusal::ExpansionTooSmall {
            required: ExpansionCounts {
                nodes: required,
                predicates: 0,
                expressions: 0,
                members: roots.len(),
            },
        });
    }
    if usize::try_from(first.0)
        .ok()
        .and_then(|start| start.checked_add(required))
        .is_none_or(|end| end > u32::MAX as usize)
    {
        return Err(AggregationRefusal::AggregationTooLarge {
            resource: AggregationLimit::Arithmetic,
            actual: required,
            limit: MAX_AGGREGATION_NODES,
        });
    }
    let mut writer = Writer {
        out,
        written: 0,
        first: first.0,
    };
    let root = match quantifier {
        Quantifier::ForEach if roots.is_empty() => writer.push(NodeKind::True),
        Quantifier::ForSome if roots.is_empty() => writer.push(NodeKind::False),
        Quantifier::ForEach => writer.balanced(roots, true),
        Quantifier::ForSome => writer.balanced(roots, false),
        Quantifier::AtLeast { n: 0 } => writer.push(NodeKind::True),
        Quantifier::AtLeast { n } if n as usize > roots.len() => writer.push(NodeKind::False),
        Quantifier::AtLeast { n } => writer.at_least(roots, n as usize),
    };
    debug_assert_eq!(writer.written, required);
    Ok((root, writer.written))
}
