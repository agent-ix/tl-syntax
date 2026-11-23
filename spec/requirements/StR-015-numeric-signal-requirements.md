---
id: StR-015
title: Requirement authors need numeric signal predicates and groups of signals
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-302
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-303
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-304
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-306
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-307
    type: satisfied_by
---

# StR-015: Requirement authors need numeric signal predicates and groups of signals

## Stakeholder Need

Requirement authors and assurance reviewers require that a temporal
requirement over signal values ("altitude above 500 for 3 steps") and over a
group of signals ("for each engine", "at least two of these sensors") shall be
expressible, evaluated and exported without a layer outside the temporal
crate family converting numbers to Booleans invisibly.

## Rationale

Most real requirements are about signal values, but only Boolean signals can
appear in a formula today. The R2U2 input language already accepts arithmetic,
set aggregation and contracts, so the family cannot yet carry what those users
write, and an unspecified external predicate step is exactly where reproducible
evidence is lost.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-015-VC-1 | A reviewer can read, for every numeric predicate, its exact meaning (type, range, scale) from one versioned document, and every type or domain error is refused before any evaluation. | Demonstration |
| StR-015-VC-2 | A group requirement expands to an ordinary formula that a reviewer can compare with the hand-written form, and an oversized expansion is refused rather than truncated. | Demonstration |
| StR-015-VC-3 | The TL-419 specification package is complete: ADR-014 through ADR-018 recorded as proposed, requirements and a test matrix written in each of tl-syntax, tl-parse, tl-mltl and tl-solve, and a spec review recorded, with human acceptance left pending. | Inspection |

## Stakeholders

Requirement authors, FSL typed-predicate bridge maintainers, R2U2/C2PO users,
and certification assessors.

## Context and Assumptions

FSL remains the only editable formal-clause language; predicate tables,
aggregation documents and any TL text dialect are derived interchange. The
catalog's Integer and fixed-Decimal domains (FR-007) are the type source.

## Dependencies

[StR-003](./StR-003-formal-temporal-frontends.md) supplies the typed catalog.
Downstream requirements live in tl-parse (FR-028..), tl-mltl (FR-053..) and
tl-solve (FR-200..).
