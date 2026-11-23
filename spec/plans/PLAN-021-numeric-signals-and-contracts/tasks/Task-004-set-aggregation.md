---
id: Task-004
title: "Set aggregation document and expansion (TL-424)"
type: Task
status: todo
track: A
priority: P1
relationships:
  - target: ix://agent-ix/tl-syntax/FR-306
    type: references
  - target: ix://agent-ix/tl-syntax/FR-307
    type: references
  - target: ix://agent-ix/tl-syntax/FR-308
    type: references
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/TC-1516
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1517
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1518
    type: verifies
---
# Task-004: Set aggregation document and expansion (TL-424)

## Scope

FR-306..FR-308: set-aggregation document, bounded deterministic expansion (balanced for_each/for_some, counting table for at_least), hand-written equivalents.

## Subtasks

- [ ] Document and reader.
- [ ] Expansion core with checked counts, no-std.
- [ ] Hand-written forms and populations (567 compared traces).

## Deliverables

Document, expansion, corpus cases, tests TC-1514..TC-1521.

## Notes

Specification-only session: no code was written or run. Layer 3 of P-TL-150.
