---
id: Plan-007
title: "tl-syntax — progressive source-qualification readiness"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: references
  - target: ix://agent-ix/tl-syntax/FR-014
    type: references
  - target: ix://agent-ix/tl-syntax/FR-015
    type: references
  - target: ix://agent-ix/tl-syntax/FR-016
    type: references
  - target: ix://agent-ix/tl-syntax/FR-017
    type: references
  - target: ix://agent-ix/tl-syntax/FR-018
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: references
  - target: ix://agent-ix/tl-syntax/IT-001
    type: references
  - target: ix://agent-ix/tl-syntax/IT-002
    type: references
---
# Implementation Plan: Progressive source-qualification readiness

This TDD plan advances M6 only through released shared contracts and attributed
human decisions. It does not qualify native Quire, a monitor, or an integrating
system, and it does not turn tl-syntax into a user-authored language.

## Requirements Summary

### Stakeholder Requirements

- [ ] **StR-004**: Supply attributable, progressively reviewable source-readiness facts without claiming downstream qualification.

### Functional Requirements

- [ ] **FR-014**: Bind every fact to one immutable candidate/configuration and fresh producer execution.
- [ ] **FR-015**: Preserve lifecycle stages, authority, retention and supersession without promotion.
- [ ] **FR-016**: Emit a lossless integrator package only through the accepted shared contract.
- [ ] **FR-017**: Admit an exact, policy-authorized human source-release disposition.
- [ ] **FR-018**: Classify every executable readiness path and remove stable reliance on legacy paths.

### Non-Functional and Integration Requirements

- [ ] **NFR-004**: Reproduce deterministic observations and preserve every volatile field.
- [ ] **NFR-005**: Preserve authority, limitation and retention truth.
- [ ] **IT-001**: Exercise the real source-grounding handoff.
- [ ] **IT-002**: Exercise the real integrator-package round trip.

## Dependency Graph

- `M6 specification + review -> every implementation task`
  Reason: the specification snapshot and SR-058 through SR-065 establish the
  boundary and test vocabulary before code is authorized.
- `external prerequisite admission -> FR-014, FR-015, FR-016, FR-017, FR-018`
  Reason: source export/matrix, retention, decision policy, package format and
  LR08 ownership are shared enablement and cannot be reconstructed locally.
- `FR-018 -> FR-014`
  Reason: the complete executable-path census and Rust/shared disposition must
  be known before a stable source subject can be declared complete.
- `FR-014 -> FR-015 -> FR-017`
  Reason: lifecycle facts need an exact candidate, and a decision needs exact
  staged facts, reviews and limitations.
- `FR-014 + FR-015 + FR-017 -> IT-001`
  Reason: the real shared handoff consumes candidate, lifecycle and authority
  records together.
- `FR-014 + FR-015 + FR-017 + IT-001 -> FR-016 -> IT-002`
  Reason: package publication follows verified source facts and human decision;
  its real reader is the only admitted integration oracle.
- `NFR-004 + NFR-005 -> every implementation and integration task`
  Reason: reproducibility, non-promotion, retention and authority are
  cross-cutting correctness properties.

The principal seams are the existing Rust shared-assurance intake in
`tests/shared_assurance.rs`, future Rust source-readiness modules, the released
Quire/Engineering Assurance export, Quoin retention handles, and the future
Engineering Assurance integrator package. Markdown parsing and local contract
copies are outside every seam.

## Test Plan

### Candidate and Execution Properties

- [ ] **TC-059**: Reproduce exact candidate/configuration identity.
- [ ] **TC-060**: Reject one-axis candidate, source, configuration and contract mutations.
- [ ] **TC-064**: Census and classify every executable entry point exactly once.
- [ ] **TC-065**: Require positive and forced-failure parity before legacy removal.
- [ ] **TC-068**: Exercise every producer/package execution outcome with stale output present.
- [ ] **TC-069**: Reject source-root, path, mount and symlink boundary failures.

### Lifecycle and Authority Properties

