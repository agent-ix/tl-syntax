---
id: VO-007
title: "Native and TL correspondence"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-025, type: implemented_by }
  - { target: ix://agent-ix/quire-contract-ir/FR-026, type: implemented_by }
---
# [VO-007] Native and TL correspondence

## Properties

- **contract_set** — every native, observation, result, TL target and bridge ContractSelection used by the operation.
- **source_set** — exact immutable native definition, Protocol-backed predicate valuations, observation context and independent formula-wide QSL/TL result artifacts.
- **derived_set** — exact predicate projection, sibling QSL/TL request artifact sets and consumer-owned canonical bytes/identities.
- **mapping** — total deterministic relation from each checked predicate/node/position/result field to its exact target field; every unsupported source construct has a typed loss state.
- **identity** — lowercase SHA-256 over the bridge domain, selected profile and canonical tuple of every applicable contract/source/derived identity.
- **decision** — one closed ENUM-001 disposition with the exact shape and ordered causes specified by FR-025 or FR-026.

Correspondence equality requires equality of the entire applicable tuple, not
only equal displayed formulas or Boolean verdicts. It preserves owner
identities, never asserts that two owners use one identity domain, never calls
an evaluator/parser/plugin, and never turns an owner non-value into false.
One Protocol result mapping supplies one checked-leaf valuation; it never stands
in for a formula-wide QSL result. Corrected QSL and TL results are paired only
through a strict-read prior join that retains both owner predecessor identities.
