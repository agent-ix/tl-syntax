//! TL-227: generated round-trip and canonical-form properties for every owner
//! wire document (formula v1/v2, signal catalog, proposition map).
//!
//! Every property runs under a fixed ChaCha seed and a fixed accepted-case
//! count so a run is reproducible byte for byte. The `CAMPAIGN` table is the
//! record of those seeds and counts. A property that discards cases is a
//! failure of the generator, so no strategy here filters.

#![cfg(feature = "serde")]

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};
use tl_syntax::{
    FormulaDocument, FormulaSchemaVersion, Interval, Node, NodeId, NodeKind,
    OwnedSignalDeclaration, PropositionBinding, PropositionEntry, PropositionId,
    PropositionMapDocument, SemanticProfile, SignalCatalogDocument, SignalDomain, SignalId,
    StrictDocumentReadError, SyntaxArtifactLimits,
};

/// (property, seed byte, accepted cases). The 32-byte ChaCha seed repeats the
/// seed byte.
const CAMPAIGN: &[(&str, u8, u32)] = &[
    ("formula_round_trip", 0x11, 2000),
    ("formula_version_conversion", 0x12, 1000),
    ("catalog_round_trip", 0x13, 2000),
    ("map_round_trip", 0x14, 2000),
    ("formula_noncanonical", 0x15, 1000),
    ("catalog_noncanonical", 0x16, 1000),
    ("map_noncanonical", 0x17, 1000),
    ("map_semantic_faults", 0x18, 1000),
    ("catalog_semantic_faults", 0x19, 1000),
];

/// Cases per property under Miri, which interprets every instruction. The recorded
/// campaign above runs in full natively.
const MIRI_CASES: u32 = 4;

fn run<S: Strategy>(name: &str, strategy: S, test: impl Fn(S::Value) -> Result<(), TestCaseError>) {
    let &(_, seed, cases) = CAMPAIGN
        .iter()
        .find(|row| row.0 == name)
        .expect("property is recorded in CAMPAIGN");
    let cases = if cfg!(miri) { MIRI_CASES } else { cases };
    let config = Config {
        cases,
        failure_persistence: None,
        max_global_rejects: 0,
        max_local_rejects: 0,
        ..Config::default()
    };
    let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &[seed; 32]);
    let mut runner = TestRunner::new_with_rng(config, rng);
    if let Err(failure) = runner.run(&strategy, test) {
        panic!("{name} (seed byte {seed:#04x}, {cases} cases): {failure}");
    }
}

fn limits() -> SyntaxArtifactLimits {
    SyntaxArtifactLimits::default()
}

// ---------------------------------------------------------------- formulas

#[derive(Clone, Debug)]
struct NodeChoice {
    shape: u8,
    a: u32,
    b: u32,
    start: u32,
    width: u32,
    proposition: u32,
}

fn node_choice() -> impl Strategy<Value = NodeChoice> {
    (
        any::<u8>(),
        any::<u32>(),
        any::<u32>(),
        0_u32..=1_000_000,
        0_u32..=1_000,
        any::<u32>(),
    )
        .prop_map(|(shape, a, b, start, width, proposition)| NodeChoice {
            shape,
            a,
            b,
            start,
            width,
            proposition,
        })
}

#[derive(Clone, Copy, Debug)]
enum Flavor {
    V1Future(SemanticProfile),
    V2Future(SemanticProfile),
    V2Past,
}

fn flavor() -> impl Strategy<Value = Flavor> {
    prop_oneof![
        Just(Flavor::V1Future(SemanticProfile::ClosedTraceV1)),
        Just(Flavor::V1Future(SemanticProfile::OnlinePrefixV1)),
        Just(Flavor::V2Future(SemanticProfile::ClosedTraceV1)),
        Just(Flavor::V2Future(SemanticProfile::OnlinePrefixV1)),
        Just(Flavor::V2Past),
    ]
}

