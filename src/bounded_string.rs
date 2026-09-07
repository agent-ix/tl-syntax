//! Strict, bounded owned-string decoding for versioned wire documents.

#[cfg(feature = "serde")]
use alloc::string::String;
#[cfg(feature = "serde")]
use core::fmt;

/// Decodes one UTF-8 field while applying its byte bound before this crate
/// constructs its owned `String` value.
#[cfg(feature = "serde")]
pub(crate) fn deserialize<'de, D>(
    deserializer: D,
    limit: usize,
    description: &'static str,
) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor {
        limit: usize,
        description: &'static str,
    }

    impl Visitor {
        fn checked<E: serde::de::Error>(&self, value: &str) -> Result<(), E> {
            if value.len() > self.limit {
                Err(E::custom(format_args!(
                    "{description} has {} UTF-8 bytes, exceeding the {}-byte wire limit",
                    value.len(),
                    self.limit,
                    description = self.description,
                )))
            } else {
                Ok(())
            }
        }
    }

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                formatter,
                "a {} no longer than {} UTF-8 bytes",
                self.description, self.limit
            )
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.checked(value)?;
            Ok(value.into())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.checked(value)?;
            Ok(value.into())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.checked(&value)?;
            Ok(value)
        }
    }

    deserializer.deserialize_string(Visitor { limit, description })
}
