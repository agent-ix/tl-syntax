# Paired W/M source and canonical-graph corpus

`tl-syntax.future-operator-corpus/v1`, revision 1, is the paired corpus
FR-010 names and TC-074 replays with `tests/future_operator_corpus.rs`. It is
separate from `tl-syntax-corpus/v1` in the parent directory, which is
unchanged.

The corpus is evidence input. It is not an evaluator, not an editable source
language, and it adds no derived formula-v1 node. The replay builds every graph
through the tl-syntax lowering API (`Formula::new` and
`FutureLoweringRequest::lower`).

## Files

- `manifest.json` names the corpus, operator-profile, request, formula-schema,
  and derived-dialect identities, and pins every replayed file by SHA-256. The
  test pins the manifest digest.
- `cases.json` holds every case.
- `expected/*.json` are span-free canonical formula-v1 documents. Each one is
  shared by one derived-source case and one directly constructed case.
- `SHA256SUMS` repeats the digests for `make check-corpus`.

## Case classes

- `derived` binds `tl-parse.clean-ascii/v2`, the operator profile, a semantic
  profile, and source text to ordered `append` (primitive formula-v1 node) and
  `lower` (W or M request fields) steps. The replay checks that every span reads
  the bound source text, that each report matches `expected_lowerings`, and
  that the built graph equals the expected document without spans.
- `direct` appends only span-free primitive nodes and must equal the same
  expected document exactly.
- `refused` ends in a lower step that must refuse with the recorded code and
  axis; it produces no document. Only refused cases may override the request,
  operator-profile, or semantic-profile identity.
- `malformed` wraps an entry that must fail replay with the recorded error
  code, such as a derived wire node or a derived step under
  `tl-parse.clean-ascii/v1`.

Derived-source spans and lowering records were checked against `tl-parse`
`parse_clean_ascii_v2` at `9ca856b`. tl-syntax does not depend on tl-parse.

## Changing the corpus

Edit the case or document, update its digest in `manifest.json` and
`SHA256SUMS`, then update `MANIFEST_SHA256` in the test. The mutation controls
in the test show that a changed expectation without a matching graph turns the
replay red.
