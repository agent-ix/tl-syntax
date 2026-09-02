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

Retained evidence is sealed by a per-record `.sha256` manifest. Collection
automatically appends that manifest's digest to `evidence/ANCHORS`. These
in-repository anchors are protected by Git history and pull-request review;
they are an integrity/audit boundary, not an external timestamp or independent
attestation. Regenerating a record, manifest, or anchor requires a reviewed
diff, and the human maintainer remains the release authority.