- [ ] **TC-062**: Preserve four stages and all six decision dispositions.
- [ ] **TC-063**: Prove omissions and non-success states cannot improve readiness.
- [ ] **TC-066**: Prove absent shared capability stays unavailable with no local substitute.
- [ ] **TC-067**: Exercise every retention state and retrieval failure.
- [ ] **TC-070**: Exercise bounded supersession and decision transition topology.
- [ ] **TC-073**: Mutate every review/decision policy and identity binding.

### Real Shared Integrations

- [ ] **TC-061**: Round-trip source facts and distinct adopter subjects through the real package.
- [ ] **TC-071**: Exercise idempotent retries and racing package writers.
- [ ] **TC-072**: Complete independent, candidate-bound license/reuse-right analysis.

## Remaining Work

### Track S: Specification gate

- **S1 = Task-001** M6 specification and composite review — Done; exit: the exact snapshot is strict-valid with no review-owned finding.

### Track G: External admission gate

- **G1 = Task-002** prerequisite ledger and no-workaround gate — Blocked; exit: each consumed release, policy, authority and backend has an immutable compatible identity.

### Track A: Critical path (serial)

- **A1 = Task-003** executable-path inventory and Rust/shared parity — Hard; exit: every entry point has one reviewed class and every stable-required legacy behavior has parity.
- **A2 = Task-004** candidate binding and producer freshness — Hard; exit: identity races, stale output and path escapes fail closed.
- **A3 = Task-005** lifecycle, retention and human decision admission — Hard; exit: state/history/authority transitions are total and non-promoting.
- **Gate = Task-006** real source-grounding integration — Hard; measures lossless shared handoff; pass: every IT-001 subcase and load-bearing negative mutation succeeds at the exact candidate.

### Track C: Post-gate package work

- **C1 = Task-007** integrator package and rights handoff — Hard; exit: the real reader preserves every fact and concurrent/invalid publication exposes nothing favorable.

### Track J: Final assurance

- **J1 = Task-008** independent code/gap review and human release gate — Medium; exit: all planned rows are genuinely backed and an authorized human disposition is recorded or remains explicitly open.

## Parallel Execution Summary

```text
Track S: Task-001 (done)
Track G:          Task-002 [external prerequisites]
Track A:                    Task-003 -> Task-004 -> Task-005 -> Task-006
Track C:                                                               Task-007
Track J:                                                                        Task-008
```

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | S | StR-004, FR-014..FR-018, NFR-004..NFR-005 | TC-066 | done |
| Task-002 | G | FR-014..FR-018, NFR-005 | TC-066, TC-067, TC-073 | blocked |
| Task-003 | A | FR-018 | TC-064..TC-066 | blocked |
| Task-004 | A | FR-014, NFR-004 | TC-059, TC-060, TC-068, TC-069 | blocked |
| Task-005 | A | FR-015, FR-017, NFR-005 | TC-062, TC-063, TC-067, TC-070, TC-073 | blocked |
| Task-006 | A | IT-001, FR-014, FR-015, FR-017, FR-018 | TC-059, TC-060, TC-062, TC-063, TC-066..TC-070, TC-073 | blocked |
| Task-007 | C | FR-016, IT-002 | TC-061, TC-063, TC-066, TC-068, TC-070..TC-072 | blocked |
| Task-008 | J | StR-004, NFR-004, NFR-005 | TC-059..TC-073 | blocked |

## Coordination Rules

- Freeze MRS-004, TM-004 and the closed vocabularies after merge; a semantic
  change requires a new specification review before downstream code.
- Task-002 is an admission gate, not permission to vendor, mirror, branch-pin or
  locally emulate missing shared capabilities.
- Keep one writer for shared census, state and fixture files; downstream tasks
  rebase after each predecessor merges.
- Do not dispatch hosted CI without explicit authorization. Local evidence must
  state its exact environment and may not imply hosted execution.
- No task may present tl-syntax as an editable Quire alternative or infer a
  release/qualification decision from passing automation.
