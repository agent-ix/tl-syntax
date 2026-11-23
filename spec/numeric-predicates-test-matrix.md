---
id: TM-017
title: "Numeric predicates, set aggregation and contracts test matrix (tl-syntax)"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: covers
  - target: ix://agent-ix/tl-syntax/StR-016
    type: covers
---

# Numeric predicates, set aggregation and contracts test matrix (tl-syntax)

Project P-TL-150 (TL-419 through TL-426), tl-syntax portion: TL-420, TL-424 and
the syntax half of TL-425. Test ids TC-1500 through TC-1599 are reserved for
this project in tl-syntax. Test case ids are per repository: the ranges of tl-syntax
(TC-1500..), tl-parse (TC-1540..) and tl-mltl (TC-1570..) overlap numerically, so
a cross-repository citation qualifies the id (`tl-parse/TC-1549`), and the same number
in two repositories is two different cases. Every row is planned (`🚧`); no row claims an
implementation.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-1500 | Derived expression types equal independently computed intervals over the numeric corpus | Property | P0 | FR-302-AC-1 | 🚧 planned (TL-420) |
| TC-1501 | abs over zero-straddling ranges and mul corner hulls with negatives | Unit | P0 | FR-302-AC-1 | 🚧 planned (TL-420) |
| TC-1502 | Each expression error class from a minimal input in node order, no view | Unit | P0 | FR-302-AC-2 | 🚧 planned (TL-420) |
| TC-1503 | Corner evaluation by a test-only reference never overflows or leaves the derived range | Property | P0 | FR-302-AC-3 | 🚧 planned (TL-420) |
| TC-1504 | Predicate table round-trips byte-identically; embedded schema bytes equal the checked-in schema | Integration | P0 | FR-303-AC-1 | 🚧 planned (TL-420) |
| TC-1505 | Strict reader rejects float, exponent, negative zero, leading zero, duplicate, unknown, trailing, oversize, over-deep, non-increasing | Unit | P0 | FR-303-AC-2 | 🚧 planned (TL-420) |
| TC-1506 | Existing formula v1/v2/unbounded, catalog and proposition-map fixture bytes unchanged | Integration | P0 | FR-303-AC-3 | 🚧 planned (TL-420) |
| TC-1507 | Deserialization fails exactly as direct construction for type and domain errors | Unit | P0 | FR-303-AC-4 | 🚧 planned (TL-420) |
| TC-1508 | Mixed Boolean-bound and predicate atoms bind in node order | Unit | P0 | FR-304-AC-1 | 🚧 planned (TL-420) |
| TC-1509 | Unbound, doubly defined, non-numeric refusals; bind_formula still fails closed | Unit | P0 | FR-304-AC-2 | 🚧 planned (TL-420) |
| TC-1510 | NodeKind, SemanticProfile and schema digest constants unchanged | Inspection | P1 | FR-304-AC-3 | 🚧 planned (TL-420) |
| TC-1511 | Numeric corpus manifest covers every class, listed files exist and per-class counts match | Integration | P0 | FR-305-AC-1 | 🚧 planned (TL-420) |
| TC-1512 | Refused corpus cases yield declared classes; valid cases yield declared types | Integration | P0 | FR-305-AC-2 | 🚧 planned (TL-420) |
| TC-1513 | Existing shared and past-history corpora unchanged | Integration | P1 | FR-305-AC-3 | 🚧 planned (TL-420) |
| TC-1514 | Aggregation document round-trips for every quantifier and member kind | Integration | P0 | FR-306-AC-1 | 🚧 planned (TL-424) |
| TC-1515 | Aggregation refusals: misplaced slot, undeclared or duplicate member, ill-typed member, missing binding, id collision | Unit | P0 | FR-306-AC-2 | 🚧 planned (TL-424) |
| TC-1516 | Expansion is byte-deterministic across runs and targets | Property | P0 | FR-307-AC-1 | 🚧 planned (TL-424) |
| TC-1517 | Expansion node and predicate counts equal closed forms on the (k, n) grid | Property | P0 | FR-307-AC-1 | 🚧 planned (TL-424) |
| TC-1518 | Expansion above any bound refused before output; limit minus one succeeds | Unit | P0 | FR-307-AC-2 | 🚧 planned (TL-424) |
| TC-1519 | No-std expansion core builds without alloc; ExpansionTooSmall reports exact counts | Compile | P0 | FR-307-AC-3 | 🚧 planned (TL-424) |
| TC-1520 | Each aggregation case has a hand-written form and a population whose count equals the manifest | Integration | P0 | FR-308-AC-1 | 🚧 planned (TL-424) |
| TC-1521 | Structural subset check passes; member-omitting expansion fails it | Integration | P1 | FR-308-AC-2 | 🚧 planned (TL-424) |
| TC-1522 | Contract set round-trips over v1, v2 and unbounded editions | Integration | P0 | FR-309-AC-1 | 🚧 planned (TL-425) |
| TC-1523 | Contract set error classes each distinct, no document | Unit | P0 | FR-309-AC-2 | 🚧 planned (TL-425) |
| TC-1524 | Formula graph and schema digests unchanged by a contract set | Inspection | P1 | FR-309-AC-3 | 🚧 planned (TL-425) |
| TC-1525 | Nine-row status truth table and spellings | Unit | P0 | FR-310-AC-1 | 🚧 planned (TL-425) |
| TC-1526 | Status function monotone under refinement | Property | P0 | FR-310-AC-2 | 🚧 planned (TL-425) |
| TC-1527 | Obligation lowering appends one Implies node and refuses at the limit | Unit | P0 | FR-310-AC-3 | 🚧 planned (TL-425) |
| TC-1528 | ContractView exposes roots and obligation, no scope field | Unit | P1 | FR-311-AC-1 | 🚧 planned (TL-425) |
| TC-1529 | Realizability decision recorded by a human in ADR-017 | Inspection | P0 | FR-311-AC-2 | ⛔ blocked on inter-project decision (Peter) |
| TC-1530 | No-std feature matrix and empty default dependencies over new modules | Integration | P0 | NFR-018-AC-1 | 🚧 planned (TL-420) |
| TC-1531 | Every limit accepted at limit and refused at limit plus one | Unit | P0 | NFR-019-AC-1 | 🚧 planned (TL-420) |
| TC-1532 | Corpus replay byte-identical across runs and targets | Integration | P1 | NFR-019-AC-2 | 🚧 planned (TL-420) |
| TC-1533 | ADR-014..019 proposed, requirements and matrices present in all four repos, spec review recorded | Inspection | P0 | StR-015-VC-3 | 🚧 planned (TL-419) |
| TC-1534 | Reviewer reads a predicate's type, range and scale from one document | Demonstration | P2 | StR-015-VC-1 | 🚧 planned (TL-420) |
| TC-1535 | Reviewer compares an expansion with the hand-written form | Demonstration | P2 | StR-015-VC-2 | 🚧 planned (TL-424) |
| TC-1536 | Reviewer sees inactive, invalid and verified for the three situations | Demonstration | P2 | StR-016-VC-1 | 🚧 planned (TL-425) |
| TC-1537 | Realizability can reference roots without scope claim | Inspection | P1 | StR-016-VC-2 | 🚧 planned (TL-425) |
| TC-1538 | Fresh-identity precedence across templates and caller ids; nesting depth 2 | Unit | P0 | FR-307-AC-4 | 🚧 planned (TL-424) |
| TC-1539 | No digest or pin beyond the retained items of the ADR-019 register | Inspection | P1 | NFR-019-AC-3 | 🚧 planned (TL-420) |

