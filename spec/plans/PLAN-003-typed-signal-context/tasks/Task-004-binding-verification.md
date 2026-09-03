---
id: Task-004
title: "Formula binding and verification"
type: Task
status: not_started
track: Verification
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-003
    type: part_of
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: references
---
# Task-004: Formula binding and verification

## Scope

Return an allocation-free bound-formula view after deterministic node-order
resolution, expose typed lookup for downstream consumers, back TC-027 through
TC-033, and execute the existing formatting, feature, lint, test, corpus,
conformance, supply-chain, specification, MSRV, rustdoc, and assurance gates.

## Completion Evidence

Every new matrix row has a native trace symbol, every refusal has an accepted
neighboring control, `corpus/SHA256SUMS` still passes unchanged, strict coverage
has no new unbacked implementation row, and the existing full local gate passes
without dispatching hosted CI.
