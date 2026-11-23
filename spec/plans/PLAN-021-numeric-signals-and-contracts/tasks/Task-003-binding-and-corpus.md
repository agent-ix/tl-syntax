---
id: Task-003
title: "Predicate binding and numeric corpus (TL-420)"
type: Task
status: todo
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/FR-304
    type: references
  - target: ix://agent-ix/tl-syntax/FR-305
    type: references
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/TC-1508
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1511
    type: verifies
---
# Task-003: Predicate binding and numeric corpus (TL-420)

## Scope

FR-304 binding view and FR-305 corpus `tl-syntax.numeric-corpus/v1` with independently computed truth tables and retained provenance script.

## Subtasks

- [ ] BoundPredicateFormula.
- [ ] Corpus files, manifest, provenance.
- [ ] TC-1508..TC-1513, TC-1530..TC-1532.

## Deliverables

Binding view, corpus consumed by tl-parse, tl-mltl, tl-solve.

## Notes

Specification-only session: no code was written or run. Layer 1 of P-TL-150.
