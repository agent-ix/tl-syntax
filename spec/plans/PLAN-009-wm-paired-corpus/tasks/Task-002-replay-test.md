---
id: Task-002
title: Implement the corpus replay test and mutation controls
type: Task
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-009
    type: part_of
---

# Task-002: Implement the corpus replay test and mutation controls

Add `tests/future_operator_corpus.rs` for TC-074. The replay verifies the pinned
digests, binds spans to the source text, builds derived and direct documents
through the tl-syntax lowering API, and compares them with the shared expected
document. Mutation controls cover each FR-010 dimension. Update the exact
live-source census and the TM-002 status.
