---
id: Task-012
title: "Break the Contract IR and native-owner dependency cycle"
type: Task
status: in_progress
track: Architecture
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-028
    type: references
---
# Task-012: Break the Contract IR and native-owner dependency cycle

## Scope

Split the existing Contract IR semantic substrate into the cycle-free
`quire-contract-model` package before the bridge imports QSL owner views,
without changing any existing public API, wire identity or QSL Rust import.

## Subtasks

- [x] Convert the Contract IR repository to the reviewed two-package workspace
  and move only the existing substrate into `quire-contract-model`.
- [x] Re-export the complete existing model API from `quire-contract-ir`.
- [x] Point QSL's existing `quire-contract-ir` dependency key at the pinned
  `quire-contract-model` package so its source imports remain unchanged.
- [x] Prove the default/all/minimum-feature production graphs are acyclic and
  the model package reaches no owner or TL crate.
- [x] Replay the full Contract IR and QSL baseline corpora and verify exact
  schema, byte, identity, diagnostic and outcome compatibility.
- [ ] Run code review, Rust review and gap analysis on unchanged heads and fix
  all findings before owner pins advance.

## Deliverables

- Merged cycle-free model package and compatibility bridge package.
- Exact QSL pin and TC-041 dependency/API compatibility evidence.

## Notes

- This is architecture-required implementation, not a semantic rewrite.
- No copied owner type, trust flag, callback or local owner parser is allowed.
- GitHub tracker: `quire-contract-ir#73`.
