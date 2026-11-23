//! P-TL-150 set aggregation (TL-424): FR-306, FR-307, FR-308.
#![cfg(feature = "serde")]

use std::{collections::BTreeSet, fs, path::PathBuf};

use proptest::prelude::*;
use serde_json::Value;
use tl_syntax::{
    expand_quantifier_into, quantifier_node_count, Aggregation, AggregationLimit,
    AggregationRefusal, AggregationTemplate, ComparisonOperator, ExpansionCounts, ExprId,
    ExprNode, ExprSlot, Formula, FormulaDocument, IntegerSignalDomain, MemberSet, Node, NodeId,
    NodeKind, OwnedSignalDeclaration, Predicate, PredicateTable, PropositionBinding,
    PropositionId, Quantifier, SemanticProfile, SetAggregationDocument, SetAggregationError,
    SignalCatalogDocument, SignalDomain, SignalId, SyntaxArtifactLimits, TemplateNode,
    CORPUS_DIR, MAX_SET_MEMBERS,
};

/// `count` Boolean signals 1..=count, proposition i bound to signal i, plus
/// Integer signals 1001..=1000+integers in [0, 9].
fn catalog(count: u32, integers: u32) -> SignalCatalogDocument {
    let mut signals: Vec<OwnedSignalDeclaration> = (1..=count)
        .map(|id| OwnedSignalDeclaration::new(SignalId(id), format!("b{id}"), SignalDomain::Boolean))
        .collect();
    signals.extend((1..=integers).map(|offset| {
        OwnedSignalDeclaration::new(
            SignalId(1000 + offset),
            format!("i{offset}"),
            SignalDomain::Integer(IntegerSignalDomain::new(0, 9).unwrap()),
        )
    }));
    let bindings = (1..=count)
        .map(|id| PropositionBinding::new(PropositionId(id), SignalId(id)))
        .collect();
    SignalCatalogDocument::new(signals, bindings).unwrap()
}

fn set(name: &str, members: impl IntoIterator<Item = u32>) -> MemberSet {
    MemberSet {
        name: name.to_owned(),
        members: members.into_iter().map(SignalId).collect(),
    }
}

fn slot_template(depth: u8) -> AggregationTemplate {
    AggregationTemplate {
        nodes: vec![TemplateNode::SlotProposition { depth }],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: None,
    }
}

fn quantified(set: &str, quantifier: Quantifier, template: AggregationTemplate) -> Aggregation {
    Aggregation {
        set: set.to_owned(),
        quantifier,
        template,
    }
}

/// A numeric template `slot > 5 && p10`, with placeholder proposition 50.
fn numeric_template() -> AggregationTemplate {
    AggregationTemplate {
        nodes: vec![
            TemplateNode::Node {
                node: Node::new(NodeKind::Proposition {
                    proposition: PropositionId(50),
                }),
            },
            TemplateNode::Node {
                node: Node::new(NodeKind::Proposition {
                    proposition: PropositionId(1),
                }),
            },
            TemplateNode::Node {
                node: Node::new(NodeKind::And {
                    left: NodeId(0),
                    right: NodeId(1),
                }),
            },
        ],
        root: NodeId(2),
        expressions: vec![
            ExprNode::SlotSignal { depth: 0 },
            ExprNode::Integer { value: 5 },
        ],
        predicates: vec![Predicate {
            proposition: PropositionId(50),
            operator: ComparisonOperator::Gt,
            left: ExprId(0),
            right: ExprId(1),
        }],
        inner: None,
    }
}

fn single(quantifier: Quantifier, k: u32) -> (SignalCatalogDocument, SetAggregationDocument) {
    let catalog_document = catalog(k.max(1), 0);
    let document = SetAggregationDocument::new(
        catalog_document.validate().unwrap(),
        vec![set("members", 1..=k)],
        PropositionId(100_000),
        vec![quantified("members", quantifier, slot_template(0))],
    )
    .unwrap();
    (catalog_document, document)
}

