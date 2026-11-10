//! TL-371: behavior tests for wire readers, identities and messages that the first full
//! cargo-mutants run showed were not asserted. Each test pins an exact value or boundary.
//! The file name sorts first so cargo runs it before the heavy suites: a mutant that hangs a later
//! suite is still killed by a test here.
#![cfg(all(feature = "alloc", feature = "serde"))]

use std::str::FromStr;

use tl_syntax::{
    FairnessPremisesDocument, FairnessPremisesError, FairnessReadError, FormulaConversionError,
    FormulaDocument, FormulaSchemaVersion, InfiniteClock, InfiniteClockError,
    InfiniteFormulaDocument, InfiniteNode, InfiniteNodeKind, InfiniteProfileError,
    LassoTraceDocument, LassoTraceError, Node, NodeId, NodeKind, PartialValuation,
    PartialValuationError, PartialValue, PropositionId, SemanticProfile, StrictDocumentReadError,
    SyntaxArtifactLimits, TemporalInterval, TemporalIntervalParseError, TraceObservation,
    ValuationEntry, EVENT_POSITION_CLOCK, FORMULA_UNBOUNDED_V1,
};

const OWNER: SyntaxArtifactLimits = SyntaxArtifactLimits::OWNER_MAXIMA;

/// Writes a JSON escape with `@` standing for the backslash, so the source stays readable.
fn escaped(text: &str) -> Vec<u8> {
    text.replace('@', "\\").into_bytes()
}

/// Replaces the first occurrence of `from` in `bytes`.
fn replace(bytes: &[u8], from: &str, to: &[u8]) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).unwrap();
    let index = text.find(from).expect("field present");
    let mut out = bytes[..index].to_vec();
    out.extend_from_slice(to);
    out.extend_from_slice(&bytes[index + from.len()..]);
    out
}

fn resource(error: StrictDocumentReadError) -> (&'static str, usize, usize) {
    match error {
        StrictDocumentReadError::ResourceLimitExceeded {
            resource,
            actual,
            limit,
        } => (resource, actual, limit),
        other => panic!("expected a resource refusal, got {other:?}"),
    }
}

/// The retained work a reader charges is part of the work limit: one under it is refused.
fn assert_work_accounting<T: std::fmt::Debug>(
    bytes: &[u8],
    retained: usize,
    read: impl Fn(SyntaxArtifactLimits) -> Result<T, StrictDocumentReadError>,
) {
    let total = bytes.len() * 3 + retained;
    let limits = |work| SyntaxArtifactLimits { work, ..OWNER };
    assert!(read(limits(total)).is_ok());
    match read(limits(total - 1)) {
        Err(StrictDocumentReadError::WorkLimitExceeded { actual, limit }) => {
            assert_eq!((actual, limit), (total, total - 1));
        }
        other => panic!("expected the work limit, got {other:?}"),
    }
}

#[test]
fn temporal_interval_bounds_are_plain_decimal_digits() {
    assert_eq!(
        TemporalInterval::from_str("[+1,2]"),
        Err(TemporalIntervalParseError::InvalidBound)
    );
    assert_eq!(
        TemporalInterval::from_str("[1,+2]"),
        Err(TemporalIntervalParseError::InvalidBound)
    );
    assert_eq!(
        TemporalInterval::from_str("[,2]"),
        Err(TemporalIntervalParseError::InvalidBound)
    );
    assert!(TemporalInterval::from_str("[1,2]").is_ok());
}

#[test]
fn infinite_error_messages_and_identities_are_exact() {
    assert_eq!(
        TemporalIntervalParseError::Inverted.to_string(),
        "invalid temporal interval: Inverted"
    );
    assert_eq!(InfiniteClock::EventPosition.as_str(), "event_position");
    assert_eq!(
        InfiniteClockError::Missing.to_string(),
        "infinite-trace clock identity is missing"
    );
    assert_eq!(
        InfiniteClockError::Unsupported.to_string(),
        "infinite-trace clock requires event_position"
    );
    assert_eq!(
        InfiniteProfileError::Missing.to_string(),
        "infinite-trace TL profile identity is missing"
    );
    assert_eq!(
        InfiniteProfileError::Mismatched(SemanticProfile::ClosedTraceV1).to_string(),
        format!(
            "TL profile {} is not infinite trace",
            SemanticProfile::ClosedTraceV1.as_str()
        )
    );
    assert_eq!(
        InfiniteProfileError::Unknown.to_string(),
        "unknown TL profile identity"
    );
}

