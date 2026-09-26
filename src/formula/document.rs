#[cfg(feature = "serde")]
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{array_field_population, read_strict_document, StrictDocument},
    },
    SyntaxArtifactLimits,
};
use crate::{
    Formula, FormulaError, Node, NodeId, PastOperatorKind, SemanticProfile,
    MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES,
};

#[cfg(feature = "serde")]
pub use crate::contracts::reader::{
    StrictDocumentReadError, MAX_TL_DOCUMENT_BYTES, MAX_TL_DOCUMENT_DEPTH,
};
/// Exact checked-in Draft 7 schema text for `tl-syntax.formula/v1`.
#[cfg(feature = "serde")]
pub const FORMULA_V1_SCHEMA: &str = include_str!("../../corpus/schema/formula-v1.schema.json");

/// Exact checked-in Draft 7 schema text for `tl-syntax.formula/v2`.
#[cfg(feature = "serde")]
pub const FORMULA_V2_SCHEMA: &str = include_str!("../../corpus/schema/formula-v2.schema.json");

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.formula/v1`.
#[cfg(feature = "serde")]
pub const FORMULA_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../corpus/schema/formula-v1.schema.json");
/// Lowercase SHA-256 of [`FORMULA_V1_SCHEMA_BYTES`].
#[cfg(feature = "serde")]
pub const FORMULA_V1_SCHEMA_SHA256: &str =
    "780c70055eb979a7913dc224e752d56f057793a0ef60af0e7270f53a4f973393";

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.formula/v2`.
#[cfg(feature = "serde")]
pub const FORMULA_V2_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../corpus/schema/formula-v2.schema.json");
/// Lowercase SHA-256 of [`FORMULA_V2_SCHEMA_BYTES`].
#[cfg(feature = "serde")]
pub const FORMULA_V2_SCHEMA_SHA256: &str =
    "a78889a4ed04dabd271bfd68953a8ddf642b14e8609d2268421b23fa4540c7c4";

/// Version of the serialized formula document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FormulaSchemaVersion {
    /// Initial tl-syntax formula schema.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.formula/v1"))]
    V1,
    /// Additive schema admitting the closed past-time node vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.formula/v2"))]
    V2,
}

impl FormulaSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "tl-syntax.formula/v1",
            Self::V2 => "tl-syntax.formula/v2",
        }
    }
}

/// Refusal returned by guarded formula-schema conversion.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FormulaConversionError {
    /// Formula-v1 cannot represent this semantic profile.
    UnsupportedSemanticProfile {
        /// Rejected profile.
        profile: SemanticProfile,
    },
    /// Formula-v1 cannot represent this past-time node.
    PastNodeUnsupported {
        /// First rejected node in topological order.
        node: NodeId,
        /// Past operator found there.
        operator: PastOperatorKind,
    },
    /// A node position cannot be represented by the stable `NodeId` type.
    NodeIdentityOutOfRange {
        /// Number of nodes in the document.
        node_count: usize,
    },
}

impl fmt::Display for FormulaConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSemanticProfile { profile } => write!(
                formatter,
                "formula-v1 cannot represent semantic profile {}",
                profile.as_str()
            ),
            Self::PastNodeUnsupported { node, operator } => write!(
                formatter,
                "formula-v1 cannot represent {} at node {}",
                operator.semantic_name(),
                node.0
            ),
            Self::NodeIdentityOutOfRange { node_count } => write!(
                formatter,
                "formula node table length {node_count} exceeds the NodeId range"
            ),
        }
    }
}

/// Owned, versioned formula exchange document.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

/// A span-free, semantic serialization view of a formula document.
///
/// This view preserves the stable document fields and topological node order,
/// but serializes each node's operator and operands without its diagnostic
/// source span. Use it for semantic cache keys, replay identities, and other
/// content-addressed operations. The `FormulaDocument` wire form remains the
/// diagnostic exchange form and retains spans when they are available.
#[derive(Debug)]
pub struct SemanticFormulaDocument<'a> {
    document: &'a FormulaDocument,
}

impl PartialEq for SemanticFormulaDocument<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.document.schema_version == other.document.schema_version
            && self.document.semantic_profile == other.document.semantic_profile
            && self.document.root == other.document.root
            && self.document.nodes.iter().map(|node| node.kind).eq(other
                .document
                .nodes
                .iter()
                .map(|node| node.kind))
    }
}

