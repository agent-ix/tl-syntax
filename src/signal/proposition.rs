use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::fmt;

use crate::PropositionId;
#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{
            array_field_population, read_strict_document, StrictDocument, StrictDocumentReadError,
        },
    },
    SyntaxArtifactLimits,
};

/// Maximum proposition entries accepted by the v1 proposition-map reader.
#[cfg(feature = "serde")]
pub const MAX_PROPOSITION_MAP_ENTRIES: usize = crate::contracts::limits::OWNER_PROPOSITIONS;

/// Exact checked-in Draft 7 schema text for `tl-syntax.proposition-map/v1`.
#[cfg(feature = "serde")]
pub const PROPOSITION_MAP_V1_SCHEMA: &str =
    include_str!("../../corpus/schema/proposition-map-v1.schema.json");

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.proposition-map/v1`.
#[cfg(feature = "serde")]
pub const PROPOSITION_MAP_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../corpus/schema/proposition-map-v1.schema.json");

/// Lowercase SHA-256 of [`PROPOSITION_MAP_V1_SCHEMA_BYTES`].
#[cfg(feature = "serde")]
pub const PROPOSITION_MAP_V1_SCHEMA_SHA256: &str =
    "ed5be5c2747db16f7936a94b6203bb2747a1abb5a0c5521c2efa38beb8dfdc80";

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
    #[serde(deserialize_with = "deserialize_propositions")]
    propositions: Vec<PropositionEntry>,
}

#[cfg(feature = "serde")]
fn deserialize_propositions<'de, D>(deserializer: D) -> Result<Vec<PropositionEntry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct PropositionVisitor;
    impl<'de> serde::de::Visitor<'de> for PropositionVisitor {
        type Value = Vec<PropositionEntry>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("at most 100000 proposition entries")
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            if sequence
                .size_hint()
                .is_some_and(|size| size > MAX_PROPOSITION_MAP_ENTRIES)
            {
                return Err(serde::de::Error::custom(
                    "propositions exceed the 100000-item wire limit",
                ));
            }
            let mut values = Vec::with_capacity(
                sequence
                    .size_hint()
                    .unwrap_or(0)
                    .min(MAX_PROPOSITION_MAP_ENTRIES),
            );
            while let Some(value) = sequence.next_element()? {
                if values.len() == MAX_PROPOSITION_MAP_ENTRIES {
                    return Err(serde::de::Error::custom(
                        "propositions exceed the 100000-item wire limit",
                    ));
                }
                values.push(value);
            }
            Ok(values)
        }
    }

    deserializer.deserialize_seq(PropositionVisitor)
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

    /// Reads exactly one bounded closed v1 JSON document through the owner type.
    ///
    /// Duplicate members, trailing JSON, unknown fields and versions, excess
    /// population, and semantic validation failures are refused.
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

#[cfg(feature = "serde")]
impl StrictDocument for PropositionMapDocument {
    fn preflight_resource_limits(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError> {
        let propositions = array_field_population(bytes, b"propositions");
        if propositions > limits.propositions {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "propositions",
                actual: propositions,
                limit: limits.propositions,
            });
        }
        Ok(propositions)
    }

    fn validate_resource_limits(
        &self,
        limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError> {
        if self.propositions.len() > limits.propositions {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "propositions",
                actual: self.propositions.len(),
                limit: limits.propositions,
            });
        }
        Ok(())
    }
}

/// Validation failure for a proposition-map document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
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
