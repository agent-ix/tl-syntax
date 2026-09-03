---
id: SR-013
title: Make execution-control false-success measurement
type: SpecReview
analysis: gap-analysis
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
At base `4cb578748c9f7f3353e4f32c8897aa10fd61b8b0`, one intentionally invalid Rust
item was introduced as the controlled fault. Without `.IGNORE:`, ordinary
`make ci CARGO_TARGET_DIR=target/cargo-review` stopped at `fmt-check` and exited
2. The diagnostic `make -k ci CARGO_TARGET_DIR=target/cargo-review` continued
far enough to classify the thirteen prerequisites: five completed cleanly, six
failed directly, and two were unmade because their shared producer prerequisite
failed; the command exited 2.

With a global `.IGNORE:` and the same fault, all recipes were attempted. Five
prerequisites still completed cleanly, while eight emitted an ignored failure
or consumed the invalid/empty producer output caused by one. GNU Make treated
all thirteen prerequisites as successful and `make ci` exited 0.

Both temporary mutations were removed immediately after measurement. They are
not part of this change. No hosted workflow was dispatched.

## Per-prerequisite result

| `ci` prerequisite | No `.IGNORE:` (`make -k ci`) | Global `.IGNORE:` (`make ci`) |
| --- | --- | --- |
| `fmt-check` | failed on the invalid item | error ignored; treated successful |
| `check-features` | failed on its first Cargo check | four Cargo errors ignored; treated successful |
| `check-default-dependencies` | clean | clean |
| `lint` | failed | error ignored; treated successful |
| `test` | unmade after `assurance-inputs` failed | Cargo test error ignored; treated successful |
| `check-corpus` | clean | clean |
| `conformance` | failed | error ignored; treated successful |
| `deny` | clean | clean |
| `audit-unsafe` | clean | clean |
| `spec` | clean | clean |
| `msrv` | failed | error ignored; treated successful |
| `rustdoc` | failed | error ignored; treated successful |
| `assurance` | unmade after `assurance-inputs` failed | empty conformance input made both mutation and chain checks fail; errors ignored and target treated successful |
| **Command result** | **5 clean; 6 failed; 2 unmade; exit 2** | **5 clean; 8 with ignored failure paths; all 13 treated successful; exit 0** |

Ordinary `make ci` without `.IGNORE:` was also run separately against the same
fault. It stopped immediately at `fmt-check` and exited 2, as expected; `-k` was
used only to observe the other independent prerequisite paths in one controlled
run.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1301 | medium | A global `.IGNORE:` changes an induced compile failure from `make ci` exit 2 to exit 0. Eight of thirteen prerequisite paths emit ignored failures or consume the resulting invalid producer output, yet Make treats every prerequisite as successful | `Makefile`, `agent-ix/tl-syntax#11` | correct-requirement-no-evidence |
| FND-1302 | low | Before this measurement, the Makefile and CLAUDE text bounded the structural backstop correctly, but NFR-002, AA-001, and the current change-assurance declaration did not carry the repository-specific measured result | `NFR-002`, `AA-001`, `assurance/change-assurance.json` | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1301 | **ACCEPTED** | The program owner accepts this for pre-stable development. Quoin continues to reject absent, empty, or failing charted producer bytes, but pure gates have no output it can bind. Use-specific qualification and independence remain open in `agent-ix/engineering-assurance#11`. No local guard is reintroduced. |
| FND-1302 | **FIXED** | The exact base, mutation, exit codes, five clean paths, and eight affected paths are now consistent in the Makefile header, NFR-002, AA-001, this review, CLAUDE.md, and `UNKNOWN-make-execution-control-guard-removed`. |

## Boundary of the result

This experiment demonstrates the false-success consequence of one real source
failure and the dependency paths it reaches. It does not claim that every Make
execution-control spelling was independently mutation-tested, that Make is a
qualified runner, or that Quoin or Quire executes tests. The repository's native
tools remain the producers; Quoin consumes and binds their structured output,
and Quire exports static specification facts.
