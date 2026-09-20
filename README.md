# TL Syntax

[![Discord](https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white)](https://discord.gg/6qsdhSPE)

A parser-independent, `no_std` syntax tree and semantic-profile model for
discrete bounded Mission-time Linear Temporal Logic (MLTL).

The crate models propositions, Boolean operators, bounded Future, Globally,
Until, and Release, and the origin-complete past operators Once, Historically,
strong Previous, Since, and Triggered over checked inclusive intervals. Future
and past primitives are admitted only by their matching semantic profiles. It also validates named
Boolean/Integer/fixed-Decimal signal catalogs, direct Boolean proposition
bindings, and optional caller-supplied requirement context. Formulas and
catalogs are borrowed views, so the default API needs neither `std` nor a heap
allocator. Optional owned and serde layers provide strict versioned exchange
documents.

The public semantic modules are `formula::{graph, profile, document}`,
`signal::{domain, catalog, binding, document}`, and
`contracts::{limits, manifest}`. Crate-root re-exports preserve the original
paths.

## Features

| Feature | Default | Adds |
|---|---:|---|
| core | yes | Checked values plus allocation-free borrowed formulas, signal catalogs, formula bindings, and requirement contexts |
| `alloc` | no | Owned formula, proposition-map, signal-catalog, and requirement-context documents |
| `serde` | no | Serialization for versioned owned documents; implies `alloc` |

## Example

```rust
use tl_syntax::{Formula, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile};

let nodes = [
    Node::new(NodeKind::Proposition { proposition: PropositionId(0) }),
    Node::new(NodeKind::Future {
        interval: Interval::new(1, 3).unwrap(),
        operand: NodeId(0),
    }),
];
let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
assert_eq!(formula.root(), NodeId(1));
```

Signals use identities distinct from proposition identities. A direct binding
is valid only when its target signal is Boolean; numeric signals remain typed
inputs for an explicit predicate-lowering layer outside this crate.

## Wire formats and corpus

The `serde` feature exposes formula schemas `tl-syntax.formula/v1` and
`tl-syntax.formula/v2`, proposition-
map schema `tl-syntax.proposition-map/v1`, signal-catalog schema
`tl-syntax.signal-catalog/v1`, requirement-context schema
`tl-syntax.requirement-context/v1`, and the closed set of supported semantic-
profile identifiers. Formula-v1 remains future-only and byte-compatible;
formula-v2 adds the closed `tl-syntax.past-operators/v1` node catalog and
`mltl.origin-complete-history/v1`. V1 documents upgrade losslessly, while v2
down-conversion refuses a past profile. Unknown versions and fields are rejected. The existing
formula/proposition JSON Schemas, fixtures, and expected horizon/closed-trace
results live in [`corpus/`](corpus/README.md); that v1 corpus is unchanged by
the new separate documents. The signal-catalog schema is the separately pinned
[`spec/signal-catalog-v1.schema.json`](spec/signal-catalog-v1.schema.json)
artifact exposed by the `serde` API. Downstream temporal crates must pin and
report `tl-syntax-corpus/v1`.

Each formula, signal-catalog, and proposition-map owner document exposes
`from_json_bytes(bytes, SyntaxArtifactLimits)`. These readers intersect caller
limits with immutable owner maxima, reject noncanonical or trailing JSON, and
return only fully validated values. The same API publishes exact schema bytes
and lowercase SHA-256 digest constants. Validated documents can emit canonical
bytes and a contract-domain-separated content identity.

## Build

```bash
make test
make ci
```

## Manual wire fuzzing

The versioned document decode boundary has a Rust fuzz target. It is intentionally
manual-only and is not part of `make ci` or hosted CI:

```bash
cargo +nightly fuzz run wire_decode -- -runs=100
```

Successful decodes must still pass public validation; malformed bytes are
expected to be rejected. The checked-in conformance corpus remains the
authoritative compatibility corpus. Generated fuzz inputs are local campaign
scratch, not replacement fixtures or assurance evidence.

## Development status

This crate is being developed spec-first. Its public API is not stable yet, and
registry publication is disabled until the v0.1 assurance review is complete.

Agent-assisted contributions are reviewed under the same requirements,
testing, provenance, and human release gates as every other contribution.

Under program governance, `tl-syntax` is a linked-runtime component. Its source
release provides reusable qualification support only: it does not validate,
accredit, or certify a consuming project. Candidate evidence is sealed and
retained by Quoin, and review and release decisions are left to the named human
authority.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
