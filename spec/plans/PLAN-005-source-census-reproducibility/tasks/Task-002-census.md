---
id: Task-002
title: "Reproducible byte-level census"
type: Task
status: in_progress
track: Verification
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-005
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# Task-002: Reproducible byte-level census

## Scope

Return explicit enumeration and scan errors, honor only repository `.gitignore`
rules for untracked generated paths, search arbitrary bytes, and make both the
area and exact-path diagnostics observable.

## Completion evidence

TC-034 and TC-035 exercise every expected success/refusal boundary without
intercepted panics. Machine-local excludes cannot hide an ordinary source, and
a non-UTF-8 tracked file remains searchable for forbidden identities.