/// Builds a node that references only earlier nodes, so every generated
/// graph is valid for its flavor by construction.
fn build_node(flavor: Flavor, index: usize, choice: &NodeChoice) -> Node {
    let pick = |value: u32| NodeId(value % index as u32);
    let interval = Interval::new(choice.start, choice.start + choice.width).unwrap();
    let past = matches!(flavor, Flavor::V2Past);
    let kind = if index == 0 {
        match choice.shape % 3 {
            0 => NodeKind::False,
            1 => NodeKind::True,
            _ => NodeKind::Proposition {
                proposition: PropositionId(choice.proposition),
            },
        }
    } else {
        let (left, right) = (pick(choice.a), pick(choice.b));
        match choice.shape % 13 {
            0 => NodeKind::False,
            1 => NodeKind::True,
            2 => NodeKind::Proposition {
                proposition: PropositionId(choice.proposition),
            },
            3 => NodeKind::Not { operand: left },
            4 => NodeKind::And { left, right },
            5 => NodeKind::Or { left, right },
            6 => NodeKind::Implies { left, right },
            7 => NodeKind::Equivalent { left, right },
            8 if past => NodeKind::Once {
                interval,
                operand: left,
            },
            8 => NodeKind::Future {
                interval,
                operand: left,
            },
            9 if past => NodeKind::Historically {
                interval,
                operand: left,
            },
            9 => NodeKind::Globally {
                interval,
                operand: left,
            },
            10 if past => NodeKind::Since {
                interval,
                left,
                right,
            },
            10 => NodeKind::Until {
                interval,
                left,
                right,
            },
            11 if past => NodeKind::Triggered {
                interval,
                left,
                right,
            },
            11 => NodeKind::Release {
                interval,
                left,
                right,
            },
            _ if past => NodeKind::StrongPrevious { operand: left },
            _ => NodeKind::Not { operand: right },
        }
    };
    Node::new(kind)
}

fn formula_document() -> impl Strategy<Value = FormulaDocument> {
    (flavor(), prop::collection::vec(node_choice(), 1..=24)).prop_map(|(flavor, choices)| {
        let nodes: Vec<Node> = choices
            .iter()
            .enumerate()
            .map(|(index, choice)| build_node(flavor, index, choice))
            .collect();
        let root = NodeId(nodes.len() as u32 - 1);
        match flavor {
            Flavor::V1Future(profile) => FormulaDocument::new(profile, root, nodes),
            Flavor::V2Future(profile) => FormulaDocument::new_v2(profile, root, nodes),
            Flavor::V2Past => {
                FormulaDocument::new_v2(SemanticProfile::OriginCompleteHistoryV1, root, nodes)
            }
        }
        .expect("generator builds only valid graphs")
    })
}

// Trace: TC-075, FR-014-AC-1
// spec-correctness: row=FR-014-AC-1 property=universal extraction=extractable origin=regex
#[test]
fn formula_documents_round_trip_to_equal_values_and_identical_bytes() {
    run("formula_round_trip", formula_document(), |document| {
        let bytes = document.canonical_json_bytes().unwrap();
        let decoded = FormulaDocument::from_json_bytes(&bytes, limits())
            .map_err(|error| TestCaseError::fail(format!("{error}")))?;
        prop_assert_eq!(&decoded, &document);
        // Equal values have equal bytes and equal identities.
        prop_assert_eq!(decoded.canonical_json_bytes().unwrap(), bytes.clone());
        prop_assert_eq!(
            decoded.content_identity().unwrap(),
            document.content_identity().unwrap()
        );
        // The generic serde route agrees with the strict reader.
        let via_serde: FormulaDocument = serde_json::from_slice(&bytes).unwrap();
        prop_assert_eq!(&via_serde, &document);
        Ok(())
    });
}

// Trace: TC-075, FR-014-AC-1
// spec-correctness: row=FR-014-AC-1 property=universal extraction=extractable origin=regex
#[test]
fn formula_v1_v2_conversion_is_lossless_exactly_when_v1_can_carry_the_graph() {
    run(
        "formula_version_conversion",
        formula_document(),
        |document| {
            let v2 = document.to_v2();
            prop_assert_eq!(v2.schema_version(), FormulaSchemaVersion::V2);
            prop_assert_eq!(v2.semantic_profile(), document.semantic_profile());
            prop_assert_eq!(v2.root(), document.root());
            prop_assert_eq!(v2.nodes(), document.nodes());
            let v2_bytes = v2.canonical_json_bytes().unwrap();
            prop_assert_eq!(
                &FormulaDocument::from_json_bytes(&v2_bytes, limits()).unwrap(),
                &v2
            );
            // v1 can carry the graph exactly when no past node or history
            // profile is present.
            let v1_carries = document.semantic_profile()
                != SemanticProfile::OriginCompleteHistoryV1
                && document
                    .nodes()
                    .iter()
                    .all(|node| node.kind.past_operator().is_none());
            match document.try_to_v1() {
                Ok(v1) => {
                    prop_assert!(v1_carries);
                    prop_assert_eq!(v1.schema_version(), FormulaSchemaVersion::V1);
                    prop_assert_eq!(v1.to_v2(), v2.clone());
                    let v1_bytes = v1.canonical_json_bytes().unwrap();
                    prop_assert_eq!(
                        &FormulaDocument::from_json_bytes(&v1_bytes, limits()).unwrap(),
                        &v1
                    );
                    // The same graph has different bytes and identities per edition.
                    prop_assert_ne!(v1_bytes, v2_bytes);
                    prop_assert_ne!(
                        v1.content_identity().unwrap(),
                        v2.content_identity().unwrap()
                    );
                }
                Err(_) => prop_assert!(!v1_carries),
            }
            Ok(())
        },
    );
}

