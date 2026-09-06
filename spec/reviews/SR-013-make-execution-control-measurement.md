---
id: SR-013
title: Make execution-control false-success measurement
type: SpecReview
analysis: evidence
scope: "agent-ix/tl-syntax#11; Makefile ci prerequisites at base 4cb5787; NFR-002; AA-001; assurance/change-assurance.json"
review_set: subset
relationships:
  - target: ix://agent-ix/tl-syntax/NFR-002
    type: reviews
  - target: ix://agent-ix/tl-syntax/AA-001
    type: references
---

# SR-013: Make execution-control false-success measurement

## Summary

The removed Make execution-control guard left a measured false-success path.
At base `4cb578748c9f7f3353e4f32c8897aa10fd61b8b0`, the intentionally invalid Rust
item `const MAKE_EXECUTION_CONTROL_PROBE: = ();` was appended to `src/lib.rs` as
the controlled fault. Without `.IGNORE:`, ordinary
`make ci CARGO_TARGET_DIR=target/cargo-review` stopped at `fmt-check` and exited
2. The diagnostic `make -k ci CARGO_TARGET_DIR=target/cargo-review` continued
far enough to classify the thirteen prerequisites: five were unaffected by the
selected compile fault, six failed directly, and two were unmade because their
shared producer prerequisite failed; the command exited 2.

With a global `.IGNORE:` and the same fault, all recipes were attempted. The
same five prerequisites were unaffected by that fault, while eight emitted an
ignored failure or consumed the invalid/empty producer output caused by one.
GNU Make treated all thirteen prerequisites as successful and `make ci` exited
0. This does not measure how the other five behave when faults reach them.

Both temporary mutations were removed immediately after measurement. They are
not part of this change. No hosted workflow was dispatched.

## Reproduction

The measurement was re-run in an isolated detached worktree at exact base
`4cb578748c9f7f3353e4f32c8897aa10fd61b8b0`:

1. Append `const MAKE_EXECUTION_CONTROL_PROBE: = ();` immediately after
   `CORPUS_REVISION` in `src/lib.rs`.
2. Run `make ci CARGO_TARGET_DIR=target/cargo-review`. It exits **2** after
   `fmt-check` reports “missing type for `const` item”.
3. Run `make -k ci CARGO_TARGET_DIR=target/cargo-review`. It exits **2** after
   observing six direct failures, two unmade paths, and five paths unaffected by
   this compile fault.
4. Insert a global `.IGNORE:` on its own line after the Makefile header and
   immediately before `CARGO ?= cargo`, leaving the invalid item in place.
5. Run `make ci CARGO_TARGET_DIR=target/cargo-review`. It exits **0** after the
   six direct failures, the failed producer feeding the two previously unmade
   paths, and both assurance-chain refusals are reported as ignored.
6. Remove `.IGNORE:` and the invalid const item. Verify the detached worktree is
   clean, then discard it.

The three commands were run in that order. The exact observed exits were
**2, 2, 0**. No hosted workflow was dispatched.

## Per-prerequisite result

| `ci` prerequisite | No `.IGNORE:` (`make -k ci`) | Global `.IGNORE:` (`make ci`) |
| --- | --- | --- |
| `fmt-check` | failed on the invalid item | error ignored; treated successful |
| `check-features` | failed on its first Cargo check | four Cargo errors ignored; treated successful |
| `check-default-dependencies` | unaffected by this compile fault | unaffected by this compile fault |
| `lint` | failed | error ignored; treated successful |
| `test` | unmade after `assurance-inputs` failed | Cargo test error ignored; treated successful |
| `check-corpus` | unaffected by this compile fault | unaffected by this compile fault |
| `conformance` | failed | error ignored; treated successful |
| `deny` | unaffected by this compile fault | unaffected by this compile fault |
| `audit-unsafe` | unaffected by this compile fault | unaffected by this compile fault |
| `spec` | unaffected by this compile fault | unaffected by this compile fault |
| `msrv` | failed | error ignored; treated successful |
| `rustdoc` | failed | error ignored; treated successful |
| `assurance` | unmade after `assurance-inputs` failed | empty conformance input made both mutation and chain checks fail; errors ignored and target treated successful |
| **Command result** | **5 unaffected by this fault; 6 failed; 2 unmade; exit 2** | **5 unaffected by this fault; 8 with ignored failure paths; all 13 treated successful; exit 0** |

