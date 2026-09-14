use core::fmt;

use crate::SyntaxArtifactLimits;

use super::{
    identity::canonical_json,
    limits::{OWNER_DOCUMENT_BYTES, OWNER_JSON_DEPTH},
};

/// Maximum accepted size of an owner document passed to a strict byte reader.
pub const MAX_TL_DOCUMENT_BYTES: usize = OWNER_DOCUMENT_BYTES;

/// Maximum JSON array/object nesting accepted by strict owner readers.
pub const MAX_TL_DOCUMENT_DEPTH: usize = OWNER_JSON_DEPTH;

/// Failure to read one complete bounded owner document.
#[cfg(feature = "serde")]
#[derive(Debug)]
#[non_exhaustive]
pub enum StrictDocumentReadError {
    /// The caller supplied more bytes than the public reader permits.
    DocumentTooLarge {
        /// Supplied byte count.
        actual: usize,
        /// Stable byte ceiling.
        limit: usize,
    },
    /// JSON container nesting exceeded the public reader ceiling.
    DepthLimitExceeded {
        /// First rejected nesting depth.
        actual: usize,
        /// Stable nesting ceiling.
        limit: usize,
    },
    /// One decoded JSON string exceeded the effective byte ceiling.
    StringTooLarge {
        /// First rejected decoded string size.
        actual: usize,
        /// Effective string byte ceiling.
        limit: usize,
    },
    /// Admission would exceed the effective deterministic work budget.
    WorkLimitExceeded {
        /// First rejected work charge.
        actual: usize,
        /// Effective work ceiling.
        limit: usize,
    },
    /// A typed population exceeded a caller-lowered or owner maximum.
    ResourceLimitExceeded {
        /// Stable population name.
        resource: &'static str,
        /// Rejected population or depth.
        actual: usize,
        /// Effective ceiling.
        limit: usize,
    },
    /// The input is valid JSON but not the one canonical owner encoding.
    NonCanonicalDocument,
    /// JSON shape, version, duplicate-member, trailing-data, or semantic validation failed.
    InvalidDocument(serde_json::Error),
}

#[cfg(feature = "serde")]
impl fmt::Display for StrictDocumentReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocumentTooLarge { actual, limit } => {
                write!(formatter, "document has {actual} bytes; limit is {limit}")
            }
            Self::DepthLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "document nesting depth is {actual}; limit is {limit}"
                )
            }
            Self::StringTooLarge { actual, limit } => {
                write!(
                    formatter,
                    "JSON string has {actual} bytes; limit is {limit}"
                )
            }
            Self::WorkLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "document admission costs {actual} work units; limit is {limit}"
                )
            }
            Self::ResourceLimitExceeded {
                resource,
                actual,
                limit,
            } => write!(
                formatter,
                "{resource} population/depth is {actual}; limit is {limit}"
            ),
            Self::NonCanonicalDocument => {
                formatter.write_str("document is not canonical owner JSON")
            }
            Self::InvalidDocument(error) => write!(formatter, "invalid document: {error}"),
        }
    }
}

#[cfg(feature = "serde")]
pub(crate) trait StrictDocument: serde::Serialize {
    fn preflight_resource_limits(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError>;

    fn validate_resource_limits(
        &self,
        limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError>;
}

#[cfg(feature = "serde")]
pub(crate) fn read_strict_document<T>(
    bytes: &[u8],
    limits: SyntaxArtifactLimits,
) -> Result<T, StrictDocumentReadError>
where
    T: serde::de::DeserializeOwned + StrictDocument,
{
    let limits = limits.constrained();
    if bytes.len() > limits.document_bytes {
        return Err(StrictDocumentReadError::DocumentTooLarge {
            actual: bytes.len(),
            limit: limits.document_bytes,
        });
    }
    preflight_document(bytes, limits)?;
    let retained_work = T::preflight_resource_limits(bytes, limits)?;
    let total_work = bytes.len().saturating_mul(3).saturating_add(retained_work);
    if total_work > limits.work {
        return Err(StrictDocumentReadError::WorkLimitExceeded {
            actual: total_work,
            limit: limits.work,
        });
    }
    let document: T =
        serde_json::from_slice(bytes).map_err(StrictDocumentReadError::InvalidDocument)?;
    document.validate_resource_limits(limits)?;
    let canonical = canonical_json(&document).map_err(StrictDocumentReadError::InvalidDocument)?;
    if canonical != bytes {
        return Err(StrictDocumentReadError::NonCanonicalDocument);
    }
    Ok(document)
}

#[cfg(feature = "serde")]
fn preflight_document(
    bytes: &[u8],
    limits: SyntaxArtifactLimits,
) -> Result<(), StrictDocumentReadError> {
    let minimum_work = bytes.len().saturating_mul(3);
    if minimum_work > limits.work {
        return Err(StrictDocumentReadError::WorkLimitExceeded {
            actual: minimum_work,
            limit: limits.work,
        });
    }
    let mut depth = 0_usize;
    let mut index = 0_usize;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => {
                index = scan_json_string(bytes, index + 1, limits.string_bytes)?;
            }
            b'{' | b'[' => {
                depth = depth.saturating_add(1);
                if depth > limits.json_depth {
                    return Err(StrictDocumentReadError::DepthLimitExceeded {
                        actual: depth,
                        limit: limits.json_depth,
                    });
                }
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
        index = index.saturating_add(1);
    }
    Ok(())
}

#[cfg(feature = "serde")]
fn scan_json_string(
    bytes: &[u8],
    mut index: usize,
    limit: usize,
) -> Result<usize, StrictDocumentReadError> {
    let mut decoded_bytes = 0_usize;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => return Ok(index),
            b'\\' => {
                index = index.saturating_add(1);
                if index >= bytes.len() {
                    return Ok(index);
                }
                if bytes[index] == b'u' {
                    let (units, consumed) = decoded_unicode_escape(bytes, index);
                    decoded_bytes = decoded_bytes.saturating_add(units);
                    index = index.saturating_add(consumed);
                } else {
                    decoded_bytes = decoded_bytes.saturating_add(1);
                }
            }
            _ => decoded_bytes = decoded_bytes.saturating_add(1),
        }
        if decoded_bytes > limit {
            return Err(StrictDocumentReadError::StringTooLarge {
                actual: decoded_bytes,
                limit,
            });
        }
        index = index.saturating_add(1);
    }
    Ok(index)
}

