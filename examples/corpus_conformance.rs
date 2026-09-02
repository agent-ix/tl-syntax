//! Replay the shared temporal corpus through the real crate and emit structured results.
//!
//! Four things this example deliberately is not.
//!
//! It is not a second implementation of MLTL. It decodes with `FormulaDocument`'s
//! own `Deserialize`, validates with `Formula::new`, and classifies rejections by
//! matching the crate's public typed errors. It never inspects an error message,
//! because a reason recovered from a string is a reason that changes when the
//! wording does.
//!
//! It is not an oracle for horizons or closed-trace outcomes. This crate owns no
//! evaluator; that is `tl-mltl`'s job. `scripts/validate_corpus.py` derives those
//! two values and states its own limitation. What this runner owns is the half a
//! Python re-implementation could never own honestly: whether the real decoder
//! accepts the fixture, and for exactly which typed reason it does not.
//!
//! It is not a verdict. It writes one JSON object per line to stdout and exits 0
//! if every fixture matched its declared expectation and 1 if one did not. It
//! retains nothing, digests nothing, and attests to nothing.
//!
//! It is not a Quire or Quoin client. Nothing in this file, and nothing in the
//! published crate, links either of them.
//!
//! Run it with:
//!
//! ```text
//! cargo run --quiet --example corpus_conformance --features serde -- \
//!     --manifest corpus/manifest.json
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{json, Map, Value};
use tl_syntax::{
    Formula, FormulaDocument, FormulaError, Interval, Node, NodeId, PropositionMapDocument,
    SemanticProfile, CORPUS_REVISION, MAX_FORMULA_DOCUMENT_NODES,
};

/// The stream identity a consumer matches on before reading a single row.
const PROTOCOL: &str = "tl-syntax.corpus-conformance/v1";

/// How a fixture was rejected, named by the typed error that rejected it.
///
/// These identifiers are the corpus manifest's `expected_error` vocabulary. Each
/// is produced by one public API refusing, not by reading what it said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rejection {
    UnsupportedSemanticProfile,
    IntervalInverted,
    NodeDecodeRejected,
    OperandNotPreceding,
    RootOutOfRange,
    DocumentNodeLimitExceeded,
    TooManyNodes,
}

impl Rejection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedSemanticProfile => "unsupported_semantic_profile",
            Self::IntervalInverted => "interval_inverted",
            Self::NodeDecodeRejected => "node_decode_rejected",
            Self::OperandNotPreceding => "operand_not_preceding",
            Self::RootOutOfRange => "root_out_of_range",
            Self::DocumentNodeLimitExceeded => "document_node_limit_exceeded",
            Self::TooManyNodes => "too_many_nodes",
        }
    }

    const fn from_formula_error(error: &FormulaError) -> Self {
        match error {
            FormulaError::OperandNotPreceding { .. } => Self::OperandNotPreceding,
            FormulaError::RootOutOfRange { .. } => Self::RootOutOfRange,
            FormulaError::DocumentNodeLimitExceeded { .. } => Self::DocumentNodeLimitExceeded,
            FormulaError::TooManyNodes { .. } => Self::TooManyNodes,
            // FormulaError is #[non_exhaustive]: a variant added upstream must
            // land here as an explicit unknown rather than be folded into one of
            // the classes above, which would silently widen a declared reason.
            _ => Self::NodeDecodeRejected,
        }
    }
}

/// The classification of one fixture, and whether the full public decode agreed.
struct Classification {
    accepted: bool,
    rejection: Option<Rejection>,
    /// `FormulaDocument`'s own end-to-end decode, run independently of the staged
    /// probes above. If the staged classification and the real decoder disagree,
    /// the staged classification is describing a decoder that does not exist.
    document_decode_accepted: bool,
    node_count: usize,
}

