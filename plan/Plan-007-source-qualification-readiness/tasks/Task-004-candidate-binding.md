---
id: Task-004
title: "Bind immutable candidates and fresh producer results"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-014
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: references
  - target: ix://agent-ix/tl-syntax/TC-059
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-060
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-068
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-069
    type: verifies
---
# Task-004: Bind immutable candidates and fresh producer results

## Scope

Implement candidate/configuration identity, source traversal and producer-
freshness projection through released source-grounding contracts.

## Subtasks

- [ ] Write property tests for every identity axis and source-walk boundary.
- [ ] Implement immutable-root binding and bounded symlink traversal in Rust.
- [ ] Implement total execution/decode mappings and stale-output quarantine.
- [ ] Prove deterministic equality and one-axis isolation.

## Deliverables

- Rust candidate/source binding and producer adapter
- TC-059, TC-060, TC-068 and TC-069 evidence

## Notes

- No Markdown parser, branch-head contract or repository-local compatibility map is admissible.
