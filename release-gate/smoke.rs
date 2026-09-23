//! Compiled outside every TL workspace against the four exact candidate Git commits.

use tl_mltl::{
    evaluate_closed, map_to_c2po, EvaluationLimits, MappingSourceIdentity, MappingSourceState,
    TruthValue,
};
use tl_parse::{format_document, parse, FormatLimits, ParseLimits};
use tl_rewrite::{rewrite, RewriteOptions, RewriteStatus};
use tl_syntax::{
    Formula, Node, NodeId, NodeKind, OwnedSignalDeclaration, PropositionBinding, PropositionId,
    SemanticProfile, SignalCatalogDocument, SignalDomain, SignalId,
};

fn main() {
    let parsed = parse("p0", SemanticProfile::ClosedTraceV1, ParseLimits::default());
    let document = parsed.document.expect("public parser admitted p0");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(
        format_document(&document, FormatLimits::default()).text.as_deref(),
        Some("p0")
    );

    let rewritten = rewrite(&document, "consumer", RewriteOptions::default(), "candidate");
    assert_eq!(rewritten.status, RewriteStatus::Unchanged);
    assert!(rewritten.output.is_some());

    let signal_catalog = SignalCatalogDocument::new(
        vec![OwnedSignalDeclaration::new(
            SignalId(1),
            "p".to_owned(),
            SignalDomain::Boolean,
        )],
        vec![PropositionBinding::new(PropositionId(0), SignalId(1))],
    )
    .expect("valid public signal catalog");
    signal_catalog
        .validate()
        .expect("validated catalog")
        .bind_formula(document.validate().expect("validated formula"))
        .expect("formula binds to p");

    let evaluated = evaluate_closed(
        document.validate().expect("validated formula"),
        "consumer",
        &[vec![PropositionId(0)]],
        "one-true-position",
        EvaluationLimits::default(),
    )
    .expect("public evaluator accepted the trace");
    assert_eq!(evaluated.verdict, TruthValue::True);

    let nodes = [Node::new(NodeKind::Proposition {
        proposition: PropositionId(0),
    })];
    let online = Formula::new(SemanticProfile::OnlinePrefixV1, NodeId(0), &nodes)
        .expect("public online formula");
    let mapping = map_to_c2po(
        online,
        "consumer-online",
        b"p0",
        MappingSourceIdentity {
            revision: "candidate".to_owned(),
            state: MappingSourceState::Clean,
        },
        None,
        100,
    )
    .expect("public C2PO mapping");
    assert_eq!(mapping.expression, "p0");
}
