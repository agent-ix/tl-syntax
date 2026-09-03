#![cfg(feature = "serde")]

use proptest::prelude::*;
use tl_syntax::{
    FixedDecimalSignalDomain, Formula, FormulaBindingError, IntegerSignalDomain, Node, NodeId,
    NodeKind, PropositionBinding, PropositionId, RequirementContext, RequirementContextDocument,
    RequirementContextError, RequirementContextField, RequirementContextSchemaVersion,
    SemanticProfile, SignalCatalog, SignalCatalogDocument, SignalCatalogError,
    SignalCatalogSchemaVersion, SignalDeclaration, SignalDomain, SignalDomainError, SignalId,
    SourceSpan, MAX_REQUIREMENT_CONTEXT_FIELD_BYTES, MAX_SIGNAL_CATALOG_BINDINGS,
    MAX_SIGNAL_CATALOG_SIGNALS, MAX_SIGNAL_NAME_BYTES,
};

fn boolean_signal(id: u32, name: &str) -> SignalDeclaration<'_> {
    SignalDeclaration::new(SignalId(id), name, SignalDomain::Boolean)
}

fn catalog<'a>(
    signals: &'a [SignalDeclaration<'a>],
    bindings: &'a [PropositionBinding],
) -> Result<SignalCatalog<'a>, SignalCatalogError> {
    let mut name_order_scratch = vec![0; signals.len()];
    SignalCatalog::new(signals, bindings, &mut name_order_scratch)
}

proptest! {
    // Trace: TC-027, FR-007-AC-1, NFR-002-AC-5
    #[test]
    fn bounded_domains_preserve_all_valid_extremes(
        first in any::<i64>(),
        second in any::<i64>(),
        scale in 0_u8..=18,
    ) {
        let minimum = first.min(second);
        let maximum = first.max(second);
        let signals = [
            boolean_signal(1, "enabled"),
            SignalDeclaration::new(
                SignalId(2),
                "count",
                SignalDomain::Integer(IntegerSignalDomain::new(minimum, maximum).unwrap()),
            ),
            SignalDeclaration::new(
                SignalId(3),
                "decimal",
                SignalDomain::FixedDecimal(
                    FixedDecimalSignalDomain::new(minimum, maximum, scale).unwrap(),
                ),
            ),
        ];
        let catalog = catalog(&signals, &[]).unwrap();
        let observed: Vec<_> = catalog.signals().collect();
        prop_assert_eq!(observed, signals);
        prop_assert_eq!(catalog.signal(SignalId(2)).unwrap().domain(), signals[1].domain());
        prop_assert_eq!(catalog.signal(SignalId(3)).unwrap().domain(), signals[2].domain());
    }
}

// Trace: TC-028, FR-007-AC-1, StR-003-VC-1
#[test]
fn borrowed_and_owned_catalogs_round_trip_with_distinct_identities() {
    let bytes = include_bytes!("fixtures/valid-signal-catalog.json");
    let document: SignalCatalogDocument = serde_json::from_slice(bytes).unwrap();
    assert_eq!(document.schema_version(), SignalCatalogSchemaVersion::V1);
    assert_eq!(
        document.schema_version().as_str(),
        "tl-syntax.signal-catalog/v1"
    );

    let catalog = document.validate().unwrap();
    let observed: Vec<_> = catalog.signals().collect();
    assert_eq!(observed.len(), 3);
    assert_eq!(observed[0].id(), SignalId(1));
    assert_eq!(catalog.bindings()[0].proposition(), PropositionId(7));
    assert_eq!(catalog.bindings()[0].signal(), SignalId(1));

    let copied = SignalCatalogDocument::from_catalog(catalog).unwrap();
    assert_eq!(copied, document);
    let first = serde_json::to_vec(&document).unwrap();
    let second = serde_json::to_vec(&copied).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        String::from_utf8(first.clone()).unwrap(),
        r#"{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{"id":1,"name":"request_ready","domain":{"kind":"boolean"}},{"id":2,"name":"retry_count","domain":{"kind":"integer","minimum":0,"maximum":8}},{"id":3,"name":"temperature","domain":{"kind":"fixed_decimal","minimum_coefficient":-400,"maximum_coefficient":1250,"scale":1}}],"bindings":[{"proposition":7,"signal":1}]}"#
    );
    assert_eq!(
        serde_json::from_slice::<SignalCatalogDocument>(&first).unwrap(),
        document
    );
}