## Set-aggregation count grid

For `for_each`/`for_some` the nodes added are `k - 1` for `k >= 1` and 1 for
`k = 0`. For `at_least(n)`, `1 <= n <= k`, nodes added are at most
`2 n (k - n + 1) + 2`: `(k, n) = (1,1)` gives 4, `(2,1)` 6, `(2,2)` 6, `(3,2)` 10,
`(4,2)` 14, `(5,3)` 20. The grid of TC-1517 is `k` in 0..=5 and `n` in 0..=6
(42 pairs).

## Ticket acceptance-criteria map (all of P-TL-150)

Master map for TL-419 through TL-426. Test ids TC-15xx and TC-16xx live in
tl-syntax, tl-parse (TC-1540..) and tl-mltl (TC-1570..); TC-20xx in tl-solve.
DEFERRED marks evidence that needs fresh R2U2 or C2PO output, or a human decision.

| Ticket | Acceptance criterion | Requirements | Test cases |
|---|---|---|---|
| TL-419 | ADRs accepted | tl-syntax ADR-014..ADR-019, tl-solve ADR-200, ADR-201, tl-mltl ADR-014 | OPEN (human action): TC-1533 checks only that they are recorded as proposed; acceptance, including the StR-003/FR-007 amendment of ADR-014 decision 6, is a human decision, PENDING |
| TL-419 | Requirements and test matrix written across tl-syntax, tl-parse, tl-mltl and tl-solve | StR-015, StR-016 and the per-repo sets | TC-1533 |
| TL-419 | `/spec-review` recorded | n/a | OPEN (human action): independent review artifacts exist under `spec/reviews/` of each repository (review only, branch TL-150-review); acceptance of their dispositions is pending |
| TL-420 | Versioned document with strict reader; existing formula documents unchanged | FR-303, FR-304 | TC-1504, TC-1505, TC-1506, TC-1510 |
| TL-420 | Type and domain errors rejected at construction | FR-302, FR-303-AC-4, FR-304-AC-2 | TC-1502, TC-1507, TC-1509 |
| TL-420 | Still `no_std` in the core feature set | NFR-018 | TC-1530 |
| TL-421 | Parse then format round-trips on the corpus | tl-parse FR-031 | TC-1549, TC-1550 |
| TL-421 | Existing dialects unchanged | tl-parse FR-028 | TC-1541 |
| TL-421 | Fuzz target and hostile-input corpus extended | tl-parse FR-033 | TC-1555, TC-1556, TC-1557 |
| TL-422 | Trace documents carry Integer and fixed-Decimal observations with a strict reader | tl-mltl FR-053 | TC-1570, TC-1571, TC-1572 |
| TL-422 | Missing numeric observations follow each profile's pending and padding rules | tl-mltl FR-055 | TC-1578, TC-1579, TC-1580 |
| TL-422 | Agreement with the outside reference on the corpus | tl-mltl FR-054 | TC-1573, TC-1574, TC-1576 |
| TL-423 | R2U2 differential on the numeric corpus with per-step verdicts | tl-mltl FR-058 | TC-1589, TC-1590, TC-1592: OPEN/BLOCKED, cannot be met by the specified work (see "Open and blocked acceptance criteria"); TC-1591 (nothing runs R2U2) is the only buildable row |
| TL-423 | Constructs C2PO cannot express are refused with a typed loss record | tl-mltl FR-056, FR-057 | TC-1583, TC-1584, TC-1587, TC-1588 |
| TL-424 | Expansion deterministic and bounded; oversized refused | FR-306, FR-307, tl-parse FR-032 | TC-1514..TC-1519, TC-1553 |
| TL-424 | Expanded form equivalent to the hand-written form on the corpus | FR-308, tl-mltl FR-054-AC-4 | TC-1520, TC-1521, TC-1577, TC-1552 |
| TL-425 | Contract form in tl-syntax, parsed by tl-parse, evaluated with three statuses (a per-position series with a summary; there is no trace-level status, see ADR-017 Open decisions) | FR-309, FR-310, tl-parse FR-032-AC-3, tl-mltl FR-059 | TC-1522..TC-1527, TC-1554, TC-1593..TC-1595 |
| TL-425 | Exported to C2PO contracts; R2U2 differential recorded | tl-mltl FR-060 | TC-1596, TC-1597 (decomposition export, itself refused until a profile is pinned); TC-1598: OPEN/BLOCKED (see "Open and blocked acceptance criteria") |
| TL-426 | Small-scope agreement with enumeration over small numeric domains | tl-solve FR-204 | TC-2014, TC-2015, TC-2016, TC-2017 |
| TL-426 | Unbounded or oversized domains refused, never approximated | tl-solve FR-202, FR-201-AC-3 | TC-2008, TC-2009, TC-2010 |
| TL-426 | Witness traces carry numeric values and are re-evaluated by tl-mltl | tl-solve FR-203 | TC-2011, TC-2012, TC-2013 |

