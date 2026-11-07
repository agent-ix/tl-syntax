# API reference

Everything is re-exported at the crate root. The module paths below are the public ones: `formula::{graph, profile, infinite, trace, liveness, document}`, `signal::{domain, catalog, binding, document}` and `contracts::{limits, manifest}`. Items marked `alloc` or `serde` need that feature. Errors are `#[non_exhaustive]`, so match them with a wildcard arm.

## formula::graph

| Item | Purpose |
| --- | --- |
| `Formula<'a>` | Validated borrowed view. `new(profile, root, &nodes)`, `profile()`, `root()`, `nodes()`, `node(id)`. |
| `Node` | `new(kind)`, `with_span(kind, span)`; public fields `kind` and `span`. |
| `NodeKind` | The 17 operators and constants. `temporal_family()`, `past_operator()`, `operands()`. |
| `NodeId`, `PropositionId` | Transparent `u32` identities. |
| `Interval` | Inclusive `[start, end]` of `u32`. `new`, `start`, `end`, `cardinality` (`None` for `[0, u32::MAX]`), `contains`. |
| `SourceSpan` | Half-open byte span. `new`, `start`, `end`, `len`, `is_empty`. |
| `FormulaError` | `RootOutOfRange`, `TooManyNodes`, `DocumentNodeLimitExceeded`, `DocumentDepthLimitExceeded`, `OperandNotPreceding`, `ProfileIncompatibleNode`, `FormulaV1ProfileUnsupported`, `InfiniteProfileRequiresUnboundedEdition`, `FormulaV1NodeUnsupported`. |
| `IntervalError`, `SourceSpanError` | Inverted bounds. |
| `MAX_FORMULA_DOCUMENT_NODES`, `MAX_FORMULA_DOCUMENT_DEPTH` | 100,000 nodes and depth 4,096. |

## formula::profile

| Item | Purpose |
| --- | --- |
| `SemanticProfile` | Four profiles. `ALL`, `as_str()`. |
| `TemporalFamily` | `Future` or `Past`. |
| `PastOperatorKind` | `Once`, `Historically`, `StrongPrevious`, `Since`, `Triggered`. `ALL`, `semantic_name()`, `arity()`. |
| `OperatorArity` | `Unary` or `Binary`. |
| `PAST_OPERATORS_V1` | `tl-syntax.past-operators/v1`. |

## formula::document (`alloc`; readers need `serde`)

| Item | Purpose |
| --- | --- |
| `FormulaDocument` | Owned document. `new`, `new_v2`, `from_formula`, `from_formula_v2`, `validate`, `to_v2`, `try_to_v1`, `semantic_view`, `from_json_bytes`, `canonical_json_bytes`, `content_identity` and accessors. |
| `FormulaSchemaVersion` | `V1` or `V2`. |
| `FormulaConversionError` | Why a document cannot convert to v1. |
| `SemanticFormulaDocument` | Span-free serialization view. |
| `StrictDocumentReadError` | Reader failures. |
| `FORMULA_V1_SCHEMA*`, `FORMULA_V2_SCHEMA*` | Schema text, bytes, SHA-256 (`serde`). |

## formula::infinite

| Item | Purpose |
| --- | --- |
| `InfiniteFormula<'a>`, `InfiniteNode`, `InfiniteNodeKind` | Borrowed graph of the unbounded edition. |
| `TemporalInterval`, `UnboundedInterval` | `[a,b]` or `[a,)`. `TemporalInterval` implements `FromStr` and refuses non-canonical spellings. |
| `InfiniteClock`, `EVENT_POSITION_CLOCK` | The only clock, `event_position`. |
| `select_infinite_profile(...)` | Selects the exact profile from supplied identities; it never infers one. |
| `lower_infinite_future(...)` | Three-node lowering of `W` and `M` for the unbounded edition. |
| `InfiniteFormulaDocument` (`alloc`) | Owned document with strict reader, canonical bytes and identity. |
| `FORMULA_UNBOUNDED_V1`, `INFINITE_TRACE_PROFILE`, `LIVENESS_CAPABILITY_V1` | Identifiers. |

## formula::trace (`alloc`; readers need `serde`)

`PartialValuation`, `PartialValue`, `ValuationEntry`, `TraceObservation`, `LassoTraceDocument`, `FairnessPremisesDocument`, their error types (`PartialValuationError`, `LassoTraceError`, `FairnessPremisesError`, `FairnessReadError`) and the identifiers `PARTIAL_VALUATION_V1`, `LASSO_TRACE_V1`, `FAIRNESS_PREMISES_V1`.

## formula::liveness (`alloc`)

