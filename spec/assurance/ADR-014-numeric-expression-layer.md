---
id: ADR-014
title: "Carry numeric predicates as an expression layer that lowers to atoms"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-302
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-303
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-304
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/ADR-002
    type: relates_to
---

# ADR-014: Carry numeric predicates as an expression layer that lowers to atoms

## Status

Proposed (TL-419, project P-TL-150). Ready for human acceptance; no agent
advances this to accepted.

## Context

Requirements are mostly about signal values ("altitude above 500 for 3 steps").
Today a direct proposition binding must resolve to a Boolean signal
([FR-007](../requirements/FR-007-typed-signal-context.md)), and the v1 signal
catalog already declares bounded Integer and fixed-Decimal domains that
"may be retained for a frontend's explicit predicate-lowering boundary".
Nothing in the family defines that boundary, so the typed-predicate step is
done outside the family and invisibly to every consumer. The temporal core
(`tl-syntax.formula/v1`, `/v2`, `tl-syntax.formula-unbounded/v1`), the
evaluators, the rewriter and the satisfiability encoder all operate on
`PropositionId` leaves and must not change meaning.

## Decision

1. Numeric comparisons live in a new, separate document
   `tl-syntax.predicate-table/v1`: an ordered table that defines selected
   `PropositionId` values as comparisons between exact numeric expressions over
   catalog signals. A formula references such a proposition exactly as it
   references any atom; the formula documents, their schemas and their bytes do
   not change.
2. The expression grammar is closed and small: signal reads, Integer and
   fixed-Decimal literals, negation, absolute value, addition, subtraction,
   multiplication, and an explicit widening `rescale`; comparisons `<`, `<=`,
   `>`, `>=`, `==`, `!=`. There is no division, modulo, power, root, float,
   implicit conversion, or implicit rescale.
3. Typing is static and total. Every sub-expression has a derived type
   `(scale, minimum coefficient, maximum coefficient)` computed by exact
   interval arithmetic from the catalog's declared domains. A table whose
   derived coefficient range does not fit `i64`, whose operands disagree in
   scale, or that reads a non-numeric or undeclared signal is refused at
   construction. A validated table therefore cannot overflow at evaluation.
4. A `PropositionId` is defined by at most one source: a direct Boolean catalog
   binding or one predicate definition. Overlap is refused. `bind_formula` is
   unchanged and still rejects a predicate proposition as an unbound
   proposition, so existing consumers fail closed instead of misreading it.
5. The core feature set stays `no_std` and allocation-free for borrowed
   validation (caller-owned scratch, as the catalog already does); owned
   documents and the wire form sit behind `alloc` and `serde`.
6. FSL remains the only editable formal-clause authority. The predicate table is
   derived interchange: it carries what an FSL typed-predicate bridge
   (`agent-ix/filament-contract-ir#63`) or a TL text dialect lowers to, and it
   grants no independent clause authority.

## Alternatives rejected

- New `NodeKind` variants for comparisons inside the formula graph: rejected
  because every exhaustive downstream match, schema and corpus changes and the
  v1 closed documents would need a new edition.
- Optional predicate fields added to the v1 catalog or formula documents:
  rejected; `FR-007-AC-5` forbids new fields in the closed v1 documents.
- Floats or implicit rescale for convenience: rejected; they make reproducible
  evaluation and a sound satisfiability encoding impossible (ADR-015, ADR-018).
- Division and modulo: rejected for v1; the result is not closed over
  Integer/fixed-Decimal without a rounding rule, and C2PO's integer-division
  rounding is unpinned (ADR-015).

## Consequences

Parsers, evaluators, exporters and the solver each gain one explicit,
versioned input (the table) rather than a reinterpretation of an existing one.
A formula that uses a predicate proposition is evaluable only together with its
table and catalog; without them it fails closed. The three-way type check is
the single place where overflow and scale errors are caught, which keeps every
later layer free of arithmetic failure paths.
