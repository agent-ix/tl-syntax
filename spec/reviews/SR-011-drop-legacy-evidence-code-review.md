---
id: SR-011
title: Code review — drop the retained legacy evidence tree
type: SpecReview
analysis: code-review
scope: "PR #13 at 2762c62; the deletion of evidence/, schemas/, tests/fixtures/legacy-compat/ and scripts/legacy_evidence_view.py, and every surviving reference to them"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/AA-001
    type: references
---

# SR-011: Code review — drop the retained legacy evidence tree

## Summary

The deletion is irreversible, so the review was run against one question rather
than a checklist: **does anything still need what was removed?** Two things did.
Both were found before the change was committed and both are fixed in it.

The first is the one that mattered. `FR-006-AC-5` requires twelve verification
outcomes to stay distinguishable, and `malformed` was demonstrated by exactly one
case in the whole repository — a PGM-01 fixture in the legacy-compat set whose
`collector` field had the wrong type. Deleting the census would have taken the
twelve-state census to eleven while `TC-025` continued to pass, because the test
merged the census into the chain's own state set and asserted the union. That is
the failure class this repository's own tests exist to catch, and it was one
`git rm` away from shipping.

The second is quieter. `assurance/pins.json` pinned four artifacts by digest and
all four existed only for the retained-evidence reader. Removing them would have
left `artifact_digest_mismatches()` iterating an empty list while `TC-021` still
asserted the result was empty — an assertion that passes because there is nothing
to check.

## Verdict

**PASS.** Six findings: four FIXED in this change, two ACCEPTED with rationale.
No finding is deferred.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1101 | high | `malformed` was demonstrated only by the deleted compatibility census. `TC-025` unions the census's case kinds into the chain's `states_demonstrated`, so removing the census silently drops a state and the twelve-outcome assertion still passes | `tests/shared_assurance.rs`, `scripts/assurance_chain.py` | missing-requirement |
| FND-1102 | medium | Every digest-pinned consumed artifact in `assurance/pins.json` existed for the retained-evidence reader. Deleting them leaves `TC-021`'s `artifact_mismatches` assertion checking an empty iteration | `assurance/pins.json`, `scripts/check_shared_pins.py` | correct-requirement-no-evidence |
| FND-1103 | medium | `TC-026` asserted that no source names the two frozen schemas. With the schemas deleted the assertion has no subject, and the census — the only thing keeping the claim non-vacuous — would have gone with it | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-1104 | low | `AA-001` argued sufficiency from `evidence/ANCHORS` and from `tools.lock` executable identities. `tools.lock` was already deleted by `#9` and the paragraph survived it; `evidence/ANCHORS` goes here | `spec/assurance/AA-001.md` | implementation-bug-despite-evidence |
| FND-1105 | low | `StR-001-VC-2` and `StR-002-VC-2` named `evidence/reviews/2026-08-31-current-inspections.md` as their validation artifact. That file is deleted, and it was an agent-produced inspection in the first place | `spec/requirements/StR-001-embedded-consumers.md`, `spec/requirements/StR-002-temporal-interoperability.md` | correct-requirement-no-evidence |
| FND-1106 | low | `spec/assurance/MP-001.md` and `AP-001.md` still described a collection procedure that writes revision-scoped directories under `evidence/` and emits `quire.derivation-evidence/v1` records. Stale since `#9`; now naming a directory that does not exist | `spec/assurance/MP-001.md`, `spec/assurance/AP-001.md` | implementation-bug-despite-evidence |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1101 | **FIXED** | The demonstrator moves to the surviving intake path. `adapter_probes` gains `refuses-a-malformed-row`, which truncates one row of this repository's own conformance stream mid-object and requires `adapt_conformance` to refuse it as malformed. `TC-025` no longer reads a census and asserts twelve states from the chain alone. Mutation-probed: replacing the `json.JSONDecodeError` raise with `continue` turns the probe `MISMATCH` and the chain exit 1. |
| FND-1102 | **FIXED** | `engineering_assurance/compatibility.py` is pinned at `62829251…1475f654`. It is the one behaviour this repository still consumes — `check_shared_pins.py` delegates every verdict to `load_matrix`, `classify_all` and `accepted`. The digest check keeps a subject; `make pins` re-hashes it on every run. |
| FND-1103 | **FIXED** | `TC-026` now asserts that no live source names any of the five deleted things by name, and that `evidence`, `schemas`, `scripts/legacy_evidence_view.py` and `tests/fixtures/legacy-compat` are absent. Mutation-probed: appending `# probe: legacy_evidence_view` to `scripts/validate_corpus.py` fails the test with the offending path. The census-size floor of 30 files is retained. |
| FND-1104 | **FIXED** | The paragraph is deleted, not weakened. Nothing takes its place as a source of sufficiency: what supports the claim now is the sealed Quoin chain over producer bytes and the human release owner's own reading of the diff, and `AA-001` says exactly that. The branch-retention obligation is kept, restated as what it actually is — a downstream-pin obligation tracked as `#8`. |
| FND-1105 | **FIXED** | Both validation columns now read `Inspection`, and both files state plainly that the discharging record was deleted and that no retained inspection artifact is claimed. No surviving text asserts those criteria were inspected. |
| FND-1106 | **FIXED** | `MP-001` now describes retention as Quoin's, with `make assurance-inputs` as the only producer step. `AP-001` records that the `quire.derivation-evidence/v1` records were themselves deleted and that no claim they still verify survives them. |
| Closed reviews and plans still name the deleted machinery | **ACCEPTED** | `spec/reviews/SR-004`, `SR-006`, `SR-007`, `SR-008`, `SR-009`, `SR-010` and `spec/plans/PLAN-002` describe machinery that really existed when they were written. A record is not edited to stop naming what it examined; that would be the rewriting this change is forbidden to do. `TC-026`'s census excludes exactly those two directories and covers everything live. |
| `spec/evidence/suites.md` is 0/6 backed | **ACCEPTED** | Pre-existing and unchanged in kind. Recorded as `FND-1001` in `SR-010`: a suite row is backed by a recorded run in Quoin's store, and `make assurance-record` is an operator action rather than a gate. The count moves 0/7 → 0/6 only because `SUITE-007` is retired. |

