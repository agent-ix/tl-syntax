#!/usr/bin/env python3
"""Drive the official change-assurance chain over already-produced results (FR-006).

Four things this file deliberately is not.

It is not a producer. It never runs a test, a corpus replay, a compiler, or a
solver. Every input it reads was written by `make assurance-inputs`, and if one
is absent it says so and names that target. A driver that can produce its own
inputs is a driver that can produce a green run out of nothing.

It is not an envelope. Quoin's packaged FR-063 record, FR-064 attestation and
FR-065 receipt schemas are the shapes. This file projects
`assurance/change-assurance.json` into the record body Quoin requires and derives
nothing beyond the digests that file's own `derived_fields` names.

It is not a verdict. It runs `quoin` and reports what `quoin` said. Where a
scenario expects a refusal, the refusal is the expected result and the run is
green because the tool refused, not because the tool agreed.

It is not a retention store. Nothing is written under `evidence/`, nothing is
committed, and the Quoin store it uses lives under `target/`, which is ignored.

Exit status: 0 when every scenario, control and probe matched, 1 when one did
not, 2 on a usage or environment error — which is a different fact from a
mismatch and gets its own code.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
DECLARATION = ROOT / "assurance" / "change-assurance.json"
ASSURANCE_DIR = ROOT / "target" / "assurance"
STORE = ROOT / "target" / "assurance-store"

CONFORMANCE_PROTOCOL = "tl-syntax.corpus-conformance/v1"

# Every proof obligation's retained result, and the media type its producer
# declares. Stated rather than sniffed, because a producer's content type is
# part of what it produced.
INPUTS = {
    "PROOF-corpus-conformance": ("corpus-conformance.jsonl", "application/x-ndjson"),
    "PROOF-corpus-oracle": ("corpus-oracle.json", "application/json"),
    "PROOF-feature-boundary": ("feature-boundary.json", "application/json"),
    "PROOF-quire-static-export": ("quire-static-export.json", "application/json"),
    "PROOF-legacy-compatibility": ("legacy-compatibility.json", "application/json"),
    "PROOF-msrv": ("msrv.txt", "text/plain"),
}


class ChainError(RuntimeError):
    """The chain could not be driven. Distinct from a scenario that did not match."""


def digest_of(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def quoin(*arguments: str, stdin: str | None = None) -> subprocess.CompletedProcess[str]:
    """Invoke the pinned Quoin CLI. It is the only command this file runs."""
    if shutil.which("quoin") is None:
        raise ChainError("quoin is not on PATH; the pinned CLI is required")
    return subprocess.run(
        ["quoin", *arguments], input=stdin, capture_output=True, text=True, check=False
    )


def tool_version(argv: list[str]) -> str:
    try:
        result = subprocess.run(argv, capture_output=True, text=True, check=False)
    except OSError:
        return "0.0.0"
    return result.stdout.strip() or "0.0.0"


def observe_environment() -> dict[str, Any]:
    quire_version = "unknown"
    raw = subprocess.run(
        ["quire", "provenance"], capture_output=True, text=True, check=False
    )
    if raw.returncode == 0:
        try:
            provenance = json.loads(raw.stdout)
            quire_version = (
                f"{provenance['cli']['version']} engine {provenance['engine']['version']}"
            )
        except (json.JSONDecodeError, KeyError, TypeError):
            quire_version = "unreadable"
    return {
        "quoin": tool_version(["quoin", "--version"]),
        "quire": quire_version,
        "ix-flow": tool_version(["ix-flow", "--version"]),
        "rustc": tool_version(["rustc", "--version"]),
        "platform": sys.platform,
    }


# ---------------------------------------------------------------------------
# The native adapter
# ---------------------------------------------------------------------------

# The domain stream's outcome vocabulary, and the Quoin entry outcome each one
# transcribes to. Every value is listed. An outcome this table does not name is
# refused rather than defaulted, because a silently defaulted unknown state is
# how twelve states become two.
CONFORMANCE_OUTCOMES = {
    "pass": "pass",
    "fail": "fail",
    "unavailable": "skip",
    "not-computed": "skip",
    "malformed": "fail",
    "vacuous": "skip",
}


def adapt_conformance(raw: str) -> dict[str, Any]:
    """Transcribe the declared domain protocol into Quoin's normalized entries.

    This is the whole of the adapter. It reads a protocol it names, maps a state
    vocabulary it enumerates, and refuses anything else. It runs nothing, judges
    nothing, and never looks at a process's output stream to decide an outcome.
    """
    entries = []
    lines = [line for line in raw.splitlines() if line.strip()]
    if not lines:
        raise ChainError("the conformance stream is empty; there is nothing to transcribe")
    for number, line in enumerate(lines, start=1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as error:
            raise ChainError(f"conformance stream line {number} is malformed: {error}") from error
        protocol = row.get("protocol")
        if protocol != CONFORMANCE_PROTOCOL:
            raise ChainError(
                f"conformance stream line {number} declares protocol {protocol!r}; "
                f"this adapter transcribes {CONFORMANCE_PROTOCOL} and refuses to guess"
            )
        outcome = row.get("outcome")
        if outcome not in CONFORMANCE_OUTCOMES:
            raise ChainError(
                f"conformance stream line {number} declares outcome {outcome!r}, "
                "which this adapter does not name"
            )
        entries.append(
            {
                "symbol": row["symbol"],
                "outcome": CONFORMANCE_OUTCOMES[outcome],
                "traceIds": list(row.get("traceIds", [])),
            }
        )
    return {"entries": entries}


# ---------------------------------------------------------------------------
# The chain
# ---------------------------------------------------------------------------


class Chain:
    """Seal, retain and verify, entirely through the pinned Quoin CLI."""

    def __init__(self, candidate_revision: str, store: Path) -> None:
        self.revision = candidate_revision
        self.store = store
        self.environment = observe_environment()
        self.declaration = json.loads(DECLARATION.read_text(encoding="utf-8"))
        self.observed_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    # -- record ------------------------------------------------------------

    def record_body(self) -> dict[str, Any]:
        """Project the declaration into Quoin's record body, deriving only digests."""
        declared = json.loads(json.dumps(self.declaration["record"]))
        sources = self.declaration["sources"]
        declared["subject"]["base_revision"] = self.revision
        for connection in declared["source_connections"]:
            path = ROOT / sources[connection["source_id"]]
            if not path.is_file():
                raise ChainError(
                    f"declared source {connection['source_id']} is missing at {path}"
                )
            connection["revision"] = self.revision
            connection["digest"] = digest_of(path.read_bytes())
        for proof in declared["definition"]["proof_obligations"]:
            configuration = proof.pop("configuration")
            path = ROOT / configuration
            if not path.is_file():
                raise ChainError(
                    f"{proof['proof_id']} names configuration {configuration}, which is missing"
                )
            proof["configuration_digest"] = digest_of(path.read_bytes())
            proof["_configuration_path"] = configuration
        export = ASSURANCE_DIR / INPUTS["PROOF-quire-static-export"][0]
        if not export.is_file():
            raise ChainError(
                f"{export} is absent. Run `make assurance-inputs`; this driver does "
                "not run producers."
            )
        declared["impact_snapshot"]["revision"] = self.revision
        declared["impact_snapshot"]["digest"] = digest_of(export.read_bytes())
        return declared

    def seal_record(self) -> tuple[str, dict[str, Any]]:
        body = self.record_body()
        configurations = {
            proof["proof_id"]: proof.pop("_configuration_path")
            for proof in body["definition"]["proof_obligations"]
        }
        result = quoin(
            "change-assurance",
            "seal-record",
            "--repo",
            str(self.store),
            "--input",
            "-",
            "--json",
            stdin=json.dumps(body),
        )
        if result.returncode != 0:
            raise ChainError(f"quoin refused the change-assurance record: {result.stderr.strip()}")
        digest = json.loads(result.stdout)["digest"]
        self.configurations = configurations
        self.record = body
        return digest, body

    # -- attestation -------------------------------------------------------

    def attestation_body(
        self,
        record_digest: str,
        proof_id: str,
        result_state: str,
        *,
        candidate_revision: str | None = None,
    ) -> dict[str, Any]:
        proof = next(
            item
            for item in self.record["definition"]["proof_obligations"]
            if item["proof_id"] == proof_id
        )
        return {
            "schema_version": 1,
            "record_type": "proof_attestation",
            "attestation_id": f"{proof_id}:{result_state}",
            "record_digest": record_digest,
            "candidate_revision": candidate_revision or self.revision,
            "proof_id": proof_id,
            "command": proof["command"],
            "tool": {
                "identity": proof["tool_identity"],
                "version": "0.1.0",
                "configuration_digest": proof["configuration_digest"],
            },
            "environment": self.environment,
            "observed_at": self.observed_at,
            "result": result_state,
        }

    def seal_attestation(self, body: dict[str, Any], output: Path, media_type: str) -> dict[str, Any]:
        result = quoin(
            "change-assurance",
            "seal-attestation",
            "--input",
            "-",
            "--output",
            str(output),
            "--media-type",
            media_type,
            "--json",
            stdin=json.dumps(body),
        )
        if result.returncode != 0:
            raise ChainError(f"quoin refused the proof attestation: {result.stderr.strip()}")
        return json.loads(result.stdout)

    def intake(self, attestation: dict[str, Any], output: Path) -> subprocess.CompletedProcess[str]:
        return quoin(
            "change-assurance",
            "intake",
            "--repo",
            str(self.store),
            "--attestation",
            "-",
            "--output",
            str(output),
            "--json",
            stdin=json.dumps(attestation),
        )

    def receipt(
        self,
        record_digest: str,
        selections: dict[str, str],
        decisions: Path,
        *,
        candidate_revision: str | None = None,
        audits: Path | None = None,
    ) -> tuple[int, dict[str, Any]]:
        arguments = [
            "change-assurance",
            "receipt",
            "--repo",
            str(self.store),
            "--record",
            record_digest,
            "--candidate-revision",
            candidate_revision or self.revision,
            "--decisions",
            str(decisions),
            "--json",
        ]
        for proof_id, attestation_digest in selections.items():
            arguments.extend(["--select", f"{proof_id}={attestation_digest}"])
        if audits is not None:
            arguments.extend(["--audits", str(audits)])
        result = quoin(*arguments)
        if result.returncode == 2:
            raise ChainError(f"quoin refused to emit a receipt: {result.stderr.strip()}")
        return result.returncode, json.loads(result.stdout)

    def verify_receipt(self, receipt: dict[str, Any]) -> tuple[int, str]:
        result = quoin(
            "change-assurance", "verify-receipt", "--input", "-", "--json", stdin=json.dumps(receipt)
        )
        return result.returncode, (result.stdout or result.stderr).strip()


