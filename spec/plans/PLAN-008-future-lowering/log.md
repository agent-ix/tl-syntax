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
  97/111 rows; the three unbacked criteria (FR-009-AC-1, FR-010-AC-2,
  FR-010-AC-3) belong to downstream follow-ons. The matrices keep mainline's
  `Coverage Status` header because the pinned gate environment's TestMatrix
  archetype still asserts it; status classification therefore stays skipped,
  as on main, until the toolchain pin moves past quoin#371.
  Per owner direction, review happens once at PR time. Hosted CI was not
  dispatched.
