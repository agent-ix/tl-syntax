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

- `ExpandedAggregation { nodes, predicates, expressions, root, report }` where the
  report carries the exact node, predicate and expression-node counts and the
  member count; or a typed
  `AggregationRefusal`.

## Behavior

Let `k` be the member count, `t` the template node count after removing its
leaf-only prefix (the template's own nodes) and `e` the count of expression nodes
in the template that depend on the member (those reachable from a `slot_signal`).

- **Instantiation.** For member `m` in document order, copy the template nodes
  in template order, replacing `slot_signal` by that member's signal read and
  `slot_proposition` by its bound proposition, and remapping each
  member-dependent predicate to a fresh identity `first_proposition + index`,
  where `index` counts member-dependent predicates in (member, template) order.
  Each member-dependent expression node is also copied once per member (`k * e`
  expression nodes in all, written to the expression arena in (member, template)
  order). Member-independent template nodes, predicates and expression nodes are
  shared across members and appear once. The root of instance `m` is `r_m`.
- **Identity precedence.** There is one allocator of fresh proposition identities
  per document: this expander, from `first_proposition`, and only for
  member-dependent predicates. A caller that also allocates identities itself (the
  tl-parse text dialect, FR-029 there) shall set `first_proposition` strictly above
  every identity already used in the document (catalog bindings, explicit atoms, its
  own predicate propositions), and several quantified templates in one document take
  consecutive ranges in source order. Any remaining overlap is `FreshIdentityCollision`
  (FR-306); the expander never renumbers a caller's identity.
- **Nested quantifiers.** The inner expansion is applied inside each outer
  instance, in (outer member, inner member, template) order, from the same
  allocator. The counts of nodes, predicates and expression nodes are computed over
  the whole composition, so the bound below applies to the product of the member
  counts times the template size and never to the levels separately.
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
  predicate count, expression-node count (including the `k * e` per-member copies)
  and total work; refuse with `AggregationTooLarge` when nodes exceed
  `MAX_AGGREGATION_NODES` (100,000, never above the formula-document node limit),
  predicates exceed 100,000, expression nodes exceed 100,000 (the FR-302 table
  limit), a set's members exceed 1,024, or the arithmetic overflows `usize`. No
  partial output exists.
- **No-std.** The expansion core accepts caller-owned output slices and reports
  the exact counts needed (`ExpansionTooSmall`) without allocating; an owned
  convenience under `alloc` sizes them from the pre-computed counts.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-307-AC-1 | The same document expands to byte-identical nodes, predicates and expression nodes on repeated runs and across platforms, and the node, predicate and expression-node counts equal the closed forms above on the declared (k, n) grid, including `k = 0`, `n = 0`, `n > k`, `n = k`. | property-based-testing (TC-1516, TC-1517) |
| FR-307-AC-2 | An expansion above any bound, including `k = 1,025`, a `(k, n)` whose node count exceeds 100,000, a template whose `k * e` expression nodes exceed 100,000 while nodes and predicates stay under their limits, and a nested pair whose product exceeds a bound while each level alone does not, is refused with `AggregationTooLarge` before any output is written; a one-below-the-limit case succeeds. | unit-testing (TC-1518) |
| FR-307-AC-3 | `ExpansionTooSmall` reports exact required counts and writes nothing; the no-std core builds without `alloc`. | unit-testing (TC-1519) |
| FR-307-AC-4 | In a document that mixes caller-allocated identities with two quantified templates, the fresh ranges are consecutive in source order and strictly above every caller identity; a range that would overlap one is refused with `FreshIdentityCollision`, and nesting deeper than 2 is refused with `AggregationTooDeep`. | unit-testing (TC-1538) |

## Dependencies

FR-306 supplies the document. NFR-019 bounds work and determinism.
