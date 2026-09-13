---
id: PLAN-009
title: Paired W/M source and canonical-graph corpus plan
type: Plan
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/FR-010
    type: references
  - target: ix://agent-ix/tl-syntax/TM-002
    type: references
---

# PLAN-009: Paired W/M source and canonical-graph corpus plan

## Objective

Implement `agent-ix/tl-syntax#41` by adding the paired derived-source and
canonical-document corpus that FR-010 names. Issue #41 owns TC-074: a
digest-pinned corpus, `tl-syntax.future-operator-corpus/v1`, replayed through
the tl-syntax lowering API with direct, derived, refused, and malformed cases
and mutation controls.

## Base

Branch `issue/41-wm-corpus` starts at mainline `8dc18ee`, which carries the
FR-008 lowering API from issue #40. Derived-source expectations were checked
against `tl-parse` `9ca856b` (`parse` and `parse_clean_ascii_v2`) outside this
repository; tl-syntax does not depend on tl-parse. Issue #41 was dispatched in
parallel with tl-mltl#47 and tl-rewrite#35: it consumes no output of either and
lands after them.

## Work sequence

1. Allocate TC-074 and PLAN-009 after scanning remote branches, add TC-074 to
   TM-002 and FR-010, and move the TM-002 paired-corpus cells from TC-045 to
   TC-074.
2. Add `corpus/future-operators/` with a manifest, cases, span-free expected
   formula-v1 documents, and a `SHA256SUMS` file; add its digest check to
   `make check-corpus`.
3. Add `tests/future_operator_corpus.rs`: verify digests, bind source spans to
   source text, build derived and direct documents through `Formula::new` and
   `FutureLoweringRequest::lower`, compare them with the shared expectation,
   replay refused and malformed cases, and run the mutation controls.
4. Update the exact live-source census and the repository layout notes.
5. Run the complete local gate at the exact final head, then the PR-time Rust
   review and gap analysis. Hosted CI remains manual-only and is not dispatched.

## Exit criteria

1. Every derived-source case and its direct pair produce one shared expected
   document under both semantic profiles; W and M each cover `[0,0]` and
   `[u32::MAX,u32::MAX]` under both profiles, with left-associative and
   right-nested lowerings. One primitive `tl-parse.clean-ascii/v1` source case
   shares its document with a direct case.
2. Refused and malformed cases fail with their declared codes and produce no
   document.
3. Each enumerated mutation dimension turns the replay red with a specific
   error code.
4. TC-074 is implemented; FR-010 stays planned until TC-044 and TC-045 land.
