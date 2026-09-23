---
id: FR-021
title: "Represent fairness premises without evaluating them"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-020
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-289
    type: depends_on
---

# FR-021: Represent fairness premises without evaluating them

## Description

When a caller supplies fairness premises for an infinite-trace formula,
tl-syntax shall construct a `tl-syntax.fairness-premises/v1` document that
references validated roots in the same canonical formula-unbounded graph.

## Inputs

- A formula-unbounded graph under `mltl.infinite-trace/v1` and a finite ordered
  set of premise-root node identities.
- The exact clock binding of that graph.

## Outputs

- An immutable ordered fairness document with its schema, profile, graph, and
  clock identities, or a typed refusal.

## Behavior

Each premise denotes a condition required infinitely often on an admissible
infinite trace. The syntax layer validates identity, ordering, uniqueness, and
graph membership only; the provider specified by TL-210 interprets premises
on a lasso or other supported infinite trace. An empty set means no fairness
assumption. Duplicate roots, roots outside the graph, a non-infinite profile,
or a mismatched clock refuse before a fairness document is produced. No finite
prefix is declared fair merely because its observed part satisfies a premise.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-021-AC-1 | Empty and nonempty ordered, unique premise sets round-trip with exact graph, profile, and clock identities. | Test (TC-151) |
| FR-021-AC-2 | Duplicate, foreign, or invalid roots and profile or clock mismatches yield distinct typed refusals and no document. | Test (TC-152) |
| FR-021-AC-3 | The syntax layer does not turn a finite prefix or an unevaluated premise into a fairness or liveness verdict. | Test (TC-153) |

## Dependencies

FR-020 binds the profile and clock; FR-289 supplies the canonical graph.
