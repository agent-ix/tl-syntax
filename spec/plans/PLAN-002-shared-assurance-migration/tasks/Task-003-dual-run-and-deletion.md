---
id: Task-003
title: "Dual run and deletion"
type: Task
status: done
track: Migration
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: part_of
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: references
---
# Task-003: Dual run and deletion

## Scope

Run the old and new paths against the same candidate revision, record what
happened, and only then delete the generic machinery.

## Completion Evidence

The dual-run table is in the plan overview. The old path was already failing at
the candidate revision and that is recorded as observed rather than presented as
parity.

Deletion is a separate commit, taken after the dual run, and removes only the
generic executor, verifier, retention, identity, traceability, and
failure-propagation machinery. Every byte under `evidence/` is unchanged, and
both evidence schemas are frozen rather than deleted because retained envelopes
name them by path and SHA-256. `tests/no_local_evidence_framework.rs` asserts
both facts, including that no file in the repository references either frozen
schema.