/// TC-1514: every quantifier, Boolean and numeric members and slot leaves
/// round-trip byte for byte through the strict reader.
// Trace: TC-1514, FR-306-AC-1
#[test]
fn aggregation_document_round_trips() {
    let catalog_document = catalog(4, 3);
    let catalog = catalog_document.validate().unwrap();
    let nested = AggregationTemplate {
        nodes: vec![TemplateNode::Inner],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: Some(Box::new(quantified(
            "flags",
            Quantifier::ForSome,
            slot_template(1),
        ))),
    };
    let document = SetAggregationDocument::new(
        catalog,
        vec![set("flags", 1..=4), set("levels", 1001..=1003)],
        PropositionId(200),
        vec![
            quantified("flags", Quantifier::ForEach, slot_template(0)),
            quantified("levels", Quantifier::ForSome, numeric_template()),
            quantified("flags", Quantifier::AtLeast { n: 2 }, slot_template(0)),
            quantified("levels", Quantifier::ForEach, nested),
        ],
    )
    .unwrap();
    let bytes = document.canonical_json_bytes().unwrap();
    let reread =
        SetAggregationDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
            .unwrap();
    assert_eq!(reread, document);
    assert_eq!(reread.canonical_json_bytes().unwrap(), bytes);
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.starts_with(r#"{"schema_version":"tl-syntax.set-aggregation/v1","sets":[{"name":"flags","members":[1,2,3,4]}"#));
    assert!(text.contains(r#"{"kind":"slot_signal","depth":0}"#));
    assert!(text.contains(r#"{"kind":"slot_proposition","depth":1}"#));
    assert!(text.contains(r#"{"kind":"at_least","n":2}"#));
    assert_eq!(
        tl_syntax::SET_AGGREGATION_V1_SCHEMA_BYTES,
        include_bytes!("../spec/set-aggregation-v1.schema.json")
    );
}

fn refused(
    catalog: &SignalCatalogDocument,
    sets: Vec<MemberSet>,
    first: u32,
    aggregations: Vec<Aggregation>,
) -> SetAggregationError {
    SetAggregationDocument::new(
        catalog.validate().unwrap(),
        sets,
        PropositionId(first),
        aggregations,
    )
    .unwrap_err()
}

/// TC-1515: misplaced slots, undeclared or duplicate members, an ill-typed
/// member, a missing binding and a colliding fresh range each have an error.
// Trace: TC-1515, FR-306-AC-2
#[test]
fn aggregation_refusals_are_distinct() {
    let catalog_document = catalog(3, 2);
    let each = |template| vec![quantified("s", Quantifier::ForEach, template)];
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1])], 100, each(slot_template(1))),
        SetAggregationError::SlotOutsideTemplate {
            aggregation: 0,
            depth: 0
        }
    );
    // Slot leaves outside a template: neither a formula nor a table admits one.
    let formula = br#"{"schema_version":"tl-syntax.formula/v2","semantic_profile":"mltl.closed-trace/v1","root":0,"nodes":[{"kind":"slot_proposition","depth":0}]}"#;
    assert!(FormulaDocument::from_json_bytes(formula, SyntaxArtifactLimits::default()).is_err());
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1, 9])], 100, each(slot_template(0))),
        SetAggregationError::UndeclaredMember {
            set: 0,
            signal: SignalId(9)
        }
    );
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1, 1])], 100, each(slot_template(0))),
        SetAggregationError::DuplicateMember {
            set: 0,
            signal: SignalId(1)
        }
    );
    // Member 2 is Boolean, so `slot > 5` is ill-typed for it alone.
    assert!(matches!(
        refused(&catalog_document, vec![set("s", [1001, 2])], 100, each(numeric_template())),
        SetAggregationError::TemplateIllTypedForMember {
            aggregation: 0,
            member: SignalId(2),
            ..
        }
    ));
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1, 1001])], 100, each(slot_template(0))),
        SetAggregationError::MemberHasNoBooleanBinding {
            aggregation: 0,
            member: SignalId(1001)
        }
    );
    // Fresh range [1, 3) overlaps the catalog binding of proposition 1.
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1001, 1002])], 1, each(numeric_template())),
        SetAggregationError::FreshIdentityCollision {
            proposition: PropositionId(1)
        }
    );
    assert_eq!(
        refused(&catalog_document, vec![set("s", [1])], 100, vec![quantified("t", Quantifier::ForEach, slot_template(0))]),
        SetAggregationError::UnknownSet { aggregation: 0 }
    );
}

