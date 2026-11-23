//! P-TL-150 numeric predicates (TL-420): FR-302..FR-305, NFR-018, NFR-019.
#![cfg(feature = "serde")]

use std::{collections::BTreeMap, fs, path::PathBuf};

use proptest::prelude::*;
use serde_json::Value;
use tl_syntax::{
    BoundPredicateFormula, ComparisonOperator, ExprId, ExprNode, ExprSlot, ExprType,
    ExpressionArena, ExpressionError, ExpressionLimit, FixedDecimalSignalDomain, Formula,
    FormulaBindingError, IntegerSignalDomain, Node, NodeId, NodeKind, Predicate,
    PredicateBindingError, PredicateTable, PredicateTableDocument, PredicateTableError,
    PredicateTableReadError, PropositionBinding, PropositionId, PropositionSource,
    SemanticProfile, SignalCatalog, SignalCatalogDocument, SignalDeclaration, SignalDomain,
    SignalId, StrictDocumentReadError, SyntaxArtifactLimits, CORPUS_DIR, MAX_EXPRESSION_DEPTH,
    MAX_EXPRESSION_NODES, MAX_PREDICATES, PREDICATE_TABLE_V1, PREDICATE_TABLE_V1_SCHEMA,
    PREDICATE_TABLE_V1_SCHEMA_BYTES,
};

fn numeric_dir() -> PathBuf {
    PathBuf::from(CORPUS_DIR).join("numeric")
}

fn manifest() -> Value {
    serde_json::from_slice(&fs::read(numeric_dir().join("manifest.json")).unwrap()).unwrap()
}

fn corpus_catalog() -> SignalCatalogDocument {
    serde_json::from_slice(&fs::read(numeric_dir().join("catalogs/catalog-1.json")).unwrap())
        .unwrap()
}

fn integer_domain(minimum: i64, maximum: i64) -> SignalDomain {
    SignalDomain::Integer(IntegerSignalDomain::new(minimum, maximum).unwrap())
}

fn decimal_domain(minimum: i64, maximum: i64, scale: u8) -> SignalDomain {
    SignalDomain::FixedDecimal(FixedDecimalSignalDomain::new(minimum, maximum, scale).unwrap())
}

/// Signals: 1 Boolean `flag` (bound to proposition 1), 2 Integer [-5,7],
/// 3 Decimal [-400,1250] scale 1, 4 Integer [i64::MIN, i64::MAX].
fn signals() -> [SignalDeclaration<'static>; 4] {
    [
        SignalDeclaration::new(SignalId(1), "flag", SignalDomain::Boolean),
        SignalDeclaration::new(SignalId(2), "count", integer_domain(-5, 7)),
        SignalDeclaration::new(SignalId(3), "temp", decimal_domain(-400, 1250, 1)),
        SignalDeclaration::new(SignalId(4), "wide", integer_domain(i64::MIN, i64::MAX)),
    ]
}

const BINDINGS: [PropositionBinding; 1] = [PropositionBinding::new(PropositionId(1), SignalId(1))];

