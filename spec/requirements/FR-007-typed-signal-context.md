---
id: FR-007
title: Validate typed signal catalogs and caller source context
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
---

# FR-007: Validate typed signal catalogs and caller source context

## Description

When a temporal frontend supplies named input signals or requirement provenance,
tl-syntax shall validate a parser-independent, versioned signal catalog and a
complete optional caller-source context without deriving, defaulting, or
interpreting either one.

## Inputs

- Stable signal identities, UTF-8 names, closed value-domain declarations, and
  proposition-to-signal bindings.
- A validated formula whose referenced propositions may be checked against the
  catalog.
- Optional requirement identity, exact requirement revision, clause identity,
  anchor, and checked half-open source span.

## Outputs

- A borrowed validated signal-catalog view and optional borrowed requirement
  context usable without allocation.
- Owned strict-wire documents under `alloc`/`serde` with identities
  `tl-syntax.signal-catalog/v1` and `tl-syntax.requirement-context/v1`.
- Typed deterministic validation errors with no partially validated output.

## Signal model

- `SignalId` and `PropositionId` are distinct stable unsigned identities.
- Signal declarations are strictly increasing by `SignalId`; names are non-empty
  and unique by exact UTF-8 byte equality, without normalization or case folding.
  Proposition bindings are strictly increasing by `PropositionId`.
- The v1 domain vocabulary is closed: Boolean, bounded signed integer with
  inclusive `i64` minimum/maximum, and bounded fixed decimal represented by
  inclusive signed `i64` coefficient bounds plus a decimal scale from 0 through
  18. A fixed-decimal value is its coefficient multiplied by 10 to the negative
  scale. Integer/coefficient bounds shall not be inverted.
- A direct MLTL proposition binding shall resolve to a declared Boolean signal.
  Integer and fixed-decimal declarations may be retained for a frontend's
  explicit predicate-lowering boundary but shall not be treated as Boolean.
- Binding a formula produces a borrowed bound-formula view. Validation visits
  proposition occurrences in formula-node order and shall reject the first
  missing or non-Boolean binding in that order. Every occurrence shall resolve;
  repeated occurrences may use the same binding. Extra declarations and
  bindings are allowed so one catalog may serve more than one formula.
- Catalogs contain at most 100,000 signal declarations and 100,000 proposition
  bindings. Each signal name contains 1 through 255 UTF-8 bytes.
- Borrowed catalog validation accepts caller-owned `u32` scratch with at least
  one slot per signal. It uses that scratch only to order declaration indices by
  exact name bytes for duplicate detection; the returned catalog does not borrow
  or retain the scratch. Insufficient scratch is a typed refusal. Owned document
  validation supplies its own temporary scratch under `alloc`.

## Caller-source context

- Context is optional at the consumer boundary and represented there as an
  `Option`; the standalone context value and wire document always represent a
  present context. When present, requirement id, exact revision, clause id,
  anchor, and source span are all required.
- Each textual context field contains 1 through 1,024 UTF-8 bytes. No field is
  inferred from another and no missing field receives a default.
- The clause-level source span is distinct from spans on MLTL nodes. Both use
  checked half-open byte offsets; neither is claimed to identify a file unless
  the caller's anchor does so.
- Validation establishes shape, bounds, and byte preservation only. It does not
  establish that a caller's identity or provenance statement is truthful.

## Compatibility and ownership

- Existing checked-in `tl-syntax.formula/v1` and
  `tl-syntax.proposition-map/v1` fixture bytes remain unchanged and continue to
  decode with the same results. Their APIs and closed schemas remain unchanged.
  The new schemas are separate documents, not optional fields smuggled into
  either v1 format.
- The default feature exposes borrowed identities, domains, declarations,
  bindings, catalog validation, and source-context validation without `alloc`.
  Owned strings/documents require `alloc`; wire encoding requires `serde`.
- The crate imports no contract IR, FRETish, Quire, Quoin, parser, evaluator,
  rewrite, or monitor runtime. Unsupported IR scalar types are rejected by the
  future adapter rather than accepted here through an opaque fallback.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-007-AC-1 | Boolean, bounded integer, and scale-0-through-18 fixed-decimal declarations within the count/name limits validate in O(n log n) time with adequate caller scratch and expose their identities, names, domains, and bounds unchanged through borrowed and owned forms. | Test (TC-027, TC-028) |
| FR-007-AC-2 | Insufficient name-order scratch, duplicate or non-increasing signal/binding identities, duplicate or empty/oversized names, inverted numeric bounds, scale above 18, missing signal targets, non-Boolean direct bindings, and over-limit populations are rejected with distinct typed errors. | Test (TC-029) |
| FR-007-AC-3 | Formula binding checks proposition occurrences in formula-node order, accepts a catalog's already-validated Boolean bindings as a borrowed bound-formula view, and rejects the first missing binding without fabricating a mapping. | Test (TC-030) |
| FR-007-AC-4 | Complete caller context round-trips exactly, consumer APIs accept explicit absence without fabricating context, and every missing, empty, oversized, inverted-span, or unknown-field present form is rejected. | Test (TC-031) |
| FR-007-AC-5 | Existing checked-in formula-v1 and proposition-map-v1 fixture bytes remain unchanged and decode with their prior schema identities and validation outcomes; no new field is accepted in either closed v1 document. | Test (TC-032) |

## Dependencies

Depends on stable proposition identities and source spans from
[FR-003](./FR-003-identities-and-profiles.md). It is the dependency root for
`agent-ix/tl-parse#20`, `agent-ix/tl-rewrite#21`, and
`agent-ix/tl-mltl#24`; `agent-ix/quire-contract-ir#57` owns the later IR and
FRETish adapter.
