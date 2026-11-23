# tl-syntax.numeric-corpus/v1

Shared numeric predicate evidence (FR-305), read by tl-parse, tl-mltl and
tl-solve through `tl_syntax::CORPUS_DIR` instead of vendored copies.

- `manifest.json` lists every case with its class, files and expected outcome
  (derived types and a truth table, or a refusal class) and the closed
  per-class counts. It records no per-file digest (ADR-019 register item 2):
  the corpus is immutable by its version identity, and any changed case needs
  `tl-syntax.numeric-corpus/v2`.
- `provenance/generate.py` is the independent generator. It shares no code
  with the crate; truth tables use exact rational arithmetic.
- tl-syntax evaluates nothing: it checks the classes, files, counts, refusal
  classes and derived types (`tests/numeric_predicates.rs`); tl-mltl FR-054
  replays the truth tables.