fn with_catalog<R>(run: impl FnOnce(SignalCatalog<'_>) -> R) -> R {
    let signals = signals();
    let mut scratch = [0; 4];
    let catalog = SignalCatalog::new(&signals, &BINDINGS, &mut scratch).unwrap();
    run(catalog)
}

fn types(catalog: SignalCatalog<'_>, nodes: &[ExprNode]) -> Result<Vec<ExprType>, ExpressionError> {
    let mut scratch = vec![ExprSlot::EMPTY; nodes.len()];
    ExpressionArena::new(catalog, nodes, &mut scratch)?;
    Ok(scratch.iter().map(|slot| slot.ty()).collect())
}

fn ty(scale: u8, minimum: i64, maximum: i64) -> ExprType {
    ExprType {
        scale,
        minimum,
        maximum,
    }
}

fn signal(id: u32) -> ExprNode {
    ExprNode::Signal {
        signal: SignalId(id),
    }
}

/// TC-1500: corpus types derived by the independent generator equal the crate.
// Trace: TC-1500, FR-302-AC-1
#[test]
fn derived_types_equal_independent_corpus_types() {
    let catalog_document = corpus_catalog();
    let catalog = catalog_document.validate().unwrap();
    let manifest = manifest();
    let mut checked = 0;
    for case in manifest["cases"].as_array().unwrap() {
        if case["class"] != "valid" {
            continue;
        }
        let bytes = fs::read(numeric_dir().join(case["table"].as_str().unwrap())).unwrap();
        let table =
            PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
                .unwrap();
        let derived = table.expression_types(catalog).unwrap();
        let expected: Vec<ExprType> = case["expected_types"]
            .as_array()
            .unwrap()
            .iter()
            .map(|triple| {
                ty(
                    triple[0].as_u64().unwrap() as u8,
                    triple[1].as_i64().unwrap(),
                    triple[2].as_i64().unwrap(),
                )
            })
            .collect();
        assert_eq!(derived, expected, "{}", case["id"]);
        checked += 1;
    }
    assert!(checked >= 2);
}

/// TC-1501: abs over zero-straddling and negative ranges; mul hulls with
/// negative corners.
// Trace: TC-1501, FR-302-AC-1
#[test]
fn abs_and_mul_follow_the_interval_rules() {
    with_catalog(|catalog| {
        let nodes = [
            signal(2),
            ExprNode::Abs {
                operand: ExprId(0),
            },
            ExprNode::Integer { value: -9 },
            ExprNode::Integer { value: -2 },
            ExprNode::Add {
                left: ExprId(2),
                right: ExprId(0),
            },
            ExprNode::Abs {
                operand: ExprId(4),
            },
            signal(3),
            ExprNode::Mul {
                left: ExprId(0),
                right: ExprId(6),
            },
            ExprNode::Neg {
                operand: ExprId(0),
            },
            ExprNode::Rescale {
                operand: ExprId(0),
                scale: 3,
            },
            ExprNode::Sub {
                left: ExprId(0),
                right: ExprId(2),
            },
        ];
        let derived = types(catalog, &nodes).unwrap();
        assert_eq!(derived[1], ty(0, 0, 7)); // straddles zero
        assert_eq!(derived[4], ty(0, -14, -2));
        assert_eq!(derived[5], ty(0, 2, 14)); // wholly negative
        // [-5,7] x [-400,1250]: corners 2000, -6250, -2800, 8750.
        assert_eq!(derived[7], ty(1, -6250, 8750));
        assert_eq!(derived[8], ty(0, -7, 5));
        assert_eq!(derived[9], ty(3, -5000, 7000));
        assert_eq!(derived[10], ty(0, 4, 16));
    });
}

/// TC-1502: each expression error from a minimal input, first in node order,
/// with no view.
// Trace: TC-1502, FR-302-AC-2
#[test]
fn each_expression_error_has_a_minimal_input() {
    with_catalog(|catalog| {
        let wide = ExprNode::Mul {
            left: ExprId(0),
            right: ExprId(0),
        };
        let cases: Vec<(Vec<ExprNode>, ExpressionError)> = vec![
            (
                vec![signal(4), wide, ExprNode::Integer { value: 1 }],
                ExpressionError::RangeOverflow { node: ExprId(1) },
            ),
            (
                vec![
                    signal(2),
                    signal(3),
                    ExprNode::Add {
                        left: ExprId(0),
                        right: ExprId(1),
                    },
                ],
                ExpressionError::ScaleMismatch {
                    node: ExprId(2),
                    left: 0,
                    right: 1,
                },
            ),
            (
                vec![
                    ExprNode::Decimal {
                        coefficient: 1,
                        scale: 18,
                    },
                    ExprNode::Decimal {
                        coefficient: 1,
                        scale: 1,
                    },
                    ExprNode::Mul {
                        left: ExprId(0),
                        right: ExprId(1),
                    },
                ],
                ExpressionError::ScaleOverflow {
                    node: ExprId(2),
                    scale: 19,
                },
            ),
            (
                vec![
                    signal(3),
                    ExprNode::Rescale {
                        operand: ExprId(0),
                        scale: 0,
                    },
                ],
                ExpressionError::RescaleNarrowing {
                    node: ExprId(1),
                    from: 1,
                    to: 0,
                },
            ),
            (
                vec![signal(1)],
                ExpressionError::NonNumericSignal {
                    node: ExprId(0),
                    signal: SignalId(1),
                },
            ),
            (
                vec![signal(99)],
                ExpressionError::UnknownSignal {
                    node: ExprId(0),
                    signal: SignalId(99),
                },
            ),
            (
                {
                    let mut chain = vec![ExprNode::Integer { value: 0 }];
                    for index in 0..MAX_EXPRESSION_DEPTH as u32 {
                        chain.push(ExprNode::Neg {
                            operand: ExprId(index),
                        });
                    }
                    chain
                },
                ExpressionError::ExpressionLimitExceeded {
                    resource: ExpressionLimit::Depth,
                    actual: MAX_EXPRESSION_DEPTH + 1,
                    limit: MAX_EXPRESSION_DEPTH,
                },
            ),
            (
                vec![ExprNode::Neg {
                    operand: ExprId(0),
                }],
                ExpressionError::OperandNotPreceding {
                    node: ExprId(0),
                    operand: ExprId(0),
                },
            ),
            (
                vec![ExprNode::SlotSignal { depth: 0 }],
                ExpressionError::SlotOutsideTemplate { node: ExprId(0) },
            ),
        ];
        for (nodes, expected) in cases {
            assert_eq!(types(catalog, &nodes), Err(expected));
        }
        // Node order: the earlier of two faults is reported.
        let both = [signal(99), signal(1)];
        assert_eq!(
            types(catalog, &both),
            Err(ExpressionError::UnknownSignal {
                node: ExprId(0),
                signal: SignalId(99),
            })
        );
        let mut short = [ExprSlot::EMPTY; 1];
        assert_eq!(
            ExpressionArena::new(catalog, &both, &mut short),
            Err(ExpressionError::ScratchTooSmall {
                provided: 1,
                required: 2,
            })
        );
    });
}

/// Test-only exact reference: the coefficient of `node` at its own scale.
fn reference(nodes: &[ExprNode], types: &[ExprType], index: usize, inputs: &BTreeMap<u32, i64>) -> i128 {
    let operand = |id: ExprId| reference(nodes, types, id.0 as usize, inputs);
    match nodes[index] {
        ExprNode::Signal { signal } => i128::from(inputs[&signal.0]),
        ExprNode::Integer { value } => i128::from(value),
        ExprNode::Decimal { coefficient, .. } => i128::from(coefficient),
        ExprNode::Neg { operand: id } => -operand(id),
        ExprNode::Abs { operand: id } => operand(id).abs(),
        ExprNode::Add { left, right } => operand(left) + operand(right),
        ExprNode::Sub { left, right } => operand(left) - operand(right),
        ExprNode::Mul { left, right } => operand(left) * operand(right),
        ExprNode::Rescale { operand: id, scale } => {
            operand(id) * 10_i128.pow(u32::from(scale - types[id.0 as usize].scale))
        }
        _ => unreachable!("templates are not generated here"),
    }
}

fn arbitrary_nodes() -> impl Strategy<Value = Vec<(u8, u32, u32, i64)>> {
    proptest::collection::vec((0_u8..9, any::<u32>(), any::<u32>(), -50_i64..50), 1..12)
}

proptest! {
    /// TC-1503: corner evaluation by a test-only reference never overflows
    /// and never leaves the derived range.
    // Trace: TC-1503, FR-302-AC-3
    #[test]
    fn corner_evaluation_stays_within_the_derived_range(
        raw in arbitrary_nodes(),
        bounds in (-30_i64..30, 0_i64..30, -30_i64..30, 0_i64..30),
    ) {
        let first = integer_domain(bounds.0, bounds.0 + bounds.1);
        let second = decimal_domain(bounds.2, bounds.2 + bounds.3, 2);
        let declared = [
            SignalDeclaration::new(SignalId(10), "a", first),
            SignalDeclaration::new(SignalId(11), "b", second),
        ];
        let mut scratch = [0; 2];
        let catalog = SignalCatalog::new(&declared, &[], &mut scratch).unwrap();
        let mut nodes = Vec::new();
        for (index, (kind, a, b, literal)) in raw.into_iter().enumerate() {
            let pick = |value: u32| ExprId(if index == 0 { 0 } else { value % index as u32 });
            let node = match (kind, index) {
                (_, 0) | (0, _) => signal(10 + a % 2),
                (1, _) => ExprNode::Integer { value: literal },
                (2, _) => ExprNode::Decimal { coefficient: literal, scale: (a % 3) as u8 },
                (3, _) => ExprNode::Neg { operand: pick(a) },
                (4, _) => ExprNode::Abs { operand: pick(a) },
                (5, _) => ExprNode::Add { left: pick(a), right: pick(b) },
                (6, _) => ExprNode::Sub { left: pick(a), right: pick(b) },
                (7, _) => ExprNode::Mul { left: pick(a), right: pick(b) },
                _ => ExprNode::Rescale { operand: pick(a), scale: (b % 19) as u8 },
            };
            nodes.push(node);
        }
        let Ok(derived) = types(catalog, &nodes) else {
            return Ok(());
        };
        for a in [first_bound(first, false), first_bound(first, true)] {
            for b in [first_bound(second, false), first_bound(second, true)] {
                let inputs = BTreeMap::from([(10, a), (11, b)]);
                for index in 0..nodes.len() {
                    let value = reference(&nodes, &derived, index, &inputs);
                    prop_assert!(i64::try_from(value).is_ok());
                    prop_assert!(i128::from(derived[index].minimum) <= value);
                    prop_assert!(value <= i128::from(derived[index].maximum));
                }
            }
        }
    }
}

fn first_bound(domain: SignalDomain, upper: bool) -> i64 {
    match domain {
        SignalDomain::Integer(integer) if upper => integer.maximum(),
        SignalDomain::Integer(integer) => integer.minimum(),
        SignalDomain::FixedDecimal(decimal) if upper => decimal.maximum_coefficient(),
        SignalDomain::FixedDecimal(decimal) => decimal.minimum_coefficient(),
        SignalDomain::Boolean => 0,
    }
}

fn sample_table(catalog: SignalCatalog<'_>) -> PredicateTableDocument {
    let expressions = vec![
        signal(2),
        ExprNode::Integer { value: 3 },
        ExprNode::Add {
            left: ExprId(0),
            right: ExprId(1),
        },
        ExprNode::Sub {
            left: ExprId(0),
            right: ExprId(1),
        },
        ExprNode::Neg {
            operand: ExprId(0),
        },
        ExprNode::Abs {
            operand: ExprId(0),
        },
        ExprNode::Mul {
            left: ExprId(0),
            right: ExprId(1),
        },
        signal(3),
        ExprNode::Rescale {
            operand: ExprId(0),
            scale: 1,
        },
        ExprNode::Decimal {
            coefficient: 255,
            scale: 1,
        },
    ];
    let predicates = ComparisonOperator::ALL
        .into_iter()
        .enumerate()
        .map(|(index, operator)| Predicate {
            proposition: PropositionId(10 + index as u32),
            operator,
            left: ExprId(if index < 4 { 7 } else { 2 }),
            right: ExprId(if index < 4 { 9 } else { 6 }),
        })
        .collect();
    PredicateTableDocument::new(catalog, expressions, predicates).unwrap()
}

/// TC-1504: a table of every operator and node kind round-trips byte for byte;
/// the embedded schema equals the checked-in file and names the identity.
// Trace: TC-1504, FR-303-AC-1
#[test]
fn predicate_table_round_trips_and_embeds_its_schema() {
    with_catalog(|catalog| {
        let table = sample_table(catalog);
        let bytes = table.canonical_json_bytes().unwrap();
        let reread =
            PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
                .unwrap();
        assert_eq!(reread, table);
        assert_eq!(reread.canonical_json_bytes().unwrap(), bytes);
        assert!(String::from_utf8(bytes)
            .unwrap()
            .starts_with(r#"{"schema_version":"tl-syntax.predicate-table/v1","expressions":[{"kind":"signal","signal":2},"#));
        assert_eq!(table.schema_version().as_str(), PREDICATE_TABLE_V1);
        assert_eq!(
            PREDICATE_TABLE_V1_SCHEMA_BYTES,
            include_bytes!("../spec/predicate-table-v1.schema.json")
        );
        let schema: Value = serde_json::from_str(PREDICATE_TABLE_V1_SCHEMA).unwrap();
        assert_eq!(
            schema["properties"]["schema_version"]["const"],
            "tl-syntax.predicate-table/v1"
        );
    });
}

fn read_error(bytes: &[u8], limits: SyntaxArtifactLimits) -> PredicateTableReadError {
    with_catalog(|catalog| PredicateTableDocument::from_json_bytes(bytes, limits, catalog).unwrap_err())
}

/// TC-1505: every listed wire violation is refused with its typed error.
// Trace: TC-1505, FR-303-AC-2
#[test]
fn strict_reader_refuses_every_wire_violation() {
    let base = with_catalog(|catalog| sample_table(catalog).canonical_json_bytes().unwrap());
    let text = String::from_utf8(base).unwrap();
    let variant = |error: PredicateTableReadError| match error {
        PredicateTableReadError::Read(StrictDocumentReadError::InvalidDocument(_)) => "invalid",
        PredicateTableReadError::Read(StrictDocumentReadError::NonCanonicalDocument) => "noncanonical",
        PredicateTableReadError::Read(StrictDocumentReadError::DocumentTooLarge { .. }) => "oversize",
        PredicateTableReadError::Read(StrictDocumentReadError::DepthLimitExceeded { .. }) => "deep",
        PredicateTableReadError::Table(PredicateTableError::PropositionNotIncreasing { .. }) => {
            "increasing"
        }
        other => panic!("unexpected {other:?}"),
    };
    let limits = SyntaxArtifactLimits::default();
    for (mutated, expected) in [
        (text.replacen("\"value\":3", "\"value\":3.0", 1), "invalid"),
        (text.replacen("\"value\":3", "\"value\":3e0", 1), "invalid"),
        (text.replacen("\"value\":3", "\"value\":-0", 1), "noncanonical"),
        (text.replacen("\"value\":3", "\"value\":03", 1), "invalid"),
        (text.replacen("\"value\":3", "\"value\":3,\"value\":3", 1), "invalid"),
        (text.replacen("\"value\":3", "\"value\":3,\"unit\":1", 1), "invalid"),
        (format!("{text} null"), "invalid"),
        (text.replacen("\"proposition\":11", "\"proposition\":10", 1), "increasing"),
    ] {
        assert_eq!(variant(read_error(mutated.as_bytes(), limits)), expected, "{mutated}");
    }
    let small = SyntaxArtifactLimits {
        document_bytes: text.len() - 1,
        ..limits
    };
    assert_eq!(variant(read_error(text.as_bytes(), small)), "oversize");
    let deep = format!("{{\"x\":{}0{}}}", "[".repeat(64), "]".repeat(64));
    assert_eq!(variant(read_error(deep.as_bytes(), limits)), "deep");
}

/// TC-1506: existing formula, catalog and proposition-map fixtures and schema
/// digests are unchanged and decode as before.
// Trace: TC-1506, FR-303-AC-3
#[test]
fn existing_documents_are_unchanged() {
    assert_eq!(
        tl_syntax::SIGNAL_CATALOG_V1_SCHEMA_SHA256,
        "24fe7ccc26918e9a52dfa49a0e670ef932d0bfcd155d5d73dfd0be23dd276df8"
    );
    assert_eq!(
        tl_syntax::PROPOSITION_MAP_V1_SCHEMA_SHA256,
        "ed5be5c2747db16f7936a94b6203bb2747a1abb5a0c5521c2efa38beb8dfdc80"
    );
    assert_eq!(
        tl_syntax::FORMULA_UNBOUNDED_V1_SCHEMA_SHA256,
        "eae3e9d733fe288ad53fd321c67ad0da1cae3bbecad67df16bcd266ddcc10224"
    );
    for name in [
        "primitive-true",
        "nested-not-future",
        "boundary-singleton-globally",
        "large-bound-future",
        "short-trace-future",
    ] {
        let bytes = fs::read(PathBuf::from(CORPUS_DIR).join(format!("formulas/{name}.json"))).unwrap();
        let document: tl_syntax::FormulaDocument = serde_json::from_slice(&bytes).unwrap();
        let canonical = document.canonical_json_bytes().unwrap();
        let reread =
            tl_syntax::FormulaDocument::from_json_bytes(&canonical, SyntaxArtifactLimits::default())
                .unwrap();
        assert_eq!(reread, document, "{name}");
    }
    let catalog: SignalCatalogDocument =
        serde_json::from_slice(include_bytes!("fixtures/valid-signal-catalog.json")).unwrap();
    assert_eq!(
        catalog.schema_version().as_str(),
        "tl-syntax.signal-catalog/v1"
    );
}

/// TC-1507: deserialization fails exactly as direct construction.
// Trace: TC-1507, FR-303-AC-4
#[test]
fn deserialization_fails_exactly_as_construction() {
    let catalog_document = corpus_catalog();
    let catalog = catalog_document.validate().unwrap();
    for case in manifest()["cases"].as_array().unwrap() {
        if case["class"] != "refused-table" {
            continue;
        }
        let bytes = fs::read(numeric_dir().join(case["table"].as_str().unwrap())).unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        let expressions: Vec<ExprNode> = serde_json::from_value(value["expressions"].clone()).unwrap();
        let predicates: Vec<Predicate> = serde_json::from_value(value["predicates"].clone()).unwrap();
        let direct = PredicateTableDocument::new(catalog, expressions, predicates).unwrap_err();
        match PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog) {
            Err(PredicateTableReadError::Table(read)) => assert_eq!(read, direct, "{}", case["id"]),
            other => panic!("{}: {other:?}", case["id"]),
        }
    }
}

fn binding_fixture<R>(run: impl FnOnce(SignalCatalog<'_>, PredicateTable<'_>) -> R) -> R {
    with_catalog(|catalog| {
        let expressions = [signal(2), ExprNode::Integer { value: 3 }, signal(3)];
        let predicates = [
            Predicate {
                proposition: PropositionId(5),
                operator: ComparisonOperator::Lt,
                left: ExprId(0),
                right: ExprId(1),
            },
            Predicate {
                proposition: PropositionId(6),
                operator: ComparisonOperator::Ge,
                left: ExprId(2),
                right: ExprId(2),
            },
        ];
        let mut scratch = [ExprSlot::EMPTY; 3];
        let table = PredicateTable::new(catalog, &predicates, &expressions, &mut scratch).unwrap();
        run(catalog, table)
    })
}

fn proposition(id: u32) -> Node {
    Node::new(NodeKind::Proposition {
        proposition: PropositionId(id),
    })
}

/// TC-1508: Boolean-bound and predicate atoms bind together in node order.
// Trace: TC-1508, FR-304-AC-1
#[test]
fn mixed_atoms_bind_in_node_order() {
    binding_fixture(|catalog, table| {
        let nodes = [
            proposition(6),
            proposition(1),
            proposition(5),
            Node::new(NodeKind::And {
                left: NodeId(0),
                right: NodeId(1),
            }),
            Node::new(NodeKind::Or {
                left: NodeId(3),
                right: NodeId(2),
            }),
        ];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(4), &nodes).unwrap();
        let bound = BoundPredicateFormula::new(formula, catalog, table).unwrap();
        let occurrences: Vec<_> = bound.occurrences().collect();
        assert_eq!(occurrences.len(), 3);
        assert_eq!(occurrences[0].1, PropositionId(6));
        assert!(matches!(occurrences[0].2, PropositionSource::Predicate(p) if p.proposition == PropositionId(6)));
        assert!(matches!(occurrences[1].2, PropositionSource::Signal(s) if s.id() == SignalId(1)));
        assert!(matches!(occurrences[2].2, PropositionSource::Predicate(p) if p.proposition == PropositionId(5)));
        assert_eq!(bound.numeric_signals(), vec![SignalId(2), SignalId(3)]);
    });
}

/// TC-1509: unbound, doubly defined and non-numeric refusals; `bind_formula`
/// still fails closed on a predicate proposition.
// Trace: TC-1509, FR-304-AC-2
#[test]
fn binding_refusals_are_distinct_and_bind_formula_fails_closed() {
    binding_fixture(|catalog, table| {
        let unbound = [proposition(5), proposition(9)];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &unbound).unwrap();
        assert_eq!(
            BoundPredicateFormula::new(formula, catalog, table).unwrap_err(),
            PredicateBindingError::PropositionUnbound {
                proposition: PropositionId(9)
            }
        );
        let predicate_only = [proposition(5)];
        let formula =
            Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &predicate_only).unwrap();
        assert_eq!(
            catalog.bind_formula(formula).unwrap_err(),
            FormulaBindingError::MissingPropositionBinding {
                proposition: PropositionId(5)
            }
        );

        // Proposition 1 is bound in the catalog and also defined below.
        let expressions = [signal(2)];
        let twice = [Predicate {
            proposition: PropositionId(1),
            operator: ComparisonOperator::Eq,
            left: ExprId(0),
            right: ExprId(0),
        }];
        let mut scratch = [ExprSlot::EMPTY; 1];
        let doubled = PredicateTable::new(catalog, &twice, &expressions, &mut scratch).unwrap();
        let atom = [proposition(1)];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &atom).unwrap();
        assert_eq!(
            BoundPredicateFormula::new(formula, catalog, doubled).unwrap_err(),
            PredicateBindingError::PropositionDefinedTwice {
                proposition: PropositionId(1)
            }
        );

        // A table typed against another catalog where signal 2 is Boolean.
        let other = [SignalDeclaration::new(SignalId(2), "count", SignalDomain::Boolean)];
        let mut order = [0; 1];
        let other = SignalCatalog::new(&other, &[], &mut order).unwrap();
        let atom = [proposition(5)];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &atom).unwrap();
        assert_eq!(
            BoundPredicateFormula::new(formula, other, table).unwrap_err(),
            PredicateBindingError::NonNumericSignal {
                proposition: PropositionId(5),
                signal: SignalId(2)
            }
        );
    });
}

