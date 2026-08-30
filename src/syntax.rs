use core::fmt;

/// A discrete-time inclusive interval `[start, end]`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    /// Constructs an inclusive interval, rejecting inverted bounds.
    pub const fn new(start: u32, end: u32) -> Result<Self, IntervalError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(IntervalError { start, end })
        }
    }

    /// Returns the inclusive lower bound.
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Returns the inclusive upper bound.
    pub const fn end(self) -> u32 {
        self.end
    }

    /// Returns the number of discrete instants in the interval.
    pub const fn cardinality(self) -> Option<u32> {
        if self.start == 0 && self.end == u32::MAX {
            None
        } else {
            Some(self.end - self.start + 1)
        }
    }

    /// Returns whether the inclusive interval contains `instant`.
    pub const fn contains(self, instant: u32) -> bool {
        self.start <= instant && instant <= self.end
    }
}

/// Error returned when an inclusive interval is inverted.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntervalError {
    /// Rejected lower bound.
    pub start: u32,
    /// Rejected upper bound.
    pub end: u32,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "inclusive interval start {} exceeds end {}",
            self.start, self.end
        )
    }
}

/// A half-open byte span `[start, end)` in an external source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    start: u32,
    end: u32,
}

impl SourceSpan {
    /// Constructs a span, rejecting an end before its start.
    pub const fn new(start: u32, end: u32) -> Result<Self, SourceSpanError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(SourceSpanError { start, end })
        }
    }

    /// Returns the inclusive start byte offset.
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Returns the exclusive end byte offset.
    pub const fn end(self) -> u32 {
        self.end
    }

    /// Returns the span length in bytes.
    pub const fn len(self) -> u32 {
        self.end - self.start
    }

    /// Returns whether the span has zero length.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Error returned when a source span is inverted.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpanError {
    /// Rejected start offset.
    pub start: u32,
    /// Rejected end offset.
    pub end: u32,
}

impl fmt::Display for SourceSpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "source span start {} exceeds end {}",
            self.start, self.end
        )
    }
}

/// Stable numeric identity of a proposition in a proposition map.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PropositionId(pub u32);

/// Stable numeric identity of a node in a formula document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NodeId(pub u32);

impl NodeId {
    fn as_usize(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

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
}

impl SemanticProfile {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClosedTraceV1 => "mltl.closed-trace/v1",
            Self::OnlinePrefixV1 => "mltl.online-prefix/v1",
        }
    }
}

/// One MLTL syntax node. Operands are indices into the containing node table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Node {
    /// Operator and operands.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: NodeKind,
    /// Optional parser-independent source location.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub span: Option<SourceSpan>,
}

impl Node {
    /// Constructs a node without a source span.
    pub const fn new(kind: NodeKind) -> Self {
        Self { kind, span: None }
    }

    /// Associates a checked source span with the node.
    pub const fn with_span(kind: NodeKind, span: SourceSpan) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }
}

