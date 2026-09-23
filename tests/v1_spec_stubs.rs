//! Remaining TL-207 and TL-213 test-first lane. These ignored tests are intentionally red
//! until the remaining corpus inspection and release contracts are implemented. Run with
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

// TC-144 through TC-161 now have executing tests in infinite_formula,
// infinite_trace, and infinite_trace_corpus.
// Trace: TC-162, FR-024-AC-3
pending_v1_case!(tc_162_downstream_corpus_pin);

// Trace: TC-170, FR-029-AC-1
pending_v1_case!(tc_170_exact_production_pins);
// Trace: TC-171, FR-029-AC-2
pending_v1_case!(tc_171_common_msrv_and_tag_reachability);
// Trace: TC-172, FR-029-AC-3
pending_v1_case!(tc_172_historical_lane_isolation);
// Trace: TC-173, FR-030-AC-1
pending_v1_case!(tc_173_cross_crate_corpus_replay);
// Trace: TC-174, FR-030-AC-2
pending_v1_case!(tc_174_corpus_replay_mutations);
// Trace: TC-175, FR-031-AC-1
pending_v1_case!(tc_175_api_break_migration_notes);
// Trace: TC-176, FR-031-AC-2
pending_v1_case!(tc_176_existing_wire_bytes);
// Trace: TC-177, FR-032-AC-1
pending_v1_case!(tc_177_downstream_smoke_operations);
// Trace: TC-178, FR-032-AC-2
pending_v1_case!(tc_178_msrv_and_stable_consumer);
// Trace: TC-179, FR-033-AC-1, FR-033-AC-2, NFR-007-AC-1
pending_v1_case!(tc_179_human_decision_and_bound_evidence);