// Trace: TC-027, FR-007-AC-1, NFR-002-AC-5
#[test]
fn signal_name_identity_is_exact_utf8_without_normalization_or_case_folding() {
    let signals = [
        boolean_signal(1, "E"),
        boolean_signal(2, "e\u{301}"),
        boolean_signal(3, "é"),
    ];
    let catalog = catalog(&signals, &[]).unwrap();
    assert_eq!(catalog.signals().collect::<Vec<_>>(), signals);
}

// Trace: TC-029, FR-007-AC-2
#[test]
fn every_catalog_refusal_has_a_neighboring_positive_control() {
    let valid_signals = [
        boolean_signal(1, "ready"),
        SignalDeclaration::new(
            SignalId(2),
            "count",
            SignalDomain::Integer(IntegerSignalDomain::new(0, 8).unwrap()),
        ),
    ];
    let valid_bindings = [PropositionBinding::new(PropositionId(7), SignalId(1))];
    assert!(catalog(&valid_signals, &valid_bindings).is_ok());

    let mut insufficient_scratch = [];
    assert_eq!(
        SignalCatalog::new(&valid_signals, &valid_bindings, &mut insufficient_scratch).unwrap_err(),
        SignalCatalogError::NameOrderScratchTooSmall {
            provided: 0,
            required: 2
        }
    );

    let reversed_ids = [boolean_signal(2, "later"), boolean_signal(1, "earlier")];
    assert!(matches!(
        catalog(&reversed_ids, &[]),
        Err(SignalCatalogError::SignalIdentityNotIncreasing { .. })
    ));
    let duplicate_ids = [boolean_signal(1, "first"), boolean_signal(1, "second")];
    assert!(matches!(
        catalog(&duplicate_ids, &[]),
        Err(SignalCatalogError::SignalIdentityNotIncreasing { .. })
    ));
    let empty_name = [boolean_signal(1, "")];
    assert_eq!(
        catalog(&empty_name, &[]).unwrap_err(),
        SignalCatalogError::EmptySignalName {
            signal: SignalId(1)
        }
    );
    let oversized_name = "é".repeat(MAX_SIGNAL_NAME_BYTES / 2 + 1);
    let oversized = [boolean_signal(1, &oversized_name)];
    assert!(matches!(
        catalog(&oversized, &[]),
        Err(SignalCatalogError::SignalNameTooLong { .. })
    ));
    let duplicate_names = [boolean_signal(1, "same"), boolean_signal(2, "same")];
    assert!(matches!(
        catalog(&duplicate_names, &[]),
        Err(SignalCatalogError::DuplicateSignalName { .. })
    ));

    assert_eq!(
        IntegerSignalDomain::new(9, 8).unwrap_err(),
        SignalDomainError::IntegerBoundsInverted {
            minimum: 9,
            maximum: 8
        }
    );
    assert_eq!(
        FixedDecimalSignalDomain::new(2, 1, 2).unwrap_err(),
        SignalDomainError::DecimalBoundsInverted {
            minimum_coefficient: 2,
            maximum_coefficient: 1
        }
    );
    assert_eq!(
        FixedDecimalSignalDomain::new(1, 2, 19).unwrap_err(),
        SignalDomainError::DecimalScaleOutOfRange { scale: 19 }
    );

    let repeated_bindings = [
        PropositionBinding::new(PropositionId(7), SignalId(1)),
        PropositionBinding::new(PropositionId(7), SignalId(1)),
    ];
    assert!(matches!(
        catalog(&valid_signals, &repeated_bindings),
        Err(SignalCatalogError::BindingIdentityNotIncreasing { .. })
    ));
    let reversed_bindings = [
        PropositionBinding::new(PropositionId(8), SignalId(1)),
        PropositionBinding::new(PropositionId(7), SignalId(1)),
    ];
    assert!(matches!(
        catalog(&valid_signals, &reversed_bindings),
        Err(SignalCatalogError::BindingIdentityNotIncreasing { .. })
    ));
    let missing_target = [PropositionBinding::new(PropositionId(7), SignalId(99))];
    assert!(matches!(
        catalog(&valid_signals, &missing_target),
        Err(SignalCatalogError::BindingTargetMissing { .. })
    ));
    let non_boolean = [PropositionBinding::new(PropositionId(7), SignalId(2))];
    assert!(matches!(
        catalog(&valid_signals, &non_boolean),
        Err(SignalCatalogError::NonBooleanPropositionBinding { .. })
    ));

    let too_many_signals = vec![boolean_signal(1, "same"); MAX_SIGNAL_CATALOG_SIGNALS + 1];
    assert!(matches!(
        catalog(&too_many_signals, &[]),
        Err(SignalCatalogError::SignalLimitExceeded { .. })
    ));
    let too_many_bindings = vec![
        PropositionBinding::new(PropositionId(1), SignalId(1));
        MAX_SIGNAL_CATALOG_BINDINGS + 1
    ];
    assert!(matches!(
        catalog(&valid_signals, &too_many_bindings),
        Err(SignalCatalogError::BindingLimitExceeded { .. })
    ));

    let names: Vec<_> = (0..MAX_SIGNAL_CATALOG_SIGNALS)
        .map(|index| format!("signal-{index:06}"))
        .collect();
    let maximum_signals: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(index, name)| boolean_signal(index as u32, name))
        .collect();
    let maximum_bindings: Vec<_> = (0..MAX_SIGNAL_CATALOG_BINDINGS)
        .map(|index| PropositionBinding::new(PropositionId(index as u32), SignalId(index as u32)))
        .collect();
    assert!(catalog(&maximum_signals, &maximum_bindings).is_ok());

    let invalid_fixture = include_bytes!("fixtures/invalid-nonboolean-binding.json");
    assert!(serde_json::from_slice::<SignalCatalogDocument>(invalid_fixture).is_err());
}

