//! TL-364: one failing input per reject class of the strict owner readers.
//!
//! Every table row is one concrete input and the exact typed error it must
//! produce. Valid documents are built through the public constructors and
//! mutated textually, so no canonical bytes or digests are copied here. The
//! numeric limit boundaries of NFR-006 live in `limit_boundaries.rs`; this
//! suite covers the structural and encoding classes plus the scanner edge
//! cases of the preflight (escape widths, brackets inside strings).
#![cfg(all(feature = "alloc", feature = "serde"))]

use serde_json::error::Category;
use tl_syntax::{
    FairnessPremisesDocument, FairnessReadError, FormulaDocument, InfiniteClock,
    InfiniteFormulaDocument, InfiniteNode, InfiniteNodeKind, LassoTraceDocument, Node, NodeId,
    NodeKind, OwnedSignalDeclaration, PartialValuation, PartialValue, PropositionBinding,
    PropositionEntry, PropositionId, PropositionMapDocument, RequirementContextDocument,
    SemanticProfile, SignalCatalogDocument, SignalDomain, SignalId, SourceSpan,
    StrictDocumentReadError, SyntaxArtifactLimits, TemporalInterval, TraceObservation,
    UnboundedInterval, ValuationEntry, MAX_REQUIREMENT_CONTEXT_FIELD_BYTES, MAX_TL_DOCUMENT_DEPTH,
};

type ReadOutcome = Result<(), StrictDocumentReadError>;

/// One strict byte reader under test and a canonical document it accepts.
struct Reader {
    name: &'static str,
    valid: Vec<u8>,
    read: fn(&[u8], SyntaxArtifactLimits) -> ReadOutcome,
}

fn infinite_formula() -> InfiniteFormulaDocument {
    InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(1),
        vec![
            InfiniteNode::new(InfiniteNodeKind::Proposition {
                proposition: PropositionId(1),
            }),
            InfiniteNode::new(InfiniteNodeKind::Globally {
                interval: TemporalInterval::Unbounded(UnboundedInterval::new(0)),
                operand: NodeId(0),
            }),
        ],
    )
    .unwrap()
}

fn propositions() -> Vec<PropositionId> {
    vec![PropositionId(1), PropositionId(3)]
}

fn valuation() -> PartialValuation {
    PartialValuation::new(
        "map/v1".into(),
        &propositions(),
        propositions()
            .into_iter()
            .map(|proposition| ValuationEntry {
                proposition,
                value: PartialValue::True,
            })
            .collect(),
    )
    .unwrap()
}

fn lasso() -> LassoTraceDocument {
    LassoTraceDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        "map/v1".into(),
        propositions(),
        vec![],
        vec![TraceObservation {
            position: 0,
            valuation: valuation(),
        }],
    )
    .unwrap()
}

fn fairness() -> FairnessPremisesDocument {
    let formula = infinite_formula();
    FairnessPremisesDocument::new(
        &formula,
        formula.content_identity().unwrap(),
        InfiniteClock::EventPosition,
        vec![NodeId(1)],
    )
    .unwrap()
}

fn read_fairness(bytes: &[u8], limits: SyntaxArtifactLimits) -> ReadOutcome {
    match FairnessPremisesDocument::from_json_bytes(bytes, limits, &infinite_formula()) {
        Ok(_) => Ok(()),
        Err(FairnessReadError::Document(error)) => Err(error),
        Err(FairnessReadError::Binding(error)) => panic!("unexpected binding error: {error:?}"),
    }
}

fn readers() -> Vec<Reader> {
    vec![
        Reader {
            name: "formula/v1",
            valid: FormulaDocument::new(
                SemanticProfile::ClosedTraceV1,
                NodeId(0),
                vec![Node::new(NodeKind::True)],
            )
            .unwrap()
            .canonical_json_bytes()
            .unwrap(),
            read: |bytes, limits| FormulaDocument::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "signal-catalog/v1",
            valid: SignalCatalogDocument::new(
                vec![OwnedSignalDeclaration::new(
                    SignalId(7),
                    "ready".into(),
                    SignalDomain::Boolean,
                )],
                vec![PropositionBinding::new(PropositionId(3), SignalId(7))],
            )
            .unwrap()
            .canonical_json_bytes()
            .unwrap(),
            read: |bytes, limits| SignalCatalogDocument::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "proposition-map/v1",
            valid: PropositionMapDocument::new(vec![PropositionEntry {
                id: PropositionId(3),
                name: "ready".into(),
            }])
            .unwrap()
            .canonical_json_bytes()
            .unwrap(),
            read: |bytes, limits| PropositionMapDocument::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "formula-unbounded/v1",
            valid: infinite_formula().canonical_json_bytes().unwrap(),
            read: |bytes, limits| InfiniteFormulaDocument::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "partial-valuation/v1",
            valid: valuation().canonical_json_bytes().unwrap(),
            read: |bytes, limits| PartialValuation::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "lasso-trace/v1",
            valid: lasso().canonical_json_bytes().unwrap(),
            read: |bytes, limits| LassoTraceDocument::from_json_bytes(bytes, limits).map(drop),
        },
        Reader {
            name: "fairness-premises/v1",
            valid: fairness().canonical_json_bytes().unwrap(),
            read: read_fairness,
        },
    ]
}

