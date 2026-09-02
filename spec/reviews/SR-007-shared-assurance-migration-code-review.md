---
id: SR-007
title: Shared assurance migration code review
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#9 at 38a410a; the v0.1 substrate inherited from PR #6; FR-006 and the deletion of the local evidence framework"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-002
    type: reviews
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# SR-007: Shared assurance migration code review

## Summary

This change carries two things at once. It is the v0.1 substrate that lived only
on PR #6 — the no_std MLTL AST, inclusive intervals, spans, stable identities,
the semantic-profile model, the bounded wire decoder, and the shared temporal
corpus — and it is the migration of that repository's QA machinery onto the
released Engineering Assurance, Quire, and Quoin contracts. It supersedes PR #6.

The domain half is unchanged from the base: `git diff cb7bedb -- src/ corpus/`
is empty. The review of that half is therefore a review of PR #6's thirteen
rounds, and the question asked here is not "is it correct" but "did the
migration quietly undo anything those rounds fixed". It did not, and each of the
twelve closed classes is checked below.

The migration half replaces 4,059 lines of generic evidence machinery with three
gates that delegate: pins to `engineering_assurance.compatibility`, retained
bytes to `map_pgm01_bytes`, and everything dynamic to Quoin's change-assurance
surface. The most substantive finding is that the compatibility mapping does not
cover this repository's retained schema family at all, and the honest answer is
a refusal rather than a mapping.

## Verdict

**CONDITIONAL.** Six findings, one high, three medium, two low. The high was
found by a mutation probe during this review and is fixed in `38a410a`; the
remainder are dispositioned in SR-009.

## Gates run at 38a410a

| Gate | Result |
| --- | --- |
| `make ci` | exit 0 |
| `make spec` | exit 0 |
| Rust tests | 29 passed, 0 failed, 0 ignored (10 unit, 1 feature-boundary, 10 integration, 7 shared-assurance, 1 doc) |
| Rust tests at MSRV 1.75 | 29 passed, 0 failed, 0 ignored |
| corpus conformance | 22 rows, 8 fixtures, 3 declared rejection reasons matched by typed errors |
| corpus oracle | 19 rows, all pass |
| feature boundary | 5 entries: empty default graph + 4 feature combinations |
| shared pins | 4/4 compatible, 0 artifact mismatches, 0 mirror references |
| compatibility census | 15/15 cases, 23 retained envelopes, 1232 evidence files read, 0 bytes moved |
| compatibility mutation probes | 5/5 detected |
| assurance chain | 13 scenarios, 6 controls, 6 adapter probes, all matched |
| `git diff cb7bedb -- evidence/ schemas/ corpus/ src/` | empty |
| hosted CI | not dispatched |

## Domain behaviour inherited from PR #6, re-checked

Each row is a class PR #6 closed. "Unchanged" means byte-identical to `cb7bedb`;
"still red on deletion" means the guarding test was confirmed to fail when the
guard is removed.

| Class | State |
| --- | --- |
| Hand-written `Deserialize for Node` over `NodeWire` with `deny_unknown_fields` | Unchanged |
| Twelve-operator round trip with distinct `left`/`right` operands and tag assertion | Unchanged |
| Private document fields with accessors; validation via `try_from` during decode | Unchanged |
| Inverted intervals and spans rejected; `cardinality` `None` documented | Unchanged |
| `#![forbid(unsafe_code)]`; no `as` casts outside tests | Unchanged |
| `MAX_FORMULA_DOCUMENT_NODES` on both the wire decoder and owned construction, with clamped `with_capacity` | Unchanged |
| All four public validation error types `#[non_exhaustive]` | Unchanged |
| `PropositionMapDocument::validate` via `BTreeMap`, duplicate/non-increasing/empty names rejected | Unchanged |
| Corpus: `kind`-discriminated draft-07 schema, derived horizon oracle, derived closed-trace evaluator, per-fixture declared rejection reason | Unchanged, and now cross-checked (see FND-703) |
| Empty-trace convention stated in FR-005 | Unchanged |
| `no_std` × 4 feature combos, zero default dependencies, MSRV 1.75, README doctest | Unchanged; the feature matrix now also emits a structured result |
| `cargo deny` all four lanes | Unchanged |

