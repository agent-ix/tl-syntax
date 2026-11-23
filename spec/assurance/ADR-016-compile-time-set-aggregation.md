---
id: ADR-016
title: "Expand set aggregation at compile time into ordinary formulas"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/ADR-014
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-306
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-307
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-308
    type: realizes_requirement
---

# ADR-016: Expand set aggregation at compile time into ordinary formulas

## Status

Proposed (TL-419). Ready for human acceptance.

## Context

Requirements range over groups ("for each engine", "at least two of these
sensors"). C2PO expands `foreach`, `forsome` and `atleast` at compile time into
ordinary formulas. The family must carry the same shorthand without adding a
quantifier to the temporal core or letting an expansion grow without bound.

## Decision

1. Aggregation is a separate source-level document
   `tl-syntax.set-aggregation/v1` (named ordered member sets, a template, a
   quantifier) plus a pure expansion function that returns ordinary formula
   nodes and predicate definitions. Nothing downstream ever sees a quantifier.
2. Quantifiers: `for_each` (conjunction), `for_some` (disjunction),
   `at_least(n)`. `for_each` over an empty set is `true`, `for_some` over an
   empty set is `false`; `at_least(0)` is `true`, and `at_least(n)` with `n`
   above the member count is `false`. `at_most` and `exactly` are out of scope
   for v1.
3. Expansion is deterministic: members are visited in document order, the
   template is instantiated once per member with fresh proposition identities
   allocated from a declared base, the conjunction/disjunction is a balanced
   tree split at `ceil(k/2)`, and `at_least(n)` uses a shared counting table
   that is `O(n*(k-n+1))` nodes, not `C(k,n)` terms.
4. Expansion is bounded: the exact node and predicate counts are computed with
   checked arithmetic before any node is built, and an expansion over
   `MAX_AGGREGATION_NODES` (default and hard maximum bounded by the formula
   document node limit, 100,000) or `MAX_SET_MEMBERS` (1,024) is refused.
5. The expansion is equivalent to the hand-written form on a retained corpus:
   each aggregation case carries a hand-written formula and both are evaluated
   on every trace of a declared small population.

## Alternatives rejected

- A runtime quantifier or counting operator in the evaluator: rejected, changes
  the temporal core and every evaluator.
- Disjunction of all `C(k,n)` conjunctions for `at_least`: rejected, exponential.
- Unbounded expansion with a late memory failure: rejected, a refusal must
  precede allocation.

## Consequences

Templates and sets add one more strict document to read and retain. Because
expansion is a pure function of the document, its output can be reproduced by
any consumer and is part of the corpus identity.
