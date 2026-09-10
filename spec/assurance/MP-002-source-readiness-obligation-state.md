---
id: MP-002
title: tl-syntax source-readiness obligation-state plan
type: MeasurementPlan
status: proposed
owner: tl-syntax-evidence-owner
metric: tl-syntax.source-readiness-obligation-state
definition_version: tl-syntax.source-readiness-obligation-state/v1
stage: gate
statistical_design:
  population: every applicable readiness requirement criterion, declared candidate/configuration axis, selected domain suite, review operation, limitation, exception, shared dependency and human decision field
  sampling: complete enumeration with no sampling; stochastic campaigns remain separate attributable observations
  repetitions: 2
  estimator: per-obligation categorical outcome with no weighted or aggregate qualification score
  error_model: stale or substituted source/configuration, omitted obligation, shared-contract incompatibility, producer non-execution, review/decision misattribution, limitation loss and nondeterministic domain output
  uncertainty: deterministic repetitions must agree; disagreements and every missing/unavailable/non-success state are reported individually and never averaged away
  decision_rule: block a source-readiness claim when any required obligation is non-success or lacks current attributable evidence; the measurement never approves release or adopter use
relationships:
  - target: ix://agent-ix/tl-syntax/AP-002
    type: measures
  - target: ix://agent-ix/tl-syntax/NFR-004
    type: measures
  - target: ix://agent-ix/tl-syntax/NFR-005
    type: measures
---

# tl-syntax source-readiness obligation-state plan

## Decision Use

The observation informs the human release owner whether one exact candidate and
configuration has a complete, current and reviewable source-readiness fact set.
It does not approve release, certify a tool, qualify native Quire/TL semantics,
publish a crate or accept an integrator's use.

## Population

Enumerate every FR-014 through FR-018 and NFR-004/NFR-005 criterion; each selected
feature/target/build/toolchain/corpus/profile/shared-contract axis; every domain
suite and required review; all assumptions, limitations, exceptions,
counterevidence and supersession facts; and the human decision field. Record the
exact population identity and count before collection. Additions invalidate the
old complete-population claim.

## Collection Procedure

At one clean candidate, select the compatible immutable shared releases and
record all configuration identities. Run domain producers only through their
declared entry points, obtain Quire's source-grounded export, and pass existing
bytes through the real Engineering Assurance and Quoin interfaces. Quarantine
prior producer output and either isolate immutable inputs or validate every
bound identity before and after each handoff. Repeat the deterministic
population twice under the same declared inputs. Apply one-axis mutations from
TC-060 and TC-063 through TC-070. A released Quoin contract retains produced evidence only when its
configured backend and handle pass retrieval; this
repository stores only specifications, configuration and domain fixtures it
owns.

## Interpretation

Report each categorical result and discrepancy. A missing, failed, skipped,
unsupported, stale, suspect, vacuous, tampered or nondeterministic result blocks
only the claim that requires it and remains visible. Do not compute a score that
can hide an obligation or act as a decision. Environmental or stochastic
differences retain their exact provenance. Human review and decision consume the
measurement but are never outputs of it.
