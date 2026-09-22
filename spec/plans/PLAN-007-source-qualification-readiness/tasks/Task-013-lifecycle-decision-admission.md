---
id: Task-013
title: "Preserve lifecycle, retention and human decision authority"
type: Task
status: blocked
track: A
priority: P0
owner_repository: agent-ix/tl-syntax
consumer_repositories: [agent-ix/tl-syntax]
evidence_method: state-machine-and-integration-test
github_issue: ix://agent-ix/tl-syntax/issues/48
resume_conditions: [ix://agent-ix/tl-syntax/issues/45, ix://agent-ix/tl-syntax/issues/47]
relationships:
  - target: ix://agent-ix/tl-syntax/Task-018
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-020
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-016
    type: references
  - target: ix://agent-ix/tl-syntax/FR-018
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
# Task-013: Preserve lifecycle, retention and human decision authority

## Scope

Implement stage/history projection, retention verification, bounded
supersession and policy-backed decision admission without automated promotion.

## Subtasks

- [ ] Write state-machine properties for stage, retention and decision domains.
- [ ] Implement bounded same-subject supersession/conflict traversal.
- [ ] Verify real retention handles and map every retrieval failure.
- [ ] Admit complete decision-event sets against policy, quorum and independence facts.
- [ ] Refuse incomplete/cross-snapshot event enumeration and bind expiry to the
  verified authoritative evaluation-time contract.

## Deliverables

- Rust lifecycle/retention/decision adapters
- TC-062, TC-063, TC-067, TC-070 and TC-073 evidence

## Notes

- Blocked until Task-018 selects the retention backend/operator and
  authoritative policy/event source.
