---
id: FR-335
title: "Publish an LTLf conformance corpus with the end-of-trace rule cases and the finite-versus-infinite counterexamples"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-045
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-332
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-333
    type: depends_on
---

# FR-335: Publish an LTLf conformance corpus with the end-of-trace rule cases and the finite-versus-infinite counterexamples

## Description

When the LTLf profile ships, tl-syntax shall publish the corpus `ltlf` with a
manifest of hand-authored end-of-trace rule cases and two counterexample
formulas, consumed by tl-mltl, tl-parse and tl-solve through
`tl_syntax::CORPUS_DIR`.

## Behavior

Corpus `tl-syntax.ltlf-corpus/v1`, directory `corpus/ltlf/`:

- 18 rule cases: each of the six temporal nodes (`Xs`, `Xw`, `F`, `G`, `U`, `R`)
  on three trace shapes (length 1; length 2 satisfying; length 2 violating),
  each with its expected verdict at position 0 written by hand from ADR-044.
  Count rule: 6 nodes x 3 shapes = 18.
- 2 counterexamples with two readings each (4 entries): CE-1 `G Xs true` is
  false under LTLf on every nonempty trace and its infinite-trace analogue
  `G F[1,1] true` is true on every lasso; CE-2 `F Xw false` is true under LTLf
  on every nonempty trace and its infinite analogue `F G[1,1] false` is false on
  every lasso. These two are derived from the end-of-trace rules and are the
  corpus's finite-versus-infinite separators; tying them to the literature's
  own named formula is an open citation task (see the tl-mltl reference ADR).
- The depth-one population is not stored: it is the enumeration "constants and
  both atoms (4), each of five unary operators on those 4, each of six binary
  operators on the 16 ordered pairs of those 4" = 4 + 5x4 + 6x16 = 120
  formulas, with the enumerator and its count stored in the manifest.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-335-AC-1 | The manifest lists exactly 18 rule cases and 4 counterexample entries, and the enumerator yields exactly 120 depth-one formulas. | Test (TC-229) |
| FR-335-AC-2 | Mutating any manifest field (formula, trace, expected verdict, count) is detected by the integrity check. | Test (TC-230) |

## Dependencies

FR-005 supplies the corpus publication pattern.
