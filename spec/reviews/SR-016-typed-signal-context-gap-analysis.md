---
id: SR-016
title: Typed signal and caller context closing gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/tl-syntax#15 candidate bda0909; FR-007; StR-003; NFR-001; NFR-002; PLAN-003; downstream temporal handoff"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: reviews
  - target: ix://agent-ix/tl-syntax/StR-003
    type: reviews
---

# SR-016: Typed signal and caller context closing gap analysis

## Summary

No implementation gap remains inside tl-syntax#15. The new public contract is
sufficient for downstream TL crates to consume one canonical catalog/context
model without sibling copies. Remaining work is either explicitly downstream,
owned by the future IR/FRETish adapter, or an existing shared-tool limitation.

## Requirement census

| Requirement | Evidence | Gap |
|---|---|---|
| FR-007-AC-1 | checked domains, borrowed/owned round trips, exact UTF-8 identity, accepted maximum populations | none |
| FR-007-AC-2 | typed constructor/catalog errors, single-field wire mutations, limit+1 decoding | none |
| FR-007-AC-3 | first-missing node-order test and successful borrowed BoundFormula lookup | none |
| FR-007-AC-4 | complete exact context, explicit Option absence, all missing/empty/oversized/unknown/inverted forms | none |
| FR-007-AC-5 | unchanged corpus digest manifest, legacy decode tests, closed unknown-field tests | none |
| StR-003 | typed Boolean resolution plus exact caller identity/span preservation | none |
| NFR-001/NFR-002 extensions | no-default feature test, all feature combinations, stable compact JSON snapshots | none |

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-1601 | low | tl-parse does not yet bind parsed propositions to this catalog; tl-rewrite and tl-mltl do not yet propagate it. | `tl-parse#20`, `tl-rewrite#21`, `tl-mltl#24` |
| FND-1602 | low | The TL subset does not represent UUID, String, Timestamp, Duration, Bytes, JsonObject, or arbitrary Rational signals. | SR-014, `quire-contract-ir#57` |
| FND-1603 | low | tl-syntax does not derive an IR catalog, parse FRETish, validate upstream identity truth, or issue certification/qualification decisions. | AD-001, AA-001, `quire-contract-ir#54/#57` |
| FND-1604 | low | Installed Quire still reports the shared `Status` versus `Coverage Status` declaration contradiction and an empty inspection archetype. | `agent-ix/quire-contract-ir#21` |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1601 | **DEFERRED** | The already-created dependency chain owns the consumers. No sibling implementation belongs in this repository. |
| FND-1602 | **ACCEPTED** | SR-014 fixes the v1 subset. The future adapter must reject unsupported inputs or explicitly predicate-lower them; a later TL schema version requires its own review. |
| FND-1603 | **ACCEPTED** | These responsibilities remain with `quire-contract-ir#54/#57`, downstream evidence producers, and human authorities. |
| FND-1604 | **DEFERRED** | All actual criteria and 31/31 matrix rows are backed, and no local traceability implementation is introduced. |

## Downstream handoff contract

- Core: `SignalId`, `IntegerSignalDomain`, `FixedDecimalSignalDomain`,
  `SignalDomain`, `SignalDeclaration`, `PropositionBinding`, `SignalCatalog`,
  `SignalIter`, `BoundFormula`, `RequirementContext`, and typed error enums.
- Construction: borrowed catalog validation requires mutable `u32` scratch with
  one slot per declaration and does not retain it; owned validation supplies
  scratch internally under `alloc`.
- Interchange: `SignalCatalogDocument` uses
  `tl-syntax.signal-catalog/v1`; `RequirementContextDocument` uses
  `tl-syntax.requirement-context/v1`. Both require `alloc`; serde requires the
  existing `serde` feature.
- Binding: proposition bindings are strictly ordered, target declared Boolean
  signals, and a formula is checked in node-table order before a BoundFormula is
  returned.
- Compatibility: formula-v1, proposition-map-v1, and tl-syntax-corpus/v1 are
  unchanged and must not be extended by downstream code.

## Conclusion

The candidate has no unresolved high or medium gap. After exact-final-head local
verification and reviewer clearance, tl-syntax#15 can land and unblock the
downstream dependency chain. Hosted CI remains manual-only.
