---
id: SR-615
title: Independent specification review of P-TL-149 slice in tl-syntax
type: SpecReview
analysis: base
scope: spec/requirements/FR-034-signal-direction.md, spec/signal-catalog-v2.schema.json, spec/signal-direction-test-matrix.md
review_set: all
---

# Independent specification review (TL-149, tl-syntax)

## Summary

Slice of the P-TL-149 review; the full finding set, method statement (skills
performed manually, Jev not used) and population recomputation are in tl-solve
SR-100 (`spec/reviews/26-11-23-tl-149-realizability-spec-review.md` on branch
TL-149-review). `quire validate` shows no delta against main; coverage rows
rise by planned rows only.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-1411 | low | FR-034 points to ADR-004 decision 4 for the P-TL-150 interface; the interface is decision 5. | FR-034 line 122 |
| FND-1412 | low | P-TL-150 FR-303 predicate table binds the v1 catalog; use of a v2 catalog with numeric signals and MixedDirectionAtom is unspecified on both sides. | FR-034; TL-150 FR-303 |
| FND-1413 | low | v2 schema is v1 plus direction and version const (jq diff); ids FR-034, TM-008, TC-180..184 collide with no sibling branch; quire validate delta none. | schema |

## Fix pass

All findings above were addressed in specification text in this branch on
2026-11-23; see tl-solve SR-100 "Fix pass" for dispositions. No build was run.
