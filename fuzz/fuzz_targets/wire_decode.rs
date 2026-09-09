#![no_main]

use libfuzzer_sys::fuzz_target;
use tl_syntax::{FormulaDocument, PropositionMapDocument};

// Trace: TC-036, FR-004-AC-5
fuzz_target!(|data: &[u8]| {
    let formula_is_valid_or_rejected = match serde_json::from_slice::<FormulaDocument>(data) {
        Ok(document) => document.validate().is_ok(),
        Err(_) => true,
    };
    assert!(formula_is_valid_or_rejected);

    let proposition_map_is_valid_or_rejected =
        match serde_json::from_slice::<PropositionMapDocument>(data) {
            Ok(document) => document.validate().is_ok(),
            Err(_) => true,
        };
    assert!(proposition_map_is_valid_or_rejected);
});
