//! Shared owner-contract infrastructure.

pub mod limits;

#[cfg(feature = "serde")]
mod manifest_impl;

#[cfg(feature = "serde")]
pub mod identity;

#[cfg(feature = "serde")]
pub mod reader;

#[cfg(feature = "serde")]
pub mod manifest {
    //! Authorized implementation-manifest contract.

    pub use super::manifest_impl::{
        validate_past_profile_implementation, PastProfileManifestError,
        PAST_PROFILE_IMPLEMENTATION_V1, PAST_PROFILE_MANIFEST_MAX_BYTES,
    };
}
