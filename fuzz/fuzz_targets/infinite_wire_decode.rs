#![no_main]

use libfuzzer_sys::fuzz_target;
use tl_syntax::{
    FormulaDocument, InfiniteFormulaDocument, InfiniteNodeKind, LassoTraceDocument,
    PartialValuation, SyntaxArtifactLimits, TemporalInterval,
};

fn has_unbounded_interval(formula: &InfiniteFormulaDocument) -> bool {
    use InfiniteNodeKind as K;
    formula.nodes().iter().any(|node| {
        matches!(
            node.kind,
            K::Future { interval, .. }
                | K::Globally { interval, .. }
                | K::Until { interval, .. }
                | K::Release { interval, .. }
                | K::Once { interval, .. }
                | K::Historically { interval, .. }
                | K::Since { interval, .. }
                | K::Triggered { interval, .. }
                if matches!(interval, TemporalInterval::Unbounded(_))
        )
    })
}

// Trace: TC-145, FR-289-AC-3. The bounded readers must refuse `[a,)` even
// when the document is re-labelled with a bounded schema, profile, and clock.
fn assert_bounded_readers_refuse(data: &[u8], limits: SyntaxArtifactLimits) {
    let Ok(serde_json::Value::Object(map)) = serde_json::from_slice::<serde_json::Value>(data)
    else {
        return;
    };
    for schema in ["tl-syntax.formula/v1", "tl-syntax.formula/v2"] {
        let mut relabelled = map.clone();
        relabelled.insert("schema_version".into(), schema.into());
        relabelled.insert("semantic_profile".into(), "mltl.closed-trace/v1".into());
        relabelled.remove("clock");
        let bytes = serde_json::to_vec(&relabelled).unwrap();
        assert!(FormulaDocument::from_json_bytes(&bytes, limits).is_err());
        assert!(serde_json::from_slice::<FormulaDocument>(&bytes).is_err());
    }
}

// Trace: TC-145, TC-154, TC-157, FR-289-AC-2, FR-022-AC-1, FR-023-AC-1. Each successful strict read must survive the
// owner operation and a second strict read without losing canonical identity.
fuzz_target!(|data: &[u8]| {
    let limits = SyntaxArtifactLimits::default();
    if let Ok(formula) = InfiniteFormulaDocument::from_json_bytes(data, limits) {
        let canonical = formula.canonical_json_bytes().unwrap();
        assert_eq!(canonical, data);
        let read_back = InfiniteFormulaDocument::from_json_bytes(&canonical, limits).unwrap();
        assert_eq!(read_back, formula);
        assert_eq!(
            read_back.content_identity().unwrap(),
            formula.content_identity().unwrap()
        );
        if has_unbounded_interval(&formula) {
            assert!(FormulaDocument::from_json_bytes(data, limits).is_err());
            assert_bounded_readers_refuse(data, limits);
        }
    }
    if let Ok(lasso) = LassoTraceDocument::from_json_bytes(data, limits) {
        let canonical = lasso.canonical_json_bytes().unwrap();
        assert_eq!(canonical, data);
        let read_back = LassoTraceDocument::from_json_bytes(&canonical, limits).unwrap();
        assert_eq!(read_back, lasso);
        assert_eq!(
            read_back.content_identity().unwrap(),
            lasso.content_identity().unwrap()
        );
    }
    if let Ok(valuation) = PartialValuation::from_json_bytes(data, limits) {
        let canonical = valuation.canonical_json_bytes().unwrap();
        assert_eq!(canonical, data);
        let read_back = PartialValuation::from_json_bytes(&canonical, limits).unwrap();
        assert_eq!(read_back, valuation);
        assert_eq!(
            read_back.content_identity().unwrap(),
            valuation.content_identity().unwrap()
        );
    }
});
