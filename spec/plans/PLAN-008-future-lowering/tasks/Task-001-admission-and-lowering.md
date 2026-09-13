---
id: Task-001
title: Implement future-operator admission and lowering
type: Task
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-008
    type: part_of
---

# Task-001: Implement future-operator admission and lowering

Add `src/future.rs` with the FR-008 request, report, and refusal contracts, the
closed `W`/`M` catalog, private admission into a typed request, and the total
three-node lowering. Move the formula-v1 node limit into the `no_std` core.

## Progress

Implemented in commit `f36741e`. The module builds under no default features,
`alloc`, `serde`, and all features.
