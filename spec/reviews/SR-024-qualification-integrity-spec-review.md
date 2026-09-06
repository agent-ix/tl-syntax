---
id: SR-024
title: Composite review of qualification-integrity ownership
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#19 author specification candidate; NFR-003; FR-006; NFR-002; SUITE-008; qualification lifecycle"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-006
    type: references
---

# SR-024: Composite review of qualification-integrity ownership

## Summary

This author-performed pre-implementation composite review separates functional
intake from qualification meaning and assigns every issue #19 control without
reviving the deleted local evidence framework. It grants no merge authority.

## Ownership review

| Control | Disposition | Falsifying evidence |
|---|---|---|
| Shared component compatibility | NFR-003-AC-1 owns the compatible/matching result; FR-006-AC-1 owns supplying declarations to the packaged classifier. | A missing component, incompatible version, digest mismatch, or mirror reference makes TC-021 red. |
| Verification-suite identity | NFR-003-AC-2 owns exact SUITE-008 identity and its non-attested status. | TC-036 changes the suite command or inserts the local suite into a proof claim and must fail. |
| Source-set integrity | NFR-003-AC-3 owns candidate-identity completeness; FR-006-AC-6/AC-7/AC-8 own census mechanics. | TC-026/TC-034/TC-035 mutate paths, content bytes, and ignore policy. |
| Producer/result integrity | NFR-003-AC-4/AC-5 own qualification interpretation; FR-006-AC-2/AC-5 own adapter behavior. | TC-022 removes or changes producer bytes and stubs tools; TC-025 exercises all states and controls. |
| Make execution control | Explicitly unclosed under NFR-003, SR-013, AA-001, tl-syntax#11, and engineering-assurance#11. | A green local gate cannot close it; first stable candidacy triggers re-evaluation. |
| Active qualified record | Explicitly unclaimed pre-stable and human-owned under engineering-assurance#11. | Absence remains a named limitation; no local artifact is accepted as a substitute. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2401 | medium | Moving the Make and active-record prose out of NFR-002 could erase the historical NFR-002-AC-4 meaning or imply blanket succession. | NFR-002-AC-4, NFR-003 | wrong-requirement |
| FND-2402 | medium | Naming SUITE-008 in a qualification requirement could falsely imply Quoin attests the local Rust test run. | NFR-003-AC-2, SUITE-008, TC-036 | wrong-requirement |
| FND-2403 | low | Functional FR-006 criteria and NFR-003 quality criteria can appear duplicative. | FR-006, NFR-003 | wrong-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2401 | **PREVENTED** | The retired section retains every clause and maps each explicitly; no retired identifier is reused. |
| FND-2402 | **PREVENTED** | NFR-003-AC-2 requires the opposite interpretation and TC-036 will check the proof declaration. |
| FND-2403 | **ACCEPTED WITH EXPLICIT LAYER SPLIT** | FR-006 owns operations; NFR-003 owns qualification inference and lifecycle. |

## Architecture and dependency analysis

- The only new executable work is a Rust assertion inside the existing
  `shared_assurance` suite. No Python helper, Make target, runner, collector,
  schema, envelope, registry, or retention surface is authorized.
- The shared Engineering Assurance matrix remains the compatibility authority;
  Quire remains the static exporter; Quoin remains the record/attestation/
  receipt authority and executes no producer.
- The branch is intentionally stacked on PR #22 and cannot be independently
  merged from its current base. PR #21 and PR #22 exact heads stay unchanged.
- The Make and stable-record gaps are lifecycle constraints, not requirements to
  build a Make parser or a local qualification record.

## Conclusion

The specification is coherent, falsifiable where local behavior is claimed,
and explicit where authority remains human or deferred. Implementation may
proceed on the stacked branch. Independent exact-head review remains required.
