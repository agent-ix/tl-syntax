---
id: Plan-010
title: "Origin-complete past/history implementation"
type: Plan
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/FR-011
    type: references
  - target: ix://agent-ix/tl-syntax/FR-012
    type: references
  - target: ix://agent-ix/tl-syntax/FR-013
    type: references
---
# Implementation Plan: Origin-complete past/history TL track

## Requirements Summary

### Ecosystem architecture gate

- [x] **Task-008 / tl-syntax#64**: Specify the entire epic as one bounded
  temporal-assurance ecosystem, formalize its objects and interfaces, review
  the current code/spec architecture, and fix every composite review finding
  before additional feature implementation.
- [x] **Task-009**: Reconcile already-landed TL core code with that reviewed
  architecture while preserving public compatibility.
- [x] **Task-010**: Publish the complete Quire owner contract set consumed by
  FR-025 and FR-026, including the shared `quire-specification` rulings those
  executable owners must implement.
- [x] **Task-012**: Split the existing Contract IR substrate into a cycle-free
  model package with compatibility re-exports before the bridge imports QSL.
- [x] **Task-011**: Close end-to-end integration and publish a non-authoritative
  machine-readable ecosystem model for later self-analysis.

### Functional Requirements

- [x] **FR-011**: Implement bounded O/H/Y/S/T semantics under one closed past operator profile.
- [x] **FR-012**: Bind evaluation to an exact origin-complete history, anchor, clock, limits, and immutable correction relation.
- [x] **FR-013**: Preserve versioned formula, parser, evaluator, rewrite, corpus, and native-bridge compatibility.

## Dependency Graph

- `accepted M0 + MRS-002 + MRS-003 -> Task-001`
  Reason: no existing formula/profile identity may be reinterpreted.
- `Task-001 -> Task-002, Task-003, Task-004`
  Reason: parser, evaluator, and rewrite all consume the same formula-v2 graph and closed profile types.
- `Task-002 + Task-003 + Task-004 -> Task-005`
  Reason: the shared corpus must be executable through every owning implementation rather than describe planned behavior.
- `Task-001..Task-005 + accepted FR-025/FR-026 -> Task-008`
  Reason: the whole-system architecture must account for every delivered core
  behavior and both accepted bridge contracts before it may revise structure or
  owner boundaries.
- `Task-008 -> Task-009, Task-012`
  Reason: core reconciliation and owner-contract implementation consume one
  reviewed object/interface model and may then proceed by repository in
  dependency order.
- `Task-012 -> Task-010`
  Reason: QSL must consume the cycle-free model package before Contract IR can
  import QSL's constructor-private owner views without a Cargo cycle.
- `Task-009 + Task-010 + accepted quire-contract-ir FR-025 -> Task-006`
  Reason: typed predicate projection requires both the reconciled TL public API
  and all authority-owned native/result/availability readers.
- `Task-005 + Task-006 + Task-009 + Task-010 + accepted quire-contract-ir FR-026 -> Task-007`
  Reason: the native temporal bridge needs the complete TL target, predicate
  projection, owner result/progress contracts, and reviewed subsystem layout.
- `Task-006 + Task-007 + Task-009 + Task-010 -> Task-011`
  Reason: end-to-end execution and model export are meaningful only after every
  owner and bridge implementation is merged.

### Shared dependencies

- Formula-v2 node/profile identities and typed refusals are authored once in `tl-syntax` and imported everywhere else.
- Position-history and past-result identities are authored once in `tl-mltl`; the bridge constructs those public types rather than mirroring them.
- Canonical corpus identity is authored once in `tl-syntax`; consumers pin and replay it without alternate expected semantics.
- Native checked predicates/subjects and canonical formula-wide native
  evaluation requests/results are authored once in `quire-spec-language`;
  observations, position/progress/completeness and
  availability assertions once in `quire-observation`; canonical native
  protocol results once in `quire-protocol`; Contract IR owns only mappings,
  derived artifacts, and joins.
- Cross-repository semantic objects and vocabularies are authored once in
  `quire-specification`. This is a normative reference edge, not a Cargo or
  wire-type dependency; executable schemas and validated types remain with
  their unique runtime owners.
