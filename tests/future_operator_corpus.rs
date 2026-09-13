#![cfg(feature = "serde")]
//! TC-074: replay the paired W/M source and canonical-graph corpus.
//!
//! `corpus/future-operators/` is evidence input. Each derived-source case binds
//! a dialect, operator profile, semantic profile, and source text to ordered
//! append and lower steps; the replay builds its document only through
//! [`Formula::new`] and [`FutureLoweringRequest::lower`] and compares it with a
//! span-free expected formula-v1 document that a directly constructed case
//! shares. The replay evaluates nothing and admits no derived wire node.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde::Deserialize;
use serde_json::Value;
use tl_syntax::{
    Formula, FormulaDocument, FutureKind, FutureLoweringAxis, FutureLoweringRefusal,
    FutureLoweringReport, FutureLoweringRequest, Node, NodeId, NodeKind, RawBounds,
    SemanticProfile, SourceSpan, FUTURE_LOWERING_NODE_CHARGE, FUTURE_LOWERING_REQUEST_V1,
    FUTURE_OPERATORS_V1,
};

const CORPUS_DIRECTORY: &str = "corpus/future-operators";
const CORPUS_IDENTITY: &str = "tl-syntax.future-operator-corpus/v1";
const CORPUS_REVISION: u64 = 1;
const EVIDENCE_ROLE: &str = "evidence-input";
const DERIVED_DIALECT: &str = "tl-parse.clean-ascii/v2";
const PRIMITIVE_DIALECT: &str = "tl-parse.clean-ascii/v1";
const MANIFEST: &str = "manifest.json";
const CASES: &str = "cases.json";
const EXPECTED_PREFIX: &str = "expected/";
/// Files the replay does not read: prose, and the `make check-corpus` digest list.
const UNREPLAYED: [&str; 2] = ["README.md", "SHA256SUMS"];

/// SHA-256 of `corpus/future-operators/manifest.json`; the manifest pins every other file.
const MANIFEST_SHA256: &str = "cd1748c8c04096193b86c8ee95b4703486084b1eb1a6032227f435ef48a864d8";

/// Stable replay failure classes. Tests match these, never diagnostic text.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Code {
    MissingFile,
    ManifestDigestMismatch,
    ManifestDecodeRejected,
    CorpusIdentityMismatch,
    FileDigestMismatch,
    UnpinnedFile,
    CasesDecodeRejected,
    ExpectedDocumentRejected,
    ExpectedDocumentCarriesSpan,
    CaseDecodeRejected,
    NodeDecodeRejected,
    UnknownDialect,
    DialectRefusesDerived,
    OperatorProfileBindingMismatch,
    DerivedCaseWithoutLowering,
    DirectCaseLowers,
    DirectCaseCarriesSpan,
    OverrideOutsideRefused,
    UnpinnedExpectedDocument,
    GraphInvalid,
    UnexpectedRefusal,
    MissingRefusal,
    RefusalMismatch,
    SourceBindingMismatch,
    GeneratedSpanMismatch,
    LoweringReportMismatch,
    DocumentMismatch,
    NestedMalformed,
    MalformedCaseAccepted,
    MalformedErrorMismatch,
    DuplicateCaseId,
    ClassAbsent,
    ProfileRowAbsent,
    BoundaryAbsent,
    NestingAbsent,
    UnpairedExpectedDocument,
}

impl Code {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingFile => "missing_file",
            Self::ManifestDigestMismatch => "manifest_digest_mismatch",
            Self::ManifestDecodeRejected => "manifest_decode_rejected",
            Self::CorpusIdentityMismatch => "corpus_identity_mismatch",
            Self::FileDigestMismatch => "file_digest_mismatch",
            Self::UnpinnedFile => "unpinned_file",
            Self::CasesDecodeRejected => "cases_decode_rejected",
            Self::ExpectedDocumentRejected => "expected_document_rejected",
            Self::ExpectedDocumentCarriesSpan => "expected_document_carries_span",
            Self::CaseDecodeRejected => "case_decode_rejected",
            Self::NodeDecodeRejected => "node_decode_rejected",
            Self::UnknownDialect => "unknown_dialect",
            Self::DialectRefusesDerived => "dialect_refuses_derived",
            Self::OperatorProfileBindingMismatch => "operator_profile_binding_mismatch",
            Self::DerivedCaseWithoutLowering => "derived_case_without_lowering",
            Self::DirectCaseLowers => "direct_case_lowers",
            Self::DirectCaseCarriesSpan => "direct_case_carries_span",
            Self::OverrideOutsideRefused => "override_outside_refused",
            Self::UnpinnedExpectedDocument => "unpinned_expected_document",
            Self::GraphInvalid => "graph_invalid",
            Self::UnexpectedRefusal => "unexpected_refusal",
            Self::MissingRefusal => "missing_refusal",
            Self::RefusalMismatch => "refusal_mismatch",
            Self::SourceBindingMismatch => "source_binding_mismatch",
            Self::GeneratedSpanMismatch => "generated_span_mismatch",
            Self::LoweringReportMismatch => "lowering_report_mismatch",
            Self::DocumentMismatch => "document_mismatch",
            Self::NestedMalformed => "nested_malformed",
            Self::MalformedCaseAccepted => "malformed_case_accepted",
            Self::MalformedErrorMismatch => "malformed_error_mismatch",
            Self::DuplicateCaseId => "duplicate_case_id",
            Self::ClassAbsent => "class_absent",
            Self::ProfileRowAbsent => "profile_row_absent",
            Self::BoundaryAbsent => "boundary_absent",
            Self::NestingAbsent => "nesting_absent",
            Self::UnpairedExpectedDocument => "unpaired_expected_document",
        }
    }
}

#[derive(Debug)]
struct ReplayError {
    code: Code,
    /// Case identity or corpus path the failure belongs to.
    subject: String,
    /// Human diagnostic only; no test inspects it.
    detail: String,
}

impl ReplayError {
    fn new(code: Code, subject: &str, detail: impl Into<String>) -> Self {
        Self {
            code,
            subject: subject.to_owned(),
            detail: detail.into(),
        }
    }
}

