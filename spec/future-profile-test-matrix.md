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

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-008 | FR-008-AC-1 through FR-008-AC-4 | TC-040, TC-041, TC-042, TC-044, TC-046 | 🚧 planned |
| FR-009 | FR-009-AC-1 through FR-009-AC-4 | TC-040, TC-043, TC-044, TC-046, TC-047 | 🚧 planned |
| FR-010 | FR-010-AC-1 through FR-010-AC-4 | TC-041 through TC-046 | 🚧 planned |

## Operator and profile evidence allocation

Every cell is required before the corresponding follow-on can claim the row.
`canonical` means the component consumes the lowered F/G/U/R graph and has no
derived semantic branch.

| Row | Parse / format | Wire | Evaluate / progress | Horizon | Rewrite | Corpus | Interoperability |
|---|---|---|---|---|---|---|---|
| `W[a,b]` + `mltl.closed-trace/v1` | TC-043, TC-047 | TC-040 | TC-041, TC-044 | TC-044 | TC-045 canonical | TC-045 paired | TC-045 lowered/exported |
| `W[a,b]` + `mltl.online-prefix/v1` | TC-043, TC-047 | TC-040 | TC-041, TC-044 | TC-044 | TC-045 canonical | TC-045 paired | TC-045 lowered/exported |
| `M[a,b]` + `mltl.closed-trace/v1` | TC-043, TC-047 | TC-040 | TC-042, TC-044 | TC-044 | TC-045 canonical | TC-045 paired | TC-045 lowered/exported |
| `M[a,b]` + `mltl.online-prefix/v1` | TC-043, TC-047 | TC-040 | TC-042, TC-044 | TC-044 | TC-045 canonical | TC-045 paired | TC-045 lowered/exported |
| old `tl-parse.clean-ascii/v1` | reject derived input; accept primitive canonical output in TC-043 | unchanged in TC-040 | canonical only | canonical only | canonical only | compatibility pair in TC-045 | not a source-profile claim |
| unknown/refused combinations | TC-046, TC-047 typed refusal | TC-046 no document | not evaluated | not analyzed | not rewritten | TC-046 negative cases | TC-045 unsupported/unavailable |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-040 | Bind the closed operator catalog, identified request/report/refusal contracts, unchanged formula-v1 wire vocabulary, and orthogonal operator/dialect/semantic-profile identities | Integration | P0 | FR-008-AC-1, FR-009-AC-2, FR-009-AC-3 | 🚧 planned |
| TC-041 | Lower bounded weak until in U/G/Or order, retain attribution, and compare interval/trace/profile boundaries with direct construction | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-010-AC-4 | 🚧 planned |
| TC-042 | Lower bounded strong release in R/F/And order, retain attribution, and compare interval/trace/profile boundaries with direct construction | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-010-AC-4 | 🚧 planned |
| TC-043 | Parse `tl-parse.clean-ascii/v2` with exact precedence, associativity, interval and token/expression-span rules; reject malformed forms and normalize to old-v1 primitive text | Integration | P0 | FR-009-AC-1, FR-009-AC-3, FR-010-AC-4 | 🚧 planned |
| TC-044 | Prove direct/lowered wire, semantic identity, evaluation, prefix progress, horizon, resource, report-identity, and no-partial-mutation behavior under both profiles | Property | P0 | FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-009-AC-2, FR-009-AC-3, FR-010-AC-1, FR-010-AC-4 | 🚧 planned |
| TC-045 | Keep rewrite, paired corpus, native bridge, R2U2/C2PO, and output-only FRETish paths canonical and preserve every unsupported/unavailable result | Integration | P0 | FR-010-AC-2, FR-010-AC-3, FR-010-AC-4 | 🚧 planned |
| TC-046 | Generate every raw admission-field failure and require the exact refusal precedence; refuse every unknown, over-limit, strong/weak-next, past/mixed-time, unbounded/open/dense/timestamped/unit-bearing, window-closure, and derived-wire combination before construction | Property | P0 | FR-008-AC-1, FR-008-AC-3, FR-009-AC-4, FR-010-AC-4 | 🚧 planned |
| TC-047 | Fuzz `tl-parse.clean-ascii/v2` so arbitrary bytes either produce the specified lowered graph/report or a bounded diagnostic without unwinding or v1-dialect misattribution | Fuzz | P1 | FR-009-AC-1, FR-009-AC-4 | 🚧 planned |