proptest! {
    /// TC-1516: expansion is a pure function of the document.
    // Trace: TC-1516, FR-307-AC-1
    #[test]
    fn expansion_is_byte_deterministic(k in 0_u32..=6, n in 0_u32..=7, which in 0_u8..3) {
        let quantifier = match which {
            0 => Quantifier::ForEach,
            1 => Quantifier::ForSome,
            _ => Quantifier::AtLeast { n },
        };
        let (catalog_document, document) = single(quantifier, k);
        let catalog = catalog_document.validate().unwrap();
        let first = document.expand(catalog).unwrap();
        let second = document.expand(catalog).unwrap();
        prop_assert_eq!(&first, &second);
        prop_assert_eq!(serde_json::to_vec(&first).unwrap(), serde_json::to_vec(&second).unwrap());
    }
}

/// TC-1517: node, predicate and expression counts equal the closed forms on
/// the (k, n) grid, and every expansion is a valid formula.
// Trace: TC-1517, FR-307-AC-1
#[test]
fn expansion_counts_match_the_closed_forms() {
    for k in 0..=5_usize {
        let chain = if k == 0 { 1 } else { k - 1 };
        let mut cases = vec![(Quantifier::ForEach, chain), (Quantifier::ForSome, chain)];
        for n in 0..=6_usize {
            let added = if n == 0 || n > k {
                1
            } else {
                2 * n * (k - n + 1) + 2
            };
            cases.push((Quantifier::AtLeast { n: n as u32 }, added));
        }
        for (quantifier, added) in cases {
            assert_eq!(quantifier_node_count(quantifier, k).unwrap(), added);
            let (catalog_document, document) = single(quantifier, k as u32);
            let expanded = document.expand(catalog_document.validate().unwrap()).unwrap();
            assert_eq!(expanded.nodes.len(), k + added, "{quantifier:?} k={k}");
            assert_eq!(expanded.report.nodes, k + added);
            assert_eq!(expanded.report.members, k);
            assert!(expanded.predicates.is_empty());
            assert!(expanded.expressions.is_empty());
            Formula::new(SemanticProfile::ClosedTraceV1, expanded.roots[0], &expanded.nodes)
                .unwrap();
        }
    }
    // The worked case k = 3, n = 2: four reachable cells, 8 + 2 nodes.
    assert_eq!(quantifier_node_count(Quantifier::AtLeast { n: 2 }, 3).unwrap(), 10);
}

