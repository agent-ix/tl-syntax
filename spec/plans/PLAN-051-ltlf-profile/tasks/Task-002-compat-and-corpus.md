---
id: Task-002
title: "Compatibility proof and LTLf corpus"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-051
    type: part_of
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-334
    type: references
  - target: ix://agent-ix/tl-syntax/FR-335
    type: references
  - target: ix://agent-ix/tl-syntax/TC-226
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-227
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-228
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-229
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-230
    type: verifies
---

# Task-002: Compatibility proof and LTLf corpus

## Scope

Prove byte identity of every existing corpus and edition, and publish corpus/ltlf with 18 rule cases, 4 counterexample entries and the 120-formula enumerator.

## Subtasks

- [ ] Write the failing tests for the listed test cases first, then implement.
- [ ] Tag each test with its criterion id; run the focused tests, then the gate once at the end.

## Deliverables

- corpus/ltlf with manifest and integrity check
- Golden digests of pre-existing corpora

## Notes

- The manifest count and the enumerator count are both asserted (18, 4, 120).
