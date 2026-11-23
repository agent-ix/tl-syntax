//! LTLf finite-trace profile identity and the `tl-syntax.formula-ltlf/v1`
//! edition (P-TL-151, TL-428; FR-332..FR-334, ADR-044, ADR-045).
//!
//! The edition is a sibling of the bounded v1/v2 and unbounded editions and
//! shares none of their bytes. Its graph is an ordinary borrowed [`Formula`]
//! under `SemanticProfile::LtlfFiniteTraceV1`, so it validates in the
//! allocation-free model; the owned document is under `alloc`.

use core::fmt;

use crate::{Formula, FormulaError, LtlfRefusal, Node, NodeId, SemanticProfile};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "serde")]
use alloc::string::String;
#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{
            array_field_population, read_strict_document, StrictDocument, StrictDocumentReadError,
        },
    },
    NodeKind, PropositionId, SourceSpan, SyntaxArtifactLimits, TemporalInterval,
};

/// The LTLf profile identity.
pub const LTLF_FINITE_TRACE_PROFILE: &str = "mltl.ltlf-finite-trace/v1";
/// The LTLf formula edition identity.
pub const FORMULA_LTLF_V1: &str = "tl-syntax.formula-ltlf/v1";

/// Exact checked-in Draft 7 schema text for the LTLf edition. No digest
/// constant is added (2026-11-23 decision 7).
#[cfg(feature = "serde")]
pub const FORMULA_LTLF_V1_SCHEMA: &str =
    include_str!("../../corpus/schema/formula-ltlf-v1.schema.json");

/// Exact checked-in Draft 7 schema bytes for the LTLf edition.
#[cfg(feature = "serde")]
pub const FORMULA_LTLF_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../corpus/schema/formula-ltlf-v1.schema.json");

const EXISTING_EDITIONS: [&str; 3] = [
    "tl-syntax.formula/v1",
    "tl-syntax.formula/v2",
    "tl-syntax.formula-unbounded/v1",
];

/// Refusal of an edition or profile identity, naming the axis (FR-332-AC-3).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LtlfIdentityError {
    /// No edition identity was supplied.
    MissingEdition,
    /// The edition identity names no known edition.
    UnknownEdition,
    /// A different known edition was supplied.
    MismatchedEdition,
    /// No profile identity was supplied.
    MissingProfile,
    /// The profile identity names no known profile.
    UnknownProfile,
    /// A different known profile was supplied.
    MismatchedProfile(SemanticProfile),
}

impl fmt::Display for LtlfIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEdition => formatter.write_str("LTLf edition identity is missing"),
            Self::UnknownEdition => formatter.write_str("unknown formula edition identity"),
            Self::MismatchedEdition => {
                write!(formatter, "edition is not {FORMULA_LTLF_V1}")
            }
            Self::MissingProfile => formatter.write_str("LTLf profile identity is missing"),
            Self::UnknownProfile => formatter.write_str("unknown TL profile identity"),
            Self::MismatchedProfile(profile) => write!(
                formatter,
                "profile {} is not {LTLF_FINITE_TRACE_PROFILE}",
                profile.as_str()
            ),
        }
    }
}

/// Selects the LTLf edition and profile from raw identities without
/// inferring either; the edition axis is checked first.
pub fn select_ltlf_identity(
    edition: Option<&str>,
    profile: Option<&str>,
) -> Result<SemanticProfile, LtlfIdentityError> {
    match edition {
        None | Some("") => return Err(LtlfIdentityError::MissingEdition),
        Some(FORMULA_LTLF_V1) => {}
        Some(other) if EXISTING_EDITIONS.contains(&other) => {
            return Err(LtlfIdentityError::MismatchedEdition)
        }
        Some(_) => return Err(LtlfIdentityError::UnknownEdition),
    }
    let profile = match profile {
        None | Some("") => return Err(LtlfIdentityError::MissingProfile),
        Some(profile) => profile,
    };
    match SemanticProfile::ALL
        .into_iter()
        .find(|known| known.as_str() == profile)
    {
        Some(SemanticProfile::LtlfFiniteTraceV1) => Ok(SemanticProfile::LtlfFiniteTraceV1),
        Some(other) => Err(LtlfIdentityError::MismatchedProfile(other)),
        None => Err(LtlfIdentityError::UnknownProfile),
    }
}

