---
id: FR-334
title: "Keep every existing profile, edition and wire byte unchanged when LTLf is added"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-045
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-044
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-009
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-013
    type: depends_on
---

# FR-334: Keep every existing profile, edition and wire byte unchanged when LTLf is added

## Description

When the LTLf profile and edition are added, tl-syntax shall leave the four
existing profile identifiers, the editions `tl-syntax.formula/v1`,
`tl-syntax.formula/v2` and `tl-syntax.formula-unbounded/v1`, and every retained
corpus byte exactly as they are.

## Behavior

`SemanticProfile::ALL` lists five distinct identifiers and `as_str` is injective
over it; the four existing strings are asserted literally. Existing decoders
refuse the `tl-syntax.formula-ltlf/v1` schema string through their FR-004
unknown-schema rule rather than downgrading. The existing corpus (the five
`corpus/formulas` and three `corpus/malformed` fixtures, eight entries in
`corpus/manifest.json`, plus the future-operator, past-history, infinite-trace
and boundary corpora) is byte-identical to its pre-change digests.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-334-AC-1 | Every pre-existing corpus file hashes to its pre-change digest, and each existing profile's wire form is unchanged. | Test (TC-226) |
| FR-334-AC-2 | Each existing edition decoder refuses the LTLf schema string with the unknown-schema cause. | Test (TC-227) |
| FR-334-AC-3 | `SemanticProfile::ALL` has five entries with five distinct `as_str` values and the four old strings verbatim. | Test (TC-228) |

## Dependencies

FR-004 and FR-009 own the compatibility axes; FR-013 owns past-profile evidence.
