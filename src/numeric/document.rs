//! Owned `tl-syntax.predicate-table/v1` document and its strict reader (FR-303).
//!
//! The public document has no unchecked constructor: construction and reading
//! both take the catalog and route through [`PredicateTableDocument::new`], so
//! an ill-typed table is unrepresentable.

use alloc::vec::Vec;
use core::fmt;

use super::{
    expr::{ExprNode, ExprSlot, ExprType},
    predicate::{Predicate, PredicateTable, PredicateTableError, MAX_PREDICATES},
};
use crate::SignalCatalog;
#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{
            array_field_population, read_strict_document, StrictDocument, StrictDocumentReadError,
        },
    },
    numeric::expr::MAX_EXPRESSION_NODES,
    SyntaxArtifactLimits,
};

/// Stable identity of the predicate-table edition.
pub const PREDICATE_TABLE_V1: &str = "tl-syntax.predicate-table/v1";

/// Exact checked-in Draft 7 schema text for `tl-syntax.predicate-table/v1`.
/// No digest constant exists (ADR-019 register item 1).
#[cfg(feature = "serde")]
pub const PREDICATE_TABLE_V1_SCHEMA: &str =
    include_str!("../../spec/predicate-table-v1.schema.json");

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.predicate-table/v1`.
#[cfg(feature = "serde")]
pub const PREDICATE_TABLE_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../spec/predicate-table-v1.schema.json");

/// Version of the serialized predicate table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PredicateTableSchemaVersion {
    /// First predicate-table edition.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.predicate-table/v1"))]
    V1,
}

impl PredicateTableSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => PREDICATE_TABLE_V1,
        }
    }
}

/// Owned, validated predicate table.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PredicateTableDocument {
    schema_version: PredicateTableSchemaVersion,
    expressions: Vec<ExprNode>,
    predicates: Vec<Predicate>,
}

impl PredicateTableDocument {
    /// Types every expression against `catalog` and validates every predicate.
    pub fn new(
        catalog: SignalCatalog<'_>,
        expressions: Vec<ExprNode>,
        predicates: Vec<Predicate>,
    ) -> Result<Self, PredicateTableError> {
        let document = Self {
            schema_version: PredicateTableSchemaVersion::V1,
            expressions,
            predicates,
        };
        document.table(catalog)?;
        Ok(document)
    }

    /// Re-validates against `catalog` and returns the borrowed view.
    pub fn table(&self, catalog: SignalCatalog<'_>) -> Result<PredicateTable<'_>, PredicateTableError> {
        let mut scratch = alloc::vec![ExprSlot::EMPTY; self.expressions.len()];
        PredicateTable::new(catalog, &self.predicates, &self.expressions, &mut scratch)
    }

    /// Returns the derived type of every expression node, in arena order.
    pub fn expression_types(
        &self,
        catalog: SignalCatalog<'_>,
    ) -> Result<Vec<ExprType>, PredicateTableError> {
        let mut scratch = alloc::vec![ExprSlot::EMPTY; self.expressions.len()];
        PredicateTable::new(catalog, &self.predicates, &self.expressions, &mut scratch)?;
        Ok(scratch.into_iter().map(ExprSlot::ty).collect())
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> PredicateTableSchemaVersion {
        self.schema_version
    }

    /// Returns the shared expression arena.
    pub fn expressions(&self) -> &[ExprNode] {
        &self.expressions
    }

    /// Returns the predicates in strictly increasing proposition order.
    pub fn predicates(&self) -> &[Predicate] {
        &self.predicates
    }

    /// Serializes to the one canonical owner encoding (member order fixed by
    /// the schema: `schema_version`, `expressions`, `predicates`).
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }

    /// Reads exactly one bounded canonical document and validates it against
    /// `catalog` through the same constructor as [`Self::new`].
    #[cfg(feature = "serde")]
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
        catalog: SignalCatalog<'_>,
    ) -> Result<Self, PredicateTableReadError> {
        let wire: PredicateTableWire =
            read_strict_document(bytes, limits).map_err(PredicateTableReadError::Read)?;
        let _ = wire.schema_version;
        Self::new(catalog, wire.expressions, wire.predicates).map_err(PredicateTableReadError::Table)
    }
}

/// Failure to read a predicate table: a wire violation or a table refusal.
#[derive(Debug)]
#[non_exhaustive]
pub enum PredicateTableReadError {
    /// Byte, shape, canonical-form or resource violation.
    #[cfg(feature = "serde")]
    Read(StrictDocumentReadError),
    /// The decoded table fails exactly as direct construction would.
    Table(PredicateTableError),
}

impl fmt::Display for PredicateTableReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "serde")]
            Self::Read(error) => error.fmt(formatter),
            Self::Table(error) => error.fmt(formatter),
        }
    }
}

/// Unvalidated wire shape, used only inside the strict reader.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct PredicateTableWire {
    schema_version: PredicateTableSchemaVersion,
    #[serde(deserialize_with = "deserialize_expressions")]
    expressions: Vec<ExprNode>,
    #[serde(deserialize_with = "deserialize_predicates")]
    predicates: Vec<Predicate>,
}

#[cfg(feature = "serde")]
fn deserialize_expressions<'de, D>(deserializer: D) -> Result<Vec<ExprNode>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::signal::document::deserialize_bounded_vec(
        deserializer,
        MAX_EXPRESSION_NODES,
        "expression nodes",
    )
}

#[cfg(feature = "serde")]
fn deserialize_predicates<'de, D>(deserializer: D) -> Result<Vec<Predicate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    crate::signal::document::deserialize_bounded_vec(deserializer, MAX_PREDICATES, "predicates")
}

#[cfg(feature = "serde")]
impl StrictDocument for PredicateTableWire {
    fn preflight_resource_limits(
        bytes: &[u8],
        _limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError> {
        let expressions = array_field_population(bytes, b"expressions");
        if expressions > MAX_EXPRESSION_NODES {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "expression nodes",
                actual: expressions,
                limit: MAX_EXPRESSION_NODES,
            });
        }
        let predicates = array_field_population(bytes, b"predicates");
        if predicates > MAX_PREDICATES {
            return Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource: "predicates",
                actual: predicates,
                limit: MAX_PREDICATES,
            });
        }
        Ok(expressions.saturating_add(predicates))
    }

    fn validate_resource_limits(
        &self,
        _limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError> {
        Ok(())
    }
}
