---
id: Task-007
title: "Publish and verify the shared integrator package"
type: Task
status: blocked
track: C
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-006
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-016
    type: references
  - target: ix://agent-ix/tl-syntax/IT-002
    type: references
  - target: ix://agent-ix/tl-syntax/TC-061
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-063
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-068
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-070
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-071
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-072
    type: verifies
---
# Task-007: Publish and verify the shared integrator package

## Scope

Implement atomic/idempotent package publication and prove lossless separation
of source-release facts from adopter-owned validation and acceptance.

## Subtasks

- [ ] Write real writer/strict-reader round-trip and mutation tests.
- [ ] Implement candidate-bound temporary output and atomic visibility.
- [ ] Exercise retry identity and non-identical writer races.
- [ ] Complete the independent exact-population license/reuse-right review.

## Deliverables

- Rust adapter to the released integrator-package contract
- IT-002 and TC-061/063/066/068/070/071/072 evidence

## Notes

- Blocked until Engineering Assurance releases and accepts the real package contract; no local schema or mock can close this task.
