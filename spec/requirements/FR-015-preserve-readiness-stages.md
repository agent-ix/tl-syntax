---
id: FR-015
title: Preserve developer, source-release and integrator stages
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-014
    type: depends_on
---

# FR-015: Preserve developer, source-release and integrator stages

## Description

When a verification, review or decision fact is consumed, the tl-syntax Rust
readiness projection shall preserve whether it is a developer observation,
source-release input, human source-release decision or integrator-adoption
input.

## Inputs

- Candidate/configuration identity from [FR-014](./FR-014-bind-source-readiness-candidate.md).
- Structured domain results, Quire static exports, Quoin records/attestations/
  receipts, independent review facts, human decision events, assumptions,
  exceptions, counterevidence and limitations.
- For every retained fact: the shared retention authority/backend, immutable
  record or content handle, retention start/rule, retrieval method,
  supersession link, expiry/deletion rule and current availability.

## Outputs

- Independently classified facts retaining their producer, subject,
  configuration, stage, outcome, authority and limitations.

## Behavior

- The tl-syntax Rust readiness projection shall keep every MRS-004
  `ReadinessState` value distinct.
- The tl-syntax Rust readiness projection shall not promote a local or hosted command result
  into an independent review, Quoin attestation, human decision or integrator
  validation.
- The tl-syntax Rust readiness projection shall not treat a Quoin receipt or complete source-
  release evidence set as authority to approve or publish a release.
- The tl-syntax Rust readiness projection shall retain every declared assumption, exception,
  known anomaly, failed/missing obligation and evidence limitation alongside
  any successful fact it qualifies.
- The tl-syntax Rust readiness projection shall preserve immutable earlier facts
  when a later result supersedes, invalidates or narrows them.
- The tl-syntax Rust readiness projection shall link every superseding,
  invalidating or narrowing fact to the exact prior identity.
- Supersession/invalidation links shall target existing facts for the same
  subject lineage and remain acyclic.
- The tl-syntax Rust readiness projection shall refuse a dangling,
  cross-subject or cyclic supersession link.
- The tl-syntax Rust readiness projection shall produce `conflict` for forked
  or contradictory successors until an attributed successor explicitly
  supersedes the complete conflicting set.
- The tl-syntax Rust readiness projection shall preserve an unknown stage,
  outcome, authority or lifecycle enum value as raw input and produce
  `unsupported`.
- The tl-syntax Rust readiness projection shall produce `refused` for a
  malformed encoding of a known value.
- The tl-syntax Rust readiness projection shall distinguish a transient workspace result from
  evidence retained by an identified shared authority.
- The tl-syntax Rust readiness projection shall assign exactly one MRS-004
  `RetentionState` from current backend/handle/lifecycle/retrieval facts.
- The tl-syntax Rust readiness projection shall not infer provenance truth,
  actor identity, non-repudiation, semantic correctness or evidence sufficiency
  from a digest.
- Supersession traversal shall be iterative or maintain a visited set and obey
  the AP-002 node, edge and depth bounds.
- If a supersession bound is exceeded, then the tl-syntax Rust readiness
  projection shall produce `unsupported` without selecting a partial current
  fact.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-015-AC-1 | Developer observations, source-release inputs, human decisions and integrator-adoption inputs remain distinguishable without diagnostic prose. | Test (TC-062) |
| FR-015-AC-2 | Replacing any non-success outcome, missing review, absent decision, limitation or exception with success is detected and refused. | Test (TC-063) |
| FR-015-AC-3 | A later superseding/invalidation fact retains the earlier bytes and identity and names the exact changed premise; it cannot silently restamp the earlier fact. | Test (TC-063) |
| FR-015-AC-4 | No automated result, receipt, score or complete-evidence classification creates release, publication, certification or integrator-acceptance authority. | Test (TC-066) |
| FR-015-AC-5 | Every evidence fact names its retention authority/backend/operator, handle, lifecycle and retrieval bindings and maps to exactly one MRS-004 `RetentionState`; workspace deletion, unavailable handles and every retrieval failure follow the total mapping and cannot report `retained-current`. | Test (TC-067) |
| FR-015-AC-6 | Supersession/invalidation traversal terminates over an acyclic same-subject graph below/at AP-002's node/edge/depth bounds; over-bound is `unsupported`, dangling/cross-subject/cyclic relations are refused, and forked/contradictory successors produce `conflict` without selecting a favorable current fact. | Test (TC-070) |
| FR-015-AC-7 | Every unknown stage, outcome, authority or lifecycle enum value remains attributable raw input and produces `unsupported`; malformed known values produce `refused`, with neither coerced to success or another favorable state. | Test (TC-063) |

## Dependencies

Quoin owns the shared evidence-recording, retention and orchestration contract
but neither operates an unspecified backend nor decides evidence sufficiency.
The selected external backend operator/custodian owns stored bytes; tl-syntax
only consumes and verifies handles. Engineering Assurance owns shared
compatibility and later use-specific qualification contracts. The human source-release owner retains the decision in
[FR-017](./FR-017-require-human-source-release-decision.md).
