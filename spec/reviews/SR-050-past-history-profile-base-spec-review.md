---
id: SR-050
title: "Base review of the past/history profile"
type: SpecReview
analysis: base
scope: "MRS-003, FR-011 through FR-013, ADR-002, TM-003"
review_set: all
---

## Summary

**PASS after remediation.** The issue #33 artifacts define one separately
versioned, origin-complete past profile with exact O/H/Y/S/T semantics, immutable
anchored results, closed compatibility axes, and planned evidence. This
author-run review has been reconciled with the independent exact-head PR review;
implementation remains gated on the accepted merged MRS-003 revision.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5001 | medium | The first draft named the formula and semantic profiles but left history, analysis, result, operator, and parser contracts implicit. Fixed with seven exact identities and successor triggers. | FR-011, FR-012, FR-013 |
| FND-5002 | medium | Fixed-sample clocks could be misread as changing interval bounds from positions into durations. Fixed: bounds remain integer positions and duration reinterpretation, rounding, drift correction, and resampling refuse. | FR-012-AC-5 |
| FND-5003 | low | The draft refused strong Previous while the authoritative native contract defines it as Once[1,1]. Fixed by adding primitive StrongPrevious/Y with exactly that truth relation while keeping weak Previous refused. | FR-011, ADR-002; quire-specification FR-092 |
| FND-5004 | high | Independent review found the fixed-sample mapping, result attribution/supersession, bridge-owned non-value classification, and dependency gate under-specified. Fixed with an exact checked sample equation, complete identity/relation fields, owner-state preservation, and a closed manifest contract plus routed tickets. | FR-012, FR-013, TC-051, TC-053, TC-056, TC-058 |
| FND-5005 | low | Long semantic node names and short internal text spellings shared an ambiguous ownership axis. Fixed by assigning only semantic labels/arity to the operator catalog and only O/H/Y/S/T spelling to clean-ascii/v3. | FR-011, FR-013 |
