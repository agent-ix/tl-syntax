---
id: NFR-002
title: Deterministic and integrity-preserving artifacts
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/StR-002
    type: traces_to
---

# NFR-002: Deterministic and integrity-preserving artifacts

## Statement

For identical validated inputs, the crate shall expose identical equality,
ordering, serialization structure, corpus identities, and validation outcomes
on every supported platform.

## Scope

The requirement covers all public syntax values and checked-in JSON artifacts.

## Rationale

Downstream differential testing and retained assurance evidence depend on stable
identities and reproducible inputs.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Nondeterministic test failures | 0 | 0 | Repeated unit and integration tests |
| Unversioned serialized document kinds | 0 | 0 | Schema inspection |

## Verification

Requirement-tagged tests compare values, wire strings, corpus metadata, and
validation outcomes using fixed checked-in inputs.

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md).
