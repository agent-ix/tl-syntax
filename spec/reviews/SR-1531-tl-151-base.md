---
id: SR-1531
title: "Base review of tl-syntax LTLf profile spec (TL-427)"
type: SpecReview
analysis: base
scope: "ADR-044/045, StR-045, FR-332..335, NFR-048, TM-048, PLAN-051"
review_set: all
---

## Summary

Semantics of ADR-044 item 3 match De Giacomo-Vardi as recalled (unverified against the paper): strong next needs i<last, weak next true at last, `last` is `Xw false`, R is dual of U; CE-1 and CE-2 verified by derivation; the padding contrast with closed-trace is correct (tl-mltl src/future/evaluate.rs:517-528). Counts (18, 4, 120) recompute. Cited ids exist (FR-002..005,007,009,013,289, NFR-001, ADR-002, MRS-001 in spec.md). No sibling-branch id collisions (TL-147: FR-362, TC-250-252; TL-149: FR-034, TC-180-184; TL-150: FR-302-311, ADR-014-018). Delta vs baseline: validate failures unchanged (4 pre-existing), +4 EARS warnings, 24 new unbacked ACs, rows 239 to 265. Full set of cross-repo findings is in tl-mltl SR-1511..SR-1520.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-15311 | medium | LTLf node shape unspecified: `NodeKind` is exhaustive and its F/G/U/R carry a mandatory interval (src/formula/graph.rs:197+), so LTLf needs a new type or new variants; the ADR consequences list only `SemanticProfile::ALL` (src/formula/profile.rs:26,79) though adding NodeKind variants breaks tl-mltl, tl-rewrite and tl-parse matches. | ADR-044, FR-332 |
| FND-15312 | low | FR-333 wire tags `StrongNext`/`WeakNext` conflict with existing snake_case tag convention (`strong_next`). | FR-333 |
| FND-15313 | low | EARS warnings: FR-332 line 25, FR-333 line 19. | FR-332, FR-333 |
| FND-15314 | low | ADRs placed in new spec/decisions; existing ADRs are in spec/assurance. | ADR-044, ADR-045 |
| FND-15315 | low | FR-335 CE-1 `G Xs true` and CE-2 `F Xw false` are derived, not the literature's named formula (acknowledged open citation); TL-428 AC says "known" counterexample. | FR-335 |
