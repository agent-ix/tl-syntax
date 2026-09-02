---
id: Task-006
title: "Exact-candidate evidence"
type: Task
status: done
track: Evidence
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-001
    type: part_of
  - target: ix://agent-ix/tl-syntax/MP-001
    type: references
---
# Task-006: Exact-candidate evidence

## Scope

Retain the exact clean revision's local results, tool and dependency identities, PGM-01 checks, and
explicit limitations in a checksummed evidence record.

## Completion Evidence

The exact-head remediation source revision requires its own clean-source record;
older records remain immutable historical evidence and do not cover later
source or specification changes. The current PR must therefore retain and
anchor a new passing record before requesting another merge decision.

The retained `835833fb2338` record has a passing collection summary, two passing sealed PGM-01
validations, and an anchored checksum manifest that exactly enumerates every artifact. Its envelope
remains non-self-attesting and explicitly inconclusive; the post-seal summary records the external
result, the sealed artifacts, and exact envelope SHA-256
`895ca01aa42bb1dc384dee7af29c90d2963a59e18b95c8551c2c5e642a27ba78`.
The preceding `f30d927c7285` record is retained as failed evidence because its validator environment
lacked the required RFC 3339 format checker; it is not presented as passing evidence.