// --------------------------------------------------------- signal catalogs

fn signal_domain() -> impl Strategy<Value = SignalDomain> {
    prop_oneof![
        Just(SignalDomain::Boolean),
        (any::<i64>(), any::<i64>()).prop_map(|(a, b)| SignalDomain::Integer(
            tl_syntax::IntegerSignalDomain::new(a.min(b), a.max(b)).unwrap()
        )),
        (any::<i64>(), any::<i64>(), 0_u8..=18).prop_map(|(a, b, scale)| {
            SignalDomain::FixedDecimal(
                tl_syntax::FixedDecimalSignalDomain::new(a.min(b), a.max(b), scale).unwrap(),
            )
        }),
    ]
}

fn signal_catalog() -> impl Strategy<Value = SignalCatalogDocument> {
    // Signal identities strictly increase; each signal is named uniquely by
    // appending its identity; Boolean signals may carry one binding each, with
    // strictly increasing proposition identities.
    (
        prop::collection::vec(
            (1_u32..=1000, signal_domain(), "[a-z][a-z0-9_]{0,12}"),
            0..=8,
        ),
        prop::collection::vec(1_u32..=1000, 8),
        prop::collection::vec(any::<bool>(), 8),
    )
        .prop_map(|(raw, proposition_steps, bind)| {
            let mut next = 0_u32;
            let signals: Vec<OwnedSignalDeclaration> = raw
                .into_iter()
                .map(|(step, domain, stem)| {
                    next += step;
                    OwnedSignalDeclaration::new(SignalId(next), format!("{stem}_{next}"), domain)
                })
                .collect();
            let mut proposition = 0_u32;
            let mut bindings = Vec::new();
            for (index, signal) in signals.iter().enumerate() {
                if matches!(signal.as_borrowed().domain(), SignalDomain::Boolean)
                    && bind[index % bind.len()]
                {
                    proposition += proposition_steps[index % proposition_steps.len()];
                    bindings.push(PropositionBinding::new(
                        PropositionId(proposition),
                        signal.as_borrowed().id(),
                    ));
                }
            }
            SignalCatalogDocument::new(signals, bindings)
                .expect("generator builds only valid catalogs")
        })
}

// Trace: TC-075, FR-014-AC-1
// spec-correctness: row=FR-014-AC-1 property=universal extraction=extractable origin=regex
#[test]
fn signal_catalogs_round_trip_to_equal_values_and_identical_bytes() {
    run("catalog_round_trip", signal_catalog(), |document| {
        let bytes = document.canonical_json_bytes().unwrap();
        let decoded = SignalCatalogDocument::from_json_bytes(&bytes, limits())
            .map_err(|error| TestCaseError::fail(format!("{error}")))?;
        prop_assert_eq!(&decoded, &document);
        prop_assert_eq!(decoded.canonical_json_bytes().unwrap(), bytes);
        prop_assert_eq!(
            decoded.content_identity().unwrap(),
            document.content_identity().unwrap()
        );
        // The borrowed catalog view rebuilds the same document.
        let rebuilt = SignalCatalogDocument::from_catalog(document.validate().unwrap()).unwrap();
        prop_assert_eq!(rebuilt, document);
        Ok(())
    });
}

// ------------------------------------------------------- proposition maps

