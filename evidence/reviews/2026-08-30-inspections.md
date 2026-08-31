# tl-syntax v0.1 agent inspection record

Source subject: `a946311f6c0ff4916a13060b558f5957942eb9f0`

Provenance: performed by Codex on 2026-08-30. This is agent-produced evidence
and does not replace the human code review or source-release decision required
by AP-001.

| Obligation | Verdict | Evidence and limitation |
|---|---|---|
| FR-005-AC-3 | Pass | JSON corpus identities and expected results are checked-in strings, unsigned integers, Booleans, and arrays; the integration verifier rejects duplicate fixture identities and paths. Cross-platform downstream execution remains a downstream release obligation. |
| NFR-002-AC-2 | Pass | Every formula and proposition-map corpus document declares the supported v1 schema and every formula declares a supported profile; TC-014 verifies the complete manifest. |
| NFR-001-M-1 | Pass | `cargo tree --no-default-features --edges normal` reports only tl-syntax. |
| NFR-002-M-2 | Pass | Formula and proposition-map document kinds use closed v1 schema enums and have unknown-version rejection tests. |
> Superseded: this historical inspection does not discharge current-source
> obligations. See `2026-08-31-current-inspections.md` for the current record.
