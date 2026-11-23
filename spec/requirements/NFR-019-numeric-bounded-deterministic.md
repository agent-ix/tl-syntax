---
id: NFR-019
title: Bound and make deterministic every numeric artifact
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-303
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-307
    type: constrains
---

# NFR-019: Bound and make deterministic every numeric artifact

## Statement

The crate shall produce identical predicate-table, aggregation, expansion and
contract-set bytes, typing results and errors for identical inputs on every
supported platform, with the work and memory of every new reader, validator and
expansion bounded by a declared limit checked before allocation.

## Scope

All artifacts of FR-302..FR-311.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Expression nodes per table | 100,000 | 100,000 | Test |
| Expression depth | 256 | 256 | Test |
| Set members | 1,024 | 1,024 | Test |
| Expansion nodes | 100,000 | 100,000 | Test |
| Contracts per set | 10,000 | 10,000 | Test |
| Document bytes / JSON depth | 64 MiB / 64 | 64 MiB / 64 | Test |
| Platform-dependent output | none | none | Test |

## Verification

Limit-and-limit-plus-one tests for every row, and a byte-for-byte replay of the
corpus under the existing MSRV and Miri jobs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-019-AC-1 | Every limit row accepts the limit and refuses limit plus one with a typed error before allocation proportional to the excess. | unit-testing (TC-1531) |
| NFR-019-AC-2 | Corpus replay is byte-identical across repeated runs and targets. | integration-testing (TC-1532) |
