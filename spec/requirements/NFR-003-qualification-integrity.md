---
id: NFR-003
title: Make qualification controls explicit and fail closed
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: constrains
  - target: ix://agent-ix/tl-syntax/AA-001
    type: references
---

# NFR-003: Make qualification controls explicit and fail closed

## Statement

Candidate qualification shall preserve the distinction between repository
verification, shared-contract intake, and a human release decision; bind every
claimed result to the identified candidate and producer bytes; keep non-success
outcomes out of the passing class; and grant no automated release authority.

## Scope and ownership

[FR-006](./FR-006-shared-assurance-intake.md) owns the functional path that
produces structured domain results and hands them to the released Engineering
Assurance, Quire, and Quoin contracts. This requirement owns the qualification
meaning and lifecycle of those results. It does not make Quire or Quoin a test
runner and does not turn a local test result into a Quoin attestation.

| Control | Requirement owner | Verification | Lifecycle boundary |
|---|---|---|---|
| Shared component compatibility | NFR-003-AC-1 | TC-021 invokes the packaged Engineering Assurance classifier over every declared component and consumed artifact digest. | Re-run for every candidate or adopted shared-component release. |
| Local verification-suite identity | NFR-003-AC-2 | TC-036 checks the declared SUITE-008 command and its exclusion from Quoin proof obligations; the independent exact-head review records whether that local command ran. | Local result for one exact revision only; never a retained Quoin attestation. |
| Reviewed source-set integrity | NFR-003-AC-3 | TC-026, TC-034, and TC-035 exercise deleted-identity scanning, exact tracked paths, byte safety, and mutable-ignore refusals. | Re-evaluated whenever the non-archival tracked set or census policy changes. |
| Producer/result integrity | NFR-003-AC-4 and NFR-003-AC-5 | TC-022 and TC-025 exercise producer non-execution, input derivation, and the twelve-state vocabulary. | Re-run for every candidate and adapter or declaration change. |
| Make execution-control exposure | Qualification boundary below | SR-013 records the only completed behavioral measurement; AA-001 and `assurance/change-assurance.json` keep the limitation open. | The measured global `.IGNORE:` spelling is accepted only for pre-stable development; `agent-ix/tl-syntax#11` requires re-evaluation before the first stable release candidate. |
| Active qualified record | Qualification boundary below | Human review must inspect a current qualified record under `agent-ix/engineering-assurance#11`; no local test can create that authority. | Intentionally unclaimed during pre-stable development; required again before the first stable release candidate. |

The table assigns one qualification owner to every surviving obligation. A
functional criterion in FR-006 can share a test with this requirement without
becoming a second qualification authority: FR-006 says what the intake does;
NFR-003 says what may be inferred from it.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Declared shared components classified by the packaged matrix | 4/4 | 4/4 | Test |
| Local verification suites misrepresented as Quoin proof inputs | 0 | 0 | Test and review |
| Reviewed live-source paths omitted or admitted by mutable machine policy | 0 | 0 | Test |
| Attested results not derived from declared producer bytes | 0 | 0 | Test |
| Non-success outcomes classified as passing | 0 | 0 | Test |
| Automatic qualification or release decisions | 0 | 0 | Inspection |

## Verification

Behavior tests invoke the existing repository gates and released contracts
rather than reimplementing them. TC-036 inspects the structured declaration and
the suite registry only to establish the boundary between a local exact-head
test and a proof input. Independent review remains necessary because a local
test cannot prove that its own execution was reviewed or authorize a release.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-003-AC-1 | Every declared shared component version and consumed artifact digest is classified by the packaged Engineering Assurance compatibility matrix, with no repository-local mapping or internal mirror substitute. | Test (TC-021) |
| NFR-003-AC-2 | SUITE-008 identifies the exact local shared-assurance test command, and neither the change-assurance declaration nor any Quoin proof obligation claims that SUITE-008 was attested; its result is valid only for the exact-head review that reports it. | Test (TC-036) |
| NFR-003-AC-3 | The candidate source identity covers every non-archival tracked path, refuses every ordinary-untracked live path and mutable ignore policy, and scans arbitrary tracked bytes without allowing a forbidden deleted identity to hide. | Test (TC-026, TC-034, TC-035) |
| NFR-003-AC-4 | Every attested proof result is derived from a declared producer's structured bytes, with absent, empty, unreadable, or foreign-protocol input refused, and neither Quire nor Quoin executes a producer. | Test (TC-022) |
| NFR-003-AC-5 | Pass, fail, unavailable, unsupported, inconclusive, not-computed, malformed, partial, stale, suspect, vacuous, and tampered remain distinguishable, and no non-success outcome is reported as passing. | Test (TC-025) |

## Qualification Boundary

These controls make an identified candidate and its produced results
reproducible and reviewable. They confer no qualification, certification,
accreditation, publication decision, or downstream semantic endorsement.

The repository has no comprehensive Make execution-control guard. SR-013
measured one global `.IGNORE:` spelling: it can suppress eight observed failing
or unmade `ci` paths and the assurance chain's refusal while Make exits zero.
Seventeen other spellings and five prerequisite-specific fault cases remain
unmeasured. The `tl-syntax-release-owner` accepts only that disclosed result for
pre-stable development. `agent-ix/tl-syntax#11` owns the repository follow-up,
and `agent-ix/engineering-assurance#11` owns the use-specific qualification
decision. Neither is represented as a passing criterion here.

No active qualified record is claimed for the pre-stable source release. The
preservation constraint released by `agent-ix/engineering-assurance#7` applies
again when the project approaches its first stable release candidate. At that
trigger the human release owner must re-evaluate the Make limitation and require
a current qualified record under `agent-ix/engineering-assurance#11`. A local
test, a Quoin receipt, or a green `make ci` cannot satisfy that human decision.

Branch protection and remote independent-review history, not this repository,
establish resistance to history replacement.
