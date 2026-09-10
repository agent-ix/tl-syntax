---
id: StR-004
title: Release owners and integrators need bounded source-readiness facts
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-014
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-015
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-016
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-017
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-018
    type: satisfied_by
---

# StR-004: Release owners and integrators need bounded source-readiness facts

## Stakeholder Need

Release owners and downstream integrators require that tl-syntax shall expose
reviewable facts for one exact Rust source candidate without turning developer
checks, a source-release decision or an integrator's later validation into one
interchangeable qualification claim.

## Rationale

The same test result can be useful during development, source release and
downstream adoption while carrying different authority and assumptions. If
those stages or their configurations are collapsed, a green local command can
be mistaken for human approval or for evidence about a consuming system that
was never assessed.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-004-VC-1 | A release owner can identify the exact candidate, configuration, review state, evidence limitations and human disposition without inferring any missing field. | Demonstration (TC-059, TC-062) |
| StR-004-VC-2 | An integrator can distinguish reusable source-release facts from adopter-supplied intended use, deployment configuration and validation obligations. | Demonstration (TC-061) |
| StR-004-VC-3 | No artifact describes tl-syntax as a user-authored Quire alternative, certifies a tool or monitor, or grants automated release authority. | Inspection (TC-066) |

## Stakeholders

The tl-syntax human source-release owner, downstream TL crate maintainers,
native Quire bridge maintainers, consuming-system integrators and independent
assurance reviewers.

## Context and Assumptions

The existing v0.1 assurance profile remains the pre-stable source-release
boundary. Shared Engineering Assurance, Quire and Quoin contracts own generic
compatibility, evidence retention and review orchestration. A future reusable
integrator-package contract may be unavailable at the time this requirement is
reviewed.

## Stakeholder Constraints (Contextual)

Useful pre-stable delivery must not wait for a universal qualification scheme.
The readiness contract must remain valid when a dependent shared capability is
reported unavailable and must not replace that capability locally.

## Dependencies

Upstream policy comes from `ix://agent-ix/quire-contract-ir/PGM-01`, the native-
language owner ruling and the released shared-assurance contracts. Downstream
realization is allocated to [FR-014](./FR-014-bind-source-readiness-candidate.md)
through [FR-018](./FR-018-classify-qualification-execution-paths.md),
[NFR-004](./NFR-004-reproduce-source-readiness-observations.md) and
[NFR-005](./NFR-005-preserve-readiness-authority-and-retention.md).

## Priority and Risk (Informative)

Priority is high before a stable source-release candidate. The principal risk
is an overbroad qualification or certification inference from evidence gathered
for a narrower candidate and configuration.

## Traceability

This need is the stakeholder boundary for `agent-ix/tl-syntax#31` and
`agent-ix/tl-syntax#34`.
