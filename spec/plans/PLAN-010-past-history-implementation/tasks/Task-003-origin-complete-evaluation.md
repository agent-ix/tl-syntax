---
id: Task-003
title: "Implement origin-complete evaluation"
type: Task
status: not_started
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

- [ ] Write the independent reverse-offset oracle and every history/identity/resource red case.
- [ ] Implement strict position-history and clock admission with typed errors.
- [ ] Implement checked required-history analysis and anchored evaluation.
- [ ] Implement immutable result identities and original/superseding/invalidating validation.
- [ ] Preserve future evaluator behavior and run all owning repository gates/reviews.

## Deliverables

- Position-history, history-requirement, and past-evaluation public Rust contracts.

## Notes

- GitHub owner: `agent-ix/tl-mltl#63`.
