#![cfg(feature = "serde")]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use serde::Deserialize;
use serde_json::Value;
use tl_syntax::{
    FormulaDocument, FormulaSchemaVersion, NodeId, NodeKind, SemanticProfile,
    PAST_HISTORY_CORPUS_V1, PAST_OPERATORS_V1,
};

const DIRECTORY: &str = "corpus/past-history";
const MANIFEST_SHA256: &str = "59b86e7c888bf850cdd4e49cf86b01ffb64a99887d7fd051dca6a7d9f0a56393";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    corpus: String,
    revision: u64,
    role: String,
    formula_schema: String,
    operator_profile: String,
    semantic_profile: String,
    history_schema: String,
    dialect: String,
    implementation_revisions: Revisions,
    files: Vec<Pin>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Revisions {
    tl_syntax: String,
    tl_parse: String,
    tl_mltl: String,
    tl_rewrite: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pin {
    path: String,
    sha256: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Cases {
    corpus: String,
    formula_schema: String,
    operator_profile: String,
    semantic_profile: String,
    dialect: String,
    formulas: Vec<FormulaCase>,
    histories: Vec<History>,
    evaluations: Vec<Evaluation>,
    rewrites: Vec<Rewrite>,
    refusals: Vec<Refusal>,
    target_dispositions: Vec<Target>,
    mutation_axes: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormulaCase {
    id: String,
    source: String,
    required_history: u64,
    document: FormulaDocument,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct History {
    id: String,
    history_id: String,
    revision: u64,
    origin_position: u64,
    through_position: u64,
    clock: Clock,
    observations: Vec<Observation>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum Clock {
    EventPosition,
    FixedSample {
        epoch: ExactNumber,
        period: ExactNumber,
        unit: String,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExactNumber {
    numerator: i64,
    denominator: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Observation {
    position: u64,
    true_propositions: Vec<u32>,
    sample: Option<ExactNumber>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Evaluation {
    id: String,
    formula: String,
    history: String,
    anchor: u64,
    verdict: bool,
    required_history: u64,
    relation: String,
    predecessor_history: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rewrite {
    id: String,
    input: String,
    expected_rule: Option<String>,
    expected_status: String,
    expected_source: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Refusal {
    id: String,
    axis: String,
    expected: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Target {
    target: String,
    state: String,
    role: String,
    reason: String,
}

fn sha256(bytes: &[u8]) -> String {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_owned()
}

fn load() -> (Manifest, Cases) {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join(DIRECTORY);
    let manifest_bytes = fs::read(directory.join("manifest.json")).unwrap();
    assert_eq!(sha256(&manifest_bytes), MANIFEST_SHA256);
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).unwrap();
    let pins: BTreeMap<_, _> = manifest
        .files
        .iter()
        .map(|pin| (pin.path.as_str(), pin.sha256.as_str()))
        .collect();
    assert_eq!(pins.len(), manifest.files.len());
    assert_eq!(
        pins.keys().copied().collect::<Vec<_>>(),
        ["README.md", "cases.json", "schema.json"]
    );
    for pin in &manifest.files {
        assert_eq!(
            sha256(&fs::read(directory.join(&pin.path)).unwrap()),
            pin.sha256,
            "{}",
            pin.path
        );
    }
    let cases = serde_json::from_slice(&fs::read(directory.join("cases.json")).unwrap()).unwrap();
    (manifest, cases)
}

fn required(nodes: &[tl_syntax::Node], id: NodeId) -> Option<u64> {
    let child = |node: NodeId| required(nodes, node);
    match nodes[usize::try_from(id.0).ok()?].kind {
        NodeKind::False | NodeKind::True | NodeKind::Proposition { .. } => Some(0),
        NodeKind::Not { operand } => child(operand),
        NodeKind::And { left, right }
        | NodeKind::Or { left, right }
        | NodeKind::Implies { left, right }
        | NodeKind::Equivalent { left, right } => Some(child(left)?.max(child(right)?)),
        NodeKind::Once { interval, operand } | NodeKind::Historically { interval, operand } => {
            child(operand)?.checked_add(u64::from(interval.end()))
        }
        NodeKind::StrongPrevious { operand } => child(operand)?.checked_add(1),
        NodeKind::Since {
            interval,
            left,
            right,
        }
        | NodeKind::Triggered {
            interval,
            left,
            right,
        } => child(left)?
            .max(child(right)?)
            .checked_add(u64::from(interval.end())),
        NodeKind::Future { .. }
        | NodeKind::Globally { .. }
        | NodeKind::Until { .. }
        | NodeKind::Release { .. } => None,
    }
}

// Trace: TC-056, FR-012-AC-3, FR-012-AC-4, FR-013-AC-3, FR-013-AC-4
#[test]
fn paired_past_history_corpus_is_closed_pinned_and_complete() {
    let (manifest, cases) = load();
    assert_eq!(manifest.corpus, PAST_HISTORY_CORPUS_V1);
    assert_eq!(manifest.revision, 1);
    assert_eq!(manifest.role, "evidence-input");
    assert_eq!(manifest.formula_schema, "tl-syntax.formula/v2");
    assert_eq!(manifest.operator_profile, PAST_OPERATORS_V1);
    assert_eq!(
        manifest.semantic_profile,
        SemanticProfile::OriginCompleteHistoryV1.as_str()
    );
    assert_eq!(manifest.history_schema, "tl-mltl.position-history/v1");
    assert_eq!(manifest.dialect, "tl-parse.clean-ascii/v3");
    assert_eq!(
        manifest.implementation_revisions.tl_syntax,
        "e70f2379a752117c79603bc399a86c26feed7716"
    );
    assert_eq!(
        manifest.implementation_revisions.tl_parse,
        "f82b0c724675c0f774415aa696c360959da30481"
    );
    assert_eq!(
        manifest.implementation_revisions.tl_mltl,
        "b346cd0902794633e862f644a5575fc9776c34fb"
    );
    assert_eq!(
        manifest.implementation_revisions.tl_rewrite,
        "22b9cadcb1692cec8d3a97768f4f3b38fc654a5e"
    );

    assert_eq!(cases.corpus, manifest.corpus);
    assert_eq!(cases.formula_schema, manifest.formula_schema);
    assert_eq!(cases.operator_profile, manifest.operator_profile);
    assert_eq!(cases.semantic_profile, manifest.semantic_profile);
    assert_eq!(cases.dialect, manifest.dialect);
    let mut formula_ids = BTreeSet::new();
    for case in &cases.formulas {
        assert!(formula_ids.insert(&case.id));
        assert!(!case.source.is_empty());
        assert_eq!(case.document.schema_version(), FormulaSchemaVersion::V2);
        assert_eq!(
            case.document.semantic_profile(),
            SemanticProfile::OriginCompleteHistoryV1
        );
        case.document.validate().unwrap();
        assert_eq!(
            required(case.document.nodes(), case.document.root()),
            Some(case.required_history),
            "{}",
            case.id
        );
    }
    assert_eq!(cases.formulas.len(), 8);

    let mut history_ids = BTreeSet::new();
    for history in &cases.histories {
        assert!(history_ids.insert(&history.id));
        assert!(!history.history_id.is_empty());
        assert!(history.revision > 0);
        assert_eq!(history.origin_position, 0);
        assert_eq!(history.observations.first().unwrap().position, 0);
        assert_eq!(
            history.observations.last().unwrap().position,
            history.through_position
        );
        for (position, observation) in history.observations.iter().enumerate() {
            assert_eq!(observation.position, u64::try_from(position).unwrap());
            assert!(observation
                .true_propositions
                .windows(2)
                .all(|pair| pair[0] < pair[1]));
        }
        match &history.clock {
            Clock::EventPosition => assert!(history
                .observations
                .iter()
                .all(|item| item.sample.is_none())),
            Clock::FixedSample {
                epoch,
                period,
                unit,
            } => {
                assert!(!unit.is_empty());
                assert!(epoch.denominator > 0);
                assert!(period.numerator > 0);
                assert!(period.denominator > 0);
                assert!(history.observations.iter().all(|item| item
                    .sample
                    .as_ref()
                    .is_some_and(|sample| sample.denominator > 0)));
            }
        }
    }
    assert_eq!(cases.histories.len(), 3);

    let mut evaluation_ids = BTreeSet::new();
    for case in &cases.evaluations {
        assert!(evaluation_ids.insert(&case.id));
        assert!(formula_ids.contains(&case.formula));
        assert!(history_ids.contains(&case.history));
        assert!(
            case.anchor
                <= cases
                    .histories
                    .iter()
                    .find(|h| h.id == case.history)
                    .unwrap()
                    .through_position
        );
        assert_eq!(
            case.required_history,
            cases
                .formulas
                .iter()
                .find(|f| f.id == case.formula)
                .unwrap()
                .required_history
        );
        assert!(["original", "superseding", "invalidating"].contains(&case.relation.as_str()));
        assert_eq!(
            case.predecessor_history.is_some(),
            case.relation != "original"
        );
    }
    assert_eq!(
        cases.evaluations.iter().filter(|case| case.verdict).count(),
        5
    );

    for case in &cases.rewrites {
        assert!(!case.id.is_empty());
        assert!(formula_ids.contains(&case.input));
        assert!(!case.expected_source.is_empty());
        assert_eq!(
            case.expected_rule.is_some(),
            case.expected_status == "rewritten"
        );
    }
    let refusal_axes: BTreeSet<_> = cases
        .refusals
        .iter()
        .map(|item| item.axis.as_str())
        .collect();
    assert_eq!(
        refusal_axes,
        [
            "anchor",
            "clock",
            "history",
            "identity",
            "owner_non_value",
            "profile",
            "resource"
        ]
        .into_iter()
        .collect()
    );
    assert!(cases
        .refusals
        .iter()
        .all(|item| !item.id.is_empty() && !item.expected.is_empty()));
    let targets: Vec<_> = cases
        .target_dispositions
        .iter()
        .map(|item| {
            (
                item.target.as_str(),
                item.state.as_str(),
                item.role.as_str(),
            )
        })
        .collect();
    assert_eq!(
        targets,
        [
            ("fret", "supported", "output_only"),
            ("r2u2", "unavailable", "monitor_target"),
            ("c2po", "unavailable", "monitor_target"),
            ("isabelle_hol", "unsupported", "oracle")
        ]
    );
    assert!(cases
        .target_dispositions
        .iter()
        .all(|item| !item.reason.is_empty()));
    assert_eq!(
        cases.mutation_axes.iter().cloned().collect::<BTreeSet<_>>(),
        [
            "operator_direction",
            "offset_subtraction",
            "inclusive_endpoints",
            "since_lower_bound",
            "triggered_duality",
            "semantic_profile",
            "history_gap",
            "anchor",
            "digest",
            "required_history_arithmetic"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
}

// Trace: TC-056, FR-013-AC-3, FR-013-AC-4
#[test]
fn corpus_identity_profile_and_closed_wire_mutations_are_rejected() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join(DIRECTORY);
    let bytes = fs::read(directory.join("cases.json")).unwrap();
    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["unknown"] = Value::Bool(true);
    assert!(serde_json::from_value::<Cases>(value).is_err());

    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["formulas"][0]["document"]["semantic_profile"] =
        Value::String("mltl.closed-trace/v1".into());
    assert!(serde_json::from_value::<Cases>(value).is_err());

    let mut manifest = fs::read(directory.join("manifest.json")).unwrap();
    manifest[0] ^= 1;
    assert_ne!(sha256(&manifest), MANIFEST_SHA256);
}
