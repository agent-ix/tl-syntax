# PLAN-007 prerequisite admission (TL-24)

This is a consumer ledger, not a compatibility matrix or a source-readiness
result. It is not yet bound to a complete immutable TL-22 candidate. Each row
admits only its named use; the Task-018 gate remains blocked.

| Consumed capability | Immutable release and integrity premise | Compatibility and authority | Resume decision |
| --- | --- | --- | --- |
| Quire assurance-v1 source export | `@agent-ix/quire-cli@0.33.0`, npm SHA-512 `ylGZBZz09ficl8Q/a6JsUkw3ZWJgOQaFszGaxaexbzqtA13MKshfEpIDgGmXcxGAT1GjQ0gZTjUsncxtPgPwXA==`; source revision `5d027b057c8976a29175b3f3ebfc91088bc837d9`, engine quire-rs `0.47.1` at `92dbebc49f354f7a3d5050b94cd2d57d9084d99f` | Engineering Assurance v0.4.1 accepted matrix classifies this exact CLI/engine set as compatible. Matrix SHA-256 `43f76217b51df16b0249f0cb6abbf3e76a0181c77b1ad6da10d2a3f633cbdd43`, carried by annotated tag `v0.4.1` at `2fd04e6bf47727cb91b7d39daaedfd1a1051d238`; acceptance attributed to Peter Krenesky, 2026-09-23. | Available for TL-63 source projection. This does not attest a fresh export or complete candidate scope. |
| Quire process specification module | `agent-ix/spec-artifacts-process@v0.26.0`, peeled revision `6b7dd401065121f758ce4dd3135fe730cdf80dd9`; module version `0.2.0`; every consumed archetype/schema digest is listed in `source-grounding-premises.json` | The released module supplies the Quire-authored artifact and obligation vocabulary. The v0.4.1 Engineering Assurance matrix does not separately classify this module; TL-63 checks each export's reported schema digests against the recorded release premise. | Available only for the TL-63 exact module/schema projection. External package integrity and candidate-bound admission remain open. |
| Quoin change-assurance intake | `@agent-ix/quoin@0.24.1`, npm SHA-512 `5YMnXXZ4J+Q01stGT9LoqIRZYZIHH21X4MErYAxcU9I/0mIlhloZDgaquDU/alyidaP59gK98GUtI8C1HHJpyQ==` | The same accepted v0.4.1 matrix classifies this exact version as compatible. A local exact-head probe accepted the TL-63 projected record body; that probe is not a retention-lifecycle qualification. | Available to seal the source-connected record. Durable backend, operator, retrieval and lifecycle are unselected, so FR-016 retention cannot resume. |
| Engineering Assurance Rust producer execution | `agent-ix/engineering-assurance@v0.4.1`, tagged revision above; `src/producer_execution.rs` SHA-256 `722055ff48515f73f2a84a6c04c205fde688331bebe8c6cbcda0d6d958522a13` | The released source distribution contains the versioned Rust request/result protocol and `producer-execution` feature. The accepted matrix covers the release, but no tl-syntax consumer binding or exact producer invocation exists. | Shared executor exists for TL-23/TL-22 planning; their domain adapter and parity/freshness work have not resumed. |
| Quoin retention selection | Quoin 0.24.1 supplies retention functions, but no selected durable backend, operator, retrieval policy or lifecycle identity is recorded for this source-readiness subject. | No authority selection to evaluate. | Unavailable for FR-016 and the later real handoff. No local store is substituted. |
| Human reviewer and decision admission | `ix-flow@0.2.3` has npm SHA-512 `tmFIMgOhGS4Tova901cm/yRUANBKhRT3L9Sz/SNxZig1XXR8BGtuXXoETcU6kXqynVLg8uHMyoZ7+AM74Ig/BQ==` and is compatible in the v0.4.1 matrix. The exact reviewer policy, actor set and authoritative event source for TL V1 remain unselected. | The ix-flow release alone is not a decision-policy selection or human event. | Unavailable for FR-018. No local approval registry or synthetic event is substituted. |
| Engineering Assurance integrator package | No released package writer/reader contract with an accepted tl-syntax compatibility decision has been identified. | No package format or reader authority to admit. | Unavailable for FR-017 and IT-002. No local package format is substituted. |
| LR08 executable-language dispositions | `agent-ix/quire-research#64` remains open. Existing Quoin implementation is the recorded accommodation; Filament is excluded. | The owner has required exact path inventory and scoped dispositions. | TL-23 census may be prepared, but stable-required legacy paths cannot be marked remediated or approved. |

The global prerequisite gate is **blocked**. TL-63 is also still a draft
consumer projection: the existing `make assurance` path has not been migrated,
and no exact fresh producer attestation or complete scope claim is made. The
ledger must be rebound to one immutable candidate/configuration by TL-22; it
cannot by itself make downstream work ready.

## Standalone producer-execution boundary

The intended consumer boundary is a separate EA CLI tool, not an EA Rust
dependency in the published MIT/Apache tl-syntax library. Tool use does not
require changing that library's license. Inspection of the admitted v0.4.1
`src/main.rs` command registration and dispatch found no producer-execution
command; the released capability is currently exposed through the Rust library.
EA#34 therefore establishes executor availability, but does not establish a
standalone CLI consumer contract for this use.

Engineering Assurance owns the missing released CLI entry point and its versioned
request/result transport. Until that boundary is identified and admitted,
TL-22's shared-executor consumer binding remains unavailable. This repository
will not wrap the library in a local runner or silently expand its dependency
license policy. The released executor's Linux-only execution boundary is a
separate host limitation; a CLI does not make execution available on macOS.
