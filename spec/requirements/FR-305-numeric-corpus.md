---
id: FR-305
title: Publish the numeric predicate corpus and refusal classes
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-303
    type: depends_on
---

# FR-305: Publish the numeric predicate corpus and refusal classes

## Description

Where downstream crates need shared numeric evidence, tl-syntax shall publish
the immutable corpus `tl-syntax.numeric-corpus/v1` under `corpus/numeric/`,
reachable through `tl_syntax::CORPUS_DIR`, so that tl-parse, tl-mltl and
tl-solve replay one set of cases instead of vendoring copies.

## Behavior

- The corpus holds: valid tables (every operator, every node kind, Integer and
  Decimal scales 0, 1, 2, 9 and 18), each paired with its catalog; refused
  tables for every FR-302 and FR-303 error class; aggregation cases (FR-307,
  FR-308); contract-set cases (FR-309); and, for each valid case, the expected
  predicate truth over a declared set of input valuations, computed
  independently of the crate (exact rational arithmetic in a throwaway script
  whose source is retained under `corpus/numeric/provenance/`).
- A manifest records the corpus identity `tl-syntax.numeric-corpus/v1`, per-case
  class, expected outcome or refusal class, the case count per class and the
  relative path of each file. It records no per-file digest (see the ADR-019
  register): the corpus is immutable by version identity, so any change to a case
  needs `tl-syntax.numeric-corpus/v2`, and drift is caught semantically because
  the check replays every case and compares the result with its declared outcome.
  The class coverage list is closed: a missing class or a count that differs from
  the manifest fails the corpus check.
- The corpus is hand-authored or independently computed; generated fuzz output
  is not corpus.
- Existing corpora, their manifests and revisions are unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-305-AC-1 | The manifest covers every required class, every listed file exists, each class's case count equals the manifest, and a deleted class or a deleted case file fails the check. | integration-testing (TC-1511) |
| FR-305-AC-2 | Every refused case produces its declared class here; each valid case's derived types equal the declared expected types, and its expected truth table is retained for the evaluators (tl-syntax itself evaluates nothing; tl-mltl FR-054 replays the truth tables). | integration-testing (TC-1512) |
| FR-305-AC-3 | Existing `tl-syntax-corpus/v1` and past-history corpus bytes are unchanged. | integration-testing (TC-1513) |

## Dependencies

[FR-005](./FR-005-conformance-corpus.md) owns the corpus pattern.
