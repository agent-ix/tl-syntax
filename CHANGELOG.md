# Changelog

All notable user-visible changes to `tl-syntax` are recorded here. The crate is
distributed as a git source release (`publish = false`); versions are git tags.

## 0.4.0

Second coordinated release of the MLTL crates (`tl-syntax`, `tl-parse`,
`tl-mltl`, `tl-rewrite`). All four crates move to 0.4.0 together. Changes are
relative to v0.3.0. This is a breaking 0.x release; the API migration inventory
below lists every break reported by `cargo-semver-checks` 0.50.0.

### Added

- **Infinite-trace profile (TL-15).** `SemanticProfile::InfiniteTraceV1`
  (`mltl.infinite-trace/v1`), the unbounded formula graph (`InfiniteFormula`,
  `InfiniteFormulaDocument`, `InfiniteNode`, `TemporalInterval`,
  `UnboundedInterval`, schema `FORMULA_UNBOUNDED_V1`), `lower_infinite_future` and
  `select_infinite_profile`, the lasso trace, fairness-premise and partial-valuation
  documents (`LassoTraceDocument`, `FairnessPremisesDocument`, `PartialValuation`),
  and the liveness capability registration and settlement (`settle_liveness`,
  `LivenessBackend`, `LIVENESS_CAPABILITY_V1`). `SemanticProfile::ALL` now has four
  entries. The `corpus/infinite-trace` corpus and the `infinite_wire_decode` fuzz
  target cover the new wire documents.
- The coordinated release gate (TL-213, FR-029..033, NFR-007): the `release-gate`
  checker verifies the candidate graph, pins and MSRV, golden files against the
  previous tag, and the API lane (`cargo-semver-checks` 0.50.0).
- `make miri` runs the unit and property suites under Miri on the pinned nightly;
  `make ci` includes it. `make check-features` also builds the library for
  `thumbv7em-none-eabi` with no features, `alloc`, and `alloc,serde`.
- `tests/limit_boundaries.rs` and `src/error_variants.rs` (NFR-006): every declared
  limit is accepted at the limit and refused one past it, and every public error
  variant is produced by a test.

### Changed

- The mutation gate and the cargo-mutants runner are one Rust tool, `tools/tl-gate`, replacing `scripts/mutation_gate.py` and `scripts/run_mutants.py`. tl-parse, tl-rewrite and tl-mltl build it from this repository instead of keeping copies (TL-370).
- Strict reader reject-class tables, a feature-combination test lane and a bounded-reader refusal of `[a,)` intervals under fuzz harden the readers (TL-364, TL-363, TL-219).

### Removed

Error variants that no code path could produce:

- `SignalCatalogError::CatalogIndexOutOfRange` and `ScratchIndexUnrepresentable`:
  validation indexes declarations below the 100,000-signal ceiling, so the `u32`
  scratch slot and `usize` index conversions cannot fail.
- `FormulaConversionError::PastNodeUnsupported` and `NodeIdentityOutOfRange`: only
  the history and infinite profiles admit past nodes, and `try_to_v1` already
  refuses both profiles; owned documents hold at most 100,000 nodes.
- `LassoTraceError::PositionLimit`: observations are capped at 100,000 before
  positions are assigned.
- `PartialValuationError::MapIdentityMismatch`: never constructed; a mismatched
  map identity is reported as `LassoTraceError::MapIdentityMismatch`.

### API migration inventory

- `enum_no_repr_variant_discriminant_changed` `FormulaError::FormulaV1NodeUnsupported`: Migration: do not cast `FormulaError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::SignalIdentityNotIncreasing`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::EmptySignalName`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::SignalNameTooLong`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::DuplicateSignalName`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::BindingIdentityNotIncreasing`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::BindingTargetMissing`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_no_repr_variant_discriminant_changed` `SignalCatalogError::NonBooleanPropositionBinding`: Migration: do not cast `SignalCatalogError` variants to integers with `as`; their discriminants shifted when variants were removed or added, so match on the variant or use an explicit mapping.
- `enum_variant_added` `SemanticProfile:InfiniteTraceV1`: Migration: add a match arm for `SemanticProfile::InfiniteTraceV1` or a wildcard arm, and use `SemanticProfile::ALL` (now four entries) instead of a fixed three-element list.
- `enum_variant_missing` `FormulaConversionError::PastNodeUnsupported`: Migration: delete the match arm; `try_to_v1` reports past nodes through the profile error, and this variant was never produced.
- `enum_variant_missing` `FormulaConversionError::NodeIdentityOutOfRange`: Migration: delete the match arm; owned documents hold at most 100,000 nodes, so this variant was never produced.
- `enum_variant_missing` `SignalCatalogError::CatalogIndexOutOfRange`: Migration: delete the match arm; validation indexes below the 100,000-signal ceiling, so this variant was never produced.
- `enum_variant_missing` `SignalCatalogError::ScratchIndexUnrepresentable`: Migration: delete the match arm; the scratch slot conversions cannot fail, so this variant was never produced.
- `partial_ord_enum_variants_reordered` `FormulaError::FormulaV1NodeUnsupported`: Migration: do not rely on the derived `PartialOrd` order of `FormulaError`; sort by an explicit key such as the error's display text.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::SignalIdentityNotIncreasing`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::EmptySignalName`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::SignalNameTooLong`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::DuplicateSignalName`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::BindingIdentityNotIncreasing`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::BindingTargetMissing`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.
- `partial_ord_enum_variants_reordered` `SignalCatalogError::NonBooleanPropositionBinding`: Migration: do not rely on the derived `PartialOrd` order of `SignalCatalogError`; sort by an explicit key.

