---
id: ADR-017
title: "Represent assume-guarantee contracts as a first-class document with three statuses"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/FR-309
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-310
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-311
    type: realizes_requirement
---

# ADR-017: Represent assume-guarantee contracts as a first-class document with three statuses

## Status

Proposed (TL-419). Ready for human acceptance. Section "Contract interface
required by realizability" records an INTER-PROJECT DECISION that this ADR does
not make.

## Context

A contract pairs an assumption about the environment with a guarantee from the
system. It is how certification evidence is usually structured, and R2U2 and
the Lunar Gateway work already use it. Encoding it as a plain implication
`assume -> guarantee` loses the distinction an assessor needs: an implication is
true both when the guarantee holds and when the assumption never held, so a
contract that was never exercised looks the same as one that was verified.

## Decision

1. A contract is a named pair of roots (assumption, guarantee) in one existing
   formula graph, carried by `tl-syntax.contract-set/v1`, which binds to that
   graph by content identity exactly as `tl-syntax.fairness-premises/v1` does
   (retained, with a one-line justification, in the ADR-019 register).
2. The status of a contract at an evaluation position is a total function of
   the two Boolean-or-pending results:
   - assumption false: `inactive` (the guarantee is not consulted);
   - assumption true, guarantee false: `invalid`;
   - assumption true, guarantee true: `verified`;
   - otherwise (assumption pending, or assumption true and guarantee pending):
     `pending`, which is a non-verdict exactly like the evaluator's `Pending`.
3. The syntax crate owns the status vocabulary and this truth table so that the
   evaluator, the C2PO exporter and the realizability work share one definition.
4. A contract lowers to an ordinary obligation `assumption -> guarantee`
   appended to the graph for consumers (the solver) that need a formula.
   The lowering is explicit and never replaces the three-status report.

## Contract interface required by realizability

INTER-PROJECT DECISION REQUIRED (P-TL-149, realizability, is written
concurrently). Realizability reasons about an environment that must satisfy
assumptions and a system that must satisfy guarantees. Whether the
environment assumptions of P-TL-149 ARE these contracts is not decided here.
What this ADR guarantees to P-TL-149, whichever way it decides:

- The assumption and guarantee are addressable roots in a graph, so a
  realizability specification can reference them without copying formulas.
- Status semantics are per evaluation position. A contract says nothing about
  whether the assumption holds at every position of a trace.
- The obligation lowering above is available.

Options for Peter, none chosen here:

- A. Environment assumptions are contract assumptions with an invariant
  reading (the assumption must hold at every position). Requires adding an
  explicit assumption-scope field to `contract-set` v2; v1 stays per-position.
- B. Environment assumptions are a separate document type (like fairness
  premises) and a contract may reference one.
- C. Realizability consumes contracts as they are and defines the system
  winning condition as "no contract is ever `invalid`" over the environment
  behaviours that the realizability project itself constrains.

Recommendation, not a decision: C needs no change here and keeps contract
status per-position as the R2U2 differential requires; A is a later
`contract-set/v2` if P-TL-149 needs it.

## Open decisions (for Peter; none is made by this ADR)

1. **Contract scope: option A, B or C. Recommendation: C**, stated precisely so it
   can be compared with P-TL-149:
   - What this project provides is the per-position outcome series of tl-mltl
     FR-059, with `inactive`, `invalid`, `verified` and `pending` at every
     position, and the first `invalid` and first `inactive` times in its summary.
     There is no trace-level contract status; a whole-trace answer is a function a
     consumer computes from the series.
   - P-TL-149 under C consumes that series and defines its winning condition as
     "G holds at every position where A has held so far": for each position `t`
     such that the assumption held at every position `0..t`, the guarantee holds at
     `t`. Over the series that is "no `invalid` outcome at a position before the
     first `inactive` outcome" (all positions if there is none). This is the
     prefix-conditional reading that P-TL-149 ADR-004 decision 5 lists as its known
     limit (a) and does not specify; under C it becomes definable because the
     series exists.
   - C requires every proposition of an assumption to be bound to an `Input`
     signal, because otherwise the system could win by falsifying its own
     assumption. contract-set v1 does not and cannot check this: signal direction
     lives in P-TL-149's catalog v2. The check belongs to P-TL-149 and is part of
     what choosing C asks of it.
   - **Disagreement to resolve.** P-TL-149 ADR-004 decision 5 as written has a
     contract `(A, G)` contribute `A -> G` over the whole fixed trace, evaluated at
     position zero, with the status coinciding with the play outcome, and refuses
     non-`Input` assumptions. That differs from C in two ways (position-zero
     implication versus per-position series, and the stronger prefix-conditional
     winning condition). If C is chosen, P-TL-149 owns amending decision 5 to
     consume the series; this project does not edit it.
   - Option A needs `contract-set/v2` with an assumption-scope field and is the
     choice if P-TL-149 insists on whole-trace invariant assumptions. Option B
     keeps contracts and environment assumptions apart at the cost of a second
     document type.
2. **Who runs R2U2 and pins the numeric profile.** Not decided here. The profile
   (ADR-015) and the retained exchange cannot be produced by any specified work.
   See the matrix "Open and blocked acceptance criteria".
3. **Hash and digest exemption.** Whether the family's content-identity pattern
   (graph identity in this document, retained R2U2 bytes) is exempt from the
   no-hash rule. Recommendation: exempt only what has no structural alternative;
   the register in [ADR-019](./ADR-019-identity-and-digest-register.md) lists each
   item with its disposition.

## Alternatives rejected

- Contracts as plain implications: rejected, loses inactive versus verified.
- Contracts as a node kind in the formula graph: rejected, changes the core.
- Making the realizability scope decision here: rejected, belongs to P-TL-149.

## Consequences

Evaluators and exporters report one of four outcomes per position
(three statuses plus pending); TL-425 and StR-016 name the three statuses, and the
fourth, `pending`, exists only where a profile can leave a position undecided
(tl-mltl FR-059). The assumption/guarantee roots are shared graph
nodes, so a contract adds no formula copies.
