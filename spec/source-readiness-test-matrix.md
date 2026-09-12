---
id: TM-004
title: Progressive source-readiness test matrix
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-004
    type: covers
---

# Progressive source-readiness test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-014 | FR-014-AC-1 through FR-014-AC-6 | TC-059, TC-060, TC-066, TC-068, TC-069 | 🚧 planned |
| FR-015 | FR-015-AC-1 through FR-015-AC-7 | TC-062, TC-063, TC-066, TC-067, TC-070 | 🚧 planned |
| FR-016 | FR-016-AC-1 through FR-016-AC-7 | TC-061, TC-063, TC-066, TC-068, TC-071, TC-072, IT-002 | 🚧 planned |
| FR-017 | FR-017-AC-1 through FR-017-AC-7 | TC-062, TC-063, TC-066, TC-070, TC-073 | 🚧 planned |
| FR-018 | FR-018-AC-1 through FR-018-AC-4 | TC-064, TC-065, TC-066 | 🚧 planned |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Coverage Status |
|---|---|---|---|
| StR-004 | StR-004-VC-1 through StR-004-VC-3 | TC-059, TC-061, TC-062, TC-066 | 🚧 planned |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-004 | Repeatability, one-axis mutation and boundary inspection | TC-059, TC-060, TC-062, TC-063, TC-064, TC-066 | 🚧 planned |
| NFR-005 | Authority/subject/lifecycle mutation, retrieval and boundary inspection | TC-063, TC-066, TC-067, TC-073 | 🚧 planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-059 | Repeat the complete source-readiness population at one exact candidate/configuration | Integration | P0 | FR-014-AC-1, NFR-004-AC-1, NFR-004-AC-5, StR-004-VC-1 | 🚧 planned |
| TC-060 | Mutate every candidate, source, configuration, environment and shared-contract identity | Property | P0 | FR-014-AC-2, FR-014-AC-3, NFR-004-AC-2 | 🚧 planned |
| TC-061 | Round-trip source facts and two distinct open adopter subjects through the real shared package | Integration | P0 | FR-016-AC-1, FR-016-AC-2, StR-004-VC-2, IT-002 | 🚧 blocked on shared contract |
| TC-062 | Preserve developer, release-input, human-decision and adopter stages and all six decision dispositions | Integration | P0 | FR-015-AC-1, FR-017-AC-1, NFR-004-AC-3, StR-004-VC-1 | 🚧 planned |
| TC-063 | Refuse promotion, limitation omission, stale decisions and unknown-value coercion | Integration | P0 | FR-015-AC-2, FR-015-AC-3, FR-015-AC-7, FR-016-AC-3, FR-017-AC-2, FR-017-AC-3, NFR-004-AC-3, NFR-005-AC-3 | 🚧 planned |
| TC-064 | Census and classify every executable readiness path and new-path mutation | Integration | P0 | FR-018-AC-1, FR-018-AC-2, NFR-004-AC-4 | 🚧 blocked on LR08 disposition |
| TC-065 | Demonstrate positive and forced-failure parity for each replaced executable path | Integration | P0 | FR-018-AC-3 | 🚧 blocked on reviewed replacements |
| TC-066 | Prove authority, language, certification and no-local-substitute boundaries | Integration | P0 | FR-014-AC-4, FR-015-AC-4, FR-016-AC-4, FR-017-AC-4, FR-018-AC-4, NFR-004-AC-4, NFR-005-AC-4, StR-004-VC-3 | 🚧 planned |
| TC-067 | Mutate evidence lifecycle/content binding and exercise durable-handle retrieval after disposable workspace deletion | Integration | P0 | FR-015-AC-5, NFR-005-AC-1 | 🚧 blocked on selected shared retention contract |
| TC-068 | Quarantine stale outputs and force skipped, crashed, timed-out, malformed and partial producer/package executions | Integration | P0 | FR-014-AC-5, FR-016-AC-5 | 🚧 planned |
| TC-069 | Mutate repository/Git/materialized/admitted-root/path/mount identities and probe symlink escape/cycle/race/special/mount/bound cases | Property | P0 | FR-014-AC-6 | 🚧 planned |
| TC-070 | Mutate supersession topology/bounds, conditional/deferred transitions and concurrent review/decision successors | Property | P0 | FR-015-AC-6, FR-017-AC-7 | 🚧 planned |
| TC-071 | Race non-identical package writers and retry identical/different attempt identities | Integration | P0 | FR-016-AC-6 | 🚧 blocked on accepted shared contract |
| TC-072 | An independent rights reviewer reconciles the exact candidate's tracked source, package contents, dependency closure and generated/embedded third-party material against identity-bound authoritative license and reuse-right sources | Analysis | P0 | FR-016-AC-7, IT-002-SC-07 | 🚧 planned |
| TC-073 | Mutate reviewer/decision policy, actor, contributor, conflict, delegation, revocation, quorum, head/base/configuration and evidence bindings | Property | P0 | FR-017-AC-5, FR-017-AC-6, NFR-005-AC-2 | 🚧 blocked on authoritative policy/event source |

## Planned Suite Allocation

| Test Cases | Planned Suites |
|---|---|
| TC-059, TC-060, TC-062, TC-069, TC-073 | SUITE-009 and, for real shared boundaries, SUITE-011 |
| TC-063, TC-064, TC-066 | SUITE-009 |
| TC-070 | SUITE-009 and SUITE-012 |
| TC-065, TC-068 | SUITE-010 and, for package failures, SUITE-012 |
| TC-061, TC-071 | SUITE-012 |
| TC-067 | SUITE-011 |
| TC-072 | SUITE-013 |

## Matrix Design Coverage

| Required design rule | Coverage |
|---|---|
| Every acceptance criterion mapped | FR/StR/NFR tables map every criterion to TC-059 through TC-073. |
| Relevant option permutations | TC-061 varies intended use/deployed configuration; TC-062 varies evidence stage and decision disposition. |
| Boundary conditions | TC-059 fixes the complete population; TC-060/069 change one identity/materialization axis; TC-067 crosses transient/durable and current/expired boundaries. |
| Error conditions | TC-063/068 cover missing, stale, omitted, crashed, partial and unsupported results; TC-066 covers unavailable shared capabilities. |
| State transitions | TC-062/063/070 cover promotion, decision, expiry, invalidation, conflict and supersession transitions. |
| Edge cases | TC-060/063/067/068/069/070 cover substitution, self-review, workspace loss, stale output, source indirection, cycles/forks, unavailable handles, limitation omission and silent restamping. |

## Integration Test Matrix

| Integration ID | Purpose | Type | Target | Test Cases | Status |
|---|---|---|---|---|---|
| IT-001 | Source-grounded handoff through real released Engineering Assurance, Quire and Quoin | service | shared assurance stack | TC-059, TC-060, TC-062, TC-063, TC-066 through TC-070, TC-073 | 🚧 blocked on tl-syntax#16 release set and authoritative policy/event source |
| IT-002 | Lossless future integrator-package handoff without transferred qualification | service | Engineering Assurance integrator package | TC-061, TC-062, TC-063, TC-066, TC-068, TC-070 through TC-072 | 🚧 blocked on accepted shared contract |

## Evidence Limits

Every row is planned or explicitly blocked. This matrix claims no executed M6
test, complete source release, qualified record, shared package, hosted run,
signed tag, publication or human decision. Existing v0.1 tests do not
automatically back these new criteria. A TC row passes only when every named
subcase and conjunct succeeds; a skipped, unavailable or partially executed
subcase is reported separately and leaves the row non-passing.
