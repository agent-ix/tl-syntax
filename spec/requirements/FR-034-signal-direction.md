---
id: FR-034
title: Declare input and output direction on the signal catalog
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
  - target: ix://agent-ix/tl-mltl/ADR-004
    type: references
  - target: ix://agent-ix/tl-solve/FR-100
    type: references
---

# FR-034: Declare input and output direction on the signal catalog

## Description

Where a requirement-realizability consumer needs to know which signals the
environment controls and which the system controls, tl-syntax shall carry an
optional per-signal direction in a new versioned signal-catalog document.
When a v1 catalog is read, tl-syntax shall report every signal as having no
declared direction. If a formula references a signal with no declared
direction, then tl-syntax shall refuse to partition that formula, with a typed
error and no partial output.

[ADR-004 in tl-mltl](ix://agent-ix/tl-mltl/ADR-004) records why direction is a
property of the signal and not of the proposition binding, and why a new
document version carries it. This requirement owns only the syntax and the
partition refusal, and owns no realizability semantics, no solver and no
verdict; those belong to `ix://agent-ix/tl-mltl/FR-043` and to tl-solve
(`ix://agent-ix/tl-solve/FR-100`).

## Inputs

- A `tl-syntax.signal-catalog/v1` or `tl-syntax.signal-catalog/v2` JSON
  document, or borrowed declarations built with the existing
  `SignalDeclaration::new` plus the new direction.
- A validated formula, or an ordered set of validated formulas, whose
  propositions are to be partitioned.

## Outputs

- `SignalDirection`, a closed, non-exhaustive enumeration with three values:
  `Undeclared`, `Input` (the environment chooses the signal) and `Output` (the
  system chooses the signal). `Undeclared` is the default.
- `SignalDeclaration::direction()` and `OwnedSignalDeclaration::direction()`;
  the additive constructors `SignalDeclaration::with_direction` and
  `OwnedSignalDeclaration::with_direction`. `SignalDeclaration::new` is
  unchanged and yields `Undeclared`.
- `SignalCatalogSchemaVersion::V2` with identity `tl-syntax.signal-catalog/v2`.
  `SignalCatalogDocument::new` still constructs v1; a new
  `SignalCatalogDocument::new_v2` constructs v2.
- `SignalCatalog::proposition_direction(PropositionId)`, returning the
  direction of the bound signal.
- `SignalCatalog::partition(Formula)` and `SignalCatalog::partition_all`
  (under `alloc`), returning a `SignalPartition` whose `inputs` and `outputs`
  are strictly increasing `PropositionId` lists, or a `SignalPartitionError`.
- A checked-in Draft 7 schema `spec/signal-catalog-v2.schema.json`, embedded as
  `SIGNAL_CATALOG_V2_SCHEMA`, `SIGNAL_CATALOG_V2_SCHEMA_BYTES` and
  `SIGNAL_CATALOG_V2_SCHEMA_SHA256` beside the unchanged v1 constants.

## Behavior

- Direction is a property of the signal declaration. A proposition has the
  direction of the signal it is bound to, so two propositions bound to one
  signal always share a direction and no proposition can be both input and
  output.
- A v2 signal object has the v1 members plus an optional `direction` member
  whose only values are `"input"` and `"output"`. Absence means `Undeclared`.
  The literal `"undeclared"` is not a v2 value, so each meaning has one wire
  form. A v2 document with no direction member anywhere is valid and reads as
  all `Undeclared`.
- The v1 schema, v1 reader behavior and v1 canonical bytes and content
  identity are unchanged. A v1 document containing a `direction` member is
  refused by the closed v1 shape. `SignalCatalogDocument::from_json_bytes`
  accepts either identity and reports the one it read; a v1 document reads as
  every signal `Undeclared`. `SignalCatalogDocument::new` refuses a signal
  carrying a declared direction with a typed error rather than silently
  upgrading the identity.
- `partition` visits proposition occurrences in formula-node order, exactly as
  `bind_formula` does, and returns the first failure. A proposition with no
  binding or a non-Boolean binding yields the existing
  `FormulaBindingError` wrapped in `SignalPartitionError::Binding`. A bound
  Boolean signal whose direction is `Undeclared` yields
  `SignalPartitionError::UndeclaredDirection` carrying the proposition and the
  signal identity.
- Only signals bound to a proposition that occurs in the partitioned formulas
  must be declared. An undeclared signal that no occurring proposition
  references does not cause a refusal; one catalog may serve several formulas.
- `partition_all` partitions the union of occurrences across the formulas in
  the given order. A proposition occurring in several formulas appears once.
- Direction is a caller declaration. This crate does not check that a
  declaration is true of any system, and a catalog with every signal declared
  `Output` or every signal declared `Input` is valid.
- Integer and fixed-decimal signals may carry a direction in v2; the direct
  Boolean-binding rule of FR-007 is unchanged, so a numeric signal still
  cannot be bound directly to a proposition.
- Name and identity limits, ordering rules and resource limits of FR-007 apply
  unchanged to v2 documents.
- Use of a v2 catalog with P-TL-150 numeric predicates (tl-syntax FR-303, on
  the TL-150 branch; not edited here). The rule: `partition` reads direction
  only through the direct Boolean binding of FR-007, so a proposition defined
  by a predicate-table atom has no `proposition_direction` and `partition`
  reports it as `SignalPartitionError::Binding`, never as a direction. A
  numeric signal's declared direction is carried in v2 and is unread by
  partition in this version. Consumers (tl-solve FR-100) therefore return
  `Unsupported` for a request that uses a predicate table. A later version may
  define an atom's direction as the common direction of every numeric signal its
  lowering reads and treat a mixture as `MixedDirectionAtom`; that needs the
  lowering to expose its signal reads and belongs to P-TL-150, not to this
  requirement.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-034-AC-1 | Every existing checked-in v1 catalog fixture still validates, round-trips to byte-identical canonical bytes and an unchanged content identity, and reports every signal as `Undeclared` through `direction()` and `proposition_direction`. | Test (TC-180) |
| FR-034-AC-2 | A v2 document with any mixture of `input`, `output` and absent direction members round-trips through `from_json_bytes` and `canonical_json_bytes` with each direction preserved and absent members read as `Undeclared`; an unknown direction value, the literal `undeclared`, a duplicate `direction` member, and a `direction` member in a v1 document are each refused without partial output, and `SignalCatalogDocument::new` refuses a declared direction with a typed error. | Test (TC-181) |
| FR-034-AC-3 | For every generated catalog and formula whose referenced signals are all declared, `partition` returns disjoint strictly increasing `inputs` and `outputs` lists whose union is exactly the set of propositions occurring in the formula, each assigned the direction of its bound signal, and `partition_all` equals the partition of the formulas' union. | Test (TC-182) |
| FR-034-AC-4 | When a referenced signal is `Undeclared`, `partition` and `partition_all` return `SignalPartitionError::UndeclaredDirection` naming the first such proposition in formula-node order and its signal, and return no partial partition; an undeclared signal that no occurring proposition references does not cause a refusal; a missing or non-Boolean binding is reported as `SignalPartitionError::Binding` with the FR-007 variant unchanged. | Test (TC-183) |
| FR-034-AC-5 | The v2 schema is exposed as exact embedded UTF-8 bytes with a recorded SHA-256, the v1 schema bytes and digest are unchanged, and the v2 schema accepts exactly the documents the semantic reader accepts structurally. | Test (TC-184) |

## Dependencies

[FR-007](./FR-007-typed-signal-context.md) owns the catalog, domains and
binding rules this requirement extends. The consumer is tl-solve
`ix://agent-ix/tl-solve/FR-100`, which turns a `SignalPartitionError` into the
typed refusal of a realizability request; tl-mltl
`ix://agent-ix/tl-mltl/FR-043` consumes `SignalPartition` for its reference
play-out. P-TL-150 numeric-predicate lowering (TL-419) and the assume-guarantee
contract form (TL-425) are siblings; the interface they must meet is recorded in
tl-mltl ADR-004, decision 5 (the contract interface; decision 4 is the direction declaration this requirement implements).
