---
id: Task-014
title: "Exercise the real shared source-grounding handoff"
type: Task
status: blocked
track: A
priority: P0
owner_repository: agent-ix/tl-syntax
consumer_repositories: [agent-ix/tl-syntax]
evidence_method: real-shared-contract-integration-test
github_issue: ix://agent-ix/tl-syntax/issues/49
resume_conditions: [ix://agent-ix/tl-syntax/issues/16, ix://agent-ix/tl-syntax/issues/45, ix://agent-ix/tl-syntax/issues/47, ix://agent-ix/tl-syntax/issues/48]
relationships:
  - target: ix://agent-ix/tl-syntax/Task-010
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-012
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-013
    type: depends_on
  - target: ix://agent-ix/tl-syntax/IT-001
    type: references
  - target: ix://agent-ix/tl-syntax/FR-014
    type: references
  - target: ix://agent-ix/tl-syntax/FR-015
    type: references
  - target: ix://agent-ix/tl-syntax/FR-017
    type: references
  - target: ix://agent-ix/tl-syntax/FR-018
    type: references
  - target: ix://agent-ix/tl-syntax/TC-059
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-060
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-062
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-063
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-066
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-067
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-068
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-069
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-070
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-073
    type: verifies
---
# Task-014: Exercise the real shared source-grounding handoff

## Scope

Run IT-001 through the exact released Engineering Assurance, Quire and Quoin
contracts with no mock reader or local substitute.

## Subtasks

- [ ] Write blocked integration fixtures against the released schemas/readers.
- [ ] Exercise valid, missing, stale, malformed, unavailable and conflict paths.
- [ ] Falsify each P0 claim with a load-bearing mutation.
- [ ] Record exact tool, source, configuration and environment identities.

## Deliverables

- Real Rust/shared-contract IT-001 harness and evidence

## Notes

- Blocked on the tl-syntax#16 accepted release set and completion of Tasks
  012–013.