fn true_graph(count: usize) -> InfiniteFormulaDocument {
    InfiniteFormulaDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        NodeId(0),
        vec![InfiniteNode::new(InfiniteNodeKind::True); count],
    )
    .unwrap()
}

#[test]
fn infinite_formula_document_names_its_schema_and_node_expectation() {
    assert_eq!(true_graph(1).schema_version(), FORMULA_UNBOUNDED_V1);
    let bytes = format!(
        r#"{{"schema_version":"{FORMULA_UNBOUNDED_V1}","semantic_profile":"{}","clock":"{EVENT_POSITION_CLOCK}","root":0,"nodes":5}}"#,
        SemanticProfile::InfiniteTraceV1.as_str()
    );
    let message = serde_json::from_str::<InfiniteFormulaDocument>(&bytes)
        .unwrap_err()
        .to_string();
    assert!(
        message.contains("at most 100000 infinite formula nodes"),
        "{message}"
    );
}

#[test]
fn formula_document_names_its_schema_and_node_expectation() {
    assert_eq!(FormulaSchemaVersion::V1.as_str(), "tl-syntax.formula/v1");
    assert_eq!(FormulaSchemaVersion::V2.as_str(), "tl-syntax.formula/v2");
    let bytes = format!(
        r#"{{"schema_version":"tl-syntax.formula/v1","semantic_profile":"{}","root":0,"nodes":5}}"#,
        SemanticProfile::ClosedTraceV1.as_str()
    );
    let message = serde_json::from_str::<FormulaDocument>(&bytes)
        .unwrap_err()
        .to_string();
    assert!(
        message.contains("at most 100000 formula nodes"),
        "{message}"
    );
    assert_eq!(
        FormulaConversionError::UnsupportedSemanticProfile {
            profile: SemanticProfile::OriginCompleteHistoryV1
        }
        .to_string(),
        format!(
            "formula-v1 cannot represent semantic profile {}",
            SemanticProfile::OriginCompleteHistoryV1.as_str()
        )
    );
}

fn formula(nodes: Vec<Node>) -> FormulaDocument {
    FormulaDocument::new(SemanticProfile::ClosedTraceV1, NodeId(0), nodes).unwrap()
}

#[test]
fn formula_identity_and_depth_follow_the_graph() {
    let single = formula(vec![Node::new(NodeKind::True)]);
    let double = formula(vec![
        Node::new(NodeKind::True),
        Node::new(NodeKind::Not { operand: NodeId(0) }),
    ]);
    let identity = single.content_identity().unwrap();
    assert!(
        identity.starts_with("sha256:") || identity.len() > 16,
        "{identity}"
    );
    assert_ne!(identity, double.content_identity().unwrap());
    assert_eq!(identity, single.content_identity().unwrap());
    // The reader refuses a chain deeper than its depth limit and admits the limit itself.
    let chain = |length: usize| {
        let mut nodes = vec![Node::new(NodeKind::True)];
        for index in 1..length {
            nodes.push(Node::new(NodeKind::Not {
                operand: NodeId(index as u32 - 1),
            }));
        }
        FormulaDocument::new(
            SemanticProfile::ClosedTraceV1,
            NodeId(length as u32 - 1),
            nodes,
        )
        .unwrap()
        .canonical_json_bytes()
        .unwrap()
    };
    let at = |depth: usize| SyntaxArtifactLimits {
        formula_depth: depth,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&chain(3), at(3)).is_ok());
    assert_eq!(
        resource(FormulaDocument::from_json_bytes(&chain(3), at(2)).unwrap_err()),
        ("formula depth", 3, 2)
    );
    assert_eq!(
        resource(FormulaDocument::from_json_bytes(&chain(1), at(0)).unwrap_err()),
        ("formula depth", 1, 0)
    );
}

