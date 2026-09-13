---
id: PLAN-008
title: Future-operator lowering plan
type: Plan
status: in_progress
relationships:
  - target: ix://agent-ix/tl-syntax/FR-008
    type: references
  - target: ix://agent-ix/tl-syntax/FR-009
    type: references
  - target: ix://agent-ix/tl-syntax/FR-010
    type: references
  - target: ix://agent-ix/tl-syntax/TM-002
    type: references
---

# PLAN-008: Future-operator lowering plan

## Objective

Close `agent-ix/tl-syntax#40` by implementing the FR-008 allocation-free
admission and lowering of bounded weak until (`W`) and strong release (`M`) into
the unchanged primitive formula-v1 graph, with identified request, report, and
refusal contracts. Issue #40 owns the tl-syntax evidence: TC-040, TC-041,
TC-042, TC-046, and the tl-syntax portion of TC-044.

## Base

Branch `issue/40-future-lowering` starts at mainline `8d3ff98`, the merge of the
reviewed future-profile specification (issue #32, PR #37).

## Work sequence

1. Add a `no_std` core module that admits raw borrowed request fields in the
   FR-008 refusal precedence into a private typed request, then lowers it
   totally into three appended nodes and a typed report.
2. Move `MAX_FORMULA_DOCUMENT_NODES` into the core so the no-alloc path enforces
   the formula-v1 node limit with the fixed three-node charge.
3. Implement the traced tests: exact identities and closed catalog, W and M
   direct-construction properties, zero allocation, lowered/direct wire and
   semantic identity, the node-limit boundary, fault-injection precedence, and
   mutation controls.
4. Update the exact live-source census and the TM-002 test-case statuses; keep
   every FR row planned until its downstream test cases land.
5. Run focused checks, strict Quire validation and coverage, independent code
   review, gap analysis, and the complete local gate at the exact final head.
   Hosted CI remains manual-only and is not dispatched.

## Exit criteria

1. `W` lowers to U/G/Or and `M` to R/F/And with operand reuse, exact node ids,
   expression-span attribution, and a report carrying both spans.
2. Every raw field fault refuses in the stable FR-008 precedence before any
   construction, without allocation.
3. TC-040, TC-041, TC-042, and TC-046 are implemented and bound; TC-044 remains
   planned with its tl-syntax portion recorded.
4. Strict specification and coverage gates and the complete local CI pass at
   the final exact head; independent review grants any merge authority.
