---
id: StR-003
title: Formal temporal frontends need typed signals and attributable source context
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: satisfied_by
---

# StR-003: Formal temporal frontends need typed signals and attributable source context

## Stakeholder Need

Formal temporal frontend maintainers require that an MLTL formula shall bind its
free propositions to stable named signals with explicit bounded value domains,
and that downstream temporal evidence shall retain the exact requirement
revision, clause, anchor, and source span when the frontend supplies them.

## Rationale

`agent-ix/quire-contract-ir#57` derives variables from a typed semantic IR
package. An id-to-name proposition map cannot prove that a free variable came
from a bounded field, and a formula id cannot prove which requirement revision
or clause produced a temporal result. The shared syntax substrate must carry
those identities without importing the IR or evidence systems themselves.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-003-VC-1 | Every proposition referenced by a bound formula resolves to one stable named Boolean signal in a closed, versioned catalog; bounded non-Boolean inputs remain declared but cannot be coerced into direct propositions. | Test (TC-028, TC-030) |
| StR-003-VC-2 | A supplied requirement id/revision, clause id, anchor, and source span survives the borrowed and owned boundaries exactly; absence remains absence and a partial context is rejected. | Test (TC-031) |

## Stakeholders

Maintainers of `quire-contract-ir#57`, tl-parse, tl-rewrite, tl-mltl, R2U2/C2PO
adapters, and human assurance reviewers.

## Context and Assumptions

The future FRETish frontend owns IR-field derivation and predicate lowering.
tl-syntax owns only the parser-independent identity, bounded-domain, binding,
and source-context contract. A non-Boolean source field may participate through
an explicitly derived Boolean signal, but this crate does not define that
predicate expression or silently cast the field.

## Traceability

This need is realized by [FR-007](./FR-007-typed-signal-context.md), constrained
by [NFR-001](./NFR-001-no-std-feature-boundary.md), and consumed later by
`agent-ix/tl-parse#20`, `agent-ix/tl-rewrite#21`, and
`agent-ix/tl-mltl#24`.
