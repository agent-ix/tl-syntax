---
id: Task-002
title: "Structured producers and shared intake"
type: Task
status: done
track: Migration
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
  - target: ix://agent-ix/tl-syntax/FR-005
    type: references
---
# Task-002: Structured producers and shared intake

## Scope

Give this repository's domain tools declared structured outputs, and route them
through Quire's static export and Quoin's change-assurance surface without either
tool executing a producer.

## Completion Evidence

Three producers emit structured results. `examples/corpus_conformance.rs` replays
the shared temporal corpus through the real crate and classifies every rejection
by matching the crate's public typed errors rather than reading an error message.
`scripts/validate_corpus.py --json` emits the derived-horizon and closed-trace
oracle stream and states its own limitation, which is that tl-syntax owns no
evaluator. `scripts/check_default_dependencies.py --json` emits the feature
boundary and the empty default dependency graph.

`make assurance-inputs` is the only target that runs a producer.
`scripts/assurance_chain.py` consumes what that target wrote and refuses to
create an absent input, naming the target instead. The adapter transcribes one
named protocol and refuses any other rather than guessing.
