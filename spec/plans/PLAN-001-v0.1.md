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

## Work Packages

1. Validate the requirements, matrix, composite review, and assurance packet.
2. Implement core values and borrowed graph validation with no default features.
3. Implement optional owned and serde documents with strict version enums.
4. Add requirement-tagged tests and checked-in corpus fixtures.
5. Run CI, feature checks, corpus verification, code review, and gap analysis.

## Exit Criteria

All test-matrix rows are backed by executable or retained inspection evidence,
the complete CI gate passes, no blocking gap remains, and the Assurance Argument
stays open until a human release owner records the source-release decision.