#[test]
fn semantic_views_hash_and_order_by_content() {
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let first = formula(vec![Node::new(NodeKind::True)]);
    let second = formula(vec![Node::new(NodeKind::False)]);
    let hash = |document: &FormulaDocument| {
        let mut hasher = DefaultHasher::new();
        document.semantic_view().hash(&mut hasher);
        hasher.finish()
    };
    assert_eq!(hash(&first), hash(&first.clone()));
    assert_ne!(hash(&first), hash(&second));
    assert_eq!(
        first.semantic_view().partial_cmp(&first.semantic_view()),
        Some(Ordering::Equal)
    );
    assert_ne!(
        first.semantic_view().partial_cmp(&second.semantic_view()),
        Some(Ordering::Equal)
    );
    assert_eq!(
        first.semantic_view().partial_cmp(&second.semantic_view()),
        Some(first.semantic_view().cmp(&second.semantic_view()))
    );
}

fn valuation(count: u32) -> PartialValuation {
    let propositions: Vec<_> = (0..count).map(PropositionId).collect();
    let entries = propositions
        .iter()
        .map(|&proposition| ValuationEntry {
            proposition,
            value: PartialValue::True,
        })
        .collect();
    PartialValuation::new("map".into(), &propositions, entries).unwrap()
}

#[test]
fn partial_valuation_exposes_its_identity_and_declared_propositions() {
    let value = valuation(3);
    assert_eq!(
        value.propositions(),
        &[PropositionId(0), PropositionId(1), PropositionId(2)]
    );
    let identity = value.content_identity().unwrap();
    assert_ne!(identity, valuation(2).content_identity().unwrap());
    assert_eq!(identity, valuation(3).content_identity().unwrap());
    assert!(!identity.is_empty());
    assert_eq!(
        PartialValuationError::MapOrder.to_string(),
        "partial valuation admission: MapOrder"
    );
}

#[test]
fn partial_valuation_reader_charges_and_limits_its_entries() {
    let bytes = valuation(3).canonical_json_bytes().unwrap();
    let at = |entries| SyntaxArtifactLimits {
        propositions: entries,
        ..OWNER
    };
    assert!(PartialValuation::from_json_bytes(&bytes, at(3)).is_ok());
    assert_eq!(
        resource(PartialValuation::from_json_bytes(&bytes, at(2)).unwrap_err()),
        ("valuation entries", 3, 2)
    );
    assert_work_accounting(&bytes, 3, |limits| {
        PartialValuation::from_json_bytes(&bytes, limits)
    });
    // A field name spelled with an escape hides the entries from the byte scan, so the
    // limit is enforced on the decoded document, and an exact fit is only non-canonical.
    let hidden = replace(&bytes, "\"entries\"", &escaped("\"@u0065ntries\""));
    assert_eq!(
        resource(PartialValuation::from_json_bytes(&hidden, at(2)).unwrap_err()),
        ("valuation entries", 3, 2)
    );
    assert!(matches!(
        PartialValuation::from_json_bytes(&hidden, at(3)),
        Err(StrictDocumentReadError::NonCanonicalDocument)
    ));
}

fn observation(position: u32) -> TraceObservation {
    TraceObservation {
        position,
        valuation: valuation(2),
    }
}

fn lasso(prefix: u32, looped: u32) -> LassoTraceDocument {
    // The valuation's map identity is "map", over propositions 0 and 1.
    LassoTraceDocument::new(
        SemanticProfile::InfiniteTraceV1,
        InfiniteClock::EventPosition,
        "map".into(),
        vec![PropositionId(0), PropositionId(1)],
        (0..prefix).map(observation).collect(),
        (prefix..prefix + looped).map(observation).collect(),
    )
    .unwrap()
}

