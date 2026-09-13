---
id: SR-059
title: "Gap analysis — formula-v2 and past syntax"
type: SpecReview
analysis: gap-analysis
scope: "Task-001, FR-011 syntax obligations, FR-013-AC-1, FR-013-AC-5, TC-054, formula-decoder portion of TC-057, TC-058"
review_set: subset
---

# Gap analysis — formula-v2 and past syntax

## Summary

Task-001 is complete. The public graph contains exactly O/H/Y/S/T, strong
Previous is a distinct primitive with no weak-Previous variant, formula-v2 has
strict read/write and profile validation, formula-v1 has a fixed compatibility
snapshot, conversions are explicit, node/depth resources are bounded, and the
accepted dependency DAG is checked by a fail-closed Rust test. Real `Trace:`
tags bind TC-054, the formula-decoder portion of TC-057, and TC-058 to tests.

## Verdict

**PASS for Task-001** — no implementation or traceability gap remains in the
syntax/wire task. The parser portion of TC-057 remains deliberately planned and
owned by Task-002; it is not reported as implemented here.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5901 | high | The initial manifest negative test mutated only one prerequisite revision/state and did not establish the declared fail-closed prerequisite matrix. Fixed by independently mutating authorization state/receipt and every M0/MRS revision and acceptance state, plus task identity, owner, edge, and cardinality. | `tests/past_profile_manifest.rs`, FR-013-AC-5, TC-058 |
| FND-5902 | medium | TC-057 spans two repositories, so marking the whole row implemented after only formula-v2 decoding would be a status lie. Fixed by leaving TC-057 planned, recording the syntax half in the plan log, and retaining parser ownership in Task-002. | TM-003, Task-002 |
| FND-5903 | medium | Public behavior needed a discoverable explanation outside the implementation. Fixed by documenting formula-v2, the closed past catalog/profile, v1 compatibility, and guarded conversion in the crate README. | `README.md`, FR-013-AC-1 |
| FND-5904 | low | The host exposes Quire 0.32.0 and ix-flow 0.2.3 while this repository pins 0.31.0 and 0.0.4. The shared pin and Quoin-chain tests correctly refuse; this is an external tool-environment mismatch and not missing Task-001 code. | `assurance/pins.json`, `tests/shared_assurance.rs` |

## Trace Census

- TC-054: seven traced compatibility/profile/wire tests in `tests/past_formula_v2.rs`.
- TC-057: bounded arbitrary formula-v2 decoder test plus depth/limit controls in `tests/past_formula_v2.rs`; parser half remains Task-002.
- TC-058: accepted manifest control and mutation matrix in `tests/past_profile_manifest.rs`.
- Production ownership: node/profile validation in `src/syntax.rs`; owned wire, conversion, and resource gates in `src/document.rs`; future-v1 compatibility guard in `src/future.rs`.
