---
id: SR-051
title: "Failure-domain review of the past/history profile"
type: SpecReview
analysis: failure-domain
scope: "FR-011 through FR-013 and TL/native bridge boundaries"
review_set: all
---

## Summary

**PASS after remediation.** The profile now distinguishes semantic falsehood
from missing history, unsupported source data, unknown identities, resource
refusal, and superseding late data before an attributed result can escape.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-5101 | high | Missing pre-origin observations could be confused with a gapped supplied history. Fixed: only offsets before origin receive proposition-false extension; any gap from zero through the declared end refuses distinctly. | FR-011, FR-012-AC-1 | wrong-requirement |
| FND-5102 | high | Missing captures or partial/error-valued predicates could collapse into false and create a plausible result. Fixed: the native bridge refuses before constructing TL history and never coerces those states. | FR-012-AC-5 | missing-requirement |
| FND-5103 | medium | A late observation could silently mutate an earlier final result. Fixed: history revision/digest and every result input are immutable; late data creates a superseding identity. | FR-012-AC-3, FR-012-AC-4 | missing-requirement |
| FND-5104 | medium | Clock conversions could hide gaps through rounding or resampling. Fixed: only exact position and exact fixed-sample mappings are admitted; all implicit conversion paths refuse. | FR-012-AC-5 | wrong-requirement |
