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
versioned, origin-complete past profile with exact O/H/S/T semantics, immutable
anchored results, closed compatibility axes, and planned evidence. This
author-run specification review does not replace independent exact-head PR
review or authorize implementation.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5001 | medium | The first draft named the formula and semantic profiles but left history, analysis, result, operator, and parser contracts implicit. Fixed with seven exact identities and successor triggers. | FR-011, FR-012, FR-013 |
| FND-5002 | medium | Fixed-sample clocks could be misread as changing interval bounds from positions into durations. Fixed: bounds remain integer positions and duration reinterpretation, rounding, drift correction, and resampling refuse. | FR-012-AC-5 |
| FND-5003 | low | Previous looked derivable from a unit-offset past unary operator, but constants disprove physical-predecessor meaning under pre-origin false extension. Retained as an explicit refusal pending a position-existence contract. | FR-011, ADR-002 |
