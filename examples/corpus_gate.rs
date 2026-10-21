//! Validate the checked-in corpus and derive its declared oracles independently.

use std::{
    collections::{BTreeSet, HashMap},
    env, fs,
    path::{Path, PathBuf},
};

use jsonschema::{Draft, JSONSchema};
use serde_json::{json, Value};
use tl_syntax::{FormulaDocument, Node, NodeKind, PropositionMapDocument};

const KINDS: [&str; 12] = [
    "false",
    "true",
    "proposition",
    "not",
    "and",
    "or",
    "implies",
    "equivalent",
    "future",
    "globally",
    "until",
    "release",
];

fn read_json(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn corpus_file(corpus: &Path, relative: &str) -> Result<PathBuf, String> {
    let base = corpus.canonicalize().map_err(|error| error.to_string())?;
    let path = corpus
        .join(relative)
        .canonicalize()
        .map_err(|error| error.to_string())?;
    if !path.starts_with(base) {
        return Err(format!("fixture escapes corpus: {relative}"));
    }
    Ok(path)
}

fn schema_contract(corpus: &Path) -> Result<(JSONSchema, JSONSchema), String> {
    let schema = read_json(&corpus.join("schema/formula-v1.schema.json"))?;
    let formula_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .map_err(|error| format!("formula schema: {error}"))?;
    let validator = schema["x-tl-syntax-semantic-validator"].as_str();
    if validator != Some("cargo run --quiet --example corpus_gate --features serde --") {
        return Err("formula schema does not declare its mandatory Rust semantic validator".into());
    }
    let constraints = [
        "interval start must not exceed end",
        "span start must not exceed end",
        "root must index an existing node",
        "every operand must reference a preceding node",
    ];
    if schema["x-tl-syntax-semantic-constraints"] != json!(constraints) {
        return Err("formula schema semantic constraint declaration drifted".into());
    }
    if schema["properties"]["nodes"]["maxItems"] != 100_000 {
        return Err("formula schema wire node limit drifted".into());
    }
    let variants = schema["$defs"]["node"]["oneOf"]
        .as_array()
        .ok_or("formula schema has no node variants")?;
    let mut observed = BTreeSet::new();
    for variant in variants {
        let kind = &variant["properties"]["kind"];
        if let Some(value) = kind["const"].as_str() {
            observed.insert(value.to_owned());
        }
        if let Some(values) = kind["enum"].as_array() {
            for value in values {
                observed.insert(value.as_str().ok_or("non-string kind")?.to_owned());
            }
        }
    }
    if observed != KINDS.into_iter().map(str::to_owned).collect() {
        return Err("formula schema operator vocabulary drifted".into());
    }
    let proposition_schema = read_json(&corpus.join("schema/proposition-map-v1.schema.json"))?;
    let proposition_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&proposition_schema)
        .map_err(|error| format!("proposition schema: {error}"))?;
    if proposition_schema["properties"]["schema_version"]["const"] != "tl-syntax.proposition-map/v1"
    {
        return Err("proposition map schema version drifted".into());
    }
    for kind in KINDS {
        let mut node = json!({"kind": kind});
        match kind {
            "proposition" => node["proposition"] = json!(0),
            "not" | "future" | "globally" => node["operand"] = json!(0),
            "and" | "or" | "implies" | "equivalent" | "until" | "release" => {
                node["left"] = json!(0);
                node["right"] = json!(1);
            }
            _ => {}
        }
        if matches!(kind, "future" | "globally" | "until" | "release") {
            node["interval"] = json!({"start": 0, "end": 1});
        }
        if !formula_validator.is_valid(&json!({"schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": [node]}))
        {
            return Err(format!("formula schema rejects supported operator {kind}"));
        }
    }
    for invalid in [
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": []}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "unknown", "root": 0, "nodes": [{"kind": "true"}]}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": -1, "nodes": [{"kind": "true"}]}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": [{"kind": "true", "FABRICATED": "x"}]}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": [{"kind": "until"}]}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": [{"kind": "future", "operand": 0, "interval": {"start": 0, "end": 1, "FABRICATED": 1}}]}),
        json!({"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": 0, "nodes": [{"kind": "true", "span": {"start": 0, "end": 1, "JUNK": 1}}]}),
    ] {
        if formula_validator.is_valid(&invalid) {
            return Err("formula schema accepted a required constraint probe".into());
        }
    }
    Ok((formula_validator, proposition_validator))
}

