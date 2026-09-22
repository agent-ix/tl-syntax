---
id: Task-010
title: "Admit external prerequisites without local substitutes"
type: Task
status: blocked
track: G
priority: P0
owner_repository: agent-ix/tl-syntax
consumer_repositories: [agent-ix/tl-syntax]
evidence_method: released-contract-admission
github_issue: ix://agent-ix/tl-syntax/issues/45
resume_conditions: [ix://agent-ix/tl-syntax/issues/16, ix://agent-ix/quire-research/issues/64, ix://agent-ix/engineering-assurance/issues/11]
relationships:
  - target: ix://agent-ix/tl-syntax/Task-009
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-015
    type: references
  - target: ix://agent-ix/tl-syntax/FR-016
    type: references
  - target: ix://agent-ix/tl-syntax/FR-017
    type: references
  - target: ix://agent-ix/tl-syntax/FR-018
    type: references
  - target: ix://agent-ix/tl-syntax/FR-019
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: references
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-067
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-073
    type: verifies
---
# Task-010: Admit external prerequisites without local substitutes

## Scope

Maintain the immutable compatibility/authority ledger and admit each external
capability only when its released identity satisfies MRS-004.

## Subtasks

- [ ] Admit a released matrix selecting the source-grounded Quire export.
- [ ] Select the Quoin retention contract, backend/operator and lifecycle.
- [ ] Select the immutable reviewer/decision policy and authoritative event source.
- [ ] Admit the Engineering Assurance Rust runner and integrator-package contracts.
- [ ] Record LR08 executable-language dispositions and refuse local workarounds.

## Deliverables

- Candidate-bound prerequisite ledger with release identities, digests and resume decisions

## Notes

- Blocked on tl-syntax#16, engineering-assurance#11, an immutable release
  containing the merged engineering-assurance#34 producer boundary, the
  retention/operator selection, policy/event source, integrator-package release
  and quire-research#64.
- Individual downstream work may resume only when every prerequisite it consumes is admitted; a partial ledger is not a global pass.
