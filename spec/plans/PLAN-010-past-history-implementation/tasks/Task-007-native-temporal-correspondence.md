---
id: Task-007
title: "Implement native temporal correspondence"
type: Task
status: completed
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
  (`quire-contract-ir#68`).
- [x] Write TC-039 cases for every supported/refused profile, identity, closure, progress, completeness, settlement, correction, and resource dimension.
- [x] Implement the strict bridge using owner-published native, TL, history, evaluator, and result APIs.
- [x] Construct sibling QSL-native and TL requests from the same admitted
  observations and join only their formula-wide owner results; reject every
  Protocol leaf-result substitution.
- [x] Organize formula/valuation construction, native-node correspondence,
  request construction, result normalization/joining, progress/completeness,
  and correction handling as explicit subsystem modules rather than one flat
  bridge file.
- [x] Run all owning repository gates/reviews and prove the quire-protocol temporal input seam is available.

## Deliverables

- Native temporal projection and result-join public Rust API with traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#71`; downstream `quire-protocol#12/#14`.
- Completed through `quire-contract-ir#78`; issue #71 is closed.
- PLAN-006, SR-536 and SR-537 record complete TC-039 coverage and resolved
  code/Rust/gap findings; no parser, evaluator, Boolean coercion or copied
  owner wire vocabulary was added.