type Replay<T> = Result<T, ReplayError>;

fn fail<T>(code: Code, subject: &str, detail: impl Into<String>) -> Replay<T> {
    Err(ReplayError::new(code, subject, detail))
}

fn axis_name(axis: FutureLoweringAxis) -> &'static str {
    match axis {
        FutureLoweringAxis::RequestIdentity => "request_identity",
        FutureLoweringAxis::OperatorProfile => "operator_profile",
        FutureLoweringAxis::Kind => "kind",
        FutureLoweringAxis::SemanticProfile => "semantic_profile",
        FutureLoweringAxis::ProfileAgreement => "profile_agreement",
        FutureLoweringAxis::Interval => "interval",
        FutureLoweringAxis::Operand => "operand",
        FutureLoweringAxis::Span => "span",
        FutureLoweringAxis::NodeBudget => "node_budget",
    }
}

// ---------------------------------------------------------------------------
// Wire shapes
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestWire {
    corpus: String,
    revision: u64,
    role: String,
    formula_schema: String,
    operator_profile: String,
    request_identity: String,
    derived_dialect: String,
    files: Vec<PinWire>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PinWire {
    path: String,
    sha256: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CasesWire {
    corpus: String,
    cases: Vec<Value>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct Bounds {
    start: u64,
    end: u64,
}

impl Bounds {
    fn raw(self) -> RawBounds {
        RawBounds::new(self.start, self.end)
    }

    fn as_span(self) -> Option<SourceSpan> {
        let start = u32::try_from(self.start).ok()?;
        let end = u32::try_from(self.end).ok()?;
        SourceSpan::new(start, end).ok()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LowerWire {
    kind: String,
    left: u64,
    right: u64,
    interval: Option<Bounds>,
    operator_span: Option<Bounds>,
    expression_span: Option<Bounds>,
    #[serde(default)]
    request_identity: Option<String>,
    #[serde(default)]
    operator_profile: Option<String>,
    #[serde(default)]
    semantic_profile: Option<String>,
}

impl LowerWire {
    fn has_override(&self) -> bool {
        self.request_identity.is_some()
            || self.operator_profile.is_some()
            || self.semantic_profile.is_some()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum StepWire {
    Append(Value),
    Lower(LowerWire),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LoweringRecord {
    kind: String,
    left: u32,
    right: u32,
    first_generated: u32,
    root: u32,
    operator_span: Bounds,
    expression_span: Bounds,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RefusalRecord {
    code: String,
    axis: String,
}

#[derive(Deserialize)]
#[serde(tag = "class", rename_all = "snake_case", deny_unknown_fields)]
enum CaseWire {
    Derived {
        id: String,
        dialect: String,
        operator_profile: String,
        semantic_profile: SemanticProfile,
        source: String,
        steps: Vec<StepWire>,
        root: u32,
        expected: String,
        expected_lowerings: Vec<LoweringRecord>,
    },
    Direct {
        id: String,
        semantic_profile: SemanticProfile,
        steps: Vec<StepWire>,
        root: u32,
        expected: String,
    },
    Refused {
        id: String,
        dialect: String,
        operator_profile: String,
        semantic_profile: SemanticProfile,
        source: String,
        steps: Vec<StepWire>,
        expected_refusal: RefusalRecord,
    },
    Malformed {
        id: String,
        entry: Value,
        expected_error: String,
    },
}

enum Step {
    Append(Node),
    Lower(LowerWire),
}

// ---------------------------------------------------------------------------
// Digests and file access
// ---------------------------------------------------------------------------

/// Relative corpus path to file bytes.
type CorpusFiles = BTreeMap<String, Vec<u8>>;

fn sha256_hex(bytes: &[u8]) -> String {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn sha256sum");
    child
        .stdin
        .take()
        .expect("sha256sum stdin")
        .write_all(bytes)
        .expect("write sha256sum stdin");
    let output = child.wait_with_output().expect("sha256sum output");
    assert!(output.status.success(), "sha256sum failed");
    String::from_utf8(output.stdout)
        .expect("sha256sum prints UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256sum prints a digest")
        .to_owned()
}

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(CORPUS_DIRECTORY)
}

fn collect_files(root: &Path, directory: &Path, files: &mut CorpusFiles) {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        .map(|entry| entry.expect("directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_files(root, &path, files);
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .expect("corpus path")
            .to_str()
            .expect("UTF-8 corpus path")
            .replace('\\', "/");
        if UNREPLAYED.contains(&relative.as_str()) {
            continue;
        }
        let bytes = fs::read(&path).unwrap_or_else(|error| panic!("read {relative}: {error}"));
        files.insert(relative, bytes);
    }
}

fn load_corpus() -> CorpusFiles {
    let root = corpus_root();
    let mut files = CorpusFiles::new();
    collect_files(&root, &root, &mut files);
    files
}

// ---------------------------------------------------------------------------
// Replay
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Summary {
    derived: usize,
    direct: usize,
    refused: usize,
    malformed: usize,
    malformed_codes: BTreeSet<&'static str>,
    refusal_codes: BTreeSet<&'static str>,
}

enum Outcome {
    Derived {
        expected: String,
        reports: Vec<FutureLoweringReport>,
    },
    Direct {
        expected: String,
    },
    Refused {
        code: &'static str,
    },
    Malformed {
        code: &'static str,
    },
}

struct Corpus {
    expected: BTreeMap<String, FormulaDocument>,
}

fn replay(files: &CorpusFiles, manifest_pin: &str) -> Replay<Summary> {
    let manifest_bytes = files
        .get(MANIFEST)
        .ok_or_else(|| ReplayError::new(Code::MissingFile, MANIFEST, "manifest is absent"))?;
    let manifest_digest = sha256_hex(manifest_bytes);
    if manifest_digest != manifest_pin {
        return fail(
            Code::ManifestDigestMismatch,
            MANIFEST,
            format!("manifest digest {manifest_digest} is not the pinned {manifest_pin}"),
        );
    }
    let manifest: ManifestWire = serde_json::from_slice(manifest_bytes).map_err(|error| {
        ReplayError::new(Code::ManifestDecodeRejected, MANIFEST, error.to_string())
    })?;
    let identities = [
        (manifest.corpus.as_str(), CORPUS_IDENTITY),
        (manifest.role.as_str(), EVIDENCE_ROLE),
        (manifest.formula_schema.as_str(), "tl-syntax.formula/v1"),
        (manifest.operator_profile.as_str(), FUTURE_OPERATORS_V1),
        (
            manifest.request_identity.as_str(),
            FUTURE_LOWERING_REQUEST_V1,
        ),
        (manifest.derived_dialect.as_str(), DERIVED_DIALECT),
    ];
    for (declared, required) in identities {
        if declared != required {
            return fail(
                Code::CorpusIdentityMismatch,
                MANIFEST,
                format!("manifest declares {declared}, the replay requires {required}"),
            );
        }
    }
    if manifest.revision != CORPUS_REVISION {
        return fail(
            Code::CorpusIdentityMismatch,
            MANIFEST,
            format!(
                "manifest revision {} is not {CORPUS_REVISION}",
                manifest.revision
            ),
        );
    }

    let mut pins = BTreeMap::new();
    for pin in &manifest.files {
        if pin.path == MANIFEST || pins.insert(pin.path.clone(), pin.sha256.clone()).is_some() {
            return fail(Code::ManifestDecodeRejected, &pin.path, "pinned twice");
        }
        let bytes = files.get(&pin.path).ok_or_else(|| {
            ReplayError::new(Code::MissingFile, &pin.path, "pinned file is absent")
        })?;
        let digest = sha256_hex(bytes);
        if digest != pin.sha256 {
            return fail(
                Code::FileDigestMismatch,
                &pin.path,
                format!("digest {digest} is not the pinned {}", pin.sha256),
            );
        }
    }
    for path in files.keys() {
        if path != MANIFEST && !pins.contains_key(path) {
            return fail(
                Code::UnpinnedFile,
                path,
                "file is not pinned by the manifest",
            );
        }
    }
    if !pins.contains_key(CASES) {
        return fail(
            Code::MissingFile,
            CASES,
            "the manifest does not pin the cases",
        );
    }

    let mut corpus = Corpus {
        expected: BTreeMap::new(),
    };
    for path in pins.keys().filter(|path| path.starts_with(EXPECTED_PREFIX)) {
        let document: FormulaDocument = serde_json::from_slice(&files[path]).map_err(|error| {
            ReplayError::new(Code::ExpectedDocumentRejected, path, error.to_string())
        })?;
        if document.nodes().iter().any(|node| node.span.is_some()) {
            return fail(
                Code::ExpectedDocumentCarriesSpan,
                path,
                "expected documents are span-free canonical graphs",
            );
        }
        corpus.expected.insert(path.clone(), document);
    }
    if let Some(path) = pins
        .keys()
        .find(|path| path.as_str() != CASES && !path.starts_with(EXPECTED_PREFIX))
    {
        return fail(
            Code::UnpinnedFile,
            path,
            "the manifest pins a file the replay has no role for",
        );
    }

    let cases: CasesWire = serde_json::from_slice(&files[CASES])
        .map_err(|error| ReplayError::new(Code::CasesDecodeRejected, CASES, error.to_string()))?;
    if cases.corpus != CORPUS_IDENTITY {
        return fail(Code::CorpusIdentityMismatch, CASES, cases.corpus);
    }

    let mut summary = Summary::default();
    let mut ids = BTreeSet::new();
    let mut derived_documents = BTreeSet::new();
    let mut direct_documents = BTreeSet::new();
    let mut rows = BTreeSet::new();
    let mut intervals = BTreeSet::new();
    let mut nested_left = false;
    let mut nested_right = false;
    for value in cases.cases {
        let id = value
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<no id>")
            .to_owned();
        if !ids.insert(id.clone()) {
            return fail(Code::DuplicateCaseId, &id, "case identity is reused");
        }
        match replay_case(&corpus, value, true)? {
            Outcome::Derived { expected, reports } => {
                summary.derived += 1;
                let mut generated_roots = BTreeSet::new();
                for report in reports {
                    rows.insert((report.kind(), report.semantic_profile()));
                    let first = report.first_generated().0;
                    intervals.insert(interval_of(&corpus.expected[&expected], first));
                    nested_left |= generated_roots.contains(&report.left());
                    nested_right |= generated_roots.contains(&report.right());
                    generated_roots.insert(report.root());
                }
                derived_documents.insert(expected);
            }
            Outcome::Direct { expected } => {
                summary.direct += 1;
                direct_documents.insert(expected);
            }
            Outcome::Refused { code } => {
                summary.refused += 1;
                summary.refusal_codes.insert(code);
            }
            Outcome::Malformed { code } => {
                summary.malformed += 1;
                summary.malformed_codes.insert(code);
            }
        }
    }

    for (class, count) in [
        ("derived", summary.derived),
        ("direct", summary.direct),
        ("refused", summary.refused),
        ("malformed", summary.malformed),
    ] {
        if count == 0 {
            return fail(
                Code::ClassAbsent,
                class,
                "the corpus has no case of this class",
            );
        }
    }
    for kind in [FutureKind::WeakUntil, FutureKind::StrongRelease] {
        for profile in [
            SemanticProfile::ClosedTraceV1,
            SemanticProfile::OnlinePrefixV1,
        ] {
            if !rows.contains(&(kind, profile)) {
                return fail(
                    Code::ProfileRowAbsent,
                    kind.as_str(),
                    format!("no derived lowering under {}", profile.as_str()),
                );
            }
        }
    }
    for boundary in [(0, 0), (u32::MAX, u32::MAX)] {
        if !intervals.contains(&Some(boundary)) {
            return fail(
                Code::BoundaryAbsent,
                CASES,
                format!("no derived lowering over [{},{}]", boundary.0, boundary.1),
            );
        }
    }
    if !(nested_left && nested_right) {
        return fail(
            Code::NestingAbsent,
            CASES,
            "derived lowerings must nest a lowered root in both operand positions",
        );
    }
    for path in corpus.expected.keys() {
        if !(derived_documents.contains(path) && direct_documents.contains(path)) {
            return fail(
                Code::UnpairedExpectedDocument,
                path,
                "each expected document needs a derived and a direct case",
            );
        }
    }
    Ok(summary)
}

/// Interval of the first generated node, which is the U or R node of the expansion.
fn interval_of(document: &FormulaDocument, first_generated: u32) -> Option<(u32, u32)> {
    match document
        .nodes()
        .get(usize::try_from(first_generated).ok()?)?
        .kind
    {
        NodeKind::Until { interval, .. } | NodeKind::Release { interval, .. } => {
            Some((interval.start(), interval.end()))
        }
        _ => None,
    }
}

fn decode_steps(id: &str, steps: Vec<StepWire>) -> Replay<Vec<Step>> {
    steps
        .into_iter()
        .map(|step| match step {
            StepWire::Append(value) => serde_json::from_value(value)
                .map(Step::Append)
                .map_err(|error| ReplayError::new(Code::NodeDecodeRejected, id, error.to_string())),
            StepWire::Lower(lower) => Ok(Step::Lower(lower)),
        })
        .collect()
}

fn has_lower(steps: &[Step]) -> bool {
    steps.iter().any(|step| matches!(step, Step::Lower(_)))
}

fn check_dialect(id: &str, dialect: &str, steps: &[Step]) -> Replay<()> {
    match dialect {
        DERIVED_DIALECT => Ok(()),
        PRIMITIVE_DIALECT if has_lower(steps) => fail(
            Code::DialectRefusesDerived,
            id,
            "tl-parse.clean-ascii/v1 has no derived future operator",
        ),
        PRIMITIVE_DIALECT => fail(
            Code::DerivedCaseWithoutLowering,
            id,
            "a primitive-dialect case carries no derived source",
        ),
        other => fail(Code::UnknownDialect, id, other),
    }
}

fn check_operator_profile(id: &str, operator_profile: &str) -> Replay<()> {
    if operator_profile == FUTURE_OPERATORS_V1 {
        Ok(())
    } else {
        fail(Code::OperatorProfileBindingMismatch, id, operator_profile)
    }
}

fn expected_document<'c>(corpus: &'c Corpus, id: &str, path: &str) -> Replay<&'c FormulaDocument> {
    corpus
        .expected
        .get(path)
        .ok_or_else(|| ReplayError::new(Code::UnpinnedExpectedDocument, id, path))
}

/// One lowering the replay performed, with the step that requested it.
struct Performed {
    step: LowerWire,
    report: FutureLoweringReport,
}

struct Execution {
    nodes: Vec<Node>,
    lowerings: Vec<Performed>,
    refusal: Option<FutureLoweringRefusal>,
}

/// Appends nodes and lowers derived steps through the public tl-syntax API.
fn execute(
    id: &str,
    profile: SemanticProfile,
    steps: Vec<Step>,
    refused_case: bool,
) -> Replay<Execution> {
    let mut execution = Execution {
        nodes: Vec::new(),
        lowerings: Vec::new(),
        refusal: None,
    };
    let count = steps.len();
    for (index, step) in steps.into_iter().enumerate() {
        let lower = match step {
            Step::Append(node) => {
                execution.nodes.push(node);
                continue;
            }
            Step::Lower(lower) => lower,
        };
        if lower.has_override() && !refused_case {
            return fail(
                Code::OverrideOutsideRefused,
                id,
                "only a refused case may override the bound request identities",
            );
        }
        let Some(last) = execution.nodes.len().checked_sub(1) else {
            return fail(Code::GraphInvalid, id, "a lowering needs a preceding graph");
        };
        let root = NodeId(u32::try_from(last).expect("corpus graphs are small"));
        let formula = Formula::new(profile, root, &execution.nodes)
            .map_err(|error| ReplayError::new(Code::GraphInvalid, id, error.to_string()))?;
        let request = FutureLoweringRequest {
            request_identity: lower
                .request_identity
                .as_deref()
                .unwrap_or(FUTURE_LOWERING_REQUEST_V1)
                .as_bytes(),
            operator_profile: lower
                .operator_profile
                .as_deref()
                .unwrap_or(FUTURE_OPERATORS_V1)
                .as_bytes(),
            kind: lower.kind.as_bytes(),
            semantic_profile: lower
                .semantic_profile
                .as_deref()
                .unwrap_or(profile.as_str())
                .as_bytes(),
            formula,
            left: lower.left,
            right: lower.right,
            interval: lower.interval.map(Bounds::raw),
            operator_span: lower.operator_span.map(Bounds::raw),
            expression_span: lower.expression_span.map(Bounds::raw),
        };
        match request.lower() {
            Ok(lowering) => {
                let base = execution.nodes.len();
                let ids = lowering.node_ids();
                for (offset, generated) in ids.iter().enumerate() {
                    assert_eq!(
                        usize::try_from(generated.0).ok(),
                        Some(base + offset),
                        "{id}: the lowering API returned non-appending identities"
                    );
                }
                execution.nodes.extend_from_slice(lowering.nodes());
                execution.lowerings.push(Performed {
                    step: lower,
                    report: *lowering.report(),
                });
            }
            Err(refusal) if refused_case && index + 1 == count => {
                execution.refusal = Some(refusal);
            }
            Err(refusal) => {
                return fail(Code::UnexpectedRefusal, id, refusal.to_string());
            }
        }
    }
    Ok(execution)
}

fn span_text<'s>(id: &str, source: &'s str, span: Option<SourceSpan>) -> Replay<&'s str> {
    let span = span.ok_or_else(|| {
        ReplayError::new(
            Code::SourceBindingMismatch,
            id,
            "derived source nodes carry spans",
        )
    })?;
    let range = usize::try_from(span.start()).expect("u32 fits usize")
        ..usize::try_from(span.end()).expect("u32 fits usize");
    source.get(range).ok_or_else(|| {
        ReplayError::new(
            Code::SourceBindingMismatch,
            id,
            format!(
                "span {}..{} lies outside the source",
                span.start(),
                span.end()
            ),
        )
    })
}

fn covers(outer: SourceSpan, inner: Option<SourceSpan>) -> bool {
    inner.is_some_and(|inner| outer.start() <= inner.start() && inner.end() <= outer.end())
}

fn operands(kind: NodeKind) -> Vec<NodeId> {
    match kind {
        NodeKind::False | NodeKind::True | NodeKind::Proposition { .. } => Vec::new(),
        NodeKind::Not { operand }
        | NodeKind::Future { operand, .. }
        | NodeKind::Globally { operand, .. } => vec![operand],
        NodeKind::And { left, right }
        | NodeKind::Or { left, right }
        | NodeKind::Implies { left, right }
        | NodeKind::Equivalent { left, right }
        | NodeKind::Until { left, right, .. }
        | NodeKind::Release { left, right, .. } => vec![left, right],
    }
}

fn node_span(nodes: &[Node], id: NodeId) -> Option<SourceSpan> {
    nodes.get(usize::try_from(id.0).ok()?)?.span
}

/// Binds every appended node, operator token, and expression to the source text.
fn check_source_binding(id: &str, source: &str, execution: &Execution, root: u32) -> Replay<()> {
    let nodes = &execution.nodes;
    let mut generated = BTreeSet::new();
    for performed in &execution.lowerings {
        let report = &performed.report;
        let expression = report.expression_span();
        for offset in 0..FUTURE_LOWERING_NODE_CHARGE {
            let node = report.first_generated().0 + u32::try_from(offset).expect("charge fits");
            generated.insert(node);
            if node_span(nodes, NodeId(node)) != expression {
                return fail(
                    Code::GeneratedSpanMismatch,
                    id,
                    format!("generated node {node} does not carry the expression span"),
                );
            }
        }
        let token = span_text(id, source, report.operator_span())?;
        let expression_text = span_text(id, source, expression)?;
        let interval = performed.step.interval.ok_or_else(|| {
            ReplayError::new(
                Code::SourceBindingMismatch,
                id,
                "derived tokens carry an interval",
            )
        })?;
        let spelled = format!(
            "{}[{},{}]",
            report.kind().as_str(),
            interval.start,
            interval.end
        );
        if token != spelled {
            return fail(
                Code::SourceBindingMismatch,
                id,
                format!("operator span reads {token:?}, the step lowers {spelled:?}"),
            );
        }
        let expression = expression.expect("span_text accepted the expression span");
        for operand in [report.left(), report.right()] {
            if !covers(expression, node_span(nodes, operand)) {
                return fail(
                    Code::SourceBindingMismatch,
                    id,
                    format!(
                        "expression {expression_text:?} does not cover operand {}",
                        operand.0
                    ),
                );
            }
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        if generated.contains(&u32::try_from(index).expect("small graph")) {
            continue;
        }
        let text = span_text(id, source, node.span)?;
        let required = match node.kind {
            NodeKind::Proposition { proposition } => Some(format!("p{}", proposition.0)),
            NodeKind::True => Some("true".to_owned()),
            NodeKind::False => Some("false".to_owned()),
            _ => None,
        };
        if let Some(required) = required {
            if text != required {
                return fail(
                    Code::SourceBindingMismatch,
                    id,
                    format!("node {index} reads {text:?}, its kind spells {required:?}"),
                );
            }
        }
        let span = node.span.expect("span_text accepted the span");
        for operand in operands(node.kind) {
            if !covers(span, node_span(nodes, operand)) {
                return fail(
                    Code::SourceBindingMismatch,
                    id,
                    format!("node {index} does not cover operand {}", operand.0),
                );
            }
        }
    }
    let whole = SourceSpan::new(0, u32::try_from(source.len()).expect("small source"))
        .expect("ordered span");
    if node_span(nodes, NodeId(root)) != Some(whole) {
        return fail(
            Code::SourceBindingMismatch,
            id,
            "the root node must span the whole source",
        );
    }
    Ok(())
}

fn check_reports(id: &str, execution: &Execution, records: &[LoweringRecord]) -> Replay<()> {
    if execution.lowerings.len() != records.len() {
        return fail(
            Code::LoweringReportMismatch,
            id,
            format!(
                "{} lowerings performed, {} recorded",
                execution.lowerings.len(),
                records.len()
            ),
        );
    }
    for (performed, record) in execution.lowerings.iter().zip(records) {
        let report = &performed.report;
        let matches = report.kind().as_str() == record.kind
            && report.left() == NodeId(record.left)
            && report.right() == NodeId(record.right)
            && report.first_generated() == NodeId(record.first_generated)
            && report.root() == NodeId(record.root)
            && report.generated_count() == FUTURE_LOWERING_NODE_CHARGE
            && report.operator_span() == record.operator_span.as_span()
            && report.expression_span() == record.expression_span.as_span()
            && report.operator_profile() == FUTURE_OPERATORS_V1
            && report.request_identity() == FUTURE_LOWERING_REQUEST_V1;
        if !matches {
            return fail(
                Code::LoweringReportMismatch,
                id,
                format!("report {report:?} differs from its record"),
            );
        }
    }
    Ok(())
}

fn semantic_bytes(document: &FormulaDocument) -> Vec<u8> {
    serde_json::to_vec(&document.semantic_view()).expect("semantic view serializes")
}

fn replay_case(corpus: &Corpus, value: Value, top_level: bool) -> Replay<Outcome> {
    let fallback_id = value
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("<no id>")
        .to_owned();
    let case: CaseWire = serde_json::from_value(value).map_err(|error| {
        ReplayError::new(Code::CaseDecodeRejected, &fallback_id, error.to_string())
    })?;
    match case {
        CaseWire::Derived {
            id,
            dialect,
            operator_profile,
            semantic_profile,
            source,
            steps,
            root,
            expected,
            expected_lowerings,
        } => {
            let steps = decode_steps(&id, steps)?;
            check_dialect(&id, &dialect, &steps)?;
            check_operator_profile(&id, &operator_profile)?;
            if !has_lower(&steps) {
                return fail(Code::DerivedCaseWithoutLowering, &id, "no lower step");
            }
            let document = expected_document(corpus, &id, &expected)?;
            let execution = execute(&id, semantic_profile, steps, false)?;
            check_source_binding(&id, &source, &execution, root)?;
            check_reports(&id, &execution, &expected_lowerings)?;
            let built =
                FormulaDocument::new(semantic_profile, NodeId(root), execution.nodes.clone())
                    .map_err(|error| {
                        ReplayError::new(Code::GraphInvalid, &id, error.to_string())
                    })?;
            if built.semantic_view() != document.semantic_view()
                || semantic_bytes(&built) != semantic_bytes(document)
            {
                return fail(
                    Code::DocumentMismatch,
                    &id,
                    "the lowered graph differs from the shared expected document",
                );
            }
            Ok(Outcome::Derived {
                expected,
                reports: execution
                    .lowerings
                    .into_iter()
                    .map(|performed| performed.report)
                    .collect(),
            })
        }
        CaseWire::Direct {
            id,
            semantic_profile,
            steps,
            root,
            expected,
        } => {
            let steps = decode_steps(&id, steps)?;
            if has_lower(&steps) {
                return fail(Code::DirectCaseLowers, &id, "direct cases append only");
            }
            let document = expected_document(corpus, &id, &expected)?;
            let execution = execute(&id, semantic_profile, steps, false)?;
            if execution.nodes.iter().any(|node| node.span.is_some()) {
                return fail(
                    Code::DirectCaseCarriesSpan,
                    &id,
                    "direct cases are span-free",
                );
            }
            let built = FormulaDocument::new(semantic_profile, NodeId(root), execution.nodes)
                .map_err(|error| ReplayError::new(Code::GraphInvalid, &id, error.to_string()))?;
            if built != *document || semantic_bytes(&built) != semantic_bytes(document) {
                return fail(
                    Code::DocumentMismatch,
                    &id,
                    "the direct graph differs from the shared expected document",
                );
            }
            Ok(Outcome::Direct { expected })
        }
        CaseWire::Refused {
            id,
            dialect,
            operator_profile,
            semantic_profile,
            source,
            steps,
            expected_refusal,
        } => {
            let steps = decode_steps(&id, steps)?;
            check_dialect(&id, &dialect, &steps)?;
            check_operator_profile(&id, &operator_profile)?;
            if !matches!(steps.last(), Some(Step::Lower(_))) {
                return fail(
                    Code::MissingRefusal,
                    &id,
                    "a refused case ends in a lower step",
                );
            }
            // Refused requests may carry deliberately inconsistent spans, but each
            // supplied span still names bytes of the bound source.
            for step in &steps {
                match step {
                    Step::Append(node) if node.span.is_some() => {
                        span_text(&id, &source, node.span)?;
                    }
                    Step::Append(_) => {}
                    Step::Lower(lower) => {
                        for bounds in [lower.operator_span, lower.expression_span]
                            .into_iter()
                            .flatten()
                        {
                            span_text(&id, &source, bounds.as_span())?;
                        }
                    }
                }
            }
            let execution = execute(&id, semantic_profile, steps, true)?;
            let Some(refusal) = execution.refusal else {
                return fail(
                    Code::MissingRefusal,
                    &id,
                    "the final lower step was admitted",
                );
            };
            if refusal.code() != expected_refusal.code
                || axis_name(refusal.axis()) != expected_refusal.axis
            {
                return fail(
                    Code::RefusalMismatch,
                    &id,
                    format!(
                        "refused {} on {}, recorded {} on {}",
                        refusal.code(),
                        axis_name(refusal.axis()),
                        expected_refusal.code,
                        expected_refusal.axis
                    ),
                );
            }
            Ok(Outcome::Refused {
                code: refusal.code(),
            })
        }
        CaseWire::Malformed {
            id,
            entry,
            expected_error,
        } => {
            if !top_level {
                return fail(Code::NestedMalformed, &id, "malformed cases do not nest");
            }
            match replay_case(corpus, entry, false) {
                Ok(_) => fail(
                    Code::MalformedCaseAccepted,
                    &id,
                    "the malformed entry replayed",
                ),
                Err(error) if error.code.as_str() == expected_error => Ok(Outcome::Malformed {
                    code: error.code.as_str(),
                }),
                Err(error) => fail(
                    Code::MalformedErrorMismatch,
                    &id,
                    format!(
                        "failed with {} ({}), recorded {expected_error}",
                        error.code.as_str(),
                        error.detail
                    ),
                ),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Mutation helpers
// ---------------------------------------------------------------------------

fn json(files: &CorpusFiles, path: &str) -> Value {
    serde_json::from_slice(&files[path]).expect("corpus JSON")
}

fn put_json(files: &mut CorpusFiles, path: &str, value: &Value) {
    let mut bytes = serde_json::to_vec_pretty(value).expect("serialize JSON");
    bytes.push(b'\n');
    files.insert(path.to_owned(), bytes);
}

/// Recomputes every manifest pin and returns the new manifest digest, so a
/// mutation reaches the semantic replay instead of stopping at a digest check.
fn repin(files: &mut CorpusFiles) -> String {
    let mut manifest = json(files, MANIFEST);
    let pins: Vec<Value> = files
        .iter()
        .filter(|(path, _)| path.as_str() != MANIFEST)
        .map(|(path, bytes)| serde_json::json!({ "path": path, "sha256": sha256_hex(bytes) }))
        .collect();
    manifest["files"] = Value::Array(pins);
    put_json(files, MANIFEST, &manifest);
    sha256_hex(&files[MANIFEST])
}

fn mutate_json(files: &mut CorpusFiles, path: &str, mutate: impl FnOnce(&mut Value)) {
    let mut value = json(files, path);
    mutate(&mut value);
    put_json(files, path, &value);
}

fn mutate_case(files: &mut CorpusFiles, id: &str, mutate: impl FnOnce(&mut Value)) {
    mutate_json(files, CASES, |cases| {
        let case = cases["cases"]
            .as_array_mut()
            .expect("case array")
            .iter_mut()
            .find(|case| case["id"] == id)
            .unwrap_or_else(|| panic!("no case {id}"));
        mutate(case);
    });
}

fn remove_cases(files: &mut CorpusFiles, remove: impl Fn(&Value) -> bool) {
    mutate_json(files, CASES, |cases| {
        cases["cases"]
            .as_array_mut()
            .expect("case array")
            .retain(|case| !remove(case));
    });
}

fn replay_code(files: &CorpusFiles, pin: &str) -> Code {
    match replay(files, pin) {
        Ok(summary) => panic!("the mutated corpus replayed: {summary:?}"),
        Err(error) => error.code,
    }
}

fn assert_mutation(code: Code, subject: &str, mutate: impl FnOnce(&mut CorpusFiles)) {
    let mut files = load_corpus();
    mutate(&mut files);
    let pin = repin(&mut files);
    let error = replay(&files, &pin).expect_err("the mutated corpus replayed");
    assert_eq!(
        (error.code, error.subject.as_str()),
        (code, subject),
        "mutation failed for the wrong reason: {}",
        error.detail
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// Trace: TC-074, FR-010-AC-2
#[test]
fn paired_corpus_replays_through_the_lowering_api() {
    let files = load_corpus();
    let summary = replay(&files, MANIFEST_SHA256).unwrap_or_else(|error| {
        panic!(
            "{} at {}: {}",
            error.code.as_str(),
            error.subject,
            error.detail
        )
    });
    assert_eq!(
        (
            summary.derived,
            summary.direct,
            summary.refused,
            summary.malformed
        ),
        (9, 9, 12, 15)
    );
    assert_eq!(
        summary.refusal_codes,
        BTreeSet::from([
            "inverted_interval",
            "interval_bound_out_of_range",
            "missing_interval",
            "operand_absent",
            "operator_span_outside_expression",
            "semantic_profile_mismatch",
            "span_half_missing",
            "unknown_kind",
            "unknown_operator_profile",
            "unknown_request_identity",
            "unknown_semantic_profile",
            "unsupported_kind",
        ])
    );
    assert_eq!(
        summary.malformed_codes,
        BTreeSet::from([
            "case_decode_rejected",
            "dialect_refuses_derived",
            "direct_case_carries_span",
            "direct_case_lowers",
            "document_mismatch",
            "graph_invalid",
            "lowering_report_mismatch",
            "missing_refusal",
            "node_decode_rejected",
            "override_outside_refused",
            "refusal_mismatch",
            "source_binding_mismatch",
            "unknown_dialect",
            "unpinned_expected_document",
        ])
    );
}

// Trace: TC-074, FR-010-AC-2
#[test]
fn make_digest_list_names_exactly_the_manifest_and_its_pins() {
    let files = load_corpus();
    let listed = fs::read_to_string(corpus_root().join("SHA256SUMS")).expect("SHA256SUMS");
    let listed: BTreeMap<String, String> = listed
        .lines()
        .map(|line| {
            let (digest, path) = line.split_once("  ").expect("sha256sum line");
            let path = path
                .strip_prefix(&format!("{CORPUS_DIRECTORY}/"))
                .expect("repository-relative corpus path");
            (path.to_owned(), digest.to_owned())
        })
        .collect();
    let computed: BTreeMap<String, String> = files
        .iter()
        .map(|(path, bytes)| (path.clone(), sha256_hex(bytes)))
        .collect();
    assert_eq!(listed, computed);
    assert_eq!(computed[MANIFEST], MANIFEST_SHA256);
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_the_lowering_branch_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "weak-until-closed-derived",
        |files| {
            mutate_json(files, "expected/weak-until-closed.json", |document| {
                document["nodes"][4]["kind"] = "and".into();
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_generated_node_order_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "strong-release-online-derived",
        |files| {
            mutate_json(files, "expected/strong-release-online.json", |document| {
                document["nodes"].as_array_mut().expect("nodes").swap(2, 3);
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_an_inclusive_endpoint_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "strong-release-closed-derived",
        |files| {
            mutate_json(files, "expected/strong-release-closed.json", |document| {
                for index in [2, 3] {
                    document["nodes"][index]["interval"]["end"] = 2.into();
                }
            });
        },
    );
    assert_mutation(
        Code::DocumentMismatch,
        "strong-release-max-singleton-derived",
        |files| {
            mutate_json(
                files,
                "expected/strong-release-max-singleton.json",
                |document| {
                    for index in [2, 3] {
                        document["nodes"][index]["interval"]["start"] = 4_294_967_294_u64.into();
                    }
                },
            );
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_associativity_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "left-associative-chain-derived",
        |files| {
            mutate_json(files, "expected/left-associative-chain.json", |document| {
                document["nodes"][6]["left"] = 1.into();
                document["nodes"][7]["operand"] = 1.into();
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_the_selected_profile_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "weak-until-online-derived",
        |files| {
            mutate_json(files, "expected/weak-until-online.json", |document| {
                document["semantic_profile"] = "mltl.closed-trace/v1".into();
            });
        },
    );
    assert_mutation(
        Code::DocumentMismatch,
        "right-nested-release-derived",
        |files| {
            mutate_case(files, "right-nested-release-derived", |case| {
                case["semantic_profile"] = "mltl.closed-trace/v1".into();
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_the_node_charge_changes_the_graph_or_report() {
    assert_mutation(
        Code::LoweringReportMismatch,
        "left-associative-chain-derived",
        |files| {
            mutate_case(files, "left-associative-chain-derived", |case| {
                case["expected_lowerings"][1]["first_generated"] = 5.into();
                case["expected_lowerings"][1]["root"] = 7.into();
            });
        },
    );
    assert_mutation(
        Code::DocumentMismatch,
        "compound-operands-derived",
        |files| {
            mutate_json(files, "expected/compound-operands.json", |document| {
                let nodes = document["nodes"].as_array_mut().expect("nodes");
                nodes.remove(6);
                nodes[6]["right"] = 5.into();
                document["root"] = 6.into();
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_span_attribution_turns_the_replay_red() {
    assert_mutation(
        Code::SourceBindingMismatch,
        "right-nested-release-derived",
        |files| {
            mutate_case(files, "right-nested-release-derived", |case| {
                case["steps"][3]["lower"]["operator_span"]["start"] = 13.into();
                case["expected_lowerings"][0]["operator_span"]["start"] = 13.into();
            });
        },
    );
    assert_mutation(
        Code::SourceBindingMismatch,
        "right-nested-release-derived",
        |files| {
            mutate_case(files, "right-nested-release-derived", |case| {
                case["steps"][4]["lower"]["expression_span"]["start"] = 1.into();
                case["expected_lowerings"][1]["expression_span"]["start"] = 1.into();
            });
        },
    );
    assert_mutation(
        Code::LoweringReportMismatch,
        "compound-operands-derived",
        |files| {
            mutate_case(files, "compound-operands-derived", |case| {
                case["expected_lowerings"][0]["expression_span"]["start"] = 1.into();
            });
        },
    );
    assert_mutation(
        Code::SourceBindingMismatch,
        "compound-operands-derived",
        |files| {
            mutate_case(files, "compound-operands-derived", |case| {
                case["steps"][2]["append"]["span"] = serde_json::json!({ "start": 17, "end": 21 });
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_a_refusal_or_malformed_expectation_turns_the_replay_red() {
    assert_mutation(
        Code::RefusalMismatch,
        "refused-inverted-interval",
        |files| {
            mutate_case(files, "refused-inverted-interval", |case| {
                case["expected_refusal"]["code"] = "interval_bound_out_of_range".into();
            });
        },
    );
    assert_mutation(
        Code::RefusalMismatch,
        "refused-span-half-missing",
        |files| {
            mutate_case(files, "refused-span-half-missing", |case| {
                case["expected_refusal"]["axis"] = "interval".into();
            });
        },
    );
    assert_mutation(
        Code::MalformedErrorMismatch,
        "malformed-derived-wire-node",
        |files| {
            mutate_case(files, "malformed-derived-wire-node", |case| {
                case["expected_error"] = "case_decode_rejected".into();
            });
        },
    );
    assert_mutation(
        Code::MalformedCaseAccepted,
        "malformed-operator-span-names-operand",
        |files| {
            mutate_case(files, "malformed-operator-span-names-operand", |case| {
                case["entry"]["steps"][2]["lower"]["operator_span"] =
                    serde_json::json!({ "start": 3, "end": 9 });
                case["entry"]["expected_lowerings"][0]["operator_span"] =
                    serde_json::json!({ "start": 3, "end": 9 });
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn changed_pinned_bytes_turn_the_replay_red() {
    let files = load_corpus();

    let mut flipped = files.clone();
    flipped
        .get_mut("expected/weak-until-closed.json")
        .expect("expected document")[0] ^= 0x20;
    assert_eq!(
        replay_code(&flipped, MANIFEST_SHA256),
        Code::FileDigestMismatch
    );

    let mut manifest = files.clone();
    mutate_json(&mut manifest, MANIFEST, |value| {
        value["revision"] = 2.into();
    });
    assert_eq!(
        replay_code(&manifest, MANIFEST_SHA256),
        Code::ManifestDigestMismatch
    );

    let mut revised = files.clone();
    mutate_json(&mut revised, MANIFEST, |value| {
        value["revision"] = 2.into();
    });
    let pin = sha256_hex(&revised[MANIFEST]);
    assert_eq!(replay_code(&revised, &pin), Code::CorpusIdentityMismatch);

    let mut extra = files.clone();
    extra.insert("expected/unlisted.json".to_owned(), b"{}".to_vec());
    assert_eq!(replay_code(&extra, MANIFEST_SHA256), Code::UnpinnedFile);

    let mut missing = files;
    missing.remove("expected/compound-operands.json");
    assert_eq!(replay_code(&missing, MANIFEST_SHA256), Code::MissingFile);
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn removing_required_coverage_turns_the_replay_red() {
    assert_mutation(Code::ClassAbsent, "refused", |files| {
        remove_cases(files, |case| case["class"] == "refused");
    });
    assert_mutation(
        Code::UnpairedExpectedDocument,
        "expected/compound-operands.json",
        |files| {
            remove_cases(files, |case| case["id"] == "compound-operands-direct");
        },
    );
    assert_mutation(Code::ProfileRowAbsent, "W", |files| {
        remove_cases(files, |case| {
            case["id"] == "weak-until-online-derived" || case["id"] == "weak-until-online-direct"
        });
        files.remove("expected/weak-until-online.json");
        // The right-nested case is the only other online W lowering.
        remove_cases(files, |case| {
            case["id"] == "right-nested-release-derived"
                || case["id"] == "right-nested-release-direct"
        });
        files.remove("expected/right-nested-release.json");
    });
    assert_mutation(Code::BoundaryAbsent, CASES, |files| {
        remove_cases(files, |case| {
            case["id"] == "weak-until-zero-singleton-derived"
                || case["id"] == "weak-until-zero-singleton-direct"
        });
        files.remove("expected/weak-until-zero-singleton.json");
    });
    assert_mutation(Code::NestingAbsent, CASES, |files| {
        // The right-nested case is the only lowering whose right operand is a lowered root.
        remove_cases(files, |case| {
            case["id"] == "right-nested-release-derived"
                || case["id"] == "right-nested-release-direct"
        });
        files.remove("expected/right-nested-release.json");
    });
    assert_mutation(Code::DuplicateCaseId, "refused-next-unsupported", |files| {
        mutate_case(files, "refused-unknown-kind", |case| {
            case["id"] = "refused-next-unsupported".into();
        });
    });
}
