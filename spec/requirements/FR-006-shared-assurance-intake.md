---
id: FR-006
title: Adopt the shared assurance intake path
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-002
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
- The immutable evidence bytes retained under `evidence/`.

## Outputs

- A Quoin change-assurance record sealed from `assurance/change-assurance.json`.
- One Quoin proof attestation per declared proof obligation, over bytes a
  producer already wrote.
- A Quoin verification receipt, and the read-only compatibility view of retained
  evidence.

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
- `scripts/legacy_evidence_view.py` shall read every retained evidence byte
  through the pinned Engineering Assurance mapping.
- `scripts/legacy_evidence_view.py` shall leave every retained evidence byte
  unmodified.
- While the pinned mapping refuses a retained schema family, tl-syntax shall report that refusal.
- tl-syntax shall implement no compatibility mapping of its own.
- tl-syntax shall retain no generic runner, evidence envelope, manifest,
  tool-identity framework, retention store, audit store, anchor file, or
  aggregate verdict.
- The published crate shall depend on neither Quire nor Quoin at runtime.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-006-AC-1 | Every pinned component is classified by the packaged compatibility matrix, no consumed artifact digest differs from its pin, and no internal mirror registry is named anywhere in the repository. | Test (TC-021) |
| FR-006-AC-2 | The corpus conformance, corpus oracle, and feature-boundary results are structured, are produced by this repository's tools, and reach Quoin through the declared adapter without Quoin or Quire executing a producer. | Test (TC-022) |
| FR-006-AC-3 | Static specification, obligation, and coverage facts for a candidate revision come from the Quire export named by the sealed record's impact snapshot. | Test (TC-023) |
| FR-006-AC-4 | Every retained evidence byte is unchanged by a compatibility run, every retained envelope is read through the pinned mapping, and the mapping's answer is reported without being converted into a pass or a failure. | Test (TC-024) |
| FR-006-AC-5 | Each of the twelve verification outcomes is demonstrated by a case that produced it, and each negative case is paired with a positive control that was observed to be accepted. | Test (TC-025) |
| FR-006-AC-6 | No script, Make target, or test in the repository implements a generic evidence envelope, manifest, retention store, tool-identity lock, anchor file, or aggregate verdict, and the frozen evidence schemas are referenced by nothing. | Test (TC-026) |

## Dependencies

Depends on [NFR-002](./NFR-002-determinism-and-integrity.md) and
[FR-005](./FR-005-conformance-corpus.md). Constrained by the accepted shared
release pins recorded in `assurance/pins.json` and by the migration contract at
`agent-ix/engineering-assurance#10`.