/// The complete bounded MLTL operator vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum NodeKind {
    /// Boolean false.
    False,
    /// Boolean true.
    True,
    /// Atomic proposition.
    Proposition {
        /// Stable proposition identity.
        proposition: PropositionId,
    },
    /// Boolean negation.
    Not {
        /// Operand node.
        operand: NodeId,
    },
    /// Boolean conjunction.
    And {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Boolean disjunction.
    Or {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Boolean implication.
    Implies {
        /// Antecedent.
        left: NodeId,
        /// Consequent.
        right: NodeId,
    },
    /// Boolean equivalence.
    Equivalent {
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Bounded Future.
    Future {
        /// Inclusive temporal interval.
        interval: Interval,
        /// Operand node.
        operand: NodeId,
    },
    /// Bounded Globally.
    Globally {
        /// Inclusive temporal interval.
        interval: Interval,
        /// Operand node.
        operand: NodeId,
    },
    /// Bounded Until.
    Until {
        /// Inclusive temporal interval.
        interval: Interval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
    /// Bounded Release.
    Release {
        /// Inclusive temporal interval.
        interval: Interval,
        /// Left operand.
        left: NodeId,
        /// Right operand.
        right: NodeId,
    },
}

impl NodeKind {
    const fn operands(self) -> [Option<NodeId>; 2] {
        match self {
            Self::False | Self::True | Self::Proposition { .. } => [None, None],
            Self::Not { operand }
            | Self::Future { operand, .. }
            | Self::Globally { operand, .. } => [Some(operand), None],
            Self::And { left, right }
            | Self::Or { left, right }
            | Self::Implies { left, right }
            | Self::Equivalent { left, right }
            | Self::Until { left, right, .. }
            | Self::Release { left, right, .. } => [Some(left), Some(right)],
        }
    }
}

/// An allocation-free validated formula view.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Formula<'a> {
    profile: SemanticProfile,
    root: NodeId,
    nodes: &'a [Node],
}

impl<'a> Formula<'a> {
    /// Validates a topologically ordered borrowed node table.
    pub fn new(
        profile: SemanticProfile,
        root: NodeId,
        nodes: &'a [Node],
    ) -> Result<Self, FormulaError> {
        let root_index = match root.as_usize() {
            Some(index) => index,
            None => {
                return Err(FormulaError::RootOutOfRange {
                    root,
                    node_count: nodes.len(),
                });
            }
        };
        if root_index >= nodes.len() {
            return Err(FormulaError::RootOutOfRange {
                root,
                node_count: nodes.len(),
            });
        }
        if u32::try_from(nodes.len() - 1).is_err() {
            return Err(FormulaError::TooManyNodes {
                node_count: nodes.len(),
            });
        }

        for (index, node) in nodes.iter().enumerate() {
            for operand in node.kind.operands().into_iter().flatten() {
                let operand_is_invalid = match operand.as_usize() {
                    Some(operand) => operand >= index,
                    None => true,
                };
                if operand_is_invalid {
                    return Err(FormulaError::OperandNotPreceding {
                        node: NodeId(index as u32),
                        operand,
                    });
                }
            }
        }

        Ok(Self {
            profile,
            root,
            nodes,
        })
    }

    /// Returns the selected semantic profile.
    pub const fn profile(self) -> SemanticProfile {
        self.profile
    }

    /// Returns the root node identity.
    pub const fn root(self) -> NodeId {
        self.root
    }

    /// Returns all nodes in stable topological order.
    pub const fn nodes(self) -> &'a [Node] {
        self.nodes
    }

    /// Looks up a node by identity.
    pub fn node(self, id: NodeId) -> Option<&'a Node> {
        id.as_usize().and_then(|index| self.nodes.get(index))
    }
}

/// Structural validation failure for a formula node table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormulaError {
    /// The selected root is not present in the node table.
    RootOutOfRange {
        /// Rejected root identity.
        root: NodeId,
        /// Number of available nodes.
        node_count: usize,
    },
    /// The node table cannot be addressed by stable 32-bit node identities.
    TooManyNodes {
        /// Number of supplied nodes.
        node_count: usize,
    },
    /// An operand is a self-reference, forward reference, or absent reference.
    OperandNotPreceding {
        /// Node containing the operand.
        node: NodeId,
        /// Rejected operand.
        operand: NodeId,
    },
}

impl fmt::Display for FormulaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootOutOfRange { root, node_count } => write!(
                formatter,
                "formula root {} is outside node table of length {}",
                root.0, node_count
            ),
            Self::TooManyNodes { node_count } => write!(
                formatter,
                "formula node table length {} exceeds the 32-bit identity space",
                node_count
            ),
            Self::OperandNotPreceding { node, operand } => write!(
                formatter,
                "node {} operand {} must identify a preceding node",
                node.0, operand.0
            ),
        }
    }
}

#[cfg(feature = "serde")]
mod serde_checked_values {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    use super::{Interval, SourceSpan};

    #[derive(Deserialize, Serialize)]
    struct Bounds {
        start: u32,
        end: u32,
    }

    impl Serialize for Interval {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Bounds {
                start: self.start(),
                end: self.end(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Interval {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bounds = Bounds::deserialize(deserializer)?;
            Self::new(bounds.start, bounds.end).map_err(D::Error::custom)
        }
    }

