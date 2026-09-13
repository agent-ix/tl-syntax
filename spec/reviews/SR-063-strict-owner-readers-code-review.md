---
id: SR-063
title: "Code review — strict signal-catalog and proposition-map owner readers"
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#61 candidate; FR-007-AC-5; PLAN-010 Task-006 owner dependency"
review_set: subset
---

# Code review — strict signal-catalog and proposition-map owner readers

## Summary

Reviewed the complete `tl-syntax#61` diff against FR-007-AC-5, the immutable
corpus boundary, the public owner-reader contract, and PLAN-010 Task-006.

## Verdict

**PASS after remediation.** The public byte readers invoke the real owner
document types, remain behind `serde`, preserve the allocation-free default
feature, and refuse malformed or over-limit input without a partial document.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6301 | high | **FIXED:** the first draft placed the schema inside immutable `tl-syntax-corpus/v1`; it now lives in the separate normative spec artifact and corpus bytes/identity remain unchanged. | `spec/signal-catalog-v1.schema.json`; `corpus/SHA256SUMS` |
| FND-6302 | medium | **FIXED:** relying on serde_json's implementation recursion limit was replaced by a stable 64-container owner bound with typed refusal and boundary test. | `read_strict_document`; `MAX_TL_DOCUMENT_DEPTH` |
| FND-6303 | medium | **FIXED:** proposition-map decoding now uses a 100,000-entry bounded visitor rather than allocating an unbounded sequence. | `deserialize_propositions`; TC-032 |
| FND-6304 | low | **FIXED:** migrated only three existing matrix header labels to the installed Quire archetype; all matrix content is unchanged and `make spec` completes. | `spec/*test-matrix.md` |

## Boundary checks

- No parser, evaluator, contract-IR, Quire, or source-language vocabulary was
  added to the crate.
- Duplicate root and nested members, trailing data, unknown fields/versions,
  invalid domains, depth, byte size, and both population ceilings are exercised
  through production readers.
- The schema is Draft 7 metaschema-valid and accepts the checked-in positive
  signal-catalog fixture.
