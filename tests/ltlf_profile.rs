//! P-TL-151 LTLf finite-trace profile (TL-428 syntax half): FR-332..FR-335,
//! NFR-048 (TM-048, TC-220..TC-231).
#![cfg(feature = "serde")]

use std::{collections::BTreeSet, fs, path::PathBuf};

use serde_json::Value;
use tl_syntax::{
    select_ltlf_identity, validate_ltlf, Formula, FormulaDocument, FormulaError,
    InfiniteFormulaDocument, Interval, LtlfFormulaDocument, LtlfFormulaError, LtlfIdentityError,
    InfiniteNodeKind, LtlfReadError, LtlfRefusal, Node, NodeId, NodeKind, PropositionId,
    SemanticProfile, StrictDocumentReadError, SyntaxArtifactLimits, TemporalFamily, CORPUS_DIR,
    FORMULA_LTLF_V1, FORMULA_LTLF_V1_SCHEMA, FORMULA_LTLF_V1_SCHEMA_BYTES,
    LTLF_FINITE_TRACE_PROFILE, MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES,
};

const LTLF: SemanticProfile = SemanticProfile::LtlfFiniteTraceV1;

fn leaf(id: u32) -> Node {
    Node::new(NodeKind::Proposition {
        proposition: PropositionId(id),
    })
}

fn unary(kind: fn(NodeId) -> NodeKind) -> Vec<Node> {
    vec![leaf(0), Node::new(kind(NodeId(0)))]
}

fn read(text: &str) -> Result<LtlfFormulaDocument, LtlfReadError> {
    LtlfFormulaDocument::from_json_bytes(text.as_bytes(), SyntaxArtifactLimits::default())
}

fn wire(nodes: &str, root: u32) -> String {
    format!(
        r#"{{"schema_version":"{FORMULA_LTLF_V1}","semantic_profile":"{LTLF_FINITE_TRACE_PROFILE}","root":{root},"nodes":[{nodes}]}}"#
    )
}

/// Every admitted node kind in one graph.
fn full_vocabulary() -> Vec<Node> {
    vec![
        Node::new(NodeKind::False),
        Node::new(NodeKind::True),
        leaf(0),
        Node::new(NodeKind::Not { operand: NodeId(2) }),
        Node::new(NodeKind::And {
            left: NodeId(1),
            right: NodeId(3),
        }),
        Node::new(NodeKind::Or {
            left: NodeId(0),
            right: NodeId(4),
        }),
        Node::new(NodeKind::Implies {
            left: NodeId(4),
            right: NodeId(5),
        }),
        Node::new(NodeKind::Equivalent {
            left: NodeId(5),
            right: NodeId(6),
        }),
        Node::new(NodeKind::LtlfFuture { operand: NodeId(7) }),
        Node::new(NodeKind::LtlfGlobally { operand: NodeId(8) }),
        Node::new(NodeKind::LtlfUntil {
            left: NodeId(8),
            right: NodeId(9),
        }),
        Node::new(NodeKind::LtlfRelease {
            left: NodeId(9),
            right: NodeId(10),
        }),
        Node::new(NodeKind::StrongNext { operand: NodeId(11) }),
        Node::new(NodeKind::WeakNext { operand: NodeId(12) }),
    ]
}

/// TC-220: the full admitted vocabulary validates under the LTLf identity.
// Trace: TC-220, FR-332-AC-1
#[test]
fn ltlf_vocabulary_is_admitted_under_its_identity() {
    let nodes = full_vocabulary();
    let formula = validate_ltlf(NodeId(13), &nodes).unwrap();
    assert_eq!(formula.profile(), LTLF);
    assert_eq!(formula.profile().as_str(), "mltl.ltlf-finite-trace/v1");
    let document = LtlfFormulaDocument::new(LTLF, NodeId(13), nodes).unwrap();
    assert_eq!(document.schema_version(), "tl-syntax.formula-ltlf/v1");
    let bytes = document.canonical_json_bytes().unwrap();
    let reread =
        LtlfFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).unwrap();
    assert_eq!(reread, document);
    assert_eq!(reread.canonical_json_bytes().unwrap(), bytes);
    assert_eq!(reread.formula().profile(), LTLF);
    assert_eq!(
        FORMULA_LTLF_V1_SCHEMA_BYTES,
        include_bytes!("../corpus/schema/formula-ltlf-v1.schema.json")
    );
}

fn refused_wire(text: &str) -> LtlfRefusal {
    match read(text) {
        Err(LtlfReadError::Formula(LtlfFormulaError::Refused { refusal, .. })) => refusal,
        other => panic!("{text}: {other:?}"),
    }
}

