//! P-TL-150 assume-guarantee contracts (TL-425 syntax half): FR-309..FR-311,
//! with ADR-017 option C as decided on 2026-11-23.
#![cfg(feature = "serde")]

use std::{collections::BTreeSet, fs, path::PathBuf};

use proptest::prelude::*;
use serde_json::Value;
use tl_syntax::{
    assumption_prefix_verdict, check_assumption_inputs, contract_outcome,
    lower_contract_obligation, settled_positions, AssumptionPrefixVerdict, Contract, ContractAssumptionError, ContractGraph,
    ContractObligationError, ContractOutcome, ContractSetDocument, ContractSetError,
    ContractSetReadError, ContractStatus, ContractTruth, FormulaBindingError, FormulaDocument,
    InfiniteClock, InfiniteFormulaDocument, InfiniteNode, InfiniteNodeKind, Interval, Node,
    NodeId, NodeKind, OwnedSignalDeclaration, PropositionBinding, PropositionId,
    SemanticProfile, SignalCatalogDocument, SignalDirection, SignalDomain, SignalId,
    SyntaxArtifactLimits, CONTRACT_SET_V1, CONTRACT_SET_V1_SCHEMA_BYTES, CORPUS_DIR,
    MAX_CONTRACTS, MAX_FORMULA_DOCUMENT_NODES,
};

fn proposition(id: u32) -> Node {
    Node::new(NodeKind::Proposition {
        proposition: PropositionId(id),
    })
}

/// p1, p2, F[0,2] p1, G[0,1] p2, and.
fn bounded_nodes() -> Vec<Node> {
    vec![
        proposition(1),
        proposition(2),
        Node::new(NodeKind::Future {
            interval: Interval::new(0, 2).unwrap(),
            operand: NodeId(0),
        }),
        Node::new(NodeKind::Globally {
            interval: Interval::new(0, 1).unwrap(),
            operand: NodeId(1),
        }),
        Node::new(NodeKind::And {
            left: NodeId(2),
            right: NodeId(3),
        }),
    ]
}

fn contract(id: u32, name: &str, assumption: u32, guarantee: u32) -> Contract {
    Contract {
        id,
        name: name.to_owned(),
        assumption: NodeId(assumption),
        guarantee: NodeId(guarantee),
    }
}

fn shared_roots() -> Vec<Contract> {
    vec![
        contract(1, "door-alarm", 2, 3),
        contract(2, "same-root", 2, 2),
        contract(3, "reuse", 2, 3),
    ]
}

/// TC-1522: empty and shared-root sets round-trip byte for byte with exact
/// graph, profile and clock identities over v1, v2 and unbounded editions.
// Trace: TC-1522, FR-309-AC-1
#[test]
fn contract_sets_round_trip_over_every_edition() {
    let v1 = FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes()).unwrap();
    let v2 = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes()).unwrap();
    let unbounded = InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(2),
        vec![
            InfiniteNode::new(InfiniteNodeKind::Proposition {
                proposition: PropositionId(1),
            }),
            InfiniteNode::new(InfiniteNodeKind::Proposition {
                proposition: PropositionId(2),
            }),
            InfiniteNode::new(InfiniteNodeKind::Implies {
                left: NodeId(0),
                right: NodeId(1),
            }),
        ],
    )
    .unwrap();
    let graphs = [
        ContractGraph::Bounded(&v1),
        ContractGraph::Bounded(&v2),
        ContractGraph::Unbounded(&unbounded),
    ];
    for graph in graphs {
        let identity = graph.content_identity().unwrap();
        for contracts in [vec![], shared_roots()] {
            let contracts: Vec<Contract> = contracts
                .into_iter()
                .filter(|contract| (contract.guarantee.0 as usize) < graph.node_count())
                .collect();
            let document = ContractSetDocument::new(graph, identity.clone(), contracts).unwrap();
            assert_eq!(document.graph_identity(), identity);
            assert_eq!(document.semantic_profile(), graph.semantic_profile());
            assert_eq!(document.clock(), graph.clock());
            let bytes = document.canonical_json_bytes().unwrap();
            let reread =
                ContractSetDocument::from_json_bytes(&bytes, SyntaxArtifactLimits::default(), graph)
                    .unwrap();
            assert_eq!(reread, document);
            assert_eq!(reread.canonical_json_bytes().unwrap(), bytes);
            let text = String::from_utf8(bytes).unwrap();
            assert!(text.starts_with(&format!("{{\"schema_version\":\"{CONTRACT_SET_V1}\"")));
            assert_eq!(text.contains("\"clock\":\"event_position\""), graph.clock().is_some());
        }
    }
    assert_eq!(
        CONTRACT_SET_V1_SCHEMA_BYTES,
        include_bytes!("../spec/contract-set-v1.schema.json")
    );
}