// Trace: TC-029, FR-007-AC-2
#[test]
fn catalog_wire_refuses_unknown_and_over_limit_forms() {
    let valid: serde_json::Value =
        serde_json::from_slice(include_bytes!("fixtures/valid-signal-catalog.json")).unwrap();
    assert!(serde_json::from_value::<SignalCatalogDocument>(valid.clone()).is_ok());

    for mutation in [
        "version",
        "domain",
        "integer-bounds",
        "decimal-bounds",
        "decimal-scale",
        "document-field",
        "signal-field",
        "binding-field",
    ] {
        let mut candidate = valid.clone();
        match mutation {
            "version" => candidate["schema_version"] = "tl-syntax.signal-catalog/v2".into(),
            "domain" => candidate["signals"][0]["domain"]["kind"] = "opaque".into(),
            "integer-bounds" => candidate["signals"][1]["domain"]["minimum"] = 9.into(),
            "decimal-bounds" => {
                candidate["signals"][2]["domain"]["minimum_coefficient"] = 1_251.into()
            }
            "decimal-scale" => candidate["signals"][2]["domain"]["scale"] = 19.into(),
            "document-field" => candidate["unexpected"] = true.into(),
            "signal-field" => candidate["signals"][0]["unexpected"] = true.into(),
            "binding-field" => candidate["bindings"][0]["unexpected"] = true.into(),
            _ => unreachable!(),
        }
        assert!(
            serde_json::from_value::<SignalCatalogDocument>(candidate).is_err(),
            "wire mutation {mutation} was accepted"
        );
    }

    let repeated_signal = r#"{"id":1,"name":"ready","domain":{"kind":"boolean"}}"#;
    let signals = std::iter::repeat(repeated_signal)
        .take(MAX_SIGNAL_CATALOG_SIGNALS + 1)
        .collect::<Vec<_>>()
        .join(",");
    let oversized = format!(
        r#"{{"schema_version":"tl-syntax.signal-catalog/v1","signals":[{signals}],"bindings":[]}}"#
    );
    let error = serde_json::from_str::<SignalCatalogDocument>(&oversized).unwrap_err();
    assert!(error.to_string().contains("wire limit"));

    let repeated_binding = r#"{"proposition":1,"signal":1}"#;
    let bindings = std::iter::repeat(repeated_binding)
        .take(MAX_SIGNAL_CATALOG_BINDINGS + 1)
        .collect::<Vec<_>>()
        .join(",");
    let oversized = format!(
        r#"{{"schema_version":"tl-syntax.signal-catalog/v1","signals":[],"bindings":[{bindings}]}}"#
    );
    let error = serde_json::from_str::<SignalCatalogDocument>(&oversized).unwrap_err();
    assert!(error.to_string().contains("wire limit"));
}