/// TC-1518: every bound is refused before output; one below succeeds.
// Trace: TC-1518, FR-307-AC-2, NFR-019-AC-1
#[cfg(not(miri))]
#[test]
fn expansion_bounds_are_refused_before_output() {
    let many = catalog(1_025, 0);
    let too_many = refused(
        &many,
        vec![set("s", 1..=1_025)],
        100_000,
        vec![quantified("s", Quantifier::ForEach, slot_template(0))],
    );
    assert_eq!(
        too_many,
        SetAggregationError::AggregationTooLarge {
            resource: AggregationLimit::Members,
            actual: 1_025,
            limit: MAX_SET_MEMBERS
        }
    );
    let wide = refused(
        &many,
        vec![set("s", 1..=1_024)],
        100_000,
        vec![quantified("s", Quantifier::AtLeast { n: 512 }, slot_template(0))],
    );
    assert!(matches!(
        wide,
        SetAggregationError::AggregationTooLarge {
            resource: AggregationLimit::Nodes,
            ..
        }
    ));
    let at_limit = SetAggregationDocument::new(
        many.validate().unwrap(),
        vec![set("s", 1..=1_024)],
        PropositionId(100_000),
        vec![quantified("s", Quantifier::ForEach, slot_template(0))],
    )
    .unwrap();
    assert_eq!(at_limit.counts().unwrap().nodes, 1_024 + 1_023);

    // k * e expression nodes: e = 98 is over 100,000 at k = 1,024; e = 97 is not.
    let numeric = catalog(0, 1_024);
    let chain_template = |length: u32| {
        let mut expressions = vec![ExprNode::SlotSignal { depth: 0 }];
        for index in 0..length - 1 {
            expressions.push(ExprNode::Neg {
                operand: ExprId(index),
            });
        }
        AggregationTemplate {
            nodes: vec![TemplateNode::Node {
                node: Node::new(NodeKind::Proposition {
                    proposition: PropositionId(50),
                }),
            }],
            root: NodeId(0),
            predicates: vec![Predicate {
                proposition: PropositionId(50),
                operator: ComparisonOperator::Le,
                left: ExprId(length - 1),
                right: ExprId(length - 1),
            }],
            expressions,
            inner: None,
        }
    };
    let members = || vec![set("s", 1_001..=2_024)];
    assert!(matches!(
        refused(&numeric, members(), 100_000, vec![quantified("s", Quantifier::ForEach, chain_template(98))]),
        SetAggregationError::AggregationTooLarge {
            resource: AggregationLimit::Expressions,
            actual: 100_352,
            ..
        }
    ));
    let below = SetAggregationDocument::new(
        numeric.validate().unwrap(),
        members(),
        PropositionId(100_000),
        vec![quantified("s", Quantifier::ForEach, chain_template(97))],
    )
    .unwrap();
    let counts = below.counts().unwrap();
    assert_eq!(counts.expressions, 99_328);
    assert!(counts.nodes < 100_000 && counts.predicates < 100_000);

    // Nested: each level alone is small, the product is not.
    let pairs = catalog(400, 0);
    let nested = AggregationTemplate {
        nodes: vec![TemplateNode::Inner],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: Some(Box::new(quantified("s", Quantifier::ForEach, slot_template(1)))),
    };
    assert!(matches!(
        refused(&pairs, vec![set("s", 1..=400)], 100_000, vec![quantified("s", Quantifier::ForEach, nested)]),
        SetAggregationError::AggregationTooLarge {
            resource: AggregationLimit::Nodes,
            ..
        }
    ));
    assert!(SetAggregationDocument::new(
        pairs.validate().unwrap(),
        vec![set("s", 1..=400)],
        PropositionId(100_000),
        vec![quantified("s", Quantifier::ForEach, slot_template(0))],
    )
    .is_ok());
}