/// Admission refusal of an LTLf formula. No formula exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LtlfFormulaError {
    /// An edition or profile identity is missing, unknown or mismatched.
    Identity(LtlfIdentityError),
    /// A node class the profile refuses (FR-332's nine classes).
    Refused {
        /// First refused node.
        node: NodeId,
        /// Its class.
        refusal: LtlfRefusal,
    },
    /// Structural graph failure (root, operand order, limits).
    Graph(FormulaError),
}

impl fmt::Display for LtlfFormulaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity(error) => error.fmt(formatter),
            Self::Refused { node, refusal } => write!(
                formatter,
                "{LTLF_FINITE_TRACE_PROFILE} refuses node {} ({refusal:?})",
                node.0
            ),
            Self::Graph(error) => error.fmt(formatter),
        }
    }
}

impl From<FormulaError> for LtlfFormulaError {
    fn from(error: FormulaError) -> Self {
        match error {
            FormulaError::LtlfNodeRefused { node, refusal } => Self::Refused { node, refusal },
            other => Self::Graph(other),
        }
    }
}

/// Validates a borrowed LTLf graph without allocation.
pub fn validate_ltlf(root: NodeId, nodes: &[Node]) -> Result<Formula<'_>, LtlfFormulaError> {
    Ok(Formula::new(SemanticProfile::LtlfFiniteTraceV1, root, nodes)?)
}

/// Owned `tl-syntax.formula-ltlf/v1` document.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LtlfFormulaDocument {
    root: NodeId,
    nodes: Vec<Node>,
}

#[cfg(feature = "alloc")]
impl LtlfFormulaDocument {
    /// Admits a graph under `profile`, which must be the LTLf profile.
    pub fn new(
        profile: SemanticProfile,
        root: NodeId,
        nodes: Vec<Node>,
    ) -> Result<Self, LtlfFormulaError> {
        if profile != SemanticProfile::LtlfFiniteTraceV1 {
            return Err(LtlfFormulaError::Identity(
                LtlfIdentityError::MismatchedProfile(profile),
            ));
        }
        if nodes.len() > crate::MAX_FORMULA_DOCUMENT_NODES {
            return Err(LtlfFormulaError::Graph(
                FormulaError::DocumentNodeLimitExceeded {
                    node_count: nodes.len(),
                    limit: crate::MAX_FORMULA_DOCUMENT_NODES,
                },
            ));
        }
        validate_ltlf(root, &nodes)?;
        let mut depths: Vec<usize> = Vec::with_capacity(nodes.len());
        for (index, node) in nodes.iter().enumerate() {
            let mut depth = 1_usize;
            for operand in node.kind.operands().into_iter().flatten() {
                depth = depth.max(depths[operand.0 as usize].saturating_add(1));
            }
            if depth > crate::MAX_FORMULA_DOCUMENT_DEPTH {
                return Err(LtlfFormulaError::Graph(
                    FormulaError::DocumentDepthLimitExceeded {
                        node: NodeId(index as u32),
                        depth,
                        limit: crate::MAX_FORMULA_DOCUMENT_DEPTH,
                    },
                ));
            }
            depths.push(depth);
        }
        Ok(Self { root, nodes })
    }

