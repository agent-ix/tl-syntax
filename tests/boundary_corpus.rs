#![cfg(feature = "serde")]
//! The hand-authored boundary corpus (`corpus/boundary-v1`): every formula is a
//! valid graph under its profile and every operator x interval-shape cell keeps
//! a positive, a negative and a boundary case. Evaluation is not done here;
//! evaluators replay the stored verdicts.

use std::{collections::BTreeSet, fs, path::Path};

use serde_json::Value;
use tl_syntax::{Formula, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile};

const CLOSED: &str = "mltl.closed-trace/v1";
const PAST: &str = "mltl.origin-complete-history/v1";
const PREFIX: &str = "mltl.online-prefix/v1";

fn push(nodes: &mut Vec<Node>, kind: NodeKind) -> NodeId {
    nodes.push(Node::new(kind));
    NodeId((nodes.len() - 1) as u32)
}

fn interval(v: &Value) -> Interval {
    let a = v["interval"].as_array().expect("interval pair");
    Interval::new(a[0].as_u64().unwrap() as u32, a[1].as_u64().unwrap() as u32)
        .expect("interval is ordered")
}

fn lower(v: &Value, nodes: &mut Vec<Node>) -> NodeId {
    let op = v["op"].as_str().expect("op");
    match op {
        "false" => return push(nodes, NodeKind::False),
        "true" => return push(nodes, NodeKind::True),
        "prop" => {
            let id = v["id"].as_u64().unwrap() as u32;
            return push(
                nodes,
                NodeKind::Proposition {
                    proposition: PropositionId(id),
                },
            );
        }
        _ => {}
    }
    if v.get("arg").is_some() {
        let operand = lower(&v["arg"], nodes);
        let kind = match op {
            "not" => NodeKind::Not { operand },
            "yesterday" => NodeKind::StrongPrevious { operand },
            "future" => NodeKind::Future {
                interval: interval(v),
                operand,
            },
            "globally" => NodeKind::Globally {
                interval: interval(v),
                operand,
            },
            "once" => NodeKind::Once {
                interval: interval(v),
                operand,
            },
            "historically" => NodeKind::Historically {
                interval: interval(v),
                operand,
            },
            other => panic!("unknown unary operator `{other}`"),
        };
        return push(nodes, kind);
    }
    let left = lower(&v["left"], nodes);
    let right = lower(&v["right"], nodes);
    let kind = match op {
        "and" => NodeKind::And { left, right },
        "or" => NodeKind::Or { left, right },
        "implies" => NodeKind::Implies { left, right },
        "equivalent" => NodeKind::Equivalent { left, right },
        "until" => NodeKind::Until {
            interval: interval(v),
            left,
            right,
        },
        "release" => NodeKind::Release {
            interval: interval(v),
            left,
            right,
        },
        "since" => NodeKind::Since {
            interval: interval(v),
            left,
            right,
        },
        "triggered" => NodeKind::Triggered {
            interval: interval(v),
            left,
            right,
        },
        other => panic!("unknown binary operator `{other}`"),
    };
    push(nodes, kind)
}

fn cases() -> Vec<Value> {
    let path = Path::new(tl_syntax::CORPUS_DIR).join("boundary-v1/cases.json");
    let doc: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    assert_eq!(doc["schema"], "tl-mltl.boundary-corpus/v1");
    doc["cases"].as_array().unwrap().clone()
}

#[test]
fn every_case_is_a_valid_graph_under_its_profile() {
    let all = cases();
    let mut ids = BTreeSet::new();
    for c in &all {
        let id = c["id"].as_str().unwrap();
        assert!(ids.insert(id.to_owned()), "duplicate id {id}");
        let profile = match c["profile"].as_str().unwrap() {
            CLOSED => SemanticProfile::ClosedTraceV1,
            PAST => SemanticProfile::OriginCompleteHistoryV1,
            PREFIX => SemanticProfile::OnlinePrefixV1,
            other => panic!("{id}: unknown profile {other}"),
        };
        let mut nodes = Vec::new();
        let root = lower(&c["formula"], &mut nodes);
        Formula::new(profile, root, &nodes).unwrap_or_else(|e| panic!("{id}: {e:?}"));
        let expected = &c["expected"];
        assert!(
            expected.is_boolean() || matches!(expected.as_str(), Some("pending" | "refused")),
            "{id}: bad expected {expected}"
        );
        assert!(c["trace"].is_array() && c["at"].is_u64(), "{id}");
    }
}

#[test]
fn every_operator_cell_has_positive_negative_and_boundary_cases() {
    let all = cases();
    let has = |profile: &str, op: &str, shape: &str, kind: &str| {
        all.iter().any(|c| {
            c["profile"] == profile
                && c["operator"] == op
                && c["shape"] == shape
                && c["kind"] == kind
        })
    };
    let shapes = ["point-zero", "from-zero", "point", "interior"];
    for (profile, ops) in [
        (CLOSED, ["future", "globally", "until", "release"]),
        (PAST, ["once", "historically", "since", "triggered"]),
    ] {
        for op in ops {
            for shape in shapes {
                for kind in ["positive", "negative", "boundary"] {
                    assert!(
                        has(profile, op, shape, kind),
                        "{profile} {op} {shape} {kind}"
                    );
                }
            }
            assert!(has(profile, op, "max", "boundary"), "{profile} {op} max");
        }
    }
    let edges: BTreeSet<&str> = all
        .iter()
        .flat_map(|c| {
            c["edges"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(Value::as_str)
        })
        .collect();
    for edge in [
        "a=b",
        "a=0",
        "b=trace-end",
        "empty-trace",
        "horizon=prefix",
        "integer-extreme",
        "nested-just-fits",
        "window-before-origin",
    ] {
        assert!(edges.contains(edge), "no case exercises `{edge}`");
    }
}
