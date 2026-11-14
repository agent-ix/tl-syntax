//! Replays exact tagged wire bytes through the candidate public decoders.
use std::{env, fs, path::PathBuf};

use serde_json::Value;
use tl_mltl::wire::{trace::ValidatedTrace, OwnerLimits};
use tl_parse::{format_document, parse, FormatLimits, ParseLimits, ParseReport};
use tl_rewrite::{
    ConformanceReport, RecordLimits, ReplayReport, ReplayStatus, RewriteReport, RewriteStatus,
};
use tl_syntax::{FormulaDocument, SemanticProfile, SyntaxArtifactLimits};

/// `TL_DECLARED_BREAKS` lists `fixture:field+field` entries, comma separated;
/// `declared-decoder-breaks.toml` documents the proof.
fn declaration(declared: &str, fixture: &str) -> Option<Vec<String>> {
    declared.split(',').find_map(|entry| {
        let (name, fields) = entry.split_once(':')?;
        (name == fixture).then(|| fields.split('+').map(str::to_owned).collect())
    })
}

/// Returns `bytes` without the top-level member `key`, with every other byte,
/// including the key order, unchanged. `None` when the member is absent.
fn strip_member(bytes: &[u8], key: &str) -> Option<Vec<u8>> {
    let needle = format!("\"{key}\":");
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    let mut start = None;
    for (index, &byte) in bytes.iter().enumerate() {
        if in_string {
            match (escaped, byte) {
                (true, _) => escaped = false,
                (false, b'\\') => escaped = true,
                (false, b'"') => in_string = false,
                _ => {}
            }
            continue;
        }
        match byte {
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth -= 1,
            b'"' => {
                let after_separator = index > 0 && matches!(bytes[index - 1], b'{' | b',');
                if depth == 1 && after_separator && bytes[index..].starts_with(needle.as_bytes()) {
                    start = Some(index);
                    break;
                }
                in_string = true;
            }
            _ => {}
        }
    }
    let start = start?;
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    let mut end = start + needle.len();
    while end < bytes.len() {
        let byte = bytes[end];
        if in_string {
            match (escaped, byte) {
                (true, _) => escaped = false,
                (false, b'\\') => escaped = true,
                (false, b'"') => in_string = false,
                _ => {}
            }
        } else {
            match byte {
                b'"' => in_string = true,
                b'{' | b'[' => depth += 1,
                b'}' | b']' if depth == 0 => break,
                b'}' | b']' => depth -= 1,
                b',' if depth == 0 => break,
                _ => {}
            }
        }
        end += 1;
    }
    let (from, to) = if bytes.get(end) == Some(&b',') {
        (start, end + 1)
    } else if bytes[start - 1] == b',' {
        (start - 1, end)
    } else {
        (start, end)
    };
    Some([&bytes[..from], &bytes[to..]].concat())
}

/// Decodes one fixture under its declaration and returns the decoded value and
/// the bytes it was decoded from when the content checks should run.
///
/// Undeclared, the candidate must accept the previous release's bytes, decoded
/// from the raw bytes so duplicate keys stay refused. Declared, the bytes must
/// be refused as they are and must decode once the declared removed fields are
/// stripped from the raw bytes, so the refusal comes from exactly those fields.
fn replay_fixture<T>(
    name: &str,
    bytes: &[u8],
    declaration: Option<&[String]>,
    decode: impl Fn(&[u8]) -> Result<T, String>,
) -> (T, Vec<u8>) {
    let Some(fields) = declaration else {
        let decoded = decode(bytes)
            .unwrap_or_else(|error| panic!("candidate accepts old {name} bytes: {error}"));
        return (decoded, bytes.to_vec());
    };
    assert!(
        decode(bytes).is_err(),
        "declared {name} break: the candidate unexpectedly accepts old {name} bytes"
    );
    let mut stripped = bytes.to_vec();
    for field in fields {
        stripped = strip_member(&stripped, field).unwrap_or_else(|| {
            panic!("declared removed field {field} is absent from the old {name} bytes")
        });
    }
    let decoded = decode(&stripped).unwrap_or_else(|error| {
        panic!("declared {name} break: stripped bytes are still refused: {error}")
    });
    (decoded, stripped)
}

