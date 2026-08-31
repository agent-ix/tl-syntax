use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::fmt;

use crate::{Formula, FormulaError, Node, NodeId, PropositionId, SemanticProfile};

/// Maximum node count accepted by the v1 JSON wire decoder.
///
/// This bounds allocation for both wire decoding and programmatic construction.
pub const MAX_FORMULA_DOCUMENT_NODES: usize = 100_000;

/// Version of the serialized formula document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FormulaSchemaVersion {
    /// Initial tl-syntax formula schema.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.formula/v1"))]
    V1,
}

impl FormulaSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "tl-syntax.formula/v1",
        }
    }
}

/// Owned, versioned formula exchange document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "FormulaDocumentWire"))]
pub struct FormulaDocument {
    /// Wire schema identity.
    schema_version: FormulaSchemaVersion,
    /// Required finite-trace semantic profile.
    semantic_profile: SemanticProfile,
    /// Root node identity.
    root: NodeId,
    /// Nodes in stable topological order.
    nodes: Vec<Node>,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FormulaDocumentWire {
    schema_version: FormulaSchemaVersion,
    semantic_profile: SemanticProfile,
    root: NodeId,
    #[serde(deserialize_with = "deserialize_formula_nodes")]
    nodes: Vec<Node>,
}

#[cfg(feature = "serde")]
fn deserialize_formula_nodes<'de, D>(deserializer: D) -> Result<Vec<Node>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct NodeVisitor;

    impl<'de> serde::de::Visitor<'de> for NodeVisitor {
        type Value = Vec<Node>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                formatter,
                "at most {MAX_FORMULA_DOCUMENT_NODES} formula nodes"
            )
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut nodes = Vec::with_capacity(
                sequence
                    .size_hint()
                    .unwrap_or(0)
                    .min(MAX_FORMULA_DOCUMENT_NODES),
            );
            while let Some(node) = sequence.next_element()? {
                if nodes.len() == MAX_FORMULA_DOCUMENT_NODES {
                    return Err(serde::de::Error::custom(format_args!(
                        "formula document exceeds the {MAX_FORMULA_DOCUMENT_NODES}-node wire limit"
                    )));
                }
                nodes.push(node);
            }
            Ok(nodes)
        }
    }

    deserializer.deserialize_seq(NodeVisitor)
}

#[cfg(feature = "serde")]
impl TryFrom<FormulaDocumentWire> for FormulaDocument {
    type Error = FormulaError;

    fn try_from(wire: FormulaDocumentWire) -> Result<Self, Self::Error> {
        let document = Self {
            schema_version: wire.schema_version,
            semantic_profile: wire.semantic_profile,
            root: wire.root,
            nodes: wire.nodes,
        };
        document.validate()?;
        Ok(document)
    }
}

impl FormulaDocument {
    /// Constructs and validates a v1 document.
    pub fn new(
        semantic_profile: SemanticProfile,
        root: NodeId,
        nodes: Vec<Node>,
    ) -> Result<Self, FormulaError> {
        if nodes.len() > MAX_FORMULA_DOCUMENT_NODES {
            return Err(FormulaError::DocumentNodeLimitExceeded {
                node_count: nodes.len(),
                limit: MAX_FORMULA_DOCUMENT_NODES,
            });
        }
        let document = Self {
            schema_version: FormulaSchemaVersion::V1,
            semantic_profile,
            root,
            nodes,
        };
        document.validate()?;
        Ok(document)
    }

    /// Validates this document and returns its allocation-free view.
    pub fn validate(&self) -> Result<Formula<'_>, FormulaError> {
        Formula::new(self.semantic_profile, self.root, &self.nodes)
    }

    /// Returns the wire schema version.
    pub const fn schema_version(&self) -> FormulaSchemaVersion {
        self.schema_version
    }

    /// Returns the required finite-trace semantic profile.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        self.semantic_profile
    }

    /// Returns the root node identity.
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// Returns the nodes in stable topological order.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Copies a validated borrowed formula into a bounded owned v1 document.
    pub fn from_formula(formula: Formula<'_>) -> Result<Self, FormulaError> {
        Self::new(formula.profile(), formula.root(), formula.nodes().to_vec())
    }
}

/// Version of the serialized proposition-map document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PropositionMapSchemaVersion {
    /// Initial tl-syntax proposition-map schema.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.proposition-map/v1"))]
    V1,
}

impl PropositionMapSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "tl-syntax.proposition-map/v1",
        }
    }
}

/// One proposition identity-to-name mapping.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PropositionEntry {
    /// Stable proposition identity referenced by formula nodes.
    pub id: PropositionId,
    /// Application-defined proposition name.
    pub name: String,
}

/// Owned, versioned proposition-map exchange document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "PropositionMapDocumentWire"))]
pub struct PropositionMapDocument {
    /// Wire schema identity.
    schema_version: PropositionMapSchemaVersion,
    /// Entries ordered by strictly increasing proposition identity.
    propositions: Vec<PropositionEntry>,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PropositionMapDocumentWire {
    schema_version: PropositionMapSchemaVersion,
    propositions: Vec<PropositionEntry>,
}

#[cfg(feature = "serde")]
impl TryFrom<PropositionMapDocumentWire> for PropositionMapDocument {
    type Error = PropositionMapError;