fn semantic_error(value: &Value, schema: &JSONSchema) -> Option<&'static str> {
    if !matches!(
        value["semantic_profile"].as_str(),
        Some("mltl.closed-trace/v1" | "mltl.online-prefix/v1")
    ) {
        return Some("unsupported_semantic_profile");
    }
    if !schema.is_valid(value) {
        return Some("schema_error");
    }
    let nodes = value["nodes"].as_array()?;
    for node in nodes {
        for (field, reason) in [("interval", "interval_inverted"), ("span", "span_inverted")] {
            if let Some(bounds) = node.get(field) {
                if bounds["start"].as_u64()? > bounds["end"].as_u64()? {
                    return Some(reason);
                }
            }
        }
    }
    if value["root"].as_u64()? >= nodes.len() as u64 {
        return Some("root_out_of_range");
    }
    for (index, node) in nodes.iter().enumerate() {
        let fields: &[&str] = match node["kind"].as_str()? {
            "not" | "future" | "globally" => &["operand"],
            "and" | "or" | "implies" | "equivalent" | "until" | "release" => &["left", "right"],
            _ => &[],
        };
        for field in fields {
            if node[*field].as_u64()? >= index as u64 {
                return Some("operand_not_preceding");
            }
        }
    }
    if serde_json::from_value::<FormulaDocument>(value.clone()).is_err() {
        return Some("schema_error");
    }
    None
}

fn child(horizons: &[u64], id: tl_syntax::NodeId) -> Result<u64, String> {
    horizons
        .get(id.0 as usize)
        .copied()
        .ok_or("operand is not preceding".into())
}

fn horizon(document: &FormulaDocument) -> Result<u64, String> {
    let mut horizons = Vec::with_capacity(document.nodes().len());
    for node in document.nodes() {
        let (offset, base) = match node.kind {
            NodeKind::False | NodeKind::True | NodeKind::Proposition { .. } => (0, 0),
            NodeKind::Not { operand } => (0, child(&horizons, operand)?),
            NodeKind::And { left, right }
            | NodeKind::Or { left, right }
            | NodeKind::Implies { left, right }
            | NodeKind::Equivalent { left, right } => {
                (0, child(&horizons, left)?.max(child(&horizons, right)?))
            }
            NodeKind::Future { interval, operand } | NodeKind::Globally { interval, operand } => {
                (u64::from(interval.end()), child(&horizons, operand)?)
            }
            NodeKind::Until {
                interval,
                left,
                right,
            }
            | NodeKind::Release {
                interval,
                left,
                right,
            } => (
                u64::from(interval.end()),
                child(&horizons, left)?.max(child(&horizons, right)?),
            ),
            _ => return Err("v1 corpus contains an unsupported operator".into()),
        };
        horizons.push(offset.checked_add(base).ok_or("horizon exceeds u64")?);
    }
    child(&horizons, document.root())
}

struct Evaluation<'a> {
    nodes: &'a [Node],
    trace: &'a [Vec<u32>],
    memo: HashMap<(usize, u64), bool>,
}

