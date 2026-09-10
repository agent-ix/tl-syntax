---
id: Task-005
title: "Preserve lifecycle, retention and human decision authority"
type: Task
status: blocked
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-015
    type: references
  - target: ix://agent-ix/tl-syntax/FR-017
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: references
  - target: ix://agent-ix/tl-syntax/TC-062
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-063
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-067
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-070
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-073
    type: verifies
---
# Task-005: Preserve lifecycle, retention and human decision authority

## Scope

Implement stage/history projection, retention verification, bounded
supersession and policy-backed decision admission without automated promotion.

## Subtasks

- [ ] Write state-machine properties for stage, retention and decision domains.
- [ ] Implement bounded same-subject supersession/conflict traversal.
- [ ] Verify real retention handles and map every retrieval failure.
- [ ] Admit complete decision-event sets against policy, quorum and independence facts.

## Deliverables

- Rust lifecycle/retention/decision adapters
- TC-062, TC-063, TC-067, TC-070 and TC-073 evidence

## Notes

- Blocked until Task-002 selects the retention backend/operator and authoritative policy/event source.
