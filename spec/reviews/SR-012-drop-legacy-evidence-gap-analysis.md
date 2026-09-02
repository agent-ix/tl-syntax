---
id: SR-012
title: Gap analysis — drop the retained legacy evidence tree
type: SpecReview
analysis: gap-analysis
scope: "PR #13 at 2762c62; FR-006 coverage after retiring AC-4, the TM-001 matrix, the suite registry, and code with no owning requirement"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/TM-001
    type: reviews
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# SR-012: Gap analysis — drop the retained legacy evidence tree

## Summary

Every matrix row is backed by a real tracking tag and no row is orphaned by the
deletion. `quire coverage --scope . --json` reports **0 unbacked rows** over 59
matrix rows, 53 backed, and `status_lies` empty. The headline moves from
`55/62` in `SR-010` to `53/59` here, and the three-row difference is exactly the
three retired identifiers: `FR-006-AC-4`, `TC-024`, `SUITE-007`.

The six unbacked-by-recorded-run rows are the suite registry, unchanged in kind
from `SR-010`'s `FND-1001`. `make assurance-record` writes into
`spec/evidence/`, which is a reviewed repository change rather than something a
gate does on every run, so it was deliberately not run.

## Coverage

| Document | Backed |
| --- | --- |
| `spec/test-matrix.md` | 24/24 (100%) |
| `spec/requirements/FR-006-shared-assurance-intake.md` | 5/5 (100%) |
| `spec/requirements/FR-001` … `FR-005`, `NFR-001`, `NFR-002`, `StR-001`, `StR-002` | 24/24 (100%) |
| `spec/evidence/suites.md` | 0/6 (0%) |
| **Total** | **53/59 (90%)** |

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

## Code with no owning requirement

Checked. The change adds one thing that is not a deletion: the
`refuses-a-malformed-row` adapter probe in `scripts/assurance_chain.py`. It is
owned by `FR-006-AC-5` — "Each of the twelve verification outcomes is
demonstrated by a case that produced it" — and traced through `TC-025`, which
asserts `malformed` among the twelve. It is not new capability; it relocates a
demonstrator whose previous owner was deleted.

The one other non-deletion is the `compatibility.py` digest pin in
`assurance/pins.json`, owned by `FR-006-AC-1` and traced through `TC-021`, which
asserts `artifact_mismatches` is empty. Without it that assertion would have had
no subject.

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
| Twelve outcomes still distinguishable | Yes — all twelve demonstrated by the chain alone; mutation-probed |
| No record rewritten, backdated, or re-sealed | Yes — every retained record is deleted; no file under `evidence/` was modified before removal, and no surviving claim asserts historical evidence still verifies |
| Retired identifiers not reused | Yes — recorded in `FR-006` and `spec/evidence/suites.md` |
| Hosted CI not dispatched | Yes — `workflow_dispatch` only, unchanged and undispatched |
| Nothing published, tagged, or released | Yes |
