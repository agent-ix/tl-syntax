---
id: ADR-018
title: "Decide numeric satisfiability by bit-blasting bounded domains and never by independent atoms"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/ADR-014
    type: depends_on
  - target: ix://agent-ix/tl-solve/ADR-200
    type: realizes_requirement
  - target: ix://agent-ix/tl-solve/ADR-201
    type: realizes_requirement
---

# ADR-018: Decide numeric satisfiability by bit-blasting bounded domains and never by independent atoms

## Status

Proposed (TL-419). Ready for human acceptance. Realized in `tl-solve` by
ADR-200 and ADR-201 (ids FR-200..FR-299 / ADR-200..ADR-299 are reserved for
this project there).

## Context

"x > 5" and "x < 3" at the same step conflict. An encoder that gives each
predicate its own free Boolean variable reports the pair satisfiable, which is a
wrong "satisfiable" answer, and a witness built from it would be rejected by
the re-check only after the damage of a misleading search. Worse, the same
abstraction in the other direction produces wrong "unsatisfiable" answers
when a relation between atoms (for example `x > 5` implies `x > 3`) is dropped.

## Decision

1. **Prohibition.** A numeric predicate atom shall never be encoded as an
   independent proposition in a satisfiability, validity, implication,
   equivalence, consistency, vacuity or redundancy query. This is a hard
   requirement on every encoder, not an optimisation choice.
2. **Encoding.** Each numeric signal at each time step is a fixed-width bit
   vector (offset binary: value minus declared minimum), constrained to its
   declared domain. Each predicate at each step is the output of a circuit built
   from its expression (ripple adders, subtractors, comparators, shift-add
   multipliers) at widths taken from the ADR-014 derived ranges, so no circuit
   overflows.
3. **Refusal, not approximation.** A domain wider than the configured cap, a
   total encoding larger than the configured cap, a predicate without a table
   definition, an unsupported operator, or a missing declared bound is refused
   with a typed error. The solver never narrows a domain, truncates a width, or
   substitutes an abstraction. A caller may supply a catalog with narrower
   declared domains; the result then names that catalog and is a statement
   about it, not about the original.
4. **Trust.** As for Boolean encoding, `satisfiable` is never trusted: the
   witness trace, with numeric values, is re-evaluated by `tl-mltl`. For
   `unsatisfiable` the encoder, including these circuits, is the trusted part
   and is validated by exhaustive small-scope agreement with enumeration.

## Alternatives rejected

- Lazy SMT-style theory solving: rejected, would need an SMT backend or a
  second decision procedure; the repository forbids linked Z3 and C/C++ builds.
- Interval abstraction: rejected, unsound for equality and arithmetic.
- Treating predicates as free atoms and relying on the re-check: rejected, the
  re-check cannot detect wrong "unsatisfiable" answers.

## Consequences

Numeric satisfiability is practical only for narrow declared domains; wide
domains are refused with an explanation. That is the intended trade.
