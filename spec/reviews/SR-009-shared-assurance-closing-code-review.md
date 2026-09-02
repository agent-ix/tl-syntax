---
id: SR-009
title: Closing code review — shared assurance migration
type: SpecReview
analysis: code-review
scope: "PR #10; SR-007 findings FND-701..FND-706; the independent adversarial review; exact-head gates"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: reviews
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# SR-009: Closing code review — shared assurance migration

## Summary

An independent adversarial review was run against `87c2dc6` with a single
instruction: find false greens. It found seventeen, including three highs that
SR-007 had missed entirely, and its most useful finding was that the chain sealed
`result: "passed"` for every proof obligation without ever reading what the
producer wrote. A green chain over a red repository is the exact failure this
migration exists to prevent, and it was present in the change that claimed to
prevent it.

All three highs are fixed. Every fix was then re-probed by mutation, and each of
the five previously-missed gaps now goes red.

## Verdict

**CONDITIONAL.** All six SR-007 findings and all seventeen adversarial findings
are dispositioned below: eleven FIXED, six ACCEPTED with rationale, six DEFERRED
to filed issues.

## What the adversarial review changed about this PR

Three things were wrong in a way that mattered.

**The attestations were fabricated.** `assurance_chain.py` passed the literal
string `"passed"` for all six proofs. Rewriting every producer's output to report
total failure — a failing oracle, a broken MSRV build, a compatibility census
reporting moved bytes — still produced 13/13 scenarios, 6/6 controls, 6/6 probes
and exit 0. The chain now derives each result from the producer's own structured
output: row outcomes for the three domain streams, `matched` for the
compatibility census, cargo's own `build-finished` message for MSRV, and a
populated-document check for the Quire export. `--message-format=json` was added
to the MSRV producer specifically so its verdict is a field rather than a
sentence.

**The producer-isolation test could not fail.** It stubbed `cargo`, `rustup` and
`rustc` on `PATH` and asserted the chain still succeeded — but a PATH trace showed
only `rustc --version` was ever resolved, and the test passed identically when
the shims were removed from `PATH` altogether and when the driver was modified to
literally run `cargo build --release`. It now uses shims that log every
invocation, distinguishes asking a tool its version from asking it to do work,
requires the log to be empty, and runs a **control** that stubs `quoin` and
requires the chain to fail — because an empty log and an unconsulted `PATH` are
otherwise the same observation.

**Absent tools were given invented versions.** `tool_version` returned `"0.0.0"`
on any failure, so a sealed attestation's environment could be entirely
fabricated. It now returns `None`, recorded as `null`, and a proof whose tool
version cannot be observed raises rather than sealing.

## Findings

Residual after this round. Nothing new was found that is not dispositioned above.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-901 | medium | Two compatibility cases — a malformed field type and an unknown status — both return `unreadable` under the real record id, and the upstream reason string is the mapping's only discriminator between them. Asserted as such, and stated in `expectations.json` under `discrimination` | `scripts/legacy_evidence_view.py`, `tests/fixtures/legacy-compat/expectations.json` | wrong-requirement |
| FND-902 | low | The chain consumes what `make assurance-inputs` wrote and cannot itself verify that Make ran the command it printed. Inherent: something must run the producer, and Quoin's contract is that the caller states the result. What is now true is that the caller states what the bytes say | `Makefile`, `scripts/assurance_chain.py` | correct-requirement-no-evidence |

## Dispositions

### SR-007 findings

