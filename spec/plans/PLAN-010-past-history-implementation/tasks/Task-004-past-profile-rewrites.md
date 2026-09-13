---
id: Task-004
title: "Implement past-profile rewrites"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: references
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
---
# Task-004: Implement past-profile rewrites

## Scope

Deliver `tl-rewrite#38`: only reviewed past-profile equivalences, profile preservation, stable refusal, and corpus replay.

## Subtasks

- [ ] Write red cases for every admitted/refused rewrite and identity mutation.
- [ ] Implement profile-aware O/H/Y/S/T traversal and reviewed equivalences.
- [ ] Refuse mixed, unknown, or unproved transformations without partial output.
- [ ] Preserve future behavior and run all owning repository gates/reviews.

## Deliverables

- Past-profile rewrite support and traced tests.

## Notes

- GitHub owner: `agent-ix/tl-rewrite#38`.