fn proposition_map() -> impl Strategy<Value = PropositionMapDocument> {
    prop::collection::vec((1_u32..=1000, "[A-Za-z][A-Za-z0-9 _.-]{0,16}"), 0..=10).prop_map(|raw| {
        let mut next = 0_u32;
        let entries = raw
            .into_iter()
            .map(|(step, stem)| {
                next += step;
                PropositionEntry {
                    id: PropositionId(next),
                    name: format!("{stem}#{next}"),
                }
            })
            .collect();
        PropositionMapDocument::new(entries).expect("generator builds only valid maps")
    })
}

// Trace: TC-075, FR-014-AC-1
// spec-correctness: row=FR-014-AC-1 property=universal extraction=extractable origin=regex
#[test]
fn proposition_maps_round_trip_to_equal_values_and_identical_bytes() {
    run("map_round_trip", proposition_map(), |document| {
        let bytes = document.canonical_json_bytes().unwrap();
        let decoded = PropositionMapDocument::from_json_bytes(&bytes, limits())
            .map_err(|error| TestCaseError::fail(format!("{error}")))?;
        prop_assert_eq!(&decoded, &document);
        prop_assert_eq!(decoded.canonical_json_bytes().unwrap(), bytes);
        prop_assert_eq!(
            decoded.content_identity().unwrap(),
            document.content_identity().unwrap()
        );
        Ok(())
    });
}

// ------------------------------------------------- non-canonical rejection

/// One class of non-canonical or invalid encoding of a canonical document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mutation {
    /// Same value, object members in reverse order.
    ReorderedFields,
    /// Same value, insignificant whitespace between tokens.
    PrettyPrinted,
    /// Same value, trailing newline after the document.
    TrailingWhitespace,
    /// Same value, but an integer written as a float (`0` as `0.0`).
    NonMinimalNumber,
    /// First member written twice.
    DuplicateKey,
    /// Extra unknown member.
    UnknownField,
    /// `schema_version` replaced by an unknown version.
    UnknownVersion,
    /// Complete document followed by a second document.
    TrailingData,
}

const MUTATIONS: [Mutation; 8] = [
    Mutation::ReorderedFields,
    Mutation::PrettyPrinted,
    Mutation::TrailingWhitespace,
    Mutation::NonMinimalNumber,
    Mutation::DuplicateKey,
    Mutation::UnknownField,
    Mutation::UnknownVersion,
    Mutation::TrailingData,
];

fn mutation() -> impl Strategy<Value = Mutation> {
    prop::sample::select(MUTATIONS.to_vec())
}

fn reverse_members(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let members: Vec<String> = map
                .iter()
                .rev()
                .map(|(key, member)| format!("{}:{}", serde_json::to_string(key).unwrap(), member))
                .collect();
            format!("{{{}}}", members.join(","))
        }
        other => other.to_string(),
    }
}

/// Applies `mutation` to canonical bytes. Returns `None` when the mutation has
/// no effect for this document (no top-level member to reorder, or no integer
/// to rewrite); the caller must account for those.
fn mutate(canonical: &[u8], mutation: Mutation) -> Option<Vec<u8>> {
    let text = std::str::from_utf8(canonical).unwrap();
    let value: serde_json::Value = serde_json::from_slice(canonical).unwrap();
    let mutated = match mutation {
        Mutation::ReorderedFields => reverse_members(&value),
        Mutation::PrettyPrinted => serde_json::to_string_pretty(&value).unwrap(),
        Mutation::TrailingWhitespace => format!("{text}\n"),
        Mutation::NonMinimalNumber => {
            // Rewrite the first `:<digits>` integer member value as a float.
            let bytes = text.as_bytes();
            let mut at = None;
            for index in 1..bytes.len().saturating_sub(1) {
                if bytes[index - 1] == b':'
                    && bytes[index].is_ascii_digit()
                    && !text[index..].starts_with("0.")
                {
                    at = Some(index);
                    break;
                }
            }
            let index = at?;
            let end = index
                + text[index..]
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(text.len() - index);
            format!("{}{}.0{}", &text[..index], &text[index..end], &text[end..])
        }
        Mutation::DuplicateKey => {
            let first_end = text.find(',').unwrap();
            format!("{{{},{}", &text[1..first_end], &text[1..])
        }
        Mutation::UnknownField => format!("{},\"unexpected\":0}}", &text[..text.len() - 1]),
        Mutation::UnknownVersion => {
            let start = text.find("/v").unwrap() + 2;
            let end = start + text[start..].find('"').unwrap();
            format!("{}9{}", &text[..start], &text[end..])
        }
        Mutation::TrailingData => format!("{text}{text}"),
    };
    Some(mutated.into_bytes())
}

