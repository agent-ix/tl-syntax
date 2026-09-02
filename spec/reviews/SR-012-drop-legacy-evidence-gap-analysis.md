---
id: SR-012
title: Gap analysis — drop the retained legacy evidence tree
type: SpecReview
analysis: gap-analysis
scope: "PR #13; FR-006 coverage before and after the deletion, the TM-001 matrix, the suite registry, the twelve-outcome census measured at 953ee82, and code with no owning requirement"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/TM-001
    type: reviews
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# SR-012: Gap analysis — drop the retained legacy evidence tree

## Summary

The claim is not "coverage is clean" — it is **no row lost its backing as a side
effect of the deletion**. That is measured rather than asserted: `quire coverage
--scope . --json` was run against the pre-deletion tree extracted from `953ee82`
and against this head, and the arithmetic closes exactly.

Every row that disappeared is one this change deliberately removed. No surviving
row lost backing. `unbacked_rows` is empty in both runs and `status_lies` is
empty in both.

## Coverage, before and after

| Document | Target | Before (`953ee82`) | After |
| --- | --- | --- | --- |
| `spec/evidence/suites.md` | suite | 0/7 | **0/6** |
| `spec/requirements/FR-001-inclusive-intervals.md` | acceptance-criterion | 2/2 | 2/2 |
| `spec/requirements/FR-002-validated-formula.md` | acceptance-criterion | 3/3 | 3/3 |
| `spec/requirements/FR-003-identities-and-profiles.md` | acceptance-criterion | 3/3 | 3/3 |
| `spec/requirements/FR-004-versioned-serialization.md` | acceptance-criterion | 4/4 | 4/4 |
| `spec/requirements/FR-005-conformance-corpus.md` | acceptance-criterion | 3/3 | 3/3 |
| `spec/requirements/FR-006-shared-assurance-intake.md` | acceptance-criterion | 6/6 | **5/5** |
| `spec/requirements/NFR-001-no-std-feature-boundary.md` | nfr-acceptance-criterion | 2/2 | 2/2 |
| `spec/requirements/NFR-002-determinism-and-integrity.md` | nfr-acceptance-criterion | 3/3 | 3/3 |
| `spec/requirements/StR-001-embedded-consumers.md` | stakeholder-validation-criterion | 2/2 | 2/2 |
| `spec/requirements/StR-002-temporal-interoperability.md` | stakeholder-validation-criterion | 2/2 | 2/2 |
| `spec/test-matrix.md` | test-case | 25/25 | **24/24** |
| **Total** | | **55/62** | **53/59** |
| `unbacked_rows` | | **0** | **0** |
| `status_lies` | | **empty** | **empty** |

Three rows changed and only three:

| Row removed | Was it backed? | Effect on the totals |
| --- | --- | --- |
| `FR-006-AC-4` | backed | total −1, backed −1, criteria −1 |
| `TC-024` | backed | total −1, backed −1 |
| `SUITE-007` | unbacked | total −1, backed −0 |

`62 − 3 = 59`. `55 − 2 = 53`. Nine of the twelve documents are byte-for-byte
identical in their coverage figures, including both stakeholder documents, whose
`VC-2` rows were re-pointed from a deleted inspection record to the tests that
already carried their trace tags — a correction of a stale pointer, not a change
of backing.

The six unbacked-by-recorded-run rows are the suite registry, unchanged in kind
from `SR-010`'s `FND-1001`; the count moves 0/7 → 0/6 only because `SUITE-007`
is retired. `make assurance-record` writes into `spec/evidence/`, which is a
reviewed repository change rather than something a gate does on every run, so it
was deliberately not run.

`quire validate --scope . 'spec/**/*.md' --strict --summary` exits 0 with 44/44
documents grammar-clean.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1201 | medium | Dropping `FR-006-AC-4` without its matrix row would have left `TC-024` bound to nothing and turned a clean `quire coverage --strict` red. The criterion, its row in the FR-006 line, its test-case row, its test, and `SUITE-007` are removed together | `spec/test-matrix.md`, `spec/requirements/FR-006-shared-assurance-intake.md` | missing-requirement |
| FND-1202 | low | `FR-006-AC-4`, `TC-024` and `SUITE-007` are named by `SR-008`, a closed review. Reusing any of those identifiers would make both `SR-008` and the current specification unreadable | `spec/reviews/SR-008-shared-assurance-migration-gap-analysis.md` | wrong-requirement |
| FND-1203 | low | `spec/evidence/suites.md` remains 0-backed. Pre-existing, unchanged in kind | `spec/evidence/suites.md` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1201 | **FIXED** | Removed as a set. `quire coverage --scope . --json` reports 0 unbacked rows and `quire coverage --scope . --strict` exits 0 at the reviewed head. |
| FND-1202 | **FIXED** | `FR-006` gains a **Retired criteria** section recording `FR-006-AC-4`, `TC-024` and `SUITE-007` as retired and their identifiers as not reused, on the terms the repository already applies to `NFR-002-AC-4`. `spec/evidence/suites.md` carries the matching note for `SUITE-007`. |
| FND-1203 | **ACCEPTED** | Same rationale as `SR-010` `FND-1001`. Writing into `spec/evidence/` is a reviewed repository change, not a gate action. |