#[cfg(feature = "serde")]
fn decoded_unicode_escape(bytes: &[u8], u_index: usize) -> (usize, usize) {
    let Some(first) = parse_hex_quad(bytes, u_index.saturating_add(1)) else {
        return (0, 0);
    };
    if (0xD800..=0xDBFF).contains(&first)
        && bytes.get(u_index.saturating_add(5)) == Some(&b'\\')
        && bytes.get(u_index.saturating_add(6)) == Some(&b'u')
        && parse_hex_quad(bytes, u_index.saturating_add(7))
            .is_some_and(|second| (0xDC00..=0xDFFF).contains(&second))
    {
        return (4, 10);
    }
    let width = match first {
        0x0000..=0x007F => 1,
        0x0080..=0x07FF => 2,
        _ => 3,
    };
    (width, 4)
}

#[cfg(feature = "serde")]
fn parse_hex_quad(bytes: &[u8], start: usize) -> Option<u16> {
    let quad = bytes.get(start..start.checked_add(4)?)?;
    quad.iter().try_fold(0_u16, |value, byte| {
        let digit = match byte {
            b'0'..=b'9' => u16::from(*byte - b'0'),
            b'a'..=b'f' => u16::from(*byte - b'a' + 10),
            b'A'..=b'F' => u16::from(*byte - b'A' + 10),
            _ => return None,
        };
        Some(value.saturating_mul(16).saturating_add(digit))
    })
}

#[cfg(feature = "serde")]
pub(crate) fn array_field_population(bytes: &[u8], field: &[u8]) -> usize {
    let mut maximum = 0_usize;
    let mut index = 0_usize;
    while index < bytes.len() {
        if bytes[index] != b'"' {
            index = index.saturating_add(1);
            continue;
        }
        let start = index.saturating_add(1);
        let end = scan_string_end(bytes, start);
        if end >= bytes.len() {
            break;
        }
        index = end.saturating_add(1);
        if bytes.get(start..end) != Some(field) {
            continue;
        }
        index = skip_json_whitespace(bytes, index);
        if bytes.get(index) != Some(&b':') {
            continue;
        }
        index = skip_json_whitespace(bytes, index.saturating_add(1));
        if bytes.get(index) != Some(&b'[') {
            continue;
        }
        let (population, end) = count_array_items(bytes, index.saturating_add(1));
        maximum = maximum.max(population);
        index = end;
    }
    maximum
}

#[cfg(feature = "serde")]
fn count_array_items(bytes: &[u8], mut index: usize) -> (usize, usize) {
    let mut population = 0_usize;
    let mut nested = 0_usize;
    let mut has_item = false;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => {
                has_item = true;
                index = scan_string_end(bytes, index.saturating_add(1));
            }
            b'{' | b'[' => {
                has_item = true;
                nested = nested.saturating_add(1);
            }
            b'}' => nested = nested.saturating_sub(1),
            b']' if nested == 0 => {
                if has_item {
                    population = population.saturating_add(1);
                }
                return (population, index);
            }
            b']' => nested = nested.saturating_sub(1),
            b',' if nested == 0 => {
                population = population.saturating_add(1);
                has_item = false;
            }
            byte if !byte.is_ascii_whitespace() => has_item = true,
            _ => {}
        }
        index = index.saturating_add(1);
    }
    (population, index)
}

#[cfg(feature = "serde")]
fn scan_string_end(bytes: &[u8], mut index: usize) -> usize {
    let mut escaped = false;
    while index < bytes.len() {
        if escaped {
            escaped = false;
        } else if bytes[index] == b'\\' {
            escaped = true;
        } else if bytes[index] == b'"' {
            return index;
        }
        index = index.saturating_add(1);
    }
    index
}

#[cfg(feature = "serde")]
fn skip_json_whitespace(bytes: &[u8], mut index: usize) -> usize {
    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index = index.saturating_add(1);
    }
    index
}
