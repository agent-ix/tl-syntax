---
id: Task-003
title: "Implement origin-complete evaluation"
type: Task
status: completed
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: references
  - target: ix://agent-ix/tl-syntax/FR-012
    type: references
  - target: ix://agent-ix/tl-syntax/TC-048
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-049
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-050
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-051
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-052
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-053
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
---
# Task-003: Implement origin-complete evaluation

## Scope

Deliver `tl-mltl#63`: strict histories, exact clock binding, checked history analysis, anchored O/H/Y/S/T evaluation, immutable results, corrections, and limits.

## Subtasks

- [x] Write the independent reverse-offset oracle and every history/identity/resource red case.
- [x] Implement strict position-history and clock admission with typed errors.
- [x] Implement checked required-history analysis and anchored evaluation.
- [x] Implement immutable result identities and original/superseding/invalidating validation.
- [x] Preserve future evaluator behavior and run all owning repository gates/reviews.

## Deliverables

- Position-history, history-requirement, and past-evaluation public Rust contracts.

## Notes

- GitHub owner: `agent-ix/tl-mltl#63`.
- Merged implementation: `agent-ix/tl-mltl#64` at
  `b346cd0902794633e862f644a5575fc9776c34fb`.
- TC-053 and TC-056 remain globally open only for allocations owned by later
  plan tasks; the complete tl-mltl allocation is delivered here.
