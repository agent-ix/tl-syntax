---
id: Task-009
title: "M6 source-readiness specification and composite review"
type: Task
status: done
track: S
priority: P0
owner_repository: agent-ix/tl-syntax
consumer_repositories: [agent-ix/tl-syntax]
evidence_method: specification-and-composite-review
github_issue: ix://agent-ix/tl-syntax/issues/34
resume_conditions: []
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: references
  - target: ix://agent-ix/tl-syntax/FR-015
    type: references
  - target: ix://agent-ix/tl-syntax/FR-016
    type: references
  - target: ix://agent-ix/tl-syntax/FR-017
    type: references
  - target: ix://agent-ix/tl-syntax/FR-018
    type: references
  - target: ix://agent-ix/tl-syntax/FR-019
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: references
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
---
# Task-009: M6 source-readiness specification and composite review

## Scope

Define the source-readiness boundary, closed result domains, acceptance matrix,
assurance records and real-integration scenarios before implementation.

## Subtasks

- [x] Author MRS-004, TM-004 and their requirement/assurance/integration artifacts.
- [x] Run base, failure-domain, integrity, dependency, evidence, risk, scope and EARS reviews.
- [x] Correct every review-owned finding and validate the exact snapshot strictly.

## Deliverables

- Specification commit `767dc92a97f1b3d9ffbb76467bdda9ecf2261e40`
- Author reviews SR-095 through SR-102
- Independent exact-head reviews SR-092 through SR-101

## Notes

- TC-066 is the boundary criterion this task defines and reviews; its executable evidence remains downstream.
- Completion of this authoring task does not mark any TM-004 implementation row covered.
