---
id: FR-014
title: "Publish complete strict syntax artifact contracts"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: implements
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: depends_on
  - target: ix://agent-ix/tl-syntax/IF-004
    type: implements
  - target: ix://agent-ix/tl-syntax/VO-006
    type: implements
---
# FR-014: Publish complete strict syntax artifact contracts

## Description

When a downstream bridge supplies formula or signal document bytes, tl-syntax SHALL admit them only through bounded public strict readers for their exact immutable contract/schema selection and expose constructor-private validated documents without changing existing v1 bytes or no-std values.

## Architecture and public contracts

The implementation SHALL organize source as
`formula::{graph,profile,document}`, `signal::{domain,catalog,binding,document}`
and `contracts::{limits,manifest}`. Existing public paths remain compatibility
re-exports. The owner contracts are:

- `tl-syntax.formula/v1` for the unchanged future/Boolean graph;
- `tl-syntax.formula/v2` for the pure origin-complete past graph;
- `tl-syntax.signal-catalog/v1`; and
- `tl-syntax.proposition-map/v1`.

Each contract SHALL publish immutable schema bytes and a lowercase SHA-256
schema digest. The alloc+serde API SHALL expose `from_json_bytes(bytes, limits)`
for each document, returning a validated owned value or a typed error. Borrowed
construction/validation remains available without allocation under the existing
feature boundary.

The byte reader SHALL reject unknown/duplicate/missing/out-of-order fields,
trailing data, invalid UTF-8, unknown versions/profiles/operators/domains,
noncanonical JSON, invalid topology/references/intervals, unsorted/duplicate
populations, missing proposition bindings and every count/depth/string/byte/work
overrun. It SHALL parse once, charge before retention/traversal and expose no
partial document. Caller limits may lower but not raise owner maxima.

Formula-v1 remains byte- and behavior-identical. Formula-v2 admits only Boolean
nodes and O/H/Y/S/T under `mltl.origin-complete-history/v1` and
`tl-syntax.past-operators/v1`; it refuses every future or mixed graph. Signal
catalog/map readers preserve the already published schemas/digests and exact
Boolean correspondence behavior from FR-007.

Document content identities are domain-separated lowercase SHA-256 over the
exact canonical owner bytes. A content identity, schema digest, source span and
semantic graph identity remain distinct. Reorganization changes none of their
bytes, errors or public semantics.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-014-AC-1 | All four immutable schema byte constants/digests match independent files and their strict readers accept canonical boundary documents. | Test (TC-075) |
| FR-014-AC-2 | Unknown/duplicate/missing/reordered fields, trailing/noncanonical bytes, invalid topology/profile/operator/domain/binding and exact one-over limits refuse without a partial value or panic. | Test (TC-075) |
| FR-014-AC-3 | Formula-v1 and existing signal/map bytes, public paths, errors and semantic identities remain unchanged across the module reorganization. | Test (TC-075) |
| FR-014-AC-4 | Formula-v2 accepts every pure-past node/profile combination and refuses every future/mixed/weak/unknown combination before exposing a document. | Test (TC-075) |
| FR-014-AC-5 | Default no-std construction remains allocation-free; strict JSON readers are available only with the existing alloc/serde features and introduce no parser/evaluator. | Test (TC-075) |

## Dependencies

FR-004 supplies established wire versioning, FR-007 supplies signal/catalog
contracts, and FR-011 through FR-013 supply the accepted origin-complete past
profile. Downstream parsers, evaluators, rewriters and Contract IR consume only
these owner readers.

## Status

Proposed whole-ecosystem owner reconciliation for `tl-syntax#52/#64`.
