---
id: MP-003
title: tl-syntax source-readiness non-success guard
type: MeasurementPlan
status: proposed
owner: tl-syntax-evidence-owner
metric: tl-syntax.source-readiness-non-success-count
definition_version: tl-syntax.source-readiness-non-success-count/v1
stage: gate
ground_truth_kind: mechanical
protected_apparatus:
  - Makefile
  - Cargo.toml
  - Cargo.lock
  - rust-toolchain.toml
  - clippy.toml
  - rustfmt.toml
  - deny.toml
  - assurance/pins.json
  - spec/test-matrix.md
  - spec/requirements/**
  - scripts/**
  - tests/**
  - examples/**
  - corpus/**
  - fuzz/**
negative_controls:
  - kind: suppressed-observation
    description: Compare the complete declared criterion and configuration population with collected outcomes; a missing outcome is non-success.
  - kind: stale-evidence
    description: Bind every observation to the exact source, configuration, shared dependency, and protected-apparatus identities before it counts.
  - kind: apparatus-edit
    description: A changed harness, expectation, population selector, or checker configuration starts a new measurement definition.
  - kind: selective-reporting
    description: Preserve both required deterministic repetitions and every unfavorable, divergent, or unavailable outcome.
statistical_design:
  population: every applicable readiness criterion, declared candidate/configuration axis, selected domain suite, review operation, limitation, exception, shared dependency, and human decision attribution field
  sampling: complete enumeration with no sampling; stochastic campaigns remain separate attributable observations
  repetitions: 2
  estimator: count
  error_model: stale or substituted source/configuration, omitted obligation, shared-contract incompatibility, producer non-execution, review/decision misattribution, limitation loss, and nondeterministic domain output
  uncertainty: deterministic repetitions must agree; disagreements and every missing, unavailable, or non-success state remain individual reported results
  decision_rule:
    comparator: eq
    threshold: 0
relationships:
  - target: ix://agent-ix/tl-syntax/AP-002
    type: measures
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: measures
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: measures
  - target: ix://agent-ix/tl-syntax/MP-002
    type: references
---

# tl-syntax source-readiness non-success guard

## Decision Use

This proposed guard checks whether the complete, current source-readiness fact
set for one exact candidate and configuration has any non-success item. Its
zero-failure rule restates the block-on-any-non-success rule in the retired
categorical MP-002 proposal. It never approves a release, qualifies native
Quire or TL semantics, publishes a crate, or accepts an adopter's intended use.

## Population

Enumerate every FR-015 through FR-019 and NFR-004/NFR-005 criterion; each
selected feature, target, build, toolchain, corpus, profile, and shared-contract
axis; every domain suite and required review; all assumptions, limitations,
exceptions, counterevidence, and supersession facts; and the presence and
attribution of the human decision field. Record the exact population identity
and count before collection. An absent item is a non-success item, not an
excluded denominator. A missing human decision cannot be supplied by the
measurement itself.

The metric counts missing, failed, skipped, unsupported, stale, suspect,
vacuous, tampered, nondeterministic, or otherwise non-success items across both
deterministic repetitions. An item that differs between repetitions also
counts as non-success. The mechanical count must equal zero for the fact-set
gate to hold. Every item's categorical state and provenance remain visible;
the count is a refusal guard, not a weighted score or a release decision.
This v1 numeric series cannot reinterpret MP-002's historical categorical
observations.

## Collection Procedure

At one clean candidate, select compatible immutable shared releases and
record the configuration identities. Run domain producers through their
declared entry points, obtain Quire's source-grounded export, and pass existing
bytes through the real Engineering Assurance and Quoin interfaces. Quarantine
prior producer output and either isolate immutable inputs or validate every
bound identity before and after each handoff. Repeat the deterministic
population twice under the same declared inputs. Apply the one-axis mutations
from TC-060 and TC-063 through TC-070. Retention belongs to the released
Quoin contract when its configured backend and handle pass retrieval; this
repository stores no local evidence, score, or decision function.

## Interpretation

Report each categorical result, discrepancy, and source identity even when the
count is zero. Environmental or stochastic differences retain their exact
provenance outside the deterministic count. Human review and decision consume
these observations but are never outputs of this plan.