## What was deliberately not done

The Make execution-control guard is **not** re-added. Its absence is recorded,
not closed, by owner decision, and `agent-ix/tl-syntax#11` carries it. The
over-strong claim in `Makefile:10-13` is corrected to state what the structural
replacement actually covers — the five charted producers `assurance_chain.py`
lists in `INPUTS` — and to say that every other `ci` prerequisite has no record
to contradict and stays green under `.IGNORE:`.

`#11` lists four places its siblings corrected. In this repository the claim
lived in two, `Makefile:10-13` and its mirror in `CLAUDE.md`; both are corrected
and `NFR-002`, `AA-001` and `assurance/change-assurance.json` do not carry it
here. `#11` item 1 — measuring `.IGNORE:` against this repository's own thirteen
`ci` prerequisites — is **not** done in this change and `#11` stays open for it.

`agent-ix/tl-syntax#8` is untouched, and the branches `feat/tl-syntax-v0.1` and
`retain/issue-9-shared-assurance-migration` are not deleted.
`UNKNOWN-downstream-pin-reachability` survives unchanged in the change
declaration: `tl-parse`, `tl-rewrite` and `tl-mltl` still pin `740182f13b84`,
and that fact is about downstream pins rather than about retained evidence, so
deleting the evidence does not discharge it.

## Gates at the reviewed head

| Gate | Result |
| --- | --- |
| `make ci` | exit 0 |
| `quire coverage --scope . --json` | 0 unbacked rows, 53/59 backed, `status_lies` empty |
| `cargo test --all-features` | 28 passed, 0 failed, 0 ignored |
| Twelve-state census | all twelve demonstrated; 14 scenarios, 6 controls, 7 adapter probes, all matched |
| Mutation probe: weaken the malformed refusal | `refuses-a-malformed-row` MISMATCH, chain exit 1 |
| Mutation probe: reintroduce a `legacy_evidence_view` reference | `TC-026` FAILED, naming the file |
