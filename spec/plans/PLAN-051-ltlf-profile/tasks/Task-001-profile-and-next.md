---
id: Task-001
title: "Profile identity, admission rules and explicit next nodes"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-051
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-332
    type: references
  - target: ix://agent-ix/tl-syntax/FR-333
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-048
    type: references
  - target: ix://agent-ix/tl-syntax/TC-220
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-221
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-222
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-223
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-224
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-225
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-231
    type: verifies
---

# Task-001: Profile identity, admission rules and explicit next nodes

## Scope

Add SemanticProfile::LtlfFiniteTraceV1, the ltlf-operators catalog, the formula-ltlf edition and StrongNext/WeakNext, with the nine refusal classes.

## Subtasks

- [ ] Write the failing tests for the listed test cases first, then implement.
- [ ] Tag each test with its criterion id; run the focused tests, then the gate once at the end.

## Deliverables

- Profile, edition and node vocabulary
- Typed refusals per axis

## Notes

- Unblocks Task-002, tl-parse TL-429 and tl-mltl evaluation.
- Exhaustive matches downstream must name the new variant (source-breaking, wire-compatible).
