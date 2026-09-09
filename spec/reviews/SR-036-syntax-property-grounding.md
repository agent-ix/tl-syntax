---
id: SR-036
title: "spec-correctness — tl-syntax property and wire-fuzz scope"
type: SpecReview
analysis: spec-correctness
scope: "spec/requirements, spec/test-matrix.md, src, tests, and serde wire entry points"
review_set: base
---

## Summary

Quire 0.31.0 classified 39 binding criteria: 14 extractable and 25 concrete
examples. The existing suite already binds every criterion through its named
test case; this review distinguishes generated domains from corpus, compile,
and assurance demonstrations. Two generated caller-context properties now
cover the previously example-only complete-context and provenance boundaries.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3601 | medium | The configured functional-coverage status column is `Status`, but the authored table is headed `Coverage Status`; Quire consequently refuses status classification. The traceability repair on PR #23 remains the prerequisite for a final coverage reconciliation. | TM-001, PR #23 |
| FND-3602 | low | **Closed by SR-037:** the reviewed `FR-004-AC-5` / TC-036 obligation now binds the two public document decoders. The target is manual-only; a campaign remains a separately measured scope. | FR-004-AC-5, TC-036, SR-037 |
| FND-3603 | low | The 25 example-shaped criteria are correctly retained as their existing fixed corpus, malformed-wire, compile, or assurance demonstrations. They are not recast as generated properties merely to change the classifier census. | FR-002, FR-005, FR-006, NFR-001 |

## Classifier census

| Dimension | Counts |
| --- | --- |
| Criteria | 39 |
| Extraction | 14 extractable; 25 not-extractable |
| Property | 10 universal; 2 round-trip; 1 ordering; 1 invariant; 25 example |
| Harness | Existing Rust property harness in `src/syntax.rs`, `tests/integration.rs`, and `tests/typed_signal_context.rs` |

## Extractable-criterion grounding ledger

| Criterion | Route | Grounding |
| --- | --- | --- |
| FR-001-AC-2 | already covered | Arbitrary `u32` endpoint pairs; constructor result is equivalent to ordered bounds; `src/syntax.rs:639` tests the stated rejection in `spec/requirements/FR-001-inclusive-intervals.md:38`. |
| FR-003-AC-3 | existing witness | The closed v1 document schema has a finite required-field shape; the missing-profile witness is asserted by TC-008 rather than fabricating a broad malformed-document generator. |
| FR-004-AC-1 | already covered | Generated valid proposition/interval/profile documents are encoded then decoded for exact equality in `tests/integration.rs:73`, grounded by `spec/requirements/FR-004-versioned-serialization.md:43`. |
| FR-005-AC-2 | static demonstration | The domain is the finite checked-in corpus and its manifest-declared outcomes; TC-013 independently derives the horizon and replays every fixture. |
| FR-006-AC-1 | static demonstration | The observable is an adopted shared compatibility release and artifact set, not a local domain function. TC-021 remains the correct verification method. |
| FR-006-AC-5 | static demonstration | The finite twelve-outcome assurance scenario/control suite is its stated domain; TC-025 is the required demonstration. |
| FR-006-AC-6 | static demonstration | The predicate is a repository live-source census, not a generated library input; TC-026 is the named static verification. |
| FR-006-AC-7 | static demonstration | The predicate is a version-control population partition and fail-closed enumeration; TC-034 is the named integration verification. |
| FR-007-AC-4 | emitted | Bounded nonempty identity strings and ordered offsets construct a context, copy to its owned form, encode/decode, and recover the same context. `tests/props_fr_007.rs`. |
| NFR-002-AC-1 | existing witness | The requirement concerns repeating comparison of an identical value, a singleton-domain witness; TC-005 retains the direct ordering checks. |
| NFR-002-AC-2 | static demonstration | The domain is the finite checked-in serialized corpus; TC-014 checks the declared v1 identities. |
| StR-002-VC-1 | existing witness | The supported schema/profile vocabulary is closed and finite; TC-008 verifies the exchanged v1 identity without pretending this is an unbounded property. |
| StR-003-VC-1 | existing demonstration | The existing catalog/binding tests cover Boolean resolution and non-Boolean refusal at the public boundary. A future generated catalog family remains optional follow-up, not a prerequisite for this wire-decoding scope. |
| StR-003-VC-2 | emitted | Bounded nonempty identity strings and an ordered span survive borrowed-to-owned conversion exactly. `tests/props_fr_007.rs`. |

## Scope boundary

The generated checks use the repository's existing Rust test dependency and
public context APIs. They do not add a generic runner, evidence representation,
qualification claim, concurrent model, mutation campaign, or formal-proof
claim. The next formal-methods decision remains a separately scoped follow-up;
the production API and these generators are independent of any particular
prover.
