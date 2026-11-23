//! Exact numeric expressions over catalog signals with static interval typing
//! (FR-302). Allocation-free: typing writes into caller-owned scratch.

use core::fmt;

use crate::{SignalCatalog, SignalDomain, SignalId};

/// Maximum expression nodes in one arena (one predicate table).
pub const MAX_EXPRESSION_NODES: usize = 100_000;
/// Maximum root-to-leaf depth of one expression.
pub const MAX_EXPRESSION_DEPTH: usize = 256;
/// Largest fixed-decimal scale (base-ten fractional digits).
pub const MAX_DECIMAL_SCALE: u8 = 18;

/// Index of an expression node in its arena.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ExprId(pub u32);

impl ExprId {
    pub(crate) fn index(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

/// One closed expression node. Operands precede their user in the arena.
///
/// There is no division, modulo, float, implicit conversion or implicit
/// rescale (ADR-014 decision 2).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)
)]
#[non_exhaustive]
pub enum ExprNode {
    /// Read of an Integer or FixedDecimal catalog signal.
    Signal {
        /// Signal identity.
        signal: SignalId,
    },
    /// Integer literal (scale 0).
    Integer {
        /// Literal value.
        value: i64,
    },
    /// Fixed-decimal literal `coefficient * 10^-scale`.
    Decimal {
        /// Scaled coefficient.
        coefficient: i64,
        /// Fractional digits, at most 18.
        scale: u8,
    },
    /// Negation.
    Neg {
        /// Operand.
        operand: ExprId,
    },
    /// Absolute value.
    Abs {
        /// Operand.
        operand: ExprId,
    },
    /// Sum of equal-scale operands.
    Add {
        /// Left operand.
        left: ExprId,
        /// Right operand.
        right: ExprId,
    },
    /// Difference of equal-scale operands.
    Sub {
        /// Left operand.
        left: ExprId,
        /// Right operand.
        right: ExprId,
    },
    /// Product; the scale is the sum of the operand scales.
    Mul {
        /// Left operand.
        left: ExprId,
        /// Right operand.
        right: ExprId,
    },
    /// Explicit widening to a larger scale.
    Rescale {
        /// Operand.
        operand: ExprId,
        /// Target scale, at least the operand's and at most 18.
        scale: u8,
    },
    /// Template-only leaf standing for the current member signal of the
    /// quantifier at `depth` (FR-306). Refused in a predicate table.
    SlotSignal {
        /// Quantifier depth: 0 outer, 1 inner.
        depth: u8,
    },
}

impl ExprNode {
    /// Returns the operands of this node.
    pub const fn operands(self) -> [Option<ExprId>; 2] {
        match self {
            Self::Signal { .. }
            | Self::Integer { .. }
            | Self::Decimal { .. }
            | Self::SlotSignal { .. } => [None, None],
            Self::Neg { operand } | Self::Abs { operand } | Self::Rescale { operand, .. } => {
                [Some(operand), None]
            }
            Self::Add { left, right } | Self::Sub { left, right } | Self::Mul { left, right } => {
                [Some(left), Some(right)]
            }
        }
    }
}

/// Derived exact type: a scale and the inclusive coefficient range.
/// An Integer is scale 0.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExprType {
    /// Fractional digits.
    pub scale: u8,
    /// Inclusive minimum coefficient.
    pub minimum: i64,
    /// Inclusive maximum coefficient.
    pub maximum: i64,
}

impl ExprType {
    /// The type a catalog domain gives a signal read, or `None` for Boolean.
    pub const fn of_domain(domain: SignalDomain) -> Option<Self> {
        match domain {
            SignalDomain::Boolean => None,
            SignalDomain::Integer(integer) => Some(Self {
                scale: 0,
                minimum: integer.minimum(),
                maximum: integer.maximum(),
            }),
            SignalDomain::FixedDecimal(decimal) => Some(Self {
                scale: decimal.scale(),
                minimum: decimal.minimum_coefficient(),
                maximum: decimal.maximum_coefficient(),
            }),
        }
    }
}

/// One caller-owned typing scratch slot: the derived type and the node depth.
///
/// SPEC-GAP: FR-302-CON-2 names "one `ExprType` slot per node"; the depth
/// limit needs the depth too, so a slot carries both.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExprSlot {
    ty: ExprType,
    depth: u16,
}