/// The error class a rejected mutation must produce. Encodings of the same
/// value that differ only in form are `NonCanonicalDocument`; the rest are
/// invalid documents.
fn expected_class(mutation: Mutation) -> fn(&StrictDocumentReadError) -> bool {
    match mutation {
        Mutation::ReorderedFields | Mutation::PrettyPrinted | Mutation::TrailingWhitespace => {
            |e| matches!(e, StrictDocumentReadError::NonCanonicalDocument)
        }
        Mutation::NonMinimalNumber
        | Mutation::DuplicateKey
        | Mutation::UnknownField
        | Mutation::UnknownVersion
        | Mutation::TrailingData => |e| matches!(e, StrictDocumentReadError::InvalidDocument(_)),
    }
}

fn assert_rejected<T: std::fmt::Debug>(
    mutation: Mutation,
    canonical: &[u8],
    read: impl Fn(&[u8]) -> Result<T, StrictDocumentReadError>,
) -> Result<(), TestCaseError> {
    let Some(bytes) = mutate(canonical, mutation) else {
        return Ok(());
    };
    // A mutation that leaves the bytes unchanged is not a mutation (for
    // example reversing a single-member object); the canonical form stays
    // accepted then.
    if bytes == canonical {
        prop_assert!(read(&bytes).is_ok());
        return Ok(());
    }
    match read(&bytes) {
        Ok(value) => Err(TestCaseError::fail(format!(
            "{mutation:?} accepted non-canonical input as {value:?}"
        ))),
        Err(error) if expected_class(mutation)(&error) => Ok(()),
        Err(error) => Err(TestCaseError::fail(format!(
            "{mutation:?} rejected with unexpected class: {error}"
        ))),
    }
}

// Trace: TC-075, FR-014-AC-2
#[test]
fn formula_reader_rejects_each_noncanonical_class() {
    run(
        "formula_noncanonical",
        (formula_document(), mutation()),
        |(document, mutation)| {
            let canonical = document.canonical_json_bytes().unwrap();
            assert_rejected(mutation, &canonical, |bytes| {
                FormulaDocument::from_json_bytes(bytes, limits())
            })
        },
    );
}

// Trace: TC-075, FR-014-AC-2, FR-014-AC-3
#[test]
fn signal_catalog_reader_rejects_each_noncanonical_class() {
    run(
        "catalog_noncanonical",
        (signal_catalog(), mutation()),
        |(document, mutation)| {
            let canonical = document.canonical_json_bytes().unwrap();
            assert_rejected(mutation, &canonical, |bytes| {
                SignalCatalogDocument::from_json_bytes(bytes, limits())
            })
        },
    );
}

// Trace: TC-075, FR-014-AC-2, FR-014-AC-3
#[test]
fn proposition_map_reader_rejects_each_noncanonical_class() {
    run(
        "map_noncanonical",
        (proposition_map(), mutation()),
        |(document, mutation)| {
            let canonical = document.canonical_json_bytes().unwrap();
            assert_rejected(mutation, &canonical, |bytes| {
                PropositionMapDocument::from_json_bytes(bytes, limits())
            })
        },
    );
}

// ------------------------------------------- semantically invalid documents

/// A structurally well-formed document that breaks one semantic rule. Each
/// is written in canonical form, so only validation can refuse it.
fn assert_invalid(
    value: &serde_json::Value,
    read: impl Fn(&[u8]) -> Result<(), StrictDocumentReadError>,
) -> Result<(), TestCaseError> {
    let bytes = serde_json::to_vec(value).unwrap();
    match read(&bytes) {
        Err(StrictDocumentReadError::InvalidDocument(_)) => Ok(()),
        other => Err(TestCaseError::fail(format!(
            "semantically invalid document gave {other:?}: {value}"
        ))),
    }
}

