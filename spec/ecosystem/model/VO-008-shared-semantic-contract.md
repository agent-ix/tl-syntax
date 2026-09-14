---
id: VO-008
title: "Shared semantic contract authority"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: implemented_by }
  - { target: ix://agent-ix/quire-contract-ir/FR-027, type: references }
---
# [VO-008] Shared semantic contract authority

## Properties

- **object_ref** — exact `ix://agent-ix/quire-specification/<id>` identity of one shared domain object, value object, state machine or enumeration.
- **revision** — immutable merged Git revision containing the accepted meaning.
- **kind** — exact formal object kind and subsystem (`foundation`, `temporal`, `observation`, or `protocol`).
- **identity_rule** — authored/derived/value classification, complete key components, equality, absence, ordering and digest-domain rules.
- **membership_rule** — complete closed member set and exact wire labels where the object is an enumeration.
- **invariants** — the cross-object independence, transition, correction and non-substitution rules that every implementation must preserve.

This is normative design authority, not a runtime payload and not a second wire
owner. An executable crate cites the exact object and revision, then owns its
own canonical schema, strict reader and public validated type. A consumer may
map two separately valid owner vocabularies only through an exact selected
mapping contract; it may not copy, silently rename or infer a shared value from
display text. An unresolved identity, member set or boundary rule blocks the
dependent reader or mapping instead of being filled locally.
