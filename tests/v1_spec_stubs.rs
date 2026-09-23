//! TL-207 test-first lane. These ignored tests are intentionally red until
//! the accepted V1 syntax contracts are implemented. Run with
//! `cargo test --test v1_spec_stubs -- --ignored` to observe the red baseline.

macro_rules! pending_v1_case {
    ($name:ident) => {
        #[test]
        #[ignore = "V1 implementation pending; run the named ignored lane"]
        fn $name() {
            panic!("V1 criterion has no implementation yet");
        }
    };
}

// Trace: TC-144, FR-289-AC-1, FR-289-AC-2, FR-289-AC-4
pending_v1_case!(tc_144_unbounded_future_and_past_admission);
// Trace: TC-145, FR-289-AC-2, FR-289-AC-3
pending_v1_case!(tc_145_unbounded_wire_round_trip);
// Trace: TC-146, FR-290-AC-1, FR-290-AC-2, FR-290-AC-3
pending_v1_case!(tc_146_liveness_registration);
// Trace: TC-147, FR-291-AC-1, FR-291-AC-2, FR-291-AC-3
pending_v1_case!(tc_147_liveness_evidence_attribution);

// Trace: TC-148, FR-020-AC-1
pending_v1_case!(tc_148_distinct_profile_identity);
// Trace: TC-149, FR-020-AC-2
pending_v1_case!(tc_149_exact_event_position_clock);
// Trace: TC-150, FR-020-AC-3
pending_v1_case!(tc_150_existing_wire_bytes);
// Trace: TC-151, FR-021-AC-1
pending_v1_case!(tc_151_fairness_round_trip);
// Trace: TC-152, FR-021-AC-2
pending_v1_case!(tc_152_fairness_refusal_axes);
// Trace: TC-153, FR-021-AC-3
pending_v1_case!(tc_153_no_prefix_fairness_proof);
// Trace: TC-154, FR-022-AC-1
pending_v1_case!(tc_154_lasso_round_trip);
// Trace: TC-155, FR-022-AC-2
pending_v1_case!(tc_155_lasso_refusals);
// Trace: TC-156, FR-022-AC-3
pending_v1_case!(tc_156_lasso_unroll);
// Trace: TC-157, FR-023-AC-1
pending_v1_case!(tc_157_partial_states);
// Trace: TC-158, FR-023-AC-2
pending_v1_case!(tc_158_partial_entry_refusals);
// Trace: TC-159, FR-023-AC-3
pending_v1_case!(tc_159_missing_conflicting_distinct);
// Trace: TC-160, FR-024-AC-1
pending_v1_case!(tc_160_corpus_manifest);
// Trace: TC-161, FR-024-AC-2
pending_v1_case!(tc_161_corpus_mutation_probe);
// Trace: TC-162, FR-024-AC-3
pending_v1_case!(tc_162_downstream_corpus_pin);
