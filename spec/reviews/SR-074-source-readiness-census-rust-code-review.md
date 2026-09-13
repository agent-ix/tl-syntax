---
id: SR-074
title: "Independent Rust review of the source-readiness census update"
type: SpecReview
analysis: code-review
scope: "tests/shared_assurance.rs diff from origin/main through 33678fa"
review_set: subset
---

## Summary

The branch changes no production Rust API. The `tests/shared_assurance.rs`
source-census union was reviewed under `agent-skills/rust-review/SKILL.md` for
fail-closed behavior, test load-bearing value, panic scope, unsafe, blocking and
resource surfaces. The exact expected population and area counts pass; all
panics and filesystem/process operations remain test-only; no unsafe block,
async/lock change, wire conversion or production resource bound is introduced.

At `33678fa`, the complete local `make ci` composite passes without hosted CI:
formatting, feature matrix, strict Clippy, 44 Rust tests plus one doctest,
corpus/digest/oracle gates, cargo-deny, fuzz-target build, unsafe audit, strict
spec validation, MSRV tests, rustdoc, pin checks, mutation probes and the shared
assurance chain. Cargo-deny reports only its pre-existing unused-license and
fuzz path-dependency warnings.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-7401 | high | The branch initially excluded every path below top-level `plan/`, allowing executable source placed there to evade the live census. The exemption is now limited to Markdown plan/review records, and a tracked extensionless `plan/run` fixture proves the executable remains live. | `is_archival_record`, `live_source_enumeration_has_an_exact_fail_closed_partition` | implementation-bug-despite-evidence |
| FND-7402 | low | Review and plan Markdown remain deliberately archival so adding an independent review cannot invalidate the exact source population it reviewed. Executable content still belongs to the separate FR-018 exhaustive invocation census. | FR-018, source census | correct-requirement-no-evidence |