/// TC-1510: the temporal core is unchanged by the numeric layer.
// Trace: TC-1510, FR-304-AC-3
#[test]
fn temporal_core_identities_are_unchanged() {
    for (profile, text) in [
        (SemanticProfile::ClosedTraceV1, "mltl.closed-trace/v1"),
        (SemanticProfile::OnlinePrefixV1, "mltl.online-prefix/v1"),
        (
            SemanticProfile::OriginCompleteHistoryV1,
            "mltl.origin-complete-history/v1",
        ),
        (SemanticProfile::InfiniteTraceV1, "mltl.infinite-trace/v1"),
    ] {
        assert_eq!(profile.as_str(), text);
    }
    // A comparison is never a formula node: the wire refuses the tag.
    let refused = br#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"lt","left":0,"right":0}]}"#;
    assert!(tl_syntax::FormulaDocument::from_json_bytes(refused, SyntaxArtifactLimits::default())
        .is_err());
}

const REQUIRED_CLASSES: [&str; 5] =
    ["valid", "refused-table", "refused-wire", "aggregation", "contract-set"];

fn check_manifest(manifest: &Value, root: &std::path::Path) -> Result<(), String> {
    if manifest["corpus"] != "tl-syntax.numeric-corpus/v1" {
        return Err("corpus identity".into());
    }
    let declared = manifest["class_counts"].as_object().ok_or("class counts")?;
    let mut counted = BTreeMap::<String, u64>::new();
    for case in manifest["cases"].as_array().ok_or("cases")? {
        let class = case["class"].as_str().ok_or("class")?;
        *counted.entry(class.to_owned()).or_default() += 1;
        for key in ["table", "catalog", "truth", "document", "handwritten", "formula"] {
            if let Some(path) = case[key].as_str() {
                if !root.join(path).is_file() {
                    return Err(format!("missing {path}"));
                }
            }
        }
    }
    for class in REQUIRED_CLASSES {
        if !counted.contains_key(class) {
            return Err(format!("class {class} missing"));
        }
    }
    for (class, count) in &counted {
        if declared.get(class).and_then(Value::as_u64) != Some(*count) {
            return Err(format!("class {class} count {count} differs from manifest"));
        }
    }
    if declared.len() != counted.len() {
        return Err("manifest declares an absent class".into());
    }
    Ok(())
}

