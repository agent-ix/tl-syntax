---
id: Task-011
title: "Classify executable paths and prove Rust/shared parity"
type: Task
status: blocked
track: A
priority: P0
owner_repository: agent-ix/tl-syntax
consumer_repositories: [agent-ix/tl-syntax]
evidence_method: integration-and-property-test
github_issue: ix://agent-ix/tl-syntax/issues/46
resume_conditions: [ix://agent-ix/tl-syntax/issues/34, ix://agent-ix/tl-syntax/issues/45, ix://agent-ix/quire-research/issues/64]
relationships:
  - target: ix://agent-ix/tl-syntax/Task-010
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-019
    type: references
  - target: ix://agent-ix/tl-syntax/TC-064
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-065
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
---
# Task-011: Classify executable paths and prove Rust/shared parity

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

- Blocked until the current specification is independently reviewed/merged and
  the consumed LR08/shared-runner entries are admitted by Task-010.
