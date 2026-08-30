---
id: NFR-001
title: Preserve the no-std feature boundary
type: NFR
quality_attribute: portability
relationships:
  - target: ix://agent-ix/tl-syntax/StR-001
    type: traces_to
---

# NFR-001: Preserve the no-std feature boundary

## Statement

The crate shall compile without the standard library and without allocation
when default features are disabled or left unchanged.

## Scope

The requirement covers the public interval, identity, node, profile, borrowed
formula, and validation APIs. Owned documents are isolated behind `alloc`; serde
support is isolated behind `serde` and implies `alloc`.

## Rationale

This preserves adoption by embedded consumers while still offering ergonomic
owned formats to host tools.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Default dependency count | 0 | 0 | Cargo metadata inspection |
| Required default features | 0 | 0 | `cargo check --no-default-features` |

## Verification

CI compiles the crate without default features and separately compiles every
feature combination used by downstream consumers.

## Dependencies

Supports [StR-001](./StR-001-embedded-consumers.md).