    /// The borrowed validated graph.
    pub fn formula(&self) -> Formula<'_> {
        // Admitted at construction; revalidation cannot fail.
        Formula::new(SemanticProfile::LtlfFiniteTraceV1, self.root, &self.nodes)
            .unwrap_or_else(|_| unreachable!("LTLf document admitted at construction"))
    }

    /// The edition identity.
    pub const fn schema_version(&self) -> &'static str {
        FORMULA_LTLF_V1
    }

    /// The profile identity.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        SemanticProfile::LtlfFiniteTraceV1
    }

    /// The root.
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// The nodes in topological order.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Canonical owner bytes.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(&self.wire(true))
    }

    /// Domain-separated semantic identity; source spans do not affect it.
    #[cfg(feature = "serde")]
    pub fn content_identity(&self) -> Result<String, serde_json::Error> {
        canonical_json(&self.wire(false))
            .map(|bytes| crate::contracts::identity::content_identity(FORMULA_LTLF_V1, &bytes))
    }

    #[cfg(feature = "serde")]
    fn wire(&self, spans: bool) -> LtlfFormulaWire {
        LtlfFormulaWire {
            schema_version: Some(String::from(FORMULA_LTLF_V1)),
            semantic_profile: Some(String::from(LTLF_FINITE_TRACE_PROFILE)),
            root: self.root,
            nodes: self
                .nodes
                .iter()
                .map(|node| LtlfNodeWire::from_node(*node, spans))
                .collect(),
        }
    }

    /// Reads one canonical LTLf document. Identities, refused node classes
    /// and graph failures are typed (FR-332).
    #[cfg(feature = "serde")]
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<Self, LtlfReadError> {
        let wire: LtlfFormulaWire =
            read_strict_document(bytes, limits).map_err(LtlfReadError::Read)?;
        let profile = select_ltlf_identity(
            wire.schema_version.as_deref(),
            wire.semantic_profile.as_deref(),
        )
            .map_err(|error| LtlfReadError::Formula(LtlfFormulaError::Identity(error)))?;
        let mut nodes = Vec::with_capacity(wire.nodes.len());
        for (index, node) in wire.nodes.into_iter().enumerate() {
            match node.into_node() {
                Ok(node) => nodes.push(node),
                Err(refusal) => {
                    return Err(LtlfReadError::Formula(LtlfFormulaError::Refused {
                        node: NodeId(index as u32),
                        refusal,
                    }))
                }
            }
        }
        Self::new(profile, wire.root, nodes).map_err(LtlfReadError::Formula)
    }
}

/// Failure to read an LTLf document.
#[cfg(feature = "serde")]
#[derive(Debug)]
#[non_exhaustive]
pub enum LtlfReadError {
    /// Byte, shape, canonical-form or resource violation.
    Read(StrictDocumentReadError),
    /// Identity, refused node or graph failure.
    Formula(LtlfFormulaError),
}

#[cfg(feature = "serde")]
impl fmt::Display for LtlfReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => error.fmt(formatter),
            Self::Formula(error) => error.fmt(formatter),
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct LtlfFormulaWire {
    // Optional on the wire so that an absent identity is refused with its
    // axis named (MissingEdition / MissingProfile, FR-332-AC-3); canonical
    // output always writes both.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    schema_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    semantic_profile: Option<String>,
    root: NodeId,
    nodes: Vec<LtlfNodeWire>,
}