impl ExprSlot {
    /// An unused slot.
    pub const EMPTY: Self = Self {
        ty: ExprType {
            scale: 0,
            minimum: 0,
            maximum: 0,
        },
        depth: 0,
    };

    /// Returns the derived type written by validation.
    pub const fn ty(self) -> ExprType {
        self.ty
    }

    /// Returns the root-to-leaf depth written by validation.
    pub const fn depth(self) -> u16 {
        self.depth
    }
}

/// Resource named by [`ExpressionError::ExpressionLimitExceeded`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExpressionLimit {
    /// Expression nodes per arena.
    Nodes,
    /// Expression depth.
    Depth,
}

/// Typing or structural refusal of an expression arena. No view is returned.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ExpressionError {
    /// A derived bound does not fit `i64` at this node.
    RangeOverflow {
        /// First overflowing node.
        node: ExprId,
    },
    /// `add`, `sub` or a comparison has operands of different scale.
    ScaleMismatch {
        /// Offending node.
        node: ExprId,
        /// Left operand scale.
        left: u8,
        /// Right operand scale.
        right: u8,
    },
    /// A literal, product or rescale exceeds scale 18.
    ScaleOverflow {
        /// Offending node.
        node: ExprId,
        /// The scale that would result.
        scale: u16,
    },
    /// `rescale` to a smaller scale.
    RescaleNarrowing {
        /// Offending node.
        node: ExprId,
        /// Operand scale.
        from: u8,
        /// Requested scale.
        to: u8,
    },
    /// A signal read targets a Boolean signal.
    NonNumericSignal {
        /// Offending node.
        node: ExprId,
        /// The Boolean signal.
        signal: SignalId,
    },
    /// A signal read targets no declared signal.
    UnknownSignal {
        /// Offending node.
        node: ExprId,
        /// The absent signal.
        signal: SignalId,
    },
    /// A node or depth limit is exceeded.
    ExpressionLimitExceeded {
        /// The limited resource.
        resource: ExpressionLimit,
        /// Observed population or depth.
        actual: usize,
        /// Stable ceiling.
        limit: usize,
    },
    /// An operand does not identify a preceding node.
    OperandNotPreceding {
        /// Node containing the operand.
        node: ExprId,
        /// Rejected operand.
        operand: ExprId,
    },
    /// Caller-owned scratch has fewer slots than nodes.
    ScratchTooSmall {
        /// Provided slots.
        provided: usize,
        /// Required slots.
        required: usize,
    },
    /// A template-only slot leaf appears outside an aggregation template.
    SlotOutsideTemplate {
        /// Offending node.
        node: ExprId,
    },
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RangeOverflow { node } => {
                write!(formatter, "expression {} range does not fit i64", node.0)
            }
            Self::ScaleMismatch { node, left, right } => write!(
                formatter,
                "expression {} operands have scales {left} and {right}",
                node.0
            ),
            Self::ScaleOverflow { node, scale } => write!(
                formatter,
                "expression {} has scale {scale}, above {MAX_DECIMAL_SCALE}",
                node.0
            ),
            Self::RescaleNarrowing { node, from, to } => write!(
                formatter,
                "expression {} rescales from {from} down to {to}",
                node.0
            ),
            Self::NonNumericSignal { node, signal } => write!(
                formatter,
                "expression {} reads Boolean signal {}",
                node.0, signal.0
            ),
            Self::UnknownSignal { node, signal } => write!(
                formatter,
                "expression {} reads undeclared signal {}",
                node.0, signal.0
            ),
            Self::ExpressionLimitExceeded {
                resource,
                actual,
                limit,
            } => write!(
                formatter,
                "expression {resource:?} {actual} exceeds the limit {limit}"
            ),
            Self::OperandNotPreceding { node, operand } => write!(
                formatter,
                "expression {} operand {} must identify a preceding node",
                node.0, operand.0
            ),
            Self::ScratchTooSmall { provided, required } => write!(
                formatter,
                "expression scratch has {provided} slots but {required} are required"
            ),
            Self::SlotOutsideTemplate { node } => write!(
                formatter,
                "expression {} is a template slot outside an aggregation template",
                node.0
            ),
        }
    }
}

/// Converts an `i128` bound to `i64` or reports `RangeOverflow` at `node`.
fn fit(value: i128, node: ExprId) -> Result<i64, ExpressionError> {
    i64::try_from(value).map_err(|_| ExpressionError::RangeOverflow { node })
}

