---
id: StR-002
title: Temporal tools need stable shared identities
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-003
    type: satisfied_by
---

# StR-002: Temporal tools need stable shared identities

## Stakeholder Need

Temporal-tool maintainers require that exchanged formulas shall retain stable
proposition, source, schema, corpus, and semantic-profile identities across
platforms and repository boundaries.

## Rationale

Parsing, rewriting, evaluation, and monitor comparison cannot be audited when
an artifact loses the identity of its propositions or the semantics under which
its expected result was produced.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-002-VC-1 | A serialized formula names a supported schema and semantic profile. | Test (TC-008) |
| StR-002-VC-2 | The corpus manifest names a stable revision and fixture identities. | Test (TC-014) |

`StR-002-VC-2` previously named an agent-produced inspection record retained
under `evidence/`, which was deleted under `agent-ix/tl-syntax#12`. That pointer
was stale rather than load-bearing: the criterion is and was discharged by a
test. `tests/integration.rs::shared_corpus_is_complete_stable_and_self_consistent`
carries `// Trace: TC-012, TC-013, TC-014, …, StR-002-VC-2` and asserts the
manifest's `corpus_revision` against the crate's `CORPUS_REVISION` and every
declared fixture identity. The validation method is corrected to name it;
nothing is restated more weakly.

## Stakeholders

Maintainers of tl-parse, tl-rewrite, tl-mltl, monitor adapters, and assurance
reviewers.

## Context and Assumptions

Downstream algorithms remain responsible for interpreting formulas according to
the selected profile. tl-syntax owns identifiers and validates structure only.

## Traceability

This need is realized by [FR-003](./FR-003-identities-and-profiles.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md).