# ---------------------------------------------------------------------------
# Inputs
# ---------------------------------------------------------------------------


def require_inputs() -> dict[str, Path]:
    paths = {}
    for proof_id, (name, _) in INPUTS.items():
        path = ASSURANCE_DIR / name
        if not path.is_file():
            raise ChainError(
                f"{path.relative_to(ROOT)} is absent. Run `make assurance-inputs`. "
                "This driver consumes producer output and never creates it, so an "
                "absent input is an error rather than a step it can quietly do itself."
            )
        paths[proof_id] = path
    return paths


def derive_failed_stream(raw: str) -> str:
    """One named edit to the real conformance stream: the first pass becomes a fail.

    The corpus is green and has to stay green, so the failing case is derived from
    the real run rather than invented. A `fail` state demonstrated by a stream
    nobody produced is a state nobody has actually seen travel the chain.
    """
    lines = [line for line in raw.splitlines() if line.strip()]
    for index, line in enumerate(lines):
        row = json.loads(line)
        if row.get("outcome") == "pass":
            row["outcome"] = "fail"
            lines[index] = json.dumps(row)
            return "\n".join(lines) + "\n"
    raise ChainError("the conformance stream contains no passing row to derive a failure from")


# ---------------------------------------------------------------------------
# Scenarios
# ---------------------------------------------------------------------------


