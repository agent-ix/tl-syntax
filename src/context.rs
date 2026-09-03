use core::fmt;

use crate::SourceSpan;

/// Maximum UTF-8 byte length of each caller-context identity field.
pub const MAX_REQUIREMENT_CONTEXT_FIELD_BYTES: usize = 1_024;

/// A caller-supplied requirement/clause identity and its source span.
///
/// Validation establishes only completeness, byte bounds, and preservation. It
/// does not claim that the supplied provenance is truthful. Consumers represent
/// the absence of context with `Option<RequirementContext<'_>>`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementContext<'a> {
    requirement_id: &'a str,
    requirement_revision: &'a str,
    clause_id: &'a str,
    anchor: &'a str,
    source_span: SourceSpan,
}

impl<'a> RequirementContext<'a> {
    /// Constructs a complete caller context, rejecting empty or oversized fields.
    pub fn new(
        requirement_id: &'a str,
        requirement_revision: &'a str,
        clause_id: &'a str,
        anchor: &'a str,
        source_span: SourceSpan,
    ) -> Result<Self, RequirementContextError> {
        validate_field(RequirementContextField::RequirementId, requirement_id)?;
        validate_field(
            RequirementContextField::RequirementRevision,
            requirement_revision,
        )?;
        validate_field(RequirementContextField::ClauseId, clause_id)?;
        validate_field(RequirementContextField::Anchor, anchor)?;
        Ok(Self {
            requirement_id,
            requirement_revision,
            clause_id,
            anchor,
            source_span,
        })
    }

    /// Returns the caller's requirement identity bytes.
    pub const fn requirement_id(self) -> &'a str {
        self.requirement_id
    }

    /// Returns the caller's exact requirement revision bytes.
    pub const fn requirement_revision(self) -> &'a str {
        self.requirement_revision
    }

    /// Returns the caller's clause identity bytes.
    pub const fn clause_id(self) -> &'a str {
        self.clause_id
    }

    /// Returns the caller's anchor identity bytes.
    pub const fn anchor(self) -> &'a str {
        self.anchor
    }

    /// Returns the checked clause-level source span.
    pub const fn source_span(self) -> SourceSpan {
        self.source_span
    }
}

fn validate_field(
    field: RequirementContextField,
    value: &str,
) -> Result<(), RequirementContextError> {
    if value.is_empty() {
        Err(RequirementContextError::EmptyField { field })
    } else if value.len() > MAX_REQUIREMENT_CONTEXT_FIELD_BYTES {
        Err(RequirementContextError::FieldTooLong {
            field,
            length: value.len(),
            limit: MAX_REQUIREMENT_CONTEXT_FIELD_BYTES,
        })
    } else {
        Ok(())
    }
}

/// Identity field rejected by caller-context validation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementContextField {
    /// Requirement identity.
    RequirementId,
    /// Exact requirement revision.
    RequirementRevision,
    /// Clause identity.
    ClauseId,
    /// Execution or source anchor.
    Anchor,
}

impl fmt::Display for RequirementContextField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RequirementId => "requirement_id",
            Self::RequirementRevision => "requirement_revision",
            Self::ClauseId => "clause_id",
            Self::Anchor => "anchor",
        })
    }
}

/// Validation failure for a present caller context.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum RequirementContextError {
    /// A required present-context field is empty.
    EmptyField {
        /// Rejected field.
        field: RequirementContextField,
    },
    /// A required present-context field exceeds its byte limit.
    FieldTooLong {
        /// Rejected field.
        field: RequirementContextField,
        /// Observed UTF-8 byte length.
        length: usize,
        /// Maximum accepted UTF-8 byte length.
        limit: usize,
    },
}

impl fmt::Display for RequirementContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => write!(formatter, "caller context {field} is empty"),
            Self::FieldTooLong {
                field,
                length,
                limit,
            } => write!(
                formatter,
                "caller context {field} has {length} UTF-8 bytes, exceeding the {limit}-byte limit"
            ),
        }
    }
}
