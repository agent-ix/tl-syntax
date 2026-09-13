---
id: Task-002
title: "Implement clean-ascii/v3"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/Task-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
  - target: ix://agent-ix/tl-syntax/TC-055
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-057
    type: verifies
---
# Task-002: Implement clean-ascii/v3

## Scope

Deliver `tl-parse#35`: internal O/H/Y/S/T parsing, canonical formatting, exact spans, bounds, and diagnostics.

## Subtasks

- [ ] Write TC-055/TC-057 red cases against the public formula-v2 API.
- [ ] Implement v3 tokenization, precedence, associativity, validation, and formatting.
- [ ] Prove v1/v2 refusal and no-unwind bounded malformed input behavior.
- [ ] Run the owning repository gates and close every Rust/code/gap review finding.

## Deliverables

- `tl-parse.clean-ascii/v3` public Rust entry point and traced tests.

## Notes

- GitHub owner: `agent-ix/tl-parse#35`.
