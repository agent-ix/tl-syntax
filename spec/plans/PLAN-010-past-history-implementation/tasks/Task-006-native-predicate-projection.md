---
id: Task-006
title: "Implement native predicate projection"
type: Task
status: blocked
track: B
priority: P0
relationships:
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
  - target: ix://agent-ix/tl-syntax/TC-053
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
---
# Task-006: Implement native predicate projection

## Scope

Deliver `quire-contract-ir#70` after PR #67: the complete FR-025 strict total-Boolean projection boundary.

## Subtasks

- [x] Merge the accepted FR-025 specification (`quire-contract-ir#67`,
  `39bffb40f41b7caceaf026f438546b15bf140ce4`).
- [x] Publish the strict signal-catalog/proposition-map owner reader surface in
  `tl-syntax#61` and consume merge `c39506062e938e0fd5efd532697f07c74f7814d1`.
- [ ] Consume the complete reviewed owner-contract set from Task-010 without
  mirrored wire types, callbacks, or trust flags.
- [ ] Organize implementation as cohesive contract-admission,
  predicate-definition, correspondence-artifact, valuation, correction, and
  decision modules under the Task-008 architecture.
- [ ] Write TC-038 red cases for every value, non-value, contract, identity, completeness, correction, and resource dimension.
- [ ] Implement strict readers, projection, canonical identity, and typed decisions.
- [ ] Run all owning repository gates/reviews.

## Deliverables

- Native predicate projection public Rust API and traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#70`.
- The target TL reader dependency is complete. Implementation is blocked on the
  whole-ecosystem Task-008 review gate, Task-009 core reconciliation, and the
  Task-010 native/result/availability owner contract set; no Contract-IR
  substitute is permitted.
