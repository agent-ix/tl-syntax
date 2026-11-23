---
id: FR-306
title: Model named member sets and aggregation templates
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-016
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-303
    type: depends_on
---

# FR-306: Model named member sets and aggregation templates

## Description

When a requirement ranges over a group of signals, tl-syntax shall carry the
group and its quantified template in a `tl-syntax.set-aggregation/v1` document
with a strict reader, separately from any formula or predicate table.

## Inputs

- Ordered named sets whose members are `SignalId` values of a validated catalog.
- A quantifier: `for_each`, `for_some`, or `at_least` with an `n`.
- A template: formula nodes and predicate definitions in which the leaves
  `slot_signal` (an expression leaf) and `slot_proposition` (a formula leaf,
  resolving to the Boolean proposition the catalog binds to the member signal)
  stand for the current member. `slot_*` leaves are admitted only in a template,
  never in a formula document or predicate table.
- A `first_proposition` base from which fresh proposition identities are
  allocated. The expander is the only allocator of identities for member-dependent
  predicates (FR-307 "Identity precedence").
- Nesting: a template may itself contain one nested quantified template, to a
  total depth of 2 (`AggregationTooDeep` beyond that). The nested quantifier's set is
  an ordered set of catalog signals and is fixed per document, not per outer member.

## Outputs

- An owned `SetAggregationDocument` and bounded reader; typed
  `SetAggregationError`.

## Behavior

- Sets: 1 through 1,024 members (`MAX_SET_MEMBERS`), unique, in document order,
  each declared in the catalog. An empty set is representable only through an
  explicit empty `members` list in the document and is expanded by the
  FR-307 empty-set rule; it is not an error.
- `slot_signal` is typed against the member's declared domain; a template that is
  ill-typed for any member is refused at construction naming the member
  (`TemplateIllTypedForMember`). `slot_proposition` requires every member to have
  a Boolean catalog binding (`MemberHasNoBooleanBinding`).
- `at_least(n)` requires `n <= 1,024`; `n` greater than the member count is
  legal and expands to `false`.
- If the fresh identity range `[first_proposition, first_proposition + needed)`
  overlaps a catalog binding or a template-referenced proposition, or exceeds
  `u32`, then the validator shall refuse the document (`FreshIdentityCollision`).
- A nested template may use the outer member's slot leaves as well as its own;
  each slot leaf names the quantifier it belongs to by depth (0 outer, 1 inner),
  and a leaf naming an absent depth is refused (`SlotOutsideTemplate`).
- Wire rules follow FR-303.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-306-AC-1 | A document with each quantifier, Boolean-member and numeric-member sets, and slot leaves round-trips byte-identically through the strict reader. | integration-testing (TC-1514) |
| FR-306-AC-2 | `slot_*` leaves outside a template, an undeclared member, a duplicate member, an ill-typed-for-one-member template, a missing Boolean binding and a colliding fresh range are each refused with a distinct error. | unit-testing (TC-1515) |

## Dependencies

FR-303 supplies expression typing and the wire pattern.