/// TC-1523: every contract-set error is distinct and yields no document.
// Trace: TC-1523, FR-309-AC-2
#[test]
fn contract_set_errors_are_distinct() {
    let v2 = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes()).unwrap();
    let graph = ContractGraph::Bounded(&v2);
    let identity = graph.content_identity().unwrap();
    let new = |contracts: Vec<Contract>| ContractSetDocument::new(graph, identity.clone(), contracts);
    assert_eq!(
        new(vec![contract(1, "a", 9, 3)]),
        Err(ContractSetError::RootOutOfRange {
            contract: 1,
            root: NodeId(9)
        })
    );
    assert_eq!(
        new(vec![contract(1, "a", 2, 9)]),
        Err(ContractSetError::RootOutOfRange {
            contract: 1,
            root: NodeId(9)
        })
    );
    assert_eq!(
        new(vec![contract(2, "a", 2, 3), contract(2, "b", 2, 3)]),
        Err(ContractSetError::ContractIdNotIncreasing {
            previous: 2,
            current: 2
        })
    );
    assert_eq!(
        new(vec![contract(3, "a", 2, 3), contract(2, "b", 2, 3)]),
        Err(ContractSetError::ContractIdNotIncreasing {
            previous: 3,
            current: 2
        })
    );
    assert_eq!(
        new(vec![contract(1, "a", 2, 3), contract(2, "a", 2, 3)]),
        Err(ContractSetError::DuplicateName {
            first: 1,
            second: 2
        })
    );
    assert_eq!(
        new(vec![contract(1, "", 2, 3)]),
        Err(ContractSetError::EmptyName { contract: 1 })
    );
    assert_eq!(
        new(vec![contract(1, &"x".repeat(256), 2, 3)]),
        Err(ContractSetError::NameTooLong { contract: 1 })
    );
    #[cfg(not(miri))]
    {
        let many: Vec<Contract> = (0..=MAX_CONTRACTS as u32)
            .map(|id| contract(id, &format!("c{id}"), 2, 3))
            .collect();
        assert_eq!(
            new(many[..MAX_CONTRACTS].to_vec()).map(|_| ()),
            Ok(())
        );
        assert_eq!(
            new(many),
            Err(ContractSetError::ContractLimitExceeded {
                count: MAX_CONTRACTS + 1,
                limit: MAX_CONTRACTS
            })
        );
    }
    assert_eq!(
        ContractSetDocument::new(graph, "0".repeat(64), vec![]),
        Err(ContractSetError::GraphIdentityMismatch)
    );
    assert_eq!(
        ContractSetDocument::new(graph, String::new(), vec![]),
        Err(ContractSetError::GraphIdentityMismatch)
    );
    assert_eq!(
        ContractSetDocument::bind(graph, SemanticProfile::OnlinePrefixV1, None, identity.clone(), vec![]),
        Err(ContractSetError::ProfileMismatch)
    );
    assert_eq!(
        ContractSetDocument::bind(
            graph,
            SemanticProfile::ClosedTraceV1,
            Some(InfiniteClock::EventPosition),
            identity.clone(),
            vec![]
        ),
        Err(ContractSetError::ClockMismatch)
    );
    // A set bound to one graph is refused for another of the same shape.
    let document = new(shared_roots()).unwrap();
    let mut other_nodes = bounded_nodes();
    other_nodes[0] = proposition(7);
    let other = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), other_nodes).unwrap();
    assert!(matches!(
        ContractSetDocument::from_json_bytes(
            &document.canonical_json_bytes().unwrap(),
            SyntaxArtifactLimits::default(),
            ContractGraph::Bounded(&other)
        ),
        Err(ContractSetReadError::Binding(ContractSetError::GraphIdentityMismatch))
    ));
}

