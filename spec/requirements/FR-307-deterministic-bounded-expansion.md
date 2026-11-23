---
id: FR-307
title: Expand aggregation deterministically within a bound
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-306
    type: depends_on
  - target: ix://agent-ix/tl-syntax/NFR-019
    type: depends_on
---

# FR-307: Expand aggregation deterministically within a bound

## Description

When a validated aggregation document is expanded, tl-syntax shall return
ordinary formula nodes and predicate definitions whose content is a pure,
byte-reproducible function of the document, and shall refuse an expansion above
the declared bound before building any node.

## Outputs

- `ExpandedAggregation { nodes, predicates, root, report }` where the report
  carries the exact node and predicate counts and the member count; or a typed
  `AggregationRefusal`.

## Behavior

Let `k` be the member count and `t` the template node count after removing its
leaf-only prefix (the template's own nodes).

- **Instantiation.** For member `m` in document order, copy the template nodes
  in template order, replacing `slot_signal` by that member's signal read and
  `slot_proposition` by its bound proposition, and remapping each
  member-dependent predicate to a fresh identity `first_proposition + index`,
  where `index` counts member-dependent predicates in (member, template) order.
  Member-independent template nodes and predicates are shared across members and
  appear once. The root of instance `m` is `r_m`.
- **`for_each`.** The conjunction of `r_1..r_k` as a balanced tree: split the
  ordered list at `ceil(len/2)` recursively; each internal node is an `And`
  whose left operand is the left half. Binary nodes added: `k - 1`. Empty set:
  one `True` node.
- **`for_some`.** The same shape with `Or`. Empty set: one `False` node.
- **`at_least(n)`**, `1 <= n <= k`. Cell `T(i, j)`, `i` in `1..=k+1`, `j` in
  `0..=n`, means "at least `j` of `r_i..r_k` hold": `T(i, 0)` is the shared
  `True` node; `T(i, j)` with `j > k - i + 1` is the shared `False` node; otherwise
  `T(i, j) = Or(And(r_i, T(i+1, j-1)), T(i+1, j))`, nodes created `And` first
  then `Or`, cells in increasing `i`, then increasing `j`. Only cells reachable
  from `T(1, n)` are built. For each `j` in `1..=n` the reachable `i` run from
  `n - j + 1` to `k - j + 1`, so the reachable cells number exactly
  `n * (k - n + 1)` and the nodes added are at most `2 * n * (k - n + 1) + 2`
  (two per cell plus the two shared constants), and exactly that when no
  constant is unreferenced.
  `at_least(0)` is `True`; `at_least(n)` with `n > k` is `False`.
  Worked case `k = 3, n = 2`: reachable cells `(1,2) (2,2) (2,1) (3,1)`,
  8 cell nodes plus `True` and `False` = 10 = `2*2*(3-2+1)+2`.
- **Bound.** Before building, compute with checked arithmetic the node count,
  predicate count and total work; refuse with `AggregationTooLarge` when nodes
  exceed `MAX_AGGREGATION_NODES` (100,000, never above the formula-document
  node limit), predicates exceed 100,000, members exceed 1,024, or the
  arithmetic overflows `usize`. No partial output exists.
- **No-std.** The expansion core accepts caller-owned output slices and reports
  the exact counts needed (`ExpansionTooSmall`) without allocating; an owned
  convenience under `alloc` sizes them from the pre-computed counts.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-307-AC-1 | The same document expands to byte-identical nodes and predicates on repeated runs and across platforms, and the node/predicate counts equal the closed forms above on the declared (k, n) grid, including `k = 0`, `n = 0`, `n > k`, `n = k`. | property-based-testing (TC-1516, TC-1517) |
| FR-307-AC-2 | An expansion above any bound, including `k = 1,025` and a `(k, n)` whose count exceeds 100,000, is refused with `AggregationTooLarge` before any output is written; a one-below-the-limit case succeeds. | unit-testing (TC-1518) |
| FR-307-AC-3 | `ExpansionTooSmall` reports exact required counts and writes nothing; the no-std core builds without `alloc`. | unit-testing (TC-1519) |

## Dependencies

FR-306 supplies the document. NFR-019 bounds work and determinism.
