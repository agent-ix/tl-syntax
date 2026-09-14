---
id: SR-071
title: "Rust review — PLAN-010 ecosystem closure"
type: SpecReview
analysis: code-review
scope: "candidate 686bf13 TC-057 property-test change"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/TC-057
    type: reviews
---

# Rust review — PLAN-010 ecosystem closure

## Summary

Applied the Rust review checklist to the only Rust change in candidate
`686bf13`: the bounded arbitrary-byte FormulaDocument property. Production Rust
is unchanged.

## Verdict

**PASS after remediation.** No scoped Rust finding remains.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-7101 | low | **FIXED:** the first unconditional `prop_assert!` embedded a closure body that the macro interpreted as an invalid format string. Hoisting both predicates into named booleans produces clear diagnostics, compiles on the Rust 1.75 MSRV, and retains an unconditional oracle. | `tests/past_formula_v2.rs` | implementation-bug-despite-evidence |

## Rust checklist

| Area | Result |
|---|---|
| Panic/unsafe | No production change; arbitrary input still reaches the real serde reader and any unwind fails the property. |
| Ownership/allocation | The test borrows the decoded result and allocates no copy of an accepted document. |
| Compatibility | `Option::map_or` remains compatible with the declared Rust 1.75 MSRV. |
| Mock/stub boundary | The property invokes the real public `FormulaDocument` deserializer and validator with no mock or duplicated parser. |
| Traceability | TC-057 and FR-013-AC-1 remain attached to the executing property symbol. |

## Gates

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- TC-057 repaired property: pass.
- Complete non-qualification Rust suite: 86 passed, 0 failed.
- `make spec`: structural validation pass; 214/214 documents grammar-clean and
  no new status lie. Cross-repository symbols remain explicitly non-federated.

The separate shared-assurance tests still report the already-paused
qualification-tool and source-census drift. This PR neither changes nor claims
that lane.
