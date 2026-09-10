---
id: SR-043
title: "Failure-domain review of the future operator profile"
type: SpecReview
analysis: failure-domain
scope: "FR-008 through FR-010 and current tl-syntax/tl-mltl boundaries"
review_set: all
---

## Summary

**PASS after remediation.** The reviewed boundary now covers profile, kind,
operand, interval, span, count, identity, schema, evaluation, closure, and
target-correspondence failures before a partial document or claim can escape.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-4301 | high | Next could silently disagree at the physical trace boundary while appearing to be a one-node derived lowering. Fixed by refusing both next variants and recording the constant/proposition discriminator. | FR-009, ADR-001 | wrong-requirement |
| FND-4302 | medium | “Append after checking” did not define storage ownership or no-alloc transactional behavior. Fixed: tl-syntax returns a preflighted fixed-size three-node value; only the caller appends after success. | FR-008-AC-3 | missing-requirement |
| FND-4303 | medium | One token span on every synthetic node confused token attribution with the expression denoted by the lowered root. Fixed: nodes carry the expression span; a non-wire report retains the token span; partial/non-contained pairs refuse. | FR-008-AC-4 | wrong-requirement |
