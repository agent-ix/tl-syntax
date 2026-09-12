---
id: NFR-005
title: Preserve source-readiness authority and retention truth
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/FR-015
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-016
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-017
    type: constrains
---

# NFR-005: Preserve source-readiness authority and retention truth

## Statement

When a source-readiness fact is retained, reviewed, transferred or consumed in
a decision, the readiness path shall preserve its exact authority, subject,
limitations and lifecycle.

## Scope

- Applies to developer observations selected for release, source-release facts,
  review relations, decision events and future integrator-package references.
- Covers transient and durable evidence, unavailable or expired handles,
  supersession, accepted limitations and independent-review identity.
- The readiness path shall not convert identity or integrity evidence into
  provenance, sufficiency or decision authority.
- Does not create a repository-local evidence store, reviewer registry,
  signature authority, compatibility map or approval service.

## Rationale

A content digest can establish byte identity but cannot by itself establish who
produced the bytes, whether the procedure ran, whether the evidence is
sufficient or whether a human accepted the subject. Likewise, a path under an
ignored workspace directory is not durable retention merely because a command
describes it as retained.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Required facts missing authority, subject, limitation or lifecycle identity | 0 | 0 | Test (TC-067) |
| Self-reviews or stale reviews accepted as independent/current | 0 | 0 | Test (TC-073) |
| Unavailable or workspace-only bytes reported as durably retained | 0 | 0 | Test (TC-067) |
| Digest/receipt facts promoted into provenance, sufficiency or decision claims | 0 | 0 | Test (TC-066) |

## Verification

Inspect the selected shared retention contract and exercise retrieval for every
retained handle. Mutate reviewer, subject, head, base, configuration, evidence
set, limitation, lifecycle, availability, returned bytes, media type and handle
binding independently. Delete only a
disposable test workspace and prove that any claimed durable handle remains
retrievable from its declared shared authority; otherwise classify it as
transient, unavailable or expired.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-005-AC-1 | Every evidence fact has a declared retention authority/backend/operator, immutable handle, lifecycle/retrieval/content/subject/freshness bindings and exactly one MRS-004 `RetentionState`; current retrieval verifies each binding and every failure follows the total mapping. | Test (TC-067) |
| NFR-005-AC-2 | Reviewer identity/independence policy and exact head/base/configuration/evidence-set bindings are preserved; self-review, unverifiable policy/authority and each one-axis binding mutation remain open or invalid. | Test (TC-073) |
| NFR-005-AC-3 | Every claim retains applicable assumptions, limitations, exceptions, counterevidence and accepted dispositions; omission cannot improve readiness. | Test (TC-063) |
| NFR-005-AC-4 | A digest or receipt proves only the property supplied by its released contract and never implies provenance truth, actor identity, non-repudiation, correctness, sufficiency or acceptance. | Test (TC-066) |

## Dependencies

Released Quoin contracts own the retention/receipt interface; a separately
selected external backend operator/custodian owns stored bytes. tl-syntax only
consumes and verifies handles. GitHub review identity and the declared human
source-release owner remain external authorities. Until those exact contracts,
operators and handles are selected and exercised, retention and independent-
review claims remain planned or unavailable.
