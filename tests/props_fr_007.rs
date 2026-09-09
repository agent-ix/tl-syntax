#![cfg(feature = "serde")]

use proptest::prelude::*;
use tl_syntax::{RequirementContext, RequirementContextDocument, SourceSpan};

fn context_field() -> impl Strategy<Value = String> {
    "[A-Za-z0-9._/-]{1,64}"
}

proptest! {
    /// Trace: FR-007-AC-4 — complete caller contexts round-trip exactly.
    /// spec-correctness: row=FR-007-AC-4 property=round-trip extraction=extractable origin=regex
    #[test]
    fn fr_007_ac_4_complete_context_round_trips_exactly(
        requirement_id in context_field(),
        requirement_revision in context_field(),
        clause_id in context_field(),
        anchor in context_field(),
        first_offset in any::<u32>(),
        second_offset in any::<u32>(),
    ) {
        let start = first_offset.min(second_offset);
        let end = first_offset.max(second_offset);
        let context = RequirementContext::new(
            &requirement_id,
            &requirement_revision,
            &clause_id,
            &anchor,
            SourceSpan::new(start, end).unwrap(),
        )
        .unwrap();
        let document = RequirementContextDocument::from_context(context);
        let decoded: RequirementContextDocument =
            serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();

        prop_assert_eq!(&decoded, &document);
        prop_assert_eq!(decoded.validate().unwrap(), context);
    }

    /// Trace: StR-003-VC-2 — supplied requirement provenance survives owned and borrowed boundaries.
    /// spec-correctness: row=StR-003-VC-2 property=universal extraction=extractable origin=regex
    #[test]
    fn str_003_vc_2_context_preserves_each_supplied_identity(
        requirement_id in context_field(),
        requirement_revision in context_field(),
        clause_id in context_field(),
        anchor in context_field(),
        first_offset in any::<u32>(),
        second_offset in any::<u32>(),
    ) {
        let start = first_offset.min(second_offset);
        let end = first_offset.max(second_offset);
        let span = SourceSpan::new(start, end).unwrap();
        let context = RequirementContext::new(
            &requirement_id,
            &requirement_revision,
            &clause_id,
            &anchor,
            span,
        )
        .unwrap();
        let document = RequirementContextDocument::from_context(context);
        let recovered = document.validate().unwrap();

        prop_assert_eq!(recovered.requirement_id(), requirement_id);
        prop_assert_eq!(recovered.requirement_revision(), requirement_revision);
        prop_assert_eq!(recovered.clause_id(), clause_id);
        prop_assert_eq!(recovered.anchor(), anchor);
        prop_assert_eq!(recovered.source_span(), span);
    }
}
