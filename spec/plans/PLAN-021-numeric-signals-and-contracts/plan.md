---
id: PLAN-021
title: Numeric signals, aggregation and contracts plan (tl-syntax)
type: Plan
status: draft
relationships:
  - target: ix://agent-ix/tl-syntax/FR-302
    type: references
  - target: ix://agent-ix/tl-syntax/FR-303
    type: references
  - target: ix://agent-ix/tl-syntax/FR-304
    type: references
  - target: ix://agent-ix/tl-syntax/FR-305
    type: references
  - target: ix://agent-ix/tl-syntax/FR-306
    type: references
  - target: ix://agent-ix/tl-syntax/FR-307
    type: references
  - target: ix://agent-ix/tl-syntax/FR-308
    type: references
  - target: ix://agent-ix/tl-syntax/FR-309
    type: references
  - target: ix://agent-ix/tl-syntax/FR-310
    type: references
  - target: ix://agent-ix/tl-syntax/FR-311
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-018
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-019
    type: references
---

# Implementation Plan: numeric signals, aggregation and contracts (tl-syntax)

## Requirements Summary

- [ ] **FR-302/303/304/305** (TL-420): expression typing, predicate table, binding, corpus.
- [ ] **FR-309/310/311** (TL-425): contract set, status algebra, realizability interface.
- [ ] **FR-306/307/308** (TL-424): aggregation document, bounded expansion, equivalence evidence.
- [ ] **NFR-018/019**: no-std boundary, bounded determinism.

## Dependency Graph

Layers match `ix-board project-chunk P-TL-150` (blocker layers):

| Layer | Tickets | tl-syntax tasks |
|---|---|---|
| 0 | TL-419 | this specification (ADR-014..019 proposed) |
| 1 | TL-420, TL-425 | Task-001, Task-002 (independent of each other) |
| 2 | TL-421, TL-422 | downstream in tl-parse, tl-mltl; consume Task-001/003 |
| 3 | TL-423, TL-424 | Task-004 (after Task-001, Task-003) |
| 4 | TL-426 | tl-solve; consumes corpus from Task-003 |

- `Task-001 -> Task-003` (binding and corpus need the table).
- `Task-001, Task-003 -> Task-004` (templates use typed expressions and the corpus).
- `Task-002` has no dependency inside this plan.

## Quality Gates

**Gate 1** (after Task-001): every FR-302 error class is refused before any
view exists; existing fixture bytes unchanged (TC-1506).
**Gate 2** (after Task-003): numeric corpus manifest closed and replayed (TC-1511).
**Gate 3** (before merge): no-std matrix (TC-1530) and full local gate once.

## Execution Tracks

- **Track A**: Task-001 -> Task-003 -> Task-004.
- **Track B**: Task-002.

## Test Plan

The matrix `spec/numeric-predicates-test-matrix.md` is authoritative; this plan
adds no test id.

## Deferred verification

| Item | Why deferred | Evidence needed to close |
|---|---|---|
| Realizability decision (TC-1529) | Inter-project decision between P-TL-150 and P-TL-149 | Human decision recorded in ADR-017 |
| Cross-target byte replay (TC-1532) beyond the CI host | No build capacity in the specification session; needs MSRV and Miri jobs | `make msrv`, `make miri` results at the final head |
| Build, clippy, tests of every task | No build capacity during authoring | `make ci` at the implementation PR head |
| Fresh R2U2 or C2PO output | Nothing in this repository executes R2U2 or C2PO | Retained exchange recorded by tl-mltl (PLAN in tl-mltl) |
| TL-423 AC1 and TL-425 AC2 (R2U2 differential) | BLOCKED, not met by any task in this plan; the only deliverable is the refusal-until-pinned path | A human runs R2U2 locally, pins the numeric profile and retains the exchange (matrix section "Open and blocked acceptance criteria") |
| Contract scope A, B or C; digest exemption | Human decisions | Peter; see ADR-017 Open decisions and ADR-019 |
