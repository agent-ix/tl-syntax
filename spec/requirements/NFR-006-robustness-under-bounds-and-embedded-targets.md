---
id: NFR-006
title: Stay robust at every declared bound, on embedded targets and under Miri
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/tl-syntax/StR-001
    type: traces_to
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-014
    type: constrains
---

# NFR-006: Stay robust at every declared bound, on embedded targets and under Miri

## Statement

The crate shall build for a bare-metal embedded target, run its unit and
property suites without undefined behavior under Miri, accept each declared
resource limit exactly at the limit and refuse it one past the limit with that
limit's typed error, and produce every variant of every public error enum from
at least one test.

## Scope

- Applies to the library built with no default features, with `alloc`, and with
  `alloc` plus `serde`, for `thumbv7em-none-eabi`.
- Applies to the unit tests and property tests of the crate, run under Miri on
  the host.
- Applies to every public limit constant and every limit of
  `SyntaxArtifactLimits`: `MAX_TL_DOCUMENT_BYTES`, `MAX_TL_DOCUMENT_DEPTH`,
  the string, work, signal, binding, proposition and formula node and depth
  ceilings, `MAX_SIGNAL_NAME_BYTES`, `MAX_REQUIREMENT_CONTEXT_FIELD_BYTES`,
  `PAST_PROFILE_MANIFEST_MAX_BYTES`, `MAX_FUTURE_LOWERING_IDENTITY_BYTES` and
  `MAX_FUTURE_LOWERING_KIND_BYTES`, and to every admission path that enforces
  one of them.
- Applies to every public enum whose name ends in `Error` or `Refusal`.
- Does not change any limit value, wire format or error variant.

## Rationale

Embedded and certification-oriented consumers rely on the crate being
allocation-bounded and free of undefined behavior. A limit whose boundary is
untested can drift by one without any test failing, and an error variant that
no test produces can become unreachable or mislabeled unnoticed.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Declared limits without an at-limit and a limit-plus-one test | 0 | 0 | Test (TC-165) |
| Public error variants not produced by any test | 0 | 0 | Test (TC-166) |
| Miri-reported undefined behavior in the unit and property suites | 0 | 0 | Test (TC-164) |
| Embedded-target feature builds that fail | 0 | 0 | Test (TC-163) |

## Verification

The local gate builds the library for `thumbv7em-none-eabi` in the three
feature configurations. A Miri target runs the unit and property suites with
a reduced property case count. One boundary test per limit and path accepts
the value at the limit and refuses it with the exact typed error one past the
limit. Per-enum exhaustive `match` statements in the error test fail to compile
when a variant is added without a test.

Where an owner ceiling cannot be materialized under Miri, the native run
exercises the owner value and the Miri run exercises the same code path through
a caller-lowered limit; the native run is never skipped.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-006-AC-1 | The library builds for `thumbv7em-none-eabi` with no default features, with `alloc`, and with `alloc` and `serde`, as part of the local gate. | Test (TC-163) |
| NFR-006-AC-2 | The unit and property suites complete under Miri with no undefined behavior reported, and the Miri run is part of the local gate. | Test (TC-164) |
| NFR-006-AC-3 | For every declared resource limit and each path that enforces it, a value equal to the limit is accepted and a value one past the limit is refused with that path's typed error. | Test (TC-165) |
| NFR-006-AC-4 | Every variant of every public error enum is produced by at least one test that asserts the exact variant. | Test (TC-166) |

## Dependencies

Depends on [NFR-001](./NFR-001-no-std-feature-boundary.md) for the feature
boundary and constrains [FR-014](./FR-014-publish-strict-syntax-artifacts.md)
for the strict reader limits. Supports [StR-001](./StR-001-embedded-consumers.md).
