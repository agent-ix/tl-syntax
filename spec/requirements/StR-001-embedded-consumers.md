---
id: StR-001
title: Embedded consumers need allocation-free syntax access
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: satisfied_by
---

# StR-001: Embedded consumers need allocation-free syntax access

## Stakeholder Need

Embedded consumers require that validated MLTL formulas shall be inspectable
without a standard library, heap allocator, parser, evaluator, or monitor.

## Rationale

The syntax model is a shared substrate and must not force downstream firmware or
restricted targets to adopt host-only facilities.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-001-VC-1 | The crate compiles with default features for a no-std library target. | Test (TC-019) |
| StR-001-VC-2 | Borrowed validated formulas require no owned collection. | Test (TC-015) |

`StR-001-VC-2` previously named an agent-produced inspection record retained
under `evidence/`, which was deleted under `agent-ix/tl-syntax#12`. That pointer
was stale rather than load-bearing: the criterion is and was discharged by a
test. `src/syntax.rs::allocation_free_core_api_constructs_formula` carries
`// Trace: TC-015, NFR-001-AC-1, NFR-001-AC-2, StR-001-VC-1, StR-001-VC-2` and
constructs a validated `Formula` over a borrowed slice with no owned collection,
compiled under the no-default-features boundary by `TC-019`. The validation
method is corrected to name it; nothing is restated more weakly.

## Stakeholders

Embedded Rust consumers, downstream temporal-crate maintainers, and the human
v0.1 release owner.

## Context and Assumptions

Callers may provide node and proposition storage with an application-chosen
lifetime. Recursive formula depth is represented by indices rather than the
call stack.

## Traceability

This need is realized by [NFR-001](./NFR-001-no-std-feature-boundary.md) and the
compile checks in [TM-001](../test-matrix.md).
