---
id: MRS-004
title: Progressive tl-syntax source-readiness and integrator boundary
type: MasterRequirements
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/issues/34
    type: references
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
---

# Progressive tl-syntax source-readiness and integrator boundary

## Purpose

This specification defines progressive readiness facts for a human decision
about one exact tl-syntax Rust source candidate and configuration, plus a
deferred boundary for transferring those facts to a future shared integrator
package. It extends the v0.1 source-release boundary without retroactively
blocking useful pre-stable delivery.

“Source” means the Rust repository candidate. Native Quire remains the sole
editable formal-clause language. TL text and formula documents are internal
interchange/evaluator inputs and never a second user-authored Quire source
profile.

## Scope

### In Scope

- Exact candidate, source-population, crate, configuration, toolchain, profile,
  corpus, shared-contract and producer identities.
- Distinct developer observation, source-release input, human decision and
  integrator-adoption stages.
- Review, evidence/counterevidence, limitation, exception, retention,
  supersession and human-authority rules.
- A future shared-contract package containing reusable source facts and still-
  open adopter obligations.
- Reproducibility controls and a complete executable-path disposition.

### Out of Scope

- Native Quire language qualification or an editable TL/FRETish source path.
- Automatic release/publication decisions, crates.io publication, universal
  trusted-tool status, certification, accreditation, authorization or
  non-repudiation.
- Qualification of tl-parse, tl-rewrite, tl-mltl, R2U2/C2PO, another monitor,
  a consuming system or an integrator's intended use.
- A repository-local source parser, compatibility matrix, generic runner,
  evidence schema/store, aggregate score, approval workflow or substitute for
  an unavailable shared capability.

## System Overview

### System Description

tl-syntax supplies domain results and exact component-owned assumptions/
limitations. Released Engineering Assurance, Quire and Quoin contracts supply
shared compatibility, source grounding, retention and orchestration. Independent
reviewers assess the exact head and evidence set. Only the declared human
release owner decides the source release. A later integrator supplies a distinct
intended use, deployment/configuration and validation decision.

### Intended Users

The human tl-syntax release owner, independent specification/code/gap reviewers,
TL ecosystem maintainers, native Quire bridge maintainers and future
consuming-system integrators.

## Lifecycle Vocabulary

| Stage | Meaning | Authority limit |
|---|---|---|
| Developer observation | One attributable local or hosted result for an exact revision/configuration | No retention, independence, release or adopter inference unless separately established |
| Source-release input | Candidate-bound fact selected for review and retained through shared contracts | Does not approve release |
| Human source-release decision | Attributed accepted/rejected/deferred/conditional/open/conflict disposition for the exact candidate | Does not publish or qualify downstream use |
| Integrator-adoption input | Reusable source facts plus adopter intended use/configuration and open validation fields | Does not inherit source-release acceptance as adopter acceptance |
| Use-specific qualification | Separately scoped producer/system decision under Engineering Assurance | Outside this component profile until its shared contract and owner decision exist |

## Closed Result Vocabularies

The following closed types are distinct even where a label is linguistically
similar. No implementation may store them in one untyped status field.

| Type | Values |
|---|---|
| `ExecutionOutcome` | `succeeded`, `skipped`, `spawn-failed`, `completed-nonzero`, `signalled`, `timed-out`, `truncated`, `invalid-output` |
| `ReadinessState` | `pass`, `fail`, `unavailable`, `unsupported`, `inconclusive`, `not-computed`, `malformed`, `partial`, `stale`, `suspect`, `vacuous`, `tampered`, `incomplete`, `conflict`, `refused` |
| `ReviewRelation` | `current`, `open`, `invalid`, `conflict` |
| `DecisionEventAdmission` | `admitted`, `refused` |
| `DecisionDisposition` | `accepted`, `rejected`, `deferred`, `conditional`, `open`, `conflict` |
| `PackageResult` | `published`, `failed`, `unavailable`, `unsupported`, `incomplete`, `refused`, `conflict`; every non-published result carries its exact cause |
| `RetentionState` | `transient`, `retained-current`, `unavailable`, `expired`, `deleted`, `tampered`, `refused`, `unsupported` |
| `LicenseResolution` | `complete`, `incomplete`, `conflict`, `refused`; `complete` contains one normalized SPDX expression and its complete authoritative source set |
| `TypedDecode<T>` | `decoded(T)`, `unsupported { raw }`, `refused { raw, cause }`; the non-decoded variants do not construct a value of `T` |

Execution mapping is total and non-improving: `skipped` maps to `not-computed`;
`spawn-failed` to `unavailable`; `completed-nonzero`, `signalled` and
`timed-out` to `fail`; and `truncated`/`invalid-output` to `malformed`.
`succeeded` only admits fresh output for domain interpretation and does not
itself map to `pass`. Every serialized occurrence of every closed result type is
decoded through `TypedDecode<T>`: an unknown enum value produces `unsupported`
and preserves the raw value, while a malformed known encoding produces
`refused` and preserves its raw bytes and cause. Review/event/package/decision/
license states never implicitly map into another type, and a decode failure is
never injected into the domain enum it failed to construct.