// Trace: TC-030, FR-007-AC-3, StR-003-VC-1
#[test]
fn formula_binding_visits_propositions_in_node_order() {
    let nodes = [
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(7),
        }),
        Node::new(NodeKind::Proposition {
            proposition: PropositionId(2),
        }),
        Node::new(NodeKind::And {
            left: NodeId(0),
            right: NodeId(1),
        }),
    ];
    let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(2), &nodes).unwrap();
    let signals = [boolean_signal(1, "two"), boolean_signal(2, "seven")];
    let incomplete = [PropositionBinding::new(PropositionId(2), SignalId(1))];
    let incomplete_catalog = catalog(&signals, &incomplete).unwrap();
    assert_eq!(
        incomplete_catalog.bind_formula(formula).unwrap_err(),
        FormulaBindingError::MissingPropositionBinding {
            proposition: PropositionId(7)
        }
    );

    let complete = [
        PropositionBinding::new(PropositionId(2), SignalId(1)),
        PropositionBinding::new(PropositionId(7), SignalId(2)),
    ];
    let catalog = catalog(&signals, &complete).unwrap();
    let bound = catalog.bind_formula(formula).unwrap();
    assert_eq!(bound.formula(), formula);
    assert_eq!(
        bound
            .signal_for_proposition(PropositionId(7))
            .unwrap()
            .name(),
        "seven"
    );
    assert_eq!(bound.catalog().signal_count(), 2);
}

// Trace: TC-031, FR-007-AC-4, StR-003-VC-2, NFR-002-AC-5
#[test]
fn requirement_context_is_complete_exact_or_explicitly_absent() {
    let bytes = include_bytes!("fixtures/valid-requirement-context.json");
    let document: RequirementContextDocument = serde_json::from_slice(bytes).unwrap();
    assert_eq!(
        document.schema_version(),
        RequirementContextSchemaVersion::V1
    );
    assert_eq!(
        document.schema_version().as_str(),
        "tl-syntax.requirement-context/v1"
    );
    let context = document.validate().unwrap();
    assert_eq!(context.requirement_id(), "agent-ix/config/FR-006");
    assert_eq!(context.requirement_revision(), "17");
    assert_eq!(context.clause_id(), "AC-1");
    assert_eq!(context.anchor(), "handler.overlay.accepted");
    assert_eq!(context.source_span(), SourceSpan::new(128, 196).unwrap());

    let copied = RequirementContextDocument::from_context(context);
    assert_eq!(copied, document);
    assert_eq!(
        serde_json::to_vec(&copied).unwrap(),
        serde_json::to_vec(&document).unwrap()
    );
    assert_eq!(
        serde_json::to_string(&document).unwrap(),
        r#"{"schema_version":"tl-syntax.requirement-context/v1","requirement_id":"agent-ix/config/FR-006","requirement_revision":"17","clause_id":"AC-1","anchor":"handler.overlay.accepted","source_span":{"start":128,"end":196}}"#
    );

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Consumer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context: Option<RequirementContextDocument>,
    }
    let absent: Consumer = serde_json::from_str("{}").unwrap();
    assert!(absent.context.is_none());
    assert_eq!(serde_json::to_string(&absent).unwrap(), "{}");

    assert!(
        serde_json::from_slice::<RequirementContextDocument>(include_bytes!(
            "fixtures/invalid-partial-context.json"
        ))
        .is_err()
    );
}