fn main() {
    assert_eq!(
        strip_member(br#"{"a":1,"b":{"b":"x,}\"","c":[2]},"d":3}"#, "b").unwrap(),
        br#"{"a":1,"d":3}"#
    );
    assert_eq!(strip_member(br#"{"a":1,"b":2}"#, "b").unwrap(), br#"{"a":1}"#);
    assert_eq!(strip_member(br#"{"b":2,"a":1}"#, "b").unwrap(), br#"{"a":1}"#);
    assert_eq!(strip_member(br#"{"b":2}"#, "b").unwrap(), br#"{}"#);
    assert!(strip_member(br#"{"a":{"b":2},"c":"\"b\":"}"#, "b").is_none());

    let directory = PathBuf::from(env::var_os("TL_LEGACY_DIR").expect("fixture directory"));
    let declared = env::var("TL_DECLARED_BREAKS").unwrap_or_default();
    let syntax = fs::read(directory.join("syntax.json")).unwrap();
    let formula = FormulaDocument::from_json_bytes(&syntax, SyntaxArtifactLimits::default())
        .expect("candidate accepts old formula-v1 bytes");
    assert_eq!(formula.schema_version().as_str(), "tl-syntax.formula/v1");
    let mut damaged = syntax.clone();
    damaged.push(b'!');
    assert!(FormulaDocument::from_json_bytes(&damaged, SyntaxArtifactLimits::default()).is_err());

    let parser_bytes = fs::read(directory.join("parse.json")).unwrap();
    let (old, used) = replay_fixture(
        "parse-report",
        &parser_bytes,
        declaration(&declared, "parse").as_deref(),
        |bytes| serde_json::from_slice::<ParseReport>(bytes).map_err(|error| error.to_string()),
    );
    assert_eq!(old.schema_version, "tl-parse.diagnostics/v1");
    assert!(old.diagnostics.is_empty());
    assert_eq!(old.document.as_ref(), Some(&formula));
    let mut wrong_parse: Value = serde_json::from_slice(&used).unwrap();
    wrong_parse["document"] = Value::Bool(true);
    assert!(serde_json::from_value::<ParseReport>(wrong_parse).is_err());
    let current = parse("p0", SemanticProfile::ClosedTraceV1, ParseLimits::default());
    assert!(current.diagnostics.is_empty());
    assert_eq!(current.document.as_ref(), Some(&formula));
    assert_eq!(format_document(&formula, FormatLimits::default()).text.as_deref(), Some("p0"));

    let trace = fs::read(directory.join("mltl.json")).unwrap();
    let decoded = ValidatedTrace::from_json_bytes(&trace, OwnerLimits::default())
        .expect("candidate accepts old trace-v1 bytes");
    assert_eq!(decoded.document().trace_id, "legacy-one");
    assert_eq!(decoded.canonical_json_bytes(), trace);
    let mut damaged = trace.clone();
    damaged.push(b'!');
    assert!(ValidatedTrace::from_json_bytes(&damaged, OwnerLimits::default()).is_err());

    let rewrite = fs::read(directory.join("rewrite.json")).unwrap();
    let (decoded, used) = replay_fixture(
        "rewrite-report",
        &rewrite,
        declaration(&declared, "rewrite").as_deref(),
        |bytes| {
            RewriteReport::from_json_bytes(bytes, RecordLimits::default())
                .map_err(|error| error.to_string())
        },
    );
    assert_eq!(decoded.schema_version, "tl-rewrite.report/v1");
    assert_eq!(decoded.status, RewriteStatus::Unchanged);
    assert_eq!(decoded.output.as_ref(), Some(&formula));
    let mut damaged = used;
    damaged.push(b'!');
    assert!(RewriteReport::from_json_bytes(&damaged, RecordLimits::default()).is_err());

    let conformance = fs::read(directory.join("conformance.json")).unwrap();
    let (old, used) = replay_fixture(
        "conformance-report",
        &conformance,
        declaration(&declared, "conformance").as_deref(),
        |bytes| {
            serde_json::from_slice::<ConformanceReport>(bytes).map_err(|error| error.to_string())
        },
    );
    assert_eq!(old.schema_version, "tl-rewrite.conformance/v1");
    assert_eq!(old.comparison_id, "legacy-cmp");
    assert_eq!(old.semantic_profile, "mltl.closed-trace/v1");
    assert_eq!(old.original_sha256, old.rewritten_sha256);
    // Every decoded field must equal the bytes it came from.
    let source: Value = serde_json::from_slice(&used).unwrap();
    assert_eq!(serde_json::to_value(&old).unwrap(), source);
    let mut wrong = source;
    wrong["schemaVersion"] = Value::Bool(true);
    assert!(serde_json::from_value::<ConformanceReport>(wrong).is_err());

    // No declared break covers the replay report: it must stay accepted.
    let replay = fs::read(directory.join("replay.json")).unwrap();
    let decoded = ReplayReport::from_json_bytes(&replay, RecordLimits::default())
        .expect("candidate accepts old replay-report-v1 bytes");
    assert_eq!(decoded.schema_version, "tl-rewrite.replay/v1");
    assert_eq!(decoded.status, ReplayStatus::Verified);
    assert_eq!(decoded.expected_report_sha256, decoded.observed_report_sha256);
    let mut damaged = replay.clone();
    damaged.push(b'!');
    assert!(ReplayReport::from_json_bytes(&damaged, RecordLimits::default()).is_err());
}
