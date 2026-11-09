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

The crate shall build for the bare-metal target `thumbv7em-none-eabi` in three feature configurations.

The crate shall run its unit and property suites under Miri without reported undefined behavior.

The crate shall enforce each declared resource limit so that a value equal to the limit is accepted; a value one past the limit is refused with the typed error that names that limit.

The crate shall produce every variant of every public error type from at least one test.

## Scope

- The embedded builds apply to the library at `thumbv7em-none-eabi` in exactly
  the three configurations stated in NFR-006-AC-1.
- Miri applies to the unit tests and property tests, run on the host.
- The limit contract applies to the limits and error types enumerated in the
  tables below, on every path that enforces them.
- A variant that no code path can produce is removed from its enum in the same
  change. This change removes `SignalCatalogError::CatalogIndexOutOfRange` and
  `ScratchIndexUnrepresentable`, `FormulaConversionError::PastNodeUnsupported` and
  `NodeIdentityOutOfRange`, `LassoTraceError::PositionLimit` and
  `PartialValuationError::MapIdentityMismatch`. No limit value or wire format changes.

## Rationale

Embedded and certification-oriented consumers rely on the crate being
allocation-bounded and free of undefined behavior. A limit whose boundary is
untested can drift by one without any test failing, and an error variant that
no test produces can become unreachable or mislabeled unnoticed.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|---|---|---|---|
| Declared limits without an accept test and a refuse test per enforcing path | 0 | 0 | Test (TC-165) |
| Public error variants neither produced by a test nor removed | 0 | 0 | Test (TC-166) |
| Miri-reported undefined behavior in the unit and property suites | 0 | 0 | Analysis (TC-164) |
| Embedded-target feature builds that fail | 0 | 0 | Test (TC-163) |

## Limit contract table

Order is by source module. For the two gate-style limits whose owner value no canonical
document reaches at that depth (row 2), "accepted" means the gate passes and a later,
different check refuses the input. Contract for every row: a value equal to the limit
is accepted; a value one past the limit is refused with the typed error that
names that limit. "Caller input" says whether a caller-lowered
`SyntaxArtifactLimits` field reaches the path (used by the Miri run, see
NFR-006-AC-2).

| # | Limit (value) | Typed error | Accept side | Reject side | Caller input |
|---|---|---|---|---|---|
| 1 | `MAX_TL_DOCUMENT_BYTES` (64 MiB) | `StrictDocumentReadError::DocumentTooLarge` | document of exactly the limit | limit + 1 bytes | `document_bytes` |
| 2 | `MAX_TL_DOCUMENT_DEPTH` (64) | `StrictDocumentReadError::DepthLimitExceeded` | nesting of 64 | nesting of 65 | `json_depth` |
| 3 | string bytes (64 KiB) | `StrictDocumentReadError::StringTooLarge` | string of 64 KiB | 64 KiB + 1 | `string_bytes` |
| 4 | work (256 Mi units) | `StrictDocumentReadError::WorkLimitExceeded` | exact work | work + 1 | `work` (the owner value is unreachable: a 64 MiB document costs about 192 Mi units, so the boundary is tested at a caller-lowered value and the test asserts the owner value exceeds any admissible cost) |
| 5 | `MAX_FORMULA_DOCUMENT_NODES` (100,000), reader | `StrictDocumentReadError::ResourceLimitExceeded` (formula nodes) | 100,000 nodes | 100,001 | `formula_nodes` |
| 6 | `MAX_FORMULA_DOCUMENT_NODES`, `FormulaDocument::new`/`new_v2` | `FormulaError::TooManyNodes`, `DocumentNodeLimitExceeded` | 100,000 | 100,001 | none |
| 7 | `MAX_FORMULA_DOCUMENT_NODES`, future lowering (3-node charge) | `FutureLoweringRefusal::DocumentNodeLimitExceeded` | fills to 100,000 | 100,001 | none |
| 8 | `MAX_FORMULA_DOCUMENT_NODES`, infinite graph | `InfiniteFormulaError::NodeLimit` | 100,000 | 100,001 | none |
| 9 | `MAX_FORMULA_DOCUMENT_NODES`, lasso observations (prefix plus loop) | `LassoTraceError::ResourceLimit` | 100,000 | 100,001 | none |
| 10 | `MAX_FORMULA_DOCUMENT_NODES`, fairness roots | `FairnessPremisesError::ResourceLimit` | 100,000 | 100,001 | none |
| 11 | `MAX_FORMULA_DOCUMENT_DEPTH` (4,096), formula v2 | `FormulaError::DocumentDepthLimitExceeded` | depth 4,096 | depth 4,097 | none |
| 12 | `MAX_FORMULA_DOCUMENT_DEPTH`, infinite graph | `InfiniteFormulaError::DepthLimit` | 4,096 | 4,097 | none |
| 13 | `MAX_FORMULA_DOCUMENT_DEPTH`, reader | `StrictDocumentReadError::ResourceLimitExceeded` (formula depth) at a caller-lowered value; at the owner value the graph constructor refuses first and the reader reports `InvalidDocument` carrying `InfiniteFormulaError::DepthLimit` | 4,096 | 4,097 | `formula_depth` |
| 14 | `MAX_SIGNAL_CATALOG_SIGNALS` (100,000), borrowed | `SignalCatalogError::SignalLimitExceeded` | 100,000 | 100,001 | none |
| 15 | `MAX_SIGNAL_CATALOG_SIGNALS`, reader | `StrictDocumentReadError::ResourceLimitExceeded` (signals) | 100,000 | 100,001 | `signals` |
| 16 | `MAX_SIGNAL_CATALOG_BINDINGS` (100,000), borrowed | `SignalCatalogError::BindingLimitExceeded` | 100,000 | 100,001 | none |
| 17 | `MAX_SIGNAL_CATALOG_BINDINGS`, reader | `StrictDocumentReadError::ResourceLimitExceeded` (bindings) | 100,000 | 100,001 | `bindings` |
| 18 | `MAX_PROPOSITION_MAP_ENTRIES` (100,000), reader | `StrictDocumentReadError::ResourceLimitExceeded` (propositions) | 100,000 | 100,001 | `propositions` |
| 19 | `MAX_PROPOSITION_MAP_ENTRIES`, partial valuation entries | `PartialValuationError::ResourceLimit` | 100,000 | 100,001 | none |
| 20 | `MAX_SIGNAL_NAME_BYTES` (255), borrowed and wire | `SignalCatalogError::SignalNameTooLong` | 255 bytes | 256 bytes | none |
| 21 | `MAX_REQUIREMENT_CONTEXT_FIELD_BYTES` (1,024) | `RequirementContextError::FieldTooLong` | 1,024 bytes, each field | 1,025 | none |
| 22 | `PAST_PROFILE_MANIFEST_MAX_BYTES` (64 KiB) | `PastProfileManifestError` (struct; `field()` is `"size"`) | exactly 64 KiB | 64 KiB + 1 | none |
| 23 | `MAX_FUTURE_LOWERING_IDENTITY_BYTES` (128) | `FutureLoweringRefusal::RequestIdentityTooLong`, `OperatorProfileTooLong`, `SemanticProfileTooLong` | 128 bytes | 129 | none |
| 24 | `MAX_FUTURE_LOWERING_KIND_BYTES` (16) | `FutureLoweringRefusal::KindTooLong` | 16 bytes | 17 | none |

