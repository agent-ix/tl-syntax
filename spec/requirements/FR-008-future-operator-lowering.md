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

The public boundary receives an allocation-free borrowed
`tl-syntax.future-lowering-request/v1` admission request. Its fields preserve
invalid states until this component has classified them:

- byte slices bounded to 128 bytes for operator-profile and semantic-profile
  identities and 16 bytes for derived kind, so unknown and non-UTF-8 values
  remain representable and over-limit identity is a distinct refusal;
- two raw `u64` operand identities and raw `u64` existing-node/document-limit
  counts;
- an optional pair of raw `u64` interval bounds, so missing, inverted, and
  values beyond `u32::MAX` remain distinct; and
- independently optional raw token/expression spans with `u64` endpoints, so
  missing halves, inverted spans, non-contained tokens, and `u32` conversion
  overflow remain representable.

Successful admission produces a private typed request containing the exact
`tl-syntax.future-operators/v1` identity, W or M, one existing
`SemanticProfile`, validated operand roots, checked `[a,b]`, checked paired
spans, and preflighted counts. The total lowerer accepts only that typed value.

## Outputs

- Exactly three topologically ordered generated `Node` values in a fixed-size
  result, their absolute node identities and output root, or one typed refusal.
- The selected semantic profile unchanged.
- A deterministic `tl-syntax.future-lowering-report/v1` non-wire Rust value
  naming the request identity, operator profile, derived kind, semantic
  profile, operand roots, output root, generated node range and count, and
  supplied token/expression spans; or one
  `tl-syntax.future-lowering-refusal/v1` value.

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

Admission checks, in stable precedence order: request identity; operator
profile; known-but-unsupported versus unknown kind; semantic-profile identity;
interval presence, `u32` range and order; operand range; span pair, endpoint
range, order and containment; `existing_node_count + 3`; conversion of every
generated identity to `NodeId`; then the formula-document node limit. A caller
therefore receives exactly one refusal even when several raw fields are bad.
Recognized X or past-time kinds are `unsupported_kind`; an unrecognized kind is
`unknown_kind`. Text tokenization and grammar failures remain owned by
tl-parse, while this request boundary owns decoded identity/value admission.
Failure returns no typed request and no fixed-size result, so the caller is the
only graph-storage owner and cannot observe a partial append.

Both operands are referenced and never copied. All three generated nodes carry
the full expression span when present because no smaller source expression
denotes either supporting node. The report separately retains the exact
operator-token span. If spans are absent, none is invented. The report is
diagnostic attribution, not formula semantic identity and not a serialized
document.

The request, report, and refusal identities are owned by `tl-syntax`; changing
their fields, refusal precedence, or meaning requires a successor identity and
Rust API compatibility review. None is a formula-wire or authored-source
identity.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-008-AC-1 | The raw v1 admission boundary represents and classifies every valid, unsupported, unknown, missing, malformed, over-range, and inconsistent field in the stated precedence, while the closed operator catalog admits only W/M without fallback. | Test (TC-040, TC-046) |
| FR-008-AC-2 | For all valid operands, intervals, counts, spans, and existing semantic profiles, W generates exactly U/G/Or and M generates exactly R/F/And in the specified order, reuses both operand graphs, and preserves the selected profile. | Test (TC-041, TC-042, TC-044) |
| FR-008-AC-3 | The default-feature admission/lowering API allocates nothing, preflights every raw and typed input, and returns either one fixed-size three-node result with absolute identities or one identified typed refusal without caller mutation. | Test (TC-041, TC-042, TC-044, TC-046) |
| FR-008-AC-4 | A successful identified report preserves request/profile/kind/operand/generated-range/root/count and paired token/expression attribution; generated nodes carry only the expression span, and equal inputs return structurally equal nodes and reports. | Test (TC-041, TC-042, TC-044) |

## Dependencies

FR-001 supplies checked inclusive intervals. FR-002 supplies validated
topological graphs and `NodeId`. FR-009 governs the profile and wire
compatibility of the returned canonical nodes; FR-010 governs downstream use.
