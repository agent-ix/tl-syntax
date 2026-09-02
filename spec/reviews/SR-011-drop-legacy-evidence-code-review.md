---
id: SR-011
title: Code review — drop the retained legacy evidence tree
type: SpecReview
analysis: code-review
scope: "PR #13; the deletion of evidence/, schemas/, tests/fixtures/legacy-compat/ and scripts/legacy_evidence_view.py, every surviving reference to them, and an independent adversarial review of the same diff"
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

An independent adversarial review was then run against the committed diff with
one instruction: find anything that still needs what was removed. It found eight
more, including a high the first pass missed — `AA-001`'s *Sufficiency Decision*
still gated the top claim on retained evidence eleven lines below a paragraph
this change had just rewritten to say the argument makes no appeal to it. The
result was a release gate that could never close.

## Verdict

**PASS.** Fourteen findings: twelve FIXED in this change, two ACCEPTED with
rationale. No finding is deferred.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1101 | high | `malformed` was demonstrated only by the deleted compatibility census. `TC-025` unions the census's case kinds into the chain's `states_demonstrated`, so removing the census silently drops a state and the twelve-outcome assertion still passes | `tests/shared_assurance.rs`, `scripts/assurance_chain.py` | missing-requirement |
| FND-1102 | medium | Every digest-pinned consumed artifact in `assurance/pins.json` existed for the retained-evidence reader. Deleting them leaves `TC-021`'s `artifact_mismatches` assertion checking an empty iteration | `assurance/pins.json`, `scripts/check_shared_pins.py` | correct-requirement-no-evidence |
| FND-1103 | medium | `TC-026` asserted that no source names the two frozen schemas. With the schemas deleted the assertion has no subject, and the census — the only thing keeping the claim non-vacuous — would have gone with it | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-1104 | low | `AA-001` argued sufficiency from `evidence/ANCHORS` and from `tools.lock` executable identities. `tools.lock` was already deleted by `#9` and the paragraph survived it; `evidence/ANCHORS` goes here | `spec/assurance/AA-001.md` | implementation-bug-despite-evidence |
| FND-1105 | low | `StR-001-VC-2` and `StR-002-VC-2` named `evidence/reviews/2026-08-31-current-inspections.md` as their validation artifact. That file is deleted, and it was an agent-produced inspection in the first place | `spec/requirements/StR-001-embedded-consumers.md`, `spec/requirements/StR-002-temporal-interoperability.md` | correct-requirement-no-evidence |
| FND-1106 | low | `spec/assurance/MP-001.md` and `AP-001.md` still described a collection procedure that writes revision-scoped directories under `evidence/` and emits `quire.derivation-evidence/v1` records. Stale since `#9`; now naming a directory that does not exist | `spec/assurance/MP-001.md`, `spec/assurance/AP-001.md` | implementation-bug-despite-evidence |

