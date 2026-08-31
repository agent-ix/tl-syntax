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

The requirement covers all public syntax values, checked-in JSON artifacts, and
retained evidence classifications.

## Rationale

Downstream differential testing and retained assurance evidence depend on stable
identities and reproducible inputs.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Nondeterministic test failures | 0 | 0 | Test |
| Unversioned serialized document kinds | 0 | 0 | Inspection |
| Unqualified executable or uncompiled traced tests | 0 | 0 | Test |

## Verification

Requirement-tagged tests compare values, wire strings, corpus metadata, and
validation outcomes using fixed checked-in inputs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-002-AC-1 | Repeated comparisons over identical values produce one stable order. | Test (TC-005) |
| NFR-002-AC-2 | Every checked-in serialized document names a v1 schema and supported profile where applicable. | Test (TC-014) |
| NFR-002-AC-3 | Missing, skipped, failed, or not-yet-sealed checks cannot be classified as conclusive passing evidence. | Test (TC-016) |
| NFR-002-AC-4 | Mandatory local CI verifies source-locked executable paths and SHA-256 identities, binds the compiled non-ignored Rust test census to requirement-tagged tests, propagates non-zero exits, and rejects incomplete traceability or successful evidence without source-derived positive output. | Test (TC-018) |

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md).
