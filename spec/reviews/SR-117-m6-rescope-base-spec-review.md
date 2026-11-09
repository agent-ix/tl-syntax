---
id: SR-117
title: "M6 source-readiness rescope base specification review"
type: SpecReview
analysis: base
scope: "tl-syntax branch tl-m6-spec-rescope: FR-009/010/011/012/013/017/291, TM-002/003/004, IT-002, suites"
review_set: base
---

# SR-117: M6 source-readiness rescope base specification review

## Summary

Base checklist over the diff only. No applicable AssuranceProfile makes spec-review required for this change; the owner reviews the withdrawal table in the PR body. Strict coverage unbacked rows fall from 76 to 62 and contradicted statuses from 3 to 0. The remaining 62 are the unbuilt M6 matrix (60 rows) plus NFR-006-AC-2/TC-164 (Analysis of a Miri gate, out of scope). Withdrawn criteria keep their original text or point to the owning repository's requirement; no IDs dangle.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-011-AC-2/3 and FR-012-AC-1/2/5 are withdrawn as re-homed to tl-mltl, but tl-mltl has no counterpart requirement and its past_history tests carry no trace tags. The criterion text is preserved in the withdrawn rows; a tl-mltl follow-up must adopt it or the evaluator semantics have no verified owner row. | FR-011, FR-012, TM-003 |
| FND-002 | low | All M6 rows (FR-015 through FR-019, NFR-004/005, TC-059 through TC-073) remain planned with no implementation anywhere; they stay unbacked under strict coverage until suites exist. FR-019 (executable-path census) is a maintainer-process obligation and a candidate for owner withdrawal. | FR-019, TM-004 |
| FND-003 | low | TC-072 changed from a human Analysis row to an adapter Integration test; the reviewer reconciliation moved to FR-018 decision evidence. IT-002 and suites text updated to match. | TC-072, FR-017-AC-7, IT-002 |
| FND-004 | low | FR-011-AC-4 retagged on two existing tl-syntax tests; they cover graph, profile and wire admission, while evaluator-entry refusal is in tl-mltl. Acceptable as one honest instance. | FR-011-AC-4 |

## Verdict

No hash, digest or pin antipattern introduced. ID formats, link integrity and duplicates clean. PASS with the follow-ups above.
