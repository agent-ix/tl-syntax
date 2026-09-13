---
id: Task-002
title: Implement traced lowering tests and matrix bindings
type: Task
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-008
    type: part_of
---

# Task-002: Implement traced lowering tests and matrix bindings

Add `tests/future_lowering.rs` for TC-040, TC-041, TC-042, TC-046, and the
tl-syntax portion of TC-044. Update the exact live-source census and the TM-002
test-case statuses.

## Progress

Ten traced tests pass in commit `f36741e`. The census adds `src/future.rs` and
`tests/future_lowering.rs`. TC-040, TC-041, TC-042, and TC-046 are implemented;
TC-044 and every FR row stay planned until downstream evidence lands.