### Found by the independent adversarial review

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1107 | high | `AA-001`'s *Sufficiency Decision* still read "the claim remains open until **the retained evidence** names the candidate revision". The same file's *Reasoning* had just been rewritten to say the argument makes no appeal to retained evidence. One AssuranceArgument, two contradictory statements, and a close condition that can never be satisfied | `spec/assurance/AA-001.md` | implementation-bug-despite-evidence |
| FND-1108 | medium | `spec/spec.md` (`MRS-001`) told reviewers to "use the corpus and retained evidence to check compatibility and determinism". Never touched by the deletion, and now false. `MRS-001` is also the only tl-syntax artifact any sibling repository references | `spec/spec.md` | missing-requirement |
| FND-1109 | medium | `StR-001-VC-2` and `StR-002-VC-2` were rewritten from `Inspection (evidence/reviews/…)` to a bare `Inspection` plus prose saying the criterion is "verified by inspection at review time" — a validation method with no artifact, no date and no reviewer. That is a claim restated more weakly, which this change is forbidden to do, while `TM-001` still marked both rows covered | `spec/requirements/StR-001-embedded-consumers.md`, `spec/requirements/StR-002-temporal-interoperability.md`, `spec/test-matrix.md` | wrong-requirement |
| FND-1110 | medium | The new `malformed` demonstrator asserted a substring against an exception the same file had just raised, with no external oracle. It was load-bearing against deletion but not against weakening: hard-coding the predicate true went undetected. The deleted compatibility gate had run `--mutation-probes` on every CI run; nothing replaced it | `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-1111 | medium | `TC-026`'s census did not read `assurance/`, the directory that held `PROOF-legacy-compatibility` and the deleted digest pins, nor `CLAUDE.md`/`CONTRIBUTING.md`/`AGENTS.md`. The criterion it discharges had just been widened to cover every live "source, gate, workflow, or specification row" | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-1112 | low | `assurance/change-assurance.json` still carried the pre-change wording of `FR-006-AC-6` while the specification's text had been widened. Same identifier, two statements | `assurance/change-assurance.json` | implementation-bug-despite-evidence |
| FND-1113 | low | The census anti-vacuity floor stayed at `> 30` while excluding `spec/reviews` and `spec/plans` shrank the population by about a third, leaving a margin of two | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-1114 | low | `spec/evidence/suites.md` said `SUITE-007`'s identifier "was reused deliberately" six lines above saying it "is not reused". Both true under different tenses; reads as a contradiction | `spec/evidence/suites.md` | implementation-bug-despite-evidence |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1101 | **FIXED** | The demonstrator moves to the surviving intake path. `adapter_probes` gains `refuses-a-malformed-row`, which truncates one row of this repository's own conformance stream mid-object and requires `adapt_conformance` to refuse it as malformed. `TC-025` no longer reads a census and asserts twelve states from the chain alone. Mutation-probed: replacing the `json.JSONDecodeError` raise with `continue` turns the probe `MISMATCH` and the chain exit 1. |
| FND-1102 | **FIXED** | `engineering_assurance/compatibility.py` is pinned at `62829251…1475f654`. It is the one behaviour this repository still consumes — `check_shared_pins.py` delegates every verdict to `load_matrix`, `classify_all` and `accepted`. The digest check keeps a subject; `make pins` re-hashes it on every run. |
| FND-1103 | **FIXED** | `TC-026` now asserts that no live source names any of the five deleted things by name, and that `evidence`, `schemas`, `scripts/legacy_evidence_view.py` and `tests/fixtures/legacy-compat` are absent. Mutation-probed: appending `# probe: legacy_evidence_view` to `scripts/validate_corpus.py` fails the test with the offending path. The census-size floor of 30 files is retained. |
| FND-1104 | **FIXED** | The paragraph is deleted, not weakened. Nothing takes its place as a source of sufficiency: what supports the claim now is the sealed Quoin chain over producer bytes and the human release owner's own reading of the diff, and `AA-001` says exactly that. The branch-retention obligation is kept, restated as what it actually is — a downstream-pin obligation tracked as `#8`. |
| FND-1105 | **FIXED** | Both validation columns now read `Inspection`, and both files state plainly that the discharging record was deleted and that no retained inspection artifact is claimed. No surviving text asserts those criteria were inspected. |
| FND-1106 | **FIXED** | `MP-001` now describes retention as Quoin's, with `make assurance-inputs` as the only producer step. `AP-001` records that the `quire.derivation-evidence/v1` records were themselves deleted and that no claim they still verify survives them. |
| FND-1107 | **FIXED** | The close condition names the sealed Quoin chain: the record's `subject.base_revision` and every attestation's `candidate_revision`, both derived from the revision the caller names and both refused by Quoin when they disagree. Stated as the stricter binding it is, and as available at every candidate revision rather than only the ones somebody collected. Not deferred and not softened. |
| FND-1108 | **FIXED** | `spec/spec.md` now reads "the corpus and the sealed assurance chain". |
| FND-1109 | **FIXED** | The premise was wrong: both criteria were already discharged by tests, and the `evidence/` pointer was a stale second reference rather than the discharge. `src/syntax.rs::allocation_free_core_api_constructs_formula` carries `StR-001-VC-2` in its trace comment; `tests/integration.rs::shared_corpus_is_complete_stable_and_self_consistent` carries `StR-002-VC-2`. The validation methods now read `Test (TC-015)` and `Test (TC-014)`, `TM-001` names those cases on both the stakeholder rows and the test-case rows, and the weaker prose is deleted. `quire coverage` reports both documents 2/2 backed before and after. |
| FND-1110 | **FIXED** | `adapt_conformance` is now a strict call into `transcribe()`, whose four refusals are named keyword switches. `scripts/assurance_chain.py --mutation-probes` turns each off in turn and requires the check guarding it to go red; all four are detected. `make mutation-probes` is a prerequisite of `make assurance` — the slot `compat-view` used to occupy — so it runs on every `make ci`, and `TC-025` asserts its exit status and that all four probes ran. The four adapter probes in the normal run share the same predicates, so a probe passing and a mutation being detected are statements about one piece of code. |
| FND-1111 | **FIXED** | The census now reads `assurance/` and `CLAUDE.md`, `CONTRIBUTING.md`, `AGENTS.md`. |
| FND-1112 | **FIXED** | The obligation statement in `change-assurance.json` is synced to `FR-006-AC-6`. |
| FND-1113 | **FIXED** | Floor raised to `>= 38` against a measured population of 42, with the reason in a comment. |
| FND-1114 | **FIXED** | The note separates the two events: `#9` reused the identifier for a new suite, `#12` retires it and does not reuse it again. |
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

