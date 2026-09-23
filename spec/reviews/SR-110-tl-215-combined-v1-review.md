---
id: SR-110
title: "TL-215 combined V1 specification review"
type: SpecReview
analysis: base
scope: "TL-207 through TL-213; tl-syntax c6010f7, tl-parse 83f696c, tl-rewrite 67edaa4, tl-mltl 7db847f"
review_set: all
---

# SR-110: TL-215 combined V1 specification review

## Summary

The owner selected `all`: the base checklist and seven analysis lenses. The
individual validated reviews are `SR-072` through `SR-079` in
`tl-mltl/spec/reviews/` (commit `070c857`).
This document indexes their cross-crate conclusions. Quire grammar validation
passed for all four V1 spec branches. Planned Test Matrix rows have red stubs;
they are not implementation evidence.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | TL-212 promises an all-tree depth-three local gate that cannot finish at the stated domain size. FR-044 reports exact visited and unvisited counts and grants exhaustive credit only to fully visited declared finite partitions. The owner must disposition the ticket/spec mismatch before acceptance. | TL-212, FR-044 |
| FND-002 | medium | R2U2 past-origin equivalence depends on a reviewed target version. FR-039 refuses export without that contract; FR-052 reports live differential outcomes separately. | FR-039, FR-052 |
| FND-003 | low | The independent `tl-oracle` repository is an enablement dependency, still unscaffolded. The TL-210 first-lap lasso scaffold is not a qualified repeated-loop oracle. | TL-245, TL-221, FR-053 |

## Cross-crate decisions

| Concern | Reviewed disposition |
|---|---|
| Profile and correspondence | TL uses `mltl.infinite-trace/v1`; `quire.temporal.infinite-trace/v1` identifies the QSpec correspondence. |
| Wire | `tl-syntax.formula/v1` and `/v2` remain unchanged. `formula-unbounded/v1` is a sibling edition; TL `[a,)` maps to QSL `[a,*]`. Lasso, partial valuation and fairness premises have separate schema identities. |
| Provider | ADR-003 keeps `tl_mltl::infinite` behind a non-default feature. The liveness capability has one registrant, and only the provider may emit `proved`. No whole-crate certification or audit claim is made for this prerelease library. |
| Outcomes | FR-341 labels are preserved. `failed` with execution `resource-incomplete` is distinct from `inconclusive`; missing and conflicting valuations remain distinct reasons. Model proof requires a model-wide procedure; V1 lasso truth is trace-scoped. |
| Oracle and corpus | `tl-oracle` is dev-only, independent of production evaluators. tl-syntax corpus families are read at the pinned `CORPUS_DIR`; R2U2 artifacts stay in tl-mltl. |
| Release | TL-213 defines 0.3.0 baseline pins, MSRV and historical-pin exceptions. No new release or tag is claimed here. |

## Acceptance state

The selected reviews are validated in ix-flow run
`f0833bf5-4584-4ea1-8de8-65dcfdb8d197`. The run is at `validated` and awaits
human acceptance. FND-001 remains open; Stage 1 production implementation is
still gated by TL-215.
