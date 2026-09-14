---
id: Task-006
title: "Implement native predicate projection"
type: Task
status: completed
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
  `tl-syntax#61` and consume the reconciled owner merge
  `842d82553f045eb69a7f38745756d968254fc25e`.
- [x] Consume the complete reviewed owner-contract set from Task-010 without
  mirrored wire types, callbacks, or trust flags.
- [x] Organize implementation as cohesive contract-admission,
  predicate-definition, correspondence-artifact, valuation, correction, and
  decision modules under the Task-008 architecture.
- [x] Write TC-038 red cases for every value, non-value, contract, identity, completeness, correction, and resource dimension.
- [x] Implement strict readers, projection, canonical identity, and typed decisions.
- [x] Run all owning repository gates/reviews.

## Deliverables

- Native predicate projection public Rust API: `predicate::project`,
  `predicate::read_projection`, `predicate::value`, and
  `predicate::read_valuation`, with typed selections/decisions and traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#70`.
- Merged through `quire-contract-ir#77` at
  `202210cf6339208740299ae4050d6f16908d557e`; issue #70 is closed.
- Exact owner revisions are QSL `4f404454b3d5cfb78dfdc468c76de85c199191e5`,
  Quire Observation `9ac80e93f4b68a2c7d5a337f9a448ad10de798fc`,
  Quire Protocol `36af8d7bb4753ea89f020fe1e5080cef21879b65`, and
  tl-syntax `842d82553f045eb69a7f38745756d968254fc25e`.
- Exact consumed schema SHA-256 values are checked-predicate
  `459b72a948ddc17be824412b04929fb5795ea033b0bf2aa3f60cf378bc42a531`,
  result-availability
  `2e6c00d6dc94a00b0859b346dc12a14c7142735a565e35d15b85895d311e8416`,
  protocol-result
  `8825d5b05edf7aef9e53dd117b4f344b824e1c6fb2757071f6dd14a268079dd2`,
  result mapping
  `c1df7b18d0e93c70a3b1d67c6a71de8d5ff8b9780510265ebdad69cdd14b79bd`,
  signal-catalog
  `24fe7ccc26918e9a52dfa49a0e670ef932d0bfcd155d5d73dfd0be23dd276df8`,
  and proposition-map
  `ed5be5c2747db16f7936a94b6203bb2747a1abb5a0c5521c2efa38beb8dfdc80`.
- The locked workspace suite passes 67 tests. SR-061 code/Rust review and
  SR-062 gap analysis pass after all nine scoped findings were fixed; FR-025 is
  8/8 backed by 16 TC-038 source symbols.
