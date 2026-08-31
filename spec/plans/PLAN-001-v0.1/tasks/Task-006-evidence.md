---
id: Task-006
title: "Exact-candidate evidence"
type: Task
status: done
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

## Completion Evidence

The retained `269d352d1050` record has a passing collection summary, two passing sealed PGM-01
validations, and a checksum manifest that verifies every artifact. Its envelope remains
non-self-attesting and explicitly inconclusive; the post-seal summary records the external result.
