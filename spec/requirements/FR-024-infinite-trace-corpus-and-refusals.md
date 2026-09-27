---
id: FR-024
title: "Publish independent infinite-trace corpus and refusal cases"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-021
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-022
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-023
    type: depends_on
---

# FR-024: Publish independent infinite-trace corpus and refusal cases

## Description

When the V1 infinite-trace corpus is published, tl-syntax shall retain a
versioned `tl-syntax.infinite-trace-corpus/v1` manifest, a schema, and
digest-pinned hand-verified positive and negative cases under `corpus/`.

## Inputs

- Formula-unbounded, fairness, lasso, and partial-valuation documents.
- Independently derived expected results and typed refusal expectations.

## Outputs

- Stable case identities and SHA-256 digests for every retained corpus file.
- Positive cases with the derivation of each expected verdict beside that
  verdict; negative cases with the exact rejected axis and reason.

## Behavior

The corpus includes lasso witnesses, an empty-prefix lasso, missing and
conflicting valuations, fair and unfair loops, bounded/unbounded future and
past forms, and finite-prefix inconclusive cases. It includes negative cases
for unbounded syntax under a finite profile, formula/profile and clock
mismatches, fairness on a non-lasso trace, malformed loops, and unsupported
operator/interval combinations. Syntax refusals classify profile, schema,
operator, interval, fairness, clock, lasso, valuation, and resource axes with
typed reasons; no wildcard converts an unknown axis to another. A consumer
reads the corpus through `tl_syntax::CORPUS_DIR` at its pinned revision rather
than vending a copy. The corpus records its independently checked oracle
reasoning; it does not derive golden verdicts from production evaluation.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-024-AC-1 | Every case has a unique identity, declared schema/profile/clock, expected result or refusal, and human derivation; the manifest digest-verifies every retained corpus file. | Test (TC-160) |
| FR-024-AC-2 | Each listed positive and negative family has at least one case, and mutating its input, expected axis, verdict, or pinned digest makes replay fail. | Test (TC-161) |
| FR-024-AC-3 | Downstream consumers resolve the owner corpus through `CORPUS_DIR` at a pinned revision and do not vendor its bytes. | Inspection (TC-162) |

## Dependencies

FR-005 supplies corpus identity rules. FR-021 through FR-023 supply the new
input documents.