#[test]
fn lasso_trace_exposes_its_parts_and_identity() {
    let trace = lasso(1, 2);
    assert_eq!(trace.proposition_map_identity(), "map");
    assert_eq!(trace.propositions(), &[PropositionId(0), PropositionId(1)]);
    assert_eq!(trace.prefix().len(), 1);
    assert_eq!(trace.prefix()[0].position, 0);
    assert_eq!(trace.loop_observations().len(), 2);
    assert_eq!(trace.loop_observations()[0].position, 1);
    let identity = trace.content_identity().unwrap();
    assert!(!identity.is_empty());
    assert_ne!(identity, lasso(1, 1).content_identity().unwrap());
    assert_eq!(identity, lasso(1, 2).content_identity().unwrap());
    assert_eq!(
        LassoTraceError::EmptyLoop.to_string(),
        "lasso trace admission: EmptyLoop"
    );
}

#[test]
fn lasso_trace_reader_charges_and_limits_propositions_and_positions() {
    let bytes = lasso(1, 2).canonical_json_bytes().unwrap();
    let props = |n| SyntaxArtifactLimits {
        propositions: n,
        ..OWNER
    };
    let positions = |n| SyntaxArtifactLimits {
        formula_nodes: n,
        ..OWNER
    };
    assert!(LassoTraceDocument::from_json_bytes(&bytes, OWNER).is_ok());
    assert!(LassoTraceDocument::from_json_bytes(&bytes, props(2)).is_ok());
    assert_eq!(
        resource(LassoTraceDocument::from_json_bytes(&bytes, props(1)).unwrap_err()),
        ("trace propositions", 2, 1)
    );
    assert!(LassoTraceDocument::from_json_bytes(&bytes, positions(3)).is_ok());
    assert_eq!(
        resource(LassoTraceDocument::from_json_bytes(&bytes, positions(2)).unwrap_err()),
        ("trace positions", 3, 2)
    );
    assert_work_accounting(&bytes, bytes.len(), |limits| {
        LassoTraceDocument::from_json_bytes(&bytes, limits)
    });
    // The first `propositions` field is the trace's own.
    let hidden = replace(
        &bytes,
        "\"propositions\"",
        &escaped("\"@u0070ropositions\""),
    );
    assert_eq!(
        resource(LassoTraceDocument::from_json_bytes(&hidden, props(1)).unwrap_err()),
        ("trace propositions", 2, 1)
    );
    assert!(matches!(
        LassoTraceDocument::from_json_bytes(&hidden, props(2)),
        Err(StrictDocumentReadError::NonCanonicalDocument)
    ));
}

