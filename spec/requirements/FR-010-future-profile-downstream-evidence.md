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

The paired corpus is `tl-syntax.future-operator-corpus/v1` in
`corpus/future-operators/`. Each derived-source case binds its dialect,
operator profile, semantic profile, source text, and ordered append/lower steps
to a span-free expected formula-v1 document; a directly constructed case shares
the same document. A primitive-source case binds `tl-parse.clean-ascii/v1` text
to the compatibility graph the same way. The replay binds spans to the source
bytes and operator spellings but does not parse; grammar stays with TC-043. Refused and malformed cases declare their typed refusal or
replay error and produce no document. File digests pin the corpus identity.
The corpus is evidence input replayed through the tl-syntax lowering API: it is
neither an evaluator nor an editable source language, and it defines no derived
formula-v1 node.

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
| FR-010-AC-2 | Rewrite and paired-corpus paths contain no derived semantic branch and preserve canonical graph, semantic-profile, fixture, and unsupported-state identities. | Test (TC-045, TC-074) |
| FR-010-AC-3 | Native bridge and external-target mappings consume only canonical graphs, preserve supported/unsupported/unavailable results, keep FRETish output-only, and make no external-parser, monitor, qualification, or source-authority claim. | Test (TC-045) |
| FR-010-AC-4 | Mutating every enumerated lowering, ordering, interval, associativity, profile, resource, or span dimension changes the expected graph/report or makes its owning control fail. | Test (TC-041, TC-042, TC-043, TC-044, TC-045, TC-046, TC-074) |

## Dependencies

Implementation starts only after M0 stabilization and acceptance of MRS-002,
FR-008, FR-009, this requirement, ADR-001, and TM-002. Dependency order is:

1. `tl-syntax` issue [#40](https://github.com/agent-ix/tl-syntax/issues/40)
   owns the no-alloc admission/lowering values, closed catalog, report, typed
   refusals, and exact node-budget accounting.
2. `tl-parse` issue [#31](https://github.com/agent-ix/tl-parse/issues/31)
   owns `tl-parse.clean-ascii/v2`, precedence, spans,
   diagnostics, fuzz target, and primitive-only canonical formatter.
3. `tl-mltl` issue [#47](https://github.com/agent-ix/tl-mltl/issues/47)
   and `tl-rewrite` issue
   [#35](https://github.com/agent-ix/tl-rewrite/issues/35) independently add
   direct-versus-lowered evaluation/progress/horizon/resource and
   rewrite-equivalence controls; they add no operator branch.
4. `tl-syntax` issue [#41](https://github.com/agent-ix/tl-syntax/issues/41)
   adds paired source/document fixtures. It builds against the #40 and #31
   revisions and consumes no tl-mltl or tl-rewrite output, so it may be
   implemented in parallel with #47 and #35 but lands after them.
5. `tl-mltl` issue [#48](https://github.com/agent-ix/tl-mltl/issues/48)
   adds lowered-graph export and target-profile loss/refusal cases after the
   evaluator and corpus revisions exist.

Each issue names M0 closure and the preceding issue identities as hard
predecessors. These links route work; they do not authorize implementation
before the acceptance gates above are satisfied.

quire-contract-ir's native bridge consumes a reviewed canonical profile but is
blocked independently on its total-Boolean predicate projection and native
temporal source profile. It is not a prerequisite for the internal lowering.
