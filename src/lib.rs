#![no_std]
//! Parser-independent syntax and semantic-profile identities for bounded MLTL.
//!
//! The default feature set has no dependencies and does not require allocation.
//! Enable `alloc` for owned documents and `serde` for the versioned wire model.

#[cfg(feature = "alloc")]
extern crate alloc;

mod syntax;

#[cfg(feature = "alloc")]
mod document;

#[cfg(feature = "alloc")]
pub use document::{
    FormulaDocument, FormulaSchemaVersion, PropositionEntry, PropositionMapDocument,
    PropositionMapError, PropositionMapSchemaVersion,
};
pub use syntax::{
    Formula, FormulaError, Interval, IntervalError, Node, NodeId, NodeKind, PropositionId,
    SemanticProfile, SourceSpan, SourceSpanError,
};

/// Stable revision identifier for the checked-in shared temporal corpus.
pub const CORPUS_REVISION: &str = "tl-syntax-corpus/v1";