`settle_liveness(formula, subject, backend)` routes a canonical infinite formula to an optional `LivenessBackend` and returns a `LivenessSettlement`. The subject is a `LivenessSubject` of kind `LassoTrace`, `FinitePrefix` or `Model`. The disposition is one of `Proved`, `Refuted`, `Inconclusive`, `Unsupported`, `ResourceIncomplete` or `Failed`.

- With no backend, the result is `Unsupported` and carries `LIVENESS_CAPABILITY_V1` as its warning.
- A `Model` subject needs a backend whose `supports_model_proof()` is true; otherwise the result is `Unsupported`.
- A backend answer of `Proved` for a `FinitePrefix` becomes `Failed`, because a finite prefix cannot prove an unbounded claim.

## signal

| Item | Purpose |
| --- | --- |
| `SignalDomain` | `Boolean`, `Integer(IntegerSignalDomain)`, `FixedDecimal(FixedDecimalSignalDomain)`. |
| `IntegerSignalDomain` | `new(min, max)`, inclusive `i64` bounds. |
| `FixedDecimalSignalDomain` | `new(min_coefficient, max_coefficient, scale)`, scale 0 to 18. |
| `SignalDeclaration` | `new(id, name, domain)` and accessors. |
| `SignalCatalog<'a>` | `new(signals, bindings, scratch)`, `signals()`, `signal(id)`, `bindings()`, `signal_for_proposition(p)`, `signal_count()`, `bind_formula(formula)`. |
| `PropositionBinding` | `new(proposition, signal)`. |
| `BoundFormula` | `formula()`, `catalog()`, `signal_for_proposition(p)`. |
| `SignalCatalogError`, `SignalDomainError`, `FormulaBindingError` | Typed refusals. `SignalCatalogError` covers limits, identity order, empty, long and duplicate names, missing binding targets and non-Boolean targets. |
| `SignalCatalogDocument`, `OwnedSignalDeclaration` (`alloc`) | Owned catalog document. `from_catalog`, `validate`, readers. |
| `PropositionMapDocument`, `PropositionEntry`, `PropositionMapError` (`alloc`) | Proposition identities and names. |
| `MAX_SIGNAL_CATALOG_SIGNALS`, `MAX_SIGNAL_CATALOG_BINDINGS`, `MAX_SIGNAL_NAME_BYTES`, `MAX_PROPOSITION_MAP_ENTRIES` | 100,000, 100,000, 255 and 100,000. |

## Requirement context

`RequirementContext::new(requirement_id, requirement_revision, clause_id, anchor, source_span)` refuses empty fields and fields over `MAX_REQUIREMENT_CONTEXT_FIELD_BYTES` (1,024). `RequirementContextDocument` (`alloc`) is the owned form. Errors: `RequirementContextError`, `RequirementContextField`.

## Derived future operators

| Item | Purpose |
| --- | --- |
| `FutureLoweringRequest` | Raw, unchecked request. `lower()` returns `Result<FutureLowering, FutureLoweringRefusal>`. |
| `FutureLowering` | `nodes()`, `node_ids()`, `root()`, `report()`. |
| `FutureLoweringReport` | Identities, operands, generated range and spans. |
| `FutureLoweringRefusal` | One variant per axis of the check; `code()` is a stable snake_case name and `axis()` the `FutureLoweringAxis`. |
| `FutureKind` | `WeakUntil` (`W`) and `StrongRelease` (`M`). |
| `UnsupportedFutureKind` | Recognized spellings the profile refuses: `X`, `Y`, `O`, `H`, `S`, `T`. |
| `RawBounds` | Pair of `u64` endpoints, unchecked. |
| `FUTURE_LOWERING_NODE_CHARGE` | 3 appended nodes. |

The refusal order is fixed: request identity, operator profile, kind, semantic profile, profile agreement, interval, operands, spans, then node budget.

## contracts

| Item | Purpose |
| --- | --- |
| `SyntaxArtifactLimits` | Reader limits. `OWNER_MAXIMA`, `constrained()`, `Default`. |
| `CONTENT_IDENTITY_DOMAIN_V1` (`serde`) | Domain prefix of content identities. |
| `validate_past_profile_implementation`, `PastProfileManifestError`, `PAST_PROFILE_IMPLEMENTATION_V1`, `PAST_PROFILE_MANIFEST_MAX_BYTES` (`serde`) | Manifest gate. |

## Crate constants

| Constant | Value |
| --- | --- |
| `CORPUS_REVISION` | `tl-syntax-corpus/v1` |
| `PAST_HISTORY_CORPUS_V1` | `tl-syntax.past-history-corpus/v1` |
| `CORPUS_DIR` | Path of this crate's `corpus/` directory, resolved at compile time. |

For full signatures run `cargo doc --all-features --no-deps --open`.
