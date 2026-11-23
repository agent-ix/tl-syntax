//! Signal direction and the input/output partition of a formula (FR-034).
//!
//! Direction is a caller declaration on the signal, never on the proposition:
//! a proposition has the direction of the signal it is bound to.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::{binding::FormulaBindingError, catalog::SignalId};
use crate::PropositionId;

/// Who chooses a signal's value. `Undeclared` is the default and the reading of
/// every signal in a `tl-syntax.signal-catalog/v1` document.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SignalDirection {
    /// No direction was declared.
    #[default]
    Undeclared,
    /// The environment chooses the signal.
    Input,
    /// The system chooses the signal.
    Output,
}

impl SignalDirection {
    /// Returns the v2 wire spelling; `Undeclared` has none (the member is absent).
    pub const fn wire_name(self) -> Option<&'static str> {
        match self {
            Self::Undeclared => None,
            Self::Input => Some("input"),
            Self::Output => Some("output"),
        }
    }

    /// Returns whether no direction was declared.
    pub const fn is_undeclared(&self) -> bool {
        matches!(self, Self::Undeclared)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SignalDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.wire_name() {
            Some(name) => serializer.serialize_str(name),
            None => Err(serde::ser::Error::custom(
                "an undeclared signal direction has no wire form",
            )),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignalDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DirectionVisitor;

        impl<'v> serde::de::Visitor<'v> for DirectionVisitor {
            type Value = SignalDirection;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("\"input\" or \"output\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "input" => Ok(SignalDirection::Input),
                    "output" => Ok(SignalDirection::Output),
                    // `undeclared` is deliberately not a wire value: absence is
                    // the one spelling of an undeclared direction.
                    other => Err(E::unknown_variant(other, &["input", "output"])),
                }
            }
        }

        deserializer.deserialize_str(DirectionVisitor)
    }
}

/// The input and output propositions of one or more formulas.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SignalPartition {
    pub(crate) inputs: Vec<PropositionId>,
    pub(crate) outputs: Vec<PropositionId>,
}

#[cfg(feature = "alloc")]
impl SignalPartition {
    /// Propositions bound to `Input` signals, strictly increasing.
    pub fn inputs(&self) -> &[PropositionId] {
        &self.inputs
    }

    /// Propositions bound to `Output` signals, strictly increasing.
    pub fn outputs(&self) -> &[PropositionId] {
        &self.outputs
    }
}

/// Refusal to partition a formula's propositions by signal direction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SignalPartitionError {
    /// A proposition occurrence has no direct Boolean binding (FR-007 error).
    Binding(FormulaBindingError),
    /// The first occurring proposition whose bound signal has no direction.
    UndeclaredDirection {
        /// Proposition occurrence, first in formula-node order.
        proposition: PropositionId,
        /// The signal it is bound to.
        signal: SignalId,
    },
}

impl fmt::Display for SignalPartitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binding(error) => error.fmt(formatter),
            Self::UndeclaredDirection {
                proposition,
                signal,
            } => write!(
                formatter,
                "proposition {} is bound to signal {} which declares no direction",
                proposition.0, signal.0
            ),
        }
    }
}
