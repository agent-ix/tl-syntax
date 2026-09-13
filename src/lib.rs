#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod bounded_string;
mod context;
mod future;
mod signal;
mod syntax;

#[cfg(feature = "alloc")]
mod context_document;
#[cfg(feature = "alloc")]
mod document;
#[cfg(feature = "serde")]
mod past_manifest;
#[cfg(feature = "alloc")]
mod signal_document;

pub use context::{
    RequirementContext, RequirementContextError, RequirementContextField,
    MAX_REQUIREMENT_CONTEXT_FIELD_BYTES,
};
#[cfg(feature = "alloc")]
pub use context_document::{RequirementContextDocument, RequirementContextSchemaVersion};
#[cfg(feature = "alloc")]
pub use document::{
    FormulaConversionError, FormulaDocument, FormulaSchemaVersion, PropositionEntry,
    PropositionMapDocument, PropositionMapError, PropositionMapSchemaVersion,
    SemanticFormulaDocument,
};
#[cfg(feature = "serde")]
pub use document::{
    StrictDocumentReadError, MAX_PROPOSITION_MAP_ENTRIES, MAX_TL_DOCUMENT_BYTES,
    MAX_TL_DOCUMENT_DEPTH, PROPOSITION_MAP_V1_SCHEMA,
};
pub use future::{
    FutureKind, FutureLowering, FutureLoweringAxis, FutureLoweringOperand, FutureLoweringRefusal,
    FutureLoweringReport, FutureLoweringRequest, FutureLoweringSpanRole, RawBounds,
    UnsupportedFutureKind, FUTURE_LOWERING_NODE_CHARGE, FUTURE_LOWERING_REFUSAL_V1,
    FUTURE_LOWERING_REPORT_V1, FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1,
    MAX_FUTURE_LOWERING_IDENTITY_BYTES, MAX_FUTURE_LOWERING_KIND_BYTES,
};
#[cfg(feature = "serde")]
pub use past_manifest::{
    validate_past_profile_implementation, PastProfileManifestError, PAST_PROFILE_IMPLEMENTATION_V1,
    PAST_PROFILE_MANIFEST_MAX_BYTES,
};
pub use signal::{
    BoundFormula, FixedDecimalSignalDomain, FormulaBindingError, IntegerSignalDomain,
    PropositionBinding, SignalCatalog, SignalCatalogError, SignalDeclaration, SignalDomain,
    SignalDomainError, SignalId, SignalIter, MAX_SIGNAL_CATALOG_BINDINGS,
    MAX_SIGNAL_CATALOG_SIGNALS, MAX_SIGNAL_NAME_BYTES,
};
#[cfg(feature = "serde")]
pub use signal_document::SIGNAL_CATALOG_V1_SCHEMA;
#[cfg(feature = "alloc")]
pub use signal_document::{
    OwnedSignalDeclaration, SignalCatalogDocument, SignalCatalogSchemaVersion,
};
pub use syntax::{
    Formula, FormulaError, Interval, IntervalError, Node, NodeId, NodeKind, OperatorArity,
    PastOperatorKind, PropositionId, SemanticProfile, SourceSpan, SourceSpanError, TemporalFamily,
    MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES, PAST_OPERATORS_V1,
};

/// Stable revision identifier for the checked-in shared temporal corpus.
pub const CORPUS_REVISION: &str = "tl-syntax-corpus/v1";

/// Stable identity of the immutable paired past/history corpus.
pub const PAST_HISTORY_CORPUS_V1: &str = "tl-syntax.past-history-corpus/v1";
