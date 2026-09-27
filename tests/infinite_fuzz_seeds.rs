#![cfg(feature = "serde")]

use std::{fs, path::Path};

use sha2::{Digest, Sha256};
use tl_syntax::{
    InfiniteFormulaDocument, LassoTraceDocument, PartialValuation, SyntaxArtifactLimits,
};

// Trace: TC-181, FR-046-AC-1. Fuzz seeds must reach successful strict owner
// readers as well as a malformed refusal, so the target cannot be vacuous.
#[test]
fn infinite_wire_fuzz_seeds_reach_the_intended_reader_paths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fuzz/corpus/infinite_wire_decode");
    let checksum_lines = fs::read_to_string(root.join("SHA256SUMS")).unwrap();
    let mut seen = 0;
    for line in checksum_lines.lines() {
        let (expected, name) = line.split_once("  ").unwrap();
        let bytes = fs::read(root.join(name)).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), expected, "{name}");
        let limits = SyntaxArtifactLimits::default();
        match name {
            "formula.json" => {
                let formula = InfiniteFormulaDocument::from_json_bytes(&bytes, limits).unwrap();
                assert_eq!(formula.canonical_json_bytes().unwrap(), bytes);
            }
            "lasso.json" => {
                let lasso = LassoTraceDocument::from_json_bytes(&bytes, limits).unwrap();
                assert_eq!(lasso.canonical_json_bytes().unwrap(), bytes);
            }
            "valuation.json" => {
                let valuation = PartialValuation::from_json_bytes(&bytes, limits).unwrap();
                assert_eq!(valuation.canonical_json_bytes().unwrap(), bytes);
            }
            "malformed.json" => {
                assert!(InfiniteFormulaDocument::from_json_bytes(&bytes, limits).is_err());
            }
            unexpected => panic!("unexpected fuzz seed {unexpected}"),
        }
        seen += 1;
    }
    assert_eq!(seen, 4);
}
