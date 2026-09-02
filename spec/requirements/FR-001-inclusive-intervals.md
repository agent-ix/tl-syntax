---
id: FR-001
title: Validate inclusive discrete intervals
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-002
    type: implements
---

# FR-001: Validate inclusive discrete intervals

## Description

When an interval is constructed, the library shall accept it only when its
lower discrete bound is less than or equal to its upper bound.

## Inputs

- Unsigned lower and upper discrete-time bounds.

## Outputs

- A validated inclusive interval or an interval-validation error.

## Behavior

- The interval constructor shall preserve both inclusive bounds exactly, expose
  them through `start()` and `end()`, report membership through `contains()`,
  and report the inclusive cardinality when it fits in `u32` (`None` only for
  the full `[0, u32::MAX]` interval).
- The interval constructor shall reject inverted bounds.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-001-AC-1 | Equal and increasing bounds construct successfully, preserve both endpoints, include exactly the instants between them, and report their checked inclusive cardinality. | Test (TC-001) |
| FR-001-AC-2 | Every lower bound greater than its upper bound is rejected. | Test (TC-002) |

## Dependencies

This value is consumed by the temporal nodes specified in
[FR-002](./FR-002-validated-formula.md).