The generic-machinery classes PR #6 fixed — false-green Make execution controls,
PATH-shadowed tools, uncompiled test census, disabled per-record validators — are
not re-checked, because the machinery they guarded no longer exists. The two
class-level lessons that outlive it are carried: "skipped is not passed" is now
`NFR-002-AC-3` discharged by the twelve-outcome test, and "no text assertion as
the load-bearing control" is why every rejection reason in the conformance runner
comes from a typed error rather than a message.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-701 | high | Collapsing the adapter's outcome map to `pass` left every gate green; the vacuous probe built its skipped entries by hand rather than through the adapter, so the mapping it was supposed to exercise was never exercised | `scripts/assurance_chain.py:763` | implementation-bug-despite-evidence |
| FND-702 | medium | The compatibility census reported `source_digest` without comparing it, so a view that misattributed an answer to the wrong record would not be noticed. Found by the `drop-source-identity` probe | `scripts/legacy_evidence_view.py:210` | missing-requirement |
| FND-703 | medium | The corpus accept/reject oracle existed only in Python, a re-implementation of crate semantics. PR #6 raised this twice (FND-107, FND-R3-002) and it survived the migration unaddressed | `examples/corpus_conformance.rs` | correct-requirement-no-evidence |
| FND-704 | medium | `map_pgm01_bytes` at the pinned release covers `quire.pgm01-evidence` v1 and v2 only. All 23 retained envelopes here are `quire.derivation-evidence/v1` and are refused. A census across the campaign found 142 such envelopes in six of the eight repositories; `quire-contract-ir` is the only one that retained PGM-01 records, which is why Wave 0 did not meet this | `assurance/pins.json`, `agent-ix/engineering-assurance#21` | wrong-requirement |
| FND-705 | low | Altering a retained evidence byte between runs is not detected by the compatibility gate, whose census is scoped to a single run. Detected by `git diff` | `scripts/legacy_evidence_view.py:245` | correct-requirement-no-evidence |
| FND-706 | low | `[status-column-matches-nothing]` remains open: the `TestMatrix` archetype asserts a `Coverage Status` header and the traceability declaration is configured for `Status`. The two cannot both be satisfied from this repository | `spec/test-matrix.md`, `agent-ix/quire-contract-ir#21` | wrong-requirement |

## Mutation probes

Ten probes, each removing or weakening exactly one load-bearing check. A gate
never observed to fail is indistinguishable from a gate that does not run.

| Probe | Result |
| --- | --- |
| Doctored corpus `expected_error` | conformance runner exit 1 |
| Doctored declared horizon | corpus oracle exit 1 |
| Altered retained evidence byte | **not detected by the gate**; detected by `git diff` — see FND-705 |
| Wrong consumed-artifact digest in `pins.json` | pins gate exit 1 |
| `npm.ix` added to a requirement | pins gate exit 1 |
| Producer input removed | chain exit 2, naming `make assurance-inputs` |
| Adapter protocol check removed | chain exit 1 |
| Adapter outcome map collapsed to `pass` | **not detected**, then fixed as FND-701; now exit 1 |
| Frozen schema deleted | TC-026 fails |
| Empty-stream refusal removed | chain exit 1 |

Plus the five in-tree compatibility probes (`collapse-non-success-states`,
`repair-unreadable-outcome`, `accept-refused-schema`, `unbind-tamper-digest`,
`drop-source-identity`), 5/5 detected, run by `make compat-view` on every CI run.

## What this review does not claim

It does not claim the crate is correct, only that this change does not alter it.
It does not claim the retained records are valid — nothing in this repository has
that authority any more, which is the point of the migration. It does not claim
hosted CI passes; hosted CI was not dispatched and remains manual-only.
