---
id: NFR-004
title: Reproduce source-readiness observations without authority drift
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/FR-014
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-015
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-016
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-017
    type: constrains
  - target: ix://agent-ix/tl-syntax/FR-018
    type: constrains
---

# NFR-004: Reproduce source-readiness observations without authority drift

## Statement

For identical candidate bytes, configuration identities, shared-contract
artifacts and deterministic producer inputs, the readiness path shall reproduce
the same source-grounded subject and per-obligation outcome.

## Scope

- Applies to source-readiness preparation, shared-contract intake, review
  handoff and future integrator-package emission.
- Does not require identical wall-clock time, filesystem location, human
  rationale or stochastic campaign result.
- Does not make Make, a hosted workflow, the repository, Quire or Quoin a human
  decision authority.
- The readiness path shall retain environmental differences, non-deterministic
  results and human decisions as separate attributable facts.
- Every producer declaration identifies the deterministic comparison projection
  and every volatile, environmental or stochastic field excluded from equality;
  an undeclared volatile field is a discrepancy, not silently normalized data.

## Rationale

Reproducibility is useful only when the repeated subject and configuration are
the same and every meaningful environmental difference remains visible. A
repeated command name alone cannot establish this identity.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Deterministic obligations changing under identical declared inputs | 0 | 0 | Test (TC-059) |
| Configuration/source/shared-contract mutations reusing an earlier subject | 0 | 0 | Test (TC-060) |
| Missing, failed or stale facts promoted during repetition | 0 | 0 | Test (TC-063) |
| Unclassified executable readiness paths | 0 | 0 | Test (TC-064) |

## Verification

Repeat the real local/shared handoff for one frozen candidate and configuration,
then mutate source, feature, target, toolchain, corpus, shared-release,
environment and limitation axes independently. Compare source-grounded subject
and per-obligation states, not only aggregate exit status. Retain each invalid or
non-deterministic run as its actual state.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-004-AC-1 | Two executions over identical declared deterministic inputs produce the same source-grounded subject identity and per-obligation states. | Test (TC-059) |
| NFR-004-AC-2 | Each one-axis source/configuration/shared-contract mutation creates a distinct subject or a typed refusal and cannot reuse earlier evidence. | Test (TC-060) |
| NFR-004-AC-3 | Environmental differences, stochastic results, exceptions, limitations and human decisions remain separate facts rather than inputs to a fabricated deterministic aggregate. | Test (TC-062, TC-063) |
| NFR-004-AC-4 | Evidence retention and interpretation use released Quoin/Engineering Assurance contracts; no repository-local store, score or decision function exists. | Test (TC-066) |
| NFR-004-AC-5 | Each producer declares its deterministic comparison fields and separately preserves every volatile/environmental/stochastic field; an undeclared or changed field cannot be silently normalized away. | Test (TC-059) |

## Measurement Allocation

[MP-002](../assurance/MP-002-source-readiness-obligation-state.md) defines the
planned complete observation population without granting release authority.
