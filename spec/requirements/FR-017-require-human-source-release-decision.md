---
id: FR-017
title: Require an exact human source-release decision
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-015
    type: depends_on
---

# FR-017: Require an exact human source-release decision

## Description

When a source-release disposition is requested, the tl-syntax Rust admission
adapter shall
require an attributed human release-owner decision for the exact candidate and
configuration after the required independent reviews and limitation inspection.

## Inputs

- Exact candidate/configuration identity.
- Required specification, code and gap-review outcomes, each binding reviewer
  identity/independence, exact head and base, reviewed configuration/evidence
  set, finding dispositions and accepted limitations.
- Current evidence/counterevidence, assumptions, exceptions, limitations and
  unresolved challenges.
- The authoritative release-owner/reviewer-independence policy identity and
  version, authorized actors/quorum, candidate author/contributor set, conflict
  disclosures and current delegation/revocation facts.
- Zero or more authenticated decision events for the exact subject lineage from
  the authoritative event source, including every current concurrent successor.

## Outputs

- A `DecisionDisposition` from MRS-004 for the exact subject plus zero or more
  per-event `DecisionEventAdmission` values and event records.
- An event-backed accepted/rejected/deferred/conditional disposition retains
  exact event identity and rationale; `open` requires no absent event fields,
  and `conflict` retains the complete conflicting event set.

## Behavior

- While a required review or human decision is absent, the tl-syntax Rust
  admission adapter shall keep the disposition `open`.
- The tl-syntax Rust admission adapter shall aggregate the complete event set
  for the exact subject lineage through the AP-002 node, edge and depth bounds.
- If an event is stale or bound to the wrong actor/subject, then the tl-syntax
  Rust admission adapter shall set its `DecisionEventAdmission` to `refused`.
- If current events are ambiguous or contradictory, then the tl-syntax Rust
  admission adapter shall produce `conflict`.
- The tl-syntax Rust admission adapter shall preserve a human rejection, conditional
  acceptance, exception and expiry without translating any of them into an
  unconditional pass.
- A conditional disposition shall bind each condition to its owner, required
  evidence and expiry.
- While any condition is unresolved and unexpired, the tl-syntax Rust admission
  adapter shall keep the disposition `conditional`.
- If a condition is satisfied, then the tl-syntax Rust admission adapter shall
  preserve `conditional` until a new authorized decision is recorded.
- If a condition expires, then the tl-syntax Rust admission adapter shall mark
  the prior conditional decision stale and set the current disposition `open`.
- The tl-syntax Rust admission adapter shall require a new decision when the candidate,
  configuration, applicable profile, material limitation, required review or
  shared-contract identity changes.
- The tl-syntax Rust admission adapter shall not synthesize, infer or replay a human decision
  from a green gate, Quoin receipt, pull-request merge or prior release.
- The tl-syntax Rust admission adapter shall not count an author's self-review as an
  independent review.
- The tl-syntax Rust admission adapter shall invalidate a review relation when
  its exact head, base, configuration, evidence set or accepted limitation changes.
- The tl-syntax Rust admission adapter shall verify reviewer independence and decision
  authority against the bound policy/version, actor, author/contributor set,
  conflicts, delegations, revocations and quorum instead of accepting a
  self-asserted Boolean independence field.
- The tl-syntax Rust admission adapter shall require every accepted limitation
  or exception to be attributable to the authorized human release owner for the
  exact decision subject.
- The tl-syntax Rust admission adapter shall preserve every accepted
  limitation's or exception's rationale, affected obligations, counterevidence
  and expiry.
- If review/decision successors are concurrent, forked or contradictory, then
  the tl-syntax Rust admission adapter shall produce `conflict` until an
  attributed authorized resolution explicitly supersedes every conflicting
  predecessor in the same subject lineage.
- A deferred decision shall bind its owner, rationale, resume condition and
  expiry.
- A deferred decision shall remain `deferred` until a new authorized decision
  resolves it.
- If a current accepted, rejected, deferred or conditional decision, exception
  or policy binding expires, then the tl-syntax Rust admission adapter shall
  preserve it as stale history and set the current disposition `open`.
- A source-release decision shall not decide crates.io publication or an
  integrator's use-specific qualification.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-017-AC-1 | Accepted/rejected/deferred/conditional/open/conflict remain distinct exact-subject dispositions; event-backed states retain identity/rationale/exception/expiry, `open` requires no absent event fields, and `conflict` retains the complete event set. | Test (TC-062) |
| FR-017-AC-2 | An absent required review/decision leaves the disposition `open`; a stale or wrong-actor/subject event is `refused` while the disposition stays `open`; an ambiguous/contradictory current set produces `conflict`, and none is inferred as accepted. | Test (TC-063) |
| FR-017-AC-3 | Every material candidate/configuration/profile/limitation/shared-contract change requires a new review/decision relation and preserves the historical disposition. | Test (TC-063) |
| FR-017-AC-4 | A source-release acceptance grants neither publication nor integrator/system/monitor qualification. | Test (TC-066) |
| FR-017-AC-5 | Each required review binds an attributable independent reviewer, exact head/base/configuration/evidence set, finding dispositions and accepted limitations; self-review or any bound-identity change leaves the review requirement open. | Test (TC-073) |
| FR-017-AC-6 | Independence and authority are evaluated against an exact policy/version, author/contributor set, conflicts, delegations, revocations and quorum; an unverifiable event is `refused`, and an insufficient quorum leaves the disposition `open`. | Test (TC-073) |
| FR-017-AC-7 | Conditional decisions retain condition owner/evidence/expiry; unresolved and satisfied conditions stay `conditional` until a new authorized decision, expiry reopens the disposition, deferred decisions retain owner/resume/expiry until a new authorized decision, and concurrent/contradictory successors produce `conflict` until explicitly resolved. | Test (TC-070) |

## Dependencies

- [AP-002](../assurance/AP-002-progressive-source-readiness.md) selects the
  required review operations and decision boundary.
- `agent-ix/engineering-assurance#11` owns later use-specific producer
  qualification and independence, not this source-release decision.
- PGM-01 and the repository's exact human-release-owner policy own source-
  release authority; GitHub supplies authenticated review/event facts but does
  not define independence or quorum. Until the exact immutable policy/event
  source and authorized actor set are selected, decision admission remains
  unavailable and tl-syntax creates no local reviewer or approval registry.
