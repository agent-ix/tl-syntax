---
id: SR-1470
title: "Independent base review of FR-362 and the four-to-five edits (TL-393)"
type: SpecReview
analysis: base
scope: "FR-029, FR-030, FR-032, FR-033, NFR-007 edits, FR-362, release-gates-test-matrix.md"
review_set: base
---

# Independent base review: tl-syntax TL-147-spec

## Summary

Checked FR-362 against `release-gate/src/lib.rs` on this tree: `CRATES` is `[&str; 4]` (line 11), the candidate-set, consumer and decision checks use `CRATES.len()` (lines 477, 667, 746), so the "read the length from the constant" claim is true and the work is mostly the constant and four message strings. `quire validate` shows no delta; coverage adds six planned rows (exempt). FR-362-AC-3 table row closes correctly.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Status overstated: FR-029 and FR-033 rows and TC-170, TC-172, TC-179 stay marked implemented, but the requirement text now says five crates while `CRATES` has four. A revision note explains it; the status column should be planned for the changed clauses. | release-gates-test-matrix.md |
| FND-002 | medium | FR-030 gains new behaviour (replay owner corpora through tl-solve's encoder and witness re-check, citing tl-solve FR-019, which is an oracle cross-check on a different corpus) and FR-032-AC-1 requires a satisfiability step; ACs of FR-030 are not updated and TL-394 only authorises the count change. | FR-030, FR-032 |
| FND-003 | low | FR-362 has no AC that the tag plan places tl-solve before tl-rewrite as a refusal-only check beyond AC-1; AC-1 covers order, the refusal of a wrong order is in Behavior but not in an AC. | FR-362 |
| FND-004 | low | FR-362 states a sixth crate is refused; tl-oracle is unpublished and outside the set; say so to avoid a false refusal when the graph test sees it as a dev edge. | FR-362 |
| FND-005 | low | Verification cells carry `TC-` ids in FR-362 (integration-testing (TC-250..)). | FR-362 |
