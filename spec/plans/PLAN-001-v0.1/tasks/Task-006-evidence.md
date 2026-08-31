---
id: Task-006
title: "Exact-candidate evidence"
type: Task
status: not_started
track: Evidence
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-001
    type: part_of
  - target: ix://agent-ix/tl-syntax/MP-001
    type: references
---
# Task-006: Exact-candidate evidence

## Scope

Retain the exact clean revision's local results, tool and dependency identities, PGM-01 checks, and
explicit limitations in a checksummed evidence record.

## Guard

Missing, skipped, failed, or not-yet-sealed validation remains non-conclusive. Collection begins
only after Task-005 is done and the source revision is clean.
