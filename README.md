# tl-syntax

[![Discord](https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white)](https://discord.gg/k8DVhuYBR2)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.98.1%2B-orange?logo=rust)](Cargo.toml)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen)](docs/features.md)

tl-syntax is the shared vocabulary of the TL family: the formula graph, the
semantic profiles and the signal catalog for discrete bounded Mission-time Linear
Temporal Logic (MLTL). Use it to build a formula, check that it is valid under a
profile, and bind its propositions to typed signals.

Formulas and signal catalogs read and write as strict versioned JSON with a
canonical form and a content identity. The default build needs neither `std` nor
an allocator.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/hero-dark.svg">
  <img alt="A node table of 2 nodes and a semantic profile, mltl.closed-trace/v1, go into Formula::new, which returns a borrowed Formula or the first FormulaError. A signal catalog of 3 signals and 1 binding goes into SignalCatalog::new, which returns a SignalCatalog or the first SignalCatalogError. bind_formula joins the two into a BoundFormula when every proposition has a binding to a Boolean signal. The crate defines 4 semantic profiles." src="docs/images/hero-light.svg">
</picture>

## Getting started

tl-syntax is not on crates.io. Depend on it through git:

```toml
[dependencies]
tl-syntax = { git = "https://github.com/agent-ix/tl-syntax" }
```

Add `rev = "<commit>"` to pin a commit. It needs the Rust version in `rust-version` in `Cargo.toml`, or newer.

The crate is `no_std` by default. Enable `alloc` for the owned document types and `serde` for the strict JSON readers and writers, for example `features = ["serde"]`.

To build from a clone:

```sh
git clone https://github.com/agent-ix/tl-syntax.git
cd tl-syntax
cargo build --release
```

## Usage

This builds a formula, binds it to a signal catalog and looks up the signal for a
proposition.

```rust
use tl_syntax::{
    Formula, Interval, Node, NodeId, NodeKind, PropositionBinding, PropositionId,
    SemanticProfile, SignalCatalog, SignalDeclaration, SignalDomain, SignalId,
};

let nodes = [
    Node::new(NodeKind::Proposition { proposition: PropositionId(7) }),
    Node::new(NodeKind::Future {
        interval: Interval::new(1, 3).unwrap(),
        operand: NodeId(0),
    }),
];
let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();

let signals = [SignalDeclaration::new(SignalId(1), "request_ready", SignalDomain::Boolean)];
let bindings = [PropositionBinding::new(PropositionId(7), SignalId(1))];
let mut scratch = [0_u32; 1];
let catalog = SignalCatalog::new(&signals, &bindings, &mut scratch).unwrap();

let bound = catalog.bind_formula(formula).unwrap();
let signal = bound.signal_for_proposition(PropositionId(7)).unwrap();
assert_eq!(signal.name(), "request_ready");
```

The formula is `F[1,3] p7`. More examples, including JSON documents, are in
[docs/examples.md](docs/examples.md).

## Reference

| Module | What it holds |
| --- | --- |
| `formula::graph` | `Formula`, `Node`, `NodeKind`, `Interval`, `SourceSpan` and `FormulaError`. |
| `formula::profile` | `SemanticProfile`, `TemporalFamily` and the closed past-operator catalog. |
| `formula::document` | `FormulaDocument` for schemas `tl-syntax.formula/v1` and `v2` (feature `alloc`, strict readers with `serde`). |
| `formula::infinite` | The unbounded formula edition, `tl-syntax.formula-unbounded/v1`, and the infinite-trace profile. |
| `formula::trace` | Lasso traces, partial valuations and fairness premises (feature `alloc`). |
| `formula::liveness` | Routing of liveness requests to an optional backend (feature `alloc`). |
| `signal::{domain, catalog, binding}` | Boolean, integer and fixed-decimal signals, catalogs, and proposition bindings. |
| `signal::document` | Owned signal-catalog and proposition-map documents (feature `alloc`). |
| `contracts::{limits, manifest}` | Resource limits for readers, and the past-profile implementation manifest check. |
| `future` (crate root) | Lowering of derived operators `W` and `M` to primitive nodes, with typed refusals. |

| Feature | Default | Adds |
| --- | ---: | --- |
| none | yes | Borrowed formulas, signal catalogs, bindings and requirement contexts, with no allocation. |
| `alloc` | no | Owned documents. |
| `serde` | no | JSON serialization and strict readers, schema constants, content identity. Implies `alloc`. |

See [docs/api-reference.md](docs/api-reference.md) and
[docs/features.md](docs/features.md).

## The TL stack

tl-syntax holds the types every other TL crate shares.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/tl-stack-dark.svg">
  <img alt="The TL crates. tl-syntax defines the formula graph, profiles and signal catalog, and tl-parse, tl-oracle, tl-mltl, tl-rewrite and quire-mltl all depend on it. tl-rewrite depends on tl-mltl. quire-mltl depends on tl-mltl and quire-observation, and is the single bridge to Quire. Dashed lines are development-only: tl-mltl and tl-rewrite test against tl-oracle, and tl-rewrite tests against tl-parse." src="docs/images/tl-stack-light.svg">
</picture>

## Building and testing

```sh
make test          # cargo test --all-features
make ci            # format, features, clippy, tests, corpus, conformance, deny, fuzz build, spec, MSRV, rustdoc
make check-corpus  # validate the corpus schemas and derived oracles
```

Wire-decoder fuzzing is described in [docs/wire-formats.md](docs/wire-formats.md).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [CLA.md](CLA.md). The change history is in
[CHANGELOG.md](CHANGELOG.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or the
[MIT license](LICENSE-MIT) at your option.
