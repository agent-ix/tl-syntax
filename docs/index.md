# tl-syntax documentation

tl-syntax is the syntax tree, semantic profile and signal model for discrete bounded Mission-time Linear Temporal Logic (MLTL). It is the base crate of the TL family. This directory documents what the crate contains and how to use it. The [README](../README.md) has the overview and the smallest example.

| Page | What it covers |
| --- | --- |
| [how-it-works.md](how-it-works.md) | The formula graph, semantic profiles, signal catalogs, strict documents, and where the crate sits in the pipeline from text to a Quire outcome, and this crate's step in a worked debugging session. |
| [examples.md](examples.md) | Runnable examples: refusals, JSON documents and identities, derived-operator lowering. |
| [api-reference.md](api-reference.md) | Public types and functions by module, with their error types. |
| [wire-formats.md](wire-formats.md) | Document schemas, canonical encoding, content identity, resource limits and fuzzing. |
| [corpus.md](corpus.md) | The checked-in conformance corpora and the commands that replay them. |
| [features.md](features.md) | The `alloc` and `serde` features and the `no_std` guarantee. |

`docs/plan-history/` holds the delivery plans for earlier versions. They are a record of past work and do not describe the current API.