## The schemas, proved dead per file

The two evidence schemas were frozen by `#9` rather than deleted, and a
name-based reading is not evidence. Measured on the pre-deletion tree extracted
from `953ee82`:

| Search | Result |
| --- | --- |
| `include_str!` / `include_bytes!` anywhere in `src/`, `tests/`, `examples/` | one hit: `src/lib.rs:3` includes `../README.md`. No schema is embedded |
| `tl-syntax-evidence-input-v1.schema.json` across `src/ scripts/ tests/ examples/ assurance/ .github/ Makefile Cargo.toml` | 2 hits: `tests/shared_assurance.rs:526` pinning it by digest, and the `PRESERVE-frozen-schemas` prose. Both deleted with it |
| `tl-syntax-evidence-manifest-v1.schema.json`, same scope | 2 hits, the same two |
| `schemas/README.md`, same scope | 0 hits |
| any `schemas/` reference at all | the remainder are the **upstream** release's `schemas/pgm01-compatibility-view-v1.schema.json` in `pins.json`, and `"path": "schemas/pgm01-evidence-v1.schema.json"` inside legacy-compat fixture payloads. Neither names this repository's `schemas/` |

Nothing validated against any of the three at runtime or in a test. All three
are deleted; none is kept.

## Gates at the reviewed head

| Gate | Result |
| --- | --- |
| `make ci` | exit 0 |
| `quire coverage --scope . --json` | 0 unbacked rows, 53/59 backed, `status_lies` empty |
| `cargo test --all-features` | 28 passed, 0 failed, 0 ignored |
| Twelve-state census | all twelve demonstrated by the chain alone; 14 scenarios, 6 controls, 8 adapter probes, all matched |
| `make mutation-probes` | 4/4 adapter refusals switched off, 4/4 detected |
| Baseline: chain-only states at `953ee82` | eleven — `malformed` absent, supplied only by the compatibility census. Measured by running the pre-deletion chain in an extracted tree, not inferred |
| Baseline: pre-deletion census | 1,232 files read, 23 retained records, all `incompatible`, `malformed` among its case kinds |
| Mutation probe: weaken the malformed refusal | `refuses-a-malformed-row` UNDETECTED reported, `--mutation-probes` exit 1 |
| Mutation probe: one-byte edit to pinned `compatibility.py` | `make pins` exit 1 naming the digest mismatch; restored, exit 0 |
| Mutation probe: reintroduce a `legacy_evidence_view` reference | `TC-026` FAILED, naming the file |
| Census population | 42 files inspected against a floor of 38 |
