# Interval arithmetic proof (TC-185 scoped evidence)

Run on 2026-09-22 against the source in this revision:

```text
source src/formula/graph.rs SHA-256 e2ab26c19565e9097b28c1adffa601a833e7437f2eec357cbbc38321a90786df
manifest Cargo.toml SHA-256 ae75ae8cb920a384bf845e6ae2566f726db0fc007dffedae7f9637afe3c24aa8
Kani 0.68.0; CBMC 6.11.0; host aarch64-apple-darwin
ordinary/MSRV rustc 1.98.1
```

Command:

```sh
cargo kani --lib \
  --harness formula::graph::kani_proofs::interval_cardinality_matches_wide_arithmetic \
  --exact --unwind 4
```

The symbolic domain is every pair of `u32` endpoints. There are no
assumptions. The harness checks admission, endpoint preservation, and inclusive
cardinality against wider `u64` arithmetic. Kani reported 0 of 96 checks
failed and one successfully verified harness. Default overflow, memory-safety,
unwinding, and assertion-reachability checks remained enabled. The harness has
no loop or recursion; the explicit unwind bound is four. The full transcript
was retained at `/private/tmp/tl-syntax-kani-interval.log`, SHA-256
`351f74e39c61e81521327fa36f34e25f2b9a3fa1dd037019bd0ba7c1e26b1e11`.

This establishes only the stated interval arithmetic claim. It does not prove
whole-formula validation or temporal semantics. TC-185's campaign-wide proof
population and TC-186's negative control remain separate obligations.
