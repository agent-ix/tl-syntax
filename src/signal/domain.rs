//! Closed typed-signal domain vocabulary.

use core::fmt;

/// Checked inclusive bounds for a signed integer signal.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IntegerSignalDomain {
    minimum: i64,
    maximum: i64,
}

impl IntegerSignalDomain {
    /// Constructs inclusive signed integer bounds.
    pub const fn new(minimum: i64, maximum: i64) -> Result<Self, SignalDomainError> {
        if minimum <= maximum {
            Ok(Self { minimum, maximum })
        } else {
            Err(SignalDomainError::IntegerBoundsInverted { minimum, maximum })
        }
    }

    /// Returns the inclusive minimum.
    pub const fn minimum(self) -> i64 {
        self.minimum
    }

    /// Returns the inclusive maximum.
    pub const fn maximum(self) -> i64 {
        self.maximum
    }
}

/// Checked inclusive coefficient bounds and scale for a fixed-decimal signal.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FixedDecimalSignalDomain {
    minimum_coefficient: i64,
    maximum_coefficient: i64,
    scale: u8,
}

impl FixedDecimalSignalDomain {
    /// Constructs inclusive coefficient bounds at a scale from 0 through 18.
    pub const fn new(
        minimum_coefficient: i64,
        maximum_coefficient: i64,
        scale: u8,
    ) -> Result<Self, SignalDomainError> {
        if minimum_coefficient > maximum_coefficient {
            Err(SignalDomainError::DecimalBoundsInverted {
                minimum_coefficient,
                maximum_coefficient,
            })
        } else if scale > 18 {
            Err(SignalDomainError::DecimalScaleOutOfRange { scale })
        } else {
            Ok(Self {
                minimum_coefficient,
                maximum_coefficient,
                scale,
            })
        }
    }

    /// Returns the inclusive minimum coefficient.
    pub const fn minimum_coefficient(self) -> i64 {
        self.minimum_coefficient
    }

    /// Returns the inclusive maximum coefficient.
    pub const fn maximum_coefficient(self) -> i64 {
        self.maximum_coefficient
    }

    /// Returns the number of base-ten fractional digits.
    pub const fn scale(self) -> u8 {
        self.scale
    }
}

/// Closed v1 value-domain vocabulary for temporal input signals.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalDomain {
    /// A Boolean signal that may bind directly to an MLTL proposition.
    Boolean,
    /// A bounded signed integer signal.
    Integer(IntegerSignalDomain),
    /// A bounded fixed-decimal signal represented by a scaled coefficient.
    FixedDecimal(FixedDecimalSignalDomain),
}

#[cfg(feature = "serde")]
impl serde::Serialize for SignalDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        match self {
            Self::Boolean => {
                let mut state = serializer.serialize_struct("SignalDomain", 1)?;
                state.serialize_field("kind", "boolean")?;
                state.end()
            }
            Self::Integer(value) => {
                let mut state = serializer.serialize_struct("SignalDomain", 3)?;
                state.serialize_field("kind", "integer")?;
                state.serialize_field("minimum", &value.minimum())?;
                state.serialize_field("maximum", &value.maximum())?;
                state.end()
            }
            Self::FixedDecimal(value) => {
                let mut state = serializer.serialize_struct("SignalDomain", 4)?;
                state.serialize_field("kind", "fixed_decimal")?;
                state.serialize_field("minimum_coefficient", &value.minimum_coefficient())?;
                state.serialize_field("maximum_coefficient", &value.maximum_coefficient())?;
                state.serialize_field("scale", &value.scale())?;
                state.end()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignalDomain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        #[derive(serde::Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum Wire {
            Boolean {},
            Integer {
                minimum: i64,
                maximum: i64,
            },
            FixedDecimal {
                minimum_coefficient: i64,
                maximum_coefficient: i64,
                scale: u8,
            },
        }

        match Wire::deserialize(deserializer)? {
            Wire::Boolean {} => Ok(Self::Boolean),
            Wire::Integer { minimum, maximum } => IntegerSignalDomain::new(minimum, maximum)
                .map(Self::Integer)
                .map_err(D::Error::custom),
            Wire::FixedDecimal {
                minimum_coefficient,
                maximum_coefficient,
                scale,
            } => FixedDecimalSignalDomain::new(minimum_coefficient, maximum_coefficient, scale)
                .map(Self::FixedDecimal)
                .map_err(D::Error::custom),
        }
    }
}

/// Construction failure for a bounded scalar signal domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SignalDomainError {
    /// Integer bounds are inverted.
    IntegerBoundsInverted { minimum: i64, maximum: i64 },
    /// Fixed-decimal coefficient bounds are inverted.
    DecimalBoundsInverted {
        minimum_coefficient: i64,
        maximum_coefficient: i64,
    },
    /// A fixed-decimal scale exceeds 18.
    DecimalScaleOutOfRange { scale: u8 },
}

impl fmt::Display for SignalDomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerBoundsInverted { minimum, maximum } => write!(
                formatter,
                "integer signal minimum {minimum} exceeds maximum {maximum}"
            ),
            Self::DecimalBoundsInverted {
                minimum_coefficient,
                maximum_coefficient,
            } => write!(
                formatter,
                "decimal signal coefficient minimum {minimum_coefficient} exceeds maximum {maximum_coefficient}"
            ),
            Self::DecimalScaleOutOfRange { scale } => {
                write!(formatter, "decimal signal scale {scale} exceeds 18")
            }
        }
    }
}
