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

Retained evidence is no longer in scope for this requirement. The bytes under
`evidence/` remain immutable and are read through the shared compatibility
mapping under [FR-006](./FR-006-shared-assurance-intake.md); this repository
holds no retention, integrity, or qualification authority over them.

## Rationale

Downstream differential testing and retained assurance evidence depend on stable
identities and reproducible inputs.

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
| NFR-002-AC-3 | Missing, skipped, failed, or not-yet-computed checks cannot be classified as passing evidence. | Test (TC-016) |

### Retired criteria

**NFR-002-AC-4** is retired. It required this repository to verify source-locked
launcher and toolchain identities, bind a compiled test census, reject in-file
Make execution controls, behaviourally verify per-record evidence validators, and
require an active qualified record. Every one of those is a generic assurance
control that the released Engineering Assurance, Quire, and Quoin contracts now
own, and a repository that keeps its own copy is the arrangement
`agent-ix/engineering-assurance#10` was written to end.

The criterion is not reassigned and **the identifier is not reused**. No retained
record under `evidence/` cites it — the retained records carry no acceptance
criterion identifiers at all — so the reason is not that reuse would rewrite a
sealed claim. The reason is that `SR-005` and `SR-006` adjudicate findings
against `NFR-002-AC-4` by name, and an identifier that means one thing in a
closed review and another in the current specification makes both unreadable.
Its successor obligations are [FR-006](./FR-006-shared-assurance-intake.md) AC-1
through AC-6.

Test case **TC-018** is retired with it, for the same reason and on the same
terms.

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md). Its retired evidence obligations are
carried by [FR-006](./FR-006-shared-assurance-intake.md).