/// TC-1511: the manifest covers every class, files exist, counts match, and a
/// deleted class or case file fails the check.
// Trace: TC-1511, FR-305-AC-1
#[test]
fn numeric_manifest_is_closed_and_complete() {
    let root = numeric_dir();
    let manifest = manifest();
    check_manifest(&manifest, &root).unwrap();
    let mut without_class = manifest.clone();
    without_class["cases"]
        .as_array_mut()
        .unwrap()
        .retain(|case| case["class"] != "refused-wire");
    assert!(check_manifest(&without_class, &root).is_err());
    let mut missing_file = manifest.clone();
    missing_file["cases"][0]["table"] = Value::from("valid/deleted.json");
    assert!(check_manifest(&missing_file, &root).is_err());
    let mut dropped_case = manifest;
    dropped_case["cases"].as_array_mut().unwrap().remove(0);
    assert!(check_manifest(&dropped_case, &root).is_err());
}

fn refusal_class(error: &PredicateTableReadError) -> &'static str {
    match error {
        PredicateTableReadError::Read(StrictDocumentReadError::InvalidDocument(_)) => "InvalidDocument",
        PredicateTableReadError::Read(StrictDocumentReadError::NonCanonicalDocument) => {
            "NonCanonicalDocument"
        }
        PredicateTableReadError::Table(PredicateTableError::Expression(expression)) => match expression {
            ExpressionError::RangeOverflow { .. } => "RangeOverflow",
            ExpressionError::ScaleMismatch { .. } => "ScaleMismatch",
            ExpressionError::ScaleOverflow { .. } => "ScaleOverflow",
            ExpressionError::RescaleNarrowing { .. } => "RescaleNarrowing",
            ExpressionError::NonNumericSignal { .. } => "NonNumericSignal",
            ExpressionError::UnknownSignal { .. } => "UnknownSignal",
            ExpressionError::ExpressionLimitExceeded { .. } => "ExpressionLimitExceeded",
            ExpressionError::OperandNotPreceding { .. } => "OperandNotPreceding",
            ExpressionError::SlotOutsideTemplate { .. } => "SlotOutsideTemplate",
            _ => "other-expression",
        },
        PredicateTableReadError::Table(PredicateTableError::ScaleMismatch { .. }) => {
            "PredicateScaleMismatch"
        }
        PredicateTableReadError::Table(PredicateTableError::PropositionNotIncreasing { .. }) => {
            "PropositionNotIncreasing"
        }
        PredicateTableReadError::Table(PredicateTableError::RootOutOfRange { .. }) => "RootOutOfRange",
        _ => "other",
    }
}