/// TC-1524: the formula graph and its digests are unchanged by a contract set.
// Trace: TC-1524, FR-309-AC-3
#[test]
fn contract_sets_leave_the_graph_unchanged() {
    let v2 = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes()).unwrap();
    let before = v2.canonical_json_bytes().unwrap();
    let identity = v2.content_identity().unwrap();
    let _ = ContractSetDocument::new(ContractGraph::Bounded(&v2), identity.clone(), shared_roots()).unwrap();
    assert_eq!(v2.canonical_json_bytes().unwrap(), before);
    assert_eq!(v2.content_identity().unwrap(), identity);
    assert_eq!(
        tl_syntax::FORMULA_V2_SCHEMA_SHA256,
        "a78889a4ed04dabd271bfd68953a8ddf642b14e8609d2268421b23fa4540c7c4"
    );
    assert_eq!(
        tl_syntax::FORMULA_V1_SCHEMA_SHA256,
        "996b5fef4690df59a7b0c655f95cfbdb2e56a9db534e0db58859bed9666389ce"
    );
}

/// TC-1525: all nine (a, g) combinations and their spellings.
// Trace: TC-1525, FR-310-AC-1, StR-016-VC-1
#[test]
fn status_truth_table_and_spellings() {
    use ContractTruth::{False, Pending, True};
    let inactive = ContractOutcome::Status(ContractStatus::Inactive);
    let table = [
        (False, False, inactive, "inactive"),
        (False, True, inactive, "inactive"),
        (False, Pending, inactive, "inactive"),
        (True, False, ContractOutcome::Status(ContractStatus::Invalid), "invalid"),
        (True, True, ContractOutcome::Status(ContractStatus::Verified), "verified"),
        (True, Pending, ContractOutcome::Pending, "pending"),
        (Pending, False, ContractOutcome::Pending, "pending"),
        (Pending, True, ContractOutcome::Pending, "pending"),
        (Pending, Pending, ContractOutcome::Pending, "pending"),
    ];
    for (assumption, guarantee, outcome, spelling) in table {
        assert_eq!(contract_outcome(assumption, guarantee), outcome);
        assert_eq!(outcome.as_str(), spelling);
    }
}

proptest! {
    /// TC-1526: refining a pending input never changes a decided outcome.
    // Trace: TC-1526, FR-310-AC-2
    #[test]
    fn status_function_is_monotone(
        a in 0_usize..3, g in 0_usize..3, refined_a in 0_usize..3, refined_g in 0_usize..3,
    ) {
        let (a, g) = (ContractTruth::ALL[a], ContractTruth::ALL[g]);
        let (refined_a, refined_g) = (ContractTruth::ALL[refined_a], ContractTruth::ALL[refined_g]);
        prop_assume!(refined_a.refines(a) && refined_g.refines(g));
        let coarse = contract_outcome(a, g);
        if coarse != ContractOutcome::Pending {
            prop_assert_eq!(contract_outcome(refined_a, refined_g), coarse);
        }
    }
}

/// TC-1527: lowering appends one Implies, keeps every node, refuses at the
/// limit.
// Trace: TC-1527, FR-310-AC-3
#[test]
fn obligation_lowering_appends_one_implies() {
    let mut nodes = bounded_nodes();
    let before = nodes.clone();
    let id = lower_contract_obligation(&mut nodes, NodeId(2), NodeId(3)).unwrap();
    assert_eq!(id, NodeId(5));
    assert_eq!(nodes.len(), before.len() + 1);
    assert_eq!(&nodes[..5], &before[..]);
    assert_eq!(
        nodes[5].kind,
        NodeKind::Implies {
            left: NodeId(2),
            right: NodeId(3)
        }
    );
    assert_eq!(
        lower_contract_obligation(&mut nodes, NodeId(9), NodeId(3)),
        Err(ContractObligationError::RootOutOfRange { root: NodeId(9) })
    );
    #[cfg(not(miri))]
    {
        let mut full = vec![Node::new(NodeKind::True); MAX_FORMULA_DOCUMENT_NODES];
        assert_eq!(
            lower_contract_obligation(&mut full, NodeId(0), NodeId(1)),
            Err(ContractObligationError::NodeLimit {
                limit: MAX_FORMULA_DOCUMENT_NODES
            })
        );
        assert_eq!(full.len(), MAX_FORMULA_DOCUMENT_NODES);
        full.pop();
        assert!(lower_contract_obligation(&mut full, NodeId(0), NodeId(1)).is_ok());
    }
}

