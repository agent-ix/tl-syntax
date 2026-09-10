---
id: FR-016
title: Emit a shared-contract integrator readiness package
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-014
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-015
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-017
    type: depends_on
---

# FR-016: Emit a shared-contract integrator readiness package

## Description

Where a compatible released integrator-package contract exists, tl-syntax shall
populate that contract with reusable source-release facts while leaving the
integrator's intended-use, deployment and validation decision open.

## Inputs

- Exact source candidate/configuration and stage-classified facts from
  [FR-014](./FR-014-bind-source-readiness-candidate.md) and
  [FR-015](./FR-015-preserve-readiness-stages.md).
- The compatible released integrator-package contract identity, revision and
  digest.
- Declared tl-syntax assumptions, feature/target limits, semantic profile and
  corpus scope, known anomalies, exceptions, counterevidence, review facts,
  human source-release disposition, applicable license/rights notices and
  adoption guidance.
- The authoritative material population plus the issuer/source and applicability
  basis for every license or reuse-right notice.

## Outputs

- A `PackageResult` from MRS-004 and, only for `published`, a package in the
  released shared format with every reusable and still-open obligation
  represented.

## Behavior

- The package shall distinguish immutable source-release facts from adopter-
  supplied intended use, deployed artifact/configuration, platform, monitor,
  hazard/consequence analysis, validation evidence and acceptance authority.
- The package shall retain exact source, crate artifact, configuration, profile,
  corpus, shared-contract, evidence, review, decision, assumption, limitation,
  exception, supersession, license and reuse-right identities.
- The package shall list unsupported and unrepresented consumer obligations
  rather than omitting them or relabeling them as satisfied.
- Each included source/package material shall map to exactly one attributable
  `LicenseResolution` containing a normalized SPDX expression and complete
  authoritative source set.
- The tl-syntax Rust package adapter shall preserve valid `AND`, `OR` and nested
  SPDX expressions as compound expressions rather than conflicts.
- If a license/reuse-right disposition is missing, then the tl-syntax Rust
  package adapter shall produce `incomplete`.
- If license/reuse-right dispositions conflict, then the tl-syntax Rust package
  adapter shall produce `conflict`.
- If a license/reuse-right authority is unverifiable, then the tl-syntax Rust
  package adapter shall produce `refused` without repair by copying a notice.
- The package shall not claim native Quire source-language qualification,
  R2U2/C2PO monitor qualification, consuming-system certification,
  accreditation, authorization or non-repudiation.
- If the compatible released integrator-package contract is absent, then
  tl-syntax shall report `unavailable`.
- If package writing, strict reading or integrity verification fails by spawn,
  timeout, signal, nonzero exit, truncation, partial write or schema/version
  mismatch, then tl-syntax shall retain the exact failure and expose no package.
- The tl-syntax Rust package adapter shall map every package-stage
  `ExecutionOutcome` to the total `PackageResult` mapping in MRS-004 while
  retaining the exact execution value.
- The tl-syntax Rust package adapter shall use candidate-bound temporary output.
- If strict reading and integrity verification succeed, then the tl-syntax Rust
  package adapter shall expose the package atomically.
- If package production fails, then the tl-syntax Rust package adapter shall
  leave no partial or prior package addressable as new.
- The tl-syntax Rust package adapter shall bind a shared-contract idempotency/
  attempt identity.
- If a retry has identical inputs and attempt identity, then the tl-syntax Rust
  package adapter shall resolve it to the same verified object.
- If a retry reuses an attempt identity with non-identical inputs, then the
  tl-syntax Rust package adapter shall produce `conflict`.
- If concurrent writers have non-identical inputs, then the tl-syntax Rust
  package adapter shall produce `conflict` without exposing either as current.
- tl-syntax shall not create a local package schema, evidence store, validator,
  approval workflow or compatibility map.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-016-AC-1 | A package round trip through the real released shared reader preserves every named source-release fact, identity, assumption, limitation, exception, counterexample, review, decision, applicable license and reuse-right state. | Test (TC-061, IT-002) |
| FR-016-AC-2 | The same source release paired with two intended uses or deployed configurations remains two distinct open integrator subjects and inherits no adopter acceptance. | Test (TC-061) |
| FR-016-AC-3 | Removing an unsupported obligation, limitation, exception, negative result or open adopter field cannot improve the package or create a complete integrator disposition. | Test (TC-063) |
| FR-016-AC-4 | Without a compatible released shared contract, package production is explicitly unavailable and no repository-local substitute exists. | Test (TC-066) |
| FR-016-AC-5 | Every package-stage `ExecutionOutcome` maps exactly as MRS-004 specifies, retains its cause and exposes no partial/stale/mislabeled package; only a strictly read and verified candidate-bound temporary object becomes atomically visible. | Test (TC-068, IT-002) |
| FR-016-AC-6 | Identical retries resolve to one verified object; attempt-identity reuse with different inputs and concurrent non-identical writers produce `conflict`, so neither input can win by timing. | Test (TC-071, IT-002) |
| FR-016-AC-7 | Every included source/package material maps to one normalized applicable license/reuse-right resolution and its complete authoritative source set, preserving valid compound expressions; missing disposition is `incomplete`, incompatible expressions are `conflict`, and unverifiable authority is `refused` even when a notice round-trips unchanged. | Analysis (TC-072) |

## Dependencies

The reusable package format and use-specific qualification model belong to
Engineering Assurance. Quoin owns the retention/receipt interface, and a
selected external backend operator/custodian owns stored bytes. No
implementation begins until exact compatible released contracts, authorities
and lifecycle semantics are available and reviewed for this consumer.
