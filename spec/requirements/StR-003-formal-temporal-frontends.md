---
id: StR-003
title: Native Quire temporal bridges need typed signals and attributable source context
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: satisfied_by
---

# StR-003: Native Quire temporal bridges need typed signals and attributable source context

## Stakeholder Need

Native Quire temporal-bridge maintainers require that the bridge shall preserve
stable named total-Boolean signal bindings and the exact native requirement
revision, clause, anchor, and source span through internal MLTL evaluation.

## Rationale

`agent-ix/quire-contract-ir#63` projects total Boolean predicates from a typed
native semantic IR package. An id-to-name proposition map cannot prove that a
free variable came from an admitted bounded field, and a formula id cannot
prove which requirement revision or clause produced a temporal result. The
shared syntax substrate must carry those identities without importing the IR or
evidence systems themselves.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-003-VC-1 | Every proposition referenced by a bound formula resolves to one stable named Boolean signal in a closed, versioned catalog; bounded non-Boolean inputs remain declared but cannot be coerced into direct propositions. | Test (TC-028, TC-030) |
| StR-003-VC-2 | A supplied requirement id/revision, clause id, anchor, and source span survives the borrowed and owned boundaries exactly; absence remains absence and a partial context is rejected. | Test (TC-031) |

## Stakeholders

Maintainers of `quire-contract-ir#63`, tl-parse, tl-rewrite, tl-mltl, R2U2/C2PO
adapters, and human assurance reviewers.

## Context and Assumptions

The native Quire frontend owns IR-field derivation and typed predicate lowering.
tl-syntax owns only the internal parser-independent identity, bounded-domain,
binding, and source-context contract. A non-Boolean source field may participate
through an explicitly derived total Boolean signal, but this crate does not
define that predicate expression or silently cast the field. FRETish is an
export-only mapping and never an editable source authority or qualification
dependency.

## Traceability

This need is realized by [FR-007](./FR-007-typed-signal-context.md), constrained
by [NFR-001](./NFR-001-no-std-feature-boundary.md), and consumed later by
`agent-ix/tl-parse#20`, `agent-ix/tl-rewrite#21`, and
`agent-ix/tl-mltl#24`.
