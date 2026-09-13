---
id: SR-054
title: "Evidence-method review of the past/history profile"
type: SpecReview
analysis: evidence
scope: "FR-011 through FR-013 and TM-003"
review_set: all
---

## Summary

**PASS after remediation.** The pinned Quoin advisor evaluated the original 14
acceptance criteria with zero mismatch, uncatalogued, or inconclusive results.
The added strong-Previous criterion reuses the already selected property and
integration methods. TM-003 allocates property, integration, fuzz, mutation,
corpus, and independent Rust-oracle evidence without treating a foreign runtime
as semantic authority.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5401 | medium | The first dependency criterion was authored as manual inspection although its universal rejection behavior is automatable. Fixed by defining a dependency-manifest test and making TC-058 Integration evidence. | FR-013-AC-5, TC-058 |
| FND-5402 | medium | Parser and formula-v2 decode accept untrusted recursive input. Fixed: TC-057 applies bounded arbitrary-input tests to both for unwind and profile misattribution, while resource-limit refusals remain in TC-052/TC-053. | FR-013-AC-1, FR-013-AC-2, TC-057 |
| FND-5403 | low | No reviewed exact R2U2/C2PO past correspondence exists. Correctly retained as unavailable rather than manufacturing differential evidence; Isabelle remains future-only and FRET remains output-only. | FR-013-AC-4, TC-056 |