// Trace: TC-031, FR-007-AC-4, StR-003-VC-2
#[test]
fn every_context_refusal_has_a_neighboring_positive_control() {
    let span = SourceSpan::new(1, 2).unwrap();
    assert!(RequirementContext::new("REQ", "1", "AC", "handler", span).is_ok());

    let empty_cases = [
        (
            RequirementContextField::RequirementId,
            "",
            "1",
            "AC",
            "handler",
        ),
        (
            RequirementContextField::RequirementRevision,
            "REQ",
            "",
            "AC",
            "handler",
        ),
        (RequirementContextField::ClauseId, "REQ", "1", "", "handler"),
        (RequirementContextField::Anchor, "REQ", "1", "AC", ""),
    ];
    for (field, requirement, revision, clause, anchor) in empty_cases {
        assert_eq!(
            RequirementContext::new(requirement, revision, clause, anchor, span).unwrap_err(),
            RequirementContextError::EmptyField { field }
        );
    }

    let oversized = "x".repeat(MAX_REQUIREMENT_CONTEXT_FIELD_BYTES + 1);
    let oversized_cases = [
        (
            RequirementContextField::RequirementId,
            RequirementContext::new(&oversized, "1", "AC", "handler", span),
        ),
        (
            RequirementContextField::RequirementRevision,
            RequirementContext::new("REQ", &oversized, "AC", "handler", span),
        ),
        (
            RequirementContextField::ClauseId,
            RequirementContext::new("REQ", "1", &oversized, "handler", span),
        ),
        (
            RequirementContextField::Anchor,
            RequirementContext::new("REQ", "1", "AC", &oversized, span),
        ),
    ];
    for (field, result) in oversized_cases {
        assert!(matches!(
            result,
            Err(RequirementContextError::FieldTooLong {
                field: observed,
                ..
            }) if observed == field
        ));
    }

    let valid: serde_json::Value =
        serde_json::from_slice(include_bytes!("fixtures/valid-requirement-context.json")).unwrap();
    for field in [
        "requirement_id",
        "requirement_revision",
        "clause_id",
        "anchor",
        "source_span",
    ] {
        let mut candidate = valid.clone();
        candidate.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<RequirementContextDocument>(candidate).is_err(),
            "missing context field {field} was accepted"
        );
    }
    for mutation in [
        "version",
        "unknown-field",
        "unknown-span-field",
        "inverted-span",
    ] {
        let mut candidate = valid.clone();
        match mutation {
            "version" => candidate["schema_version"] = "tl-syntax.requirement-context/v2".into(),
            "unknown-field" => candidate["unexpected"] = true.into(),
            "unknown-span-field" => candidate["source_span"]["unexpected"] = true.into(),
            "inverted-span" => {
                candidate["source_span"] = serde_json::json!({
                    "start": 196,
                    "end": 128
                })
            }
            _ => unreachable!(),
        }
        assert!(
            serde_json::from_value::<RequirementContextDocument>(candidate).is_err(),
            "context mutation {mutation} was accepted"
        );
    }
}
