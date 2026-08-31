---
id: Task-004
title: "Wire documents and conformance corpus"
type: Task
status: done
track: Interchange
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-001
    type: part_of
---
# Task-004: Wire documents and conformance corpus

## Scope

Implement invariant-preserving owned documents, strict versioned serde formats, and the shared
conformance corpus with reviewed outcomes and rejection reasons.

## Completion Evidence

Round-trip and malformed-input tests pass, JSON Schemas reject unknown fields and wrong arity, and
the corpus checker recomputes every reviewed outcome and rejection class.
