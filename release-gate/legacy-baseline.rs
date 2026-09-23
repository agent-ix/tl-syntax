//! Executed against the four preceding immutable tags, in an external project.
use std::{env, fs, path::PathBuf};

use tl_mltl::{TraceDocument, TraceSchemaVersion};
use tl_parse::{parse, report_json, ParseLimits};
use tl_rewrite::{rewrite, RewriteOptions, RewriteStatus};
use tl_syntax::{PropositionId, SemanticProfile};

fn main() {
    let directory = PathBuf::from(env::var_os("TL_LEGACY_DIR").expect("fixture directory"));
    let parsed = parse("p0", SemanticProfile::ClosedTraceV1, ParseLimits::default());
    let formula = parsed.document.as_ref().expect("tagged parser accepts p0");
    assert!(parsed.diagnostics.is_empty());
    fs::write(directory.join("syntax.json"), serde_json::to_vec(formula).unwrap()).unwrap();
    fs::write(directory.join("parse.json"), report_json(&parsed).unwrap()).unwrap();
    let trace = TraceDocument {
        schema_version: TraceSchemaVersion::V1,
        trace_id: "legacy-one".to_owned(),
        closed: true,
        instants: vec![vec![PropositionId(0)]],
    };
    fs::write(directory.join("mltl.json"), serde_json::to_vec(&trace).unwrap()).unwrap();
    let rewritten = rewrite(formula, "legacy-one", RewriteOptions::default(), "v0.3.0");
    assert_eq!(rewritten.status, RewriteStatus::Unchanged);
    fs::write(directory.join("rewrite.json"), serde_json::to_vec(&rewritten).unwrap()).unwrap();
}
