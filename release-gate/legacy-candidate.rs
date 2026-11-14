//! Replays exact tagged wire bytes through the candidate public decoders.
use std::{env, fs, path::PathBuf};

use tl_mltl::wire::{trace::ValidatedTrace, OwnerLimits};
use tl_parse::{format_document, parse, FormatLimits, ParseLimits, ParseReport};
use tl_rewrite::{RecordLimits, RewriteReport, RewriteStatus};
use tl_syntax::{FormulaDocument, SemanticProfile, SyntaxArtifactLimits};

fn main() {
    let directory = PathBuf::from(env::var_os("TL_LEGACY_DIR").expect("fixture directory"));
    let syntax = fs::read(directory.join("syntax.json")).unwrap();
    let formula = FormulaDocument::from_json_bytes(&syntax, SyntaxArtifactLimits::default())
        .expect("candidate accepts old formula-v1 bytes");
    assert_eq!(formula.schema_version().as_str(), "tl-syntax.formula/v1");
    let mut damaged = syntax.clone();
    damaged.push(b'!');
    assert!(FormulaDocument::from_json_bytes(&damaged, SyntaxArtifactLimits::default()).is_err());

    // `TL_DECLARED_BREAKS` lists `fixture:removed_field` pairs.
    let declared = env::var("TL_DECLARED_BREAKS").unwrap_or_default();
    let removed_field = declared
        .split(',')
        .find_map(|pair| pair.strip_prefix("parse:"));
    let parser_bytes = fs::read(directory.join("parse.json")).unwrap();
    let mut parser_json: serde_json::Value = serde_json::from_slice(&parser_bytes).unwrap();
    if let Some(field) = removed_field {
        // A declared break: the candidate must refuse the previous release's bytes
        // as they are, and the refusal must come from exactly the declared
        // removed field, so the same bytes without that key still decode.
        assert!(
            serde_json::from_slice::<ParseReport>(&parser_bytes).is_err(),
            "declared parse-report break: the candidate unexpectedly accepts old parse-report bytes"
        );
        let removed = parser_json
            .as_object_mut()
            .and_then(|object| object.remove(field));
        assert!(
            removed.is_some(),
            "declared removed field {field} is absent from the old parse-report bytes"
        );
    }
    let old: ParseReport = serde_json::from_value(parser_json.clone())
        .expect("candidate accepts old parse-report bytes");
    assert_eq!(old.schema_version, "tl-parse.diagnostics/v1");
    assert!(old.diagnostics.is_empty());
    assert_eq!(old.document.as_ref(), Some(&formula));
    let mut wrong_parse = parser_json;
    wrong_parse["document"] = serde_json::Value::Bool(true);
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
    let decoded = RewriteReport::from_json_bytes(&rewrite, RecordLimits::default())
        .expect("candidate accepts old rewrite-report-v1 bytes");
    assert_eq!(decoded.schema_version, "tl-rewrite.report/v1");
    assert_eq!(decoded.status, RewriteStatus::Unchanged);
    assert_eq!(decoded.output.as_ref(), Some(&formula));
    let mut damaged = rewrite.clone();
    damaged.push(b'!');
    assert!(RewriteReport::from_json_bytes(&damaged, RecordLimits::default()).is_err());
}
