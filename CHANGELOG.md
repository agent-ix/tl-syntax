# Changelog

All notable user-visible changes to `tl-syntax` are recorded here. The crate is
distributed as a git source release (`publish = false`); versions are git tags.

## 0.3.0

First coordinated release of the MLTL crates (`tl-syntax`, `tl-parse`,
`tl-mltl`, `tl-rewrite`). The version skips 0.2.0 so all four crates share one
number past `tl-mltl`'s existing v0.2.0. Changes are relative to v0.1.0.

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
