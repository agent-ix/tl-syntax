---
id: Task-007
title: "Human source-release decision"
type: Task
status: not_started
track: Gate
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-001
    type: part_of
  - target: ix://agent-ix/tl-syntax/AA-001
    type: references
---
# Task-007: Human source-release decision

## Scope

Review the exact candidate, conformance evidence, residual limitations, and
downstream impact before recording the v0.1 source-release decision.
`make spec-release` must pass for that candidate; `make spec` is the authoring
gate and deliberately reports accepted but not-yet-implemented roadmap rows
without treating them as release evidence.

## Guard

This task is human-owned. No agent or automated gate may mark it done.
