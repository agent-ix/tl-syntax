---
type: log
title: "PLAN-008 - Update log"
description: "Chronological changes to the future-operator lowering plan."
---

# PLAN-008 - Update log

## History

- **2026-09-12** - Implemented FR-008 admission and lowering in `src/future.rs`
  and ten traced tests in `tests/future_lowering.rs` at commit `f36741e`. The
  formula-v1 node limit moved into the `no_std` core. Strict coverage backed
  97/104 rows; the three unbacked criteria (FR-009-AC-1, FR-010-AC-2,
  FR-010-AC-3) belong to downstream follow-ons. Per owner direction, TM-001
  and TM-002 adopt the single `Status` column (spec-artifacts-process#87,
  quoin#371), so status classification runs instead of being skipped. Released
  quoin 0.23.1 still pins spec-artifacts-process `d605caa`; the gate environment
  installs `375fc2a` explicitly until a quoin release carries #371.
  Per owner direction, review happens once at PR time. Hosted CI was not
  dispatched.
- **2026-09-12** - Addressed the PR #42 Rust review and gap analysis. The
  TC-046 precedence property now forces each admission axis to be the first
  fault with every earlier axis valid, generates one-sided out-of-range
  endpoints and start-side containment faults, and is backed by adjacent-pair,
  exact-field, byte-limit, and all-variant code/axis/Display tables. The
  TC-041, TC-042, and TC-046 titles no longer claim trace or source-level
  refusals this boundary cannot express.
