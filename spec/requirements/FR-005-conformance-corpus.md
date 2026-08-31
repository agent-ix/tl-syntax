---
id: FR-005
title: Publish a stable temporal conformance corpus
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
---

# FR-005: Publish a stable temporal conformance corpus

## Description

When the shared temporal corpus is published, the repository shall retain a
versioned manifest and fixtures covering primitive, nested, boundary, malformed,
short-trace, and large-bound cases.

## Inputs

- Versioned formula and proposition-map documents plus finite Boolean traces.

## Outputs

- Stable fixture identities with expected horizons and closed-trace outcomes
  where those results are defined by the corpus case.

## Behavior

- The manifest shall report one stable corpus revision.
- Every fixture shall identify its semantic profile and expected validation state.
- Evaluator cases shall include a horizon derived from the bounded-MLTL node
  graph and a reviewed `mltl.closed-trace/v1` outcome. For the empty trace,
  constants retain their Boolean value, propositions are absent, and temporal
  witnesses outside the trace are absent.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-005-AC-1 | The manifest includes every required fixture class with unique stable identities. | Test (TC-012) |
| FR-005-AC-2 | Every valid formula fixture deserializes and validates, its declared horizon equals the value derived from its node graph, and every malformed fixture is rejected for its declared reason. | Test (TC-013) |
| FR-005-AC-3 | The `tl-syntax-corpus/v1` identity and reviewed closed-trace results use only platform-independent JSON scalar and array values and remain unique. | Test (TC-014) |

## Dependencies

Depends on [FR-004](./FR-004-versioned-serialization.md). Downstream repositories
must pin and report the exported corpus revision before their v0.1 release.