impl Evaluation<'_> {
    fn instants(&self, start: u64, end: u64) -> Vec<u64> {
        if start > end {
            return Vec::new();
        }
        let trace_end = self.trace.len() as u64;
        let mut instants = (start..=end.min(trace_end.saturating_sub(1)))
            .filter(|_| trace_end > 0)
            .collect::<Vec<_>>();
        let outside = start.max(trace_end);
        if outside <= end {
            instants.push(outside);
        }
        instants
    }

    fn prefix(&self, instant: u64, witness: u64) -> Vec<u64> {
        if witness <= instant {
            Vec::new()
        } else {
            self.instants(instant, witness - 1)
        }
    }

    fn at(&mut self, id: usize, instant: u64) -> Result<bool, String> {
        let key = (id, instant.min(self.trace.len() as u64));
        if let Some(value) = self.memo.get(&key) {
            return Ok(*value);
        }
        let kind = self.nodes.get(id).ok_or("invalid oracle operand")?.kind;
        let one = |id: tl_syntax::NodeId| id.0 as usize;
        let value = match kind {
            NodeKind::False => false,
            NodeKind::True => true,
            NodeKind::Proposition { proposition } => usize::try_from(instant)
                .ok()
                .and_then(|index| self.trace.get(index))
                .is_some_and(|values| values.contains(&proposition.0)),
            NodeKind::Not { operand } => !self.at(one(operand), instant)?,
            NodeKind::And { left, right } => {
                self.at(one(left), instant)? && self.at(one(right), instant)?
            }
            NodeKind::Or { left, right } => {
                self.at(one(left), instant)? || self.at(one(right), instant)?
            }
            NodeKind::Implies { left, right } => {
                !self.at(one(left), instant)? || self.at(one(right), instant)?
            }
            NodeKind::Equivalent { left, right } => {
                self.at(one(left), instant)? == self.at(one(right), instant)?
            }
            NodeKind::Future { interval, operand } => self
                .instants(
                    instant + u64::from(interval.start()),
                    instant + u64::from(interval.end()),
                )
                .into_iter()
                .try_fold(false, |found, at| {
                    Ok::<_, String>(found || self.at(one(operand), at)?)
                })?,
            NodeKind::Globally { interval, operand } => self
                .instants(
                    instant + u64::from(interval.start()),
                    instant + u64::from(interval.end()),
                )
                .into_iter()
                .try_fold(true, |all, at| {
                    Ok::<_, String>(all && self.at(one(operand), at)?)
                })?,
            NodeKind::Until {
                interval,
                left,
                right,
            } => {
                let mut found = false;
                for witness in self.instants(
                    instant + u64::from(interval.start()),
                    instant + u64::from(interval.end()),
                ) {
                    if self.at(one(right), witness)?
                        && self
                            .prefix(instant, witness)
                            .into_iter()
                            .try_fold(true, |all, at| {
                                Ok::<_, String>(all && self.at(one(left), at)?)
                            })?
                    {
                        found = true;
                        break;
                    }
                }
                found
            }
            NodeKind::Release {
                interval,
                left,
                right,
            } => {
                let mut all = true;
                for witness in self.instants(
                    instant + u64::from(interval.start()),
                    instant + u64::from(interval.end()),
                ) {
                    if !self.at(one(right), witness)?
                        && !self
                            .prefix(instant, witness)
                            .into_iter()
                            .try_fold(false, |any, at| {
                                Ok::<_, String>(any || self.at(one(left), at)?)
                            })?
                    {
                        all = false;
                        break;
                    }
                }
                all
            }
            _ => return Err("v1 corpus contains an unsupported operator".into()),
        };
        self.memo.insert(key, value);
        Ok(value)
    }
}