- Native and TL results retain their owner vocabularies. QSL publishes the
  formula-wide native result, TL-MLTL publishes the TL result mapping, and
  Contract IR compares those views without parsing owner bytes or normalizing
  labels by display text. QProtocol's separate predicate-result mapping supplies
  FR-025 leaf valuations and cannot substitute for a formula result.
- The ecosystem model describes these owner edges and may drive later analysis,
  but cannot authorize its own contract revision or certify its own output.

### Cross-cutting constraints

- Existing formula-v1, clean-ascii/v1/v2, and future evaluator/rewrite behavior remains byte- and result-compatible.
- All new executable logic and tests are Rust, resource bounded, panic-free on untrusted input, and local-gated.
- Native Quire remains the only editable formal-clause language; the TL parser is internal interchange only.
- Public compatibility is defined by exact contract/profile/schema identities,
  not Rust module paths. Architecture-driven source reorganization must retain
  public re-exports or introduce an explicitly versioned successor contract.
- All semantic inputs cross repository boundaries as exact bytes plus an
  immutable selection and a public strict owner reader. A Boolean trust flag,
  callback, private wire import, copied schema, or display-name match is never
  authority.

## Test Plan

### Syntax and wire

- [x] **TC-054**: Round-trip formula-v2; enforce profile compatibility, v1 preservation, and guarded conversions.
- [x] **TC-057**: Exercise bounded arbitrary formula-v2 decode without unwind or profile misattribution.
- [x] **TC-058**: Reject every dependency-manifest owner, edge, prerequisite, revision, or authorization mutation.

### Parser

- [x] **TC-055**: Parse and format O/H/Y/S/T in clean-ascii/v3 with exact precedence, associativity, intervals, and spans.

### History and evaluation

- [x] **TC-048**: Compare O/H against an independent reverse-offset oracle.
- [x] **TC-049**: Prove S uses witness `[a,b]` and left range `[a,j)`.
- [x] **TC-050**: Prove T duality and strong Previous equality with O[1,1].
- [x] **TC-051**: Validate histories, clocks, anchors, digests, ordering, and immutable result attribution.
- [x] **TC-052**: Compare evaluation and checked required-history analysis at semantic/resource boundaries.
- [x] **TC-053**: The tl-mltl history/clock/profile/non-value allocation and the native predicate-projection non-value path are complete; Contract-IR TC-038 supplies the traced native allocation.

### Cross-repository behavior

- [x] **TC-056**: Replay corpus, rewrite, correction, target-disposition, and native-bridge scenarios against exact public contracts.

## Completion Record

### Architecture and formal-model gate

- **D1 = Task-008 (completed)** whole-ecosystem architecture — Hard; exit: one reviewed
  bounded context, formal object/interface model, dependency DAG, state model,
  repository/module topology, and complete owner contract set with all ten
  review passes resolved.

### Specification correction inventory

- **S1 — Shared authority (completed)**: close the `quire-specification` gaps that block
  temporal execution: input-completeness membership, admitted-observation
  identity, inclusive-native/half-open-observation boundary conversion,
  activation under missing/refused trigger evidence, and claim-kind/admitted-
  fragment/backend identities. Reuse the accepted FR-240/242/243/284/285/286
  rulings without local alternatives.
- **S2 — Contract IR (completed)**: amend FR-025/FR-026 together so every owner input names
  an exact public reader and mapping contract, the four progress/closure axes
  use `open`/`closed`, completeness remains independent, and native
  `satisfied`/`violated` versus TL Boolean output is normalized only by selected
  owner mappings.
- **S3 — Executable owners (completed)**: author QSL checked-leaf/temporal-subject, QObs
  position/clock/capture/progress/closure/completeness/availability, QProtocol
  result/lineage/mapping, and TL formula/history/trace/request/report/mapping
  interfaces as one cross-referenced design set.
- **S4 — Structure (completed)**: specify compatibility-preserving module boundaries and
  public re-exports for each implementation that currently mixes contract,
  canonicalization, domain state and decision logic.
- **S5 — One review gate (completed)**: after S1–S4 are all authored, run the base review,
  all seven lenses, object review and architecture evaluation over the complete
  nine-repository design; fix every finding before any implementation wave.

### Core reconciliation

- **E1 = Task-009 (completed)** TL core reconciliation — Hard; exit: every architecture
  finding against Tasks 001–005 is fixed without changing an accepted wire or
  semantic identity, and all four crates replay the complete corpus.

