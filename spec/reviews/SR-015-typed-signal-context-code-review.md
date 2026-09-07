---
id: SR-015
title: Typed signal and caller context code review
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#15 proposed revision e3a2a921; changes from proposed base 6c252717; FR-007; StR-003; TC-027 through TC-033"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-003
    type: references
---

# SR-015: Typed signal and caller context code review

## Summary

The exact proposed revision `e3a2a921` implements the reviewed boundary without
adding a parser, expression language, evaluator, runner, evidence collector,
Python helper, Make target, or runtime Quire/Quoin/contract-IR dependency. One
high invariant defect found in the preceding implementation candidate was fixed
before this review head. No high or medium finding remains open.

## Review coverage

| Surface | Result |
|---|---|
| no-default core | Distinct signal/proposition identities, checked scalar domains, borrowed catalog/context, caller scratch, deterministic lookup, and bound-formula views compile and execute without `alloc`. |
| alloc/serde | Owned declarations and two separate v1 documents round-trip with fixed field order, closed tags, required context fields, bounded sequences, and unknown-field/version refusal. |
| resource behavior | 100,000 signals and bindings validate with O(n log n) exact-name sorting; insufficient scratch and both limit+1 populations fail before acceptance. |
| semantic boundary | Only Boolean signals bind directly; Integer/fixed-Decimal declarations remain available to an external predicate lowerer and are never coerced. |
| compatibility | FormulaDocument, PropositionMapDocument, corpus files, and corpus SHA256SUMS are unchanged; their strict unknown-field tests remain active. |
| traceability | FR-007 and StR-003 are 100% backed; all 31 test-matrix rows and all 36 Rust trace symbols reconcile. |

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-1501 | high | In pre-review candidate `7e6bc05`, public `SignalDomain` enum fields allowed inverted bounds or scale 19 to exist and serialize before catalog validation, unlike checked Interval/SourceSpan values. | `src/signal.rs`, FR-007-AC-2 |
| FND-1502 | low | Caller-context strings intentionally remain opaque: the crate proves completeness and byte preservation, not upstream identifier grammar or provenance truth. | `src/context.rs`, AA-001, AD-001 |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1501 | **FIXED** | `IntegerSignalDomain` and `FixedDecimalSignalDomain` have private fields and checked constructors. `SignalDomain` can contain only checked wrappers, and custom serde conversion rejects malformed wire values before constructing the enum. Programmatic and wire mutation tests cover all three refusals. |
| FND-1502 | **ACCEPTED** | This is the reviewed ownership boundary, not missing validation. The future `quire-contract-ir#57` adapter owns conversion from its typed identities; downstream evidence owns attribution and truth claims. |

## Verification observed

Focused no-default tests, all-feature domain tests, Clippy with warnings denied,
rustdoc with warnings denied, corpus SHA-256 validation, and strict Quire
validation/coverage pass at or after the reviewed changes. The full local gate
passed at exact proposed revision `e3a2a921` after integrating proposed base
`6c252717`; its exact source-state and revision checks, 53-file census, 60/60
grammar result, 68/74 coverage result, 36/36 Rust trace result, and complete
assurance chain all passed. The earlier sandboxed dependency-download failure
was environmental: reinstall of the pinned distribution repaired the partial
virtualenv. Hosted CI was not dispatched.

## Conclusion

The code is ready for closing gap analysis and exact-final-head verification.