## The twelve-outcome census, measured before the deletion

`FR-006-AC-5`'s twelve outcomes were measured at the pre-deletion head before
anything was removed, by running the old chain and the old compatibility census
in a tree extracted from `953ee82`:

| Source at `953ee82` | Outcomes it demonstrated |
| --- | --- |
| the assurance chain alone | eleven: pass, fail, unavailable, unsupported, inconclusive, not-computed, partial, stale, suspect, vacuous, tampered |
| the compatibility census | error, failed, inconclusive, **malformed**, not-computed, positive-control, stale, tampered, unavailable, unreadable, unsupported |
| **missing from the chain alone** | **`malformed`, and only `malformed`** |

So exactly one of the twelve rested solely on the census. At this head the chain
alone demonstrates all twelve, `malformed` included, and the mutation run shows
the check that produces it can be made to fail.

## Code with no owning requirement

Checked. The change adds three things that are not deletions:

| Addition | Owning requirement | Traced through |
| --- | --- | --- |
| `transcribe()` with four named refusal switches, and `--mutation-probes` over them | `FR-006-AC-5` | `TC-025`, which runs the mutation gate and requires exit 0 |
| the four shared adapter-refusal checks, including `refuses-a-malformed-row` | `FR-006-AC-5` | `TC-025`, which asserts `malformed` among the twelve |
| the `compatibility.py` digest pin | `FR-006-AC-1` | `TC-021`, which asserts `artifact_mismatches` is empty |

None is new capability. The first two relocate a demonstrator and a mutation
gate whose previous owners were deleted; the third replaces four digest pins
that had no reader left, so that the assertion over them keeps a subject.

Nothing else is added. Every remaining hunk removes a file, a declaration, a
prerequisite, an obligation, a criterion, a row, or a claim.

## Requirement statements that changed

`FR-006`'s Behavior section loses the three `legacy_evidence_view` statements
and the two about reporting a refused retained schema family. It gains two, both
of which are what the repository now actually does:

- tl-syntax shall retain no evidence of its own.
- tl-syntax shall implement no compatibility mapping of its own.

Both are single-`shall` and EARS-clean. `FR-006`'s Inputs no longer name
`evidence/` and its Outputs no longer name a compatibility view.

`FR-006-AC-6` changes from asserting that the frozen schemas are referenced by
nothing — a claim with no subject once they are deleted — to asserting that no
live source, gate, workflow or specification row still names the deleted
retained-evidence machinery. `TC-026` implements exactly that and goes red when
a reference is reintroduced.

## Exit criteria

| Criterion | Met |
| --- | --- |
| Nothing live still requires the deleted material | Yes — `TC-026` census over `scripts`, `tests`, `examples`, `src`, `spec` (less closed records), `.github`, `Makefile`, `Cargo.toml`, `requirements-assurance.txt`, `README.md`; mutation-probed |
| No orphan matrix row | Yes — 0 unbacked rows, `status_lies` empty |
| Twelve outcomes still distinguishable | Yes — measured at `953ee82` before deleting anything (chain alone gave eleven, the census supplied only `malformed`), and all twelve now come from the chain alone; all four adapter refusals mutation-probed red |
| No row unbacked as a side effect | Yes — before/after `quire coverage` compared per document; three rows removed, all deliberately, no surviving row lost backing |
| No record rewritten, backdated, or re-sealed | Yes — every retained record is deleted; no file under `evidence/` was modified before removal, and no surviving claim asserts historical evidence still verifies |
| Retired identifiers not reused | Yes — recorded in `FR-006` and `spec/evidence/suites.md` |
| Hosted CI not dispatched | Yes — `workflow_dispatch` only, unchanged and undispatched |
| Nothing published, tagged, or released | Yes |