#[test]
fn fairness_premises_expose_identity_and_charge_their_roots() {
    let formula = true_graph(3);
    let identity = formula.content_identity().unwrap();
    let premises = FairnessPremisesDocument::new(
        &formula,
        identity.clone(),
        InfiniteClock::EventPosition,
        vec![NodeId(0), NodeId(2)],
    )
    .unwrap();
    assert_eq!(premises.graph_identity(), identity);
    let own = premises.content_identity().unwrap();
    assert!(!own.is_empty());
    let other = FairnessPremisesDocument::new(
        &formula,
        identity,
        InfiniteClock::EventPosition,
        vec![NodeId(0)],
    )
    .unwrap();
    assert_ne!(own, other.content_identity().unwrap());
    let bytes = premises.canonical_json_bytes().unwrap();
    let read = |limits| FairnessPremisesDocument::from_json_bytes(&bytes, limits, &formula);
    let roots = |n| SyntaxArtifactLimits {
        formula_nodes: n,
        ..OWNER
    };
    assert!(read(roots(2)).is_ok());
    match read(roots(1)).unwrap_err() {
        FairnessReadError::Document(error) => {
            assert_eq!(resource(error), ("fairness roots", 2, 1));
        }
        other => panic!("{other:?}"),
    }
    assert_work_accounting(&bytes, 2, |limits| match read(limits) {
        Ok(value) => Ok(value),
        Err(FairnessReadError::Document(error)) => Err(error),
        Err(other) => panic!("{other:?}"),
    });
    let hidden = replace(&bytes, "\"roots\"", &escaped("\"@u0072oots\""));
    let read_hidden = |limits| FairnessPremisesDocument::from_json_bytes(&hidden, limits, &formula);
    match read_hidden(roots(1)).unwrap_err() {
        FairnessReadError::Document(error) => {
            assert_eq!(resource(error), ("fairness roots", 2, 1));
        }
        other => panic!("{other:?}"),
    }
    assert!(matches!(
        read_hidden(roots(2)),
        Err(FairnessReadError::Document(
            StrictDocumentReadError::NonCanonicalDocument
        ))
    ));
    assert_eq!(
        FairnessPremisesError::Profile.to_string(),
        "fairness premises admission: Profile"
    );
    let rendered = FairnessReadError::Binding(FairnessPremisesError::Clock).to_string();
    assert_eq!(rendered, "fairness premises admission: Clock");
    let rendered =
        FairnessReadError::Document(StrictDocumentReadError::NonCanonicalDocument).to_string();
    assert_eq!(
        rendered,
        StrictDocumentReadError::NonCanonicalDocument.to_string()
    );
}

#[test]
fn formula_reader_charges_and_limits_its_nodes() {
    let document = formula(vec![
        Node::new(NodeKind::True),
        Node::new(NodeKind::Not { operand: NodeId(0) }),
        Node::new(NodeKind::Not { operand: NodeId(1) }),
    ]);
    let bytes = document.canonical_json_bytes().unwrap();
    let nodes = |n| SyntaxArtifactLimits {
        formula_nodes: n,
        ..OWNER
    };
    assert!(FormulaDocument::from_json_bytes(&bytes, nodes(3)).is_ok());
    assert_eq!(
        resource(FormulaDocument::from_json_bytes(&bytes, nodes(2)).unwrap_err()),
        ("formula nodes", 3, 2)
    );
    assert_work_accounting(&bytes, 3, |limits| {
        FormulaDocument::from_json_bytes(&bytes, limits)
    });
    let hidden = replace(&bytes, "\"nodes\"", &escaped("\"@u006eodes\""));
    assert_eq!(
        resource(FormulaDocument::from_json_bytes(&hidden, nodes(2)).unwrap_err()),
        ("formula nodes", 3, 2)
    );
    assert!(matches!(
        FormulaDocument::from_json_bytes(&hidden, nodes(3)),
        Err(StrictDocumentReadError::NonCanonicalDocument)
    ));
}

mod signals {
    use super::*;
    use tl_syntax::{
        FixedDecimalSignalDomain, FormulaBindingError, IntegerSignalDomain, OwnedSignalDeclaration,
        PropositionBinding, PropositionEntry, PropositionMapDocument, PropositionMapError,
        PropositionMapSchemaVersion, SignalCatalog, SignalCatalogDocument, SignalCatalogError,
        SignalCatalogSchemaVersion, SignalDeclaration, SignalDomain, SignalDomainError, SignalId,
    };

    #[test]
    fn domain_accessors_return_their_bounds() {
        let integer = IntegerSignalDomain::new(-7, 9).unwrap();
        assert_eq!((integer.minimum(), integer.maximum()), (-7, 9));
        let decimal = FixedDecimalSignalDomain::new(-250, 300, 4).unwrap();
        assert_eq!(
            (
                decimal.minimum_coefficient(),
                decimal.maximum_coefficient(),
                decimal.scale()
            ),
            (-250, 300, 4)
        );
    }

