---
id: Task-003
title: "Required roots, population, and retained records"
type: Task
status: done
track: Integrity
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: part_of
---

# Task-003: Required roots, population, and retained records

## Scope

Require every declared root, assert exact total and per-area populations with
self-locating diagnostics, and correct the NFR-002 and SR-013 statements.

## Completion evidence

A missing root and a compensating cross-area change have distinct reactors;
the retained prose matches the live read sites and retired identifiers.

TC-026 checks all seven source directories and seven required root files,
asserts the exact 42-file tracked population and eight per-area cardinalities,
and reports both expected and observed maps. NFR-002 and SR-013 carry the
corrected retained statements.
