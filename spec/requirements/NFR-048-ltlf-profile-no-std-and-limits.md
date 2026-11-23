---
id: NFR-048
title: "Keep the LTLf profile inside the no_std borrowed model and existing limits"
type: NFR
quality_attribute: portability
relationships:
  - target: ix://agent-ix/tl-syntax/FR-332
    type: constrains
  - target: ix://agent-ix/tl-syntax/NFR-001
    type: depends_on
---

# NFR-048: Keep the LTLf profile inside the no_std borrowed model and existing limits

## Statement

The tl-syntax LTLf profile shall build and validate in the allocation-free
borrowed model and shall apply the existing node-count, depth and identifier
limits with no new unbounded resource.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| LTLf validation in the `thumbv7em-none-eabi` borrowed build | builds | builds | Test |
| Documents refused one past the existing node limit | 100% | 100% | Test |

## Verification

The existing no_std build and limit tests are extended with LTLf cases.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-048-AC-1 | LTLf documents validate in the allocation-free borrowed build and are refused at one past each existing node, depth and identifier limit. | Test (TC-231) |
