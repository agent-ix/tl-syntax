---
id: Task-010
title: "Publish the complete Quire owner contract set"
type: Task
status: blocked
track: Owners
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-012
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-025
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-026
    type: references
---
# Task-010: Publish the complete Quire owner contract set

## Scope

Implement every authority-owned immutable input required by both Contract-IR
bridges, with canonical bytes, schema identities/digests, bounded public strict
readers, and constructor-private validated views after Task-012 makes the
native-owner/bridge Cargo graph acyclic.

## Subtasks

- [ ] In `quire-specification`, accept the remaining shared temporal,
  observation and protocol object rulings needed by FR-025/FR-026, while
  importing the already resolved canonical-result identity and vocabulary set.
- [x] In `quire-spec-language`, publish source-bound checked predicate leaves
  and complete native temporal subjects derived from admitted compiled
  protocol packages, including clock, activation, capture, node, span, type,
  model, and definition identities.
- [ ] In `quire-observation`, publish observation, population, position,
  clock, capture, progress, closure, completeness, and result-availability
  assertions derived from qualified immutable observation state; retain trigger
  facts without deriving protocol activation.
- [ ] In `quire-protocol`, publish the complete canonical native result and
  direct-predecessor contract with independent execution, truth, settlement,
  decision-scope/execution progress and closure, decision premises, completeness, and
  embedded owner selections.
- [ ] In `quire-protocol`, publish one exact selected Contract-IR result
  mapping contract/view that binds the applicable predicate or temporal subject
  and correspondence so Contract IR never interprets owner bytes or assumes
  normalized Boolean labels are native wire labels.
- [ ] Provide closed version selection, canonical schemas, exact digests,
  duplicate/trailing/unknown-field refusal, byte/depth/count/string limits,
  and no partial output at every owner boundary.
- [ ] Run code review, Rust review, and gap analysis in every owner repository
  and fix all findings before dependency pins advance.

## Deliverables

- Reviewed and merged owner crates/revisions satisfying the complete FR-025
  and FR-026 input contract set.
- Exact ContractSelection values and public Rust reader/API handoff inventory.

## Notes

- Existing tickets include `quire-specification#31/#40`, `quire-spec-language#90`,
  `quire-observation#15`, and `quire-protocol#8`; Task-008 may split additional
  repository tickets where ownership requires separate merge order.
- No owner imports Contract-IR wire vocabulary or calls an evaluator through a
  callback.
