use alloc::{format, string::String, vec::Vec};

use sha2::{Digest, Sha256};

/// Domain prefix used before contract identity and canonical document bytes.
pub const CONTENT_IDENTITY_DOMAIN_V1: &[u8] = b"tl-syntax.content-identity/v1\0";

pub(crate) fn canonical_json<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(value)
}

pub(crate) fn content_identity(contract: &str, canonical_bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(CONTENT_IDENTITY_DOMAIN_V1);
    hash.update(contract.as_bytes());
    hash.update([0]);
    hash.update(canonical_bytes);
    let digest = hash.finalize();
    format!("{digest:x}")
}
