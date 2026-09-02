# Contributing

Contributions are welcome from people using any development method, including
agent-assisted workflows. The standard is the same for every contribution:

- requirements and acceptance criteria are updated before implementation;
- the repository's specification, review, test, and assurance gates pass;
- source and third-party provenance remain truthful and reviewable;
- a human maintainer reviews the pull request and owns release decisions.

Do not push directly to `main`. Generated artifacts must retain their declared
derivation metadata and licensing. Do not copy material from repositories or
documents whose license does not permit reuse.

This repository retains no evidence of its own. `make assurance` hands each
producer's already-written result to Quoin, which seals it, retains it, and
computes its digests; the record, the attestations, the retained bytes, and the
receipt live in Quoin's store under `target/`. Neither that store nor any
in-repository anchor is an external timestamp or an independent attestation, and
the human maintainer remains the release authority.
