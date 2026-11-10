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

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    /// A JSON escape written with `@` for the backslash, so the sources stay readable.
    fn json(text: &str) -> Vec<u8> {
        text.replace('@', "\\").into_bytes()
    }

    fn decoded(text: &str, limit: usize) -> Result<usize, usize> {
        let bytes = json(text);
        match scan_json_string(&bytes, 0, limit) {
            Ok(end) => Ok(end),
            Err(StrictDocumentReadError::StringTooLarge { actual, .. }) => Err(actual),
            Err(other) => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn a_string_is_charged_the_utf8_width_of_each_escape() {
        // One ASCII, two-byte, three-byte and surrogate-pair escape cost 1, 2, 3 and 4 bytes.
        for (text, width) in [
            ("@u0041\"", 1),
            ("@u007f\"", 1),
            ("@u0080\"", 2),
            ("@u07ff\"", 2),
            ("@u0800\"", 3),
            ("@u20ac\"", 3),
            ("@ud83d@ude00\"", 4),
            ("@ud83d\"", 3),
            ("@udc00\"", 3),
            ("@ud83dX\"", 4),
            ("@ud83d@n\"", 4),
            ("@ud83d@u0041\"", 4),
            ("@ud83d@ue000\"", 6),
            ("@n\"", 1),
            ("abc\"", 3),
        ] {
            let end = json(text).len() - 1;
            assert_eq!(decoded(text, width), Ok(end), "{text} within {width}");
            assert_eq!(decoded(text, width - 1), Err(width), "{text} past {width}");
        }
        // An escape with bad hex digits decodes to nothing and its digits are ordinary bytes.
        assert_eq!(decoded("@uzzzz\"", 4), Ok(6));
        assert_eq!(decoded("@uzzzz\"", 3), Err(4));
    }

    #[test]
    fn an_escape_ending_the_input_is_not_charged_past_the_end() {
        assert_eq!(decoded("ab@", 5), Ok(3));
    }

    #[test]
    fn unicode_escapes_decode_to_their_width_and_consumed_length() {
        for (text, expected) in [
            ("@u0041", (1, 4)),
            ("@u007f", (1, 4)),
            ("@u0080", (2, 4)),
            ("@u07ff", (2, 4)),
            ("@u0800", (3, 4)),
            ("@ud7ff", (3, 4)),
            ("@ud800", (3, 4)),
            ("@udbff", (3, 4)),
            ("@udc00", (3, 4)),
            ("@ud800@udc00", (4, 10)),
            ("@udbff@udfff", (4, 10)),
            ("@ud800@udbff", (3, 4)),
            ("@ud800@ue000", (3, 4)),
            ("@ud800Xudc00", (3, 4)),
            ("@ud800@Xdc00", (3, 4)),
            ("@ud800@udc0", (3, 4)),
            ("@uzzzz", (0, 0)),
            ("@u00", (0, 0)),
        ] {
            assert_eq!(decoded_unicode_escape(&json(text), 1), expected, "{text}");
        }
    }

    #[test]
    fn hex_quads_accept_exactly_hexadecimal_digits() {
        assert_eq!(parse_hex_quad(b"0000", 0), Some(0));
        assert_eq!(parse_hex_quad(b"0001", 0), Some(1));
        assert_eq!(parse_hex_quad(b"0009", 0), Some(9));
        assert_eq!(parse_hex_quad(b"000a", 0), Some(10));
        assert_eq!(parse_hex_quad(b"000f", 0), Some(15));
        assert_eq!(parse_hex_quad(b"000A", 0), Some(10));
        assert_eq!(parse_hex_quad(b"000F", 0), Some(15));
        assert_eq!(parse_hex_quad(b"09afAF", 0), Some(0x09af));
        assert_eq!(parse_hex_quad(b"x1234", 1), Some(0x1234));
        assert_eq!(parse_hex_quad(b"ffff", 0), Some(0xffff));
        for bad in [
            &b"000/"[..],
            b"000:",
            b"000g",
            b"000G",
            b"000@",
            b"000`",
            b"00",
        ] {
            assert_eq!(parse_hex_quad(bad, 0), None, "{bad:?}");
        }
        assert_eq!(parse_hex_quad(b"0000", usize::MAX), None);
        assert_eq!(parse_hex_quad(b"0000", 1), None);
    }

    #[test]
    fn array_population_ignores_whitespace_only_items() {
        assert_eq!(array_field_population(br#"{"a": [ ]}"#, b"a"), 0);
        assert_eq!(array_field_population(br#"{"a": [1, ]}"#, b"a"), 1);
        assert_eq!(array_field_population(br#"{"a": [1,  2 , 3]}"#, b"a"), 3);
        assert_eq!(array_field_population(br#"{"a": []}"#, b"a"), 0);
    }
}
