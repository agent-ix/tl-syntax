//! Replays exact tagged wire bytes through the candidate public decoders.
use std::{env, fs, path::PathBuf};

use serde_json::Value;
use tl_mltl::wire::{trace::ValidatedTrace, OwnerLimits};
use tl_parse::{format_document, parse, FormatLimits, ParseLimits, ParseReport};
use tl_rewrite::{
    ConformanceReport, RecordLimits, ReplayReport, ReplayStatus, RewriteReport, RewriteStatus,
};
use tl_syntax::{FormulaDocument, SemanticProfile, SyntaxArtifactLimits};

/// One declared decoder break read from `TL_DECLARED_BREAKS`.
struct Declaration {
    proof: String,
    fields: Vec<String>,
}

/// `TL_DECLARED_BREAKS` lists `fixture:proof:field+field` entries, comma
/// separated; `declared-decoder-breaks.toml` documents the proofs.
fn declaration(declared: &str, fixture: &str) -> Option<Declaration> {
    declared.split(',').find_map(|entry| {
        let mut parts = entry.splitn(3, ':');
        if parts.next()? != fixture {
            return None;
        }
        let proof = parts.next()?.to_owned();
        let fields = parts.next()?.split('+').map(str::to_owned).collect();
        Some(Declaration { proof, fields })
    })
}

/// Decodes one fixture under its declaration and returns the decoded value and
/// the JSON it was decoded from when the content checks should run.
///
/// Undeclared, the candidate must accept the previous release's bytes, decoded
/// from the raw bytes so duplicate keys stay refused. `strip_decode` requires
/// the bytes to be refused as they are and to decode once the declared removed
/// fields are stripped, so the refusal comes from exactly those fields.
/// `refused_names_field` requires the refusal message to name the single
/// declared field and runs no content checks.
fn replay_fixture<T>(
    name: &str,
    bytes: &[u8],
    declaration: Option<&Declaration>,
    decode: impl Fn(&[u8]) -> Result<T, String>,
) -> Option<(T, Value)> {
    let json = || serde_json::from_slice::<Value>(bytes).unwrap();
    let Some(declaration) = declaration else {
        let decoded =
            decode(bytes).unwrap_or_else(|error| panic!("candidate accepts old {name} bytes: {error}"));
        return Some((decoded, json()));
    };
    match declaration.proof.as_str() {
        "refused_names_field" => {
            let [field] = declaration.fields.as_slice() else {
                panic!("declared {name} break: refused_names_field names exactly one field");
            };
            let Err(message) = decode(bytes) else {
                panic!("declared {name} break: the candidate unexpectedly accepts old {name} bytes");
            };
            assert!(
                message.contains(field.as_str()),
                "declared {name} break: the refusal does not name {field}: {message}"
            );
            None
        }
        "strip_decode" => {
            assert!(
                decode(bytes).is_err(),
                "declared {name} break: the candidate unexpectedly accepts old {name} bytes"
            );
            let mut json = json();
            for field in &declaration.fields {
                let removed = json.as_object_mut().and_then(|object| object.remove(field));
                assert!(
                    removed.is_some(),
                    "declared removed field {field} is absent from the old {name} bytes"
                );
            }
            let stripped = serde_json::to_vec(&json).unwrap();
            let decoded = decode(&stripped).unwrap_or_else(|error| {
                panic!("declared {name} break: stripped bytes are still refused: {error}")
            });
            Some((decoded, json))
        }
        other => panic!("declared {name} break has unknown proof {other}"),
    }
}

fn main() {
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
    let decoded = replay_fixture(
        "parse-report",
        &parser_bytes,
        declaration(&declared, "parse").as_ref(),
        |bytes| serde_json::from_slice::<ParseReport>(bytes).map_err(|error| error.to_string()),
    );
    if let Some((old, json)) = decoded {
        assert_eq!(old.schema_version, "tl-parse.diagnostics/v1");
        assert!(old.diagnostics.is_empty());
        assert_eq!(old.document.as_ref(), Some(&formula));
        let mut wrong_parse = json;
        wrong_parse["document"] = Value::Bool(true);
        assert!(serde_json::from_value::<ParseReport>(wrong_parse).is_err());
    }
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
    let rewrite_declaration = declaration(&declared, "rewrite");
    let decoded = replay_fixture("rewrite-report", &rewrite, rewrite_declaration.as_ref(), |bytes| {
        RewriteReport::from_json_bytes(bytes, RecordLimits::default())
            .map_err(|error| error.to_string())
    });
    if let Some((decoded, _)) = decoded {
        assert_eq!(decoded.schema_version, "tl-rewrite.report/v1");
        assert_eq!(decoded.status, RewriteStatus::Unchanged);
        assert_eq!(decoded.output.as_ref(), Some(&formula));
    }
    if rewrite_declaration.is_none() {
        let mut damaged = rewrite.clone();
        damaged.push(b'!');
        assert!(RewriteReport::from_json_bytes(&damaged, RecordLimits::default()).is_err());
    }

    let conformance = fs::read(directory.join("conformance.json")).unwrap();
    let decoded = replay_fixture(
        "conformance-report",
        &conformance,
        declaration(&declared, "conformance").as_ref(),
        |bytes| {
            serde_json::from_slice::<ConformanceReport>(bytes).map_err(|error| error.to_string())
        },
    );
    if let Some((old, json)) = decoded {
        assert_eq!(old.schema_version, "tl-rewrite.conformance/v1");
        assert_eq!(old.comparison_id, "legacy-cmp");
        assert_eq!(old.semantic_profile, "mltl.closed-trace/v1");
        assert_eq!(old.original_sha256, old.rewritten_sha256);
        let mut wrong = json;
        wrong["schemaVersion"] = Value::Bool(true);
        assert!(serde_json::from_value::<ConformanceReport>(wrong).is_err());
    }

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
