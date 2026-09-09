---
id: FR-006
title: Adopt the shared assurance intake path
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
---

# FR-006: Adopt the shared assurance intake path

## Description

When verification results are recorded for a candidate revision, tl-syntax shall
hand its own tools' declared structured results to the released Engineering
Assurance, Quire, and Quoin contracts rather than to a repository-local evidence
framework.

## Inputs

- The accepted Engineering Assurance compatibility matrix and the component
  versions it pins.
- Structured results produced by this repository's own tools: the corpus
  conformance runner, the corpus semantic oracle, and the feature-boundary gate.
- The Quire static export of specification, obligation, and coverage facts.

## Outputs

- A Quoin change-assurance record sealed from `assurance/change-assurance.json`.
- One Quoin proof attestation per declared proof obligation, over bytes a
  producer already wrote.
- A Quoin verification receipt.

## Behavior

- `engineering_assurance.compatibility` shall classify every observed component
  version.
- tl-syntax shall observe its own toolchain without restating the compatibility
  matrix.
- Quire shall export static specification, obligation, and coverage facts
  without executing a producer.
- Quoin shall transcribe declared structured results without executing a
  producer.
- `make assurance-inputs` shall be the only target that executes a producer.
- Each downstream gate shall report an absent producer input as an error.
- The native adapter shall transcribe the one protocol it names.
- The native adapter shall refuse a stream declaring any other protocol.
- No gate shall recover a verdict from a process's output stream while that
  process emits a structured result.
- tl-syntax shall keep pass, fail, unavailable, unsupported, inconclusive,
  not-computed, malformed, partial, stale, suspect, vacuous, and tampered
  distinguishable from one another.
- tl-syntax shall report no non-success outcome as a success.
- tl-syntax shall retain no evidence of its own.
- tl-syntax shall implement no compatibility mapping of its own.
- tl-syntax shall retain no generic runner, evidence envelope, manifest,
  tool-identity framework, retention store, audit store, anchor file, or
  aggregate verdict.
- The live-source scan shall inspect arbitrary file bytes for the deleted ASCII
  identities; a non-UTF-8 tracked input shall not be misreported as an
  enumeration failure.
- Repository-authored `.gitignore` files shall be the only ignore rules applied
  to the ordinary-untracked census. Workstation `core.excludesFile` and
  administrative `.git/info/exclude` rules shall not hide a live input, and the
  active `.gitignore` bytes shall match the version-control index.
- The published crate shall depend on neither Quire nor Quoin at runtime.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-006-AC-1 | Every declared component version and consumed artifact digest is supplied to the packaged compatibility classifier, and the repository supplies neither a local compatibility mapping nor an internal mirror registry. | Test (TC-021) |
| FR-006-AC-2 | The corpus conformance, corpus oracle, and feature-boundary results are structured, are produced by this repository's tools, and reach Quoin through the declared adapter without Quoin or Quire executing a producer. | Test (TC-022) |
| FR-006-AC-3 | Static specification, obligation, and coverage facts for a candidate revision come from the Quire export named by the sealed record's impact snapshot. | Test (TC-023) |
| FR-006-AC-5 | Each of the twelve verification outcomes is demonstrated by a case that produced it, and each negative case is paired with a positive control that was observed to be accepted. | Test (TC-025) |
| FR-006-AC-6 | No live repository source implements or names the deleted generic evidence envelope, manifest, retention store, tool-identity lock, anchor file, or aggregate verdict. Immutable review and plan records that describe the deleted subjects, the declaring test, and an inert directory marker are explicit non-live exclusions. | Test (TC-026) |
| FR-006-AC-7 | Every version-control-tracked non-archival path is present in the exact reviewed live-source set regardless of its name or extension; a changed per-area population is diagnosed before the exact path delta; every ordinary-untracked live path is scanned, named, and refuses a clean reviewed population; repository-authored ignored generated paths do not redefine that population; and unavailable enumeration is a typed refusal rather than an intercepted panic. | Test (TC-034) |
| FR-006-AC-8 | The deleted-identity scan accepts arbitrary non-UTF-8 tracked bytes, still detects an embedded forbidden ASCII identity with its path named, includes ordinary-untracked paths even when workstation or administrative Git excludes name them, and refuses untracked or index-divergent `.gitignore` policy. | Test (TC-035) |

### Source-census reproducibility

The clean-population refusal in FR-006-AC-7 is deliberate: an unstaged live
source is scanned so it cannot hide a deleted identity, then the gate fails
because the file is not part of the reviewed tracked population. Generated
paths are excluded only by version-controlled per-directory `.gitignore`
rules whose active bytes match the Git index. A developer-global ignore file,
`.git/info/exclude`, an untracked `.gitignore`, or unstaged edits to a tracked
`.gitignore` are mutable local state, so accepting any of them as census policy
would let two checkouts of one revision scan different inputs.

### Qualification ownership

This requirement owns intake behavior: producing domain results, exporting
static facts, transcribing declared bytes, retaining state distinctions, and
enumerating the live source set. [NFR-003](./NFR-003-qualification-integrity.md)
owns what those behaviors permit a reviewer to infer about one candidate, the
identity and non-attested status of the local SUITE-008 run, the disclosed Make
execution-control limitation, and the stable-release qualified-record trigger.
That split does not make either requirement a second runner or evidence format.

### Retired criteria

**FR-006-AC-4** is retired. It required this repository to read every retained
evidence byte through the pinned Engineering Assurance mapping, leave those
bytes unmodified, and report the mapping's answer without collapsing it. There
is no longer anything to read: the 23 retained `quire.derivation-evidence/v1`
envelopes were deleted under `agent-ix/tl-syntax#12`, on the preservation
constraint that `agent-ix/engineering-assurance#7` released for the pre-stable
phase on 2026-09-02. The criterion is not restated more weakly and its
obligation is not moved elsewhere; it went with its subject.

The identifier is **not reused**, on the same terms as NFR-002-AC-4. `SR-008`
adjudicates coverage against `FR-006-AC-4` and `TC-024` by name, and an
identifier that means one thing in a closed review and another in the current
specification makes both unreadable. Test case **TC-024** and suite
**SUITE-007** are retired with it.

The mapping refusal this criterion reported is filed upstream as
`agent-ix/engineering-assurance#21`, which becomes moot for this repository
rather than fixed. The constraint re-applies unchanged at the move toward stable
releases; evidence retained from that point is immutable.

## Dependencies

Depends on [NFR-003](./NFR-003-qualification-integrity.md) and
[FR-005](./FR-005-conformance-corpus.md). Constrained by the accepted shared
release pins recorded in `assurance/pins.json` and by the migration contract at
`agent-ix/engineering-assurance#10`.