## 0.3.0

First coordinated release of the MLTL crates (`tl-syntax`, `tl-parse`,
`tl-mltl`, `tl-rewrite`). The version skips 0.2.0 so all four crates share one
number past `tl-mltl`'s existing v0.2.0. Changes are relative to v0.1.0.

Release gate: the full local gate (`make ci`) passes. `make spec-release` does
not. Its strict coverage check reports rows with no backing test, and every one
belongs to work this release does not claim:
- source-qualification readiness (FR-015..019, NFR-004/005)
- infinite-trace and liveness (FR-289..291)
- downstream profile rows that other crates' evidence will back
- rows verified by analysis, which strict mode never counts as backed

The owner accepted this exception for 0.3.0. Later releases follow the release
gates specified in the V1 spec cycle.

### Added

- **Past-time (history) profile.** The formula graph admits the origin-complete
  past operators Once, Historically, strong Previous, Since and Triggered
  (`NodeKind::{Once, Historically, StrongPrevious, Since, Triggered}`) under the
  new semantic profile `mltl.origin-complete-history/v1`
  (`SemanticProfile::OriginCompleteHistoryV1`). The closed operator catalog is
  `PastOperatorKind` / `PAST_OPERATORS_V1`, and `NodeKind::temporal_family()` /
  `NodeKind::past_operator()` classify nodes. Future and past primitives are
  admitted only by their matching profile.
- **Formula document v2.** `tl-syntax.formula/v2` carries the past catalog.
  `FormulaDocument::new_v2`, `from_formula_v2`, `to_v2` (lossless upgrade) and
  `try_to_v1` (refuses past profiles and nodes with `FormulaConversionError`).
  Formula-v1 stays future-only and byte-compatible.
- **W/M derived operators.** An allocation-free lowering admission
  (`FutureLoweringRequest`, `FutureLowering`, `FutureLoweringReport`,
  `FutureLoweringRefusal`) lowers weak-until `W` to `U`/`G`/`Or` and
  strong-release `M` to `R`/`F`/`And`, so consumers only ever see the canonical
  F/G/U/R core. Refusals are typed and reported in a fixed precedence.
- **Typed signal catalogs and requirement contexts as owner contracts.** The
  signal catalog, proposition map and requirement context now have public
  module paths (`signal::{domain, catalog, binding, document}`,
  `formula::{graph, profile, document}`, `contracts::{limits, manifest}`), with
  crate-root re-exports unchanged. With `serde`, each owner document exposes
  pinned schema text, bytes and SHA-256 constants (`FORMULA_V1_SCHEMA*`,
  `FORMULA_V2_SCHEMA*`, `SIGNAL_CATALOG_V1_SCHEMA*`,
  `PROPOSITION_MAP_V1_SCHEMA*`).
- **Strict owner readers.** Formula, signal-catalog and proposition-map
  documents expose `from_json_bytes(bytes, SyntaxArtifactLimits)`, which
  intersects caller limits with fixed owner maxima, rejects noncanonical or
  trailing JSON, and returns only fully validated values
  (`StrictDocumentReadError`). Validated documents emit
  `canonical_json_bytes()` and a domain-separated `content_identity()`.
- **Past-profile implementation manifest.** `validate_past_profile_implementation`
  (`serde`) parses and validates the `PAST_PROFILE_IMPLEMENTATION_V1` manifest
  that fixes the authorized cross-crate past-profile implementation order.
- **Shared corpus without vendoring.** `CORPUS_DIR` is the filesystem path of
  this crate's `corpus/` directory, so a git or path dependent can read the
  shared conformance corpus directly. `PAST_HISTORY_CORPUS_V1` names the new
  paired past/history corpus; W/M source and canonical-graph pairs were added
  to the future-operator corpus.

### Changed

- **MSRV is now Rust 1.98.1** (was 1.75). `rust-toolchain.toml` pins that exact
  toolchain.
- Formula documents are rejected when their operator nesting exceeds
  `MAX_FORMULA_DOCUMENT_DEPTH` (4096), reported as
  `FormulaError::DocumentDepthLimitExceeded`.
- The `serde` feature now also enables `serde_json` and `sha2` (both
  `no_std`/`alloc`), which back the strict readers and content identities.
- The crate has no Quire-ecosystem dependency; it depends only on `serde`,
  `serde_json` and `sha2`, all optional.

### Breaking changes

- **`SemanticProfile` has a new variant** `OriginCompleteHistoryV1`. The enum is
  exhaustive. *Migration:* add an arm for it (or a wildcard) to every `match`
  on `SemanticProfile`; use `SemanticProfile::ALL` to iterate.
- **`NodeKind` has five new variants** `Once`, `Historically`, `StrongPrevious`,
  `Since`, `Triggered`. The enum is exhaustive. *Migration:* handle them in
  every `match` on `NodeKind`, or use `NodeKind::temporal_family()` /
  `past_operator()` to route past nodes to a refusal.
- **`FormulaSchemaVersion` has a new variant** `V2`. *Migration:* handle it in
  every `match`; call `try_to_v1()` if you must emit v1 only.
- **Rust 1.75 through 1.98.0 can no longer build the crate.** *Migration:*
  build with Rust 1.98.1 or newer and raise your own `rust-version` to match.
- **Formulas nested deeper than 4096 are refused** where v0.1.0 accepted them.
  *Migration:* none needed for realistic formulas; restructure any
  machine-generated formula that exceeds the limit.
