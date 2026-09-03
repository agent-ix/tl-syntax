use alloc::string::String;

use crate::{RequirementContext, RequirementContextError, SourceSpan};

/// Version of the serialized caller-context document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RequirementContextSchemaVersion {
    /// Initial tl-syntax caller-context schema.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.requirement-context/v1"))]
    V1,
}

impl RequirementContextSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "tl-syntax.requirement-context/v1",
        }
    }
}

/// Owned, versioned caller-context exchange document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RequirementContextDocumentWire"))]
pub struct RequirementContextDocument {
    schema_version: RequirementContextSchemaVersion,
    requirement_id: String,
    requirement_revision: String,
    clause_id: String,
    anchor: String,
    source_span: SourceSpan,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RequirementContextDocumentWire {
    schema_version: RequirementContextSchemaVersion,
    requirement_id: String,
    requirement_revision: String,
    clause_id: String,
    anchor: String,
    source_span: SourceSpan,
}

#[cfg(feature = "serde")]
impl TryFrom<RequirementContextDocumentWire> for RequirementContextDocument {
    type Error = RequirementContextError;

    fn try_from(wire: RequirementContextDocumentWire) -> Result<Self, Self::Error> {
        Self::from_parts(
            wire.schema_version,
            wire.requirement_id,
            wire.requirement_revision,
            wire.clause_id,
            wire.anchor,
            wire.source_span,
        )
    }
}

impl RequirementContextDocument {
    /// Constructs and validates a v1 caller-context document.
    pub fn new(
        requirement_id: String,
        requirement_revision: String,
        clause_id: String,
        anchor: String,
        source_span: SourceSpan,
    ) -> Result<Self, RequirementContextError> {
        Self::from_parts(
            RequirementContextSchemaVersion::V1,
            requirement_id,
            requirement_revision,
            clause_id,
            anchor,
            source_span,
        )
    }

    fn from_parts(
        schema_version: RequirementContextSchemaVersion,
        requirement_id: String,
        requirement_revision: String,
        clause_id: String,
        anchor: String,
        source_span: SourceSpan,
    ) -> Result<Self, RequirementContextError> {
        let document = Self {
            schema_version,
            requirement_id,
            requirement_revision,
            clause_id,
            anchor,
            source_span,
        };
        document.validate()?;
        Ok(document)
    }

    /// Copies a validated borrowed context into a v1 owned document.
    pub fn from_context(context: RequirementContext<'_>) -> Self {
        Self {
            schema_version: RequirementContextSchemaVersion::V1,
            requirement_id: context.requirement_id().into(),
            requirement_revision: context.requirement_revision().into(),
            clause_id: context.clause_id().into(),
            anchor: context.anchor().into(),
            source_span: context.source_span(),
        }
    }

    /// Validates and borrows the complete context.
    pub fn validate(&self) -> Result<RequirementContext<'_>, RequirementContextError> {
        RequirementContext::new(
            &self.requirement_id,
            &self.requirement_revision,
            &self.clause_id,
            &self.anchor,
            self.source_span,
        )
    }

    /// Returns the wire schema version.
    pub const fn schema_version(&self) -> RequirementContextSchemaVersion {
        self.schema_version
    }
}