    #[test]
    fn domain_and_binding_errors_render_their_numbers() {
        assert_eq!(
            SignalDomainError::IntegerBoundsInverted {
                minimum: 5,
                maximum: 2
            }
            .to_string(),
            "integer signal minimum 5 exceeds maximum 2"
        );
        assert_eq!(
            SignalDomainError::DecimalBoundsInverted {
                minimum_coefficient: 5,
                maximum_coefficient: 2
            }
            .to_string(),
            "decimal signal coefficient minimum 5 exceeds maximum 2"
        );
        assert_eq!(
            SignalDomainError::DecimalScaleOutOfRange { scale: 19 }.to_string(),
            "decimal signal scale 19 exceeds 18"
        );
        assert_eq!(
            FormulaBindingError::MissingPropositionBinding {
                proposition: PropositionId(7)
            }
            .to_string(),
            "formula proposition 7 has no signal binding"
        );
    }

    #[test]
    fn catalog_errors_render_their_numbers() {
        let rendered = |error: SignalCatalogError| error.to_string();
        assert_eq!(
            rendered(SignalCatalogError::SignalLimitExceeded { count: 9, limit: 8 }),
            "signal count 9 exceeds the 8-signal limit"
        );
        assert_eq!(
            rendered(SignalCatalogError::BindingLimitExceeded { count: 9, limit: 8 }),
            "proposition binding count 9 exceeds the 8-binding limit"
        );
        assert_eq!(
            rendered(SignalCatalogError::NameOrderScratchTooSmall {
                provided: 1,
                required: 2
            }),
            "signal name-order scratch has 1 slots but 2 are required"
        );
        assert_eq!(
            rendered(SignalCatalogError::SignalIdentityNotIncreasing {
                previous: SignalId(4),
                current: SignalId(3)
            }),
            "signal identity 3 does not follow 4"
        );
        assert_eq!(
            rendered(SignalCatalogError::EmptySignalName {
                signal: SignalId(2)
            }),
            "signal 2 has an empty name"
        );
        assert_eq!(
            rendered(SignalCatalogError::SignalNameTooLong {
                signal: SignalId(2),
                length: 70,
                limit: 64
            }),
            "signal 2 name has 70 UTF-8 bytes, exceeding the 64-byte limit"
        );
        assert_eq!(
            rendered(SignalCatalogError::DuplicateSignalName {
                first: SignalId(1),
                second: SignalId(3)
            }),
            "signals 1 and 3 have the same name"
        );
        assert_eq!(
            rendered(SignalCatalogError::BindingIdentityNotIncreasing {
                previous: PropositionId(4),
                current: PropositionId(3)
            }),
            "proposition binding identity 3 does not follow 4"
        );
        assert_eq!(
            rendered(SignalCatalogError::BindingTargetMissing {
                proposition: PropositionId(1),
                signal: SignalId(9)
            }),
            "proposition 1 binding targets absent signal 9"
        );
        assert_eq!(
            rendered(SignalCatalogError::NonBooleanPropositionBinding {
                proposition: PropositionId(1),
                signal: SignalId(9)
            }),
            "proposition 1 binding targets non-Boolean signal 9"
        );
        assert_eq!(
            PropositionMapError::EmptyName {
                id: PropositionId(2)
            }
            .to_string(),
            "proposition 2 has an empty name"
        );
        assert_eq!(
            PropositionMapError::IdentityNotIncreasing {
                previous: PropositionId(4),
                current: PropositionId(3)
            }
            .to_string(),
            "proposition identity 3 does not follow 4"
        );
        assert_eq!(
            PropositionMapError::DuplicateName {
                first: PropositionId(1),
                second: PropositionId(3)
            }
            .to_string(),
            "propositions 1 and 3 have the same name"
        );
    }

