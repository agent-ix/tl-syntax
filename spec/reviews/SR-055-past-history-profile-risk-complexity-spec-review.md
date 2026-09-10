---
id: SR-055
title: "Risk-complexity review of the past/history profile"
type: SpecReview
analysis: risk-complexity
scope: "FR-011 through FR-013 and ADR-002"
review_set: all
---

## Summary

**PASS after remediation.** The principal risks are nonzero-lower-bound
direction errors, origin/history confusion, cross-repository identity drift,
and resource blow-up. Each now has an exact rule, preflight refusal, and planned
oracle or mutation control.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5501 | high | Since can appear correct on zero-based intervals while using the wrong left range. Fixed: the witness is inclusive in `[a,b]`, the left range is exactly `[a,j)`, offsets before `a` are irrelevant, and TC-049/TC-052 mutate the distinction. | FR-011-AC-2, TC-049, TC-052 |
| FND-5502 | high | Previous encoded as O/H at `[1,1]` would conflate false extension with physical predecessor existence. Fixed by refusing both variants in v1. | FR-011, ADR-002 |
| FND-5503 | medium | Deep nesting and `u32::MAX` bounds can overflow analysis or iterate impractically. Fixed with checked u64 recurrences and preflight recursion, span, step, cardinality, and input-position limits. | FR-012-AC-2, TC-052, TC-053 |
| FND-5504 | medium | Seven independently evolving contracts could drift under one vague “past profile” label. Fixed with exact orthogonal identities and successor triggers. | FR-013 identity table |