/// TC-1519: too-small output reports exact counts and writes nothing; the
/// allocation-free quantifier core works on stack arrays.
// Trace: TC-1519, FR-307-AC-3
#[test]
fn expansion_too_small_reports_exact_counts_and_writes_nothing() {
    let (catalog_document, document) = single(Quantifier::AtLeast { n: 2 }, 3);
    let catalog = catalog_document.validate().unwrap();
    let sentinel = Node::new(NodeKind::Proposition {
        proposition: PropositionId(u32::MAX),
    });
    let mut nodes = vec![sentinel; 12];
    let mut roots = [NodeId(u32::MAX); 1];
    let refusal = document
        .expand_into(catalog, &mut nodes, &mut [], &mut [], &mut roots)
        .unwrap_err();
    assert_eq!(
        refusal,
        AggregationRefusal::ExpansionTooSmall {
            required: ExpansionCounts {
                nodes: 13,
                predicates: 0,
                expressions: 0,
                members: 3,
            }
        }
    );
    assert!(nodes.iter().all(|node| *node == sentinel));
    assert_eq!(roots, [NodeId(u32::MAX)]);

    let leaves = [NodeId(0), NodeId(1), NodeId(2)];
    let mut out = [sentinel; 9];
    assert_eq!(
        expand_quantifier_into(Quantifier::AtLeast { n: 2 }, &leaves, NodeId(3), &mut out),
        Err(AggregationRefusal::ExpansionTooSmall {
            required: ExpansionCounts {
                nodes: 10,
                predicates: 0,
                expressions: 0,
                members: 3,
            }
        })
    );
    assert!(out.iter().all(|node| *node == sentinel));
    let mut out = [sentinel; 10];
    let (root, written) =
        expand_quantifier_into(Quantifier::AtLeast { n: 2 }, &leaves, NodeId(3), &mut out)
            .unwrap();
    assert_eq!(written, 10);
    assert_eq!(root, NodeId(12));
    let mut all = vec![
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(1)
        });
        3
    ];
    all.extend_from_slice(&out);
    Formula::new(SemanticProfile::ClosedTraceV1, root, &all).unwrap();
}

fn numeric_dir() -> PathBuf {
    PathBuf::from(CORPUS_DIR).join("numeric")
}

fn read_json(relative: &str) -> Value {
    serde_json::from_slice(&fs::read(numeric_dir().join(relative)).unwrap()).unwrap()
}

fn aggregation_cases() -> Vec<Value> {
    read_json("manifest.json")["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["class"] == "aggregation")
        .cloned()
        .collect()
}

/// TC-1520: every aggregation case has a hand-written form and a population
/// whose computed trace count equals the manifest.
// Trace: TC-1520, FR-308-AC-1
#[test]
fn aggregation_cases_carry_handwritten_forms_and_populations() {
    let mut total = 0_u64;
    let cases = aggregation_cases();
    assert_eq!(cases.len(), 54);
    for case in &cases {
        let handwritten: FormulaDocument =
            serde_json::from_value(read_json(case["handwritten"].as_str().unwrap())).unwrap();
        handwritten.validate().unwrap();
        let population = &case["population"];
        assert_eq!(population["trace_length"], 1);
        let valuations = population["valuations"].as_array().unwrap().len() as u64;
        let members = population["members"].as_u64().unwrap();
        let traces = valuations.pow(members as u32);
        assert_eq!(Some(traces), case["traces"].as_u64(), "{}", case["id"]);
        total += traces;
    }
    assert_eq!(total, 567);
    assert_eq!(read_json("manifest.json")["aggregation_compared_traces"], 567);
}

fn propositions(nodes: &[Node]) -> BTreeSet<PropositionId> {
    nodes
        .iter()
        .filter_map(|node| match node.kind {
            NodeKind::Proposition { proposition } => Some(proposition),
            _ => None,
        })
        .collect()
}

fn structural_match(expanded: &[Node], handwritten: &[Node]) -> bool {
    propositions(expanded) == propositions(handwritten)
}

/// TC-1521: the expansion reads exactly the hand-written form's propositions,
/// and an expansion with a member omitted fails the check.
// Trace: TC-1521, FR-308-AC-2
#[test]
fn structural_check_passes_and_detects_an_omitted_member() {
    for case in aggregation_cases() {
        let catalog_document: SignalCatalogDocument =
            serde_json::from_value(read_json(case["catalog"].as_str().unwrap())).unwrap();
        let catalog = catalog_document.validate().unwrap();
        let bytes = fs::read(numeric_dir().join(case["document"].as_str().unwrap())).unwrap();
        let document =
            SetAggregationDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
                .unwrap();
        let expanded = document.expand(catalog).unwrap();
        let handwritten: FormulaDocument =
            serde_json::from_value(read_json(case["handwritten"].as_str().unwrap())).unwrap();
        assert!(
            structural_match(&expanded.nodes, handwritten.nodes()),
            "{}",
            case["id"]
        );
    }
    let catalog_document: SignalCatalogDocument =
        serde_json::from_value(read_json("catalogs/boolean-members.json")).unwrap();
    let catalog = catalog_document.validate().unwrap();
    let original = fs::read_to_string(numeric_dir().join("aggregation/for_each-k3.json")).unwrap();
    let altered = original.replacen("\"members\":[1,2,3]", "\"members\":[1,2]", 1);
    assert_ne!(altered, original);
    let bytes = altered.into_bytes();
    let document =
        SetAggregationDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), catalog)
            .unwrap();
    let expanded = document.expand(catalog).unwrap();
    let handwritten: FormulaDocument =
        serde_json::from_value(read_json("handwritten/for_each-k3.json")).unwrap();
    assert!(!structural_match(&expanded.nodes, handwritten.nodes()));
}