fn directed_catalog() -> SignalCatalogDocument {
    SignalCatalogDocument::new_v2(
        vec![
            OwnedSignalDeclaration::new(SignalId(1), "door".into(), SignalDomain::Boolean)
                .with_direction(SignalDirection::Input),
            OwnedSignalDeclaration::new(SignalId(2), "alarm".into(), SignalDomain::Boolean)
                .with_direction(SignalDirection::Output),
            OwnedSignalDeclaration::new(SignalId(3), "spare".into(), SignalDomain::Boolean),
        ],
        vec![
            PropositionBinding::new(PropositionId(1), SignalId(1)),
            PropositionBinding::new(PropositionId(2), SignalId(2)),
            PropositionBinding::new(PropositionId(3), SignalId(3)),
        ],
    )
    .unwrap()
}

fn shared_roots_set(v2: &FormulaDocument) -> ContractSetDocument {
    ContractSetDocument::new(
        ContractGraph::Bounded(v2),
        v2.content_identity().unwrap(),
        shared_roots(),
    )
    .unwrap()
}

fn keys(value: &Value) -> BTreeSet<String> {
    value.as_object().unwrap().keys().cloned().collect()
}

/// TC-1528: the view exposes ids, names, roots and the obligation, and the
/// wire form has no member beyond them (no scope or realizability field).
// Trace: TC-1528, FR-311-AC-1
#[test]
fn contract_view_exposes_roots_and_obligation_only() {
    let v2 = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes())
        .unwrap();
    let document = shared_roots_set(&v2);
    let view = document.view();
    assert_eq!(view.contracts().len(), 3);
    let first = view.contract(1).unwrap();
    assert_eq!(
        (first.name.as_str(), first.assumption, first.guarantee),
        ("door-alarm", NodeId(2), NodeId(3))
    );
    assert!(view.contract(9).is_none());
    let (id, node) = view.obligation(2, v2.nodes().len()).unwrap().unwrap();
    assert_eq!(id, NodeId(5));
    assert_eq!(
        node.kind,
        NodeKind::Implies {
            left: NodeId(2),
            right: NodeId(2)
        }
    );
    let wire: Value = serde_json::from_slice(&document.canonical_json_bytes().unwrap()).unwrap();
    let top: BTreeSet<String> = ["schema_version", "semantic_profile", "graph_identity", "contracts"]
        .map(String::from)
        .into();
    assert_eq!(keys(&wire), top);
    let member: BTreeSet<String> = ["id", "name", "assumption", "guarantee"].map(String::from).into();
    for contract in wire["contracts"].as_array().unwrap() {
        assert_eq!(keys(contract), member);
    }
}

/// Option C with the settled-positions rule (decision 1; tl-mltl ADR-004
/// decision 5). SPEC-GAP: no tl-syntax matrix row covers this reading; the
/// consuming rows are tl-solve FR-100's, so it is traced to FR-311 only.
// Trace: FR-311-AC-1
#[test]
fn settled_positions_decide_the_option_c_verdict() {
    let verified = ContractOutcome::Status(ContractStatus::Verified);
    let inactive = ContractOutcome::Status(ContractStatus::Inactive);
    let invalid = ContractOutcome::Status(ContractStatus::Invalid);
    let pending = ContractOutcome::Pending;
    // Lookahead 2 over a 5-position play: positions 0..=2 are settled.
    assert_eq!(settled_positions(5, 2), 3);
    assert_eq!(settled_positions(1, 2), 0);
    assert_eq!(assumption_prefix_verdict(&[], 0), AssumptionPrefixVerdict::Holds);
    // A winning play: its unsettled tail is pending and decides nothing.
    assert_eq!(
        assumption_prefix_verdict(&[verified, verified, verified, pending, pending], 3),
        AssumptionPrefixVerdict::Holds
    );
    // An invalid in the unsettled tail does not decide either.
    assert_eq!(
        assumption_prefix_verdict(&[verified, verified, verified, invalid, pending], 3),
        AssumptionPrefixVerdict::Holds
    );
    // An earlier inactive position ends the obligation.
    assert_eq!(
        assumption_prefix_verdict(&[verified, inactive, invalid, pending, pending], 3),
        AssumptionPrefixVerdict::Holds
    );
    assert_eq!(
        assumption_prefix_verdict(&[verified, invalid, inactive, pending, pending], 3),
        AssumptionPrefixVerdict::Violated { position: 1 }
    );
    // Pending at a settled position is a tool fault, never a verdict.
    assert_eq!(
        assumption_prefix_verdict(&[verified, pending, invalid, pending, pending], 3),
        AssumptionPrefixVerdict::Fault { position: 1 }
    );
}

