//! Resource ceilings shared by every strict syntax-artifact reader.

pub(crate) const OWNER_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;
pub(crate) const OWNER_JSON_DEPTH: usize = 64;
pub(crate) const OWNER_STRING_BYTES: usize = 64 * 1024;
pub(crate) const OWNER_FORMULA_NODES: usize = 100_000;
pub(crate) const OWNER_FORMULA_DEPTH: usize = 4_096;
pub(crate) const OWNER_SIGNALS: usize = 100_000;
pub(crate) const OWNER_BINDINGS: usize = 100_000;
pub(crate) const OWNER_PROPOSITIONS: usize = 100_000;
#[cfg(feature = "serde")]
pub(crate) const OWNER_MANIFEST_BYTES: usize = 64 * 1024;
pub(crate) const OWNER_WORK: usize = 256 * 1024 * 1024;

/// Owner-controlled resource limits for strict syntax-artifact admission.
///
/// Callers may lower any field. The reader intersects supplied limits with
/// [`Self::OWNER_MAXIMA`], so callers can never raise an owner ceiling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SyntaxArtifactLimits {
    /// Maximum exact input bytes.
    pub document_bytes: usize,
    /// Maximum JSON object/array nesting.
    pub json_depth: usize,
    /// Maximum decoded UTF-8 bytes in any JSON string.
    pub string_bytes: usize,
    /// Maximum formula nodes.
    pub formula_nodes: usize,
    /// Maximum formula graph depth.
    pub formula_depth: usize,
    /// Maximum signal declarations.
    pub signals: usize,
    /// Maximum proposition-to-signal bindings.
    pub bindings: usize,
    /// Maximum proposition-map entries.
    pub propositions: usize,
    /// Maximum deterministic admission work units.
    pub work: usize,
}

impl SyntaxArtifactLimits {
    /// Immutable maxima enforced by the syntax owner.
    pub const OWNER_MAXIMA: Self = Self {
        document_bytes: OWNER_DOCUMENT_BYTES,
        json_depth: OWNER_JSON_DEPTH,
        string_bytes: OWNER_STRING_BYTES,
        formula_nodes: OWNER_FORMULA_NODES,
        formula_depth: OWNER_FORMULA_DEPTH,
        signals: OWNER_SIGNALS,
        bindings: OWNER_BINDINGS,
        propositions: OWNER_PROPOSITIONS,
        work: OWNER_WORK,
    };

    /// Returns limits that cannot exceed the syntax owner's maxima.
    pub const fn constrained(self) -> Self {
        let owner = Self::OWNER_MAXIMA;
        Self {
            document_bytes: minimum(self.document_bytes, owner.document_bytes),
            json_depth: minimum(self.json_depth, owner.json_depth),
            string_bytes: minimum(self.string_bytes, owner.string_bytes),
            formula_nodes: minimum(self.formula_nodes, owner.formula_nodes),
            formula_depth: minimum(self.formula_depth, owner.formula_depth),
            signals: minimum(self.signals, owner.signals),
            bindings: minimum(self.bindings, owner.bindings),
            propositions: minimum(self.propositions, owner.propositions),
            work: minimum(self.work, owner.work),
        }
    }
}

impl Default for SyntaxArtifactLimits {
    fn default() -> Self {
        Self::OWNER_MAXIMA
    }
}

const fn minimum(left: usize, right: usize) -> usize {
    if left < right {
        left
    } else {
        right
    }
}