/// TC-1512: refused cases give their declared class; valid cases give their
/// declared types and retain a truth table for the evaluators.
// Trace: TC-1512, FR-305-AC-2
#[test]
fn corpus_cases_yield_their_declared_outcomes() {
    let catalog_document = corpus_catalog();
    let catalog = catalog_document.validate().unwrap();
    for case in manifest()["cases"].as_array().unwrap() {
        if !["valid", "refused-table", "refused-wire"].contains(&case["class"].as_str().unwrap()) {
            continue; // aggregation and contract cases have their own suites
        }
        let bytes = fs::read(numeric_dir().join(case["table"].as_str().unwrap())).unwrap();
        let result =
            PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog);
        match case["class"].as_str().unwrap() {
            "valid" => {
                let table = result.unwrap();
                assert_eq!(
                    table.expression_types(catalog).unwrap().len(),
                    case["expected_types"].as_array().unwrap().len()
                );
                let truth: Value = serde_json::from_slice(
                    &fs::read(numeric_dir().join(case["truth"].as_str().unwrap())).unwrap(),
                )
                .unwrap();
                for row in truth.as_array().unwrap() {
                    assert_eq!(
                        row["truth"].as_array().unwrap().len(),
                        table.predicates().len()
                    );
                }
            }
            _ => {
                let error = result.unwrap_err();
                assert_eq!(refusal_class(&error), case["refusal"], "{}", case["id"]);
            }
        }
    }
}

