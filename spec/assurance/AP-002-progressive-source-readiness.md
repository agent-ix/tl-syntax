---
id: AP-002
title: tl-syntax progressive source-readiness profile
type: AssuranceProfile
status: proposed
owner: tl-syntax-release-owner
profile_version: 0.2
profile_kind: general
scope: one exact tl-syntax Rust source candidate, declared build/evaluation configuration and source-release disposition
impact_assessments:
  - id: impact-readiness-subject-substitution
    scenario: evidence or review for one source revision or configuration is reused for another
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/tl-syntax/FR-014
  - id: impact-automated-authority-promotion
    scenario: a local gate, receipt or package is treated as human release or adopter authority
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/tl-syntax/FR-017
  - id: impact-limitation-loss
    scenario: a limitation, exception, negative result or open adopter obligation is omitted from a favorable readiness view
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/tl-syntax/FR-015
  - id: impact-local-assurance-substitute
    scenario: an unavailable shared capability is replaced by repository-local qualification logic
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/tl-syntax/FR-018
review_policy:
  mode: require
  operations: [spec-review, code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-004
    type: governs
  - target: ix://agent-ix/tl-syntax/AP-001
    type: references
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: governs
---

# tl-syntax progressive source-readiness profile

## Decision Boundary

This profile prepares a human source-release decision for one exact Rust source
candidate and configuration. It extends rather than rewrites AP-001. It does not
qualify the native Quire language, a generated MLTL formula, a parser/evaluator/
rewriter/monitor, crates.io publication, a consuming system or an integrator's
intended use.

## Impact Scenarios

The material risks are candidate/configuration substitution, promotion of an
automated observation into human authority, omission of unfavorable or open
facts, and construction of a local assurance substitute when a shared contract
is unavailable. Each is deterministic and admits a one-axis negative control.

## Evidence Policy

Every fact retains source, configuration, producer, procedure, environment,
outcome, stage, authority, limitation, retention-lifecycle and supersession
identities. Developer, source-release and adopter facts remain distinct. A
released Quoin contract retains evidence only when its configured backend and
handle remain retrievable;
Engineering Assurance defines shared compatibility and later use-specific
qualification; Quire owns static source-grounded exports. tl-syntax retains no
evidence store, score, approval registry or compatibility map.

Specification review is base plus failure-domain, integrity, dependency,
evidence, risk-complexity, scope-boundary and EARS analyses. Implementation later
requires independent code review and gap analysis. Human source-release
acceptance remains a separate attributed decision after those reviews.
Every P0 M6 implementation suite must demonstrate at least one load-bearing
negative mutation before its positive results become admissible; matrix
priority is the current criticality carrier because the installed obligation
schema has no acceptance-criterion criticality column.

## Resource Bounds

- A symlink-resolution chain admits at most 32 links.
- The only admitted source-resolution root is the canonical immutable isolated
  candidate root, whose path and filesystem/mount identity are candidate-bound;
  descendant mount crossing is refused.
- A supersession traversal admits at most 4,096 facts, 8,192 directed edges and
  depth 4,096.
- A value at a bound is admitted; the first value above it produces the typed
  non-success required by FR-014 or FR-015 without exposing a partial result.

## Exceptions

No implicit exception exists. Every exception names an owner, exact subject,
affected impact scenario/obligation, rationale, expiry, counterevidence and
human disposition. Expiry or a material subject/configuration change reopens the
affected decision. The existing Quoin implementation is the only permanent
shared-runtime non-Rust accommodation; temporary legacy paths remain pre-stable,
explicitly inventoried and inadmissible for stable qualification. A bounded
owner disposition can authorize only its named pre-stable use; stable use still
requires a Rust/shared replacement with parity. Neither category grants
permission for a new non-Rust readiness path.
