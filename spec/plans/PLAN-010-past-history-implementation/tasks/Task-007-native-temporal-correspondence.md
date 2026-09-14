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
  (`quire-contract-ir#68`, `58f834be44927e31755b3abf797a35fb9cace507`).
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
- Completed through `quire-contract-ir#78` at
  `69ec82bf4da1bdbee710544a4570c2042dc781a5`; issue #71 is closed.
- Consumes immutable QSL `f1700a9264d6d3bcdd07e0f77b70f3dae9ed4c07`, Quire
  Observation `9ac80e93f4b68a2c7d5a337f9a448ad10de798fc`, Quire Protocol
  `34d1752e6c5f789a52ccf115b0694eedd96cdd46`, tl-syntax
  `842d82553f045eb69a7f38745756d968254fc25e`, and tl-mltl
  `22862189ac4eb515ab84928faec25b2eac47d835` owner revisions.
- PLAN-006, SR-536 and SR-537 record complete TC-039 coverage and resolved
  code/Rust/gap findings; no parser, evaluator, Boolean coercion or copied
  owner wire vocabulary was added.
