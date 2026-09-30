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
on every supported platform. Identical validated signal catalogs and caller
source contexts shall likewise retain identical order and wire bytes.

## Scope

The requirement covers public syntax values and checked-in domain JSON
artifacts.

## Rationale

Downstream differential testing depends on stable identities and reproducible
inputs.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Nondeterministic test failures | 0 | 0 | Test |
| Unversioned serialized document kinds | 0 | 0 | Inspection |

## Verification

Requirement-tagged tests compare values, wire strings, corpus metadata, and
validation outcomes using fixed checked-in inputs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-002-AC-1 | Repeated comparisons over identical values produce one stable order. | Test (TC-005) |
| NFR-002-AC-2 | Every checked-in serialized document names a v1 schema and supported profile where applicable. | Test (TC-014) |
| NFR-002-AC-5 | Identical signal declarations, proposition bindings, and requirement contexts produce identical validated ordering and serialized bytes. | Test (TC-027, TC-031) |

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md), and
[FR-007](./FR-007-typed-signal-context.md).
