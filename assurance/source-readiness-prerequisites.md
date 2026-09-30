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

## TL-23 executable population preparation

Read-only inspection at candidate `c526700` found the following entry-point
families. This is an authorial inventory preparation, not the executable
classifier, an approved disposition, or proof of exhaustiveness. Each family
must be expanded to individual entry points/runtime invocations in Task-019.
The shared LR08 policy is recorded in quire-research#64; it grants no blanket
external-tool, host, glue or test exemption.

| Tracked source / construction | Language and purpose | Owner and proposed treatment | Evidence / resume condition |
| --- | --- | --- | --- |
| `src/lib.rs`, reachable modules under `src/` | Rust production syntax/profile/document contracts and embedded unit tests | tl-syntax; domain Rust | Existing implementation remains; per-entry census is outstanding. |
| `src/bin/source_grounding.rs::main` | Rust source-connected record projection | tl-syntax; domain Rust | PR #96 focused adverse cases; no full scope/freshness claim. |
| `src/bin/conformance_adapter.rs::main` | Rust corpus-result transcription | tl-syntax; domain Rust | PR #96 real 22-entry parity; malformed-stream refusal. |
| `examples/corpus_conformance.rs::main` | Rust temporal corpus producer | tl-syntax; domain Rust | Existing domain producer; exact shared execution binding remains TL-22 work. |
| Every `tests/*.rs` harness and each test function | Rust domain assertions and shared-intake checks | tl-syntax; domain Rust, with nested runtime paths separately classified | Rust file ownership does not dispose subprocesses or generated code. |
| `tests/v1_spec_stubs.rs::pending_v1_case!` expansion | Rust generated ignored test for TC-162 | tl-syntax; domain Rust | Macro-generated test remains in population; ignored assertion is not feature evidence. |
| `fuzz/fuzz_targets/wire_decode.rs`, `infinite_wire_decode.rs` | Rust executable fuzz targets declared by `fuzz/Cargo.toml` | tl-syntax; domain Rust | Inventory only; no campaign is authorized in this phase. |
| `scripts/assurance_chain.py` CLI, including `--adapt` and mutation mode | Python result mapping, environment observations and Quoin orchestration | tl-syntax; temporary legacy, shared/domain split required | Native corpus transcription is replaced only at `assurance-record`; full-chain parity is absent. |
| `scripts/check_shared_pins.py` | Python package/tool observations plus shared compatibility invocation | tl-syntax observation adapter; EA owns classification | Consume admitted EA compatibility CLI rather than duplicate its matrix; immutable consumer binding outstanding. |
| `scripts/check_default_dependencies.py` | Python default dependency and four feature-combination checks | tl-syntax owns domain interpretation; EA owns bounded execution | Separate domain response adaptation from missing shared CLI execution; no local runner port. |
| `scripts/validate_corpus.py` | Python schema/digest/oracle domain validation | tl-syntax; temporary legacy pending domain Rust replacement | Positive/adverse parity required before removal; schema-runtime/shared boundaries must be resolved. |
| `scripts/test_corpus_gate.py` | Python subprocess mutation assertions over corpus validation | tl-syntax; temporary legacy | Nested Python invocation remains executable; no campaign expansion in this phase. |
| `scripts/check_spec_id_uniqueness.py` | Python specification ID validation | Quire owns reusable specification semantics; tl-syntax currently carries legacy entry point | Identify admitted shared equivalent or upstream missing capability before removing it. |
| `scripts/test_check_spec_id_uniqueness.py` | Python uniqueness-check assertions | tl-syntax; temporary legacy | Requires Rust/shared positive and adverse parity; not disposed merely because it is a test. |
| `scripts/check_unsafe_comments.sh` and nested `grep`, `sed`, `sort` invocations | Bash source safety-comment audit and baseline writer | tl-syntax; temporary legacy; reusable source-audit boundary unresolved | `--update-baseline` is a separate executable use, not data; no scoped owner approval identified. |
| Make recipes for every named target, plus `$(shell git rev-parse HEAD)` | Make/shell orchestration and dynamically selected tool variables | tl-syntax owns recipes; tool owners own invoked semantics | Expand each recipe command and variable-selected invocation; defaults do not prove resolved runtime identity. |
| Make `assurance-env`: Python `-m venv` and generated environment `pip` | Python environment/package construction | tl-syntax legacy orchestration; external tool dispositions required | Existing two-lane policy is not a Rust-language exception. |
| Make `assurance-inputs`, `assurance-record`, `source-grounding-record` | Shell redirects, Cargo/domain producers, Quire and Quoin consumers | tl-syntax domain adapters; shared owners retain execution/export/evidence semantics | Each nested command needs its own row; producer-free projection is distinct from producer execution. |
| `tests/feature_boundary.rs` nested `make` | Rust test invokes Make, then Cargo and Python recipes | tl-syntax; temporary legacy invocation underneath Rust test | A Rust caller does not remediate non-Rust behavior. |
| `tests/shared_assurance.rs::run`, `run_chain_with_path` | Dynamic interpreter/tool selection; Python chain, Quoin, ix-flow, Git probes | tl-syntax adapter; shared tool and host dispositions outstanding | Resolve every caller/argument construction rather than approve generic `run(program, args)`. |
| `tests/shared_assurance.rs::producer_shims` | Rust writes executable `#!/bin/sh` scripts containing `case`, `echo`, `exit` | tl-syntax; generated temporary legacy | Generated shell is executable population even though authoring source is Rust. |
| `tests/shared_assurance.rs` Git init/add/config/update-index/diff probes | Rust invokes Git for candidate fixtures and index observations | tl-syntax owns assertions; Git host disposition outstanding | Separate each runtime invocation; no blanket external-tool exception. |
| `tests/future_operator_corpus.rs`, `past_history_corpus.rs`, `shared_assurance.rs` digest probes | Rust invokes `sha256sum` | tl-syntax owns checks; external host disposition outstanding | Either scoped disposition or Rust domain hashing with parity; caller language alone is insufficient. |
| `.github/workflows/ci.yml` inline `run` blocks | Shell, pip, npm, Cargo, Make and shared-tool invocation | tl-syntax orchestration; temporary legacy and external-tool dispositions outstanding | Includes npm-installed launchers/module operations, not just `make ci`; hosted dispatch remains unauthorized here. |
| `.github/workflows/ci.yml` `uses` entries | External checkout/toolchain/cache/install actions with runtime implementations | Action owners; missing scoped retained-tool dispositions | Their names and versions do not establish accepted language/runtime exceptions. |
| `.github/workflows/cla.yml` reusable workflow `@main` | External generated workflow execution for CLA handling | agent-ix/.github owns workflow; TL owner owns consumption scope | Mutable reference and runtime expansion remain unresolved; no automatic classification as data. |
| Cargo dependency/procedural macro/build behavior, including dev dependencies | External runtime/build-time executable supply chain | Package owners; scoped tool/runtime dispositions outstanding | Cargo manifests/locks identify packages, not complete executable construction or approved dispositions. |

Schema JSON, corpus formula/trace JSON, digest lists and
`scripts/unsafe_comment_baseline.txt` are data inputs, not standalone executable
entry points. Their consumers stay in the population. `include_str!` embeds data
and documentation; `pending_v1_case!` constructs executable tests. Inspection
found only three Git executable-mode files (the ID checker, its test and the
unsafe-comment shell audit), while five additional Python script files are
interpreter-invoked. Executable mode and extension counts therefore cannot
establish this census.

No family above is owner-dispositioned by this document. Unresolved dynamic
constructions stay unclassified. The exact-census/refusal implementation,
individual row expansion, scoped owner decisions and complete parity remain
unimplemented; Task-019 remains blocked.
