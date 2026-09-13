---
id: Task-005
title: "Publish the shared past/history corpus"
type: Task
status: completed
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-058
    type: verifies
---
# Task-005: Publish the shared past/history corpus

## Scope

Deliver `tl-syntax#54`: one immutable paired corpus, exact digest, consumer replay, and the authorized implementation manifest gate.

## Subtasks

- [x] Populate every semantic, history, clock, identity, correction, resource, rewrite, and target-disposition case.
- [x] Implement the Rust dependency-manifest reader and mutation controls.
- [x] Pin and replay the exact corpus from parser, evaluator, and rewrite repositories.
- [x] Run all owning repository gates/reviews.

## Deliverables

- Canonical past/history corpus, checksum set, manifest gate, and consumer replays.

## Notes

- GitHub owner: `agent-ix/tl-syntax#54`.
