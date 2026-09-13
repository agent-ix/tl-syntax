---
id: TM-002
title: "Future-time operator-profile test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: covers
---

# Future-time operator-profile test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-008 | FR-008-AC-1 through FR-008-AC-4 | TC-040, TC-041, TC-042, TC-044, TC-046 | 🚧 planned |
| FR-009 | FR-009-AC-1 through FR-009-AC-4 | TC-040, TC-043, TC-044, TC-046, TC-047 | 🚧 planned |
| FR-010 | FR-010-AC-1 through FR-010-AC-4 | TC-041 through TC-046, TC-074 | 🚧 planned |

Each row stays planned until every listed test case is implemented. Issue #40
implements TC-040, TC-041, TC-042, and TC-046 in `tests/future_lowering.rs`,
and the tl-syntax portion of TC-044: lowered and direct formula-v1 documents
are equal, byte-identical on the wire, and semantically identical under both
profiles, and the 100,000-node limit holds. The typed request carries one
kind and one semantic-profile identity, so TC-046 refuses dense, timestamped,
unit-bearing, and window-closure selections as unknown semantic profiles;
distinct source-level refusals are TC-047's. TC-044 stays planned until the
evaluation, prefix-progress, horizon, and resource evidence owned downstream
lands; TC-043, TC-045, and TC-047 are owned by the parser, rewrite, and
interoperability follow-ons.

Issue #41 implements TC-074 in `tests/future_operator_corpus.rs` over the
`tl-syntax.future-operator-corpus/v1` corpus in `corpus/future-operators/`.
Each derived-source case binds the `tl-parse.clean-ascii/v2` dialect, the
operator and semantic profiles, the source text, and its lowering steps to one
span-free expected formula-v1 document that a directly constructed case shares.
The replay builds both through the tl-syntax lowering API only; it is evidence
input, not an evaluator or an editable source language. FR-010 stays planned
until TC-044 and TC-045 land.

## Operator and profile evidence allocation

Every cell is required before the corresponding follow-on can claim the row.
`canonical` means the component consumes the lowered F/G/U/R graph and has no
derived semantic branch.

| Row | Parse / format | Wire | Evaluate / progress | Horizon | Rewrite | Corpus | Interoperability |
|---|---|---|---|---|---|---|---|
| `W[a,b]` + `mltl.closed-trace/v1` | TC-043, TC-047 | TC-040 | TC-041, TC-044 | TC-044 | TC-045 canonical | TC-074 paired | TC-045 lowered/exported |
| `W[a,b]` + `mltl.online-prefix/v1` | TC-043, TC-047 | TC-040 | TC-041, TC-044 | TC-044 | TC-045 canonical | TC-074 paired | TC-045 lowered/exported |
| `M[a,b]` + `mltl.closed-trace/v1` | TC-043, TC-047 | TC-040 | TC-042, TC-044 | TC-044 | TC-045 canonical | TC-074 paired | TC-045 lowered/exported |
| `M[a,b]` + `mltl.online-prefix/v1` | TC-043, TC-047 | TC-040 | TC-042, TC-044 | TC-044 | TC-045 canonical | TC-074 paired | TC-045 lowered/exported |
| old `tl-parse.clean-ascii/v1` | reject derived input; accept primitive canonical output in TC-043 | unchanged in TC-040 | canonical only | canonical only | canonical only | TC-074 refuses a derived step under the v1 dialect | not a source-profile claim |
| unknown/refused combinations | TC-046, TC-047 typed refusal | TC-046 no document | not evaluated | not analyzed | not rewritten | TC-046 negative cases; TC-074 refused and malformed cases | TC-045 unsupported/unavailable |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-040 | Bind the closed operator catalog, identified request/report/refusal contracts, unchanged formula-v1 wire vocabulary, and orthogonal operator/dialect/semantic-profile identities | Integration | P0 | FR-008-AC-1, FR-009-AC-2, FR-009-AC-3 | ✅ implemented |
| TC-041 | Lower bounded weak until in U/G/Or order, retain attribution, and compare interval/span/profile boundaries with direct construction | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-010-AC-4 | ✅ implemented |
| TC-042 | Lower bounded strong release in R/F/And order, retain attribution, and compare interval/span/profile boundaries with direct construction | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-010-AC-4 | ✅ implemented |
| TC-043 | Parse `tl-parse.clean-ascii/v2` with exact precedence, associativity, interval and token/expression-span rules; reject malformed forms and normalize to old-v1 primitive text | Integration | P0 | FR-009-AC-1, FR-009-AC-3, FR-010-AC-4 | 🚧 planned |
| TC-044 | Prove direct/lowered wire, semantic identity, evaluation, prefix progress, horizon, resource, report-identity, and no-partial-mutation behavior under both profiles | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-009-AC-2, FR-009-AC-3, FR-010-AC-1, FR-010-AC-4 | 🚧 planned |
| TC-045 | Keep rewrite, native bridge, R2U2/C2PO, and output-only FRETish paths canonical and preserve every unsupported/unavailable result | Integration | P0 | FR-010-AC-2, FR-010-AC-3, FR-010-AC-4 | 🚧 planned |
| TC-046 | Generate every raw admission-field failure and require the exact refusal precedence; include unknown request identity, semantic-profile/formula mismatch, an operand absent from the borrowed validated formula, and the fixed 100,000-node formula-v1 limit; refuse every unknown, over-limit, next, past, missing-interval, open-bound, unknown-semantic-profile, and derived-wire combination before construction; at this typed boundary dense, timestamped, unit-bearing, and window-closure selections are unknown semantic profiles, and source-level distinct refusals for them and for weak-next and mixed-time forms belong to TC-047 | Property | P0 | FR-008-AC-1, FR-008-AC-3, FR-009-AC-4, FR-010-AC-4 | ✅ implemented |
| TC-047 | Fuzz `tl-parse.clean-ascii/v2` so arbitrary bytes either produce the specified lowered graph/report or a bounded diagnostic without unwinding or v1-dialect misattribution | Fuzz | P1 | FR-009-AC-1, FR-009-AC-4 | 🚧 planned |
| TC-074 | Replay the digest-pinned paired W/M corpus through the tl-syntax lowering API so every derived-source case and its directly constructed pair produce one shared span-free canonical document under its bound dialect, operator, and semantic profile, every refused and malformed case fails with its declared code, and mutating a lowering branch, generated-node order, inclusive endpoint, associativity, profile, node charge, span attribution, refusal, or pinned byte turns the replay red | Integration | P0 | FR-010-AC-2, FR-010-AC-4 | ✅ implemented |