/// Wire vocabulary of the LTLf edition. Refused classes decode so that they
/// can be refused with a typed cause; there is no bare `next` or `x` tag.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum LtlfNodeWire {
    False {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    True {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Proposition {
        proposition: PropositionId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Not {
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    And {
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Or {
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Implies {
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Equivalent {
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Future {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Globally {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Until {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Release {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    StrongNext {
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    WeakNext {
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    // Refused classes: decoded only to be refused with their own cause.
    Once {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Historically {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    StrongPrevious {
        operand: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Since {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    Triggered {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    WeakUntil {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
    StrongRelease {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<TemporalInterval>,
        left: NodeId,
        right: NodeId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<SourceSpan>,
    },
}

#[cfg(feature = "serde")]
fn interval_refusal(interval: Option<TemporalInterval>) -> Option<LtlfRefusal> {
    match interval {
        None => None,
        Some(TemporalInterval::Closed(_)) => Some(LtlfRefusal::ClosedInterval),
        Some(TemporalInterval::Unbounded(_)) => Some(LtlfRefusal::OpenInterval),
    }
}

#[cfg(feature = "serde")]
impl LtlfNodeWire {
    fn into_node(self) -> Result<Node, LtlfRefusal> {
        let with = |kind: NodeKind, span: Option<SourceSpan>| Node { kind, span };
        let checked = |interval: Option<TemporalInterval>| match interval_refusal(interval) {
            Some(refusal) => Err(refusal),
            None => Ok(()),
        };
        Ok(match self {
            Self::False { span } => with(NodeKind::False, span),
            Self::True { span } => with(NodeKind::True, span),
            Self::Proposition { proposition, span } => {
                with(NodeKind::Proposition { proposition }, span)
            }
            Self::Not { operand, span } => with(NodeKind::Not { operand }, span),
            Self::And { left, right, span } => with(NodeKind::And { left, right }, span),
            Self::Or { left, right, span } => with(NodeKind::Or { left, right }, span),
            Self::Implies { left, right, span } => with(NodeKind::Implies { left, right }, span),
            Self::Equivalent { left, right, span } => {
                with(NodeKind::Equivalent { left, right }, span)
            }
            Self::Future {
                interval,
                operand,
                span,
            } => {
                checked(interval)?;
                with(NodeKind::LtlfFuture { operand }, span)
            }
            Self::Globally {
                interval,
                operand,
                span,
            } => {
                checked(interval)?;
                with(NodeKind::LtlfGlobally { operand }, span)
            }
            Self::Until {
                interval,
                left,
                right,
                span,
            } => {
                checked(interval)?;
                with(NodeKind::LtlfUntil { left, right }, span)
            }
            Self::Release {
                interval,
                left,
                right,
                span,
            } => {
                checked(interval)?;
                with(NodeKind::LtlfRelease { left, right }, span)
            }
            Self::StrongNext { operand, span } => with(NodeKind::StrongNext { operand }, span),
            Self::WeakNext { operand, span } => with(NodeKind::WeakNext { operand }, span),
            Self::Once { .. } => return Err(LtlfRefusal::Once),
            Self::Historically { .. } => return Err(LtlfRefusal::Historically),
            Self::StrongPrevious { .. } => return Err(LtlfRefusal::Previous),
            Self::Since { .. } => return Err(LtlfRefusal::Since),
            Self::Triggered { .. } => return Err(LtlfRefusal::Triggered),
            Self::WeakUntil { .. } => return Err(LtlfRefusal::WeakUntil),
            Self::StrongRelease { .. } => return Err(LtlfRefusal::StrongRelease),
        })
    }

    /// Wire form of an admitted node. Only LTLf-admitted kinds occur in a
    /// validated document; any other kind is unreachable here.
    fn from_node(node: Node, spans: bool) -> Self {
        let span = if spans { node.span } else { None };
        match node.kind {
            NodeKind::False => Self::False { span },
            NodeKind::True => Self::True { span },
            NodeKind::Proposition { proposition } => Self::Proposition { proposition, span },
            NodeKind::Not { operand } => Self::Not { operand, span },
            NodeKind::And { left, right } => Self::And { left, right, span },
            NodeKind::Or { left, right } => Self::Or { left, right, span },
            NodeKind::Implies { left, right } => Self::Implies { left, right, span },
            NodeKind::Equivalent { left, right } => Self::Equivalent { left, right, span },
            NodeKind::LtlfFuture { operand } => Self::Future {
                interval: None,
                operand,
                span,
            },
            NodeKind::LtlfGlobally { operand } => Self::Globally {
                interval: None,
                operand,
                span,
            },
            NodeKind::LtlfUntil { left, right } => Self::Until {
                interval: None,
                left,
                right,
                span,
            },
            NodeKind::LtlfRelease { left, right } => Self::Release {
                interval: None,
                left,
                right,
                span,
            },
            NodeKind::StrongNext { operand } => Self::StrongNext { operand, span },
            NodeKind::WeakNext { operand } => Self::WeakNext { operand, span },
            _ => unreachable!("validated LTLf documents hold only LTLf-admitted nodes"),
        }
    }
}

#[cfg(feature = "serde")]
impl StrictDocument for LtlfFormulaWire {
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
        Ok(())
    }
}
