---
id: PLAN-051
title: LTLf finite-trace profile plan
type: Plan
status: not_started
relationships:
  - target: ix://agent-ix/tl-syntax/FR-332
    type: references
  - target: ix://agent-ix/tl-syntax/FR-333
    type: references
  - target: ix://agent-ix/tl-syntax/FR-334
    type: references
  - target: ix://agent-ix/tl-syntax/FR-335
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-048
    type: references
---

# PLAN-051: LTLf finite-trace profile plan

## Objective

Land the tl-syntax half of TL-428: profile `mltl.ltlf-finite-trace/v1`, edition
`tl-syntax.formula-ltlf/v1`, the explicit strong and weak next nodes, proof that
nothing existing changed, and the shared LTLf corpus. Build starts only after
ADR-044 and ADR-045 are accepted by a human.

## Dependency DAG

Layer numbers match `ix-board project-chunk P-TL-151`.

```text
layer 0  TL-427 (this specification, human acceptance of ADRs)
layer 1  TL-428
           Task-001 profile, admission, next nodes (FR-332, FR-333, NFR-048)
             -> Task-002 compatibility proof and corpus (FR-334, FR-335)
layer 2  TL-429 (tl-parse v5) consumes the graphs and corpus
```

Pin sequencing (the new `NodeKind` variants break exhaustive matches downstream,
ADR-044 type shape): (1) TL-428 lands the tl-syntax change on tl-syntax main and
is released at a tag or revision; (2) only then do tl-parse, tl-mltl and tl-rewrite
bump their tl-syntax pin, each in its own PR that adds the explicit arms and
changes nothing else (tl-mltl currently pins a tl-syntax git revision and a tl-parse
revision, so tl-parse bumps first and tl-mltl bumps both together); (3) tl-solve
bumps last. No downstream repository carries LTLf code against an unreleased
tl-syntax; the downstream compile check is DEFERRED until (1).

Reason for Task-001 before Task-002: the corpus and the byte-identity proof need
the new variants to exist.

## Task File Mapping

| Task | Ticket | Scope | Exit evidence |
|---|---|---|---|
| Task-001 | TL-428 | FR-332, FR-333, NFR-048 | TC-220 to TC-225, TC-231 |
| Task-002 | TL-428 | FR-334, FR-335 | TC-226 to TC-230 |

## Quality gates

- Gate A after Task-001: all four existing profile strings and the eight
  existing top-level corpus fixtures unchanged. If it fails, stop and fix before
  Task-002.

## Deferred verification

| Item | Why deferred | Needed to run |
|---|---|---|
| Downstream compile of tl-parse, tl-mltl, tl-rewrite against the five-variant `SemanticProfile` | no build capacity during specification | `make ci` in each dependent repository |
| `thumbv7em-none-eabi` build (TC-231) | no build capacity | the existing no_std CI target |

## Ungrounded items

None specific to tl-syntax beyond the citation of the literature's own
counterexample formula (see tl-mltl ADR-044).