## Error type table

Every variant of each of these types is produced by a test in `src/error_variants.rs` that
asserts the exact variant (a per-enum exhaustive `match` there fails to compile when a
variant is added without being listed; the enums are `#[non_exhaustive]` outside the crate,
so the check lives inside it): `RequirementContextError`, `FutureLoweringRefusal`,
`StrictDocumentReadError`, `TemporalIntervalParseError`, `InfiniteClockError`,
`InfiniteProfileError`, `InfiniteFormulaError`, `FormulaError`,
`PropositionMapError`, `SignalDomainError`, `FormulaConversionError`,
`PartialValuationError`, `LassoTraceError`, `FairnessPremisesError`,
`FairnessReadError`, `SignalCatalogError`, `FormulaBindingError`. The struct
`PastProfileManifestError` is not an enum; its every distinct `field()` refusal
value is produced by a test.

## Verification

TC-163 runs the three embedded builds through the `check-features` Make
target, which `tests/feature_boundary.rs` executes; a failed build fails the test. TC-164 is an Analysis of the completion of the `miri`
Make target inside `make ci`: a Miri-reported error fails the gate, and the
analysis inspects the gate's exit status and the Miri target's presence in the
composite. TC-165 is `tests/limit_boundaries.rs`; TC-166 is `src/error_variants.rs` plus the
manifest refusal-field test in `tests/past_profile_manifest.rs`.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| NFR-006-AC-1 | In addition to the host builds of NFR-001-AC-2, the library builds for `thumbv7em-none-eabi` with exactly three configurations: no default features, `alloc` only (`--features alloc`), and `alloc` plus `serde` (`--features alloc,serde`); the all-features configuration is excluded because `--all-features` equals `alloc,serde` in this crate and adds no distinct configuration. | Test (TC-163) |
| NFR-006-AC-2 | The unit and property suites complete under Miri with no undefined behavior reported, as part of `make ci`. The native run uses the owner value and is never skipped. The Miri run uses a caller-lowered limit for the same code path for the limits that have a caller input in the table (rows 1 to 5, 13, 15, 17, 18), and the Miri run of the paths with no caller input (rows 6 to 12, 14, 16, 19) is excluded with `#[cfg(not(miri))]` on those boundary tests only, while their smaller-input logic still runs under Miri through the unit suite; rows 20 to 24 are small enough to run at the owner value under Miri. | Analysis (TC-164) |
| NFR-006-AC-3 | For every row of the limit contract table, a value equal to the limit is accepted; a value one past the limit is refused with the typed error that names that limit. | Test (TC-165) |
| NFR-006-AC-4 | Every variant of every public error type in the error type table is produced by at least one test that asserts the exact variant, or is removed from the enum in the same change. | Test (TC-166) |

## Dependencies

Depends on [NFR-001](./NFR-001-no-std-feature-boundary.md) for the feature
boundary and constrains [FR-014](./FR-014-publish-strict-syntax-artifacts.md)
for the strict reader limits. Supports [StR-001](./StR-001-embedded-consumers.md).
