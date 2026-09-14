---
id: Task-011
title: "Close the integrated ecosystem and model export"
type: Task
status: completed
track: Integration
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-006
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-007
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-009
    type: depends_on
  - target: ix://agent-ix/tl-syntax/Task-010
    type: depends_on
  - target: ix://agent-ix/tl-syntax/TC-053
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-056
    type: verifies
---
# Task-011: Close the integrated ecosystem and model export

## Scope

Execute the complete owner-reader-to-bridge-to-TL path, publish the
machine-readable ecosystem model needed for later self-analysis, and close
epic #52 only when every planned feature and architecture repair is merged.

## Subtasks

- [x] Pin every owner/consumer edge to immutable reviewed revisions and reject
  any undeclared edge or copied schema.
- [x] Execute the complete future and origin-complete past corpus through
  source-bound predicate projection, formula/history/request construction,
  TL evaluation, protocol result production, and native/TL result joining.
- [x] Prove every unsupported, incomplete, unavailable, failed, refused,
  conflict, correction, and resource state remains distinct end to end.
- [x] From `quire-contract-ir::ecosystem_model`, export the crate/component,
  interface, object, identity, dependency, and evidence graph in a bounded
  machine-readable model suitable for later analysis without granting that
  model authority over itself.
- [x] Run unchanged-head code, Rust, architecture, and gap reviews; fix all
  findings; update every ticket, task, matrix, and epic status.
- [x] Remove only redundant clean agent-b worktrees/targets after all useful
  work is merged or otherwise preserved; never touch agent-e worktrees.

## Deliverables

- End-to-end integrated Rust ecosystem and complete traced campaign corpus.
- Machine-readable ecosystem model with an explicit non-authoritative
  self-analysis boundary.
- Closed child tickets and epic #52 with exact merge/review evidence.

## Notes

- This is implementation integration and model export, not an external
  qualification campaign or self-certification claim.
- Model-export tracker: `quire-contract-ir#74`.
- Contract IR PR #79 merged the complete PLAN-007/FR-027 implementation. Its
  promoted implementation revision is
  `0c450731626f40fd90c99e787cc0f7f5e053904c`; PR #80 reconciled the exact
  self-selection after the required rebase merge, leaving final reviewed main
  at `4d139309bc86b3d698cc73404356900e904d43be`.
- `ecosystem_model::{manifest::read, export, read}` strict-read the exact
  nine-repository campaign and export/re-read the bounded descriptive model.
  The immutable manifest and model schema SHA-256 digests are respectively
  `e9fb2e4f1f6657e3e1015304bd602de670999ed91b6879ab30dea30303d757ce`
  and `c41e60466661fe8197410efd71229e8081d799f7078e72d7f7292021502e471d`.
- QCI TC-038, TC-039, and TC-040 execute the predicate, future/past temporal,
  correction/non-success, and exact model paths against public owner APIs.
  The closing workspace run passed 82 Rust tests and two compile-fail doctests;
  SR-538, SR-539, and SR-540 pass with every finding fixed.
- All implementation child tickets are closed. `quire-protocol#8` remains a
  truthful downstream Plan-001 gap for the broader assessment/refusal engine;
  the exact QProtocol result/mapping owner allocation consumed by this campaign
  was completed in `quire-protocol#51` and therefore does not block Task-011.
- Eleven redundant agent-b worktrees (about 5.7 GiB, primarily derived build
  targets) were removed after their tracked state was verified clean. All
  branch refs were retained, and both agent-e TL worktrees remain untouched.
