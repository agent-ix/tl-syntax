---
id: FR-010
title: "Keep derived future semantics canonical downstream"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-009
    type: depends_on
---

# FR-010: Keep derived future semantics canonical downstream

## Description

Where a downstream temporal component consumes derived future input, it shall
operate only on the FR-008 canonical graph and shall demonstrate equivalence to
direct construction without acquiring an independent derived semantics.

## Inputs

- Paired derived inputs and directly constructed canonical graphs.
- Both existing semantic profiles, valid and boundary intervals, finite closed
  traces, open prefixes, and declared component resource limits.
- Target mapping profiles and their explicit supported, unsupported, or
  unavailable states.

## Outputs

- Evaluation, prefix-progress, horizon, rewrite, corpus, and mapping results
  attributable to the same canonical graph and selected semantic profile.
- Explicit target loss/refusal states where correspondence is not established.
- Routed implementation tickets with one owner and dependency per concern.

## Behavior

Evaluation, prefix progress, horizon analysis, rewriting, corpus replay, and
export run only after lowering. Their result for derived input shall equal the
result for the byte-identical directly constructed graph across W/M, both
semantic profiles, empty and short traces, `[0,0]`,
`[u32::MAX,u32::MAX]`, nested lowerings, pending prefixes, count arithmetic,
and component work/resource limits.

The shared corpus retains paired derived-source and canonical-document cases.
Mutation controls change each lowering branch, generated-node order, inclusive
endpoint, associativity, selected profile, node charge, and token/expression
span attribution; each mutation must change the expected graph/report or turn
its owning gate red.

Interop adapters emit only from the canonical graph and report unsupported or
unavailable when a target cannot preserve the selected profile. External parser
acceptance is not equivalence evidence. R2U2 and C2PO remain monitor targets,
not dependencies or alternate evaluators. FRETish remains output-only. No Java,
Node, Electron, or other foreign runtime enters a production or qualification
path.

Native Quire remains the only editable formal-clause authority. Its
quire-contract-ir bridge may produce the canonical graph only when that
bridge's separately reviewed correspondence profile admits the native clause;
it does not consume the TL text dialect. TL text and formula-v1 documents remain
derived interchange/evaluator inputs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-010-AC-1 | For every declared boundary, evaluation, prefix progress, horizon, and resource outcomes for a lowered graph equal direct canonical construction under both existing semantic profiles. | Test (TC-044) |
| FR-010-AC-2 | Rewrite and paired-corpus paths contain no derived semantic branch and preserve canonical graph, semantic-profile, fixture, and unsupported-state identities. | Test (TC-045) |
| FR-010-AC-3 | Native bridge and external-target mappings consume only canonical graphs, preserve supported/unsupported/unavailable results, keep FRETish output-only, and make no external-parser, monitor, qualification, or source-authority claim. | Test (TC-045) |
| FR-010-AC-4 | Mutating every enumerated lowering, ordering, interval, associativity, profile, resource, or span dimension changes the expected graph/report or makes its owning control fail. | Test (TC-041, TC-042, TC-043, TC-044, TC-045, TC-046) |

## Dependencies

Implementation starts only after M0 stabilization and acceptance of MRS-002,
FR-008, FR-009, this requirement, ADR-001, and TM-002. Dependency order is:

1. `tl-syntax` owns the no-alloc lowering values, closed catalog, report, typed
   refusals, and exact node-budget accounting.
2. `tl-parse` owns the successor internal dialect, precedence, spans,
   diagnostics, fuzz target, and primitive-only canonical formatter.
3. `tl-mltl` and `tl-rewrite` independently add direct-versus-lowered
   evaluation/progress/horizon/resource and rewrite-equivalence controls; they
   add no operator branch.
4. The shared corpus adds paired source/document fixtures after syntax and
   parser revisions exist.
5. Interoperability work adds lowered-graph export and target-profile
   loss/refusal cases after the evaluator and corpus revisions exist.

quire-contract-ir's native bridge consumes a reviewed canonical profile but is
blocked independently on its total-Boolean predicate projection and native
temporal source profile. It is not a prerequisite for the internal lowering.
