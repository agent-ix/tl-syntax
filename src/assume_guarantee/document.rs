//! `tl-syntax.contract-set/v1`: contracts bound to one formula graph by its
//! content identity (FR-309), and the read-only [`ContractView`] (FR-311).
//!
//! Pattern: `tl-syntax.fairness-premises/v1`. The `graph_identity` is the one
//! retained digest of this document (ADR-019 register item 3).

use alloc::{string::String, vec::Vec};
use core::fmt;

use super::{contract_obligation_node, ContractObligationError, MAX_CONTRACTS};
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{
            array_field_population, read_strict_document, StrictDocument, StrictDocumentReadError,
        },
    },
    FormulaDocument, InfiniteClock, InfiniteFormulaDocument, Node, NodeId, SemanticProfile,
    SyntaxArtifactLimits, MAX_SIGNAL_NAME_BYTES,
};

/// Stable identity of the contract-set edition.
pub const CONTRACT_SET_V1: &str = "tl-syntax.contract-set/v1";

/// Exact checked-in Draft 7 schema text for `tl-syntax.contract-set/v1`.
/// No digest constant exists (ADR-019 register item 1).
pub const CONTRACT_SET_V1_SCHEMA: &str = include_str!("../../spec/contract-set-v1.schema.json");

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.contract-set/v1`.
pub const CONTRACT_SET_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../spec/contract-set-v1.schema.json");

/// The formula graph a contract set binds to, in any existing edition.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ContractGraph<'a> {
    /// `tl-syntax.formula/v1` or `/v2`.
    Bounded(&'a FormulaDocument),
    /// `tl-syntax.formula-unbounded/v1`.
    Unbounded(&'a InfiniteFormulaDocument),
}

impl ContractGraph<'_> {
    /// The graph's semantic profile.
    pub fn semantic_profile(self) -> SemanticProfile {
        match self {
            Self::Bounded(document) => document.semantic_profile(),
            Self::Unbounded(document) => document.semantic_profile(),
        }
    }

    /// The graph's clock, where the edition has one.
    pub fn clock(self) -> Option<InfiniteClock> {
        match self {
            Self::Bounded(_) => None,
            Self::Unbounded(document) => Some(document.clock()),
        }
    }

    /// The graph's node count.
    pub fn node_count(self) -> usize {
        match self {
            Self::Bounded(document) => document.nodes().len(),
            Self::Unbounded(document) => document.nodes().len(),
        }
    }

    /// The `CONTENT_IDENTITY_DOMAIN_V1` identity of the canonical graph.
    pub fn content_identity(self) -> Result<String, serde_json::Error> {
        match self {
            Self::Bounded(document) => document.content_identity(),
            Self::Unbounded(document) => document.content_identity(),
        }
    }
}

/// One named contract: an assumption root and a guarantee root.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    /// Strictly increasing contract identity.
    pub id: u32,
    /// Unique name, 1..=255 UTF-8 bytes.
    pub name: String,
    /// Assumption root.
    pub assumption: NodeId,
    /// Guarantee root (may equal the assumption).
    pub guarantee: NodeId,
}

/// Refusal of a contract set. No document exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ContractSetError {
    /// A root names no node of the bound graph.
    RootOutOfRange {
        /// Contract identity.
        contract: u32,
        /// Rejected root.
        root: NodeId,
    },
    /// Contract identities repeat or decrease.
    ContractIdNotIncreasing {
        /// Previous identity.
        previous: u32,
        /// Current identity.
        current: u32,
    },
    /// A contract name is empty.
    EmptyName {
        /// Contract identity.
        contract: u32,
    },
    /// A contract name exceeds 255 UTF-8 bytes.
    NameTooLong {
        /// Contract identity.
        contract: u32,
    },
    /// Two contracts share a name.
    DuplicateName {
        /// First contract.
        first: u32,
        /// Second contract.
        second: u32,
    },
    /// More than [`MAX_CONTRACTS`] contracts.
    ContractLimitExceeded {
        /// Supplied count.
        count: usize,
        /// Stable ceiling.
        limit: usize,
    },
    /// The graph identity is empty or does not identify the supplied graph.
    GraphIdentityMismatch,
    /// The recorded profile is not the graph's.
    ProfileMismatch,
    /// The recorded clock is not the graph's (or one is absent).
    ClockMismatch,
}

impl fmt::Display for ContractSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "contract set refused: {self:?}")
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
enum ContractSetSchema {
    #[serde(rename = "tl-syntax.contract-set/v1")]
    V1,
}

/// Owned contract set bound to one canonical formula graph.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
pub struct ContractSetDocument {
    schema_version: ContractSetSchema,
    semantic_profile: SemanticProfile,
    #[serde(skip_serializing_if = "Option::is_none")]
    clock: Option<InfiniteClock>,
    graph_identity: String,
    contracts: Vec<Contract>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct ContractSetWire {
    schema_version: ContractSetSchema,
    semantic_profile: SemanticProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    clock: Option<InfiniteClock>,
    graph_identity: String,
    contracts: Vec<Contract>,
}

impl ContractSetDocument {
    /// Binds `contracts` to `graph`, recording its profile and clock.
    pub fn new(
        graph: ContractGraph<'_>,
        graph_identity: String,
        contracts: Vec<Contract>,
    ) -> Result<Self, ContractSetError> {
        Self::bind(
            graph,
            graph.semantic_profile(),
            graph.clock(),
            graph_identity,
            contracts,
        )
    }

    /// Binds recorded identities to `graph`, refusing any mismatch.
    pub fn bind(
        graph: ContractGraph<'_>,
        semantic_profile: SemanticProfile,
        clock: Option<InfiniteClock>,
        graph_identity: String,
        contracts: Vec<Contract>,
    ) -> Result<Self, ContractSetError> {
        if graph_identity.is_empty()
            || graph
                .content_identity()
                .map_or(true, |identity| identity != graph_identity)
        {
            return Err(ContractSetError::GraphIdentityMismatch);
        }
        if semantic_profile != graph.semantic_profile() {
            return Err(ContractSetError::ProfileMismatch);
        }
        if clock != graph.clock() {
            return Err(ContractSetError::ClockMismatch);
        }
        if contracts.len() > MAX_CONTRACTS {
            return Err(ContractSetError::ContractLimitExceeded {
                count: contracts.len(),
                limit: MAX_CONTRACTS,
            });
        }
        let node_count = graph.node_count();
        for (index, contract) in contracts.iter().enumerate() {
            if let Some(previous) = index.checked_sub(1).map(|item| &contracts[item]) {
                if previous.id >= contract.id {
                    return Err(ContractSetError::ContractIdNotIncreasing {
                        previous: previous.id,
                        current: contract.id,
                    });
                }
            }
            if contract.name.is_empty() {
                return Err(ContractSetError::EmptyName {
                    contract: contract.id,
                });
            }
            if contract.name.len() > MAX_SIGNAL_NAME_BYTES {
                return Err(ContractSetError::NameTooLong {
                    contract: contract.id,
                });
            }
            for root in [contract.assumption, contract.guarantee] {
                if usize::try_from(root.0).map_or(true, |root| root >= node_count) {
                    return Err(ContractSetError::RootOutOfRange {
                        contract: contract.id,
                        root,
                    });
                }
            }
        }
        let mut names: Vec<(&str, u32)> = contracts
            .iter()
            .map(|contract| (contract.name.as_str(), contract.id))
            .collect();
        names.sort_unstable();
        if let Some(pair) = names.windows(2).find(|pair| pair[0].0 == pair[1].0) {
            return Err(ContractSetError::DuplicateName {
                first: pair[0].1,
                second: pair[1].1,
            });
        }
        Ok(Self {
            schema_version: ContractSetSchema::V1,
            semantic_profile,
            clock,
            graph_identity,
            contracts,
        })
    }

    /// The recorded semantic profile.
    pub const fn semantic_profile(&self) -> SemanticProfile {
        self.semantic_profile
    }

    /// The recorded clock, where the edition has one.
    pub const fn clock(&self) -> Option<InfiniteClock> {
        self.clock
    }

    /// The graph's content identity.
    pub fn graph_identity(&self) -> &str {
        &self.graph_identity
    }

    /// Contracts in strictly increasing identity order.
    pub fn contracts(&self) -> &[Contract] {
        &self.contracts
    }

    /// The read-only realizability interface (FR-311).
    pub fn view(&self) -> ContractView<'_> {
        ContractView {
            contracts: &self.contracts,
        }
    }

    /// Canonical owner bytes.
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }

    /// Reads one canonical document and binds it to `graph`.
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
        graph: ContractGraph<'_>,
    ) -> Result<Self, ContractSetReadError> {
        let wire: ContractSetWire =
            read_strict_document(bytes, limits).map_err(ContractSetReadError::Read)?;
        let _ = wire.schema_version;
        Self::bind(
            graph,
            wire.semantic_profile,
            wire.clock,
            wire.graph_identity,
            wire.contracts,
        )
        .map_err(ContractSetReadError::Binding)
    }
}

/// Failure to read a contract set.
#[derive(Debug)]
#[non_exhaustive]
pub enum ContractSetReadError {
    /// Byte, shape, canonical-form or resource violation.
    Read(StrictDocumentReadError),
    /// Graph, profile, clock or contract validation failed.
    Binding(ContractSetError),
}

impl fmt::Display for ContractSetReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => error.fmt(formatter),
            Self::Binding(error) => error.fmt(formatter),
        }
    }
}

impl StrictDocument for ContractSetWire {
    fn preflight_resource_limits(
        bytes: &[u8],
        _limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError> {
        let contracts = array_field_population(bytes, b"contracts");
        if contracts > MAX_CONTRACTS {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "contracts",
                actual: contracts,
                limit: MAX_CONTRACTS,
            });
        }
        Ok(contracts)
    }

    fn validate_resource_limits(
        &self,
        _limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError> {
        Ok(())
    }
}

/// Read-only per-contract interface for realizability (FR-311): ids, names,
/// roots and the obligation lowering, and nothing that states an assumption
/// scope. Under option C the consumer reads the per-position outcome series
/// with [`super::assumption_prefix_verdict`] and checks Input-only
/// assumptions with [`super::check_assumption_inputs`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContractView<'a> {
    contracts: &'a [Contract],
}

impl<'a> ContractView<'a> {
    /// Every contract in identity order.
    pub const fn contracts(self) -> &'a [Contract] {
        self.contracts
    }

    /// Looks up one contract by identity.
    pub fn contract(self, id: u32) -> Option<&'a Contract> {
        self.contracts
            .binary_search_by_key(&id, |contract| contract.id)
            .ok()
            .map(|index| &self.contracts[index])
    }

    /// The obligation node for contract `id` over a graph of `node_count`
    /// nodes (FR-310): `Implies(assumption, guarantee)` at identity
    /// `node_count`.
    pub fn obligation(
        self,
        id: u32,
        node_count: usize,
    ) -> Option<Result<(NodeId, Node), ContractObligationError>> {
        self.contract(id).map(|contract| {
            contract_obligation_node(node_count, contract.assumption, contract.guarantee)
        })
    }
}