/// TC-1538: fresh ranges are consecutive in source order above every caller
/// identity; overlaps and nesting beyond two are refused.
// Trace: TC-1538, FR-307-AC-4
#[test]
fn fresh_identities_follow_source_order_and_nesting_is_bounded() {
    let catalog_document = catalog(3, 3);
    let catalog = catalog_document.validate().unwrap();
    let sets = || vec![set("a", 1_001..=1_003), set("b", 1_001..=1_002)];
    let two = || {
        vec![
            quantified("a", Quantifier::ForEach, numeric_template()),
            quantified("b", Quantifier::ForSome, numeric_template()),
        ]
    };
    let document = SetAggregationDocument::new(catalog, sets(), PropositionId(200), two()).unwrap();
    let expanded = document.expand(catalog).unwrap();
    let fresh: Vec<u32> = expanded
        .predicates
        .iter()
        .map(|predicate| predicate.proposition.0)
        .collect();
    assert_eq!(fresh, [200, 201, 202, 203, 204]);
    // Instance order: aggregation "a" members first, in document order.
    let reads: Vec<SignalId> = expanded
        .predicates
        .iter()
        .map(|predicate| match expanded.expressions[predicate.left.0 as usize] {
            ExprNode::Signal { signal } => signal,
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(
        reads,
        [1_001, 1_002, 1_003, 1_001, 1_002].map(SignalId)
    );
    let mut scratch = vec![ExprSlot::EMPTY; expanded.expressions.len()];
    PredicateTable::new(catalog, &expanded.predicates, &expanded.expressions, &mut scratch).unwrap();
    for root in &expanded.roots {
        Formula::new(SemanticProfile::ClosedTraceV1, *root, &expanded.nodes).unwrap();
    }
    // The caller placeholder 50 and binding 1 lie below 200; a range over them fails.
    assert_eq!(
        refused(&catalog_document, sets(), 48, two()),
        SetAggregationError::FreshIdentityCollision {
            proposition: PropositionId(50)
        }
    );
    assert_eq!(
        refused(&catalog_document, sets(), u32::MAX - 1, two()),
        SetAggregationError::FreshIdentityCollision {
            proposition: PropositionId(u32::MAX - 1)
        }
    );
    let innermost = quantified("a", Quantifier::ForEach, slot_template(0));
    let middle = AggregationTemplate {
        nodes: vec![TemplateNode::Inner],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: Some(Box::new(innermost)),
    };
    let outer = AggregationTemplate {
        nodes: vec![TemplateNode::Inner],
        root: NodeId(0),
        expressions: vec![],
        predicates: vec![],
        inner: Some(Box::new(quantified("a", Quantifier::ForEach, middle))),
    };
    assert_eq!(
        refused(
            &catalog_document,
            vec![set("a", 1..=3)],
            200,
            vec![quantified("a", Quantifier::ForEach, outer)]
        ),
        SetAggregationError::AggregationTooDeep { aggregation: 0 }
    );
}