fn range(node: ExprId, scale: u8, low: i128, high: i128) -> Result<ExprType, ExpressionError> {
    Ok(ExprType {
        scale,
        minimum: fit(low, node)?,
        maximum: fit(high, node)?,
    })
}

/// `10^exponent` for `exponent <= 18`, exact in `i128`.
fn power_of_ten(exponent: u8) -> i128 {
    let mut value = 1_i128;
    let mut remaining = exponent;
    while remaining > 0 {
        value *= 10;
        remaining -= 1;
    }
    value
}

/// Types one node from its already typed operands. `member` resolves a
/// `SlotSignal` leaf; it is `None` outside a template.
pub(crate) fn type_node(
    node_id: ExprId,
    node: ExprNode,
    operand_type: impl Fn(ExprId) -> ExprType,
    signal_type: impl Fn(ExprId, SignalId) -> Result<ExprType, ExpressionError>,
    member: Option<&dyn Fn(ExprId, u8) -> Result<ExprType, ExpressionError>>,
) -> Result<ExprType, ExpressionError> {
    match node {
        ExprNode::Signal { signal } => signal_type(node_id, signal),
        ExprNode::SlotSignal { depth } => match member {
            Some(member) => member(node_id, depth),
            None => Err(ExpressionError::SlotOutsideTemplate { node: node_id }),
        },
        ExprNode::Integer { value } => Ok(ExprType {
            scale: 0,
            minimum: value,
            maximum: value,
        }),
        ExprNode::Decimal { coefficient, scale } => {
            if scale > MAX_DECIMAL_SCALE {
                return Err(ExpressionError::ScaleOverflow {
                    node: node_id,
                    scale: u16::from(scale),
                });
            }
            Ok(ExprType {
                scale,
                minimum: coefficient,
                maximum: coefficient,
            })
        }
        ExprNode::Neg { operand } => {
            let operand = operand_type(operand);
            range(
                node_id,
                operand.scale,
                -i128::from(operand.maximum),
                -i128::from(operand.minimum),
            )
        }
        ExprNode::Abs { operand } => {
            let operand = operand_type(operand);
            let low = i128::from(operand.minimum);
            let high = i128::from(operand.maximum);
            let upper = low.abs().max(high.abs());
            let lower = if low <= 0 && 0 <= high {
                0
            } else {
                low.abs().min(high.abs())
            };
            range(node_id, operand.scale, lower, upper)
        }
        ExprNode::Add { left, right } | ExprNode::Sub { left, right } => {
            let left_type = operand_type(left);
            let right_type = operand_type(right);
            if left_type.scale != right_type.scale {
                return Err(ExpressionError::ScaleMismatch {
                    node: node_id,
                    left: left_type.scale,
                    right: right_type.scale,
                });
            }
            let (low, high) = if matches!(node, ExprNode::Add { .. }) {
                (
                    i128::from(left_type.minimum) + i128::from(right_type.minimum),
                    i128::from(left_type.maximum) + i128::from(right_type.maximum),
                )
            } else {
                (
                    i128::from(left_type.minimum) - i128::from(right_type.maximum),
                    i128::from(left_type.maximum) - i128::from(right_type.minimum),
                )
            };
            range(node_id, left_type.scale, low, high)
        }
        ExprNode::Mul { left, right } => {
            let left_type = operand_type(left);
            let right_type = operand_type(right);
            let scale = u16::from(left_type.scale) + u16::from(right_type.scale);
            if scale > u16::from(MAX_DECIMAL_SCALE) {
                return Err(ExpressionError::ScaleOverflow {
                    node: node_id,
                    scale,
                });
            }
            let corners = [
                i128::from(left_type.minimum) * i128::from(right_type.minimum),
                i128::from(left_type.minimum) * i128::from(right_type.maximum),
                i128::from(left_type.maximum) * i128::from(right_type.minimum),
                i128::from(left_type.maximum) * i128::from(right_type.maximum),
            ];
            let mut low = corners[0];
            let mut high = corners[0];
            for corner in corners {
                low = low.min(corner);
                high = high.max(corner);
            }
            // `scale` is at most 18 here, so the narrowing is lossless.
            range(node_id, scale as u8, low, high)
        }
        ExprNode::Rescale { operand, scale } => {
            let operand = operand_type(operand);
            if scale > MAX_DECIMAL_SCALE {
                return Err(ExpressionError::ScaleOverflow {
                    node: node_id,
                    scale: u16::from(scale),
                });
            }
            if scale < operand.scale {
                return Err(ExpressionError::RescaleNarrowing {
                    node: node_id,
                    from: operand.scale,
                    to: scale,
                });
            }
            let factor = power_of_ten(scale - operand.scale);
            range(
                node_id,
                scale,
                i128::from(operand.minimum) * factor,
                i128::from(operand.maximum) * factor,
            )
        }
    }
}