For package execution, `skipped` maps to `incomplete`, `spawn-failed` to
`unavailable`, `completed-nonzero`/`signalled`/`timed-out` to `failed`, and
`truncated`/`invalid-output` to `refused`; each result retains the exact
`ExecutionOutcome`. For retention, workspace-only bytes are `transient`, a
fresh successful exact retrieval is `retained-current`, denied/missing backend
or handle is `unavailable`, elapsed lifecycle is `expired`, confirmed governed
removal is `deleted`, content/binding mismatch is `tampered`, malformed known
input is `refused`, and an unknown enum value is `unsupported`.

## Requirements Architecture

[StR-004](./requirements/StR-004-progressive-source-readiness.md) states the
stakeholder need. [FR-014](./requirements/FR-014-bind-source-readiness-candidate.md)
binds the candidate/configuration;
[FR-015](./requirements/FR-015-preserve-readiness-stages.md) separates evidence
stages; [FR-016](./requirements/FR-016-emit-integrator-readiness-package.md)
defines the deferred package boundary;
[FR-017](./requirements/FR-017-require-human-source-release-decision.md) keeps
human authority exact; and
[FR-018](./requirements/FR-018-classify-qualification-execution-paths.md)
governs executable-path ownership. [NFR-004](./requirements/NFR-004-reproduce-source-readiness-observations.md)
constrains reproducibility, while
[NFR-005](./requirements/NFR-005-preserve-readiness-authority-and-retention.md)
constrains authority and retention truth. AP-002, AD-002 and MP-002 record assurance scope,
architecture and observation meaning. [TM-004](./source-readiness-test-matrix.md)
allocates planned evidence.

## Dependency Map

The internal hard-prerequisite graph is acyclic:

```text
MRS-001 + PGM-01 + NFR-003 + FR-006
  -> FR-018 executable-path classification
  -> FR-014 candidate/configuration binding
  -> FR-015 stage and lifecycle preservation
  -> FR-017 review and human-decision relation
  -> FR-016 deferred integrator package
```

NFR-004 and NFR-005 are cross-cutting controls designed with FR-018 and gate
completion of every functional requirement they constrain. AP-002 selects the
review boundary; MP-002 operationalizes the NFR observations and does not
precede or redefine either NFR. Specification/review can finish while an
implementation node remains unavailable at an external resume condition.

## Dependencies and Resume Conditions

- Specification and composite review may proceed now.
- Actual v0.1 source-release evidence remains provisional until the exact M0
  candidate, required independent reviews, human Task-007 decision and signed
  tag/checksum exist in the order governed by `quire-contract-ir#4`/PGM-01.
- Authoritative source-path/scope sealing remains blocked on `tl-syntax#16`:
  resume only after an immutable, reviewed and human-accepted Engineering
  Assurance release names the Quire 0.32 source-grounded export and exact
  compatible artifacts.
- Use-specific qualified records remain blocked on
  `engineering-assurance#11`; reusable bounded Rust producer execution remains
  owned by `engineering-assurance#34` when required.
- Executable-language disposition remains owned by `quire-research#64`.
- Quire status-classification and Quoin binary-attachment/non-release-build
  gaps remain conditional blockers for claims that depend on those capabilities.
- The packaged Engineering Assurance matrix is bound by its exact immutable
  package/version and package-integrity mechanism, not by a self-digest. Every
  externally consumed artifact still requires its own declared digest; until
  this distinction is exercised, a complete shared-artifact claim is unavailable.
- Existing documentation disagrees about whether Quoin evidence survives only
  in ignored `target/` state or in a repository evidence area. Durable-retention
  claims remain unavailable until the selected released Quoin backend, handle,
  retrieval and lifecycle rule are named and exercised; prose or a digest is
  not a substitute.
- Historical plans and reviews remain records of their original campaigns, not
  current M6 evidence or authority. PLAN-007 will carry the current dependency
  and resume-state addendum without rewriting those archival claims.
- Hosted CI remains manual-only and is evidence only when a human dispatches and
  identifies its exact run; this specification dispatches nothing.

## References

- [M6 parent](https://github.com/agent-ix/tl-syntax/issues/31) and
  [specification ticket](https://github.com/agent-ix/tl-syntax/issues/34).
- [Cross-repository source-release campaign](https://github.com/agent-ix/quire-contract-ir/issues/4).
- [Source-grounding consumer gate](https://github.com/agent-ix/tl-syntax/issues/16).
- [Use-specific qualification](https://github.com/agent-ix/engineering-assurance/issues/11)
  and [Rust producer execution](https://github.com/agent-ix/engineering-assurance/issues/34).
- [Executable-language boundary](https://github.com/agent-ix/quire-research/issues/64).
