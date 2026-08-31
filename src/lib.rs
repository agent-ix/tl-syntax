#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

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
