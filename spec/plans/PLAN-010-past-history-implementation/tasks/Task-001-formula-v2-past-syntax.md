---
id: Task-001
title: "Implement formula-v2 and past syntax"
type: Task
status: completed
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/FR-011
    type: references
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
  - target: ix://agent-ix/tl-syntax/TC-054
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-057
    type: verifies
---
# Task-001: Implement formula-v2 and past syntax

## Scope

Deliver `tl-syntax#53`: formula-v2 nodes/documents, closed past identities, profile validation, conversions, resource checks, and typed refusals.

## Subtasks

- [x] Write TC-054/TC-057 red cases for every node, profile, wire, conversion, malformed, depth, and count boundary.
- [x] Add O/H/Y/S/T borrowed and owned nodes under formula-v2 without changing formula-v1.
- [x] Add strict v2 read/write and fail-closed pure-future/pure-past profile validation.
- [x] Add lossless v1 upgrade and guarded v2 down-conversion.
- [x] Run the owning repository gates and close every Rust/code/gap review finding.

## Deliverables

- Public formula-v2 and past-profile Rust API with traced tests.

## Notes

- GitHub owner: `agent-ix/tl-syntax#53`.