// Trace: TC-075, FR-014-AC-2, FR-014-AC-3
#[test]
fn proposition_map_reader_rejects_each_semantic_fault() {
    let strategy = (
        prop::collection::vec((1_u32..=1000, "[a-z]{1,8}"), 2..=10),
        0_u8..4,
        any::<prop::sample::Index>(),
    );
    run("map_semantic_faults", strategy, |(raw, fault, index)| {
        let mut next = 0_u32;
        let mut entries: Vec<(u32, String)> = raw
            .into_iter()
            .map(|(step, stem)| {
                next += step;
                (next, format!("{stem}{next}"))
            })
            .collect();
        let at = index.index(entries.len() - 1) + 1;
        match fault {
            // Repeated identity.
            0 => entries[at].0 = entries[at - 1].0,
            // Decreasing identity.
            1 => entries.swap(at - 1, at),
            // Repeated name.
            2 => entries[at].1 = entries[at - 1].1.clone(),
            // Empty name.
            _ => entries[at].1 = String::new(),
        }
        let value = serde_json::json!({
            "schema_version": "tl-syntax.proposition-map/v1",
            "propositions": entries
                .iter()
                .map(|(id, name)| serde_json::json!({"id": id, "name": name}))
                .collect::<Vec<_>>(),
        });
        assert_invalid(&value, |bytes| {
            PropositionMapDocument::from_json_bytes(bytes, limits()).map(|_| ())
        })
    });
}

// Trace: TC-075, FR-014-AC-2, FR-014-AC-3
#[test]
fn signal_catalog_reader_rejects_each_semantic_fault() {
    let strategy = (
        prop::collection::vec((1_u32..=1000, "[a-z]{1,8}"), 2..=8),
        0_u8..5,
        any::<prop::sample::Index>(),
        any::<i64>(),
    );
    run(
        "catalog_semantic_faults",
        strategy,
        |(raw, fault, index, bound)| {
            let mut next = 0_u32;
            let mut signals: Vec<(u32, String, serde_json::Value)> = raw
                .into_iter()
                .map(|(step, stem)| {
                    next += step;
                    (
                        next,
                        format!("{stem}{next}"),
                        serde_json::json!({"kind": "boolean"}),
                    )
                })
                .collect();
            let at = index.index(signals.len() - 1) + 1;
            let mut bindings = Vec::new();
            match fault {
                // Repeated signal identity.
                0 => signals[at].0 = signals[at - 1].0,
                // Repeated signal name.
                1 => signals[at].1 = signals[at - 1].1.clone(),
                // Inverted integer bounds.
                2 => {
                    let low = bound.max(i64::MIN + 1);
                    signals[at].2 = serde_json::json!({
                        "kind": "integer", "minimum": low, "maximum": low - 1
                    });
                }
                // Binding to an undeclared signal.
                3 => bindings.push(serde_json::json!({"proposition": 1, "signal": next + 1})),
                // Fixed-decimal scale above 18.
                _ => {
                    signals[at].2 = serde_json::json!({
                        "kind": "fixed_decimal", "minimum_coefficient": 0,
                        "maximum_coefficient": 1, "scale": 19
                    });
                }
            }
            let value = serde_json::json!({
                "schema_version": "tl-syntax.signal-catalog/v1",
                "signals": signals
                    .iter()
                    .map(|(id, name, domain)| {
                        serde_json::json!({"id": id, "name": name, "domain": domain})
                    })
                    .collect::<Vec<_>>(),
                "bindings": bindings,
            });
            assert_invalid(&value, |bytes| {
                SignalCatalogDocument::from_json_bytes(bytes, limits()).map(|_| ())
            })
        },
    );
}

// Control for the semantic-fault properties: the hand-built JSON shapes they
// corrupt are accepted when uncorrupted, so a rejection is the fault's doing.
// Trace: TC-075, FR-014-AC-2
#[test]
fn semantic_fault_json_shapes_are_accepted_when_uncorrupted() {
    let catalog = br#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{"id":1,"name":"a","domain":{"kind":"boolean"}},{"id":2,"name":"b","domain":{"kind":"integer","minimum":-3,"maximum":4}},{"id":3,"name":"c","domain":{"kind":"fixed_decimal","minimum_coefficient":0,"maximum_coefficient":1,"scale":18}}],"bindings":[{"proposition":1,"signal":1}]}"#;
    SignalCatalogDocument::from_json_bytes(catalog, limits()).unwrap();
    let map = br#"{"schema_version":"tl-syntax.proposition-map/v1","propositions":[{"id":1,"name":"a"},{"id":2,"name":"b"}]}"#;
    PropositionMapDocument::from_json_bytes(map, limits()).unwrap();
}