/// Classify a formula document using only the crate's public typed API.
///
/// The stages mirror the order the real decoder refuses in, and each stage's
/// verdict comes from a public constructor returning `Err`, never from text.
fn classify(raw: &str) -> Result<Classification, String> {
    let value: Value = serde_json::from_str(raw).map_err(|error| error.to_string())?;
    let object = value
        .as_object()
        .ok_or_else(|| "formula document must be a JSON object".to_string())?;

    let document_decode_accepted = serde_json::from_str::<FormulaDocument>(raw).is_ok();

    // Stage 1: the semantic profile. An unknown profile is refused before any
    // structural question is asked, because a formula whose semantics are unknown
    // has no structure worth validating.
    let profile_value = object
        .get("semantic_profile")
        .cloned()
        .unwrap_or(Value::Null);
    let profile: SemanticProfile = match serde_json::from_value(profile_value) {
        Ok(profile) => profile,
        Err(_) => {
            return Ok(Classification {
                accepted: false,
                rejection: Some(Rejection::UnsupportedSemanticProfile),
                document_decode_accepted,
                node_count: 0,
            })
        }
    };

    let node_values = object
        .get("nodes")
        .and_then(Value::as_array)
        .ok_or_else(|| "formula document /nodes must be an array".to_string())?;

    // Stage 2: intervals, individually. `Interval`'s own `Deserialize` is what
    // refuses an inverted bound, so an inverted interval is discovered by asking
    // the type that owns the invariant rather than by comparing two numbers here.
    for node in node_values {
        if let Some(interval) = node.get("interval") {
            if serde_json::from_value::<Interval>(interval.clone()).is_err() {
                return Ok(Classification {
                    accepted: false,
                    rejection: Some(Rejection::IntervalInverted),
                    document_decode_accepted,
                    node_count: node_values.len(),
                });
            }
        }
    }

    // Stage 3: the node table. Unknown kinds, unknown fields and malformed spans
    // are all refused here, by `Node`'s hand-written decoder.
    let nodes: Vec<Node> = match serde_json::from_value(Value::Array(node_values.clone())) {
        Ok(nodes) => nodes,
        Err(_) => {
            return Ok(Classification {
                accepted: false,
                rejection: Some(Rejection::NodeDecodeRejected),
                document_decode_accepted,
                node_count: node_values.len(),
            })
        }
    };

    if nodes.len() > MAX_FORMULA_DOCUMENT_NODES {
        return Ok(Classification {
            accepted: false,
            rejection: Some(Rejection::DocumentNodeLimitExceeded),
            document_decode_accepted,
            node_count: nodes.len(),
        });
    }

    // Stage 4: the graph. `Formula::new` returns a typed `FormulaError`, which is
    // matched on as a value.
    let root: NodeId = serde_json::from_value(object.get("root").cloned().unwrap_or(Value::Null))
        .map_err(|error| error.to_string())?;
    match Formula::new(profile, root, &nodes) {
        Ok(_) => Ok(Classification {
            accepted: true,
            rejection: None,
            document_decode_accepted,
            node_count: nodes.len(),
        }),
        Err(error) => Ok(Classification {
            accepted: false,
            rejection: Some(Rejection::from_formula_error(&error)),
            document_decode_accepted,
            node_count: nodes.len(),
        }),
    }
}

fn row(fixture: &str, check: &str, outcome: &str, trace_ids: &[&str], detail: Value) -> String {
    let entry = json!({
        "protocol": PROTOCOL,
        "corpus_revision": CORPUS_REVISION,
        "fixture": fixture,
        "check": check,
        "symbol": format!("corpus::{fixture}::{check}"),
        "outcome": outcome,
        "traceIds": trace_ids,
        "detail": detail,
    });
    entry.to_string()
}

fn manifest_path(arguments: &[String]) -> Result<PathBuf, String> {
    let mut iterator = arguments.iter();
    while let Some(argument) = iterator.next() {
        if argument == "--manifest" {
            return iterator
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| "--manifest requires a path".to_string());
        }
    }
    Err("usage: corpus_conformance --manifest corpus/manifest.json".to_string())
}