/// Which `StrictDocumentReadError` a row must produce.
#[derive(Clone, Copy, Debug)]
enum Expect {
    /// `InvalidDocument` whose serde category and message prefix are exact.
    Invalid(Category, &'static str),
    NonCanonical,
}

fn check(reader: &str, row: &str, outcome: ReadOutcome, expect: Expect) {
    let context = format!("{reader} / {row}");
    match (outcome, expect) {
        (Err(StrictDocumentReadError::NonCanonicalDocument), Expect::NonCanonical) => {}
        (Err(StrictDocumentReadError::InvalidDocument(error)), Expect::Invalid(class, prefix)) => {
            assert_eq!(error.classify(), class, "{context}: {error}");
            assert!(
                error.to_string().starts_with(prefix),
                "{context}: expected message starting {prefix:?}, got {error}"
            );
        }
        (other, expected) => panic!("{context}: expected {expected:?}, got {other:?}"),
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// Replaces the single occurrence of `from`, failing loudly if it is absent.
fn mutate(valid: &[u8], from: &str, to: &str) -> Vec<u8> {
    let source = text(valid);
    assert_eq!(source.matches(from).count(), 1, "{from:?} in {source}");
    source.replacen(from, to, 1).into_bytes()
}

/// Replaces the first occurrence of `from` (the outermost document member).
fn mutate_first(valid: &[u8], from: &str, to: &str) -> Vec<u8> {
    let source = text(valid);
    assert!(source.contains(from), "{from:?} in {source}");
    source.replacen(from, to, 1).into_bytes()
}

/// The `"schema_version":"<id>"` member that every owner document opens with.
fn version_member(valid: &[u8]) -> String {
    let source = text(valid);
    let prefix = "{\"schema_version\":\"";
    assert!(source.starts_with(prefix), "{source}");
    let end = source[prefix.len()..].find('"').unwrap() + prefix.len();
    source[1..=end].to_string()
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn every_reader_refuses_each_structural_and_encoding_class() {
    let limits = SyntaxArtifactLimits::default();
    for reader in readers() {
        let valid = reader.valid.as_slice();
        assert!(
            (reader.read)(valid, limits).is_ok(),
            "{}: canonical baseline must be accepted",
            reader.name
        );
        let member = version_member(valid);
        let with_comma = format!("{member},");
        let unknown_version = format!("{}9\"", &member[..member.len() - 1]);
        let moved_last = {
            let rest = text(valid).replacen(&with_comma, "", 1);
            format!("{},{member}}}", &rest[..rest.len() - 1])
        };
        let rows: Vec<(&str, Vec<u8>, Expect)> = vec![
            (
                "empty input",
                Vec::new(),
                Expect::Invalid(Category::Eof, "EOF while parsing a value"),
            ),
            (
                "truncated document",
                valid[..valid.len() - 1].to_vec(),
                Expect::Invalid(Category::Eof, "EOF while parsing"),
            ),
            (
                "invalid UTF-8 inside a string",
                mutate(valid, &member, "\"schema_version\":\"\u{1}")
                    .into_iter()
                    .flat_map(|byte| if byte == 1 { vec![0xff] } else { vec![byte] })
                    .collect(),
                Expect::Invalid(Category::Syntax, "invalid unicode code point"),
            ),
            (
                "trailing data: stray continuation byte after the document",
                [valid, b"\x80"].concat(),
                Expect::Invalid(Category::Syntax, "trailing characters"),
            ),
            (
                "trailing garbage",
                [valid, b"x"].concat(),
                Expect::Invalid(Category::Syntax, "trailing characters"),
            ),
            (
                "second document appended",
                [valid, valid].concat(),
                Expect::Invalid(Category::Syntax, "trailing characters"),
            ),
            (
                "trailing whitespace is valid JSON but not canonical",
                [valid, b"\n"].concat(),
                Expect::NonCanonical,
            ),
            (
                "inserted whitespace is not canonical",
                mutate_first(valid, "{\"schema_version\"", "{ \"schema_version\""),
                Expect::NonCanonical,
            ),
            (
                "escaped member name decodes but is not canonical",
                mutate_first(valid, "\"schema_version\"", "\"\\u0073chema_version\""),
                Expect::NonCanonical,
            ),
            (
                "members out of canonical order",
                moved_last.into_bytes(),
                Expect::NonCanonical,
            ),
            (
                "duplicate member",
                mutate(valid, &with_comma, &format!("{with_comma}{with_comma}")),
                Expect::Invalid(Category::Data, "duplicate field `schema_version`"),
            ),
            (
                "unknown member",
                mutate_first(
                    valid,
                    "{\"schema_version\"",
                    "{\"unknown\":0,\"schema_version\"",
                ),
                Expect::Invalid(Category::Data, "unknown field `unknown`"),
            ),
            (
                "missing member",
                mutate(valid, &with_comma, ""),
                Expect::Invalid(Category::Data, "missing field `schema_version`"),
            ),
            (
                "unknown schema version",
                mutate(
                    valid,
                    &member,
                    &format!("\"schema_version\":{unknown_version}"),
                ),
                Expect::Invalid(Category::Data, "unknown variant"),
            ),
            (
                // serde_json reports a non-string enum tag as a syntax-level "expected value".
                "schema version of the wrong JSON type",
                mutate(valid, &member, "\"schema_version\":7"),
                Expect::Invalid(Category::Syntax, "expected value"),
            ),
            (
                "document root of the wrong JSON type",
                b"7".to_vec(),
                Expect::Invalid(Category::Data, "invalid type: integer `7`"),
            ),
            (
                "null document",
                b"null".to_vec(),
                Expect::Invalid(Category::Data, "invalid type: null"),
            ),
        ];
        for (row, bytes, expect) in rows {
            check(reader.name, row, (reader.read)(&bytes, limits), expect);
        }
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn preflight_scanner_edge_cases_are_exact_for_every_reader() {
    // Each row is a JSON string literal, the lowered `string_bytes` limit, and
    // the decoded size the preflight must report for the first rejected byte.
    let escape_rows: [(&str, usize, usize); 8] = [
        (r#""\u0041""#, 0, 1),
        (r#""\u00e9""#, 1, 2),
        (r#""\u20ac""#, 2, 3),
        (r#""\ud83d\ude00""#, 3, 4),
        (r#""\n""#, 0, 1),
        // A high surrogate without its pair is charged as a three-byte scalar.
        (r#""\ud83dxx""#, 2, 3),
        // Malformed hex digits charge nothing for the escape, then one byte each.
        (r#""\uZZZZ""#, 3, 4),
        (r#""abc""#, 2, 3),
    ];
    for reader in readers() {
        for (literal, string_bytes, actual) in escape_rows {
            let limits = SyntaxArtifactLimits {
                string_bytes,
                ..SyntaxArtifactLimits::default()
            };
            let bytes = format!("{{\"schema_version\":{literal}}}");
            match (reader.read)(bytes.as_bytes(), limits) {
                Err(StrictDocumentReadError::StringTooLarge { actual: a, limit }) => {
                    assert_eq!(
                        (a, limit),
                        (actual, string_bytes),
                        "{} / {literal}",
                        reader.name
                    );
                }
                other => panic!("{} / {literal}: got {other:?}", reader.name),
            }
        }

        // Brackets inside a string are not nesting, however many there are.
        let brackets = format!("{{\"schema_version\":\"{}\"}}", "[".repeat(200));
        check(
            reader.name,
            "brackets inside a string",
            (reader.read)(brackets.as_bytes(), SyntaxArtifactLimits::default()),
            Expect::Invalid(Category::Data, "unknown variant"),
        );

        // An escaped quote does not end the string, so the bracket after it
        // stays inside the string and the document stays within depth 1.
        let escaped_quote = "{\"schema_version\":\"\\\"[[[[\"}";
        let one = SyntaxArtifactLimits {
            json_depth: 1,
            ..SyntaxArtifactLimits::default()
        };
        check(
            reader.name,
            "escaped quote keeps brackets inside the string",
            (reader.read)(escaped_quote.as_bytes(), one),
            Expect::Invalid(Category::Data, "unknown variant"),
        );

        // Closers without openers saturate at depth zero instead of underflowing.
        let closers = format!("{}{{}}", "]}".repeat(8));
        check(
            reader.name,
            "unbalanced closers",
            (reader.read)(closers.as_bytes(), one),
            Expect::Invalid(Category::Syntax, "expected value"),
        );

        // Nesting past the owner ceiling is refused before any parsing.
        let nesting = MAX_TL_DOCUMENT_DEPTH + 1;
        let deep = format!("{}{}", "[".repeat(nesting), "]".repeat(nesting));
        match (reader.read)(deep.as_bytes(), SyntaxArtifactLimits::default()) {
            Err(StrictDocumentReadError::DepthLimitExceeded { actual, limit }) => {
                assert_eq!(
                    (actual, limit),
                    (nesting, MAX_TL_DOCUMENT_DEPTH),
                    "{}",
                    reader.name
                );
            }
            other => panic!("{} / over-depth: {other:?}", reader.name),
        }

        // A string left open at the end of input never reaches a size error
        // below its limit and is then refused by the JSON parser.
        check(
            reader.name,
            "unterminated string",
            (reader.read)(
                b"{\"schema_version\":\"abc",
                SyntaxArtifactLimits::default(),
            ),
            Expect::Invalid(Category::Eof, "EOF while parsing a string"),
        );
        check(
            reader.name,
            "dangling backslash at end of input",
            (reader.read)(
                b"{\"schema_version\":\"abc\\",
                SyntaxArtifactLimits::default(),
            ),
            Expect::Invalid(Category::Eof, "EOF while parsing a string"),
        );
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn byte_and_work_budgets_refuse_with_exact_charges_for_every_reader() {
    for reader in readers() {
        let length = reader.valid.len();
        let tight = SyntaxArtifactLimits {
            document_bytes: length - 1,
            ..SyntaxArtifactLimits::default()
        };
        match (reader.read)(&reader.valid, tight) {
            Err(StrictDocumentReadError::DocumentTooLarge { actual, limit }) => {
                assert_eq!((actual, limit), (length, length - 1), "{}", reader.name);
            }
            other => panic!("{} / over-bytes: {other:?}", reader.name),
        }
        // The fixed admission charge is three work units per input byte (no exported constant).
        let starved = SyntaxArtifactLimits {
            work: length * 3 - 1,
            ..SyntaxArtifactLimits::default()
        };
        match (reader.read)(&reader.valid, starved) {
            Err(StrictDocumentReadError::WorkLimitExceeded { actual, limit }) => {
                assert_eq!(
                    (actual, limit),
                    (length * 3, length * 3 - 1),
                    "{}",
                    reader.name
                );
            }
            other => panic!("{} / over-work: {other:?}", reader.name),
        }
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn array_population_scanner_charges_only_real_array_items() {
    let limits = SyntaxArtifactLimits {
        propositions: 1,
        ..SyntaxArtifactLimits::default()
    };
    let map = |entries: &str| {
        format!(
            "{{\"schema_version\":\"tl-syntax.proposition-map/v1\",\"propositions\":{entries}}}"
        )
    };
    let read =
        |entries: &str| PropositionMapDocument::from_json_bytes(map(entries).as_bytes(), limits);
    // Nested objects, strings containing commas/brackets, and whitespace around
    // the member separator are each counted as exactly one item.
    for (row, entries, population) in [
        (
            "two objects",
            r#"[{"id":1,"name":"a"},{"id":2,"name":"b"}]"#,
            2,
        ),
        ("two strings with punctuation", r#"["a,]","b[,"]"#, 2),
        ("spaced separator", r#"  [ 1 , 2 ]"#, 2),
        ("nested arrays", r#"[[1,2],[3]]"#, 2),
        ("three scalars", "[1,2,3]", 3),
    ] {
        match read(entries) {
            Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource,
                actual,
                limit,
            }) => assert_eq!(
                (resource, actual, limit),
                ("propositions", population, 1),
                "{row}"
            ),
            other => panic!("{row}: {other:?}"),
        }
    }
    // An empty array and a single item stay within the limit and reach the parser.
    for entries in ["[]", r#"[{"id":1,"name":"a"}]"#] {
        assert!(
            !matches!(
                read(entries),
                Err(StrictDocumentReadError::ResourceLimitExceeded { .. })
            ),
            "{entries}"
        );
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn trace_documents_refuse_each_semantic_reject_class_with_its_typed_error() {
    // The admission errors (`PartialValuationError`, `LassoTraceError`) are wrapped into
    // `serde_json::Error` by the `try_from` wire types, so their Display text (which embeds
    // the variant and its fields) is the only observable form through the strict reader.
    let limits = SyntaxArtifactLimits::default();
    let valuation_bytes = valuation().canonical_json_bytes().unwrap();
    let lasso_bytes = lasso().canonical_json_bytes().unwrap();
    let lasso_text = text(&lasso_bytes);
    let loop_tail = lasso_text[lasso_text.find("\"loop_observations\":").unwrap()..]
        .trim_end_matches('}')
        .to_string();
    let message = |expected: &'static str| Expect::Invalid(Category::Data, expected);

    let valuation_rows: Vec<(&str, Vec<u8>, Expect)> = vec![
        (
            "empty map identity",
            mutate(&valuation_bytes, "\"map/v1\"", "\"\""),
            message("partial valuation admission: MissingMapIdentity"),
        ),
        (
            "declared propositions out of order",
            mutate(&valuation_bytes, "\"propositions\":[1,3]", "\"propositions\":[3,1]"),
            message("partial valuation admission: MapOrder"),
        ),
        (
            "entry for an undeclared proposition",
            mutate(
                &valuation_bytes,
                "{\"proposition\":3,\"value\":\"true\"}",
                "{\"proposition\":9,\"value\":\"true\"}",
            ),
            message("partial valuation admission: ForeignProposition { proposition: PropositionId(9) }"),
        ),
        (
            "duplicate entry",
            mutate(
                &valuation_bytes,
                "{\"proposition\":3,\"value\":\"true\"}",
                "{\"proposition\":1,\"value\":\"true\"}",
            ),
            message("partial valuation admission: DuplicateProposition { proposition: PropositionId(1) }"),
        ),
        (
            "entries out of order",
            mutate(
                &valuation_bytes,
                "[{\"proposition\":1,\"value\":\"true\"},{\"proposition\":3,\"value\":\"true\"}]",
                "[{\"proposition\":3,\"value\":\"true\"},{\"proposition\":1,\"value\":\"true\"}]",
            ),
            message("partial valuation admission: UnorderedProposition { proposition: PropositionId(1) }"),
        ),
        (
            "declared proposition without an entry",
            mutate(
                &valuation_bytes,
                ",{\"proposition\":3,\"value\":\"true\"}",
                "",
            ),
            message("partial valuation admission: OmittedProposition { proposition: PropositionId(3) }"),
        ),
        (
            "unknown evidence value",
            mutate(&valuation_bytes, "\"value\":\"true\"},{", "\"value\":\"maybe\"},{"),
            message("unknown variant `maybe`"),
        ),
        (
            "negative proposition identity",
            mutate(&valuation_bytes, "\"propositions\":[1,3]", "\"propositions\":[-1,3]"),
            message("invalid value: integer `-1`"),
        ),
    ];
    for (row, bytes, expect) in valuation_rows {
        check(
            "partial-valuation/v1",
            row,
            PartialValuation::from_json_bytes(&bytes, limits).map(drop),
            expect,
        );
    }

    let lasso_rows: Vec<(&str, Vec<u8>, Expect)> = vec![
        (
            "empty loop",
            mutate(&lasso_bytes, &loop_tail, "\"loop_observations\":[]"),
            message("lasso trace admission: EmptyLoop"),
        ),
        (
            "observation at the wrong position",
            mutate(&lasso_bytes, "\"position\":0", "\"position\":4"),
            message("lasso trace admission: Position { expected: 0, actual: 4 }"),
        ),
        (
            "valuation from another proposition map",
            mutate(
                &lasso_bytes,
                "\"proposition_map_identity\":\"map/v1\",\"propositions\":[1,3],\"entries\"",
                "\"proposition_map_identity\":\"map/v2\",\"propositions\":[1,3],\"entries\"",
            ),
            message("lasso trace admission: MapIdentityMismatch"),
        ),
        (
            "profile other than infinite trace",
            mutate(
                &lasso_bytes,
                "mltl.infinite-trace/v1",
                "mltl.closed-trace/v1",
            ),
            message("lasso trace admission: Profile { actual: ClosedTraceV1 }"),
        ),
        (
            "unknown profile identity",
            mutate(&lasso_bytes, "mltl.infinite-trace/v1", "mltl.invented/v1"),
            message("unknown variant `mltl.invented/v1`"),
        ),
        (
            "unknown clock",
            mutate(&lasso_bytes, "event_position", "wall_clock"),
            message("unknown variant `wall_clock`"),
        ),
    ];
    for (row, bytes, expect) in lasso_rows {
        check(
            "lasso-trace/v1",
            row,
            LassoTraceDocument::from_json_bytes(&bytes, limits).map(drop),
            expect,
        );
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn fairness_reader_refuses_each_binding_reject_class_with_its_typed_error() {
    // Root order/duplicate and graph-identity errors are raised in the wire `try_from` and so
    // reach the caller wrapped in serde; only formula-bound errors surface as `Binding(..)`.
    let limits = SyntaxArtifactLimits::default();
    let formula = infinite_formula();
    let valid = fairness().canonical_json_bytes().unwrap();
    let identity = formula.content_identity().unwrap();
    type Matcher = fn(&FairnessReadError) -> bool;
    let rows: Vec<(&str, Vec<u8>, Matcher)> = vec![
        (
            "root absent from the graph",
            mutate(&valid, "\"roots\":[1]", "\"roots\":[9]"),
            |e| matches!(e, FairnessReadError::Binding(tl_syntax::FairnessPremisesError::ForeignRoot { root }) if *root == NodeId(9)),
        ),
        (
            "repeated root",
            mutate(&valid, "\"roots\":[1]", "\"roots\":[1,1]"),
            |e| matches!(e, FairnessReadError::Document(StrictDocumentReadError::InvalidDocument(error)) if error.to_string().starts_with("fairness premises admission: DuplicateRoot { root: NodeId(1) }")),
        ),
        (
            "roots out of order",
            mutate(&valid, "\"roots\":[1]", "\"roots\":[1,0]"),
            |e| matches!(e, FairnessReadError::Document(StrictDocumentReadError::InvalidDocument(error)) if error.to_string().starts_with("fairness premises admission: UnorderedRoot { root: NodeId(0) }")),
        ),
        ("empty graph identity", mutate(&valid, &identity, ""), |e| {
            matches!(
                e,
                FairnessReadError::Document(StrictDocumentReadError::InvalidDocument(error))
                    if error.to_string().starts_with("fairness premises admission: MissingGraphIdentity")
            )
        }),
        (
            "graph identity of another document",
            mutate(&valid, &identity, &"0".repeat(identity.len())),
            |e| {
                matches!(
                    e,
                    FairnessReadError::Binding(
                        tl_syntax::FairnessPremisesError::GraphIdentityMismatch
                    )
                )
            },
        ),
        (
            "unknown profile identity",
            mutate(&valid, "mltl.infinite-trace/v1", "mltl.invented/v1"),
            |e| matches!(e, FairnessReadError::Document(StrictDocumentReadError::InvalidDocument(error)) if error.to_string().starts_with("unknown variant `mltl.invented/v1`")),
        ),
        (
            "non-canonical whitespace",
            mutate(&valid, "\"roots\":[1]", "\"roots\": [1]"),
            |e| {
                matches!(
                    e,
                    FairnessReadError::Document(StrictDocumentReadError::NonCanonicalDocument)
                )
            },
        ),
    ];
    for (row, bytes, matches) in rows {
        let outcome = FairnessPremisesDocument::from_json_bytes(&bytes, limits, &formula);
        match outcome {
            Err(error) if matches(&error) => {}
            other => panic!("{row}: {other:?}"),
        }
    }
}

fn context_json(fields: [&str; 4], span: &str, version: &str) -> Vec<u8> {
    format!(
        "{{\"schema_version\":{version},\"requirement_id\":{},\"requirement_revision\":{},\"clause_id\":{},\"anchor\":{},\"source_span\":{span}}}",
        fields[0], fields[1], fields[2], fields[3]
    )
    .into_bytes()
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn requirement_context_document_refuses_each_reject_class() {
    let version = "\"tl-syntax.requirement-context/v1\"";
    let ok = ["\"R\"", "\"1\"", "\"c\"", "\"a\""];
    let span = "{\"start\":0,\"end\":2}";
    let baseline = context_json(ok, span, version);
    assert!(serde_json::from_slice::<RequirementContextDocument>(&baseline).is_ok());

    let over = format!(
        "\"{}\"",
        "x".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1)
    );
    let over_escaped = format!(
        "\"{}\"",
        "\\u0078".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1)
    );
    let over_wide = format!(
        "\"{}\"",
        "\u{e9}".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES / 2 + 1)
    );
    let too_long = |decoded: usize| {
        format!(
            "caller context field has {decoded} UTF-8 bytes, exceeding the {MAX_REQUIREMENT_CONTEXT_FIELD_BYTES}-byte wire limit"
        )
    };
    let field_len = MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1;
    let wide_len = (MAX_REQUIREMENT_CONTEXT_FIELD_BYTES / 2 + 1) * 2;

    let rows: Vec<(&str, Vec<u8>, Category, String)> = vec![
        (
            "empty requirement id",
            context_json(["\"\"", ok[1], ok[2], ok[3]], span, version),
            Category::Data,
            "caller context requirement_id is empty".into(),
        ),
        (
            "empty revision",
            context_json([ok[0], "\"\"", ok[2], ok[3]], span, version),
            Category::Data,
            "caller context requirement_revision is empty".into(),
        ),
        (
            "empty clause id",
            context_json([ok[0], ok[1], "\"\"", ok[3]], span, version),
            Category::Data,
            "caller context clause_id is empty".into(),
        ),
        (
            "empty anchor",
            context_json([ok[0], ok[1], ok[2], "\"\""], span, version),
            Category::Data,
            "caller context anchor is empty".into(),
        ),
        (
            "field one byte over the wire limit",
            context_json([&over, ok[1], ok[2], ok[3]], span, version),
            Category::Data,
            too_long(field_len),
        ),
        (
            "escaped field one byte over the wire limit",
            context_json([ok[0], ok[1], ok[2], &over_escaped], span, version),
            Category::Data,
            too_long(field_len),
        ),
        (
            "multi-byte field over the wire limit",
            context_json([ok[0], &over_wide, ok[2], ok[3]], span, version),
            Category::Data,
            too_long(wide_len),
        ),
        (
            "field of the wrong JSON type",
            context_json([ok[0], "7", ok[2], ok[3]], span, version),
            Category::Data,
            "invalid type: integer `7`, expected a caller context field no longer than 1024 UTF-8 bytes".into(),
        ),
        (
            "null field",
            context_json([ok[0], ok[1], "null", ok[3]], span, version),
            Category::Data,
            "invalid type: null, expected a caller context field no longer than 1024 UTF-8 bytes".into(),
        ),
        (
            "inverted source span",
            context_json(ok, "{\"start\":5,\"end\":2}", version),
            Category::Data,
            "source span start 5 exceeds end 2".into(),
        ),
        (
            "unknown schema version",
            context_json(ok, span, "\"tl-syntax.requirement-context/v2\""),
            Category::Data,
            "unknown variant `tl-syntax.requirement-context/v2`".into(),
        ),
        (
            "unknown member",
            mutate(&baseline, "{\"schema_version\"", "{\"extra\":1,\"schema_version\""),
            Category::Data,
            "unknown field `extra`".into(),
        ),
        (
            "duplicate member",
            mutate(&baseline, "\"anchor\":\"a\",", "\"anchor\":\"a\",\"anchor\":\"b\","),
            Category::Data,
            "duplicate field `anchor`".into(),
        ),
        (
            "missing member",
            mutate(&baseline, "\"clause_id\":\"c\",", ""),
            Category::Data,
            "missing field `clause_id`".into(),
        ),
        (
            "trailing data",
            [baseline.as_slice(), b"{}"].concat(),
            Category::Syntax,
            "trailing characters".into(),
        ),
        (
            "invalid UTF-8 inside a field",
            mutate(&baseline, "\"R\"", "\"\u{1}\"")
                .into_iter()
                .flat_map(|byte| if byte == 1 { vec![0xff] } else { vec![byte] })
                .collect(),
            Category::Syntax,
            "invalid unicode code point".into(),
        ),
        (
            "truncated document",
            baseline[..baseline.len() - 1].to_vec(),
            Category::Eof,
            "EOF while parsing".into(),
        ),
    ];
    for (row, bytes, class, prefix) in rows {
        match serde_json::from_slice::<RequirementContextDocument>(&bytes) {
            Err(error) => {
                assert_eq!(error.classify(), class, "{row}: {error}");
                assert!(
                    error.to_string().starts_with(&prefix),
                    "{row}: expected {prefix:?}, got {error}"
                );
            }
            Ok(document) => panic!("{row}: accepted {document:?}"),
        }
    }

    // The span type itself refuses an inverted range by value, independent of the wire.
    let inverted = SourceSpan::new(5, 2).unwrap_err();
    assert_eq!((inverted.start, inverted.end), (5, 2));
    assert_eq!(inverted.to_string(), "source span start 5 exceeds end 2");
}

type PopulationRow = (&'static str, ReadOutcome, (&'static str, usize, usize));

// Trace: TC-075, FR-014-AC-2
#[test]
fn trace_readers_refuse_populations_over_lowered_limits_with_exact_resources() {
    let rows: Vec<PopulationRow> = {
        let one_proposition = SyntaxArtifactLimits {
            propositions: 1,
            ..SyntaxArtifactLimits::default()
        };
        let one_node = SyntaxArtifactLimits {
            formula_nodes: 1,
            ..SyntaxArtifactLimits::default()
        };
        let two_positions = LassoTraceDocument::new(
            SemanticProfile::InfiniteTraceV1,
            InfiniteClock::EventPosition,
            "map/v1".into(),
            vec![PropositionId(1)],
            vec![TraceObservation {
                position: 0,
                valuation: PartialValuation::new(
                    "map/v1".into(),
                    &[PropositionId(1)],
                    vec![ValuationEntry {
                        proposition: PropositionId(1),
                        value: PartialValue::True,
                    }],
                )
                .unwrap(),
            }],
            vec![TraceObservation {
                position: 1,
                valuation: PartialValuation::new(
                    "map/v1".into(),
                    &[PropositionId(1)],
                    vec![ValuationEntry {
                        proposition: PropositionId(1),
                        value: PartialValue::True,
                    }],
                )
                .unwrap(),
            }],
        )
        .unwrap()
        .canonical_json_bytes()
        .unwrap();
        let two_roots = {
            let formula = infinite_formula();
            FairnessPremisesDocument::new(
                &formula,
                formula.content_identity().unwrap(),
                InfiniteClock::EventPosition,
                vec![NodeId(0), NodeId(1)],
            )
            .unwrap()
            .canonical_json_bytes()
            .unwrap()
        };
        vec![
            (
                "valuation entries",
                PartialValuation::from_json_bytes(
                    &valuation().canonical_json_bytes().unwrap(),
                    one_proposition,
                )
                .map(drop),
                ("valuation entries", 2, 1),
            ),
            (
                "trace propositions",
                LassoTraceDocument::from_json_bytes(
                    &lasso().canonical_json_bytes().unwrap(),
                    one_proposition,
                )
                .map(drop),
                ("trace propositions", 2, 1),
            ),
            (
                "trace positions",
                LassoTraceDocument::from_json_bytes(&two_positions, one_node).map(drop),
                ("trace positions", 2, 1),
            ),
            (
                "fairness roots",
                read_fairness(&two_roots, one_node),
                ("fairness roots", 2, 1),
            ),
        ]
    };
    for (row, outcome, (name, actual, limit)) in rows {
        match outcome {
            Err(StrictDocumentReadError::ResourceLimitExceeded {
                resource,
                actual: a,
                limit: l,
            }) => assert_eq!((resource, a, l), (name, actual, limit), "{row}"),
            other => panic!("{row}: {other:?}"),
        }
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn read_errors_render_their_exact_diagnostics() {
    let limits = SyntaxArtifactLimits::default();
    let render = |error: StrictDocumentReadError| error.to_string();
    assert_eq!(
        render(StrictDocumentReadError::DocumentTooLarge {
            actual: 9,
            limit: 8
        }),
        "document has 9 bytes; limit is 8"
    );
    assert_eq!(
        render(StrictDocumentReadError::DepthLimitExceeded {
            actual: 65,
            limit: 64
        }),
        "document nesting depth is 65; limit is 64"
    );
    assert_eq!(
        render(StrictDocumentReadError::StringTooLarge {
            actual: 3,
            limit: 2
        }),
        "JSON string has 3 bytes; limit is 2"
    );
    assert_eq!(
        render(StrictDocumentReadError::WorkLimitExceeded {
            actual: 7,
            limit: 6
        }),
        "document admission costs 7 work units; limit is 6"
    );
    assert_eq!(
        render(StrictDocumentReadError::ResourceLimitExceeded {
            resource: "fairness roots",
            actual: 2,
            limit: 1
        }),
        "fairness roots population/depth is 2; limit is 1"
    );
    assert_eq!(
        render(StrictDocumentReadError::NonCanonicalDocument),
        "document is not canonical owner JSON"
    );
    let invalid = FormulaDocument::from_json_bytes(b"7", limits).unwrap_err();
    assert!(render(invalid).starts_with("invalid document: invalid type: integer `7`"));

    let formula = infinite_formula();
    let document = FairnessPremisesDocument::from_json_bytes(b"7", limits, &formula).unwrap_err();
    assert!(matches!(document, FairnessReadError::Document(_)));
    assert!(document
        .to_string()
        .starts_with("invalid document: invalid type: integer `7`"));
    let valid = fairness().canonical_json_bytes().unwrap();
    let binding = FairnessPremisesDocument::from_json_bytes(
        &mutate(&valid, "\"roots\":[1]", "\"roots\":[9]"),
        limits,
        &formula,
    )
    .unwrap_err();
    assert_eq!(
        binding.to_string(),
        "fairness premises admission: ForeignRoot { root: NodeId(9) }"
    );
}
