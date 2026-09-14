---
id: Task-007
title: "Implement native temporal correspondence"
type: Task
status: in_progress
track: C
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-006
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-009
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-010
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
- [ ] Construct sibling QSL-native and TL requests from the same admitted
  observations and join only their formula-wide owner results; reject every
  Protocol leaf-result substitution.
- [ ] Organize formula/valuation construction, native-node correspondence,
  request construction, result normalization/joining, progress/completeness,
  and correction handling as explicit subsystem modules rather than one flat
  bridge file.
- [ ] Run all owning repository gates/reviews and prove the quire-protocol temporal input seam is available.

## Deliverables

- Native temporal projection and result-join public Rust API with traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#71`; downstream `quire-protocol#12/#14`.
- Task-008 architecture, Task-009 TL core and Task-006/#70 are complete.
  Projection implementation starts from Contract-IR merge `202210cf6339208740299ae4050d6f16908d557e`.
  Result-join completion is sequenced after the reopened Task-010 delivers QSL
  FR-052; this is an owner-boundary correction inside the same campaign, not a
  scope deferment.