fn run() -> Result<(Vec<String>, usize), String> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let manifest_file = manifest_path(&arguments)?;
    let root = manifest_file
        .parent()
        .ok_or_else(|| "manifest has no parent directory".to_string())?
        .to_path_buf();
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_file).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;

    let declared_revision = manifest
        .get("corpus_revision")
        .and_then(Value::as_str)
        .ok_or_else(|| "manifest has no corpus_revision".to_string())?;

    let mut rows = Vec::new();
    let mut failures = 0usize;

    // The revision the crate publishes and the revision the manifest declares are
    // one identity in two places. Consumers pin it, so a silent divergence would
    // make every downstream conformance claim name a corpus that does not exist.
    let revision_matches = declared_revision == CORPUS_REVISION;
    if !revision_matches {
        failures += 1;
    }
    rows.push(row(
        "corpus",
        "revision_identity",
        if revision_matches { "pass" } else { "fail" },
        &["FR-005-AC-3", "NFR-002-AC-2"],
        json!({ "declared": declared_revision, "crate": CORPUS_REVISION }),
    ));

    // The proposition map is part of the corpus contract and is decoded by the
    // real crate for the same reason the formulas are.
    let map_relative = manifest
        .get("proposition_map")
        .and_then(Value::as_str)
        .ok_or_else(|| "manifest has no proposition_map".to_string())?;
    let map_raw = std::fs::read_to_string(root.join(map_relative))
        .map_err(|error| format!("{map_relative}: {error}"))?;
    let map_accepted = serde_json::from_str::<PropositionMapDocument>(&map_raw).is_ok();
    if !map_accepted {
        failures += 1;
    }
    rows.push(row(
        "proposition-map",
        "decode",
        if map_accepted { "pass" } else { "fail" },
        &["FR-003-AC-1", "FR-004-AC-1"],
        json!({ "path": map_relative }),
    ));

    let fixtures = manifest
        .get("fixtures")
        .and_then(Value::as_array)
        .ok_or_else(|| "manifest has no fixtures array".to_string())?;

    let mut classes: BTreeMap<String, usize> = BTreeMap::new();

    for fixture in fixtures {
        let entry: &Map<String, Value> = fixture
            .as_object()
            .ok_or_else(|| "each fixture must be an object".to_string())?;
        let id = entry
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| "fixture has no id".to_string())?;
        let class = entry.get("class").and_then(Value::as_str).unwrap_or("");
        *classes.entry(class.to_string()).or_insert(0) += 1;
        let relative = entry
            .get("formula")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{id}: fixture has no formula path"))?;
        let expected_validation = entry
            .get("expected_validation")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{id}: fixture has no expected_validation"))?;

        let path: &Path = &root.join(relative);
        let raw = match std::fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(error) => {
                failures += 1;
                rows.push(row(
                    id,
                    "decode",
                    "unavailable",
                    &["FR-005-AC-1"],
                    json!({ "path": relative, "error": error.to_string() }),
                ));
                continue;
            }
        };

        let classification = match classify(&raw) {
            Ok(classification) => classification,
            Err(error) => {
                failures += 1;
                rows.push(row(
                    id,
                    "decode",
                    "malformed",
                    &["FR-005-AC-1"],
                    json!({ "path": relative, "error": error }),
                ));
                continue;
            }
        };

        // Check 1: accept or reject, as the manifest declares.
        let expected_accept = expected_validation == "valid";
        let validation_ok = classification.accepted == expected_accept;
        if !validation_ok {
            failures += 1;
        }
        rows.push(row(
            id,
            "validation",
            if validation_ok { "pass" } else { "fail" },
            &["FR-005-AC-1", "FR-002-AC-2"],
            json!({
                "expected": expected_validation,
                "observed": if classification.accepted { "valid" } else { "invalid" },
                "node_count": classification.node_count,
            }),
        ));

        // Check 2: the staged classification and the real end-to-end decoder must
        // agree. Without this, the staged probes could drift into describing a
        // decoder this crate does not ship.
        let decoder_agrees = classification.document_decode_accepted == classification.accepted;
        if !decoder_agrees {
            failures += 1;
        }
        rows.push(row(
            id,
            "decoder_agreement",
            if decoder_agrees { "pass" } else { "fail" },
            &["FR-004-AC-1", "NFR-002-AC-1"],
            json!({
                "staged": classification.accepted,
                "document_decode": classification.document_decode_accepted,
            }),
        ));

        // Check 3: a rejected fixture must be rejected for the declared reason.
        // A fixture that fails for a different reason than the corpus says is a
        // fixture that is no longer testing what it was published to test.
        match entry.get("expected_error").and_then(Value::as_str) {
            Some(expected_error) => {
                let observed = classification.rejection.map(Rejection::as_str);
                let reason_ok = observed == Some(expected_error);
                if !reason_ok {
                    failures += 1;
                }
                rows.push(row(
                    id,
                    "rejection_reason",
                    if reason_ok { "pass" } else { "fail" },
                    &["FR-005-AC-2"],
                    json!({ "expected": expected_error, "observed": observed }),
                ));
            }
            None if !expected_accept => {
                failures += 1;
                rows.push(row(
                    id,
                    "rejection_reason",
                    "not-computed",
                    &["FR-005-AC-2"],
                    json!({
                        "why": "the manifest declares this fixture invalid and names no expected_error, so no reason could be checked",
                    }),
                ));
            }
            None => {}
        }
    }

    // A conformance stream over zero fixtures, or over only valid ones, would
    // report a clean run while proving nothing about refusal. The corpus contract
    // requires both, so the runner states the census rather than leaving a reader
    // to count rows.
    let has_invalid = fixtures.iter().any(|fixture| {
        fixture.get("expected_validation").and_then(Value::as_str) == Some("invalid")
    });
    let has_valid = fixtures
        .iter()
        .any(|fixture| fixture.get("expected_validation").and_then(Value::as_str) == Some("valid"));
    let census_ok = !fixtures.is_empty() && has_invalid && has_valid;
    if !census_ok {
        failures += 1;
    }
    rows.push(row(
        "corpus",
        "census",
        if census_ok { "pass" } else { "vacuous" },
        &["FR-005-AC-1"],
        json!({
            "fixtures": fixtures.len(),
            "classes": classes,
            "has_valid": has_valid,
            "has_invalid": has_invalid,
        }),
    ));

    Ok((rows, failures))
}

fn main() -> ExitCode {
    match run() {
        Ok((rows, failures)) => {
            for line in &rows {
                println!("{line}");
            }
            if failures == 0 {
                ExitCode::SUCCESS
            } else {
                eprintln!("{failures} corpus conformance check(s) did not match the manifest");
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("corpus conformance could not run: {error}");
            // 2 is reserved for "the run did not happen", which is a different
            // fact from "the run happened and something did not match".
            ExitCode::from(2)
        }
    }
}
