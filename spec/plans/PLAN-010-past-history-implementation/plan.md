---
id: Plan-010
title: "Origin-complete past/history implementation"
type: Plan
status: active
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

### Functional Requirements

- [ ] **FR-011**: Implement bounded O/H/Y/S/T semantics under one closed past operator profile.
- [ ] **FR-012**: Bind evaluation to an exact origin-complete history, anchor, clock, limits, and immutable correction relation.
- [ ] **FR-013**: Preserve versioned formula, parser, evaluator, rewrite, corpus, and native-bridge compatibility.

## Dependency Graph

- `accepted M0 + MRS-002 + MRS-003 -> Task-001`
  Reason: no existing formula/profile identity may be reinterpreted.
- `Task-001 -> Task-002, Task-003, Task-004`
  Reason: parser, evaluator, and rewrite all consume the same formula-v2 graph and closed profile types.
- `Task-002 + Task-003 + Task-004 -> Task-005`
  Reason: the shared corpus must be executable through every owning implementation rather than describe planned behavior.
- `accepted quire-contract-ir FR-025 -> Task-006`
  Reason: typed predicate projection is the total-Boolean input seam for temporal valuations.
- `Task-005 + Task-006 + accepted quire-contract-ir FR-026 -> Task-007`
  Reason: the native bridge needs the complete TL target and predicate projection before it can emit an executable correspondence.

### Shared dependencies

- Formula-v2 node/profile identities and typed refusals are authored once in `tl-syntax` and imported everywhere else.
- Position-history and past-result identities are authored once in `tl-mltl`; the bridge constructs those public types rather than mirroring them.
- Canonical corpus identity is authored once in `tl-syntax`; consumers pin and replay it without alternate expected semantics.

### Cross-cutting constraints

- Existing formula-v1, clean-ascii/v1/v2, and future evaluator/rewrite behavior remains byte- and result-compatible.
- All new executable logic and tests are Rust, resource bounded, panic-free on untrusted input, and local-gated.
- Native Quire remains the only editable formal-clause language; the TL parser is internal interchange only.

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
- [ ] **TC-053**: The tl-mltl history/clock/profile/non-value allocation is complete; native predicate-projection refusal remains with Task-006.

### Cross-repository behavior

- [ ] **TC-056**: Replay corpus, rewrite, correction, target-disposition, and native-bridge scenarios against exact public contracts.

## Remaining Work

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

## Parallel Execution Summary

```text
A1 -> A2 --\
   -> A3 ---+-> A5 --\
   -> A4 --/         +-> C1
B1 -----------------/
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-011, FR-013 | TC-054, TC-057 | completed |
| Task-002 | A | FR-013 | TC-055, TC-057 | completed |
| Task-003 | A | FR-011, FR-012 | TC-048..TC-053, TC-056 | completed |
| Task-004 | A | FR-011, FR-013 | TC-056 | completed |
| Task-005 | A | FR-013 | TC-056, TC-058 | completed |
| Task-006 | B | FR-012, FR-013 | TC-053, TC-056 | blocked on native strict readers and tl-syntax#61 |
| Task-007 | C | FR-012, FR-013 | TC-056 | blocked on Task 006/#70 and owner temporal APIs |

## Coordination Rules

- Merge and record the exact MRS-003 revision before changing the manifest to `authorized` or starting Task-001.
- One repository owns each public type. Cross-repository code imports the owner's contract and never mirrors it, including in tests.
- Preserve existing worktrees and agent branches. Use a fresh issue branch in the owning repository, and merge in dependency order.
- Tests and self-review are implementation gates only; no external qualification campaign or retained local evidence framework is introduced.
