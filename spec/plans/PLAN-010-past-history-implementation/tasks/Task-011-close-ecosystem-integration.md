---
id: Task-011
title: "Close the integrated ecosystem and model export"
type: Task
status: not_started
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

- [ ] Pin every owner/consumer edge to immutable reviewed revisions and reject
  any undeclared edge or copied schema.
- [ ] Execute the complete future and origin-complete past corpus through
  source-bound predicate projection, formula/history/request construction,
  TL evaluation, protocol result production, and native/TL result joining.
- [ ] Prove every unsupported, incomplete, unavailable, failed, refused,
  conflict, correction, and resource state remains distinct end to end.
- [ ] From `quire-contract-ir::ecosystem_model`, export the crate/component,
  interface, object, identity, dependency, and evidence graph in a bounded
  machine-readable model suitable for later analysis without granting that
  model authority over itself.
- [ ] Run unchanged-head code, Rust, architecture, and gap reviews; fix all
  findings; update every ticket, task, matrix, and epic status.
- [ ] Remove only redundant clean agent-b worktrees/targets after all useful
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
- All declared implementation predecessors are merged; this task is available
  for the architecture/spec owner to schedule, but it is not started or
  discharged by Task-007's completion.
