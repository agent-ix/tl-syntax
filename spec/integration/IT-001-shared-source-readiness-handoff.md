---
id: IT-001
title: Hand source-readiness inputs through the released shared stack
type: IT
relationships:
  - target: ix://agent-ix/tl-syntax/FR-014
    type: verifies
  - target: ix://agent-ix/tl-syntax/FR-015
    type: verifies
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: verifies
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: verifies
  - target: ix://agent-ix/tl-syntax/FR-017
    type: verifies
---

# IT-001: Hand source-readiness inputs through the released shared stack

## Objective

Verify that one exact tl-syntax candidate and configuration flow through the
real released Engineering Assurance, Quire and Quoin interfaces without a local
schema/parser/runner and without promoting an unavailable or incomplete result.

## Target Integration

The system under test is the tl-syntax source-readiness handoff. External
dependencies are the exact released Engineering Assurance compatibility
classifier, Quire source-grounded export and Quoin record/attestation/intake/
receipt interfaces. Tests invoke the installed real interfaces; none is mocked.

## Preconditions

A compatible immutable release set exists, is installed from its public
artifacts and is identified by the packaged Engineering Assurance matrix. The
worktree is clean at one exact reviewed commit. Domain producers and their
configurations are available. Hosted CI remains manual and need not be
dispatched for this local integration.

## Inputs

One valid candidate/configuration, its complete tracked source set, real
structured domain results and Quire export; paired mutations omit or alter one
source path/digest, feature, target, toolchain, corpus, shared artifact or
producer input.

## Test Procedure

1. Probe and classify every installed shared artifact through the released
   compatibility interface.
   - IT-001-SC-01: every selected identity/version/digest is compatible and no local mapping is consulted.
2. Produce domain results only through the declared tl-syntax producer target,
   then obtain the real source-grounded Quire export.
   - IT-001-SC-02: the export binds every selected requirement/source identity and reports no inferred or omitted live source.
3. Pass the existing bytes through Quoin seal, attestation, intake and receipt
   operations without allowing Quire or Quoin to execute a producer.
   - IT-001-SC-03: every output retains the exact subject/configuration and actual non-success/human-decision state.
4. Repeat each one-axis missing, stale, incompatible and substituted input case.
   - IT-001-SC-04: each case refuses or remains unavailable/incomplete and cannot reuse the healthy result.
5. Quarantine prior output, force skip/crash/timeout/partial-output outcomes,
   and mutate one bound source/configuration identity during a run.
   - IT-001-SC-05: no stale output is accepted and every mid-run mutation refuses the observation.
6. Substitute repository/Git object identity and each applicable symlink,
   submodule, Git LFS, generated or fetched-input identity.
   - IT-001-SC-06: each changed materialized input creates a new subject or an explicit unavailable result.
7. Mutate the selected reviewer/decision policy and each actor, contributor,
   conflict, delegation, revocation, quorum and review-subject binding.
   - IT-001-SC-07: an unverifiable event is refused while the disposition stays open; insufficient quorum stays open without a local policy registry.
8. Retrieve every claimed durable handle after deleting only the disposable
   producer workspace, then substitute returned bytes, media type, subject,
   lifecycle and handle binding.
   - IT-001-SC-08: durable success requires current exact retrieval from the declared backend/operator; every mutation is unavailable, tampered or refused rather than retained success.
9. Submit acyclic, dangling, cyclic, cross-subject, forked and contradictory
   supersession relations plus unresolved, satisfied and expired conditional
   decision states.
   - IT-001-SC-09: graph and decision transitions match FR-015/FR-017 exactly and never choose a favorable branch or auto-accept a satisfied condition.

## Expected Results

The healthy candidate produces source-grounded shared artifacts for its exact
subject. Every mutated case fails on the changed axis. A missing human decision
remains missing, and no command or receipt approves the release.

## Metadata

- Priority: High
- Target Integration: released Engineering Assurance + Quire + Quoin interfaces
- Automation: Planned Rust integration test plus real shared CLI boundary

## Dependencies

`agent-ix/tl-syntax#16` remains blocked until a released Engineering Assurance
matrix accepts the source-grounded Quire artifact. This test cannot run against
a branch-head or local substitute.

## Notes

The exact shared versions are selected at implementation time from a reviewed
immutable compatible release set; this draft does not approve the currently
missing combination.

## Traceability

Planned matrix cases TC-059, TC-060, TC-062, TC-063, TC-066 through TC-070 and
TC-073 exercise this integration boundary.
