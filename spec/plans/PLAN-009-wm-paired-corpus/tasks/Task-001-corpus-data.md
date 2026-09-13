---
id: Task-001
title: Add the paired W/M corpus data and digests
type: Task
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-009
    type: part_of
---

# Task-001: Add the paired W/M corpus data and digests

Add `corpus/future-operators/` with `manifest.json`, `cases.json`, span-free
expected formula-v1 documents under `expected/`, `SHA256SUMS`, and a README.
Add the digest check to `make check-corpus`. The existing
`tl-syntax-corpus/v1` corpus is unchanged.
