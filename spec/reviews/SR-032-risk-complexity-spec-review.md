---
id: SR-032
title: "Risk-complexity review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: risk-complexity
scope: "spec/"
review_set: all
---

## Summary

The highest technical-risk area is the shared assurance boundary: it combines
immutable release provenance, source identity, and human qualification limits.
Its volatility is external rather than product-driven and is already isolated
from tl-syntax runtime code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3201 | low | The external Quire/Engineering-Assurance release boundary remains the top integration risk; retain #16 as an explicit prerequisite and do not absorb it into the crate or its local test suite. | tl-syntax#16, NFR-003 |
