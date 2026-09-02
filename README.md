# TL Syntax

A parser-independent, `no_std` syntax tree and semantic-profile model for
discrete bounded Mission-time Linear Temporal Logic (MLTL).

The crate models propositions, Boolean operators, and bounded Future, Globally,
Until, and Release over checked inclusive intervals. Formulas are borrowed,
topologically indexed node tables, so the default API needs neither `std` nor a
heap allocator. Optional owned and serde layers provide strict versioned
exchange documents.

## Features

| Feature | Default | Adds |
|---|---:|---|
| core | yes | Checked values and allocation-free borrowed formulas |
| `alloc` | no | Owned formula and proposition-map documents |
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

## Wire formats and corpus

The `serde` feature exposes formula schema `tl-syntax.formula/v1`, proposition-
map schema `tl-syntax.proposition-map/v1`, and the closed set of supported
semantic-profile identifiers. Unknown versions are rejected. JSON Schemas,
fixtures, and expected horizon/closed-trace results live in [`corpus/`](corpus/README.md).
Downstream temporal crates must pin and report `tl-syntax-corpus/v1`.

## Build

```bash
make test
make ci
```

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
