---
id: Task-001
title: "Numeric expression typing and predicate table (TL-420)"
type: Task
status: todo
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/FR-302
    type: references
  - target: ix://agent-ix/tl-syntax/FR-303
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-018
    type: references
  - target: ix://agent-ix/tl-syntax/TC-1500
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1502
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1504
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1505
    type: verifies
---
# Task-001: Numeric expression typing and predicate table (TL-420)

## Scope

FR-302 and FR-303: the closed expression model, derived types, the `tl-syntax.predicate-table/v1` document, schema and strict reader.

## Subtasks

- [ ] Expression arena, ExprType, ExpressionError (no_std, caller scratch).
- [ ] PredicateTableDocument, schema, digest constants, bounded reader.
- [ ] Traced tests TC-1500..TC-1507.

## Deliverables

Typed model, document, tests.

## Notes

Specification-only session: no code was written or run. Layer 1 of P-TL-150.
