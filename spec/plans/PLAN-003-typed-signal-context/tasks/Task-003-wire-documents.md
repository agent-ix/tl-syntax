---
id: Task-003
title: "Owned strict wire documents"
type: Task
status: not_started
track: Interchange
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-003
    type: part_of
  - target: ix://agent-ix/tl-syntax/FR-007
    type: references
---
# Task-003: Owned strict wire documents

## Scope

Add alloc-owned catalog/context values and separate v1 serde documents with
closed version enums, required fields, unknown-field refusal, bounded sequence
decoding, deterministic field/vector order, and lossless borrowed conversions.

## Completion Evidence

Positive and single-fault negative JSON fixtures plus round-trip/property tests
cover both new schema identities, all domain variants, count/string limits,
missing/unknown fields, and unknown versions. Formula-v1 and
proposition-map-v1 types remain untouched.
