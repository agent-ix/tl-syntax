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
on every supported platform.

## Scope

The requirement covers all public syntax values and checked-in JSON artifacts.

Retained evidence is not in scope for this requirement. This repository retains
no evidence: the records its deleted collector wrote were themselves deleted
under `agent-ix/tl-syntax#12`, and retention, integrity, and qualification
authority belong to Quoin under
[FR-006](./FR-006-shared-assurance-intake.md).

Make execution control is also outside the evidence-integrity claim. The local
guard that rejected Make failure-suppression features was removed with the
collector it protected. At base `4cb5787`, an intentionally invalid Rust item
made ordinary `make ci` stop at `fmt-check` and exit 2. A `make -k ci`
diagnostic classified eight paths as failed or unmade. The other five were
unaffected by the selected compile fault, so their behaviour under their own
faults and `.IGNORE:` remains unmeasured. Adding a global `.IGNORE:` made the
eight affected paths emit ignored failures while Make treated all thirteen
prerequisites as successful and returned 0.
[SR-013](../reviews/SR-013-make-execution-control-measurement.md) records the
exact matrix and reproduction procedure. Quoin constrains charted producer
bytes only when a record is actually produced. The chain retains nothing
locally; under `.IGNORE:` its refusal was suppressed and no record was produced.
The `tl-syntax-release-owner` accepts only this measured spelling for pre-stable
development under `agent-ix/tl-syntax#11`. Seventeen other spellings remain
unmeasured, and the deviation must be re-evaluated before the first stable
release candidate. Use-specific qualification remains open under
`agent-ix/engineering-assurance#11`.

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
The live [FR-006](./FR-006-shared-assurance-intake.md) criteria are AC-1, AC-2,
AC-3, AC-5, AC-6, and AC-7. Their relationship to the five retired clauses is
explicit and not a blanket reassignment:

- FR-006-AC-1 carries only classification of the adopted shared component
  versions. It does not claim source-lock or qualification of every launcher
  and toolchain; that broader use-specific obligation remains outside this
  pre-stable claim under `agent-ix/engineering-assurance#11`.
- FR-006-AC-3 carries Quire's static specification, obligation, and Rust-symbol
  coverage export. The deleted repository-local compiled-test census is not
  retained or claimed as an equivalent control.
- The Make execution-control clause remains unowned by an acceptance criterion
  and is tracked as `challenge-make-execution-control` and
  `agent-ix/tl-syntax#11`.
- The per-record evidence-validator obligation went with the deleted retained
  record subject and retired FR-006-AC-4.
- No active qualified record is claimed for this pre-stable release. That
  obligation re-applies at the first stable release candidate under
  `agent-ix/engineering-assurance#11`.

Test case **TC-018** is retired with it, for the same reason and on the same
terms.

## Dependencies

Constrains [FR-002](./FR-002-validated-formula.md),
[FR-004](./FR-004-versioned-serialization.md), and
[FR-005](./FR-005-conformance-corpus.md). The surviving shared-intake
relationships and the clauses that remain unclaimed are enumerated above; no
blanket succession by [FR-006](./FR-006-shared-assurance-intake.md) is implied.
