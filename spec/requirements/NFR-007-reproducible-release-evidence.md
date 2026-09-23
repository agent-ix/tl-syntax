---
id: NFR-007
title: Make coordinated release evidence reproducible
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/FR-029
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-030
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-031
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-032
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-033
    type: constrains
---

# NFR-007: Make coordinated release evidence reproducible

## Statement

Every coordinated release gate after 0.3.0 shall bind its observed result to
the exact candidate commits, dependency resolution, input corpus digests,
tool versions and toolchains so a second run can identify the same subject.

## Scope

Applies to FR-029 through FR-033 and their four-crate release candidate. It
does not define a local evidence framework or replace shared assurance
retention and human decision authorities.

## Rationale

A green result without an exact source and dependency subject cannot justify
an immutable tag. Rerunning at a moving branch or a newer toolchain would
answer a different question.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Release reports missing a required candidate, input or tool identity | 0 | 0 | Test (TC-179) |
| Results accepted after one identity changes | 0 | 0 | Test (TC-179) |

## Verification

Run the gates twice at the same fixed subject, then mutate one commit, pin,
corpus digest, tool version or toolchain identity at a time and verify that
the prior result is not reused as current release evidence.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-007-AC-1 | Each report records exact commits, pins, corpus digests, tool versions, toolchains and gate result; changing any bound identity invalidates the previous release result. | Test (TC-179) |

## Dependencies

Shared assurance contracts own durable evidence and provenance. FR-018 owns
the final human disposition.
