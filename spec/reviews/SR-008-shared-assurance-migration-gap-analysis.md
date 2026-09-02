---
id: SR-008
title: Shared assurance migration gap analysis
type: SpecReview
analysis: gap-analysis
scope: "PLAN-002 completeness, FR-006 coverage, the twelve-outcome census, and code with no owning requirement, at 38a410a"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: reviews
  - target: ix://agent-ix/tl-syntax/TM-001
    type: references
---

# SR-008: Shared assurance migration gap analysis

## Summary

Every PLAN-002 task is done and backed. The matrix is 25/25 rows backed by real
tracking tags, and `quire coverage --strict` exits 0 with 28 of 28 evidence
symbols bound. Four findings, none blocking: two are upstream defects already
filed, one is a scope statement the programme's wording does not anticipate, and
one is a pre-existing diagnostic this repository cannot close on its own.

## Verdict

**CONDITIONAL.** Four findings: two DEFERRED to filed issues, two ACCEPTED with
rationale. Dispositioned in SR-010.

## PLAN-002 completeness

| Task | Status | Backed by |
| --- | --- | --- |
| Task-001 Inventory and pins | done | The inventory table in `plan.md`; `make pins` 4/4 compatible; TC-021 |
| Task-002 Structured producers and shared intake | done | Three producers emitting declared structured results; `make assurance` green; TC-022, TC-023 |
| Task-003 Dual run and deletion | done | The dual-run table in `plan.md`; deletion in its own commit `38a410a`; TC-026 |

## FR-006 coverage

| Criterion | Test | Backed |
| --- | --- | --- |
| FR-006-AC-1 | TC-021 `every_shared_pin_is_classified_by_the_packaged_matrix` | yes |
| FR-006-AC-2 | TC-022 `the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer`, `the_chain_completes_with_every_producer_removed_from_the_path` | yes |
| FR-006-AC-3 | TC-023 `the_sealed_records_impact_snapshot_is_the_quire_export` | yes |
| FR-006-AC-4 | TC-024 `retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte` | yes |
| FR-006-AC-5 | TC-025 `all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` | yes |
| FR-006-AC-6 | TC-026 `no_local_evidence_framework_remains_and_the_frozen_schemas_are_referenced_by_nothing` | yes |

## The twelve outcomes: what demonstrated each

| Outcome | Demonstrated by | Observed |
| --- | --- | --- |
| pass | chain `retain-producer-output`; retained bytes read back identical | intake accepted, bytes byte-equal |
| fail | chain `attested-failed`, over a stream derived from the real run by one edit | proof row reason `result_failed` |
| unavailable | chain `attested-unavailable` | proof row reason `result_unavailable` |
| unsupported | chain `non-success-states-stay-distinguishable`; adapter probe `refuses-a-foreign-protocol` | the three non-success reason sets are pairwise distinct; a foreign protocol is refused |
| inconclusive | chain `declared-unknowns-are-carried-not-dropped`; compatibility case `derived-inconclusive` | four open unknowns carried into the receipt with reason `unresolved_unknown` |
| not-computed | chain `attested-not_computed` and `audited-clean-versus-unaudited`; adapter probe `adapter-preserves-non-success-outcomes` | reason `result_not_computed`; `audit_not_evaluated` present unaudited and absent audited |
| malformed | compatibility case `derived-malformed-collector` | `unreadable`, reason `/collector must be an object` |
| partial | chain `receipt-reports-the-absent-human-decision`, `unattested-proof-stays-missing` | receipt `incomplete`, reasons `decision_missing` and `attestation_missing` |
| stale | chain `stale-candidate-binding`; compatibility case `derived-stale-disposition` | reason `candidate_revision_mismatch`; a retracted v2 record maps to state `stale` |
| suspect | adapter probe `audit-reports-a-suspect-link` | `quoin evidence audit` reports `suspect-link` after a statement is reworded |
| vacuous | adapter probes `audit-reports-a-vacuous-run`, `refuses-an-empty-stream` | `vacuous-evidence` finding; the adapter refuses an empty stream |
| tampered | chain `refuse-an-edited-receipt`, `retained-bytes-changed-after-sealing`; compatibility cases `derived-tampered`, `retained-envelope-bound-to-a-foreign-identity` | receipt refused exit 2; intake refused with a digest mismatch |

Every negative names a positive control that was seen to be accepted:
`intake-accepts-unchanged-bytes`, `verify-accepts-an-unedited-receipt`,
`receipt-discharges-a-current-binding`,
`an-audited-passing-proof-is-valid-and-reasonless`,
`passing-proof-is-not-reported-as-failing`,
`an-audit-that-was-run-clears-not-computed`, plus the two released PGM-01
fixtures the compatibility view accepts as `lossy`.

## Underspecified code

None. Every file added by this change is owned by an FR-006 acceptance criterion
and cited by a test the Quire census binds:

- `scripts/check_shared_pins.py` — FR-006-AC-1, TC-021
- `scripts/assurance_chain.py` — FR-006-AC-2 and AC-5, TC-022 and TC-025
- `scripts/legacy_evidence_view.py` — FR-006-AC-4, TC-024
- `examples/corpus_conformance.rs` — FR-006-AC-2 via `PROOF-corpus-conformance`, and FR-005-AC-1/AC-2 through the rows it emits
- `assurance/change-assurance.json`, `assurance/pins.json` — FR-006-AC-1 and AC-3
- `tests/fixtures/legacy-compat/` — FR-006-AC-4 and AC-5, TC-024 and TC-025

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-801 | medium | The pinned release records `accepted.state = pending_human_acceptance` and ships no `human_acceptance_recorded` predicate; the acceptance exists only on `main`. The gate reports the state and does not read an absent field as approval in either direction | `agent-ix/engineering-assurance#20` | wrong-requirement |
| FND-802 | medium | The shared mapping has no reader for `quire.derivation-evidence/v1`, so this repository's entire retained family reads `incompatible`. The programme's wording assumes every migrating repository retained PGM-01 records. A census found 142 `derivation-evidence` envelopes across six of the eight campaign repositories and PGM-01 records in only `quire-contract-ir`, so five later waves will meet this too | `agent-ix/engineering-assurance#21` | missing-requirement |
| FND-803 | low | A verification receipt for this change is `incomplete`, and correctly so: no ix-flow decision event exists. Only the repository owner may create one, and synthesising one would forge the single field in the chain that exists to say a person looked | `assurance/change-assurance.json` unknown `UNKNOWN-human-decision-absent` | correct-requirement-no-evidence |
| FND-804 | low | `740182f13b84`, pinned by tl-parse, tl-rewrite and tl-mltl, is reachable only from this branch. The approved landing form makes it an ancestor of `main`, but the three repins are not part of this change | `agent-ix/tl-syntax#8` | correct-requirement-no-evidence |

## What this analysis does not claim

It does not claim the shared contracts are correct, only that this repository now
uses them and states what it observed. It does not claim the compatibility answer
for retained evidence is satisfactory — it is a refusal, it is reported as one,
and FND-802 is the request to make it better upstream rather than locally.
