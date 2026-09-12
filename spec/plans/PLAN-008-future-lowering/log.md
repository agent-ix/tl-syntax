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
