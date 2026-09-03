---
id: MRS-001
title: tl-syntax v0.1 master requirements
type: MasterRequirements
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
---

# Master Requirements Specification

## Purpose

This specification defines the parser-independent, `no_std` MLTL syntax and
semantic-profile substrate shared by the temporal crate family. It is the
authoritative requirements boundary for tl-syntax v0.1.

The governing compatibility, provenance, evidence, release-order, and
qualification policy is PGM-01 at
`ix://agent-ix/quire-contract-ir/PGM-01`. This specification cites that policy
without redefining or weakening it.

## Scope

### In Scope

- Discrete bounded MLTL syntax with inclusive intervals.
- Stable proposition identities, source spans, and semantic-profile identities.
- Versioned named signal catalogs with closed bounded value domains.
- Optional, validated requirement/clause source context for downstream evidence.
- A validated borrowed representation usable without allocation.
- Optional owned and serde representations.
- Versioned wire documents and a shared conformance corpus.

### Out of Scope

- Text parsing and formatting.
- Formula rewriting or normalization.
- Finite-trace evaluation and horizon computation algorithms.
- Production stream monitoring.

## System Overview

### System Description

tl-syntax is a Rust library whose trusted boundary is construction and
validation of syntax values. Downstream parsers, rewriters, evaluators, and
monitor adapters consume those values while retaining profile, signal, and
caller-supplied source identity.

### Intended Users

Embedded Rust consumers use the allocation-free borrowed model. Temporal tools
use the optional owned and serialization features. Reviewers use the corpus and
the sealed assurance chain to check compatibility and determinism.

## Requirements Architecture

The stakeholder requirements define portability and interoperability needs.
Functional requirements own interval validation, graph validation, identity,
versioning, corpus publication, bounded signal declarations, proposition
binding, and caller-source context. Non-functional requirements constrain the
feature boundary and deterministic behavior. The test matrix maps every
acceptance criterion to executable or inspection evidence.

## References

- [tl-syntax epic](https://github.com/agent-ix/tl-syntax/issues/5).
- [Contract-derived verification program](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [FRETish temporal frontend](https://github.com/agent-ix/quire-contract-ir/issues/57).
- [Typed signal and source-context child](https://github.com/agent-ix/tl-syntax/issues/15).
- [PGM-01 governance gate](https://github.com/agent-ix/quire-contract-ir/issues/3),
  identified as `ix://agent-ix/quire-contract-ir/PGM-01`.
- Cargo package manifest and repository contribution policy.
