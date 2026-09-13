---
id: Task-004
title: "Implement past-profile rewrites"
type: Task
status: completed
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

- [x] Write red cases for every admitted/refused rewrite and identity mutation.
- [x] Implement profile-aware O/H/Y/S/T traversal and reviewed equivalences.
- [x] Refuse mixed, unknown, or unproved transformations without partial output.
- [x] Preserve future behavior and run all owning repository gates/reviews.

## Deliverables

- Past-profile rewrite support and traced tests.

## Notes

- GitHub owner: `agent-ix/tl-rewrite#38`.
- Completed by `agent-ix/tl-rewrite#39` at merge revision
  `22b9cadcb1692cec8d3a97768f4f3b38fc654a5e`.
- The tl-rewrite allocation of TC-056 is complete. Canonical shared-corpus
  publication and exact consumer replay remain with dependent Task-005; native
  allocations remain with Tasks 006 and 007.