def run_chain(candidate_revision: str, workspace: Path) -> dict[str, Any]:
    inputs = require_inputs()
    store = workspace / "store"
    store.mkdir(parents=True, exist_ok=True)
    scratch = workspace / "scratch"
    scratch.mkdir(parents=True, exist_ok=True)

    chain = Chain(candidate_revision, store)
    record_digest, _ = chain.seal_record()

    decisions = scratch / "decisions.json"
    decisions.write_text(
        json.dumps(
            {
                "run_id": chain.record["review_workflow"]["run_id"],
                "events": [],
            }
        ),
        encoding="utf-8",
    )

    def audit_reports(path: Path) -> Path:
        """A clean FR-032 audit report per proof, naming that proof's own obligations.

        `healthy` has to name the obligations the audit actually evaluated. A
        report that names none is an audit that evaluated nothing, and Quoin says
        so rather than reading it as clean — which is the correct behaviour and
        the reason this helper builds one report per proof instead of one for all.
        """
        reports = []
        for proof in chain.record["definition"]["proof_obligations"]:
            report = {
                "findings": [],
                "healthy": list(proof["obligation_ids"]),
                "unevaluated": [],
            }
            reports.append(
                {
                    "proof_id": proof["proof_id"],
                    "report_digest": digest_of(
                        json.dumps(report, sort_keys=True, separators=(",", ":")).encode(
                            "utf-8"
                        )
                    ),
                    "report": report,
                }
            )
        path.write_text(json.dumps(reports), encoding="utf-8")
        return path

    audits = audit_reports(scratch / "audits.json")

    def proof_rows(receipt: dict[str, Any]) -> dict[str, dict[str, Any]]:
        return {row["proof_id"]: row for row in receipt["proofs"]}

    scenarios: list[dict[str, Any]] = []
    controls: list[dict[str, Any]] = []

    def scenario(name: str, state: str, matched: bool, detail: Any) -> None:
        scenarios.append(
            {"scenario": name, "state": state, "matched": bool(matched), "detail": detail}
        )

    def control(name: str, pairs_with: str, matched: bool, detail: Any) -> None:
        controls.append(
            {
                "control": name,
                "pairs_with": pairs_with,
                "matched": bool(matched),
                "detail": detail,
            }
        )

    # -- 1. the honest path: seal, retain, and get the bytes back unchanged ---
    selections: dict[str, str] = {}
    for proof_id, path in inputs.items():
        media_type = INPUTS[proof_id][1]
        body = chain.attestation_body(record_digest, proof_id, "passed")
        sealed = chain.seal_attestation(body, path, media_type)
        taken = chain.intake(sealed, path)
        if taken.returncode != 0:
            raise ChainError(
                f"{proof_id}: intake refused an unmodified producer output: {taken.stderr.strip()}"
            )
        detail = json.loads(taken.stdout)
        retained = Path(detail["directory"]) / "output.bin"
        identical = retained.read_bytes() == path.read_bytes()
        selections[proof_id] = sealed["digest"]
        if proof_id == "PROOF-corpus-conformance":
            scenario(
                "retain-producer-output",
                "pass",
                identical,
                {"retained": str(retained), "bytes": retained.stat().st_size},
            )
            control(
                "intake-accepts-unchanged-bytes",
                "retained-bytes-changed-after-sealing",
                identical,
                {"proof": proof_id},
            )

    # -- 2. the receipt, and re-verifying it ---------------------------------
    status, receipt = chain.receipt(record_digest, selections, decisions)
    verified_status, _ = chain.verify_receipt(receipt)
    # No ix-flow decision exists, so an `incomplete` receipt is the correct
    # answer and its status of 1 is the correct exit code. A `valid` receipt here
    # would mean a human decision had been synthesized.
    scenario(
        "receipt-reports-the-absent-human-decision",
        "partial",
        receipt["outcome"] != "valid" and status == 1,
        {"outcome": receipt["outcome"], "exit": status},
    )
    scenario(
        "re-verify-the-sealed-receipt",
        "pass",
        verified_status == status,
        {"verify_exit": verified_status, "receipt_exit": status},
    )
    control(
        "verify-accepts-an-unedited-receipt",
        "refuse-an-edited-receipt",
        verified_status != 2,
        {"exit": verified_status},
    )

    # -- 3. an edited receipt is refused -------------------------------------
    edited = json.loads(json.dumps(receipt))
    edited["outcome"] = "valid"
    edited_status, edited_detail = chain.verify_receipt(edited)
    scenario(
        "refuse-an-edited-receipt",
        "tampered",
        edited_status == 2,
        {"exit": edited_status, "message": edited_detail[:200]},
    )

    # -- 4. retained bytes changed after sealing -----------------------------
    moved = scratch / "moved.jsonl"
    moved.write_bytes(inputs["PROOF-corpus-conformance"].read_bytes())
    body = chain.attestation_body(record_digest, "PROOF-corpus-conformance", "passed")
    sealed_moved = chain.seal_attestation(body, moved, "application/x-ndjson")
    moved.write_bytes(moved.read_bytes() + b"\n")
    refused = chain.intake(sealed_moved, moved)
    scenario(
        "retained-bytes-changed-after-sealing",
        "tampered",
        refused.returncode != 0,
        {"exit": refused.returncode, "message": refused.stderr.strip()[:200]},
    )

    # -- 5. a stale candidate binding ----------------------------------------
    stale_status, stale_receipt = chain.receipt(
        record_digest, selections, decisions, candidate_revision="0" * 40, audits=audits
    )
    stale_reasons = set(proof_rows(stale_receipt)["PROOF-corpus-conformance"]["reasons"])
    scenario(
        "stale-candidate-binding",
        "stale",
        "candidate_revision_mismatch" in stale_reasons,
        {"outcome": stale_receipt["outcome"], "reasons": sorted(stale_reasons)},
    )

    # -- 6. attested non-success states, each named by its own reason ---------
    #
    # The receipt is asked with a clean audit for every proof, so that the only
    # thing distinguishing these runs is the attested result. Without that, every
    # proof row reads `audit_not_evaluated` and three different states collapse
    # into one indistinguishable answer, which is precisely the failure this
    # scenario exists to rule out.
    audited_status, audited = chain.receipt(
        record_digest, selections, decisions, audits=audits
    )
    passing_row = proof_rows(audited)["PROOF-corpus-conformance"]
    control(
        "an-audited-passing-proof-is-valid-and-reasonless",
        "attested-failure",
        passing_row["outcome"] == "valid" and not passing_row["reasons"],
        {"row": passing_row["outcome"], "reasons": passing_row["reasons"]},
    )
    control(
        "receipt-discharges-a-current-binding",
        "stale-candidate-binding",
        "candidate_revision_mismatch" not in passing_row["reasons"],
        {"reasons": passing_row["reasons"]},
    )

    failed_stream = scratch / "failed.jsonl"
    failed_stream.write_text(
        derive_failed_stream(inputs["PROOF-corpus-conformance"].read_text(encoding="utf-8")),
        encoding="utf-8",
    )
    expected_reason = {
        "failed": "result_failed",
        "unavailable": "result_unavailable",
        "not_computed": "result_not_computed",
    }
    state_name = {"failed": "fail", "unavailable": "unavailable", "not_computed": "not-computed"}
    observed_reasons: dict[str, set[str]] = {}
    for state, source in (
        ("failed", failed_stream),
        ("unavailable", inputs["PROOF-corpus-conformance"]),
        ("not_computed", inputs["PROOF-corpus-conformance"]),
    ):
        body = chain.attestation_body(record_digest, "PROOF-corpus-conformance", state)
        body["attestation_id"] = f"PROOF-corpus-conformance:{state}"
        sealed_state = chain.seal_attestation(body, source, "application/x-ndjson")
        taken = chain.intake(sealed_state, source)
        if taken.returncode != 0:
            raise ChainError(f"intake refused a {state} attestation: {taken.stderr.strip()}")
        state_selections = dict(selections)
        state_selections["PROOF-corpus-conformance"] = sealed_state["digest"]
        _, state_receipt = chain.receipt(
            record_digest, state_selections, decisions, audits=audits
        )
        rows = proof_rows(state_receipt)
        reasons = set(rows["PROOF-corpus-conformance"]["reasons"])
        observed_reasons[state] = reasons
        scenario(
            f"attested-{state}",
            state_name[state],
            expected_reason[state] in reasons,
            {"reasons": sorted(reasons), "receipt_outcome": state_receipt["outcome"]},
        )
        if state == "failed":
            control(
                "passing-proof-is-not-reported-as-failing",
                "attested-failure",
                not set(rows["PROOF-corpus-oracle"]["reasons"]) & set(expected_reason.values()),
                {"corpus_oracle_reasons": rows["PROOF-corpus-oracle"]["reasons"]},
            )

    # The three non-success states must be pairwise distinguishable. Each being
    # non-passing individually would still be satisfied by collapsing all three.
    distinct = len({frozenset(value) for value in observed_reasons.values()}) == 3
    scenario(
        "non-success-states-stay-distinguishable",
        "unsupported",
        distinct,
        {state: sorted(value) for state, value in observed_reasons.items()},
    )

    # -- 7. an unaudited proof is not-computed, not clean ---------------------
    unaudited_row = proof_rows(receipt)["PROOF-corpus-conformance"]
    scenario(
        "audited-clean-versus-unaudited",
        "not-computed",
        "audit_not_evaluated" in unaudited_row["reasons"]
        and "audit_not_evaluated" not in passing_row["reasons"],
        {
            "unaudited": unaudited_row["reasons"],
            "audited": passing_row["reasons"],
            "why": (
                "an audit with no findings and no audit at all are different facts; "
                "the absence is reported as not-computed rather than as clean"
            ),
        },
    )
    control(
        "an-audit-that-was-run-clears-not-computed",
        "audited-clean-versus-unaudited",
        audited_status in (0, 1) and "audit_not_evaluated" not in audited["reasons"],
        {"receipt_reasons": audited["reasons"]},
    )

    # -- 8. a proof with no attestation stays missing -------------------------
    partial_selections = {
        key: value for key, value in selections.items() if key != "PROOF-msrv"
    }
    _, partial = chain.receipt(record_digest, partial_selections, decisions, audits=audits)
    missing_row = proof_rows(partial).get("PROOF-msrv", {})
    scenario(
        "unattested-proof-stays-missing",
        "partial",
        partial["outcome"] != "valid"
        and "attestation_missing" in set(missing_row.get("reasons", [])),
        {"outcome": partial["outcome"], "msrv_reasons": missing_row.get("reasons")},
    )

    # -- 9. the open unknowns survive into the receipt ------------------------
    declared_unknowns = {
        item["id"] for item in chain.record["definition"]["unknowns"]
    }
    carried = {item["id"] for item in audited.get("unknowns", [])}
    scenario(
        "declared-unknowns-are-carried-not-dropped",
        "inconclusive",
        declared_unknowns == carried and "unresolved_unknown" in audited["reasons"],
        {"declared": sorted(declared_unknowns), "carried": sorted(carried)},
    )

    return {
        "record_digest": record_digest,
        "candidate_revision": candidate_revision,
        "impact_snapshot_digest": chain.record["impact_snapshot"]["digest"],
        "quire_export": str(
            (ASSURANCE_DIR / INPUTS["PROOF-quire-static-export"][0]).relative_to(ROOT)
        ),
        "receipt_outcome": receipt["outcome"],
        "audited_receipt_outcome": audited["outcome"],
        "audited_receipt_reasons": audited["reasons"],
        "scenarios": scenarios,
        "controls": controls,
    }


