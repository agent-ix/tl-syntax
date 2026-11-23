---
id: FR-303
title: Publish a versioned predicate table with a strict reader
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-302
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-014
    type: depends_on
---

# FR-303: Publish a versioned predicate table with a strict reader

## Description

When a caller defines numeric comparison atoms, tl-syntax shall carry them in a
new `tl-syntax.predicate-table/v1` document with a checked-in Draft 7 schema and
a bounded strict byte reader, leaving every existing document unchanged.

## Inputs

- An ordered list of predicates, each a `PropositionId`, a comparison operator
  (`lt`, `le`, `gt`, `ge`, `eq`, `ne`) and two expression roots in a shared
  expression arena (FR-302).
- A validated signal catalog for construction-time typing.

## Outputs

- An owned `PredicateTableDocument` (under `alloc`/`serde`) with exact embedded
  schema bytes and SHA-256 constants following the pattern of
  `SIGNAL_CATALOG_V1_SCHEMA` and `FORMULA_V1_SCHEMA_SHA256`, and a bounded public
  reader.
- Typed `PredicateTableError`.

## Behavior

- Predicates are strictly increasing by `PropositionId`; at most 100,000.
- Both comparison operands shall have equal scale (`ScaleMismatch`). The
  comparison is Boolean; there is no numeric-to-Boolean coercion elsewhere.
- Numbers on the wire are JSON integers within `i64`; the reader rejects floats,
  exponents, negative zero, leading zeros, duplicate members, unknown members,
  trailing data, documents over 64 MiB and nesting beyond 64 containers, with no
  partial output.
- The document is separate from the formula and catalog documents; it is bound
  to them by FR-304 rather than embedded.
- Constructors take the catalog and fail on any FR-302 error, so an invalid
  table is unrepresentable as a public value. Deserialization routes through the
  same constructor.
- Canonical JSON bytes are deterministic (member order fixed by the schema).

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-303-AC-1 | A table of every operator and every node kind round-trips byte-identically through the strict reader and its schema digest constants match the checked-in schema bytes. | integration-testing (TC-1504) |
| FR-303-AC-2 | Each wire violation listed (float, exponent, `-0`, leading zero, duplicate or unknown member, trailing data, oversize, over-deep, non-increasing identities) is rejected with a distinct typed error and no output. | unit-testing (TC-1505) |
| FR-303-AC-3 | Checked-in `tl-syntax.formula/v1`, `/v2`, `tl-syntax.formula-unbounded/v1`, signal-catalog and proposition-map fixture bytes are unchanged and decode as before. | integration-testing (TC-1506) |
| FR-303-AC-4 | Deserializing a table whose expression overflows, mismatches scale, or reads a Boolean or undeclared signal fails exactly as direct construction does. | unit-testing (TC-1507) |

## Dependencies

[FR-014](./FR-014-publish-strict-syntax-artifacts.md) owns the owner-artifact
publication pattern this document joins.
