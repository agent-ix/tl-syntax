---
id: NFR-002
title: Deterministic and integrity-preserving artifacts
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/StR-002
    type: traces_to
---

# NFR-002: Deterministic and integrity-preserving artifacts

## Statement

For identical validated inputs, the crate shall expose identical equality,
ordering, serialization structure, corpus identities, and validation outcomes
on every supported platform. Identical validated signal catalogs and caller
source contexts shall likewise retain identical order and wire bytes.

## Scope

The requirement covers public syntax values and checked-in domain JSON
artifacts. Shared-assurance intake, candidate source identity, local-suite
identity, Make execution-control exposure, retained-record policy, and release
qualification are outside this deterministic-domain claim and are owned by
[NFR-003](./NFR-003-qualification-integrity.md).

## Rationale

Downstream differential testing and the sealed assurance chain depend on stable
identities and reproducible inputs.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Nondeterministic test failures | 0 | 0 | Test |
| Unversioned serialized document kinds | 0 | 0 | Inspection |

## Verification

Requirement-tagged tests compare values, wire strings, corpus metadata, and
validation outcomes using fixed checked-in inputs.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-002-AC-1 | Repeated comparisons over identical values produce one stable order. | Test (TC-005) |
| NFR-002-AC-2 | Every checked-in serialized document names a v1 schema and supported profile where applicable. | Test (TC-014) |
| NFR-002-AC-3 | Missing, skipped, failed, or not-yet-computed checks cannot be classified as passing evidence. | Test (TC-016) |
| NFR-002-AC-5 | Identical signal declarations, proposition bindings, and requirement contexts produce identical validated ordering and serialized bytes. | Test (TC-027, TC-031) |

### Retired criteria

**NFR-002-AC-4** is retired. It required this repository to verify source-locked
launcher and toolchain identities, bind a compiled test census, reject in-file
Make execution controls, behaviourally verify per-record evidence validators,
and require an active qualified record. Those local implementations were the
arrangement `agent-ix/engineering-assurance#10` was written to end; retirement
does not imply that every clause moved wholesale into another local criterion.

The criterion is not reassigned and **the identifier is not reused**. The reason
is that `SR-005` and `SR-006` adjudicate findings against `NFR-002-AC-4` by
name, and an identifier that means one thing in a closed review and another in
the current specification makes both unreadable.
The live [FR-006](./FR-006-shared-assurance-intake.md) criteria and
[NFR-003](./NFR-003-qualification-integrity.md) qualification boundary do not
inherit the five retired clauses wholesale. Their relationship is explicit:

- FR-006-AC-1 supplies declared component versions and artifact digests to the
  released classifier; NFR-003-AC-1 owns the resulting shared-component
  compatibility claim. Neither claims source-lock or qualification of every
  launcher and toolchain; that broader use-specific obligation remains outside
  this pre-stable claim under `agent-ix/engineering-assurance#11`.
- FR-006-AC-3 carries Quire's static specification, obligation, and Rust-symbol
  coverage export. The deleted repository-local compiled-test census is not
  retained or claimed as an equivalent control.
- The Make execution-control clause is an explicitly unclosed NFR-003
  qualification boundary, tracked as `challenge-make-execution-control` and
  `agent-ix/tl-syntax#11`; no acceptance criterion claims the deleted guard
  still exists.
- The per-record evidence-validator obligation went with the deleted retained
  record subject and retired FR-006-AC-4.
- NFR-003 explicitly claims no active qualified record for this pre-stable
  release. The obligation re-applies at the first stable release candidate
  under `agent-ix/engineering-assurance#11`.

Test case **TC-018** is retired with it, for the same reason and on the same
terms.

The next acceptance-criterion identifier is AC-5; retired AC-4 is not reused.

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md), and
[FR-007](./FR-007-typed-signal-context.md). Qualification and shared-intake
relationships are enumerated above and in NFR-003; no blanket succession by
FR-006 or NFR-003 is implied.