/// Types a catalog signal read.
pub(crate) fn catalog_signal_type(
    catalog: SignalCatalog<'_>,
    node: ExprId,
    signal: SignalId,
) -> Result<ExprType, ExpressionError> {
    let declaration = catalog
        .signal(signal)
        .ok_or(ExpressionError::UnknownSignal { node, signal })?;
    ExprType::of_domain(declaration.domain())
        .ok_or(ExpressionError::NonNumericSignal { node, signal })
}

/// Checks operand order and depth, and types every node in arena order.
///
/// `member` resolves `SlotSignal` leaves (templates only).
pub(crate) fn type_arena(
    catalog: SignalCatalog<'_>,
    nodes: &[ExprNode],
    scratch: &mut [ExprSlot],
    member: Option<&dyn Fn(ExprId, u8) -> Result<ExprType, ExpressionError>>,
) -> Result<(), ExpressionError> {
    if nodes.len() > MAX_EXPRESSION_NODES {
        return Err(ExpressionError::ExpressionLimitExceeded {
            resource: ExpressionLimit::Nodes,
            actual: nodes.len(),
            limit: MAX_EXPRESSION_NODES,
        });
    }
    if scratch.len() < nodes.len() {
        return Err(ExpressionError::ScratchTooSmall {
            provided: scratch.len(),
            required: nodes.len(),
        });
    }
    for (index, node) in nodes.iter().copied().enumerate() {
        // `index` is below MAX_EXPRESSION_NODES, which fits `u32`.
        let node_id = ExprId(index as u32);
        let mut depth = 1_usize;
        for operand in node.operands().into_iter().flatten() {
            let preceding = operand.index().filter(|position| *position < index);
            let Some(position) = preceding else {
                return Err(ExpressionError::OperandNotPreceding {
                    node: node_id,
                    operand,
                });
            };
            depth = depth.max(usize::from(scratch[position].depth) + 1);
        }
        if depth > MAX_EXPRESSION_DEPTH {
            return Err(ExpressionError::ExpressionLimitExceeded {
                resource: ExpressionLimit::Depth,
                actual: depth,
                limit: MAX_EXPRESSION_DEPTH,
            });
        }
        let ty = {
            let typed: &[ExprSlot] = scratch;
            type_node(
                node_id,
                node,
                |operand| {
                    operand
                        .index()
                        .and_then(|position| typed.get(position))
                        .map(|slot| slot.ty)
                        .unwrap_or_default()
                },
                |id, signal| catalog_signal_type(catalog, id, signal),
                member,
            )?
        };
        // `depth` is at most MAX_EXPRESSION_DEPTH, which fits `u16`.
        scratch[index] = ExprSlot {
            ty,
            depth: depth as u16,
        };
    }
    Ok(())
}

const _: () = assert!(MAX_EXPRESSION_NODES <= u32::MAX as usize);
const _: () = assert!(MAX_EXPRESSION_DEPTH <= u16::MAX as usize);

/// A validated, allocation-free expression arena. Its derived types live in
/// the caller's scratch slots, indexed like the nodes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpressionArena<'a> {
    nodes: &'a [ExprNode],
}

impl<'a> ExpressionArena<'a> {
    /// Validates and types every node against `catalog`, writing one slot per
    /// node into `scratch`. Template slot leaves are refused.
    pub fn new(
        catalog: SignalCatalog<'_>,
        nodes: &'a [ExprNode],
        scratch: &mut [ExprSlot],
    ) -> Result<Self, ExpressionError> {
        type_arena(catalog, nodes, scratch, None)?;
        Ok(Self { nodes })
    }

    /// Returns the nodes in arena order.
    pub const fn nodes(self) -> &'a [ExprNode] {
        self.nodes
    }

    /// Looks up one node.
    pub fn node(self, id: ExprId) -> Option<ExprNode> {
        id.index().and_then(|index| self.nodes.get(index)).copied()
    }

    /// Returns the node count.
    pub const fn len(self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the arena has no node.
    pub const fn is_empty(self) -> bool {
        self.nodes.is_empty()
    }
}
