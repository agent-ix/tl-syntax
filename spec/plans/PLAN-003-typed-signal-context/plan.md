---
id: PLAN-003
title: Typed signal and caller context implementation plan
type: Plan
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: references
  - target: ix://agent-ix/tl-syntax/StR-003
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: references
---

# Typed signal and caller context implementation plan

## Objective

Implement `agent-ix/tl-syntax#15` as the parser-independent substrate for the
typed temporal workstream. The crate will validate borrowed and owned signal
catalogs, bind MLTL propositions only to Boolean signals, preserve complete
caller context, and publish two strict new wire documents without changing the
existing formula-v1, proposition-map-v1, or shared corpus bytes.

## Base and landing

The work begins on exact local base
`73a5d69c3095aede9bf5e6f29ce9c22bf27c6d2e`, the current head of tl-syntax PR
#14. The #15 branch shall not be proposed for merge until #14 lands or the
feature branch is rebased onto its landed equivalent. No #15 commit may mutate
the #14 branch or obscure its pending review.

## Dependency DAG

```text
FR-007 + StR-003 + SR-014
  -> borrowed identities, domains, declarations, bindings, and context
  -> catalog/context validation and deterministic typed refusals
  -> alloc-owned values and bounded strict serde documents
  -> borrowed formula binding and lookup
  -> positive/negative fixtures, property tests, feature checks, legacy snapshots
  -> existing local gate and shared assurance intake
  -> code review + gap analysis
  -> exact downstream API handoff to tl-parse#20 and tl-rewrite#21
```

## Task File Mapping

| Task | Scope | Exit evidence |
|---|---|---|
| Task-001 | Specification, matrix, assurance impacts, composite review | Quire grammar-clean specification and SR-014 dispositions |
| Task-002 | Borrowed signal catalog, source context, validation errors | no-default unit/property tests covering every value and refusal boundary |
| Task-003 | Owned values and separate strict wire documents | alloc/serde round trips, bounded decoding, unknown-field/version refusals, checked fixtures |
| Task-004 | Formula binding, compatibility snapshots, full verification | TC-027 through TC-033 backed by native Rust symbols; existing corpus digests unchanged |
| Task-005 | Closing reviews and downstream handoff | resolved code-review/gap-analysis findings and published exact public API contract |

## Implementation shape

- `src/signal.rs` owns no-allocation `SignalId`, `SignalDomain`, borrowed
  declarations/bindings, `SignalCatalog`, `BoundFormula`, lookup, and errors.
  Catalog construction uses caller-owned `u32` scratch for O(n log n) exact
  name uniqueness and does not retain it.
- `src/context.rs` owns borrowed `RequirementContext` validation and errors.
- Alloc-only modules own `String`/`Vec` forms and the
  `tl-syntax.signal-catalog/v1` and `tl-syntax.requirement-context/v1` documents.
- Serde wire intermediates use closed enums, `deny_unknown_fields`, required
  present-context fields, and bounded sequence visitors before semantic
  validation.
- Existing formula and proposition-map structs are not extended. Existing
  corpus files and `corpus/SHA256SUMS` remain byte-for-byte unchanged.

Names are compared as exact UTF-8 bytes. Direct binding searches the sorted
catalog deterministically and visits formula propositions in node order. A
successful `BoundFormula` borrows both validated inputs and owns no duplicate
mapping. Context identity strings are opaque preserved caller values: tl-syntax
checks lengths and completeness but does not import or duplicate contract-IR
identifier semantics.

## Verification method

TC-027 through TC-033 are implemented in Rust unit, property, integration,
snapshot, and feature-boundary tests. Negative tests construct or mutate one
input field at a time and retain an accepted neighboring control, so every
refusal is executable without a new test runner or mutation script. Positive
and negative JSON fixtures live under the repository's test fixtures, not the
versioned shared temporal corpus.

The existing `make` targets remain orchestration only. No new Python helper,
shell gate, evidence collector, tool-identity implementation, traceability
implementation, runner, or Make target is introduced. The existing native
producers and shared Quire/Quoin/Engineering Assurance intake remain the only
assurance path. Hosted CI remains manual-only and is not dispatched by this
plan.

## Exit Criteria

1. Every FR-007 and StR-003 criterion has a named, executing trace symbol.
2. Every catalog, domain, binding, context, size, version, and unknown-field
   refusal has a positive control and typed negative assertion.
3. Default/no-default builds allocate nothing and retain an empty normal
   dependency graph; alloc and serde remain explicit feature boundaries.
4. Signal/context wire output is deterministic for identical validated inputs;
   decode rejects over-limit sequences before accepting a document.
5. Existing formula-v1, proposition-map-v1, and shared-corpus bytes and behavior
   are unchanged, as demonstrated by the existing digest gate and TC-032.
6. The full local gate passes, code review and gap analysis have no unresolved
   blocking findings, and hosted CI was not dispatched.
7. tl-parse#20 and tl-rewrite#21 receive exact type, constructor, lookup, error,
   feature, and wire identities; no sibling copy is required.
