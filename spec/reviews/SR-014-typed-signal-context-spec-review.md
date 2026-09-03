---
id: SR-014
title: Composite review of typed signal and caller context specification
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#15; FR-007; StR-003; NFR-001; NFR-002; AD-001; CAC-001; AP-001; AA-001; MP-001; test matrix"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: reviews
  - target: ix://agent-ix/tl-syntax/StR-003
    type: reviews
---

# SR-014: Composite review of typed signal and caller context specification

## Summary

The dependency, risk, evidence, integrity, scope, failure-domain, and EARS
reviews found no unresolved blocking ambiguity after four specification defects
were corrected. The resulting boundary is deliberately smaller than a contract
IR: tl-syntax validates a closed Boolean/Integer/Decimal signal subset, Boolean
proposition bindings, and opaque caller-supplied identity bytes. It does not
derive IR fields, interpret predicates, validate the truth of provenance, or
own canonical evidence digests.

The scalar choice was checked against `agent-ix/filament-core-data#35` and the
future binding in `agent-ix/quire-contract-ir#54`. Boolean, bounded Integer, and
fixed Decimal are the conservative temporal-input subset. UUID, String,
Timestamp, Duration, Bytes, JsonObject, and future scalar kinds remain explicit
adapter refusals unless a later versioned TL schema adds them. The current
contract-IR Rational type is not silently equated with fixed Decimal.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-1401 | medium | Formula binding originally required a stable order without defining that order or an allocation-free result. It now validates proposition occurrences in formula-node order and returns a borrowed bound-formula view. | FR-007-AC-3, TC-030 |
| FND-1402 | medium | The first draft required an absent standalone context document to round-trip, although a context document necessarily represents presence. Absence is now explicitly an `Option` at consumer APIs; a present document remains complete and strict. | FR-007-AC-4, TC-031 |
| FND-1403 | low | NFR-002 originally implied that tl-syntax owns a digest contract. That exceeded this crate's identity/serialization boundary and was removed; downstream evidence systems may digest the preserved deterministic bytes. | NFR-002-AC-5 |
| FND-1404 | low | “Accepted bytes” could have been misread as requiring deserialization and reserialization to reproduce fixture whitespace. Compatibility now means checked-in legacy bytes are unchanged, still accepted with prior outcomes, and their closed v1 documents accept no new fields. | FR-007-AC-5, TC-032 |
| FND-1405 | low | Strict Quire coverage cannot currently classify planned rows because the installed traceability declaration expects `Status` while the validated TestMatrix archetype requires `Coverage Status`. | `agent-ix/quire-contract-ir#21`, `spec/test-matrix.md` |
| FND-1406 | medium | The reviewed draft asked formula binding to reject a non-Boolean binding that a validated catalog already makes unrepresentable. Catalog validation now owns that refusal; formula binding owns deterministic first-missing resolution. | FR-007-AC-2, FR-007-AC-3, TC-029, TC-030 |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1401 | **FIXED** | FR-007 and TC-030 name formula-node order, first refusal, and the borrowed bound-formula result. |
| FND-1402 | **FIXED** | FR-007 separates optional consumer use from the complete standalone context value and wire document. |
| FND-1403 | **FIXED** | NFR-002 promises deterministic order and serialized bytes only. |
| FND-1404 | **FIXED** | FR-007-AC-5 and TC-032 distinguish immutable fixture bytes from semantic decode compatibility. |
| FND-1405 | **DEFERRED** | The shared module contradiction is already tracked by `agent-ix/quire-contract-ir#21`; this repository shall not fork or locally patch Quire's traceability model. Planned rows remain visibly marked and become backed during implementation. |
| FND-1406 | **FIXED** | FR-007-AC-2 retains the non-Boolean catalog refusal; FR-007-AC-3 and TC-030 now test only states reachable from a validated catalog. |

## Boundary and implementation obligations

- `SignalId` never aliases `PropositionId`; direct proposition bindings target
  declared Boolean signals only.
- Catalog and binding arrays are bounded and strictly increasing. Exact UTF-8
  byte equality defines signal-name uniqueness; no normalization is implied.
- Fixed Decimal means an `i64` coefficient range at scale 0 through 18, not an
  arbitrary rational or floating-point representation.
- The no-allocation surface owns borrowed validation and lookup. Owned vectors,
  strings, and strict serde documents remain feature-gated.
- The two new wire identities are separate from formula-v1 and
  proposition-map-v1. Existing bytes and APIs are not extended in place.
- Every refusal named by FR-007-AC-2 and FR-007-AC-4 requires a positive control
  and a mutation probe during implementation. Hosted CI remains manual-only.

## Review conclusion

FR-007 and StR-003 are sufficiently bounded for planning and implementation.
The implementation may not add a FRETish parser, predicate expression language,
contract-IR dependency, evidence collector, test runner, or local replacement
for shared Quire/Quoin/Engineering Assurance machinery.