impl Eq for SemanticFormulaDocument<'_> {}

impl Hash for SemanticFormulaDocument<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.document.schema_version.hash(state);
        self.document.semantic_profile.hash(state);
        self.document.root.hash(state);
        self.document.nodes.len().hash(state);
        for node in &self.document.nodes {
            node.kind.hash(state);
        }
    }
}

impl Ord for SemanticFormulaDocument<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.document.schema_version,
            self.document.semantic_profile,
            self.document.root,
        )
            .cmp(&(
                other.document.schema_version,
                other.document.semantic_profile,
                other.document.root,
            ))
            .then_with(|| {
                self.document
                    .nodes
                    .iter()
                    .map(|node| node.kind)
                    .cmp(other.document.nodes.iter().map(|node| node.kind))
            })
    }
}

impl PartialOrd for SemanticFormulaDocument<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SemanticFormulaDocument<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeSeq, SerializeStruct};

        struct SemanticNodes<'a>(&'a [Node]);

        impl serde::Serialize for SemanticNodes<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
                for node in self.0 {
                    sequence.serialize_element(&node.kind)?;
                }
                sequence.end()
            }
        }

        let mut document = serializer.serialize_struct("FormulaDocument", 4)?;
        document.serialize_field("schema_version", &self.document.schema_version)?;
        document.serialize_field("semantic_profile", &self.document.semantic_profile)?;
        document.serialize_field("root", &self.document.root)?;
        document.serialize_field("nodes", &SemanticNodes(&self.document.nodes))?;
        document.end()
    }
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
        Self::construct(FormulaSchemaVersion::V1, semantic_profile, root, nodes)
    }

    /// Constructs and validates a v2 document.
    pub fn new_v2(
        semantic_profile: SemanticProfile,
        root: NodeId,
        nodes: Vec<Node>,
    ) -> Result<Self, FormulaError> {
        Self::construct(FormulaSchemaVersion::V2, semantic_profile, root, nodes)
    }

    fn construct(
        schema_version: FormulaSchemaVersion,
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
            schema_version,
            semantic_profile,
            root,
            nodes,
        };
        document.validate()?;
        Ok(document)
    }

    /// Reads exactly one bounded canonical formula document through the owner type.
    #[cfg(feature = "serde")]
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<Self, StrictDocumentReadError> {
        read_strict_document(bytes, limits)
    }

    /// Serializes the validated document to its one canonical owner encoding.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }

    /// Returns the domain-separated lowercase SHA-256 content identity.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        let bytes = self.canonical_json_bytes()?;
        Ok(crate::contracts::identity::content_identity(
            self.schema_version.as_str(),
            &bytes,
        ))
    }

    /// Validates this document and returns its allocation-free view.
    pub fn validate(&self) -> Result<Formula<'_>, FormulaError> {
        self.validate_schema_compatibility()?;
        let formula = Formula::new(self.semantic_profile, self.root, &self.nodes)?;
        if self.schema_version == FormulaSchemaVersion::V2 {
            self.validate_depth()?;
        }
        Ok(formula)
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

    /// Returns the canonical semantic view, excluding diagnostic source spans.
    pub const fn semantic_view(&self) -> SemanticFormulaDocument<'_> {
        SemanticFormulaDocument { document: self }
    }

    /// Copies a validated borrowed formula into a bounded owned v1 document.
    pub fn from_formula(formula: Formula<'_>) -> Result<Self, FormulaError> {
        Self::new(formula.profile(), formula.root(), formula.nodes().to_vec())
    }

    /// Copies a validated borrowed formula into a bounded owned v2 document.
    pub fn from_formula_v2(formula: Formula<'_>) -> Result<Self, FormulaError> {
        Self::new_v2(formula.profile(), formula.root(), formula.nodes().to_vec())
    }

    /// Losslessly upgrades this document to formula-v2.
    pub fn to_v2(&self) -> Self {
        Self {
            schema_version: FormulaSchemaVersion::V2,
            semantic_profile: self.semantic_profile,
            root: self.root,
            nodes: self.nodes.clone(),
        }
    }

    /// Down-converts to formula-v1 only when the profile and nodes are v1-compatible.
    pub fn try_to_v1(&self) -> Result<Self, FormulaConversionError> {
        if matches!(
            self.semantic_profile,
            SemanticProfile::OriginCompleteHistoryV1 | SemanticProfile::InfiniteTraceV1
        ) {
            return Err(FormulaConversionError::UnsupportedSemanticProfile {
                profile: self.semantic_profile,
            });
        }
        if let Some((index, operator)) =
            self.nodes.iter().enumerate().find_map(|(index, node)| {
                node.kind.past_operator().map(|operator| (index, operator))
            })
        {
            let node = u32::try_from(index).map(NodeId).map_err(|_| {
                FormulaConversionError::NodeIdentityOutOfRange {
                    node_count: self.nodes.len(),
                }
            })?;
            return Err(FormulaConversionError::PastNodeUnsupported { node, operator });
        }
        Ok(Self {
            schema_version: FormulaSchemaVersion::V1,
            semantic_profile: self.semantic_profile,
            root: self.root,
            nodes: self.nodes.clone(),
        })
    }

    fn validate_schema_compatibility(&self) -> Result<(), FormulaError> {
        if self.semantic_profile == SemanticProfile::InfiniteTraceV1 {
            return Err(FormulaError::InfiniteProfileRequiresUnboundedEdition);
        }
        if self.schema_version == FormulaSchemaVersion::V2 {
            return Ok(());
        }
        if matches!(
            self.semantic_profile,
            SemanticProfile::OriginCompleteHistoryV1 | SemanticProfile::InfiniteTraceV1
        ) {
            return Err(FormulaError::FormulaV1ProfileUnsupported {
                profile: self.semantic_profile,
            });
        }
        for (index, node) in self.nodes.iter().enumerate() {
            if let Some(operator) = node.kind.past_operator() {
                let node =
                    u32::try_from(index)
                        .map(NodeId)
                        .map_err(|_| FormulaError::TooManyNodes {
                            node_count: self.nodes.len(),
                        })?;
                return Err(FormulaError::FormulaV1NodeUnsupported { node, operator });
            }
        }
        Ok(())
    }

    fn validate_depth(&self) -> Result<(), FormulaError> {
        let mut depths = Vec::with_capacity(self.nodes.len());
        for (index, node) in self.nodes.iter().enumerate() {
            let mut depth = 1_usize;
            for operand in node.kind.operands().into_iter().flatten() {
                let operand_depth = usize::try_from(operand.0)
                    .ok()
                    .and_then(|operand| depths.get(operand))
                    .copied()
                    .unwrap_or(MAX_FORMULA_DOCUMENT_DEPTH);
                depth = depth.max(operand_depth.saturating_add(1));
            }
            if depth > MAX_FORMULA_DOCUMENT_DEPTH {
                let node =
                    u32::try_from(index)
                        .map(NodeId)
                        .map_err(|_| FormulaError::TooManyNodes {
                            node_count: self.nodes.len(),
                        })?;
                return Err(FormulaError::DocumentDepthLimitExceeded {
                    node,
                    depth,
                    limit: MAX_FORMULA_DOCUMENT_DEPTH,
                });
            }
            depths.push(depth);
        }
        Ok(())
    }

    #[cfg(feature = "serde")]
    fn maximum_depth(&self) -> usize {
        let mut depths: Vec<usize> = Vec::with_capacity(self.nodes.len());
        let mut maximum = 0_usize;
        for node in &self.nodes {
            let mut depth = 1_usize;
            for operand in node.kind.operands().into_iter().flatten() {
                let operand_depth = usize::try_from(operand.0)
                    .ok()
                    .and_then(|index| depths.get(index))
                    .copied()
                    .unwrap_or(0);
                depth = depth.max(operand_depth.saturating_add(1));
            }
            maximum = maximum.max(depth);
            depths.push(depth);
        }
        maximum
    }
}

#[cfg(feature = "serde")]
impl StrictDocument for FormulaDocument {
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

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec};

    use super::*;
    use crate::{
        NodeKind, PropositionEntry, PropositionId, PropositionMapDocument, PropositionMapError,
    };

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
