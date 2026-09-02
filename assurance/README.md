# Shared assurance

Two files and no evidence.

`change-assurance.json` is what this repository *states* about the change under
[issue #12](https://github.com/agent-ix/tl-syntax/issues/12): the requirements it
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

## There is no compatibility answer any more, and that is deliberate

This repository used to retain 23 `quire.derivation-evidence/v1` envelopes and
read them, on every run, through
`engineering_assurance.verification_semantics.map_pgm01_bytes`. The mapping
answered `incompatible` for every one of them — "unknown PGM-01 schema version"
— because that family is one the PGM-01 programme governed but never defined.
That answer was reported as it stood and was never converted into a pass.

Those records were deleted under
[issue #12](https://github.com/agent-ix/tl-syntax/issues/12), on the
preservation constraint that
[engineering-assurance#7](https://github.com/agent-ix/engineering-assurance/issues/7)
released for the pre-stable phase on 2026-09-02 by owner decision. They were
deleted, not rewritten: nothing was backdated, re-sealed, or edited to look like
it still verifies, and the claim that this repository reads retained evidence
through the shared mapping was removed rather than weakened.
`agent-ix/engineering-assurance#21` becomes moot here rather than fixed. The
constraint re-applies unchanged at the move toward stable releases.
