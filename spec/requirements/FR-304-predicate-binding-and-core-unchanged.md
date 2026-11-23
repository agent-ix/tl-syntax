---
id: FR-304
title: Bind predicate atoms to formulas without changing the temporal core
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-303
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
---

# FR-304: Bind predicate atoms to formulas without changing the temporal core

## Description

When a formula references predicate-defined propositions, tl-syntax shall bind
the formula, the predicate table and the signal catalog into one borrowed view
that resolves every proposition occurrence to exactly one source, while the
formula graph, node kinds and profiles stay unchanged.

## Inputs

- A validated formula (any existing edition), a validated predicate table, a
  validated catalog.

## Outputs

- A borrowed `BoundPredicateFormula` view: for each proposition occurrence in
  node order, either a Boolean signal binding or a predicate definition, plus
  the ordered set of numeric signals the formula reads.
- Typed `PredicateBindingError`.

## Behavior

- Resolution is in formula-node order and reports the first unresolved
  proposition (`PropositionUnbound`), the first proposition defined by both a
  catalog binding and a predicate (`PropositionDefinedTwice`), and the first
  predicate whose signals are not declared numeric (`NonNumericSignal`).
- The existing `bind_formula` is unchanged: a predicate-only proposition remains
  an unbound proposition there, so a consumer that does not know the table fails
  closed.
- Extra predicates and catalog entries are allowed so one table and catalog may
  serve several formulas.
- The view allocates nothing and exposes no mutation.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-304-AC-1 | A formula mixing Boolean-bound and predicate-defined atoms binds, and the view reports each occurrence's source and the numeric signals read, in node order. | unit-testing (TC-1508) |
| FR-304-AC-2 | Unbound, doubly defined and non-numeric cases fail with distinct errors at the first offending occurrence, and `bind_formula` on the same formula still reports the predicate proposition as unbound. | unit-testing (TC-1509) |
| FR-304-AC-3 | The `NodeKind` vocabulary, `SemanticProfile` set and all existing schema digest constants are unchanged. | inspection (TC-1510) |

## Dependencies

FR-303 supplies the table; FR-007 supplies the catalog and Boolean binding rule.
