---
id: SR-065
title: "Gap analysis — strict signal-catalog and proposition-map owner readers"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/tl-syntax#61 against FR-007-AC-5 and PLAN-010 Task-006 dependency"
review_set: subset
---

# Gap analysis — strict signal-catalog and proposition-map owner readers

## Summary

Traced every `tl-syntax#61` deliverable from FR-007-AC-5 and PLAN-010 Task-006
through production symbols, exact schema bytes, and executing Rust tests.

## Verdict

**COMPLETE for the owner dependency.** No open implementation or traceability
gap remains in the `tl-syntax#61` scope. Completion of the consuming
quire-contract-ir bridge remains owned by `quire-contract-ir#70`.

## Requirement-to-evidence map

| Obligation | Production evidence | Test evidence | Status |
|---|---|---|---|
| Publish signal-catalog/v1 schema bytes | `SIGNAL_CATALOG_V1_SCHEMA` embeds `spec/signal-catalog-v1.schema.json` | `public_signal_catalog_reader_and_schema_are_strict_owner_artifacts` | complete |
| Publish proposition-map/v1 schema bytes | `PROPOSITION_MAP_V1_SCHEMA` embeds the unchanged corpus schema | `public_proposition_map_reader_and_schema_are_strict_owner_artifacts` | complete |
| Strict owner byte readers | `SignalCatalogDocument::from_json_bytes`; `PropositionMapDocument::from_json_bytes` | duplicate, trailing, version, shape, and semantic refusals under TC-029/032 | complete |
| Stable resource ceilings | `MAX_TL_DOCUMENT_BYTES`; `MAX_TL_DOCUMENT_DEPTH`; existing catalog limits; `MAX_PROPOSITION_MAP_ENTRIES` | byte, depth, signal/binding, and proposition population boundaries | complete |
| Preserve v1 compatibility and no_std core | existing constructors/serde identities unchanged; APIs gated by `serde` | corpus digest check; feature-boundary test; existing round trips | complete |
| Owner-only vocabulary | typed documents and schemas remain in tl-syntax; no downstream/native source types | source review and dependency graph | complete |

All changed acceptance language is backed by executing TC-028/029/032 symbols.
No new executable Python path, test-local semantic model, stub, or tautological
implementation was introduced.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6501 | medium | Both target schemas are exact public owner bytes and both target documents expose strict public readers. | `SIGNAL_CATALOG_V1_SCHEMA`; `PROPOSITION_MAP_V1_SCHEMA`; `from_json_bytes` |
| FND-6502 | medium | Every issue #61 refusal class is backed by a production-path test, including byte/depth/population ceilings and duplicate/trailing JSON. | TC-029; TC-032 |
| FND-6503 | low | The only remaining work is the explicitly separate consuming bridge in `quire-contract-ir#70`; no owner feature is deferred. | PLAN-010 Task-006 |