/// TC-221: each of the nine refused classes has its own typed cause and no
/// partial formula, on the wire and, where representable, in memory.
// Trace: TC-221, FR-332-AC-2, StR-045-VC-1
#[test]
fn nine_refusal_classes_are_typed() {
    let p = r#"{"kind":"proposition","proposition":0}"#;
    let q = r#"{"kind":"proposition","proposition":1}"#;
    let cases = [
        (
            format!(r#"{p},{{"kind":"future","interval":{{"kind":"closed","start":0,"end":3}},"operand":0}}"#),
            LtlfRefusal::ClosedInterval,
        ),
        (
            format!(r#"{p},{{"kind":"globally","interval":{{"kind":"unbounded","start":0}},"operand":0}}"#),
            LtlfRefusal::OpenInterval,
        ),
        (
            format!(r#"{p},{{"kind":"until","interval":{{"kind":"unbounded","start":2}},"left":0,"right":0}}"#),
            LtlfRefusal::OpenInterval,
        ),
        (format!(r#"{p},{{"kind":"once","operand":0}}"#), LtlfRefusal::Once),
        (
            format!(r#"{p},{{"kind":"historically","operand":0}}"#),
            LtlfRefusal::Historically,
        ),
        (
            format!(r#"{p},{{"kind":"strong_previous","operand":0}}"#),
            LtlfRefusal::Previous,
        ),
        (format!(r#"{p},{q},{{"kind":"since","left":0,"right":1}}"#), LtlfRefusal::Since),
        (
            format!(r#"{p},{q},{{"kind":"triggered","left":0,"right":1}}"#),
            LtlfRefusal::Triggered,
        ),
        (
            format!(r#"{p},{q},{{"kind":"weak_until","left":0,"right":1}}"#),
            LtlfRefusal::WeakUntil,
        ),
        (
            format!(r#"{p},{q},{{"kind":"strong_release","left":0,"right":1}}"#),
            LtlfRefusal::StrongRelease,
        ),
    ];
    let mut seen = BTreeSet::new();
    for (nodes, expected) in cases {
        // The refused node is always the last top-level node.
        let value: Value = serde_json::from_str(&format!("[{nodes}]")).unwrap();
        let root = value.as_array().unwrap().len() as u32 - 1;
        assert_eq!(refused_wire(&wire(&nodes, root)), expected);
        seen.insert(expected);
    }
    // A refused node that carries a span, as a parser emits it, keeps its cause.
    let spanned = format!(
        r#"{p},{q},{{"kind":"weak_until","left":0,"right":1,"span":{{"start":0,"end":5}}}}"#
    );
    assert_eq!(refused_wire(&wire(&spanned, 2)), LtlfRefusal::WeakUntil);
    let spanned = format!(r#"{p},{{"kind":"once","operand":0,"span":{{"start":0,"end":3}}}}"#);
    assert_eq!(refused_wire(&wire(&spanned, 1)), LtlfRefusal::Once);
    assert_eq!(seen.len(), LtlfRefusal::ALL.len());

    let interval = Interval::new(0, 3).unwrap();
    for (kind, expected) in [
        (
            NodeKind::Future {
                interval,
                operand: NodeId(0),
            },
            LtlfRefusal::ClosedInterval,
        ),
        (
            NodeKind::Release {
                interval,
                left: NodeId(0),
                right: NodeId(0),
            },
            LtlfRefusal::ClosedInterval,
        ),
        (
            NodeKind::Once {
                interval,
                operand: NodeId(0),
            },
            LtlfRefusal::Once,
        ),
        (
            NodeKind::Historically {
                interval,
                operand: NodeId(0),
            },
            LtlfRefusal::Historically,
        ),
        (NodeKind::StrongPrevious { operand: NodeId(0) }, LtlfRefusal::Previous),
        (
            NodeKind::Since {
                interval,
                left: NodeId(0),
                right: NodeId(0),
            },
            LtlfRefusal::Since,
        ),
        (
            NodeKind::Triggered {
                interval,
                left: NodeId(0),
                right: NodeId(0),
            },
            LtlfRefusal::Triggered,
        ),
    ] {
        let nodes = [leaf(0), Node::new(kind)];
        assert_eq!(
            validate_ltlf(NodeId(1), &nodes),
            Err(LtlfFormulaError::Refused {
                node: NodeId(1),
                refusal: expected
            })
        );
        assert_eq!(
            LtlfFormulaDocument::new(LTLF, NodeId(1), nodes.to_vec()),
            Err(LtlfFormulaError::Refused {
                node: NodeId(1),
                refusal: expected
            })
        );
    }
}

/// TC-222: missing, unknown and mismatched identities name their axis.
// Trace: TC-222, FR-332-AC-3
#[test]
fn identity_refusals_name_the_axis() {
    let ltlf = Some(LTLF_FINITE_TRACE_PROFILE);
    let edition = Some(FORMULA_LTLF_V1);
    assert_eq!(select_ltlf_identity(edition, ltlf), Ok(LTLF));
    assert_eq!(select_ltlf_identity(None, ltlf), Err(LtlfIdentityError::MissingEdition));
    assert_eq!(select_ltlf_identity(Some(""), ltlf), Err(LtlfIdentityError::MissingEdition));
    assert_eq!(
        select_ltlf_identity(Some("tl-syntax.formula-ltlf/v2"), ltlf),
        Err(LtlfIdentityError::UnknownEdition)
    );
    for existing in [
        "tl-syntax.formula/v1",
        "tl-syntax.formula/v2",
        "tl-syntax.formula-unbounded/v1",
    ] {
        assert_eq!(
            select_ltlf_identity(Some(existing), ltlf),
            Err(LtlfIdentityError::MismatchedEdition)
        );
    }
    assert_eq!(select_ltlf_identity(edition, None), Err(LtlfIdentityError::MissingProfile));
    assert_eq!(
        select_ltlf_identity(edition, Some("ltlf")),
        Err(LtlfIdentityError::UnknownProfile)
    );
    for profile in &SemanticProfile::ALL[..4] {
        assert_eq!(
            select_ltlf_identity(edition, Some(profile.as_str())),
            Err(LtlfIdentityError::MismatchedProfile(*profile))
        );
        assert_eq!(
            LtlfFormulaDocument::new(*profile, NodeId(0), vec![leaf(0)]),
            Err(LtlfFormulaError::Identity(
                LtlfIdentityError::MismatchedProfile(*profile)
            ))
        );
    }
    let p = r#"{"kind":"proposition","proposition":0}"#;
    let closed = wire(p, 0).replace(LTLF_FINITE_TRACE_PROFILE, "mltl.closed-trace/v1");
    assert!(matches!(
        read(&closed),
        Err(LtlfReadError::Formula(LtlfFormulaError::Identity(
            LtlfIdentityError::MismatchedProfile(SemanticProfile::ClosedTraceV1)
        )))
    ));
    let v2 = wire(p, 0).replace(FORMULA_LTLF_V1, "tl-syntax.formula/v2");
    assert!(matches!(
        read(&v2),
        Err(LtlfReadError::Formula(LtlfFormulaError::Identity(
            LtlfIdentityError::MismatchedEdition
        )))
    ));
    let missing = wire(p, 0).replace(
        &format!(r#""semantic_profile":"{LTLF_FINITE_TRACE_PROFILE}","#),
        "",
    );
    assert!(matches!(
        read(&missing),
        Err(LtlfReadError::Formula(LtlfFormulaError::Identity(
            LtlfIdentityError::MissingProfile
        )))
    ));
    let missing = wire(p, 0).replace(&format!(r#""schema_version":"{FORMULA_LTLF_V1}","#), "");
    assert!(matches!(
        read(&missing),
        Err(LtlfReadError::Formula(LtlfFormulaError::Identity(
            LtlfIdentityError::MissingEdition
        )))
    ));
}

/// TC-223: `Xs p` and `Xw p` are different graphs and both round-trip.
// Trace: TC-223, FR-333-AC-1, StR-045-VC-2
#[test]
fn strong_and_weak_next_are_distinct_graphs() {
    let strong = LtlfFormulaDocument::new(
        LTLF,
        NodeId(1),
        unary(|operand| NodeKind::StrongNext { operand }),
    )
    .unwrap();
    let weak = LtlfFormulaDocument::new(
        LTLF,
        NodeId(1),
        unary(|operand| NodeKind::WeakNext { operand }),
    )
    .unwrap();
    assert_ne!(
        strong.content_identity().unwrap(),
        weak.content_identity().unwrap()
    );
    for document in [&strong, &weak] {
        let bytes = document.canonical_json_bytes().unwrap();
        assert_eq!(
            &LtlfFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default()).unwrap(),
            document
        );
    }
    assert!(String::from_utf8(strong.canonical_json_bytes().unwrap())
        .unwrap()
        .contains(r#"{"kind":"strong_next","operand":0}"#));
    assert!(String::from_utf8(weak.canonical_json_bytes().unwrap())
        .unwrap()
        .contains(r#"{"kind":"weak_next","operand":0}"#));
    // `Xw p` and `not Xs not p` stay structurally distinct graphs.
    let dual = LtlfFormulaDocument::new(
        LTLF,
        NodeId(3),
        vec![
            leaf(0),
            Node::new(NodeKind::Not { operand: NodeId(0) }),
            Node::new(NodeKind::StrongNext { operand: NodeId(1) }),
            Node::new(NodeKind::Not { operand: NodeId(2) }),
        ],
    )
    .unwrap();
    assert_ne!(
        dual.content_identity().unwrap(),
        weak.content_identity().unwrap()
    );
}

/// TC-224: no bare next exists on the wire or in the node vocabulary.
// Trace: TC-224, FR-333-AC-2, StR-045-VC-2
#[test]
fn bare_next_is_refused() {
    let p = r#"{"kind":"proposition","proposition":0}"#;
    for tag in ["next", "x", "Next", "X", "strongnext", "weak", "next_strong"] {
        let text = wire(&format!(r#"{p},{{"kind":"{tag}","operand":0}}"#), 1);
        assert!(
            matches!(read(&text), Err(LtlfReadError::Read(_))),
            "{tag} was admitted"
        );
    }
    // The only next nodes carry their strength in the variant itself.
    let strengths: Vec<&str> = [
        NodeKind::StrongNext { operand: NodeId(0) },
        NodeKind::WeakNext { operand: NodeId(0) },
    ]
    .iter()
    .map(|kind| match kind {
        NodeKind::StrongNext { .. } => "strong",
        NodeKind::WeakNext { .. } => "weak",
        _ => "other",
    })
    .collect();
    assert_eq!(strengths, ["strong", "weak"]);
}

/// TC-225: strong and weak next are refused under every existing profile and
/// edition.
// Trace: TC-225, FR-333-AC-3
#[test]
fn next_nodes_are_refused_outside_ltlf() {
    for kind in [
        NodeKind::StrongNext { operand: NodeId(0) },
        NodeKind::WeakNext { operand: NodeId(0) },
        NodeKind::LtlfFuture { operand: NodeId(0) },
        NodeKind::LtlfUntil {
            left: NodeId(0),
            right: NodeId(0),
        },
    ] {
        let nodes = [leaf(0), Node::new(kind)];
        for profile in &SemanticProfile::ALL[..4] {
            assert_eq!(
                Formula::new(*profile, NodeId(1), &nodes),
                Err(FormulaError::ProfileIncompatibleNode {
                    profile: *profile,
                    node: NodeId(1),
                    family: TemporalFamily::Future,
                })
            );
        }
        assert!(FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(1), nodes.to_vec())
            .is_err());
    }
    let p = r#"{"kind":"proposition","proposition":0}"#;
    for (edition, profile, extra) in [
        ("tl-syntax.formula/v1", "mltl.closed-trace/v1", ""),
        ("tl-syntax.formula/v2", "mltl.closed-trace/v1", ""),
        (
            "tl-syntax.formula-unbounded/v1",
            "mltl.infinite-trace/v1",
            r#""clock":"event_position","#,
        ),
    ] {
        let document = |second: &str| {
            format!(
                r#"{{"schema_version":"{edition}","semantic_profile":"{profile}",{extra}"root":1,"nodes":[{p},{second}]}}"#
            )
        };
        let limits = SyntaxArtifactLimits::default();
        let decode = |text: &str| -> Result<(), String> {
            if edition.contains("unbounded") {
                InfiniteFormulaDocument::from_json_bytes(text.as_bytes(), limits)
                    .map(|_| ())
                    .map_err(|error| format!("{error:?} {error}"))
            } else {
                FormulaDocument::from_json_bytes(text.as_bytes(), limits)
                    .map(|_| ())
                    .map_err(|error| format!("{error:?} {error}"))
            }
        };
        // Control: the same document with `not` in place of the next node reads.
        decode(&document(r#"{"kind":"not","operand":0}"#)).unwrap();
        for tag in ["strong_next", "weak_next"] {
            let error = decode(&document(&format!(r#"{{"kind":"{tag}","operand":0}}"#)))
                .unwrap_err();
            assert!(error.contains("InvalidDocument"), "{edition} {tag}: {error}");
            assert!(error.contains(&format!("unknown variant `{tag}`")), "{edition}: {error}");
        }
    }
    // A bounded edition cannot carry the LTLf profile at all.
    assert_eq!(
        FormulaDocument::new_v2(LTLF, NodeId(0), vec![leaf(0)]).unwrap_err(),
        FormulaError::LtlfProfileRequiresLtlfEdition
    );
}

/// TC-226: the existing corpus and profile wire forms are unchanged.
///
/// SPEC-GAP: FR-334-AC-1 asks for pre-change digests; decision 7 adds no new
/// digest, so this replays the existing fixtures and asserts their identity
/// strings. Byte drift is owned by the release gate: its FR-031 golden lane
/// compares every pre-existing `corpus/` file byte for byte with the
/// preceding tag, so this test does not claim byte identity.
// Trace: TC-226, FR-334-AC-1, StR-045-VC-1
#[test]
fn existing_corpus_and_profile_wire_forms_are_unchanged() {
    let corpus = PathBuf::from(CORPUS_DIR);
    let manifest: Value =
        serde_json::from_slice(&fs::read(corpus.join("manifest.json")).unwrap()).unwrap();
    let fixtures = manifest["fixtures"].as_array().unwrap();
    assert_eq!(fixtures.len(), 8);
    for fixture in fixtures {
        let bytes = fs::read(corpus.join(fixture["formula"].as_str().unwrap())).unwrap();
        let decoded = serde_json::from_slice::<FormulaDocument>(&bytes);
        assert_eq!(
            decoded.is_ok(),
            fixture["expected_validation"] == "valid",
            "{}",
            fixture["id"]
        );
    }
    for (profile, text) in [
        (SemanticProfile::ClosedTraceV1, "\"mltl.closed-trace/v1\""),
        (SemanticProfile::OnlinePrefixV1, "\"mltl.online-prefix/v1\""),
        (
            SemanticProfile::OriginCompleteHistoryV1,
            "\"mltl.origin-complete-history/v1\"",
        ),
        (SemanticProfile::InfiniteTraceV1, "\"mltl.infinite-trace/v1\""),
    ] {
        assert_eq!(serde_json::to_string(&profile).unwrap(), text);
    }
}

/// TC-227: every existing edition decoder refuses the LTLf schema string.
// Trace: TC-227, FR-334-AC-2
#[test]
fn existing_decoders_refuse_the_ltlf_edition() {
    let text = wire(r#"{"kind":"proposition","proposition":0}"#, 0);
    let limits = SyntaxArtifactLimits::default();
    let unknown_schema = format!("unknown variant `{FORMULA_LTLF_V1}`");
    match FormulaDocument::from_json_bytes(text.as_bytes(), limits) {
        Err(StrictDocumentReadError::InvalidDocument(error)) => {
            assert!(error.to_string().contains(&unknown_schema), "{error}")
        }
        other => panic!("formula decoder: {other:?}"),
    }
    match InfiniteFormulaDocument::from_json_bytes(text.as_bytes(), limits) {
        Err(StrictDocumentReadError::InvalidDocument(error)) => {
            assert!(error.to_string().contains(&unknown_schema), "{error}")
        }
        other => panic!("unbounded decoder: {other:?}"),
    }
}

/// TC-228: five distinct profile identities, four old strings verbatim.
// Trace: TC-228, FR-334-AC-3
#[test]
fn five_distinct_profile_identities() {
    assert_eq!(SemanticProfile::ALL.len(), 5);
    let identities: BTreeSet<&str> = SemanticProfile::ALL.iter().map(|p| p.as_str()).collect();
    assert_eq!(identities.len(), 5);
    for old in [
        "mltl.closed-trace/v1",
        "mltl.online-prefix/v1",
        "mltl.origin-complete-history/v1",
        "mltl.infinite-trace/v1",
    ] {
        assert!(identities.contains(old));
    }
    assert!(identities.contains("mltl.ltlf-finite-trace/v1"));
}

/// Test-only ADR-044 reference: the verdict of `index` at position `i` of a
/// nonempty trace listing the true propositions per position.
fn reference(nodes: &[Node], index: usize, trace: &[Vec<u32>], i: usize) -> bool {
    let last = trace.len() - 1;
    let at = |id: NodeId, j: usize| reference(nodes, id.0 as usize, trace, j);
    match nodes[index].kind {
        NodeKind::False => false,
        NodeKind::True => true,
        NodeKind::Proposition { proposition } => trace[i].contains(&proposition.0),
        NodeKind::Not { operand } => !at(operand, i),
        NodeKind::And { left, right } => at(left, i) && at(right, i),
        NodeKind::Or { left, right } => at(left, i) || at(right, i),
        NodeKind::Implies { left, right } => !at(left, i) || at(right, i),
        NodeKind::Equivalent { left, right } => at(left, i) == at(right, i),
        NodeKind::StrongNext { operand } => i < last && at(operand, i + 1),
        NodeKind::WeakNext { operand } => i == last || at(operand, i + 1),
        NodeKind::LtlfFuture { operand } => (i..=last).any(|j| at(operand, j)),
        NodeKind::LtlfGlobally { operand } => (i..=last).all(|j| at(operand, j)),
        NodeKind::LtlfUntil { left, right } => {
            (i..=last).any(|j| at(right, j) && (i..j).all(|k| at(left, k)))
        }
        NodeKind::LtlfRelease { left, right } => {
            (i..=last).all(|j| at(right, j) || (i..j).any(|k| at(left, k)))
        }
        other => panic!("not an LTLf node: {other:?}"),
    }
}

fn ltlf_dir() -> PathBuf {
    PathBuf::from(CORPUS_DIR).join("ltlf")
}

fn ltlf_manifest() -> Value {
    serde_json::from_slice(&fs::read(ltlf_dir().join("manifest.json")).unwrap()).unwrap()
}

const RULE_NODES: [&str; 6] = [
    "strong-next",
    "weak-next",
    "future",
    "globally",
    "until",
    "release",
];
const SHAPES: [&str; 3] = ["length-1", "length-2-satisfying", "length-2-violating"];

/// The depth-one population: 4 leaves, 5 unary operators over each leaf,
/// 6 binary operators over each ordered pair of leaves.
fn depth_one_population() -> Vec<LtlfFormulaDocument> {
    let leaves = [
        NodeKind::True,
        NodeKind::False,
        NodeKind::Proposition {
            proposition: PropositionId(0),
        },
        NodeKind::Proposition {
            proposition: PropositionId(1),
        },
    ];
    let unary: [fn(NodeId) -> NodeKind; 5] = [
        |operand| NodeKind::Not { operand },
        |operand| NodeKind::LtlfFuture { operand },
        |operand| NodeKind::LtlfGlobally { operand },
        |operand| NodeKind::StrongNext { operand },
        |operand| NodeKind::WeakNext { operand },
    ];
    let binary: [fn(NodeId, NodeId) -> NodeKind; 6] = [
        |left, right| NodeKind::And { left, right },
        |left, right| NodeKind::Or { left, right },
        |left, right| NodeKind::Implies { left, right },
        |left, right| NodeKind::Equivalent { left, right },
        |left, right| NodeKind::LtlfUntil { left, right },
        |left, right| NodeKind::LtlfRelease { left, right },
    ];
    let mut population = Vec::new();
    let document = |nodes: Vec<Node>| {
        let root = NodeId(nodes.len() as u32 - 1);
        LtlfFormulaDocument::new(LTLF, root, nodes).unwrap()
    };
    for leaf in leaves {
        population.push(document(vec![Node::new(leaf)]));
    }
    for operator in unary {
        for leaf in leaves {
            population.push(document(vec![
                Node::new(leaf),
                Node::new(operator(NodeId(0))),
            ]));
        }
    }
    for operator in binary {
        for left in leaves {
            for right in leaves {
                population.push(document(vec![
                    Node::new(left),
                    Node::new(right),
                    Node::new(operator(NodeId(0), NodeId(1))),
                ]));
            }
        }
    }
    population
}

/// Integrity check of the LTLf manifest; every field is cross-checked.
fn check_ltlf_manifest(manifest: &Value) -> Result<(), String> {
    let rules = manifest["rule_cases"].as_array().ok_or("rule cases")?;
    if manifest["rule_case_count"].as_u64() != Some(rules.len() as u64) || rules.len() != 18 {
        return Err("rule case count".into());
    }
    let mut pairs = BTreeSet::new();
    for case in rules {
        let node = case["node"].as_str().ok_or("node")?;
        let shape = case["shape"].as_str().ok_or("shape")?;
        if !RULE_NODES.contains(&node) || !SHAPES.contains(&shape) || !pairs.insert((node, shape)) {
            return Err(format!("{node} {shape}: not a distinct node x shape pair"));
        }
        if case["formula"] != format!("formulas/{node}.json") {
            return Err(format!("{}: formula does not match node", case["id"]));
        }
        let bytes = fs::read(ltlf_dir().join(case["formula"].as_str().ok_or("formula")?))
            .map_err(|error| error.to_string())?;
        let document = LtlfFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default())
            .map_err(|error| error.to_string())?;
        let trace: Vec<Vec<u32>> =
            serde_json::from_value(case["trace"].clone()).map_err(|error| error.to_string())?;
        let length = if shape == "length-1" { 1 } else { 2 };
        if trace.len() != length {
            return Err(format!("{}: trace length differs from shape", case["id"]));
        }
        let verdict = reference(document.nodes(), document.root().0 as usize, &trace, 0);
        let satisfying = shape != "length-2-violating" || !verdict;
        if case["expected_at_0"].as_bool() != Some(verdict) || !satisfying {
            return Err(format!("{}: expected verdict differs from ADR-044", case["id"]));
        }
        if shape == "length-2-satisfying" && !verdict {
            return Err(format!("{}: satisfying trace does not satisfy", case["id"]));
        }
    }
    let counterexamples = manifest["counterexamples"].as_array().ok_or("counterexamples")?;
    if manifest["counterexample_entry_count"].as_u64() != Some(counterexamples.len() as u64)
        || counterexamples.len() != 4
    {
        return Err("counterexample entry count".into());
    }
    for (pair, number) in counterexamples.chunks(2).zip(1..) {
        let (ltlf, infinite) = (&pair[0], &pair[1]);
        if ltlf["reading"] != "ltlf" || infinite["reading"] != "infinite" {
            return Err("counterexample readings".into());
        }
        for (entry, reading) in [(ltlf, "ltlf"), (infinite, "infinite")] {
            if entry["id"] != format!("CE-{number}-{reading}")
                || entry["formula"] != format!("formulas/ce{number}-{reading}.json")
            {
                return Err(format!("{}: id or formula path mismatch", entry["id"]));
            }
        }
        if ltlf["expected"] == infinite["expected"] {
            return Err("a counterexample must separate the two readings".into());
        }
        let bytes = fs::read(ltlf_dir().join(ltlf["formula"].as_str().ok_or("formula")?))
            .map_err(|error| error.to_string())?;
        let document = LtlfFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default())
            .map_err(|error| error.to_string())?;
        for length in 1..=4 {
            let trace = vec![Vec::new(); length];
            let verdict = reference(document.nodes(), document.root().0 as usize, &trace, 0);
            if ltlf["expected"].as_bool() != Some(verdict) {
                return Err(format!("{}: LTLf verdict differs", ltlf["id"]));
            }
        }
        let bytes = fs::read(ltlf_dir().join(infinite["formula"].as_str().ok_or("formula")?))
            .map_err(|error| error.to_string())?;
        let unbounded =
            InfiniteFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default())
                .map_err(|error| error.to_string())?;
        // Constant formulas read no proposition, so every lasso agrees with the
        // one-state lasso, where every temporal operator returns its operand.
        let verdict = constant_lasso(&unbounded, unbounded.root().0 as usize)
            .ok_or("infinite counterexample is not a constant formula")?;
        if infinite["expected"].as_bool() != Some(verdict) {
            return Err(format!("{}: infinite verdict differs", infinite["id"]));
        }
    }
    let count = depth_one_population().len() as u64;
    if manifest["depth_one_enumerator"]["count"].as_u64() != Some(count) || count != 120 {
        return Err("enumerator count".into());
    }
    Ok(())
}

/// Test-only lasso reference for constant unbounded formulas: on the
/// one-state lasso F, G, U and R over any nonempty interval return the value
/// of their operand(s) (U and R: the right operand).
fn constant_lasso(document: &InfiniteFormulaDocument, index: usize) -> Option<bool> {
    let at = |id: NodeId| constant_lasso(document, id.0 as usize);
    Some(match document.nodes()[index].kind {
        InfiniteNodeKind::True => true,
        InfiniteNodeKind::False => false,
        InfiniteNodeKind::Not { operand } => !at(operand)?,
        InfiniteNodeKind::And { left, right } => at(left)? && at(right)?,
        InfiniteNodeKind::Or { left, right } => at(left)? || at(right)?,
        InfiniteNodeKind::Future { operand, .. } | InfiniteNodeKind::Globally { operand, .. } => {
            at(operand)?
        }
        _ => return None,
    })
}

/// L-6: the embedded LTLf schema accepts every corpus LTLf formula and
/// refuses interval-carrying, W/M and bare-next nodes, like the reader.
// Trace: TC-220, TC-224, FR-332-AC-1, FR-333-AC-2
#[test]
fn ltlf_schema_agrees_with_the_reader() {
    use jsonschema::{Draft, JSONSchema};
    let schema: Value = serde_json::from_str(FORMULA_LTLF_V1_SCHEMA).unwrap();
    let schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .unwrap();
    for node in RULE_NODES {
        let bytes = fs::read(ltlf_dir().join(format!("formulas/{node}.json"))).unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(schema.is_valid(&value), "{node}");
        assert!(LtlfFormulaDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default())
            .is_ok());
    }
    let p = r#"{"kind":"proposition","proposition":0}"#;
    for second in [
        r#"{"kind":"future","interval":{"kind":"closed","start":0,"end":3},"operand":0}"#,
        r#"{"kind":"weak_until","left":0,"right":0}"#,
        r#"{"kind":"strong_release","left":0,"right":0}"#,
        r#"{"kind":"next","operand":0}"#,
        r#"{"kind":"once","operand":0}"#,
    ] {
        let text = wire(&format!("{p},{second}"), 1);
        let value: Value = serde_json::from_str(&text).unwrap();
        assert!(!schema.is_valid(&value), "{second}");
        assert!(read(&text).is_err(), "{second}");
    }
}

/// TC-229: 18 rule cases, 4 counterexample entries, 120 enumerated formulas.
// Trace: TC-229, FR-335-AC-1
#[test]
fn ltlf_corpus_census() {
    let manifest = ltlf_manifest();
    assert_eq!(manifest["corpus"], "tl-syntax.ltlf-corpus/v1");
    assert_eq!(manifest["rule_cases"].as_array().unwrap().len(), 18);
    assert_eq!(manifest["counterexamples"].as_array().unwrap().len(), 4);
    let population = depth_one_population();
    assert_eq!(population.len(), 120);
    let distinct: BTreeSet<String> = population
        .iter()
        .map(|document| document.content_identity().unwrap())
        .collect();
    assert_eq!(distinct.len(), 120);
    check_ltlf_manifest(&manifest).unwrap();
}

/// TC-230: a one-field mutation of the manifest is detected.
// Trace: TC-230, FR-335-AC-2
#[test]
fn ltlf_manifest_mutations_are_detected() {
    let manifest = ltlf_manifest();
    let mutations: [fn(&mut Value); 9] = [
        |m| m["rule_cases"][0]["formula"] = Value::from("formulas/weak-next.json"),
        |m| m["rule_cases"][1]["trace"] = serde_json::json!([[], []]),
        |m| {
            let flipped = !m["rule_cases"][9]["expected_at_0"].as_bool().unwrap();
            m["rule_cases"][9]["expected_at_0"] = Value::from(flipped);
        },
        |m| m["rule_case_count"] = Value::from(17),
        |m| m["counterexample_entry_count"] = Value::from(3),
        |m| m["counterexamples"][1]["expected"] = Value::from(false),
        |m| m["depth_one_enumerator"]["count"] = Value::from(121),
        |m| m["counterexamples"][1]["formula"] = Value::from("formulas/ce2-infinite.json"),
        |m| {
            m["counterexamples"][0]["expected"] = Value::from(true);
            m["counterexamples"][1]["expected"] = Value::from(false);
        },
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut mutated = manifest.clone();
        mutate(&mut mutated);
        assert!(check_ltlf_manifest(&mutated).is_err(), "mutation {index} undetected");
    }
}

/// TC-231: LTLf graphs validate in the borrowed model and the existing node
/// and depth limits refuse one past.
// Trace: TC-231, NFR-048-AC-1
#[test]
fn ltlf_limits_match_the_existing_ones() {
    // Borrowed, allocation-free validation on a stack array.
    let nodes = [leaf(0), Node::new(NodeKind::WeakNext { operand: NodeId(0) })];
    assert_eq!(validate_ltlf(NodeId(1), &nodes).unwrap().profile(), LTLF);
    #[cfg(not(miri))]
    {
        let chain = |length: usize| {
            let mut nodes = vec![leaf(0)];
            for index in 0..length - 1 {
                nodes.push(Node::new(NodeKind::StrongNext {
                    operand: NodeId(index as u32),
                }));
            }
            nodes
        };
        let at_depth = chain(MAX_FORMULA_DOCUMENT_DEPTH);
        let root = NodeId(at_depth.len() as u32 - 1);
        assert!(LtlfFormulaDocument::new(LTLF, root, at_depth).is_ok());
        let past_depth = chain(MAX_FORMULA_DOCUMENT_DEPTH + 1);
        let root = NodeId(past_depth.len() as u32 - 1);
        assert!(matches!(
            LtlfFormulaDocument::new(LTLF, root, past_depth),
            Err(LtlfFormulaError::Graph(FormulaError::DocumentDepthLimitExceeded { .. }))
        ));
        let wide = vec![leaf(0); MAX_FORMULA_DOCUMENT_NODES];
        assert!(LtlfFormulaDocument::new(LTLF, NodeId(0), wide).is_ok());
        let wider = vec![leaf(0); MAX_FORMULA_DOCUMENT_NODES + 1];
        assert!(matches!(
            LtlfFormulaDocument::new(LTLF, NodeId(0), wider),
            Err(LtlfFormulaError::Graph(FormulaError::DocumentNodeLimitExceeded { .. }))
        ));
    }
    assert!(matches!(
        validate_ltlf(NodeId(5), &nodes),
        Err(LtlfFormulaError::Graph(FormulaError::RootOutOfRange { .. }))
    ));
}
