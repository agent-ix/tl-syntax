---
type: log
title: "PLAN-003 - Update log"
description: "Chronological changes to the typed signal and caller context plan bundle."
---
# PLAN-003 - Update log

## History

- **2026-09-03** - Completed the specification and composite review. Corrected
  deterministic binding order, optional-context ownership, digest ownership,
  and legacy-byte compatibility wording before planning. Reconciled the closed
  scalar subset with `filament-core-data#35` and retained unsupported kinds as
  explicit future-adapter refusals.
- **2026-09-03** - Opened the implementation plan on the current PR #14 head.
  The plan adds only TL domain code and native tests, reuses the existing shared
  assurance intake, and leaves hosted CI manual-only.
- **2026-09-03** - During plan-to-code analysis, fixed FND-1406: catalog
  validation owns the non-Boolean direct-binding refusal, while formula binding
  checks only the missing-binding state reachable from a validated catalog.
- **2026-09-03** - During resource-bound review, fixed FND-1407: replaced
  quadratic duplicate-name scanning at the 100,000-signal bound with transient
  caller-owned `u32` name-order scratch and O(n log n) validation.
- **2026-09-03** - Implemented the allocation-free catalog/context core, owned
  strict wire documents, exact deterministic wire snapshots, formula binding,
  maximum-population tests, single-fault fixtures, and TC-027 through TC-033
  trace symbols. The focused tests, feature checks, Clippy, rustdoc, corpus gate,
  and strict specification gate pass; the exact-head full local gate remains.