/// TC-1513: the shared and past-history corpora identities are unchanged.
// Trace: TC-1513, FR-305-AC-3
#[test]
fn existing_corpora_are_unchanged() {
    let shared: Value =
        serde_json::from_slice(&fs::read(PathBuf::from(CORPUS_DIR).join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(shared["corpus_revision"], "tl-syntax-corpus/v1");
    assert_eq!(shared["fixtures"].as_array().unwrap().len(), 8);
    let past: Value = serde_json::from_slice(
        &fs::read(PathBuf::from(CORPUS_DIR).join("past-history/manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(past["corpus"], "tl-syntax.past-history-corpus/v1");
    assert_eq!(past["revision"], 1);
}

/// TC-1531: expression nodes, depth and predicates accept the limit and refuse
/// one past it.
// Trace: TC-1531, NFR-019-AC-1
#[cfg(not(miri))]
#[test]
fn numeric_limits_accept_the_limit_and_refuse_one_past() {
    with_catalog(|catalog| {
        let at_limit = vec![ExprNode::Integer { value: 0 }; MAX_EXPRESSION_NODES];
        assert!(types(catalog, &at_limit).is_ok());
        let past = vec![ExprNode::Integer { value: 0 }; MAX_EXPRESSION_NODES + 1];
        assert_eq!(
            types(catalog, &past),
            Err(ExpressionError::ExpressionLimitExceeded {
                resource: ExpressionLimit::Nodes,
                actual: MAX_EXPRESSION_NODES + 1,
                limit: MAX_EXPRESSION_NODES,
            })
        );
        let mut chain = vec![ExprNode::Integer { value: 0 }];
        for index in 0..(MAX_EXPRESSION_DEPTH as u32 - 1) {
            chain.push(ExprNode::Neg {
                operand: ExprId(index),
            });
        }
        assert!(types(catalog, &chain).is_ok());
        let expressions = [ExprNode::Integer { value: 0 }];
        let predicates: Vec<Predicate> = (0..=MAX_PREDICATES as u32)
            .map(|index| Predicate {
                proposition: PropositionId(index + 10),
                operator: ComparisonOperator::Eq,
                left: ExprId(0),
                right: ExprId(0),
            })
            .collect();
        let mut scratch = [ExprSlot::EMPTY; 1];
        assert!(PredicateTable::new(
            catalog,
            &predicates[..MAX_PREDICATES],
            &expressions,
            &mut scratch
        )
        .is_ok());
        assert_eq!(
            PredicateTable::new(catalog, &predicates, &expressions, &mut scratch),
            Err(PredicateTableError::PredicateLimitExceeded {
                count: MAX_PREDICATES + 1,
                limit: MAX_PREDICATES,
            })
        );
    });
}

/// TC-1532: corpus replay is byte-identical across repeated runs.
// Trace: TC-1532, NFR-019-AC-2
#[test]
fn corpus_replay_is_byte_identical() {
    let catalog_document = corpus_catalog();
    let catalog = catalog_document.validate().unwrap();
    for case in manifest()["cases"].as_array().unwrap() {
        if case["class"] != "valid" {
            continue;
        }
        let bytes = fs::read(numeric_dir().join(case["table"].as_str().unwrap())).unwrap();
        let first =
            PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
                .unwrap();
        let second =
            PredicateTableDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
                .unwrap();
        assert_eq!(first.canonical_json_bytes().unwrap(), bytes);
        assert_eq!(second.canonical_json_bytes().unwrap(), bytes);
        assert_eq!(
            first.expression_types(catalog).unwrap(),
            second.expression_types(catalog).unwrap()
        );
    }
}

/// TC-1539: no digest or pin beyond the ADR-019 register in numeric artifacts.
// Trace: TC-1539, NFR-019-AC-3
#[test]
fn numeric_artifacts_carry_no_digest() {
    let manifest = fs::read_to_string(numeric_dir().join("manifest.json")).unwrap();
    assert!(!manifest.to_ascii_lowercase().contains("sha256"));
    assert!(!manifest.to_ascii_lowercase().contains("digest"));
    assert!(!PREDICATE_TABLE_V1_SCHEMA.to_ascii_lowercase().contains("sha256"));
}
