# Features

| Feature | Default | Enables | Dependencies added |
| --- | ---: | --- | --- |
| none | yes | Borrowed, allocation-free formulas, signal catalogs, bindings, requirement contexts, the unbounded formula graph types, future lowering, and the resource-limit types. | none |
| `alloc` | no | Owned documents (`FormulaDocument`, `SignalCatalogDocument`, `PropositionMapDocument`, `RequirementContextDocument`, `InfiniteFormulaDocument`), lasso traces, partial valuations, fairness premises and liveness routing. | none |
| `serde` | no | JSON serialization, strict byte readers, canonical bytes, content identity, schema constants and the past-profile manifest check. Implies `alloc`. | `serde`, `serde_json`, `sha2` (all `no_std` with `alloc`) |

## no_std

The crate is `#![no_std]` and `#![forbid(unsafe_code)]`. The `alloc` feature uses the `alloc` crate and never `std`. `examples/default_dependencies.rs` checks that the default build has no dependencies: `cargo tree` for the default features must print only `tl-syntax`.

`tests/feature_boundary.rs` runs `make check-features` and `make check-default-dependencies` in an isolated target directory, so the feature matrix is part of `cargo test`.

## Checking each combination

`make check-features` runs `cargo check --lib` with no features, with `alloc`, with `serde`, and with all features.

## Rust version

The minimum supported Rust version is 1.98.1, set in `Cargo.toml` and `rust-toolchain.toml`. `make msrv` runs the tests with that toolchain.
