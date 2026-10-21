---
id: SR-541
title: "Rust review — STD-228 corpus and dependency gates"
type: SpecReview
analysis: code-review
scope: "tl-syntax STD-228; examples/corpus_gate.rs, examples/default_dependencies.rs, Cargo.toml, Cargo.lock, Makefile, corpus/schema/formula-v1.schema.json, src/formula/document.rs, corpus/README.md, CLAUDE.md, spec/evidence/suites.md, removed Python scripts"
review_set: subset
---

## Summary

Reviewed the STD-228 diff against the repository Rust rules and the rust-review
checklist. The Rust gate compiles the checked-in Draft 7 schemas, checks their
declared semantic contract, classifies the manifest's invalid formulas, and
derives horizons and finite-trace oracles. The dependency gate checks the real
default normal Cargo tree. The Make targets execute both gates and their
behavioral tests. No CI workflow was changed.

## Verdict

**PASS after remediation.** No finding remains in the scoped diff. Full gate
results are recorded with the PR after execution.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5411 | medium | A zero-offset `until` witness originally evaluated the left operand at the witness instant, so `false U[0,0] true` was rejected. The prefix must be empty when witness equals the current instant. | `examples/corpus_gate.rs` |
| FND-5412 | low | A bare `u64` to `usize` trace-index cast could truncate on 32-bit targets and inspect the wrong instant. | `examples/corpus_gate.rs` |

## Dispositions

- FND-5411: fixed with a prefix helper and a zero-witness regression test.
- FND-5412: fixed with checked conversion before trace lookup.

## Rust checklist

- The gate binaries introduce no unsafe or new public library API.
- Formula operators are matched exhaustively for the v1 vocabulary; later
  operators produce an explicit error instead of a default semantic result.
- Horizon addition is checked, corpus paths are confined to the requested
  corpus directory, and closed-trace candidate instants collapse the
  out-of-trace suffix to one representative instant.
- The mutation test changes a real checked-in oracle and exercises the gate's
  verification path. The dependency test adds a normal dependency to a sample
  Cargo tree and checks that the gate rejects it.