| ID | Severity | Disposition | Where |
| --- | --- | --- | --- |
| FND-701 adapter outcome map collapsible | high | **FIXED** | `scripts/assurance_chain.py` — the vacuous probe transcribes a derived stream through the adapter instead of hand-building entries. Re-probed: collapsing to all-`pass` gives 2 mismatches, to all-`skip` gives 3. |
| FND-702 unread `source_digest` | medium | **FIXED** | `scripts/legacy_evidence_view.py` — every case and every retained record compares the reported identity to the bytes. The `drop-source-identity` probe now goes red. |
| FND-703 corpus oracle only in Python | medium | **FIXED** | `examples/corpus_conformance.rs` — the real crate classifies accept/reject, and a `decoder_agreement` check requires the staged classification and the end-to-end decode to agree per fixture. |
| FND-704 mapping refuses the retained family | medium | **DEFERRED**, `agent-ix/engineering-assurance#21` | Reported as a refusal, not converted. The census across the campaign (142 envelopes, six repositories) is in the issue. |
| FND-705 altered retained byte undetected | low | **FIXED**, and it was worse than reported | See adversarial finding 5 below. |
| FND-706 `[status-column-matches-nothing]` | low | **DEFERRED**, `agent-ix/quire-contract-ir#21` | The `TestMatrix` archetype asserts `Coverage Status`; the traceability declaration is configured for `Status`. Renaming the column was attempted and fails structural validation, so both cannot be satisfied from here. |

### Adversarial findings

| ID | Severity | Disposition | Where |
| --- | --- | --- | --- |
| 1 attestations hardcoded `passed` | high | **FIXED** | `derive_result()` reads each producer's own structured verdict. Probe: every oracle row set to `fail` → chain exit 1. |
| 2 producer-isolation test vacuous | high | **FIXED** | Logging shims, an emptiness assertion, and a `quoin`-stubbed control. Probe: adding `cargo build --release` to the driver → the test fails. |
| 3 `tool_version` fabricates `"0.0.0"` | high | **FIXED** | Returns `None`; an unobservable tool version raises. Probe: `PATH=/usr/bin:/bin` → exit 2, "the version of quire could not be observed". |
| 4 crash counted as probe detection | medium | **FIXED** | The `except` clause is gone. A probe that crashes is a broken probe, not a detection. |
| 5 `evidence_bytes_moved` cannot fire | medium | **FIXED** | Renamed `evidence_bytes_moved_during_this_run`, which is all it ever measured, and `uncommitted_evidence_changes` now asks Git — the boundary `CONTRIBUTING.md` has always named. Probe: tampering a retained byte → census exit 1. |
| 6 dangling `pairs_with` names | medium | **FIXED** | The two names corrected, and the chain now refuses to emit a report when any control names a scenario that does not exist. Probe: introducing a typo → exit 2. |
| 7 cases separated by upstream free text | medium | **PARTLY FIXED, remainder ACCEPTED** | Eight cases now assert `source_record_id` and `source_schema_version`, which the mapping sets structurally: `tampered-source` and `unreadable-source` are told apart by field, not wording. Two cases — a malformed field type and an unknown status — both return `unreadable` under the real record id, and the reason string is genuinely the mapping's only discriminator. Stated in `expectations.json` under `discrimination` rather than papered over. |
| 8 TC-023 asserts nothing about Quire | medium | **FIXED** | The export must name every requirement in the repository, be a populated object, and have been attested `passed` rather than `not_computed`. Probe: `echo '{}' >` the export → the test fails. |
| 9 `states_demonstrated` uses labels | medium | **FIXED** | Only matched cases count, and a scenario that demonstrates no outcome carries `null` instead of borrowing a label. Two scenarios were relabelled. |
| 10 declared command ≠ executed command | medium | **FIXED** | `PROOF-msrv` declares the `cargo check --locked --all-targets --all-features --message-format=json` that actually runs; `PROOF-legacy-compatibility` names `.venv-assurance/bin/python`. |
| 11 hardcoded tool version `0.1.0` | medium | **FIXED** | Observed per tool identity; the crate's own version is read from `Cargo.toml` rather than written twice. |
| 12 absent-decision scenario asserts wrong object | medium | **FIXED** | Now requires `decision_missing` in the receipt reasons, `checks.review.outcome == "incomplete"`, and a null `decision_event`. |
| 13 SR-007 gate row `git diff … empty` false | low | **FIXED** | `schemas/README.md` is added. The row now scopes to the two frozen schema files, `evidence/`, `corpus/` and `src/`, which are byte-identical. |
| 14 coverage numbers selectively quoted | low | **FIXED** | See SR-010. The headline is `55/62 rows backed (88%)` with `spec/evidence/suites.md 0/7`. |
| 15 catch-all folds into a declared reason | low | **FIXED** | `Rejection::UnknownFormulaError` → `unknown_formula_error`, an identifier no corpus manifest declares. |
| 16 PLAN-002 names a file that does not exist | low | **FIXED** | `tests/shared_assurance.rs`. |
| 17 TC-026 census non-recursive and narrow | low | **FIXED** | Walks recursively over `scripts`, `tests`, `examples`, `src`, `spec`, `.github`, plus `Makefile`, `Cargo.toml` and `requirements-assurance.txt`. The vacuity floor is raised from 5 files to 30. |

