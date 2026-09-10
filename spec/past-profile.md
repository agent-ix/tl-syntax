---
id: MRS-003
title: "Finite-trace past/history semantic profile"
type: MasterRequirements
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/issues/33
    type: references
---

# Finite-trace past/history semantic profile

## Purpose

This specification defines a separately versioned, origin-complete past-time
MLTL profile with explicit anchor, history, progress, closure, compatibility,
and evidence rules. It extends the one canonical TL graph architecture without
reinterpreting any existing future-time formula or profile.

Native Quire remains the sole editable formal-clause language. The past-time TL
profile is internal representation and evaluator infrastructure reached only
through a separately reviewed native correspondence.

## Scope

### In scope

- Bounded Once, Historically, Since, and Triggered operators over discrete
  event positions.
- Inclusive intervals, exact origin/anchor behavior, pre-origin false
  extension, required-history analysis, and immutable result attribution.
- A new semantic profile and formula wire revision that preserve v1 documents.
- Exact operator, text-dialect, position-history, history-analysis, and
  evaluation-result identities.
- Refusal of mixed future/past graphs and unsupported clocks, history states,
  and operator combinations.
- Parser, evaluator, rewrite, corpus, property, fuzz, mutation, and
  interoperability evidence allocation.

### Out of scope

- Previous/weak-previous, mixed future/past formulas, unbounded/open/dense-time
  intervals, timestamp interpolation, implicit sampling, or wall-clock advance.
- Native Quire grammar, capture or predicate evaluation, production monitoring,
  and any external runtime as a qualification dependency.

## Requirements architecture

[FR-011](./requirements/FR-011-past-operator-semantics.md) defines the past
operator vocabulary and satisfaction relation.
[FR-012](./requirements/FR-012-history-anchor-progress.md) defines history,
anchor, clock, progress, closure, and resource meaning.
[FR-013](./requirements/FR-013-past-profile-compatibility-evidence.md) defines
wire compatibility, ecosystem ownership, and evidence.
[ADR-002](./assurance/ADR-002-origin-complete-past-profile.md) records the
separate-profile architecture; [TM-003](./past-profile-test-matrix.md) allocates
all planned evidence.

## Dependencies

M0 stabilization and accepted MRS-002 profile-evolution rules are prerequisites
to implementation. quire-contract-ir #64 is a downstream native-correspondence
consumer, not the source of TL past semantics.
