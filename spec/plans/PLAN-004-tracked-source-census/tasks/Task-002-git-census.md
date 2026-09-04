---
id: Task-002
title: "Git-derived source census"
type: Task
status: done
track: Verification
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# Task-002: Git-derived source census

## Scope

Replace recursive filesystem population discovery with fail-closed Git tracked
and non-ignored-untracked enumeration. Add scratch controls for tracked,
untracked, ignored, and unable-to-enumerate states.

## Completion evidence

TC-026 proves the tracked population and broader scan sets are distinct and
that ignored generated files cannot perturb either one.

The scratch repository admits one tracked source and one non-ignored untracked
source into the declared sets while excluding a proptest regression seed. A
real non-repository directory exercises the unable-to-enumerate refusal.
