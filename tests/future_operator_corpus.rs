#![cfg(feature = "serde")]
//! TC-074: replay the paired W/M source and canonical-graph corpus.
//!
//! `corpus/future-operators/` is evidence input. Each derived-source case binds
//! a dialect, operator profile, semantic profile, and source text to ordered
//! append and lower steps; the replay builds its document only through
//! [`Formula::new`] and [`FutureLoweringRequest::lower`] and compares it with a
//! span-free expected formula-v1 document that a directly constructed case
//! shares. A primitive-source case binds `tl-parse.clean-ascii/v1` text to an
//! append-only graph the same way. The replay evaluates nothing and admits no
//! derived wire node.
//!
//! The replay binds every span to the bytes and operator spellings of its
//! source, but it does not parse: precedence, associativity, and grouping are
//! grammar rules owned by tl-parse and TC-043. The manifest names the tl-parse
//! revision the case steps were cross-checked against.

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
    Formula, FormulaDocument, FutureKind, FutureLoweringRefusal, FutureLoweringReport,
    FutureLoweringRequest, Node, NodeId, NodeKind, RawBounds, SemanticProfile, SourceSpan,
    FUTURE_LOWERING_NODE_CHARGE, FUTURE_LOWERING_REPORT_V1, FUTURE_LOWERING_REQUEST_V1,
    FUTURE_OPERATORS_V1,
};

const CORPUS_DIRECTORY: &str = "corpus/future-operators";
const CORPUS_IDENTITY: &str = "tl-syntax.future-operator-corpus/v1";
const CORPUS_REVISION: u64 = 1;
const EVIDENCE_ROLE: &str = "evidence-input";
const DERIVED_DIALECT: &str = "tl-parse.clean-ascii/v2";
const PRIMITIVE_DIALECT: &str = "tl-parse.clean-ascii/v1";
const CROSS_CHECK_PARSER: &str = "tl-parse";
const CROSS_CHECK_ENTRY_POINTS: [&str; 2] = ["parse", "parse_clean_ascii_v2"];
const MANIFEST: &str = "manifest.json";
const CASES: &str = "cases.json";
const EXPECTED_PREFIX: &str = "expected/";
/// Files the replay does not read: prose, and the `make check-corpus` digest list.
const UNREPLAYED: [&str; 2] = ["README.md", "SHA256SUMS"];
const KINDS: [FutureKind; 2] = [FutureKind::WeakUntil, FutureKind::StrongRelease];
const PROFILES: [SemanticProfile; 2] = [
    SemanticProfile::ClosedTraceV1,
    SemanticProfile::OnlinePrefixV1,
];
const BOUNDARIES: [(u32, u32); 2] = [(0, 0), (u32::MAX, u32::MAX)];

/// SHA-256 of `corpus/future-operators/manifest.json`; the manifest pins every other file.
const MANIFEST_SHA256: &str = "e38ef2a7bfc49631932c9c8527b9d08ba1087825e8ae3bccff5f326e74605172";

