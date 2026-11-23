---
id: FR-308
title: Evidence that an expansion equals the hand-written form
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-307
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-305
    type: depends_on
---

# FR-308: Evidence that an expansion equals the hand-written form

## Description

Where an aggregation case is published in the numeric corpus, tl-syntax shall
publish beside it the hand-written ordinary formula that the aggregation is
claimed to equal, and the corpus shall record the traces over which the two are
compared.

## Behavior

- Each aggregation case carries `handwritten`: a formula document and predicate
  table written by hand (not produced by the expansion code) and a
  `population` declaration: a finite set of valuations per member signal and
  trace lengths, so that the number of compared traces is computable from the
  declaration.
- The comparison itself (evaluating both on every trace of the population)
  executes in tl-mltl (FR-054), because tl-syntax evaluates nothing; tl-syntax
  owns the case, the population declaration and the structural check that the
  expansion's signal reads and propositions are a subset of the hand-written
  form's.
- Small-scope population (declared in the manifest): members are Boolean
  signals, trace length 1, `k` in 0..=5 members, `n` in 0..=6. A case with `k`
  members has `2^k` traces. `for_each` and `for_some` each cover `k` in 0..=5:
  `1+2+4+8+16+32 = 63` traces. `at_least` covers all 42 pairs `(k, n)`: for each
  `n` the 63 traces, so `7 x 63 = 441`. The manifest records the total
  `63 + 63 + 441 = 567` compared traces, and the computed count must equal it.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-308-AC-1 | Every aggregation case has a hand-written form and a population declaration whose computed trace count equals the manifest's recorded count. | integration-testing (TC-1520) |
| FR-308-AC-2 | The structural subset check passes for every case and a deliberately altered expansion (member omitted) fails it. | integration-testing (TC-1521) |

## Dependencies

FR-307 supplies the expansion; tl-mltl FR-054 executes the equivalence.