    impl Serialize for SourceSpan {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Bounds {
                start: self.start(),
                end: self.end(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for SourceSpan {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bounds = Bounds::deserialize(deserializer)?;
            Self::new(bounds.start, bounds.end).map_err(D::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Trace: TC-001, TC-002, FR-001-AC-1, FR-001-AC-2
    #[test]
    fn intervals_are_inclusive_and_checked() {
        let singleton = Interval::new(4, 4).unwrap();
        assert_eq!(singleton.cardinality(), Some(1));
        assert!(singleton.contains(4));
        assert_eq!(Interval::new(0, u32::MAX).unwrap().cardinality(), None);
        assert_eq!(Interval::new(3, 2), Err(IntervalError { start: 3, end: 2 }));
    }

    // Trace: TC-006, FR-003-AC-1
    #[test]
    fn spans_are_half_open_and_checked() {
        let span = SourceSpan::new(2, 7).unwrap();
        assert_eq!(span.len(), 5);
        assert!(!span.is_empty());
        assert_eq!(
            SourceSpan::new(2, 1),
            Err(SourceSpanError { start: 2, end: 1 })
        );
    }

    // Trace: TC-003, FR-002-AC-1
    #[test]
    fn formula_accepts_complete_operator_vocabulary() {
        let interval = Interval::new(0, 3).unwrap();
        let nodes = [
            Node::new(NodeKind::False),
            Node::new(NodeKind::True),
            Node::new(NodeKind::Proposition {
                proposition: PropositionId(7),
            }),
            Node::new(NodeKind::Not { operand: NodeId(2) }),
            Node::new(NodeKind::And {
                left: NodeId(1),
                right: NodeId(3),
            }),
            Node::new(NodeKind::Or {
                left: NodeId(0),
                right: NodeId(4),
            }),
            Node::new(NodeKind::Implies {
                left: NodeId(4),
                right: NodeId(5),
            }),
            Node::new(NodeKind::Equivalent {
                left: NodeId(5),
                right: NodeId(6),
            }),
            Node::new(NodeKind::Future {
                interval,
                operand: NodeId(7),
            }),
            Node::new(NodeKind::Globally {
                interval,
                operand: NodeId(8),
            }),
            Node::new(NodeKind::Until {
                interval,
                left: NodeId(8),
                right: NodeId(9),
            }),
            Node::new(NodeKind::Release {
                interval,
                left: NodeId(9),
                right: NodeId(10),
            }),
        ];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(11), &nodes).unwrap();
        assert_eq!(formula.nodes(), &nodes);
        assert_eq!(formula.node(formula.root()), Some(&nodes[11]));
    }

    // Trace: TC-015, NFR-001-AC-1, NFR-001-AC-2
    #[test]
    fn allocation_free_core_api_constructs_formula() {
        let nodes = [Node::new(NodeKind::True)];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes).unwrap();
        assert_eq!(formula.nodes(), &nodes);
    }

    // Trace: TC-004, FR-002-AC-2
    #[test]
    fn formula_rejects_invalid_references() {
        assert_eq!(
            Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &[]),
            Err(FormulaError::RootOutOfRange {
                root: NodeId(0),
                node_count: 0
            })
        );
        let nodes = [Node::new(NodeKind::Not { operand: NodeId(0) })];
        assert_eq!(
            Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes),
            Err(FormulaError::OperandNotPreceding {
                node: NodeId(0),
                operand: NodeId(0)
            })
        );
    }

    // Trace: TC-005, TC-007, FR-002-AC-3, FR-003-AC-2, NFR-002-AC-1
    #[test]
    fn identities_profiles_and_nodes_have_stable_order() {
        assert!(PropositionId(1) < PropositionId(2));
        assert!(Node::new(NodeKind::False) < Node::new(NodeKind::True));
        assert_eq!(
            SemanticProfile::ClosedTraceV1.as_str(),
            "mltl.closed-trace/v1"
        );
        assert_eq!(
            SemanticProfile::OnlinePrefixV1.as_str(),
            "mltl.online-prefix/v1"
        );
    }
}