    fn boolean(id: u32, name: &str) -> SignalDeclaration<'_> {
        SignalDeclaration::new(SignalId(id), name, SignalDomain::Boolean)
    }

    #[test]
    fn catalog_looks_up_every_signal_and_counts_and_iterates_them() {
        let names = ["a", "b", "c", "d", "e"];
        let signals: Vec<_> = names
            .iter()
            .enumerate()
            .map(|(index, name)| boolean(index as u32 * 2 + 1, name))
            .collect();
        let mut scratch = [0_u32; 5];
        let catalog = SignalCatalog::new(&signals, &[], &mut scratch).unwrap();
        assert_eq!(catalog.signal_count(), 5);
        for (index, name) in names.iter().enumerate() {
            let found = catalog.signal(SignalId(index as u32 * 2 + 1)).unwrap();
            assert_eq!(found.name(), *name);
        }
        for missing in [0, 2, 4, 6, 8, 10, 11] {
            assert!(catalog.signal(SignalId(missing)).is_none(), "{missing}");
        }
        let mut iterator = catalog.signals();
        assert_eq!(iterator.len(), 5);
        assert_eq!(iterator.size_hint(), (5, Some(5)));
        assert_eq!(iterator.next().unwrap().name(), "a");
        assert_eq!(iterator.size_hint(), (4, Some(4)));
        assert_eq!(iterator.next().unwrap().name(), "b");
        assert_eq!(iterator.size_hint(), (3, Some(3)));
        let rest: Vec<_> = iterator.map(|signal| signal.name()).collect();
        assert_eq!(rest, ["c", "d", "e"]);
    }

    #[test]
    fn catalog_finds_duplicate_names_that_are_not_adjacent_by_identity() {
        let signals = [boolean(1, "b"), boolean(2, "a"), boolean(3, "b")];
        let mut scratch = [0_u32; 3];
        assert_eq!(
            SignalCatalog::new(&signals, &[], &mut scratch).unwrap_err(),
            SignalCatalogError::DuplicateSignalName {
                first: SignalId(1),
                second: SignalId(3)
            }
        );
    }

    #[test]
    fn schema_identities_are_exact() {
        assert_eq!(
            PropositionMapSchemaVersion::V1.as_str(),
            "tl-syntax.proposition-map/v1"
        );
        assert_eq!(
            SignalCatalogSchemaVersion::V1.as_str(),
            "tl-syntax.signal-catalog/v1"
        );
    }

    fn proposition_map(count: u32) -> PropositionMapDocument {
        PropositionMapDocument::new(
            (0..count)
                .map(|id| PropositionEntry {
                    id: PropositionId(id),
                    name: format!("p{id}"),
                })
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn proposition_map_reader_charges_and_limits_its_entries() {
        let document = proposition_map(3);
        let bytes = document.canonical_json_bytes().unwrap();
        assert_ne!(
            document.content_identity().unwrap(),
            proposition_map(2).content_identity().unwrap()
        );
        assert!(!document.content_identity().unwrap().is_empty());
        let at = |n| SyntaxArtifactLimits {
            propositions: n,
            ..OWNER
        };
        assert!(PropositionMapDocument::from_json_bytes(&bytes, at(3)).is_ok());
        assert_eq!(
            resource(PropositionMapDocument::from_json_bytes(&bytes, at(2)).unwrap_err()),
            ("propositions", 3, 2)
        );
        assert_work_accounting(&bytes, 3, |limits| {
            PropositionMapDocument::from_json_bytes(&bytes, limits)
        });
        let hidden = replace(
            &bytes,
            "\"propositions\"",
            &escaped("\"@u0070ropositions\""),
        );
        assert_eq!(
            resource(PropositionMapDocument::from_json_bytes(&hidden, at(2)).unwrap_err()),
            ("propositions", 3, 2)
        );
        assert!(matches!(
            PropositionMapDocument::from_json_bytes(&hidden, at(3)),
            Err(StrictDocumentReadError::NonCanonicalDocument)
        ));
        let message = serde_json::from_str::<PropositionMapDocument>(
            r#"{"schema_version":"tl-syntax.proposition-map/v1","propositions":5}"#,
        )
        .unwrap_err()
        .to_string();
        assert!(
            message.contains("at most 100000 proposition entries"),
            "{message}"
        );
    }

    fn catalog_document() -> SignalCatalogDocument {
        SignalCatalogDocument::new(
            vec![
                OwnedSignalDeclaration::new(SignalId(1), "a".into(), SignalDomain::Boolean),
                OwnedSignalDeclaration::new(SignalId(2), "b".into(), SignalDomain::Boolean),
                OwnedSignalDeclaration::new(SignalId(3), "c".into(), SignalDomain::Boolean),
            ],
            vec![
                PropositionBinding::new(PropositionId(0), SignalId(1)),
                PropositionBinding::new(PropositionId(1), SignalId(2)),
            ],
        )
        .unwrap()
    }

    #[test]
    fn catalog_document_exposes_parts_and_charges_signals_and_bindings() {
        let document = catalog_document();
        assert_eq!(document.signals().len(), 3);
        assert_eq!(document.signals()[1].as_borrowed().name(), "b");
        assert_eq!(document.bindings().len(), 2);
        assert_eq!(document.bindings()[1].signal(), SignalId(2));
        let bytes = document.canonical_json_bytes().unwrap();
        assert!(bytes.starts_with(b"{\"schema_version\":\"tl-syntax.signal-catalog/v1\""));
        let mut preimage = tl_syntax::CONTENT_IDENTITY_DOMAIN_V1.to_vec();
        preimage.extend_from_slice(SignalCatalogSchemaVersion::V1.as_str().as_bytes());
        preimage.push(0);
        preimage.extend_from_slice(&bytes);
        assert_eq!(
            document.content_identity().unwrap(),
            format!("{:x}", <sha2::Sha256 as sha2::Digest>::digest(&preimage))
        );
        assert!(!document.content_identity().unwrap().is_empty());
        let limits = |signals, bindings| SyntaxArtifactLimits {
            signals,
            bindings,
            ..OWNER
        };
        let read = |limits| SignalCatalogDocument::from_json_bytes(&bytes, limits);
        assert!(read(limits(3, 2)).is_ok());
        assert_eq!(resource(read(limits(2, 2)).unwrap_err()), ("signals", 3, 2));
        assert_eq!(
            resource(read(limits(3, 1)).unwrap_err()),
            ("bindings", 2, 1)
        );
        assert_work_accounting(&bytes, 5, read);
        let hidden_signals = replace(&bytes, "\"signals\"", &escaped("\"@u0073ignals\""));
        let read_hidden =
            |bytes: &[u8], limits| SignalCatalogDocument::from_json_bytes(bytes, limits);
        assert_eq!(
            resource(read_hidden(&hidden_signals, limits(2, 2)).unwrap_err()),
            ("signals", 3, 2)
        );
        assert!(matches!(
            read_hidden(&hidden_signals, limits(3, 2)),
            Err(StrictDocumentReadError::NonCanonicalDocument)
        ));
        let hidden_bindings = replace(&bytes, "\"bindings\"", &escaped("\"@u0062indings\""));
        assert_eq!(
            resource(read_hidden(&hidden_bindings, limits(3, 1)).unwrap_err()),
            ("bindings", 2, 1)
        );
        assert!(matches!(
            read_hidden(&hidden_bindings, limits(3, 2)),
            Err(StrictDocumentReadError::NonCanonicalDocument)
        ));
        let message = serde_json::from_str::<SignalCatalogDocument>(
            r#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":5,"bindings":[]}"#,
        )
        .unwrap_err()
        .to_string();
        assert!(
            message.contains("at most 100000 signal declarations"),
            "{message}"
        );
    }

    #[test]
    fn catalog_document_reader_keeps_what_it_reads() {
        let document = catalog_document();
        let bytes = document.canonical_json_bytes().unwrap();
        let read = SignalCatalogDocument::from_json_bytes(&bytes, OWNER).unwrap();
        assert_eq!(read, document);
        assert_eq!(read.signals().len(), 3);
        assert_eq!(read.bindings().len(), 2);
        let empty_name = replace(&bytes, "\"name\":\"a\"", b"\"name\":\"\"");
        assert!(SignalCatalogDocument::from_json_bytes(&empty_name, OWNER).is_err());
    }
}
