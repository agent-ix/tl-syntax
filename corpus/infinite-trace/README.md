# Infinite-trace V1 corpus

`cases.json` is the owner corpus for `tl-syntax.infinite-trace-corpus/v1`.
Every case carries its formula-unbounded document, lasso input, optional
fairness premises, an expected result or typed refusal, and a derivation
written without consulting the production evaluator. Negative cases are
intentionally invalid inputs. `schema.json` checks the corpus envelope;
the reviewed Rust decoders enforce the formula, lasso, clock, valuation, and
fairness invariants in the TL-207 matrix.

Positions are discrete event positions. A lasso with prefix `u` and nonempty
loop `v` denotes `u·vʷ`. A `missing` valuation permits either Boolean value;
`conflicting` records incompatible evidence and is never treated as missing.
The one fairness form in V1 asks that its referenced formula hold infinitely
often. `SHA256SUMS` and `manifest.json` pin the retained bytes.

Cases without `subject_kind` select the complete lasso. The
`finite-prefix-globally-inconclusive` case selects only the materialized prefix
of its otherwise valid lasso document; the stored loop is outside that selected
subject. This makes the scope difference explicit without minting a finite-prefix
wire edition in tl-syntax.

This corpus is test input. It is not a language authority or a production
oracle. Consumers use `tl_syntax::CORPUS_DIR` at their pinned tl-syntax revision.
