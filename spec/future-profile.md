---
id: MRS-002
title: "Future-time operator-profile evolution"
type: MasterRequirements
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/issues/32
    type: references
---

# Future-time operator-profile evolution

## Purpose

This specification defines the first post-v0.1 evolution of the internal,
parser-independent future-time MLTL operator profile. It adds a small derived
surface without changing the canonical formula graph, interval meaning, or the
two existing evaluator profiles.

Native Quire is the only editable formal-clause language. TL text and formula
documents are derived interchange and evaluator inputs. Nothing in this
specification creates a second Quire authoring path or grants TL source text
independent clause authority.

## Scope

### In scope

- A closed taxonomy of primitive, Boolean convenience, and derived future-time
  operators.
- Exact lowering of `W[a,b]` and `M[a,b]` into the existing canonical
  F/G/U/R node vocabulary.
- Version, compatibility, source-span, canonical-format, resource, horizon,
  corpus, rewrite, and interoperability rules for that lowering.
- Explicit refusal of unknown spellings and unsupported profile combinations.

### Out of scope

- A new `NodeKind`, formula-wire schema, evaluator, rewrite semantics, or
  semantic interpretation for an existing formula.
- Past-time/history operators, which belong to tl-syntax #33.
- Strong- or weak-next operators, unbounded or open intervals, dense or timestamped time,
  duration-unit conversion, and mixed future/past profiles.
- Typed predicates, native Quire grammar, production monitoring, and claims of
  FRETish, R2U2, or C2PO equivalence.

## System boundary

`tl-syntax` owns the canonical operator-profile identity and lowering contract.
`tl-parse` may later own a new internal ASCII dialect revision and must lower
before constructing a validated `tl-syntax` formula. `tl-mltl` evaluates only
the canonical graph. `tl-rewrite` rewrites only canonical nodes. Corpus and
interoperability owners consume the lowered representation and preserve the
selected semantic-profile identity.

The native Quire temporal bridge in `quire-contract-ir` may produce the same
canonical graph when its separately reviewed correspondence profile admits the
source clause. It does not consume the TL textual dialect and does not delegate
source authority to this profile.

## Requirements architecture

[FR-008](./requirements/FR-008-future-operator-lowering.md) defines canonical
lowering, [FR-009](./requirements/FR-009-future-profile-compatibility.md) defines
profile compatibility, and
[FR-010](./requirements/FR-010-future-profile-downstream-evidence.md) defines
downstream evidence and dependency order.
[ADR-001](./assurance/ADR-001-future-operator-profile.md) records why lowering,
rather than AST expansion, is the selected architecture.
[TM-002](./future-profile-test-matrix.md) assigns every proposed operator and
semantic-profile combination to its required downstream evidence.

## References

- [Future-time evolution epic](https://github.com/agent-ix/tl-syntax/issues/29).
- [Specification ticket](https://github.com/agent-ix/tl-syntax/issues/32).
- Kosaian, Wang, Sloan, and Rozier, *Formalizing MLTL Formula Progression in
  Isabelle/HOL*, arXiv:2410.03465. This is a non-normative semantic cross-check;
  the checked-in requirements remain authoritative for this ecosystem.