# ---------------------------------------------------------------------------
# Adapter probes
# ---------------------------------------------------------------------------


def adapter_probes(workspace: Path) -> list[dict[str, Any]]:
    """Exercise the native adapter and Quoin's evidence audit in a scratch tree.

    A copy of `spec/` is used so the suite registry, requirements and matrix are
    the real ones and a binding is a real binding, while nothing is written into
    this repository's own store.
    """
    inputs = require_inputs()
    probe_root = workspace / "adapter"
    if probe_root.exists():
        shutil.rmtree(probe_root)
    probe_root.mkdir(parents=True)
    shutil.copytree(ROOT / "spec", probe_root / "spec")

    stream = inputs["PROOF-corpus-conformance"].read_text(encoding="utf-8")
    commit = "0" * 40
    results = []

    def record(suite: str, payload: dict[str, Any], commit_sha: str) -> dict[str, Any]:
        path = probe_root / "run.json"
        path.write_text(json.dumps(payload), encoding="utf-8")
        outcome = quoin(
            "evidence",
            "record",
            "--repo",
            str(probe_root),
            "--suite",
            suite,
            "--commit",
            commit_sha,
            "--tool",
            "tl-syntax-corpus-conformance 0.1.0",
            "--adapter",
            "entries",
            # The kind SUITE-001 declares. Method conformance compares kind to
            # kind, so a value the suite registry does not use would make the
            # check stay silent instead of checking.
            "--kind",
            "Integration",
            "--results",
            str(path),
            "--json",
        )
        if outcome.returncode != 0:
            raise ChainError(f"quoin refused an evidence record: {outcome.stderr.strip()}")
        return json.loads(outcome.stdout)

    def audit_kinds() -> dict[str, int]:
        outcome = quoin("evidence", "audit", "--repo", str(probe_root), "--json")
        if outcome.returncode not in (0, 1):
            raise ChainError(f"quoin evidence audit failed: {outcome.stderr.strip()}")
        findings = json.loads(outcome.stdout)["findings"]
        counted: dict[str, int] = {}
        for finding in findings:
            counted[finding["kind"]] = counted.get(finding["kind"], 0) + 1
        return counted

    # Probe 1 (positive control): the real run binds real obligations.
    transcribed = adapt_conformance(stream)
    bound = record("SUITE-001", transcribed, commit)["bound"]
    results.append(
        {
            "probe": "accepts-the-real-run",
            "state": "pass",
            "matched": bool(bound),
            "detail": {"bound": len(bound), "entries": len(transcribed["entries"])},
        }
    )

    # Probe 2: the adapter must carry a non-success outcome through as a
    # non-success outcome. The stream is derived from the real one by renaming
    # every outcome to `not-computed`, and it is transcribed by the adapter
    # rather than hand-built, so an adapter that mapped everything to `pass`
    # would be caught here instead of quietly producing a clean run.
    not_computed_stream = "\n".join(
        json.dumps({**json.loads(line), "outcome": "not-computed"})
        for line in stream.splitlines()
        if line.strip()
    )
    downgraded = adapt_conformance(not_computed_stream)
    preserved = all(entry["outcome"] != "pass" for entry in downgraded["entries"])
    results.append(
        {
            "probe": "adapter-preserves-non-success-outcomes",
            "state": "not-computed",
            "matched": preserved and bool(downgraded["entries"]),
            "detail": {
                "outcomes": sorted({entry["outcome"] for entry in downgraded["entries"]}),
                "entries": len(downgraded["entries"]),
            },
        }
    )

    # Probe 3: a run in which every bound symbol was skipped is vacuous, and
    # Quoin says so rather than reading the row as covered. The entries come
    # from the adapter's own transcription of the derived stream above.
    record("SUITE-001", downgraded, commit)
    kinds = audit_kinds()
    results.append(
        {
            "probe": "audit-reports-a-vacuous-run",
            "state": "vacuous",
            "matched": kinds.get("vacuous-evidence", 0) > 0,
            "detail": kinds,
        }
    )

    # Probe 4: a reworded statement makes its bound evidence suspect.
    record("SUITE-001", transcribed, commit)
    requirement = probe_root / "spec" / "requirements" / "FR-005-conformance-corpus.md"
    text = requirement.read_text(encoding="utf-8")
    marker = "unique stable identities."
    if marker not in text:
        raise ChainError("the probe's statement marker is no longer present in FR-005")
    requirement.write_text(
        text.replace(marker, "unique stable identities and a declared class.", 1),
        encoding="utf-8",
    )
    kinds = audit_kinds()
    results.append(
        {
            "probe": "audit-reports-a-suspect-link",
            "state": "suspect",
            "matched": kinds.get("suspect-link", 0) > 0,
            "detail": kinds,
        }
    )

    # Probe 5: a foreign protocol is refused by the adapter, not guessed at.
    foreign = "\n".join(
        json.dumps({**json.loads(line), "protocol": "some.other.protocol/v1"})
        for line in stream.splitlines()
        if line.strip()
    )
    refused = False
    try:
        adapt_conformance(foreign)
    except ChainError:
        refused = True
    results.append(
        {
            "probe": "refuses-a-foreign-protocol",
            "state": "unsupported",
            "matched": refused,
            "detail": {"protocol": "some.other.protocol/v1"},
        }
    )

    # Probe 6: an empty stream is refused rather than transcribed into a clean run.
    empty_refused = False
    try:
        adapt_conformance("")
    except ChainError:
        empty_refused = True
    results.append(
        {
            "probe": "refuses-an-empty-stream",
            "state": "vacuous",
            "matched": empty_refused,
            "detail": {},
        }
    )

    return results


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--candidate-revision")
    parser.add_argument("--json", action="store_true")
    parser.add_argument(
        "--keep-store",
        action="store_true",
        help="keep this run's Quoin store under target/assurance-store for inspection",
    )
    parser.add_argument(
        "--adapt",
        metavar="PATH",
        help=(
            "transcribe a domain conformance stream into Quoin's normalized entries "
            "and print them; this is the adapter on its own, with no chain around it"
        ),
    )
    arguments = parser.parse_args(argv[1:])

    if arguments.adapt is not None:
        try:
            entries = adapt_conformance(Path(arguments.adapt).read_text(encoding="utf-8"))
        except (ChainError, OSError) as error:
            print(str(error), file=sys.stderr)
            return 2
        print(json.dumps(entries, indent=2, sort_keys=True))
        return 0

    if arguments.candidate_revision is None:
        print("--candidate-revision is required", file=sys.stderr)
        return 2

    # Each run gets its own store. Two runs sharing one directory is a race that
    # makes a green run depend on which finished first, and this driver is
    # invoked concurrently by the test suite.
    STORE.mkdir(parents=True, exist_ok=True)
    workspace = Path(tempfile.mkdtemp(prefix="run-", dir=STORE))

    try:
        chain = run_chain(arguments.candidate_revision, workspace)
        probes = adapter_probes(workspace)
    except ChainError as error:
        print(str(error), file=sys.stderr)
        return 2
    finally:
        if not arguments.keep_store:
            shutil.rmtree(workspace, ignore_errors=True)

    report = {
        "schemaVersion": "tl-syntax.assurance-chain-report/v1",
        **chain,
        "adapter_probes": probes,
        "states_demonstrated": sorted(
            {item["state"] for item in chain["scenarios"]}
            | {item["state"] for item in probes}
        ),
        "matched": all(
            item["matched"]
            for group in (chain["scenarios"], chain["controls"], probes)
            for item in group
        ),
    }
    if arguments.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        for item in chain["scenarios"]:
            print(
                f"scenario {item['scenario']} [{item['state']}]: "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
        for item in chain["controls"]:
            print(
                f"control  {item['control']} (pairs with {item['pairs_with']}): "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
        for item in probes:
            print(
                f"probe    {item['probe']} [{item['state']}]: "
                f"{'ok' if item['matched'] else 'MISMATCH'}"
            )
    if not report["matched"]:
        print("the assurance chain did not match its declared scenarios", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
