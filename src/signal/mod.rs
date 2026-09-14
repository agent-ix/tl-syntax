//! Typed signal domains, catalogs, bindings, and owner documents.

pub mod binding;
pub mod catalog;
pub mod domain;
#[cfg(feature = "alloc")]
mod proposition;

#[cfg(feature = "alloc")]
pub mod document;
