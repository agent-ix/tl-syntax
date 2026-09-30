---
id: MRS-001
title: tl-syntax v0.1 master requirements
type: MasterRequirements
---

# Master Requirements Specification

## Purpose

This specification defines the parser-independent, `no_std` MLTL syntax and
semantic-profile substrate shared by the temporal crate family. It is the
authoritative requirements boundary for tl-syntax v0.1.

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
- A user-authored TL or FRETish source language; native Quire is the sole
  editable formal-clause authority.
- Automatic release approval, tool certification, consuming-system
  qualification, monitor qualification, crates.io publication, or a local
  evidence/approval framework.

## System Overview

### System Description

tl-syntax is a Rust library whose trusted boundary is construction and
validation of syntax values. Downstream parsers, rewriters, evaluators, and
monitor adapters consume those values while retaining profile, signal, and
caller-supplied source identity.

### Intended Users

Embedded Rust consumers use the allocation-free borrowed model. Temporal tools
use the optional owned and serialization features. Reviewers use the corpus to
check compatibility and determinism.

## Requirements Architecture

The stakeholder requirements define portability and interoperability needs.
Functional requirements own interval validation, graph validation, identity,
versioning, corpus publication, bounded signal declarations, proposition
binding, and caller-source context.
Non-functional requirements constrain the feature boundary and deterministic
domain behavior. The test matrix maps every acceptance criterion to executable or
inspection evidence.

## References

- [tl-syntax epic](https://github.com/agent-ix/tl-syntax/issues/5).
- [Contract-derived verification program](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [Native Quire typed-predicate bridge](https://github.com/agent-ix/quire-contract-ir/issues/63).
- [Export-only FRETish mapping](https://github.com/agent-ix/quire-contract-ir/issues/57).
- [Typed signal and source-context child](https://github.com/agent-ix/tl-syntax/issues/15).
- [Post-v0.1 future operator-profile specification](./future-profile.md).
- [Post-v0.1 past/history profile specification](./past-profile.md).
- [Strict owner artifact contracts](./requirements/FR-014-publish-strict-syntax-artifacts.md).
- [Progressive source-readiness child](https://github.com/agent-ix/tl-syntax/issues/34).
- [Progressive source-readiness specification](./source-readiness.md).
- Cargo package manifest and repository contribution policy.