Ordinary `make ci` without `.IGNORE:` was also run separately against the same
fault. It stopped immediately at `fmt-check` and exited 2, as expected; `-k` was
used only to observe the other independent prerequisite paths in one controlled
run.

## Quoin disposition observation

At exact PR head `73a5d69c3095aede9bf5e6f29ce9c22bf27c6d2e`, local
`make assurance-chain CARGO_TARGET_DIR=target/cargo-review` exited **0** and
reported the scenario as
`declared-unknowns-are-carried-not-dropped [inconclusive]: ok`. Quoin therefore
carries the `accepted` unknown into the unresolved,
inconclusive receipt state; it does not treat the declaration disposition as a
resolved unknown. The bracketed `inconclusive` text is the scenario's locally
declared expected-state label; the actual predicate reads the carried unknowns
and `unresolved_unknown` reason from Quoin's audited receipt. This is an
execution observation, not a local read site for the `disposition` field.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1301 | medium | A global `.IGNORE:` changes an induced compile failure from `make ci` exit 2 to exit 0. Eight of thirteen prerequisite paths emit ignored failures or consume the resulting invalid producer output, yet Make treats every prerequisite as successful | `Makefile`, `agent-ix/tl-syntax#11` | correct-requirement-no-evidence |
| FND-1302 | low | Before this measurement, the repository did not consistently state the exact base, fault, three command modes, exits, affected paths, record-production boundary, and unmeasured spellings | `Makefile`, `NFR-002`, `AA-001`, `assurance/change-assurance.json`, `CLAUDE.md` | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1301 | **ACCEPTED for the measured spelling** | The `tl-syntax-release-owner` accepts the observed global-`.IGNORE:` result for pre-stable development and must re-evaluate it before the first stable release candidate. When actually invoked, Quoin rejects absent, empty, or failing charted producer bytes; under `.IGNORE:` that refusal is suppressed and no record is produced. Seventeen other spellings, use-specific qualification, and independence remain open in `agent-ix/engineering-assurance#11`. No local guard is reintroduced. |
| FND-1302 | **FIXED for the measured spelling** | The exact base, fault, command order, exit codes, five paths unaffected by that fault, eight affected paths, no-local-record boundary, and seventeen unmeasured spellings are now stated consistently. This does not close the open qualification challenge. |

## Boundary of the result

This experiment demonstrates the false-success consequence of one real source
failure and the dependency paths it reaches. Exactly one of the program's
eighteen execution-control spellings was exercised: a global `.IGNORE:`. The
other seventeen remain unmeasured here. Quiet spellings such as
`.SHELLFLAGS := -c true` can print plausible commands without executing them,
so review of local output is not a compensating control. The five paths listed
as unaffected were never made to fail; their behaviour under `.IGNORE:` is
unmeasured.

TC-026 in `tests/shared_assurance.rs` intentionally rejects the removed local
guard names. That keeps the gap open by owner decision: this repository has no
test that goes red for the general Make execution-control class, and a local
guard reintroduced under those names makes the suite fail. The class is owned
as an open AA-001 challenge and by `agent-ix/tl-syntax#11`; unlike the sibling
temporal repositories, it has no dedicated NFR or TC here.

The change declaration has two different boundaries. Only its `record` object
is projected into the sealed Quoin record. Top-level metadata—including
`purpose`, `derived_fields`, and the `sources` path map—is unsealed. Within the
sealed record, `subject.scope` is declarative and has no local completeness read
site; sealed source connections carry the derived digest but not the source
path. This experiment does not claim those declaration gaps are closed; their
shared-contract migration is tracked in `agent-ix/tl-syntax#16`.

This experiment does not claim that Make is a qualified runner or that Quoin or
Quire executes tests. The repository's native tools remain the producers; Quoin
consumes and binds their structured output when a record is actually produced,
and Quire exports static specification facts. This measurement document has no
self-authored code-review counterpart; the independent code review is the PR
review of record.
