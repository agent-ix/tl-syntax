---
id: SR-067
title: "Code review — syntax owner reconciliation"
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#65; FR-014; PLAN-010 Task-009 tl-syntax allocation"
review_set: subset
---

# Code review — syntax owner reconciliation

## Summary

Reviewed the complete `tl-syntax#65` implementation against FR-014, ADR-003,
IF-004, the accepted v1/v2 contracts, and TC-075. The implementation is a real
owner API and semantic module reconciliation; it adds no parser, evaluator,
Contract-IR vocabulary, Boolean coercion, or qualification machinery.

## Verdict

**PASS after remediation.** All findings below were fixed before promotion.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-6701 | high | **FIXED:** the baseline owner readers accepted whitespace/reordered JSON and exposed no caller limits, formula readers, formula-v2 schema, pinned digests, or content identities. One shared reader now enforces canonical bytes and all effective limits for all four owner documents. | `contracts::reader`; `SyntaxArtifactLimits`; TC-075 | missing-requirement |
| FND-6702 | high | **FIXED:** an initial implementation interpretation narrowed formula-v2 under its existing identity. The accepted profile-partitioned behavior and lossless v1 upgrade were restored before review completion. | FR-013; FR-014-AC-3/4; `FormulaDocument` | wrong-requirement |
| FND-6703 | medium | **FIXED:** formula/profile and signal/domain/catalog/binding/document responsibilities remained coupled in flat source files. They now live under the reviewed semantic modules with deliberate crate-root compatibility re-exports. | ADR-003; `src/formula`; `src/signal`; `src/contracts` | implementation-bug-despite-evidence |
| FND-6704 | medium | **FIXED:** owner maxima were duplicated across formula, signal, proposition and manifest code. `contracts::limits` is now the single definition and old public constants alias it. | `contracts::limits` | implementation-bug-despite-evidence |
| FND-6705 | low | **FIXED:** the content-identity domain was initially private, preventing consumers from independently reproducing the identity. The exact domain prefix is now public as `CONTENT_IDENTITY_DOMAIN_V1`. | `contracts::identity` | implementation-bug-despite-evidence |

## Boundary checks

- Formula-v1 bytes and behavior, formula-v2 future/past profile partitioning,
  signal/map semantics, corpus identities and crate-root imports are preserved.
- Canonicalization serializes the admitted owner value; it does not sort a
  generic JSON tree or create a second semantic parser.
- Population and minimum work charges are computed before typed retention;
  owner bounded visitors and post-construction checks provide defense in depth.
- TC-075 calls the production readers and independently recomputes all schema
  and content digests.
