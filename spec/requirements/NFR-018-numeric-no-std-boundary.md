---
id: NFR-018
title: Keep the numeric model inside the no-std feature boundary
type: NFR
quality_attribute: portability
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-302
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-307
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-310
    type: constrains
---

# NFR-018: Keep the numeric model inside the no-std feature boundary

## Statement

When the new expression typing, predicate binding, set expansion core and contract
status function are used through default features, the crate shall compile with
`#![no_std]`, without allocation and with zero dependencies.

## Scope

Borrowed expression validation (caller-owned scratch), predicate binding,
the aggregation expansion core writing into caller slices, and the contract
status function. Owned documents and strings stay behind `alloc`; wire schemas,
readers and serialization stay behind `serde`.

## Rationale

TL-420 requires the core feature set to remain `no_std`; embedded consumers
adopt the borrowed model.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Default dependency count | 0 | 0 | Inspection |
| New default-feature APIs that allocate | 0 | 0 | Test |
| New default features | 0 | 0 | Inspection |
| Owned-document and wire items reachable without `alloc` or `serde` | 0 | 0 | Test |

## Verification

The existing no-std feature matrix and empty-default-dependency gate (TC-019) run
over the new modules and a compile-only example that uses each borrowed API.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-018-AC-1 | The feature matrix builds the new modules under no default features, `alloc`, `serde` and all features, and the default dependency set stays empty. | integration-testing (TC-1530) |
