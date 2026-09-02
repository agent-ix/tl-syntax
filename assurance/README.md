# Shared assurance

Two files and no evidence.

`change-assurance.json` is what this repository *states* about the change under
[issue #9](https://github.com/agent-ix/tl-syntax/issues/9): the requirements it
claims to meet, the things it promises not to break, the proofs it offers, and
the questions it cannot answer. `pins.json` is the Engineering Assurance
release it adopts and the digests of the artifacts it actually reads from that
release.

## Why there is no evidence in here

Because retention is Quoin's job. `make assurance` seals the declaration into a
Quoin change-assurance record, seals a proof attestation over each producer's
already-written result file, hands those bytes to Quoin's intake, and asks for a
verification receipt. The record, the attestations, the retained bytes, and the
receipt all live in Quoin's store under `target/`, which is ignored.

The repository that produced a result does not also get to be the place that
result is kept, digested, and pronounced upon. That arrangement is the thing
this migration removed, and putting a smaller version of it back under a new
directory name would be the same mistake in a nicer font.

## What runs what

One target produces:

```
make assurance-inputs
```

It runs the crate's own conformance example over the shared temporal corpus, the
corpus semantic oracle, the feature-boundary gate, and `quire coverage`, and
writes their structured output to `target/assurance/`.

Everything downstream consumes those files. `scripts/assurance_chain.py` refuses
to run a producer; if an input is missing it says so and names the target that
makes it. Quire exports and does not execute. Quoin transcribes and does not
execute. That separation is asserted by a test, not just described here.

## The decision that is not here

A verification receipt for this change reads `incomplete`, and the reason it
gives is that no human decision event exists. That is correct. An ix-flow
decision is an attributed human act; only the repository owner can create one,
and an agent that synthesized one would be forging the single field in the whole
chain that exists to say a person looked.

## The compatibility answer, stated plainly

This repository never retained a `quire.pgm01-evidence` record. Its 23 retained
envelopes are `quire.derivation-evidence/v1` — a different schema family, which
the PGM-01 programme governed but did not define. The pinned mapping
`engineering_assurance.verification_semantics.map_pgm01_bytes` therefore answers
`incompatible` for every one of them, with the reason "unknown PGM-01 schema
version".

That is the mapping declining to interpret a shape it has never seen, which is
exactly what it should do and is one of the twelve states this migration is
required to keep distinguishable. It is not a pass, it is not a failure of these
records, and it is not a licence to write a local mapper that would return a
friendlier answer. The gap is filed upstream as
`agent-ix/engineering-assurance#21`.

The mapping is shown to accept as well as refuse: the pinned release's own
`fixtures/verification-semantics/pgm01-v1.json` and `pgm01-v2.json` are read as
positive controls in the same run. A refusal that has never been seen to accept
is indistinguishable from a step that never worked.
