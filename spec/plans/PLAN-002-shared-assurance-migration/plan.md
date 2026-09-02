---
id: PLAN-002
title: Shared assurance migration
type: Plan
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: references
  - target: ix://agent-ix/tl-syntax/FR-005
    type: references
---

# Shared assurance migration

## Objective

Adopt the released Engineering Assurance, Quire, and Quoin contracts for
`agent-ix/tl-syntax#9`, keep every piece of MLTL domain behaviour this repository
owns, and remove the generic evidence machinery that the shared contracts now own.

## Base and supersession

This plan's change is branched from `feat/tl-syntax-v0.1` at `cb7bedbe162d`, not
from `main`.

`main` contains a stub `src/lib.rs`. The no_std MLTL AST, the semantic-profile
model, the wire documents, and the shared temporal corpus — the entire body of
work the migration exists to preserve — exist only on the open PR #6 branch.
Migrating from `main` would have migrated an empty crate. The change therefore
carries both the v0.1 substrate and the migration, and **supersedes PR #6**.

## Dependency DAG

```text
accepted shared release pins (engineering-assurance#8, #10)
  -> keep/delete/replace inventory
  -> structured domain producers (corpus conformance, corpus oracle, feature boundary)
  -> Quire static export + Quoin record/attestation/intake/receipt
  -> read-only compatibility view over retained bytes
  -> dual run at the same candidate revision
  -> deletion of the generic machinery
  -> reviews, PR, merge
```

## Task File Mapping

| Task | Scope | Exit evidence |
|---|---|---|
| Task-001 | Inventory and pins | `make pins` classifies four components through the packaged matrix; the inventory is recorded in this plan |
| Task-002 | Structured producers and shared intake | `make assurance` runs the chain, controls, and adapter probes green |
| Task-003 | Dual run and deletion | the dual-run table below, then deletion in its own commit |

## Keep / replace / delete / defer inventory

### KEEP — domain behaviour this repository owns

| Item | Why |
|---|---|
| `src/syntax.rs`, `src/document.rs`, `src/lib.rs` | The no_std MLTL AST, inclusive intervals, spans, stable identities, the semantic-profile model, the wire decoder, and the 100 000-node bound |
| `corpus/` in full | The shared temporal conformance corpus every sibling repository pins |
| `scripts/validate_corpus.py` | The corpus semantic oracle: derived horizons, derived closed-trace outcomes, declared rejection reasons |
| `scripts/check_default_dependencies.py` | The NFR-001 feature-boundary and empty-default-dependency gate |
| `scripts/check_unsafe_comments.sh` | The unsafe-code policy guard |
| `tests/integration.rs`, `tests/feature_boundary.rs` | Domain tests, including the twelve-operator round trip and the allocation bound |
| `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, MSRV | Safety and supply-chain policy |
| `evidence/` in full, byte for byte | Immutable retained records; the verifier is what goes, not the record |
| `schemas/tl-syntax-evidence-*.schema.json` | **Frozen**, not deleted: retained envelopes name both by path and SHA-256 |

### REPLACE — generic machinery now owned upstream

| Item | Replaced by |
|---|---|
| `scripts/build_evidence_envelope.py`, `finalize_collection.py`, `collect_evidence.sh` | `quoin change-assurance seal-record`, `seal-attestation`, `intake`, `receipt` |
| `scripts/verify_evidence.sh`, `verify_evidence_tree.py`, `verify_evidence_manifest.py`, `evidence_profile.py`, `check_evidence_shell_contract.py` | `scripts/legacy_evidence_view.py`, which calls `engineering_assurance.verification_semantics.map_pgm01_bytes` and implements no mapping of its own |
| `scripts/tool_identity.py`, `tools.lock` | The pinned release, classified by `engineering_assurance.compatibility` |
| `scripts/check_traceability_coverage.py` | `quire coverage --scope . --json` |
| `scripts/rust_test_census.py` | Nothing. It had no caller at the base revision |
| `scripts/check_failure_propagation.py`, `test_failure_propagation.py`, `test_shell_gates.py`, `run_policy_tests.py` | Nothing. Make is not a trust root, so a gate policing Make's own execution controls has no object |
| `scripts/validate_json_schema.py`, `test_json_schema_gate.py` | Quoin's packaged FR-063/FR-064/FR-065 schemas, applied by Quoin |
| `tests/evidence_contract.rs` | `tests/no_local_evidence_framework.rs`, which asserts the machinery is gone rather than asserting it is wired up |
| `evidence/ANCHORS` as a live integrity boundary | Quoin's retention. The file itself is retained, unchanged, as a historical record |

### DELETE — after the dual run, in its own commit

Every REPLACE row's left column, plus `scripts/test_evidence_tool.py`,
`scripts/test_evidence_tree.py`, `scripts/test_traceability_gate.py`, and
`scripts/test_corpus_gate.py`'s policy-runner wiring.

### DEFER

| Item | Issue | Why it does not affect migration integrity |
|---|---|---|
| Downstream pins on `740182f13b84` | `agent-ix/tl-syntax#8` | A landing-sequence constraint on how this change merges, not a property of what it changes. Handled by merging with a true merge commit and retaining the source branch |
| `engineering-assurance` v0.2.0 lacks its own acceptance record | `agent-ix/engineering-assurance#20` | The gate reports the acceptance state and never reads an absent field as approval, so the pin's behaviour is fully determined either way |
| The shared mapping does not cover `quire.derivation-evidence/v1` | `agent-ix/engineering-assurance#21` | The mapping's refusal is a correct, explicit, reported state. Writing a local mapping to avoid it is the thing this migration removes |

## Dual run at the same candidate revision

Both paths were run at `cb7bedbe162dd3534cd6d859cb24ce655bb4ddf3`, before any
generic machinery was deleted.

| Path | Result |
|---|---|
| `make ci` (old path, whole gate) | **exit 2**. 23 of 24 gate commands passed; `verify-evidence` failed |
| `bash scripts/verify_evidence.sh` (old path, isolated) | **exit 1** — `evidence qualification census failed: active evidence source revision differs from the current source head` |
| `python3 scripts/assurance_chain.py` (new path) | exit 0 |
| `python3 scripts/legacy_evidence_view.py` (new path) | exit 0 — 15/15 cases, 23 retained envelopes read, 1232 evidence files, 0 bytes moved |

**This is not parity and is not reported as parity.** The old path was already
red at the candidate revision, for a structural reason: the single active record
binds source revision `4b7d3318324c`, and every commit after it moves `HEAD`
away, so the qualification census can never pass on a branch that keeps
committing. Any sibling repository with a whole-tree, HEAD-bound retention model
should expect the same and should record what it observes rather than a parity
that does not exist.

## Exit criteria

1. Every pinned component classifies `compatible` through the packaged matrix.
2. The chain demonstrates all twelve outcomes, each negative paired with an
   accepted positive control.
3. Every retained evidence byte is unchanged and read through the pinned mapping.
4. No generic runner, envelope, manifest, identity lock, retention store, audit
   store, anchor authority, or aggregate verdict remains in the execution path.
5. The crate's default dependency graph is still empty and it still builds
   without `std` or `alloc`.
6. Hosted CI remains manual-only and undispatched.
