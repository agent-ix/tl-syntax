# Wire formats

All documents need the `serde` feature. Each is JSON with a `schema_version` that names its contract. Readers reject unknown versions and unknown fields.

## Documents

| Identifier | Rust type | Notes |
| --- | --- | --- |
| `tl-syntax.formula/v1` | `FormulaDocument` | Future operators only. Semantic profiles `mltl.closed-trace/v1` and `mltl.online-prefix/v1`. |
| `tl-syntax.formula/v2` | `FormulaDocument` | Adds the closed past-operator catalog `tl-syntax.past-operators/v1` and the profile `mltl.origin-complete-history/v1`. Depth is limited. |
| `tl-syntax.formula-unbounded/v1` | `InfiniteFormulaDocument` | Infinite-trace profile, clock `event_position`, intervals `[a,b]` or `[a,)`. |
| `tl-syntax.proposition-map/v1` | `PropositionMapDocument` | Proposition identities and names. Identities strictly increase and names are not empty or repeated. |
| `tl-syntax.signal-catalog/v1` | `SignalCatalogDocument` | Typed signals and direct bindings. |
| `tl-syntax.requirement-context/v1` | `RequirementContextDocument` | Caller-supplied requirement, clause, anchor and source span. |
| `tl-syntax.lasso-trace/v1` | `LassoTraceDocument` | An ultimately periodic trace: a prefix and a non-empty loop of observations. |
| `tl-syntax.partial-valuation/v1` | `PartialValuation` | Per-instant values `true`, `false`, `missing` or `conflicting`. |
| `tl-syntax.fairness-premises/v1` | `FairnessPremisesDocument` | Roots of a formula graph that must hold infinitely often. |

These identifiers name contracts that are not JSON documents: `tl-syntax.future-operators/v1` (derived `W` and `M`), `tl-syntax.future-lowering-request/v1`, `-report/v1` and `-refusal/v1`, and `tl-syntax.liveness/v1` (the capability that liveness routing names).

Formula-v1 is unchanged by later editions, and a v1 document upgrades to v2 without loss. A v2 document converts to v1 only when its profile and every node are expressible in v1. The unbounded edition is separate and cannot be mixed with the bounded ones.

## Schemas

The JSON Schema (Draft 7) files are checked in and compiled into the crate as constants, each with its bytes and lowercase SHA-256:

| Schema | Constants |
| --- | --- |
| `corpus/schema/formula-v1.schema.json` | `FORMULA_V1_SCHEMA`, `_BYTES`, `_SHA256` |
| `corpus/schema/formula-v2.schema.json` | `FORMULA_V2_SCHEMA`, `_BYTES`, `_SHA256` |
| `corpus/schema/formula-unbounded-v1.schema.json` | `FORMULA_UNBOUNDED_V1_SCHEMA`, `_BYTES`, `_SHA256` |
| `corpus/schema/proposition-map-v1.schema.json` | `PROPOSITION_MAP_V1_SCHEMA`, `_BYTES`, `_SHA256` |
| `spec/signal-catalog-v1.schema.json` | `SIGNAL_CATALOG_V1_SCHEMA`, `_BYTES`, `_SHA256` |

Draft 7 cannot express the formula graph's ordering rules (operands precede their node, the root is in range). The schemas declare that boundary in `x-tl-syntax-semantic-*` fields, and `Formula::new` and the document constructors enforce it.

## Strict readers

Each owner document has `from_json_bytes(bytes, SyntaxArtifactLimits)`. A reader:

- rejects input larger than the byte limit, deeper than the nesting limit, or with a string longer than the string limit,
- rejects more formula nodes, signals, bindings or propositions than the limit allows, and charges deterministic work against a work budget,
- rejects duplicate members, trailing data and unknown fields,
- rejects JSON that is valid but not the one canonical encoding (`NonCanonicalDocument`),
- returns only fully validated values.

`SyntaxArtifactLimits::default()` is the owner maximum. Callers may lower any field and can never raise one: the reader takes the minimum of the supplied and owner values.

| Limit | Owner maximum |
| --- | ---: |
| Document bytes | 67,108,864 (64 MiB) |
| JSON nesting depth | 64 |
| String bytes | 65,536 |
| Formula nodes | 100,000 |
| Formula depth | 4,096 |
| Signals | 100,000 |
| Bindings | 100,000 |
| Propositions | 100,000 |
| Work units | 268,435,456 |

Failures are `StrictDocumentReadError` values: `DocumentTooLarge`, `DepthLimitExceeded`, `StringTooLarge`, `WorkLimitExceeded`, `ResourceLimitExceeded`, `NonCanonicalDocument` and `InvalidDocument`. Fairness documents use `FairnessReadError`, which wraps a strict-read error or a binding error.

## Canonical bytes and content identity

`canonical_json_bytes()` serializes a validated document to its single canonical form: fields in declaration order, no whitespace. `content_identity()` returns the lowercase hex SHA-256 of

```
"tl-syntax.content-identity/v1\0" + schema_version + "\0" + canonical bytes
```

The domain prefix is exported as `CONTENT_IDENTITY_DOMAIN_V1`. Including the schema version keeps identities of different document kinds apart.

`FormulaDocument::semantic_view()` serializes the same document without source spans, for caches and replay keys that should not change when only source positions change. The ordinary form keeps spans.

## Past-profile implementation manifest

`validate_past_profile_implementation(bytes)` checks `spec/past-profile-implementation.json` against the one authorized dependency graph of the past-profile work: its format identifier, authorization state, prerequisites and the seven tasks with their predecessors. It refuses with a `PastProfileManifestError` that names the failing field. Input larger than `PAST_PROFILE_MANIFEST_MAX_BYTES` (64 KiB) is refused before parsing.

## Fuzzing the decoders

Two fuzz targets exercise the decode boundary. Campaigns are manual; `make ci` only compiles the targets (`make fuzz-check`), and `make fuzz-smoke` replays each checked-in corpus once:

```sh
cargo +nightly fuzz run wire_decode -- -runs=100
cargo +nightly fuzz run infinite_wire_decode -- -runs=100
```

`infinite_wire_decode` drives the unbounded formula, lasso and partial-valuation readers through canonical serialization and identity computation. Every successful decode must still pass public validation, malformed bytes must be refused, and a formula carrying an unbounded interval `[a,)` must be refused by the bounded readers even when its schema label, profile and clock are rewritten to the bounded forms. `tests/infinite_fuzz_seeds.rs` and `tests/wire_decode_fuzz_seeds.rs` check that the seed corpora reach the intended paths. `make fuzz-check` compiles the targets and audits their dependencies without starting a campaign. Generated inputs are scratch and are not fixtures.
