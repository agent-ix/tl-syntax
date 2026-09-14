---
id: Task-010
title: "Publish the complete Quire owner contract set"
type: Task
status: completed
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

- [x] In `quire-specification`, accept the remaining shared temporal,
  observation and protocol object rulings needed by FR-025/FR-026, while
  importing the already resolved canonical-result identity and vocabulary set.
- [x] In `quire-spec-language`, publish source-bound checked predicate leaves
  and complete native temporal subjects derived from admitted compiled
  protocol packages, including clock, activation, capture, node, span, type,
  model, and definition identities.
- [x] In `quire-observation`, publish observation, population, position,
  clock, capture, progress, closure, completeness, and result-availability
  assertions derived from qualified immutable observation state; retain trigger
  facts without deriving protocol activation.
- [x] In `quire-protocol`, publish the complete canonical native result and
  direct-predecessor contract with independent execution, truth, settlement,
  decision-scope/execution progress and closure, decision premises, completeness, and
  embedded owner selections.
- [x] In `quire-protocol`, publish one exact selected Contract-IR result
  mapping contract/view that binds the applicable predicate or temporal subject
  and correspondence so Contract IR never interprets owner bytes or assumes
  normalized Boolean labels are native wire labels.
- [x] Provide closed version selection, canonical schemas, exact digests,
  duplicate/trailing/unknown-field refusal, byte/depth/count/string limits,
  and no partial output at every owner boundary.
- [x] Run code review, Rust review, and gap analysis in every owner repository
  and fix all findings before dependency pins advance.

## Deliverables

- Reviewed and merged owner crates/revisions satisfying the complete FR-025
  and FR-026 input contract set.
- Exact ContractSelection values and public Rust reader/API handoff inventory.

## Notes

- Shared semantic authority merged through `quire-specification#44` at
  `983b0b28c479241fb066cbe4db3fc0980362de36`; it is a normative specification
  owner and intentionally publishes no runtime parser or duplicate wire crate.
- Existing tickets include `quire-specification#31/#40`, `quire-spec-language#90`,
  completed `quire-observation#15`, and `quire-protocol#8`; the FR-042 portion
  of the last merged through `quire-protocol#51` at `36af8d7b`, while #8 stays
  open for broader Plan-001 acceptance. Task-008 may split additional repository
  tickets where ownership requires separate merge order.
- QProtocol handoff: result contract `quire.protocol.result/v1-draft.1`, schema
  SHA-256 `8825d5b05edf7aef9e53dd117b4f344b824e1c6fb2757071f6dd14a268079dd2`;
  mapping contract `quire.protocol.contract-ir-result-map/v1`, schema SHA-256
  `c1df7b18d0e93c70a3b1d67c6a71de8d5ff8b9780510265ebdad69cdd14b79bd`;
  bounded public entry points `result::{produce,produce_bounded,read,read_bounded}`
  and `result::contract_ir::{map,map_bounded,read,read_bounded}`.
- No owner imports Contract-IR wire vocabulary or calls an evaluator through a
  callback.
