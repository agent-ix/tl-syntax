---
id: FR-009
title: "Version the future operator profile compatibly"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-008
    type: depends_on
---

# FR-009: Version the future operator profile compatibly

## Description

When a component accepts or exchanges a v1 derived future-time formula, the
ecosystem shall preserve distinct operator, text-dialect, wire-schema, and
evaluation-profile identities and shall not reinterpret an old identity.

## Inputs

- `tl-syntax.future-operators/v1` and one FR-008 lowering result.
- `mltl.closed-trace/v1` or `mltl.online-prefix/v1`.
- Optional internal ASCII text under an explicitly selected tl-parse dialect.
- Optional formula-v1 diagnostic or semantic serialization.

## Outputs

- A canonical graph accepted by existing formula-v1 consumers.
- Primitive-only canonical text, or a typed parse/profile refusal.
- No new derived node, formula field, or implicit source-authority claim.

## Behavior

The four profile axes are orthogonal:

| Axis | v1 decision |
|---|---|
| derived operator catalog | `tl-syntax.future-operators/v1` admits only W and M lowerings |
| internal text dialect | old `tl-parse.clean-ascii/v1` stays closed; `tl-parse.clean-ascii/v2` admits W/M input |
| formula wire | `tl-syntax.formula/v1` remains primitive-only and unchanged |
| evaluation | the selected closed-trace or online-prefix v1 profile is preserved, never inferred from spelling |

The `tl-parse.clean-ascii/v2` internal ASCII dialect may use only case-sensitive `W[a,b]` and
`M[a,b]` infix spellings at the existing U/R precedence and left
associativity, with the existing checked canonical interval syntax. It shall
reject interval-less W/M, any X spelling, lowercase or long-name aliases,
unknown operators, and non-canonical bounds rather than guessing.

The parser supplies distinct checked spans for the operator token and full
derived expression. Canonical formatting never re-sugars: it emits the existing
primitive F/G/U/R expression. Old v1 text parsers therefore continue to reject
derived input and accept v2-dialect canonical output. A formula-v1
document alone does not claim which surface produced it; a source
correspondence claim retains its source/dialect or native-clause identity
beside the formula.

Adding, removing, renaming, or changing a derived spelling or lowering requires
a new `tl-syntax.future-operators/vN` identity and a new text-dialect identity.
Adding or changing a canonical node requires a new formula schema. Changing
closed-trace, prefix, interval, or U/R meaning requires a new semantic-profile
identity and compatibility review. Changing only a diagnostic/report field
requires a new owning diagnostic/report identity, never silent reuse of an
exchanged identity.

Strong and weak next are refused. End-of-trace existence is not a proposition
in the canonical graph: under current false extension, `F[1,1] true` is true
beyond the physical trace, while `G[1,1] p` is false when `p` is false there.
Neither expression is a sound general encoding of the usual finite strong- or
weak-next boundary. A later closure-aware profile must define successor
existence before either spelling is admitted.

The v1 profile also refuses past/history and mixed-time operators, unbounded or
open intervals, dense or timestamped time, unit-bearing durations,
window-relative closure, a derived node in formula-v1, and every unknown
operator, dialect, wire, or semantic-profile identity. Each refusal occurs
before a formula document is produced and identifies the mismatched axis.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-009-AC-1 | `tl-parse.clean-ascii/v2` gives W/M the exact U/R precedence, associativity, interval and token/expression-span rules, rejects malformed or unrecognized forms, and normalizes only to primitive text; old v1 rejects derived input and accepts that output. | Test (TC-043, TC-047) |
| FR-009-AC-2 | Lowered formulas use the unchanged primitive-only formula-v1 wire schema and preserve the selected existing semantic profile; operator-profile and source attribution remain in their owning non-wire or source-correspondence records. | Test (TC-040, TC-044) |
| FR-009-AC-3 | A mutation in any derived spelling/lowering, canonical node, wire field, evaluation meaning, or diagnostic/report field requires the specified successor identity and cannot retain the old identity. | Test (TC-040, TC-043, TC-044) |
| FR-009-AC-4 | Every strong/weak-next, past/mixed-time, interval/time/closure, derived-wire, malformed, or unknown-profile combination returns a distinct refusal on the owning axis before document construction. | Test (TC-046, TC-047) |

## Dependencies

Depends on FR-003 identity/profile rules, FR-004's closed formula-v1 schema, and
FR-008 canonical lowering. Past/history profile design remains tl-syntax #33.
