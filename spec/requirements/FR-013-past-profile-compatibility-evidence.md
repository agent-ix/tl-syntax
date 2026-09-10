---
id: FR-013
title: "Version and verify the past/history profile"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-012
    type: depends_on
---

# FR-013: Version and verify the past/history profile

## Description

When the past/history profile crosses a component boundary, the ecosystem shall
use new closed identities, preserve all v1 future documents unchanged, and
verify one shared canonical semantics without relying on a foreign runtime.

## Inputs

- The FR-011 graph and FR-012 anchored history/result identities.
- Formula schema `tl-syntax.formula/v2` and semantic profile
  `mltl.origin-complete-history/v1`.
- Operator profile `tl-syntax.past-operators/v1`, internal text dialect
  `tl-parse.clean-ascii/v3`, and target-specific mapping profiles.

## Outputs

- Strict v2 documents or typed version/profile/operator refusals.
- Canonical past text and paired conformance fixtures.
- Routed implementation/evidence work with explicit external-target states.

## Behavior

Formula-v2 is a common graph schema containing existing Boolean/future nodes and
the new O/H/S/T node tags. Profile validation admits only Boolean plus future
nodes for the two existing future profiles and only Boolean plus past nodes for
the new past profile. Mixed graphs refuse. Existing formula-v1 bytes, schema,
profile names, validation, and outcomes remain unchanged. Future-only v1
documents upgrade losslessly to v2; v2 down-conversion to v1 succeeds only for
a future profile with no past node or new field.

The exchanged identity axes and successor triggers are:

| Axis | Exact v1 identity | Requires a successor identity when |
|---|---|---|
| operator catalog | `tl-syntax.past-operators/v1` | a spelling, arity, lowering, or admitted operator changes |
| formula wire | `tl-syntax.formula/v2` | a node tag, field, validation rule, or canonical semantic view changes |
| evaluation semantics | `mltl.origin-complete-history/v1` | truth, origin, anchor, clock, progress, or closure meaning changes |
| internal text | `tl-parse.clean-ascii/v3` | spelling, precedence, associativity, interval, formatting, or span meaning changes |
| input history | `tl-mltl.position-history/v1` | ordering, completeness, revision, digest, origin, position, or clock binding changes |
| history analysis | `tl-mltl.history-requirement/v1` | required-history units, recurrence, arithmetic, or limit meaning changes |
| evaluation result | `tl-mltl.past-evaluation/v1` | attribution fields, finality, refusal, or result meaning changes |

No identity is inferred from another. A consumer validates every identity it
uses and refuses an unknown identity on its owning axis before decode,
evaluation, mapping, or attribution. A compatible additive diagnostic field
belongs to a separately versioned diagnostic/report type and cannot silently
change one of these exchanged contracts.

`tl-parse.clean-ascii/v3` adds case-sensitive `O[a,b]`, `H[a,b]`, `S[a,b]`,
and `T[a,b]` to the v2 grammar. O/H use existing prefix precedence; S/T use
existing U/R precedence and left associativity. The old v1 and v2 dialects
reject past spellings. Canonical past-profile formatting retains them because
no future-only primitive graph denotes them. Unknown, malformed,
mixed-profile, and profile-incompatible forms refuse at stable spans.

The shared corpus covers every operator, Boolean nest, endpoint, zero/singleton
history, pre-origin extension, invalid history, mixed graph, serialization,
required-history value, and result identity. An independent Rust reference
oracle and metamorphic duality/boundary properties are required. Fuzz targets
cover v2 decode and the successor parser. Mutation covers operator direction,
offset subtraction, endpoints, Since lower-bound range, Triggered duality,
profile gating, history gaps, anchor, digest, and required-history arithmetic.

External differential targets are dispositioned as follows:

| Target | v1 disposition |
|---|---|
| FRET/FRETish | output-only mapping; Electron/Node execution is unavailable and prohibited as qualification evidence |
| R2U2 | monitor/mapping target; differential comparison unavailable until an exact origin/history/clock profile is reviewed |
| C2PO | monitor/mapping target; differential comparison unavailable until an exact profile is reviewed |
| Isabelle MLTL development | future-time reference only; intentionally unsupported as a past-profile oracle |

Target parser acceptance is never semantic equivalence. Mapping reports retain
supported, unsupported, and unavailable as distinct states with source and
profile identities.

Repository ownership and implementation order after M0 and MRS-002/MRS-003
acceptance are:

1. `tl-syntax` owns common graph nodes, formula-v2, the closed past operator
   and semantic-profile values, and profile validation.
2. `tl-parse` owns only the `tl-parse.clean-ascii/v3` internal parser,
   canonical formatter, spans, refusals, and parser fuzzing.
3. `tl-mltl` owns position-history validation, required-history analysis,
   anchored evaluation/results, clocks, limits, and the independent test
   oracle; `tl-rewrite` owns profile-preserving rewrites and refusal of any
   rewrite without an admitted equivalence. These two lanes may proceed in
   parallel after the graph/wire contract exists.
4. `tl-syntax` owns the canonical shared schema/fixture corpus; each consumer
   owns replay evidence against the exact corpus digest.
5. Target adapters own target-specific mappings and loss reports;
   `quire-contract-ir` owns the separately reviewed native Quire
   clock/capture/predicate correspondence into this canonical graph.

No repository adds its own alternate past semantics. Neither the internal text
dialect nor formula wire is an editable formal-clause language; native Quire
remains the sole source authority.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-013-AC-1 | Formula-v2 round-trips every allowed future or past graph under its exact profile, refuses mixed/profile-incompatible graphs and unknown values, preserves formula-v1 bytes/outcomes, and enforces the stated upgrade/down-conversion rules. | Test (TC-054, TC-057) |
| FR-013-AC-2 | `tl-parse.clean-ascii/v3` parses/formats O/H/S/T with exact precedence, associativity, intervals, and spans; old v1/v2 reject past spellings and arbitrary input cannot unwind either parser or formula-v2 decoder. | Test (TC-055, TC-057) |
| FR-013-AC-3 | The corpus, Rust oracle, properties, and mutations cover every enumerated semantic, history, resource, serialization, and identity dimension without a second production evaluator. | Test (TC-052, TC-056) |
| FR-013-AC-4 | Every external target retains its reviewed supported/unsupported/unavailable state, introduces no foreign qualification dependency, and makes no parser-acceptance, monitor-certification, or native-source-authority claim. | Test (TC-056) |
| FR-013-AC-5 | An automated dependency manifest gate rejects every routed implementation ticket with the wrong repository owner, a missing predecessor, or implementation authorization before M0 and MRS-002/MRS-003 acceptance. | Test (TC-058) |

## Dependencies

Depends on FR-004/FR-005 versioning and corpus foundations plus FR-011/FR-012.
MRS-002 supplies the profile-evolution policy. M0 remains a hard implementation
prerequisite.
