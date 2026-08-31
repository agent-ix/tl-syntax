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
| Default dependency count | 0 | 0 | Inspection |
| Required default features | 0 | 0 | compile-time-check |

## Verification

CI compiles the crate without default features and separately compiles every
feature combination used by downstream consumers.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-001-AC-1 | The default feature graph has no normal dependency and exposes the borrowed formula API. | Test (TC-015) and Suite (SUITE-001) |
| NFR-001-AC-2 | The crate compiles with no default feature, alloc only, serde, and all features. | Suite (SUITE-001) |

## Dependencies

Supports [StR-001](./StR-001-embedded-consumers.md).
