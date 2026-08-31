---
id: PLAN-001
title: tl-syntax v0.1 implementation plan
type: Plan
relationships:
  - target: ix://agent-ix/tl-syntax/FR-001
    type: references
  - target: ix://agent-ix/tl-syntax/FR-005
    type: references
---

# tl-syntax v0.1 implementation plan

## Dependency DAG

```text
PGM-01
  -> specification + assurance foundation
  -> interval, identity, span, profile values
  -> borrowed node graph validation
  -> alloc-owned and serde wire documents
  -> shared conformance corpus
  -> retained CI, review, and gap-analysis evidence
  -> human v0.1 source-release decision
```

## Task File Mapping

| Task | Scope | Exit evidence |
|---|---|---|
| Task-001 | Specification and assurance foundation | Validated requirements, matrix, reviews, and assurance packet |
| Task-002 | Checked values and identities | Requirement-tagged unit and property tests |
| Task-003 | Borrowed formula graph | Complete operator and graph-invariant tests |
| Task-004 | Wire documents and corpus | Strict serde tests, schemas, fixtures, and semantic corpus checker |
| Task-005 | Verification and review remediation | Complete local gate and resolved automated-review findings |
| Task-006 | Exact-candidate evidence | Sealed PGM-01 validations and checksummed retained record |
| Task-007 | Human source-release decision | Maintainer review and explicit release decision |

## Exit Criteria

All test-matrix rows are backed by executable or retained inspection evidence,
the complete CI gate passes, no blocking gap remains, and the Assurance Argument
stays open until a human release owner records the source-release decision.
