#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

mod context;
mod signal;
mod syntax;

#[cfg(feature = "alloc")]
mod context_document;
#[cfg(feature = "alloc")]
mod document;
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
    FormulaDocument, FormulaSchemaVersion, PropositionEntry, PropositionMapDocument,
    PropositionMapError, PropositionMapSchemaVersion, MAX_FORMULA_DOCUMENT_NODES,
};
pub use signal::{
    BoundFormula, FixedDecimalSignalDomain, FormulaBindingError, IntegerSignalDomain,
    PropositionBinding, SignalCatalog, SignalCatalogError, SignalDeclaration, SignalDomain,
    SignalDomainError, SignalId, SignalIter, MAX_SIGNAL_CATALOG_BINDINGS,
    MAX_SIGNAL_CATALOG_SIGNALS, MAX_SIGNAL_NAME_BYTES,
};
#[cfg(feature = "alloc")]
pub use signal_document::{
    OwnedSignalDeclaration, SignalCatalogDocument, SignalCatalogSchemaVersion,
};
pub use syntax::{
    Formula, FormulaError, Interval, IntervalError, Node, NodeId, NodeKind, PropositionId,
    SemanticProfile, SourceSpan, SourceSpanError,
};

/// Stable revision identifier for the checked-in shared temporal corpus.
pub const CORPUS_REVISION: &str = "tl-syntax-corpus/v1";