/// Assumptions must read Input signals only (decision 1); traced like the
/// test above (SPEC-GAP: the consuming row is tl-solve FR-100's).
// Trace: FR-311-AC-1
#[test]
fn assumptions_must_read_input_signals_only() {
    let v2 = FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(4), bounded_nodes())
        .unwrap();
    let catalog_document = directed_catalog();
    let catalog = catalog_document.validate().unwrap();
    let formula = v2.validate().unwrap();
    assert_eq!(check_assumption_inputs(catalog, formula, NodeId(2)), Ok(()));
    assert_eq!(
        check_assumption_inputs(catalog, formula, NodeId(3)),
        Err(ContractAssumptionError::NotInput {
            proposition: PropositionId(2),
            direction: SignalDirection::Output
        })
    );
    let undeclared =
        FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(0), vec![proposition(3)])
            .unwrap();
    assert_eq!(
        check_assumption_inputs(catalog, undeclared.validate().unwrap(), NodeId(0)),
        Err(ContractAssumptionError::NotInput {
            proposition: PropositionId(3),
            direction: SignalDirection::Undeclared
        })
    );
    let unbound =
        FormulaDocument::new_v2(SemanticProfile::ClosedTraceV1, NodeId(0), vec![proposition(8)])
            .unwrap();
    assert_eq!(
        check_assumption_inputs(catalog, unbound.validate().unwrap(), NodeId(0)),
        Err(ContractAssumptionError::Binding(
            FormulaBindingError::MissingPropositionBinding {
                proposition: PropositionId(8)
            }
        ))
    );
    assert_eq!(
        check_assumption_inputs(catalog, formula, NodeId(9)),
        Err(ContractAssumptionError::RootOutOfRange { root: NodeId(9) })
    );
}

/// FR-305 contract-set corpus cases yield their declared outcome.
// Trace: TC-1512, TC-1522, TC-1523, FR-305-AC-2, FR-309-AC-1, FR-309-AC-2
#[test]
fn contract_corpus_cases_yield_declared_outcomes() {
    let root = PathBuf::from(CORPUS_DIR).join("numeric");
    let manifest: Value =
        serde_json::from_slice(&fs::read(root.join("manifest.json")).unwrap()).unwrap();
    let mut checked = 0;
    for case in manifest["cases"].as_array().unwrap() {
        if case["class"] != "contract-set" {
            continue;
        }
        let formula = FormulaDocument::from_json_bytes(
            &fs::read(root.join(case["formula"].as_str().unwrap())).unwrap(),
            SyntaxArtifactLimits::default(),
        )
        .unwrap();
        let bytes = fs::read(root.join(case["document"].as_str().unwrap())).unwrap();
        let result = ContractSetDocument::from_json_bytes(
            &bytes,
            SyntaxArtifactLimits::default(),
            ContractGraph::Bounded(&formula),
        );
        let observed = match result {
            Ok(_) => "valid",
            Err(ContractSetReadError::Binding(error)) => match error {
                ContractSetError::RootOutOfRange { .. } => "RootOutOfRange",
                ContractSetError::DuplicateName { .. } => "DuplicateName",
                ContractSetError::ContractIdNotIncreasing { .. } => "ContractIdNotIncreasing",
                ContractSetError::EmptyName { .. } => "EmptyName",
                ContractSetError::GraphIdentityMismatch => "GraphIdentityMismatch",
                ContractSetError::ProfileMismatch => "ProfileMismatch",
                other => panic!("{other:?}"),
            },
            Err(other) => panic!("{}: {other:?}", case["id"]),
        };
        assert_eq!(observed, case["outcome"], "{}", case["id"]);
        checked += 1;
    }
    assert_eq!(checked, 8);
}
