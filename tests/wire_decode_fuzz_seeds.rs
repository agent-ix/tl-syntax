#![cfg(feature = "serde")]

use std::{fs, path::Path};

use tl_syntax::{FormulaDocument, PropositionMapDocument};

// Trace: TC-036, FR-004-AC-5. The seeds must reach both the accepting and the
// refusing side of each public decoder, so the target cannot be vacuous.
#[test]
fn wire_decode_seeds_reach_accepting_and_refusing_decoder_paths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fuzz/corpus/wire_decode");
    let (mut formulas, mut maps, mut refused) = (0, 0, 0);
    for entry in fs::read_dir(&root).unwrap() {
        let bytes = fs::read(entry.unwrap().path()).unwrap();
        let formula = serde_json::from_slice::<FormulaDocument>(&bytes);
        let map = serde_json::from_slice::<PropositionMapDocument>(&bytes);
        if let Ok(document) = &formula {
            document.validate().unwrap();
            formulas += 1;
        }
        if let Ok(document) = &map {
            document.validate().unwrap();
            maps += 1;
        }
        refused += usize::from(formula.is_err() && map.is_err());
    }
    assert!(formulas >= 3, "formula seeds accepted: {formulas}");
    assert!(maps >= 1, "proposition-map seeds accepted: {maps}");
    assert!(refused >= 3, "refused seeds: {refused}");
}