fn verify(corpus: &Path, manifest: &Value, schema: &JSONSchema) -> Result<(), String> {
    let fixtures = manifest["fixtures"]
        .as_array()
        .ok_or("manifest has no fixtures")?;
    if fixtures.is_empty() {
        return Err("the corpus oracle checked nothing".into());
    }
    for fixture in fixtures {
        let id = fixture["id"].as_str().ok_or("fixture has no id")?;
        let formula = fixture["formula"]
            .as_str()
            .ok_or("fixture has no formula")?;
        let value = read_json(&corpus_file(corpus, formula)?)?;
        let observed = semantic_error(&value, schema);
        match fixture["expected_validation"].as_str() {
            Some("invalid") => {
                if observed != fixture["expected_error"].as_str() {
                    return Err(format!(
                        "{id} rejection reason: expected {:?}, observed {observed:?}",
                        fixture["expected_error"]
                    ));
                }
            }
            Some("valid") => {
                if let Some(error) = observed {
                    return Err(format!("{id} validation: {error}"));
                }
                let document: FormulaDocument = serde_json::from_value(value)
                    .map_err(|error| format!("{id} formula: {error}"))?;
                let derived = horizon(&document)?;
                if fixture["expected_horizon"].as_u64() != Some(derived) {
                    return Err(format!(
                        "{id} derived horizon: {derived} != {}",
                        fixture["expected_horizon"]
                    ));
                }
                if let Some(expected) = fixture.get("expected_closed_trace") {
                    let trace: Vec<Vec<u32>> = serde_json::from_value(fixture["trace"].clone())
                        .map_err(|error| format!("{id} trace: {error}"))?;
                    let actual = Evaluation {
                        nodes: document.nodes(),
                        trace: &trace,
                        memo: HashMap::new(),
                    }
                    .at(document.root().0 as usize, 0)?;
                    if expected.as_bool() != Some(actual) {
                        return Err(format!("{id} derived closed trace: {actual} != {expected}"));
                    }
                }
            }
            _ => return Err(format!("{id} has unknown expected_validation")),
        }
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let arguments: Vec<_> = env::args().skip(1).collect();
    let corpus = match arguments.as_slice() {
        [] => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus"),
        [flag, path] if flag == "--corpus" => PathBuf::from(path),
        _ => return Err("usage: corpus_gate [--corpus DIRECTORY]".into()),
    };
    let (formula_schema, proposition_schema) = schema_contract(&corpus)?;
    let propositions = read_json(&corpus.join("propositions.json"))?;
    if !proposition_schema.is_valid(&propositions) {
        return Err("proposition map violates schema".into());
    }
    serde_json::from_value::<PropositionMapDocument>(propositions)
        .map_err(|error| format!("proposition map: {error}"))?;
    let manifest = read_json(&corpus.join("manifest.json"))?;
    verify(&corpus, &manifest, &formula_schema)
}

fn main() {
    match run() {
        Ok(()) => println!(
            "corpus schemas, derived horizons, rejection reasons, and derived oracles are valid"
        ),
        Err(error) => {
            eprintln!("corpus validation failed: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{read_json, schema_contract, verify, Evaluation};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tl_syntax::{Interval, Node, NodeId, NodeKind};

    // Trace: SUITE-005
    #[test]
    fn changed_closed_trace_oracle_is_rejected() {
        let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
        let mut manifest = read_json(&corpus.join("manifest.json")).unwrap();
        let fixture = manifest["fixtures"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["expected_closed_trace"].is_boolean())
            .unwrap();
        let original = fixture["expected_closed_trace"].as_bool().unwrap();
        fixture["expected_closed_trace"] = serde_json::json!(!original);
        let (schema, _) = schema_contract(&corpus).unwrap();
        assert!(verify(&corpus, &manifest, &schema)
            .unwrap_err()
            .contains("derived closed trace"));
    }

    // Trace: SUITE-005
    #[test]
    fn until_with_zero_witness_has_empty_left_prefix() {
        let nodes = [
            Node::new(NodeKind::False),
            Node::new(NodeKind::True),
            Node::new(NodeKind::Until {
                interval: Interval::new(0, 0).unwrap(),
                left: NodeId(0),
                right: NodeId(1),
            }),
        ];
        let trace = [];
        let mut evaluation = Evaluation {
            nodes: &nodes,
            trace: &trace,
            memo: HashMap::new(),
        };
        assert!(evaluation.at(2, 0).unwrap());
    }
}
