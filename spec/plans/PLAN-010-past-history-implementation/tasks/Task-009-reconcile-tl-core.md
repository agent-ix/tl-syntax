---
id: Task-009
title: "Reconcile the TL core with the subsystem architecture"
type: Task
status: not_started
track: Core
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-001
    type: references
  - target: ix://agent-ix/tl-syntax/Task-002
    type: references
  - target: ix://agent-ix/tl-syntax/Task-003
    type: references
  - target: ix://agent-ix/tl-syntax/Task-004
    type: references
  - target: ix://agent-ix/tl-syntax/Task-005
    type: references
  - target: ix://agent-ix/tl-syntax/TC-054
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-055
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-057
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-058
    type: verifies
---
# Task-009: Reconcile the TL core with the subsystem architecture

## Scope

Bring the already-landed `tl-syntax`, `tl-parse`, `tl-mltl`, and `tl-rewrite`
implementations into conformance with Task-008's reviewed subsystem and object
model while preserving every accepted wire identity and compatibility promise.

## Subtasks

- [ ] Split flat or monolithic modules only where the architecture review finds
  mixed ownership or change coupling; retain stable public re-exports.
- [ ] Centralize owner contract metadata and eliminate duplicated identity,
  limit, canonicalization, or state vocabularies across crates.
- [ ] Reconcile strict public readers for formula, signal/proposition,
  history, trace, request, evaluator report, rewrite, corpus, and dependency
  manifest artifacts.
- [ ] Re-pin every production edge to the reviewed owner revision and replay
  the complete shared corpus after structural changes.
- [ ] Run code review, Rust review, and gap analysis in every changed crate and
  fix all findings before merge.

## Deliverables

- Cohesive TL core module/API organization with unchanged accepted behavior.
- Exact dependency pins, public reader inventory, and complete corpus replay.

## Notes

- Existing Task-001 through Task-005 work is preserved and remains credited.
  This task owns only architecture-required corrections, not a rewrite for its
  own sake.
- GitHub trackers: `tl-syntax#65`, `tl-parse#38`, `tl-mltl#66`, and
  `tl-rewrite#41`.