/// Stable replay failure classes. Tests and malformed cases match these, never
/// diagnostic text; `cases.json` spells them in snake case.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
enum Code {
    MissingFile,
    ManifestDigestMismatch,
    ManifestDecodeRejected,
    ManifestPinsItself,
    DuplicatePin,
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
    PrimitiveDialectMismatch,
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

#[derive(Debug)]
struct ReplayError {
    code: Code,
    /// Case identity, corpus path, or coverage cell the failure belongs to.
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
    primitive_dialect: String,
    source_cross_check: CrossCheckWire,
    files: Vec<PinWire>,
}

/// The out-of-band parser run the case steps were checked against. tl-syntax
/// cannot depend on tl-parse, so this records the revision and does not re-run it.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CrossCheckWire {
    parser: String,
    revision: String,
    entry_points: Vec<String>,
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

/// The stable refusal code. The code determines the admission axis inside
/// tl-syntax, so the corpus records no second, test-invented axis name.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RefusalRecord {
    code: String,
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
    Primitive {
        id: String,
        dialect: String,
        semantic_profile: SemanticProfile,
        source: String,
        steps: Vec<StepWire>,
        root: u32,
        expected: String,
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
        expected_error: Code,
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
    primitive: usize,
    direct: usize,
    refused: usize,
    malformed: usize,
    malformed_codes: BTreeSet<Code>,
    refusal_codes: BTreeSet<&'static str>,
}

enum Outcome {
    Derived {
        expected: String,
        reports: Vec<FutureLoweringReport>,
    },
    Primitive {
        expected: String,
    },
    Direct {
        expected: String,
    },
    Refused {
        code: &'static str,
    },
    Malformed {
        code: Code,
    },
}

struct Corpus {
    expected: BTreeMap<String, FormulaDocument>,
}

fn check_manifest_identity(manifest: &ManifestWire) -> Replay<()> {
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
        (manifest.primitive_dialect.as_str(), PRIMITIVE_DIALECT),
        (
            manifest.source_cross_check.parser.as_str(),
            CROSS_CHECK_PARSER,
        ),
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
    let revision = &manifest.source_cross_check.revision;
    let full_commit = revision.len() == 40
        && revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !full_commit || manifest.source_cross_check.entry_points != CROSS_CHECK_ENTRY_POINTS {
        return fail(
            Code::CorpusIdentityMismatch,
            MANIFEST,
            "the source cross-check names a full tl-parse commit and both parser entry points",
        );
    }
    Ok(())
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
    check_manifest_identity(&manifest)?;

    let mut pins = BTreeMap::new();
    for pin in &manifest.files {
        if pin.path == MANIFEST {
            return fail(
                Code::ManifestPinsItself,
                MANIFEST,
                "the test pins the manifest; the manifest cannot pin itself",
            );
        }
        if pins.insert(pin.path.clone(), pin.sha256.clone()).is_some() {
            return fail(
                Code::DuplicatePin,
                &pin.path,
                "the manifest pins this path twice",
            );
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

    let cases: CasesWire = serde_json::from_slice(&files[CASES])
        .map_err(|error| ReplayError::new(Code::CasesDecodeRejected, CASES, error.to_string()))?;
    if cases.corpus != CORPUS_IDENTITY {
        return fail(Code::CorpusIdentityMismatch, CASES, cases.corpus);
    }

    let mut summary = Summary::default();
    let mut ids = BTreeSet::new();
    let mut source_documents = BTreeSet::new();
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
                    let (kind, profile) = (report.kind(), report.semantic_profile());
                    rows.insert((kind, profile));
                    let first = report.first_generated().0;
                    let interval = interval_of(&corpus.expected[&expected], first);
                    intervals.insert((kind, profile, interval));
                    nested_left |= generated_roots.contains(&report.left());
                    nested_right |= generated_roots.contains(&report.right());
                    generated_roots.insert(report.root());
                }
                source_documents.insert(expected);
            }
            Outcome::Primitive { expected } => {
                summary.primitive += 1;
                source_documents.insert(expected);
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

    check_census(
        &summary,
        &rows,
        &intervals,
        (nested_left, nested_right),
        &corpus,
        (&source_documents, &direct_documents),
    )?;
    Ok(summary)
}

type Row = (FutureKind, SemanticProfile);
type Cell = (FutureKind, SemanticProfile, Option<(u32, u32)>);

fn check_census(
    summary: &Summary,
    rows: &BTreeSet<Row>,
    intervals: &BTreeSet<Cell>,
    (nested_left, nested_right): (bool, bool),
    corpus: &Corpus,
    (source_documents, direct_documents): (&BTreeSet<String>, &BTreeSet<String>),
) -> Replay<()> {
    for (class, count) in [
        ("derived", summary.derived),
        ("primitive", summary.primitive),
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
    for kind in KINDS {
        for profile in PROFILES {
            if !rows.contains(&(kind, profile)) {
                return fail(
                    Code::ProfileRowAbsent,
                    kind.as_str(),
                    format!("no derived lowering under {}", profile.as_str()),
                );
            }
            for (start, end) in BOUNDARIES {
                if !intervals.contains(&(kind, profile, Some((start, end)))) {
                    return fail(
                        Code::BoundaryAbsent,
                        &format!("{} {} [{start},{end}]", kind.as_str(), profile.as_str()),
                        "no derived lowering over this boundary",
                    );
                }
            }
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
        if !(source_documents.contains(path) && direct_documents.contains(path)) {
            return fail(
                Code::UnpairedExpectedDocument,
                path,
                "each expected document needs a source case and a direct case",
            );
        }
    }
    Ok(())
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

/// Derived and refused cases carry derived source, which only v2 spells.
fn check_derived_dialect(id: &str, dialect: &str) -> Replay<()> {
    match dialect {
        DERIVED_DIALECT => Ok(()),
        PRIMITIVE_DIALECT => fail(
            Code::DialectRefusesDerived,
            id,
            "tl-parse.clean-ascii/v1 has no derived future operator",
        ),
        other => fail(Code::UnknownDialect, id, other),
    }
}

/// Primitive-source cases are the v1 compatibility pair and never lower.
fn check_primitive_dialect(id: &str, dialect: &str, steps: &[Step]) -> Replay<()> {
    match dialect {
        PRIMITIVE_DIALECT if has_lower(steps) => fail(
            Code::DialectRefusesDerived,
            id,
            "tl-parse.clean-ascii/v1 has no derived future operator",
        ),
        PRIMITIVE_DIALECT => Ok(()),
        DERIVED_DIALECT => fail(
            Code::PrimitiveDialectMismatch,
            id,
            "the compatibility pair binds tl-parse.clean-ascii/v1 source",
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

fn source_slice<'s>(id: &str, source: &'s str, start: u32, end: u32) -> Replay<&'s str> {
    let range = usize::try_from(start).expect("u32 fits usize")
        ..usize::try_from(end).expect("u32 fits usize");
    source.get(range).ok_or_else(|| {
        ReplayError::new(
            Code::SourceBindingMismatch,
            id,
            format!("bytes {start}..{end} are not a slice of the source"),
        )
    })
}

fn span_text<'s>(id: &str, source: &'s str, span: Option<SourceSpan>) -> Replay<&'s str> {
    let span = span.ok_or_else(|| {
        ReplayError::new(Code::SourceBindingMismatch, id, "source nodes carry spans")
    })?;
    source_slice(id, source, span.start(), span.end())
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

/// The source spelling of a node's own token: a leaf's whole text, or the
/// operator between (or before) its operands.
fn spelling(kind: NodeKind) -> String {
    let timed = |operator: &str, start: u32, end: u32| format!("{operator}[{start},{end}]");
    match kind {
        NodeKind::False => "false".to_owned(),
        NodeKind::True => "true".to_owned(),
        NodeKind::Proposition { proposition } => format!("p{}", proposition.0),
        NodeKind::Not { .. } => "!".to_owned(),
        NodeKind::And { .. } => "&".to_owned(),
        NodeKind::Or { .. } => "|".to_owned(),
        NodeKind::Implies { .. } => "->".to_owned(),
        NodeKind::Equivalent { .. } => "<->".to_owned(),
        NodeKind::Future { interval, .. } => timed("F", interval.start(), interval.end()),
        NodeKind::Globally { interval, .. } => timed("G", interval.start(), interval.end()),
        NodeKind::Until { interval, .. } => timed("U", interval.start(), interval.end()),
        NodeKind::Release { interval, .. } => timed("R", interval.start(), interval.end()),
    }
}

/// Text between tokens may hold only whitespace and the named grouping character.
fn only_grouping(text: &str, grouping: char) -> bool {
    text.chars()
        .all(|character| character.is_whitespace() || character == grouping)
}

fn is_grouping(character: char) -> bool {
    character.is_whitespace() || character == '(' || character == ')'
}

fn node_span(nodes: &[Node], id: NodeId) -> Option<SourceSpan> {
    nodes.get(usize::try_from(id.0).ok()?)?.span
}

/// Checks that a span reads `(`* left operand, operator, right operand `)`*, or
/// operator `(`* operand `)`* for a unary node.
fn check_token_layout(
    id: &str,
    source: &str,
    span: SourceSpan,
    operand_spans: &[SourceSpan],
    spelled: &str,
) -> Replay<()> {
    let separated = match operand_spans {
        [operand] => {
            let prefix = source_slice(id, source, span.start(), operand.start())?;
            let suffix = source_slice(id, source, operand.end(), span.end())?;
            prefix.trim_end_matches(|c: char| c.is_whitespace() || c == '(') == spelled
                && only_grouping(suffix, ')')
        }
        [left, right] => {
            let lead = source_slice(id, source, span.start(), left.start())?;
            let middle = source_slice(id, source, left.end(), right.start())?;
            let tail = source_slice(id, source, right.end(), span.end())?;
            only_grouping(lead, '(')
                && middle.trim_matches(is_grouping) == spelled
                && only_grouping(tail, ')')
        }
        _ => true,
    };
    if separated {
        Ok(())
    } else {
        fail(
            Code::SourceBindingMismatch,
            id,
            format!(
                "{:?} does not read as {spelled:?} around its operands",
                source_slice(id, source, span.start(), span.end())?
            ),
        )
    }
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
        let (Some(expression), Some(operator)) = (expression, report.operator_span()) else {
            return fail(
                Code::SourceBindingMismatch,
                id,
                "derived lowerings carry spans",
            );
        };
        let (Some(left), Some(right)) = (
            node_span(nodes, report.left()),
            node_span(nodes, report.right()),
        ) else {
            return fail(
                Code::SourceBindingMismatch,
                id,
                "derived operands carry spans",
            );
        };
        let ordered = expression.start() <= left.start()
            && left.end() <= operator.start()
            && operator.end() <= right.start()
            && right.end() <= expression.end();
        if !ordered {
            return fail(
                Code::SourceBindingMismatch,
                id,
                "the expression must read left operand, operator, right operand in order",
            );
        }
        let separated = only_grouping(
            source_slice(id, source, expression.start(), left.start())?,
            '(',
        ) && only_grouping(
            source_slice(id, source, left.end(), operator.start())?,
            ')',
        ) && only_grouping(
            source_slice(id, source, operator.end(), right.start())?,
            '(',
        ) && only_grouping(
            source_slice(id, source, right.end(), expression.end())?,
            ')',
        );
        if !separated {
            return fail(
                Code::SourceBindingMismatch,
                id,
                format!("{spelled:?} is separated from its operands by more than grouping"),
            );
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        if generated.contains(&u32::try_from(index).expect("small graph")) {
            continue;
        }
        let text = span_text(id, source, node.span)?;
        let span = node.span.expect("span_text accepted the span");
        let spelled = spelling(node.kind);
        let mut operand_spans = Vec::new();
        for operand in operands(node.kind) {
            if !covers(span, node_span(nodes, operand)) {
                return fail(
                    Code::SourceBindingMismatch,
                    id,
                    format!("node {index} does not cover operand {}", operand.0),
                );
            }
            operand_spans.push(node_span(nodes, operand).expect("covers checked the span"));
        }
        if operand_spans.is_empty() {
            if text != spelled {
                return fail(
                    Code::SourceBindingMismatch,
                    id,
                    format!("node {index} reads {text:?}, its kind spells {spelled:?}"),
                );
            }
        } else {
            check_token_layout(id, source, span, &operand_spans, &spelled)?;
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

fn check_reports(
    id: &str,
    profile: SemanticProfile,
    execution: &Execution,
    records: &[LoweringRecord],
) -> Replay<()> {
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
        let matches = report.identity() == FUTURE_LOWERING_REPORT_V1
            && report.kind().as_str() == record.kind
            && report.semantic_profile() == profile
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

/// Builds a source-bound graph and compares it, without spans, to the shared document.
fn build_source_document(
    id: &str,
    profile: SemanticProfile,
    root: u32,
    execution: &Execution,
    document: &FormulaDocument,
) -> Replay<()> {
    let built = FormulaDocument::new(profile, NodeId(root), execution.nodes.clone())
        .map_err(|error| ReplayError::new(Code::GraphInvalid, id, error.to_string()))?;
    if built.semantic_view() != document.semantic_view()
        || semantic_bytes(&built) != semantic_bytes(document)
    {
        return fail(
            Code::DocumentMismatch,
            id,
            "the source graph differs from the shared expected document",
        );
    }
    Ok(())
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
            check_derived_dialect(&id, &dialect)?;
            check_operator_profile(&id, &operator_profile)?;
            if !has_lower(&steps) {
                return fail(Code::DerivedCaseWithoutLowering, &id, "no lower step");
            }
            let document = expected_document(corpus, &id, &expected)?;
            let execution = execute(&id, semantic_profile, steps, false)?;
            check_source_binding(&id, &source, &execution, root)?;
            check_reports(&id, semantic_profile, &execution, &expected_lowerings)?;
            build_source_document(&id, semantic_profile, root, &execution, document)?;
            Ok(Outcome::Derived {
                expected,
                reports: execution
                    .lowerings
                    .into_iter()
                    .map(|performed| performed.report)
                    .collect(),
            })
        }
        CaseWire::Primitive {
            id,
            dialect,
            semantic_profile,
            source,
            steps,
            root,
            expected,
        } => {
            let steps = decode_steps(&id, steps)?;
            check_primitive_dialect(&id, &dialect, &steps)?;
            let document = expected_document(corpus, &id, &expected)?;
            let execution = execute(&id, semantic_profile, steps, false)?;
            check_source_binding(&id, &source, &execution, root)?;
            build_source_document(&id, semantic_profile, root, &execution, document)?;
            Ok(Outcome::Primitive { expected })
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
            check_derived_dialect(&id, &dialect)?;
            check_operator_profile(&id, &operator_profile)?;
            if !matches!(steps.last(), Some(Step::Lower(_))) {
                return fail(
                    Code::MissingRefusal,
                    &id,
                    "a refused case ends in a lower step",
                );
            }
            // Refused requests may carry deliberately inconsistent spans. Every
            // span that is representable still names bytes of the bound source;
            // unrepresentable bounds are exactly what the lowering API refuses.
            for step in &steps {
                match step {
                    Step::Append(node) if node.span.is_some() => {
                        span_text(&id, &source, node.span)?;
                    }
                    Step::Append(_) => {}
                    Step::Lower(lower) => {
                        for span in [lower.operator_span, lower.expression_span]
                            .into_iter()
                            .flatten()
                            .filter_map(Bounds::as_span)
                        {
                            span_text(&id, &source, Some(span))?;
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
            if refusal.code() != expected_refusal.code {
                return fail(
                    Code::RefusalMismatch,
                    &id,
                    format!(
                        "refused {}, recorded {}",
                        refusal.code(),
                        expected_refusal.code
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
                Err(error) if error.code == expected_error => {
                    Ok(Outcome::Malformed { code: error.code })
                }
                Err(error) => fail(
                    Code::MalformedErrorMismatch,
                    &id,
                    format!(
                        "failed with {:?} ({}), recorded {expected_error:?}",
                        error.code, error.detail
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

/// Removes a derived/direct pair and the expected document they share.
fn remove_pair(files: &mut CorpusFiles, stem: &str) {
    let derived = format!("{stem}-derived");
    let direct = format!("{stem}-direct");
    remove_cases(files, |case| {
        case["id"] == derived.as_str() || case["id"] == direct.as_str()
    });
    files
        .remove(&format!("{EXPECTED_PREFIX}{stem}.json"))
        .unwrap_or_else(|| panic!("no expected document for {stem}"));
}

fn replay_code(files: &CorpusFiles, pin: &str) -> Code {
    match replay(files, pin) {
        Ok(summary) => panic!("the mutated corpus replayed: {summary:?}"),
        Err(error) => error.code,
    }
}

fn assert_failure(files: &CorpusFiles, pin: &str, code: Code, subject: &str) {
    let error = replay(files, pin).expect_err("the mutated corpus replayed");
    assert_eq!(
        (error.code, error.subject.as_str()),
        (code, subject),
        "mutation failed for the wrong reason: {}",
        error.detail
    );
}

/// Mutates corpus files, re-pins them, and requires the named failure.
fn assert_mutation(code: Code, subject: &str, mutate: impl FnOnce(&mut CorpusFiles)) {
    let mut files = load_corpus();
    mutate(&mut files);
    let pin = repin(&mut files);
    assert_failure(&files, &pin, code, subject);
}

/// Mutates the re-pinned manifest itself, re-pins its digest, and requires the named failure.
fn assert_manifest_mutation(code: Code, subject: &str, mutate: impl FnOnce(&mut Value)) {
    let mut files = load_corpus();
    repin(&mut files);
    mutate_json(&mut files, MANIFEST, mutate);
    let pin = sha256_hex(&files[MANIFEST]);
    assert_failure(&files, &pin, code, subject);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// Trace: TC-074, FR-010-AC-2
#[test]
fn paired_corpus_replays_through_the_lowering_api() {
    let files = load_corpus();
    let summary = replay(&files, MANIFEST_SHA256)
        .unwrap_or_else(|error| panic!("{:?} at {}: {}", error.code, error.subject, error.detail));
    assert_eq!(
        (
            summary.derived,
            summary.primitive,
            summary.direct,
            summary.refused,
            summary.malformed
        ),
        (15, 1, 16, 16, 21)
    );
    assert_eq!(
        summary.refusal_codes,
        BTreeSet::from([
            "inverted_interval",
            "inverted_span",
            "interval_bound_out_of_range",
            "missing_interval",
            "operand_absent",
            "operand_id_out_of_range",
            "operator_span_outside_expression",
            "semantic_profile_mismatch",
            "span_endpoint_out_of_range",
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
            Code::CaseDecodeRejected,
            Code::DerivedCaseWithoutLowering,
            Code::DialectRefusesDerived,
            Code::DirectCaseCarriesSpan,
            Code::DirectCaseLowers,
            Code::DocumentMismatch,
            Code::GraphInvalid,
            Code::LoweringReportMismatch,
            Code::MissingRefusal,
            Code::NestedMalformed,
            Code::NodeDecodeRejected,
            Code::OverrideOutsideRefused,
            Code::PrimitiveDialectMismatch,
            Code::RefusalMismatch,
            Code::SourceBindingMismatch,
            Code::UnexpectedRefusal,
            Code::UnknownDialect,
            Code::UnpinnedExpectedDocument,
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
fn mutating_either_lowering_branch_changes_the_expected_graph() {
    assert_mutation(
        Code::DocumentMismatch,
        "weak-until-closed-derived",
        |files| {
            mutate_json(files, "expected/weak-until-closed.json", |document| {
                document["nodes"][4]["kind"] = "and".into();
            });
        },
    );
    assert_mutation(
        Code::DocumentMismatch,
        "strong-release-closed-derived",
        |files| {
            mutate_json(files, "expected/strong-release-closed.json", |document| {
                document["nodes"][4]["kind"] = "or".into();
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
    assert_mutation(
        Code::DocumentMismatch,
        "weak-until-online-derived",
        |files| {
            mutate_json(files, "expected/weak-until-online.json", |document| {
                document["nodes"].as_array_mut().expect("nodes").swap(2, 3);
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn mutating_an_inclusive_endpoint_changes_the_expected_graph() {
    let endpoints = [
        ("strong-release-closed", "end", 2_u64),
        (
            "strong-release-max-singleton-online",
            "start",
            4_294_967_294,
        ),
        ("weak-until-closed", "end", 1),
        ("weak-until-max-singleton-closed", "start", 4_294_967_294),
    ];
    for (stem, endpoint, value) in endpoints {
        assert_mutation(
            Code::DocumentMismatch,
            &format!("{stem}-derived"),
            |files| {
                mutate_json(files, &format!("expected/{stem}.json"), |document| {
                    for index in [2, 3] {
                        document["nodes"][index]["interval"][endpoint] = value.into();
                    }
                });
            },
        );
    }
}

// The replay does not parse, so this proves the graph comparison detects a
// regrouped expectation; TC-043 owns which grouping a source text denotes.
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
    assert_mutation(
        Code::SourceBindingMismatch,
        "compound-operands-derived",
        |files| {
            mutate_case(files, "compound-operands-derived", |case| {
                case["source"] = "!p0 W[0,2] (p1 | true)".into();
            });
        },
    );
    assert_mutation(
        Code::SourceBindingMismatch,
        "primitive-until-or-globally-source",
        |files| {
            mutate_case(files, "primitive-until-or-globally-source", |case| {
                case["source"] = "(p0 U[1,2] p1) | F[1,2] p0".into();
            });
        },
    );
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn a_generated_node_without_the_expression_span_turns_the_replay_red() {
    let files = load_corpus();
    let case = json(&files, CASES)["cases"]
        .as_array()
        .expect("case array")
        .iter()
        .find(|case| case["id"] == "weak-until-closed-derived")
        .cloned()
        .expect("weak-until-closed-derived");
    let CaseWire::Derived {
        id,
        semantic_profile,
        source,
        steps,
        root,
        ..
    } = serde_json::from_value(case).expect("derived case")
    else {
        panic!("weak-until-closed-derived is a derived case");
    };
    let steps = decode_steps(&id, steps).expect("steps decode");
    let mut execution = execute(&id, semantic_profile, steps, false).expect("case lowers");
    check_source_binding(&id, &source, &execution, root).expect("the unmutated case binds");
    execution.nodes[3].span = SourceSpan::new(0, 2).ok();
    let error = check_source_binding(&id, &source, &execution, root)
        .expect_err("a generated node lost its expression span");
    assert_eq!(
        (error.code, error.subject.as_str()),
        (Code::GeneratedSpanMismatch, "weak-until-closed-derived")
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
                case["expected_refusal"]["code"] = "inverted_span".into();
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
        Code::CaseDecodeRejected,
        "malformed-derived-wire-node",
        |files| {
            mutate_case(files, "malformed-derived-wire-node", |case| {
                case["expected_error"] = "node_decode_rejectd".into();
            });
        },
    );
    assert_mutation(
        Code::OperatorProfileBindingMismatch,
        "weak-until-closed-derived",
        |files| {
            mutate_case(files, "weak-until-closed-derived", |case| {
                case["operator_profile"] = "tl-syntax.future-operators/v2".into();
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

    let mut extra = files.clone();
    extra.insert("expected/unlisted.json".to_owned(), b"{}".to_vec());
    assert_eq!(replay_code(&extra, MANIFEST_SHA256), Code::UnpinnedFile);

    let mut missing = files;
    missing.remove("expected/compound-operands.json");
    assert_eq!(replay_code(&missing, MANIFEST_SHA256), Code::MissingFile);
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn manifest_identity_and_pin_faults_turn_the_replay_red() {
    assert_manifest_mutation(Code::CorpusIdentityMismatch, MANIFEST, |manifest| {
        manifest["revision"] = 2.into();
    });
    assert_manifest_mutation(Code::CorpusIdentityMismatch, MANIFEST, |manifest| {
        manifest["source_cross_check"]["revision"] = "9ca856b".into();
    });
    assert_manifest_mutation(Code::CorpusIdentityMismatch, MANIFEST, |manifest| {
        manifest["primitive_dialect"] = DERIVED_DIALECT.into();
    });
    assert_manifest_mutation(Code::ManifestDecodeRejected, MANIFEST, |manifest| {
        manifest["notes"] = "unreviewed".into();
    });
    assert_manifest_mutation(Code::ManifestPinsItself, MANIFEST, |manifest| {
        let pin = serde_json::json!({ "path": MANIFEST, "sha256": MANIFEST_SHA256 });
        manifest["files"].as_array_mut().expect("pins").push(pin);
    });
    assert_manifest_mutation(Code::DuplicatePin, CASES, |manifest| {
        let pins = manifest["files"].as_array_mut().expect("pins");
        let first = pins[0].clone();
        pins.push(first);
    });
    assert_mutation(Code::UnpinnedFile, "notes.json", |files| {
        files.insert("notes.json".to_owned(), b"{}".to_vec());
    });
    assert_mutation(Code::MissingFile, CASES, |files| {
        files.remove(CASES);
    });
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn malformed_expected_documents_and_case_files_turn_the_replay_red() {
    assert_mutation(
        Code::ExpectedDocumentCarriesSpan,
        "expected/weak-until-closed.json",
        |files| {
            mutate_json(files, "expected/weak-until-closed.json", |document| {
                document["nodes"][0]["span"] = serde_json::json!({ "start": 0, "end": 2 });
            });
        },
    );
    assert_mutation(
        Code::ExpectedDocumentRejected,
        "expected/weak-until-closed.json",
        |files| {
            mutate_json(files, "expected/weak-until-closed.json", |document| {
                document["nodes"][4]["kind"] = "weak_until".into();
            });
        },
    );
    assert_mutation(Code::CasesDecodeRejected, CASES, |files| {
        mutate_json(files, CASES, |cases| {
            cases["notes"] = "unreviewed".into();
        });
    });
    assert_mutation(Code::CorpusIdentityMismatch, CASES, |files| {
        mutate_json(files, CASES, |cases| {
            cases["corpus"] = "tl-syntax-corpus/v1".into();
        });
    });
}

// Trace: TC-074, FR-010-AC-4
#[test]
fn removing_required_coverage_turns_the_replay_red() {
    assert_mutation(Code::ClassAbsent, "refused", |files| {
        remove_cases(files, |case| case["class"] == "refused");
    });
    assert_mutation(Code::ClassAbsent, "primitive", |files| {
        remove_cases(files, |case| case["class"] == "primitive");
    });
    assert_mutation(
        Code::UnpairedExpectedDocument,
        "expected/compound-operands.json",
        |files| {
            remove_cases(files, |case| case["id"] == "compound-operands-direct");
        },
    );
    assert_mutation(
        Code::UnpairedExpectedDocument,
        "expected/weak-until-closed.json",
        |files| {
            remove_cases(files, |case| case["id"] == "weak-until-closed-derived");
        },
    );
    assert_mutation(Code::ProfileRowAbsent, "W", |files| {
        // Every online W lowering, including the nested one.
        for stem in [
            "weak-until-online",
            "weak-until-zero-singleton-online",
            "weak-until-max-singleton-online",
            "right-nested-release",
        ] {
            remove_pair(files, stem);
        }
    });
    assert_mutation(
        Code::BoundaryAbsent,
        "M mltl.online-prefix/v1 [0,0]",
        |files| {
            remove_pair(files, "strong-release-zero-singleton-online");
        },
    );
    assert_mutation(Code::NestingAbsent, CASES, |files| {
        // The right-nested case is the only lowering whose right operand is a lowered root.
        remove_pair(files, "right-nested-release");
    });
    assert_mutation(Code::DuplicateCaseId, "refused-next-unsupported", |files| {
        mutate_case(files, "refused-unknown-kind", |case| {
            case["id"] = "refused-next-unsupported".into();
        });
    });
}
