---
id: FR-008
title: "Lower derived future-time operators"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-002
    type: depends_on
---

# FR-008: Lower derived future-time operators

## Description

When an internal temporal adapter requests an admitted derived future-time
operator, tl-syntax shall return its exact canonical-node expansion without
allocating, mutating caller state, cloning operand graphs, or introducing a new
semantic implementation.

The closed operator-profile identity is `tl-syntax.future-operators/v1`. It is
an internal representation contract, not an editable formal-clause language.

## Inputs

- The exact operator-profile identity and one admitted derived kind.
- The selected existing `SemanticProfile` value.
- Two already-validated Boolean operand roots and the existing canonical node
  count at which generated nodes would begin.
- One checked inclusive discrete interval `[a,b]` satisfying
  `0 <= a <= b <= u32::MAX`.
- Checked operator-token and full-expression source spans that are either both
  absent or both present; a present token span is contained in the expression
  span.

## Outputs

- Exactly three topologically ordered generated `Node` values in a fixed-size
  result, their absolute node identities and output root, or one typed refusal.
- The selected semantic profile unchanged.
- A deterministic, non-wire report naming the operator profile, derived kind,
  operand roots, output root, generated node range and count, and supplied
  token/expression spans.

## Behavior

The v1 taxonomy is closed:

| Class | Operators | Representation |
|---|---|---|
| atomic | `false`, `true`, proposition | existing canonical nodes |
| Boolean | `!`, `&`, `&#124;`, `->`, `<->` | existing canonical nodes; implication and equivalence remain v1 convenience nodes |
| primitive future-time | `F[a,b]`, `G[a,b]`, `U[a,b]`, `R[a,b]` | existing canonical temporal nodes |
| derived future-time | `W[a,b]`, `M[a,b]` | the exact lowerings below; never node or wire variants |

For already-lowered operand roots `p` and `q`, the returned node order is:

| Derived form | Canonical result | Generated nodes |
|---|---|---:|
| `p W[a,b] q` | disjunction of `p U[a,b] q` and `G[a,b] p` | `U`, `G`, then `Or` |
| `p M[a,b] q` | `(p R[a,b] q) & F[a,b] p` | `R`, `F`, then `And` |

These equations define bounded weak until and bounded strong release for this
profile. No component may implement an independent truth table, evaluator
branch, horizon rule, rewrite rule, or wire variant for them. The meaning of
`U` retains the ecosystem convention that `p` is required from offset `a`
through the instant immediately before the witness for `q`; offsets before `a`
are irrelevant.

Before constructing a result, lowering checks the exact profile and kind, both
operand identities are below `existing_node_count`, the interval, the paired
span state and containment, `existing_node_count + 3`, the formula document
node limit, and conversion of all three generated identities to `NodeId`.
Failure returns one typed refusal and no fixed-size result, so the caller is the
only graph-storage owner and cannot observe a partially appended expansion.

Both operands are referenced and never copied. All three generated nodes carry
the full expression span when present because no smaller source expression
denotes either supporting node. The report separately retains the exact
operator-token span. If spans are absent, none is invented. The report is
diagnostic attribution, not formula semantic identity and not a serialized
document.

Distinct refusals cover unknown profile, unknown or unsupported kind, missing
or malformed interval, invalid operand root, partial/inverted/non-contained
span state, count arithmetic overflow, `NodeId` overflow, and expansion beyond
the formula document node limit.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-008-AC-1 | The closed v1 catalog classifies every admitted operator and rejects every unknown profile or operator without fallback. | Test (TC-040, TC-046) |
| FR-008-AC-2 | For all valid operands, intervals, counts, spans, and existing semantic profiles, W generates exactly U/G/Or and M generates exactly R/F/And in the specified order, reuses both operand graphs, and preserves the selected profile. | Test (TC-041, TC-042, TC-044) |
| FR-008-AC-3 | The default-feature lowering API preflights every input and returns either one fixed-size three-node result with absolute identities or one distinct typed refusal without allocation or caller mutation. | Test (TC-041, TC-042, TC-044, TC-046) |
| FR-008-AC-4 | A successful report preserves profile, kind, operands, generated range/root/count, and paired token/expression spans; generated nodes carry only the expression span, and equal inputs return structurally equal nodes and reports. | Test (TC-041, TC-042, TC-044) |

## Dependencies

FR-001 supplies checked inclusive intervals. FR-002 supplies validated
topological graphs and `NodeId`. FR-009 governs the profile and wire
compatibility of the returned canonical nodes; FR-010 governs downstream use.