### Quire owner contracts

- **F1 = Task-010 (completed)** native/result/observation owners — Hard; exit: every
  FR-025/FR-026 input is a constructor-private validated owner value with a
  canonical schema, strict reader, immutable revision/digest, and typed limits.

### Track A: TL critical path

- **A1 = Task-001** Formula-v2 and past syntax — Hard; exit: strict public graph/wire/profile types admit every pure past graph and preserve every v1 behavior.
- **A2 = Task-002** clean-ascii/v3 — Hard; exit: every O/H/Y/S/T form round-trips with exact spans and older dialects refuse it.
- **A3 = Task-003** origin-complete evaluation — Hard; exit: histories and anchored results implement every truth, boundary, identity, correction, and resource rule.
- **A4 = Task-004** past-profile rewrites — Medium; exit: only reviewed equivalences rewrite and every unproved/profile-mismatched request refuses.
- **A5 = Task-005** shared corpus and manifest — Hard; exit: all four TL owners replay one immutable corpus and the dependency gate authorizes only the exact delivered DAG.

### Track B: Native predicate foundation

- **B1 = Task-006** predicate projection — Hard; exit: checked native Boolean predicates become exact TL valuations and every non-value remains typed without coercion.

### Track C: Complete native bridge

- **C1 = Task-007** temporal correspondence and result join — Hard; exit: every supported future/past native subject reaches the matching TL request/result contract and every unsupported/incomplete/refused axis remains distinct.

### Ecosystem closure

- **G1 = Task-011 (completed)** integrated closure and model export — Hard; exit: the exact
  owner-reader path executes end to end, every non-success state remains typed,
  the machine-readable ecosystem graph is bounded and non-authoritative, and
  every ticket/matrix/epic status matches merged evidence.

## Parallel Execution Summary

```text
A1 -> A2 --\
   -> A3 ---+-> A5 --\
   -> A4 --/          \
                        D1 -> E1 --\
accepted FR-025/026 ---/     F1 ----+-> B1 -> C1 -> G1
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-011, FR-013 | TC-054, TC-057 | completed |
| Task-002 | A | FR-013 | TC-055, TC-057 | completed |
| Task-003 | A | FR-011, FR-012 | TC-048..TC-053, TC-056 | completed |
| Task-004 | A | FR-011, FR-013 | TC-056 | completed |
| Task-005 | A | FR-013 | TC-056, TC-058 | completed |
| Task-006 | B | FR-012, FR-013, Contract-IR FR-025 | TC-053, TC-056 | completed at Contract-IR `202210cf` |
| Task-007 | C | FR-012, FR-013, Contract-IR FR-026 | TC-056 | completed at Contract-IR `69ec82bf` (#78); TC-039 and PLAN-006 complete |
| Task-008 | Architecture | MRS-003, FR-011..FR-013, Contract-IR FR-025..FR-026 | composite spec/object/architecture reviews | completed |
| Task-009 | Core | Task-001..Task-005 accepted behavior | TC-048..TC-058 | completed; all four TL allocations merged |
| Task-010 | Owners | Contract-IR FR-025..FR-026 owner inputs | owner contract tests | completed; final QSL owner merged at `c2915338` |
| Task-011 | Integration | complete epic #52 ecosystem | TC-053, TC-056 and cross-owner integration | completed through QCI #79/#80 at `4d139309`; SR-538..SR-540 and SR-107..SR-109 pass |
| Task-012 | Architecture | Contract-IR FR-028 | TC-041 | completed at `53cc03c` |

## Coordination Rules

- Merge and record the exact MRS-003 revision before changing the manifest to `authorized` or starting Task-001.
- One repository owns each public type. Cross-repository code imports the owner's contract and never mirrors it, including in tests.
- Preserve existing worktrees and agent branches. Use a fresh issue branch in the owning repository, and merge in dependency order.
- Tests and self-review are implementation gates only; no external qualification campaign or retained local evidence framework is introduced.
- Specification is authored for the full ecosystem before repository
  implementation begins. Repository work may be sequenced for dependency and
  merge safety, but is not separately re-scoped or deferred.
- Existing useful work remains preserved and credited. Task-009 changes it only
  where the reviewed architecture demonstrates a boundary, ownership, or
  maintainability defect.
