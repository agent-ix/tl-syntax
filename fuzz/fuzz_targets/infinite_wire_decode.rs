#![no_main]

use libfuzzer_sys::fuzz_target;
use tl_syntax::{
    InfiniteFormulaDocument, LassoTraceDocument, PartialValuation, SyntaxArtifactLimits,
};

// Trace: TC-181, FR-046-AC-1. Each successful strict read must survive the
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