### Accepted without change

- **Adversarial 7, remainder.** Two compatibility cases separable only by the
  upstream reason text. Re-deriving a discriminator locally would mean writing
  the mapping this migration removed.
- **Adversarial 4's underlying point about `make` ordering.** The chain consumes
  what `make assurance-inputs` wrote and cannot verify that Make ran what it said.
  This is inherent: something has to run the producer, and Quoin's contract is
  that the caller states the result. What is now true, and was not, is that the
  caller states what the bytes say rather than what it hoped.

## Exact-head gates

Run at the final implementation head, not carried over from SR-007.

| Gate | Result |
| --- | --- |
| `make ci` | exit 0 |
| `make spec` | exit 0 |
| Rust tests | 29 passed, 0 failed, 0 ignored |
| Rust tests at MSRV 1.75 | 29 passed, 0 failed, 0 ignored |
| shared pins | 4/4 compatible, 0 artifact mismatches, 0 mirror references |
| compatibility census | 15/15 cases, 23 envelopes, 1232 files read, 0 bytes moved this run, 0 uncommitted |
| compatibility mutation probes | 5/5 detected, with no exception handling to inflate the count |
| assurance chain | 14 scenarios, 6 controls, 6 adapter probes, all matched |
| attested results | all six proofs `passed`, each read from the producer's own output |
| audited receipt | `incomplete`, reasons `decision_missing` and `unresolved_unknown` |
| Quire coverage | 55/62 rows backed (88%); `spec/test-matrix.md` 25/25 |
| `git diff cb7bedb -- evidence/ corpus/ src/ schemas/tl-syntax-evidence-*.json` | empty |
| hosted CI | not dispatched; latest run on this repository is from 2026-08-31 |

## Mutation probes, closing set

Fifteen. The five that were initially missed are listed with both results,
because a probe table that only shows the final state is a table written after
the fix.

| Probe | First result | Now |
| --- | --- | --- |
| Doctored corpus `expected_error` | detected | detected |
| Doctored declared horizon | detected | detected |
| Wrong consumed-artifact digest | detected | detected |
| `npm.ix` in a requirement | detected | detected |
| Producer input removed | exit 2, names the target | unchanged |
| Adapter protocol check removed | detected | detected |
| Frozen schema deleted | detected | detected |
| Empty-stream refusal removed | detected | detected |
| Adapter outcome map collapsed | **NOT detected** | detected |
| Retained byte altered | **NOT detected** | detected |
| Every producer output set to `fail` | **NOT detected** | detected |
| Driver made to run `cargo build` | **NOT detected** | the isolation test fails |
| Toolchain removed from `PATH` | **fabricated `0.0.0`** | exit 2, refuses to seal |
| Dangling `pairs_with` introduced | **NOT detected** | exit 2 |
| Quire export replaced with `{}` | **NOT detected** | TC-023 fails |

## Reviewer feedback

No GitHub review has been received on PR #10. `mergeStateStatus` is `BLOCKED`
with `reviewDecision: REVIEW_REQUIRED`, which is the CODEOWNERS requirement: the
same account authored the change and cannot approve it. The adversarial review
recorded above was run independently against the exact head and its findings are
dispositioned here rather than discarded.
