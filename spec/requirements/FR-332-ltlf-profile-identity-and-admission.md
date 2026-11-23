---
id: FR-332
title: "Admit unbounded LTLf formulas under a new finite-trace profile identity"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-045
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-044
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-289
    type: depends_on
---

# FR-332: Admit unbounded LTLf formulas under a new finite-trace profile identity

## Description

When a formula document names the semantic profile `mltl.ltlf-finite-trace/v1`,
tl-syntax shall construct a formula from it only if every node is admitted by the
closed operator catalog `tl-syntax.ltlf-operators/v1` in the edition
`tl-syntax.formula-ltlf/v1`.

## Inputs

- A formula document (edition `tl-syntax.formula-ltlf/v1`) with a profile field.
- The existing FR-007 signal catalog and proposition binding.

## Outputs

- A validated `Formula` carrying `SemanticProfile::LtlfFiniteTraceV1`, or a typed
  refusal naming the refused axis (profile, node, interval, edition).

## Behavior

Admitted nodes: `False`, `True`, `Proposition`, `Not`, `And`, `Or`, `Implies`,
`Equivalent`, `LtlfFuture` (`F`), `LtlfGlobally` (`G`), `LtlfUntil` (`U`), `LtlfRelease` (`R`),
`StrongNext`, `WeakNext`. These are new node kinds distinct from the
interval-bearing `Future`, `Globally`, `Until` and `Release` (type shape
PROPOSED in ADR-044), and the LTLf edition admits none of the interval-bearing
kinds. `F/G/U/R` carry no interval field: their reading is "from the current
position to the last position". Refused with a distinct typed
cause each: a closed interval `[a,b]` on any temporal node; an open interval
`[a,)` for any `a`; `O`, `H`, `Y`, `S`, `T`; `W`; `M`. A document whose profile
is absent, unknown, or one of the four existing identities but whose edition is
`tl-syntax.formula-ltlf/v1` (and the converse) is refused before construction.
The end-of-trace rules are recorded in ADR-044 and are normative for evaluators;
this requirement owns only admission and identity.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-332-AC-1 | A document in the vocabulary above, under `mltl.ltlf-finite-trace/v1` and edition `tl-syntax.formula-ltlf/v1`, validates and reports that profile identity. | Test (TC-220) |
| FR-332-AC-2 | Each of the nine refusal classes (closed interval, open interval, `O`, `H`, `Y`, `S`, `T`, `W`, `M`) is refused before construction with its own typed cause and no partial formula. | Test (TC-221) |
| FR-332-AC-3 | A missing, unknown, mismatched profile or edition identity is refused with the mismatched axis named. | Test (TC-222) |

## Dependencies

FR-289 supplies the sibling unbounded edition this edition does not reuse.
FR-334 constrains compatibility; FR-333 adds the next nodes.
