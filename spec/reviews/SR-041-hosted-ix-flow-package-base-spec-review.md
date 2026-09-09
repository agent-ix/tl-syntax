---
id: SR-041
title: "Base specification review — hosted ix-flow package identity"
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#35 tl-syntax slice; NFR-003-AC-6; TC-039; hosted CI package identity and trigger boundary"
review_set: base
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: reviews
---

# SR-041: Base specification review — hosted ix-flow package identity

## Summary

The owner-selected base review checked requirement grammar, scope, identifier
resolution, criterion-to-test linkage, implementation feasibility, and
consistency with the existing human qualification boundary. The slice changes
one repository's hosted-workflow contract; the ecosystem issue retains
coordination ownership for the other three repositories.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-4101 | medium | The npm package identity and installed executable name can be confused, allowing the unavailable unscoped package to appear compliant. The criterion now names scoped `@agent-ix/ix-flow@0.0.4` separately from executable `ix-flow` version `0.0.4`. | NFR-003-AC-6, TC-039 |
| FND-4102 | medium | Correcting the install line could accidentally widen CI execution or imply that hosted CI ran. The criterion now fixes `workflow_dispatch` as the sole trigger and explicitly prohibits both hosted-run and release-decision claims. | NFR-003-AC-6, NFR-003 Qualification Boundary |
| FND-4103 | low | A repository-local assertion cannot establish public-registry availability by itself. The test is limited to exact workflow bytes and the exact executable supplied to the local gate; ecosystem issue #35 retains the isolated public-install acceptance check. | TC-039, agent-ix/tl-syntax#35 |

## Dispositions

All findings are resolved in the reviewed specification. No production code,
new dependency, trigger change, local package classifier, or hosted dispatch is
authorized. Implementation may change the workflow package token and add the
Rust assertion described by TC-039. Independent exact-head review remains
required before merge.
