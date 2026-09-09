---
id: PLAN-006
title: Qualification-integrity ownership plan
type: Plan
status: in_progress
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: references
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# PLAN-006: Qualification-integrity ownership plan

## Objective

Close `agent-ix/tl-syntax#19` by giving every surviving qualification and
shared-assurance obligation an explicit requirement owner, verification method,
and lifecycle boundary. Keep FR-006 functional, keep NFR-002 about deterministic
domain artifacts, and reuse the released Engineering Assurance, Quire, and
Quoin contracts without adding repository-local assurance machinery.

## Base and dependency

Issue #20's implementation merged as PR #22. This branch is rebased onto the
resulting current mainline (`31c6ced6567645b637be20bef5c256d0ea4ed45c`) so
#19 reconciles its FR-006 changes with the reviewed tree rather than retaining
a stack dependency. Independent review of this branch's exact head remains
required before merge.

## Work sequence

1. Specify NFR-003, its ownership table, five live criteria, the disclosed Make
   limitation, and the stable-release qualified-record trigger.
2. Reconcile FR-006 and NFR-002 without reusing retired identifiers or implying
   blanket succession from deleted local controls.
3. Add NFR-003 to the shared change declaration and Quire source set; add a
   focused TC-038 boundary check and extend existing trace bindings.
4. Update the exact live-source census for the new requirement path and prove
   no local runner, collector, parser, envelope, identity registry, retention
   store, or alternate contract appeared.
5. Run focused checks, strict Quire validation and coverage, then the complete
   local gate at the exact final head. Hosted CI remains manual-only and is not
   dispatched.
6. Perform author code review and gap analysis, then request independent review
   of the exact head after the dependency has landed.

## Exit criteria

1. Every requested control has one explicit qualification owner, verification
   method, and lifecycle boundary.
2. SUITE-008 remains a local exact-head result and is not claimed by Quoin.
3. Make suppression and the stable qualified-record requirement remain named,
   unclosed, and tied to their trackers and first-stable trigger.
4. Retired FR-006-AC-4, NFR-002-AC-4, TC-018, TC-024, and SUITE-007 retain their
   historical meanings.
5. Strict specification and coverage gates and the complete local CI pass at
   the final exact head; independent review grants any merge authority.
