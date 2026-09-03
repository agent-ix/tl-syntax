---
id: Task-002
title: "Allocation-free signal and context core"
type: Task
status: not_started
track: Core
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-003
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-007
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: references
---
# Task-002: Allocation-free signal and context core

## Scope

Implement distinct signal identity, the closed Boolean/Integer/fixed-Decimal
domain vocabulary, borrowed declarations and proposition bindings, bounded
catalog validation, complete borrowed caller context, deterministic lookup, and
specific typed errors without `alloc`.

## Completion Evidence

No-default unit/property tests exercise all domain variants, numeric/scale/name
boundaries, identity ordering, duplicates, missing targets, non-Boolean direct
bindings, catalog limits, complete context, and every malformed context field.
