---
id: Task-003
title: "Classify executable paths and prove Rust/shared parity"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-018
    type: references
  - target: ix://agent-ix/tl-syntax/TC-064
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-065
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
---
# Task-003: Classify executable paths and prove Rust/shared parity

## Scope

Build the complete executable-entry-point inventory and migrate stable-required
first-party behavior to Rust or a released shared capability.

## Subtasks

- [ ] Write failing exact-census/classifier tests before the classifier.
- [ ] Inventory scripts, build steps, nested interpreters and generated commands.
- [ ] Record owner, authority, pre-stable disposition, parity evidence and resume condition.
- [ ] Demonstrate positive and forced-failure parity before removing each legacy path.

## Deliverables

- Rust executable census/classifier and reviewed disposition inventory
- TC-064 through TC-066 evidence

## Notes

- Blocked until the current specification is independently reviewed/merged and the consumed LR08/shared-runner entries are admitted by Task-002.