    fn try_from(wire: PropositionMapDocumentWire) -> Result<Self, Self::Error> {
        let document = Self {
            schema_version: wire.schema_version,
            propositions: wire.propositions,
        };
        document.validate()?;
        Ok(document)
    }
}

impl PropositionMapDocument {
    /// Constructs and validates a v1 proposition map.
    pub fn new(propositions: Vec<PropositionEntry>) -> Result<Self, PropositionMapError> {
        let document = Self {
            schema_version: PropositionMapSchemaVersion::V1,
            propositions,
        };
        document.validate()?;
        Ok(document)
    }

    /// Checks identity ordering, uniqueness, and non-empty unique names.
    pub fn validate(&self) -> Result<(), PropositionMapError> {
        let mut names = BTreeMap::new();
        for (index, entry) in self.propositions.iter().enumerate() {
            if entry.name.is_empty() {
                return Err(PropositionMapError::EmptyName { id: entry.id });
            }
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|previous| self.propositions.get(previous))
            {
                if previous.id >= entry.id {
                    return Err(PropositionMapError::IdentityNotIncreasing {
                        previous: previous.id,
                        current: entry.id,
                    });
                }
            }
            if let Some(first) = names.insert(entry.name.as_str(), entry.id) {
                return Err(PropositionMapError::DuplicateName {
                    first,
                    second: entry.id,
                });
            }
        }
        Ok(())
    }

    /// Returns the wire schema version.
    pub const fn schema_version(&self) -> PropositionMapSchemaVersion {
        self.schema_version
    }

    /// Returns proposition entries in strictly increasing identity order.
    pub fn propositions(&self) -> &[PropositionEntry] {
        &self.propositions
    }
}

/// Validation failure for a proposition-map document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PropositionMapError {
    /// A proposition name is empty.
    EmptyName {
        /// Identity associated with the empty name.
        id: PropositionId,
    },
    /// Proposition identities are duplicated or not strictly increasing.
    IdentityNotIncreasing {
        /// Previous identity.
        previous: PropositionId,
        /// Current rejected identity.
        current: PropositionId,
    },
    /// Two identities use the same proposition name.
    DuplicateName {
        /// First identity using the name.
        first: PropositionId,
        /// Second identity using the name.
        second: PropositionId,
    },
}

impl fmt::Display for PropositionMapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName { id } => write!(formatter, "proposition {} has an empty name", id.0),
            Self::IdentityNotIncreasing { previous, current } => write!(
                formatter,
                "proposition identity {} does not follow {}",
                current.0, previous.0
            ),
            Self::DuplicateName { first, second } => write!(
                formatter,
                "propositions {} and {} have the same name",
                first.0, second.0
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec};

    use super::*;
    use crate::NodeKind;

    // Trace: TC-011, FR-004-AC-3
    #[test]
    fn proposition_maps_are_unambiguous() {
        let valid = PropositionMapDocument::new(vec![
            PropositionEntry {
                id: PropositionId(1),
                name: "request".to_string(),
            },
            PropositionEntry {
                id: PropositionId(2),
                name: "response".to_string(),
            },
        ])
        .unwrap();
        assert_eq!(valid.propositions().len(), 2);

        let duplicate = PropositionMapDocument::new(vec![
            PropositionEntry {
                id: PropositionId(1),
                name: "same".to_string(),
            },
            PropositionEntry {
                id: PropositionId(2),
                name: "same".to_string(),
            },
        ]);
        assert_eq!(
            duplicate,
            Err(PropositionMapError::DuplicateName {
                first: PropositionId(1),
                second: PropositionId(2)
            })
        );

        assert_eq!(
            PropositionMapDocument::new(vec![PropositionEntry {
                id: PropositionId(1),
                name: String::new(),
            }]),
            Err(PropositionMapError::EmptyName {
                id: PropositionId(1)
            })
        );
        assert_eq!(
            PropositionMapDocument::new(vec![
                PropositionEntry {
                    id: PropositionId(2),
                    name: "later".to_string(),
                },
                PropositionEntry {
                    id: PropositionId(1),
                    name: "earlier".to_string(),
                },
            ]),
            Err(PropositionMapError::IdentityNotIncreasing {
                previous: PropositionId(2),
                current: PropositionId(1)
            })
        );
    }

    // Trace: TC-009, FR-004-AC-1
    #[test]
    fn owned_document_preserves_borrowed_formula() {
        let nodes = vec![Node::new(NodeKind::True)];
        let document =
            FormulaDocument::new(SemanticProfile::OnlinePrefixV1, NodeId(0), nodes.clone())
                .unwrap();
        let formula = document.validate().unwrap();
        assert_eq!(formula.profile(), SemanticProfile::OnlinePrefixV1);
        assert_eq!(formula.nodes(), nodes);
        let copied = FormulaDocument::from_formula(formula).unwrap();
        assert_eq!(copied.schema_version(), FormulaSchemaVersion::V1);
        assert_eq!(copied.semantic_profile(), SemanticProfile::OnlinePrefixV1);
        assert_eq!(copied.root(), NodeId(0));
        assert_eq!(copied.nodes(), nodes);
    }
}
