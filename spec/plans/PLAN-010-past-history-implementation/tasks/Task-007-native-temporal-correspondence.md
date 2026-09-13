---
id: Task-007
title: "Implement native temporal correspondence"
type: Task
status: blocked
track: C
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-006
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-012
    type: references
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
---
# Task-007: Implement native temporal correspondence

## Scope

Deliver `quire-contract-ir#71` after PR #68: the complete FR-026 formula/history/request construction and result-join path.

## Subtasks

- [x] Merge the accepted FR-026 specification after its stack base
  (`quire-contract-ir#68`, `58f834be44927e31755b3abf797a35fb9cace507`).
- [ ] Write TC-039 red cases for every supported/refused profile, identity, closure, progress, completeness, settlement, correction, and resource dimension.
- [ ] Implement the strict bridge using owner-published native, TL, history, evaluator, and result APIs.
- [ ] Run all owning repository gates/reviews and prove the quire-protocol temporal input seam is available.

## Deliverables

- Native temporal projection and result-join public Rust API with traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#71`; downstream `quire-protocol#12/#14`.
- Blocked on Task-006/#70 and the owner-published native, TL, history, evaluator,
  result and quire-protocol temporal input APIs.
