---
id: Task-006
title: "Implement native predicate projection"
type: Task
status: blocked
track: B
priority: P0
relationships:
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

- [ ] Merge the accepted FR-025 specification and set this task in progress.
- [ ] Write TC-038 red cases for every value, non-value, contract, identity, completeness, correction, and resource dimension.
- [ ] Implement strict readers, projection, canonical identity, and typed decisions.
- [ ] Run all owning repository gates/reviews.

## Deliverables

- Native predicate projection public Rust API and traced tests.

## Notes

- GitHub owner: `agent-ix/quire-contract-ir#70`.