## Open and blocked acceptance criteria

These ticket acceptance criteria are not satisfied by any work this project specifies.
They stay OPEN (or BLOCKED) until the evidence below exists; no row, plan or report
may mark them met, and a refusal is never counted as agreement.

| Ticket and criterion | State | Why the specified work cannot meet it | What unblocks it |
|---|---|---|---|
| TL-423 AC1: R2U2 differential on the numeric corpus with per-step verdicts (tl-mltl FR-058, TC-1589, TC-1590, TC-1592, TC-1601) | OPEN/BLOCKED | Nothing in any of the four repositories executes R2U2 or C2PO, and no numeric target profile is pinned; with ADR-014 (tl-mltl) and ADR-015 no numeric export is admitted until a pinned profile exists, so there is nothing to compare. | A human runs the pinned R2U2 checkout locally (`make r2u2-numeric-live`, never in `ci`), pins the profile from that run, and the bytes are retained under `corpus/r2u2-numeric-v1/`. |
| TL-425 AC2: exported to C2PO contracts, R2U2 differential recorded (tl-mltl FR-060, TC-1598, TC-1602) | OPEN/BLOCKED | Same reason; additionally the native C2PO contract syntax and R2U2's contract output encoding are UNGROUNDED, so the export is the three-formula decomposition only and its decoding is unverified. | The same retained exchange, plus the contract syntax pinned from it. |
| TL-419 AC: ADRs accepted; spec review recorded | OPEN (human) | Human acts. | Peter accepts or amends ADR-014..ADR-019 and the dispositions of the review. |

**The only deliverable for TL-423 and the export half of TL-425** is the
refusal-until-pinned path: every numeric predicate and every contract that contains
one is refused with `TargetNumericProfileUnpinned` and a typed loss record, Boolean
export is unchanged, and no agreement is claimed (tl-mltl FR-056, FR-057-AC-2).
The exact-rational admission algebra is tested with a synthetic profile that is
labelled synthetic and never said to describe R2U2.

Open decisions for Peter, with recommendations, are recorded in
[ADR-017](./assurance/ADR-017-assume-guarantee-contracts.md) (contract scope: A, B or C,
recommend C; who runs R2U2 and pins the profile; the hash and digest exemption) and
the register in [ADR-019](./assurance/ADR-019-identity-and-digest-register.md).
