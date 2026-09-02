---
id: SR-010
title: Closing gap analysis — shared assurance migration
type: SpecReview
analysis: gap-analysis
scope: "PR #10 at its final head; SR-008 findings FND-801..FND-804; PLAN-002 exit criteria; the migration contract's review checklist"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: reviews
  - target: ix://agent-ix/tl-syntax/TM-001
    type: references
---

# SR-010: Closing gap analysis — shared assurance migration

## Summary

Every SR-008 finding is dispositioned and every PLAN-002 exit criterion is met.
The coverage figures SR-008 quoted are corrected here: the headline is 55 of 62
rows backed, not 25 of 25, and the seven unbacked rows are the suite registry,
which this change deliberately does not write into.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1001 | low | `spec/evidence/suites.md` is 0/7 backed. A suite row is backed by a recorded run in Quoin's evidence store, and `make assurance-record` was deliberately not run: writing into `spec/evidence/` is a reviewed repository change, not a gate action | `spec/evidence/suites.md` | correct-requirement-no-evidence |
| FND-1002 | low | SR-008 quoted `25/25` without naming its population. Corrected in this document to `55/62 rows backed (88%)` with the per-document breakdown | `spec/reviews/SR-008-shared-assurance-migration-gap-analysis.md` | implementation-bug-despite-evidence |

## Verdict

**CONDITIONAL.** Four SR-008 findings dispositioned: two DEFERRED to filed
issues, two ACCEPTED. Every PLAN-002 exit criterion is met. No required finding
is outstanding.

## Dispositions

| SR-008 ID | Severity | Disposition | Rationale |
| --- | --- | --- | --- |
| FND-801 v0.2.0 carries no acceptance record | medium | **DEFERRED**, `agent-ix/engineering-assurance#20` | The release records `pending_human_acceptance` and ships no predicate for it. The gate reports the state and gates only on the local toolchain, so an absent field is never read as approval in either direction. Cutting `v0.2.1` is a release action and is not performed from a migrating repository. |
| FND-802 no mapping for `derivation-evidence/v1` | medium | **DEFERRED**, `agent-ix/engineering-assurance#21` | Measured and filed with the census: 142 envelopes across six of the eight campaign repositories, PGM-01 records in `quire-contract-ir` only. The refusal is reported as the compatibility result; writing a local mapping is the thing the migration removes. |
| FND-803 no ix-flow decision event | low | **ACCEPTED** | The receipt reads `incomplete` with `decision_missing`, and that is the correct answer. Only the repository owner can create an ix-flow decision; synthesising one would forge the single field in the chain whose purpose is to record that a person looked. |
| FND-804 downstream pins on a branch-only revision | low | **DEFERRED**, `agent-ix/tl-syntax#8` | A landing-sequence constraint, not a property of this change. Merging with a true merge commit makes `740182f13b84` an ancestor of `main`; the three repins remain that issue's work. |

## Corrected coverage figures

SR-008 quoted `25/25` without saying what it counted. Stated properly:

| Measure | Value |
| --- | --- |
| `quire coverage --scope . --strict` overall | **55/62 rows backed (88%)** |
| `spec/test-matrix.md` | 25/25 (100%) |
| `spec/evidence/suites.md` | **0/7 (0%)** |
| Requirement documents | 27/28 criteria backed |
| Evidence symbols with tracking tags | 28/28 |

The seven unbacked rows are the suite registry. A `SuiteRegistry` row is backed
when a run of that suite has been recorded into Quoin's evidence store, and this
change records none: `make assurance-record` exists as an operator target and was
deliberately not run as part of the migration, because writing into
`spec/evidence/` is a reviewed change to the repository rather than something a
gate does on every invocation. Wave 0 reported the same shape for a different
reason — it had no suite registry at all and bound nothing. This repository does
have one, and a trial record in a scratch tree bound 8 obligations from 22
entries, so the binding works and is simply not exercised in-tree.

`[status-column-matches-nothing]` additionally means the ✅ markers in the
functional and stakeholder tables are never validated. That is
`agent-ix/quire-contract-ir#21` and cannot be closed from here: the `TestMatrix`
archetype asserts a `Coverage Status` header and the traceability declaration is
configured for `Status`. Renaming the column was attempted during this change and
fails structural validation, so the two cannot both be satisfied.

## PLAN-002 exit criteria

| Criterion | Met |
| --- | --- |
| 1. Every pinned component classifies `compatible` | Yes — 4/4, with 0 artifact mismatches and 0 mirror references |
| 2. Twelve outcomes demonstrated, each negative paired with an accepted control | Yes — eleven from the chain and adapter probes, `malformed` from the compatibility view; six controls, none dangling |
| 3. Retained bytes unchanged and read through the pinned mapping | Yes — 23 envelopes, 1232 files, 0 moved this run, 0 uncommitted, `git diff cb7bedb -- evidence/` empty |
| 4. No generic runner, envelope, manifest, identity lock, retention store, audit store, anchor authority, or aggregate verdict remains | Yes — TC-026, over a 30-file-minimum recursive census including `Makefile` and `.github/` |
| 5. Default dependency graph empty; builds without `std` or `alloc` | Yes — 5 structured feature-boundary entries |
| 6. Hosted CI manual-only and undispatched | Yes — `workflow_dispatch` only; latest run 2026-08-31, before this work |

## The migration contract's PR review checklist

`agent-ix/engineering-assurance#10` requires eight things of a migration.

| Requirement | Answer |
| --- | --- |
| Inventory generic machinery separately from domain logic | The keep/replace/delete/defer table in `plan.md` |
| Preserve domain runners, oracles, corpora, schemas, formats, failure behaviour | `git diff cb7bedb -- src/ corpus/` empty; the corpus oracle kept and now cross-checked by the real crate |
| Register static obligations through Quire, dynamic results through Quoin | `quire coverage --json` is the impact snapshot; six proof attestations reach Quoin's intake |
| Replace local envelope, manifest, identity, retention, audit, history, traceability, anchor | 4,059 lines removed; each replacement named in the plan |
| Preserve legacy history through the compatibility view | 23 envelopes read; the refusal reported rather than converted |
| Simplify Makefiles to readable native orchestration | The Makefile is 200 lines with no parse-time guards and no self-attestation |
| Demonstrate pass, fail, unavailable, not-computed, malformed, stale, tampered | All seven, plus unsupported, inconclusive, partial, suspect and vacuous |
| Delete old code only after the shared path passes at the same revision | The dual run is at `cb7bedbe162d`; deletion is a separate later commit |

## Underspecified code

None. Every file this change adds is owned by an FR-006 acceptance criterion and
cited by a test the Quire census binds. The one addition since SR-008,
`derive_result()` in `scripts/assurance_chain.py`, is owned by FR-006-AC-2 and is
exercised by TC-022 and by the mutation probe that sets every producer row to
`fail`.

## What this analysis does not claim

It does not claim the suite registry is discharged; it is 0/7 and the number is
printed rather than omitted. It does not claim hosted CI passes. It does not
claim a human has reviewed this change — no GitHub review has been received, and
the adversarial review recorded in SR-009 was an agent, which is why its findings
are written down in full rather than summarised as "addressed".
