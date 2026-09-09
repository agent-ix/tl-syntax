---
id: SR-025
title: Qualification-integrity ownership code review
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#19 implementation candidate 6183568; changes from stacked base 273b4cd; NFR-003; FR-006; TC-038"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-006
    type: references
---

# SR-025: Qualification-integrity ownership code review

## Summary

The candidate assigns qualification meaning and lifecycle without adding an
assurance implementation. The only executable addition is TC-038 inside the
existing Rust shared-assurance suite. Three defects found during author review
were corrected before this record; no unresolved high or medium finding
remains. This author review grants no merge authority.

## Review coverage

| Surface | Result |
|---|---|
| Requirement ownership | FR-006 owns functional intake/census behavior; NFR-002 owns deterministic domain artifacts; NFR-003 owns qualification inference and lifecycle. |
| Shared contract declaration | NFR-003 and all five criteria are source-connected and declared; FR-006/NFR-003 statements match their authoritative rows. |
| Local-suite boundary | SUITE-008 has one exact command, is enumerated only through live test identifiers, and is absent from every Quoin proof obligation. |
| Source identity | The new NFR-003 path is present in both the exact live set and independent `spec` area population. |
| Deferred authority | Make suppression and the stable qualified record remain structured limitations with human owner, tracker, and first-stable trigger. |
| Architecture | No Python or shell helper, Make target, runner, collector, schema, evidence envelope, identity registry, retention path, dependency, or hosted workflow was added. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2501 | medium | Candidate 396315a added NFR-003 to the exact path set but left the independently authored `spec` population at 18, so TC-034 reached the coarse diagnostic before the intended within-area mutation. | NFR-003-AC-3, TC-034 | implementation-bug-despite-evidence |
| FND-2502 | medium | Candidate 2a490b7 paraphrased sealed FR-006/NFR-003 statements and omitted FR-006-AC-8, allowing the shared declaration to diverge from the authoritative requirement rows. | FR-006, NFR-003, `assurance/change-assurance.json` | implementation-bug-despite-evidence |
| FND-2503 | medium | The SUITE-008 note said TC-021 through TC-026, implicitly treating retired TC-024 as a live local result. | SUITE-008, FR-006-AC-4 | wrong-requirement |
| FND-2504 | low | AA-001 now states the first-stable qualified-record challenge, but the current change declaration does not bind AA-001 as an individually identified authoritative source. | AA-001, `tl-syntax#16` | correct-requirement-no-evidence |
| FND-2505 | low | TC-038 can prove the local suite is not declared as a proof input but cannot prove an independent reviewer actually ran it at an exact head. | NFR-003-AC-2, TC-038 | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2501 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | Candidate 2a490b7 expects 19 `spec` paths; focused TC-034 and the full nine-test shared suite pass. |
| FND-2502 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | Candidate 6183568 makes every live FR-006/NFR-003 declaration statement byte-identical to its criterion and includes FR-006-AC-8. |
| FND-2503 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | SUITE-008 now enumerates TC-021, TC-022, TC-023, TC-025, TC-026, TC-034, TC-035, and TC-038 explicitly. |
| FND-2504 | **DEFERRED TO EXISTING OWNER** | Issue #16 owns path/source binding and authoritative declaration metadata through the released shared contract; this change adds no local parser. |
| FND-2505 | **EXPECTED EXTERNAL BOUNDARY** | NFR-003 requires an independent exact-head review record in addition to TC-038; the author record does not self-grant that evidence. |

## Verification observed

Strict Quire validation reports 75/75 documents grammar-clean. Strict coverage
backs all 27 matrix cases, all five NFR-003 criteria, and 30/30 Rust symbols.
Focused TC-038 passes. After FND-2501 was fixed, all nine shared-assurance tests
passed serially. The declaration-alignment commit changes sealed bytes, so the
exact-final-head complete local gate remains required after this review record.
Hosted CI was not dispatched.

## Conclusion

The candidate is ready for closing gap analysis and final local verification.
Independent exact-head review must decide whether the remediated findings and
the issue #19 acceptance boundary are cleared.
