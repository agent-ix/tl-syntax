# Schemas

Two files, both frozen.

| File | Class |
|---|---|
| `tl-syntax-evidence-input-v1.schema.json` | Frozen historical. Read never, written never. |
| `tl-syntax-evidence-manifest-v1.schema.json` | Frozen historical. Read never, written never. |

## Nothing validates against these any more

The collector, envelope builder, and per-record verifiers that used them were
removed by `agent-ix/tl-syntax#9`. Quoin owns the record, attestation, and
receipt shapes now, and it ships them itself — `quoin change-assurance schema`
prints the three normative assets byte-for-byte as packaged, so a producer
validates against the same file the sealing code was written against rather than
against a local copy that has drifted.

## They are not deleted, and the reason is specific rather than sentimental

Every one of the 23 retained envelopes under `evidence/` names both files, by
path and by SHA-256:

```json
"inputs":  [{ "schema": { "id": "tl-syntax.evidence-input",    "version": "v1",
              "digest": { "value": "e6c1d95a…c51cf8c0" } } }],
"outputs": [{ "schema": { "id": "tl-syntax.evidence-manifest", "version": "v1",
              "digest": { "value": "3a3124a5…cb03059ef" } } }]
```

Deleting them would not remove a generic evidence family from this repository.
The family was the *verifier*, and that is what went. It would instead break a
reference inside bytes the migration is required to leave untouched.

Three earlier digests for the input schema also appear in older records
(`959f705c…`, `786defd4…`, `48c13e0c…`). Those revisions were superseded in the
tree during PR #6 and their bytes exist only in Git history. That is a
pre-existing fact about those records, not something this migration changed, and
it is recorded here rather than repaired, because repairing it would mean editing
retained bytes.

## The freeze is enforced, not described

`tests/shared_assurance.rs::no_local_evidence_framework_remains_and_the_frozen_schemas_are_referenced_by_nothing`
pins both files by digest and asserts that no source file under `scripts/`,
`tests/`, or `examples/` mentions either name. The census size is asserted too,
so the claim cannot pass by inspecting nothing.
