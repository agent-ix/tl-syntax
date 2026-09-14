//! Proposition-to-signal binding contracts.

use core::fmt;

use crate::{Formula, PropositionId};

use super::catalog::{SignalCatalog, SignalDeclaration, SignalId};

/// One direct MLTL proposition-to-signal binding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PropositionBinding {
    proposition: PropositionId,
    signal: SignalId,
}

impl PropositionBinding {
    /// Constructs an unvalidated binding for validation as part of a catalog.
    pub const fn new(proposition: PropositionId, signal: SignalId) -> Self {
        Self {
            proposition,
            signal,
        }
    }

    /// Returns the proposition identity.
    pub const fn proposition(self) -> PropositionId {
        self.proposition
    }

    /// Returns the target signal identity.
    pub const fn signal(self) -> SignalId {
        self.signal
    }
}

/// A formula whose proposition occurrences all resolve through one catalog.
#[derive(Clone, Copy, Debug)]
pub struct BoundFormula<'formula, 'catalog> {
    pub(super) formula: Formula<'formula>,
    pub(super) catalog: SignalCatalog<'catalog>,
}

impl<'formula, 'catalog> BoundFormula<'formula, 'catalog> {
    /// Returns the validated formula.
    pub const fn formula(self) -> Formula<'formula> {
        self.formula
    }

    /// Returns the validated signal catalog.
    pub const fn catalog(self) -> SignalCatalog<'catalog> {
        self.catalog
    }

    /// Resolves a proposition to its declared Boolean signal.
    pub fn signal_for_proposition(
        self,
        proposition: PropositionId,
    ) -> Option<SignalDeclaration<'catalog>> {
        self.catalog.signal_for_proposition(proposition)
    }
}

/// Formula-to-catalog binding failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FormulaBindingError {
    /// The first proposition occurrence without a direct catalog binding.
    MissingPropositionBinding { proposition: PropositionId },
}

impl fmt::Display for FormulaBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPropositionBinding { proposition } => write!(
                formatter,
                "formula proposition {} has no signal binding",
                proposition.0
            ),
        }
    }
}
