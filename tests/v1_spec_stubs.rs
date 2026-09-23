//! Remaining TL-207 test-first lane. The corpus inspection case stays ignored until verified.
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
