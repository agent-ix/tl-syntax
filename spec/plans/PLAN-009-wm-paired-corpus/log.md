---
type: log
title: "PLAN-009 - Update log"
description: "Chronological changes to the paired W/M corpus plan."
---

# PLAN-009 - Update log

## History

- **2026-09-12** - Allocated TC-074 and PLAN-009 after scanning the remote
  heads (`issue/33-past-profile` and `issue/34-source-qualification-readiness`
  hold FR-011 through FR-018, TC-048 through TC-073, and PLAN-007). Added the
  `tl-syntax.future-operator-corpus/v1` corpus and its replay test. Per owner
  direction, review happens once at PR time. Hosted CI was not dispatched.
- **2026-09-12** - Addressed the PR-time Rust review and gap analysis: added the
  primitive v1-source compatibility pair, all eight W/M boundary pairs across
  both profiles, report profile and identity checks, operator-spelling and
  separator binding, the recorded tl-parse cross-check revision, typed
  malformed error codes, four more refused cases, and negative controls for
  every replay code. Dropped the recorded refusal axis, which the code
  determines. Recorded that #41 was dispatched in parallel with tl-mltl#47 and
  tl-rewrite#35, consumes neither, and lands after them.
