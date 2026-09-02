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

It is not a retention store. It writes nothing into this repository, commits
nothing, and the Quoin store it uses lives under `target/`, which is ignored.

Exit status: 0 when every scenario, control and probe matched, 1 when one did
not, 2 on a usage or environment error — which is a different fact from a
mismatch and gets its own code.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
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
    "PROOF-msrv": ("msrv.jsonl", "application/x-ndjson"),
}

# The outcome vocabulary a producer's rows may use, and the attestation result
# each maps to. Every value is listed; an unlisted one is refused.
ROW_RESULTS = {
    "pass": "passed",
    "fail": "failed",
    "malformed": "failed",
    "unavailable": "unavailable",
    "not-computed": "not_computed",
    "vacuous": "not_computed",
}

# Precedence when a stream carries more than one outcome. A single failure
# outranks any number of passes, and an unavailable outranks a not-computed,
# because the strongest thing observed is what the run has to be reported as.
RESULT_PRECEDENCE = ("failed", "unavailable", "not_computed", "passed")


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


def tool_version(argv: list[str]) -> str | None:
    """Observe a tool's version, or report that it could not be observed.

    `None` is the answer when the probe failed, and it is recorded as `null`
    rather than replaced with a plausible-looking default. A fabricated version
    in a sealed attestation's environment is worse than an absent one, because a
    reader cannot tell it apart from a real observation.
    """
    try:
        result = subprocess.run(argv, capture_output=True, text=True, check=False)
    except OSError:
        return None
    if result.returncode != 0:
        return None
    return result.stdout.strip() or None


SEMVER = re.compile(r"\b(\d+\.\d+\.\d+)\b")


def semantic_version(text: str | None) -> str | None:
    """Extract the version the attestation schema's `immutable_version` accepts."""
    if text is None:
        return None
    found = SEMVER.search(text)
    return found.group(1) if found else None


def observe_environment() -> dict[str, Any]:
    quire_version: str | None = None
    try:
        raw = subprocess.run(
            ["quire", "provenance"], capture_output=True, text=True, check=False
        )
    except OSError:
        # An absent tool is an unobserved tool, recorded as null. It is not a
        # crash, and it is certainly not a version.
        raw = None
    if raw is not None and raw.returncode == 0:
        try:
            provenance = json.loads(raw.stdout)
            quire_version = (
                f"{provenance['cli']['version']} engine {provenance['engine']['version']}"
            )
        except (json.JSONDecodeError, KeyError, TypeError):
            quire_version = None
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


def transcribe(
    raw: str,
    *,
    refuse_undecodable: bool = True,
    refuse_foreign_protocol: bool = True,
    refuse_unnamed_outcome: bool = True,
    refuse_empty: bool = True,
) -> dict[str, Any]:
    """The adapter, with each of its four refusals named and switchable.

    Every keyword defaults to the strict behaviour and `adapt_conformance` is the
    only caller that uses the defaults. The switches exist so `--mutation-probes`
    can weaken exactly one refusal at a time and show that the check which is
    supposed to notice actually does. A weakening expressed as a flag on the real
    code path is a weakening of the real code path; a second hand-written copy of
    the adapter would only prove that the copy was weaker.
    """
    entries = []
    lines = [line for line in raw.splitlines() if line.strip()]
    if not lines and refuse_empty:
        raise ChainError("the conformance stream is empty; there is nothing to transcribe")
    for number, line in enumerate(lines, start=1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as error:
            if refuse_undecodable:
                raise ChainError(
                    f"conformance stream line {number} is malformed: {error}"
                ) from error
            continue
        protocol = row.get("protocol")
        if protocol != CONFORMANCE_PROTOCOL and refuse_foreign_protocol:
            raise ChainError(
                f"conformance stream line {number} declares protocol {protocol!r}; "
                f"this adapter transcribes {CONFORMANCE_PROTOCOL} and refuses to guess"
            )
        outcome = row.get("outcome")
        if outcome not in CONFORMANCE_OUTCOMES:
            if refuse_unnamed_outcome:
                raise ChainError(
                    f"conformance stream line {number} declares outcome {outcome!r}, "
                    "which this adapter does not name"
                )
            outcome = "pass"
        entries.append(
            {
                "symbol": row["symbol"],
                "outcome": CONFORMANCE_OUTCOMES[outcome],
                "traceIds": list(row.get("traceIds", [])),
            }
        )
    return {"entries": entries}


def adapt_conformance(raw: str) -> dict[str, Any]:
    """Transcribe the declared domain protocol into Quoin's normalized entries.

    This is the whole of the adapter. It reads a protocol it names, maps a state
    vocabulary it enumerates, and refuses anything else. It runs nothing, judges
    nothing, and never looks at a process's output stream to decide an outcome.
    """
    return transcribe(raw)


# ---------------------------------------------------------------------------
# The adapter's four checks, each as a predicate over a transcriber
# ---------------------------------------------------------------------------
#
# `adapter_probes` runs these against the real adapter and requires True.
# `mutation_probes` runs each against the adapter with exactly the refusal it
# tests switched off, and requires False. A check that cannot be made to fail is
# a check that was never checking, which is what the deleted compatibility gate's
# own `--mutation-probes` mode existed to rule out; this is its replacement for
# the adapter that survived.


def _rows(stream: str) -> list[str]:
    return [line for line in stream.splitlines() if line.strip()]


def derive_foreign_stream(stream: str) -> str:
    """Every row relabelled with a protocol this adapter does not transcribe."""
    return "\n".join(
        json.dumps({**json.loads(line), "protocol": "some.other.protocol/v1"})
        for line in _rows(stream)
    )


def derive_not_computed_stream(stream: str) -> str:
    """Every row's outcome renamed to `not-computed`, nothing else changed."""
    return "\n".join(
        json.dumps({**json.loads(line), "outcome": "not-computed"}) for line in _rows(stream)
    )


def derive_malformed_stream(stream: str) -> str:
    """The real stream with its second row truncated mid-object."""
    lines = _rows(stream)
    if len(lines) < 2:
        raise ChainError("the conformance stream is too short to derive a malformed row from")
    lines[1] = lines[1][: len(lines[1]) // 2]
    return "\n".join(lines)


def check_refuses_a_foreign_protocol(adapt: Any, stream: str) -> tuple[bool, dict[str, Any]]:
    try:
        adapt(derive_foreign_stream(stream))
    except ChainError as error:
        return True, {"reason": str(error)[:160]}
    return False, {"protocol": "some.other.protocol/v1", "reason": "transcribed anyway"}


def check_refuses_an_empty_stream(adapt: Any, stream: str) -> tuple[bool, dict[str, Any]]:
    try:
        transcribed = adapt("")
    except ChainError as error:
        return True, {"reason": str(error)[:160]}
    return False, {"entries": len(transcribed["entries"]), "reason": "transcribed anyway"}


def check_refuses_a_malformed_row(adapt: Any, stream: str) -> tuple[bool, dict[str, Any]]:
    """A row that cannot be decoded is refused as malformed, not dropped.

    The refusal has to be about the undecodable bytes specifically, so the reason
    text is required to name it. An adapter that skipped the row would transcribe
    a shorter clean run, and `malformed` would stop being demonstrated anywhere
    on the intake path.
    """
    corrupted = derive_malformed_stream(stream)
    try:
        transcribed = adapt(corrupted)
    except ChainError as error:
        return "is malformed" in str(error), {"reason": str(error)[:160]}
    return False, {
        "entries": len(transcribed["entries"]),
        "rows": len(_rows(corrupted)),
        "reason": "the undecodable row was dropped rather than refused",
    }


def derive_unnamed_outcome_stream(stream: str) -> str:
    """Every row's outcome replaced with one the adapter's table does not name."""
    return "\n".join(
        json.dumps({**json.loads(line), "outcome": "who-knows"}) for line in _rows(stream)
    )


def check_refuses_an_unnamed_outcome(adapt: Any, stream: str) -> tuple[bool, dict[str, Any]]:
    """An outcome the table does not name is refused, never defaulted.

    A silently defaulted unknown state is how twelve states become two, and it
    defaults towards `pass` rather than away from it, so the weakened adapter is
    required to be seen doing exactly that.
    """
    try:
        transcribed = adapt(derive_unnamed_outcome_stream(stream))
    except ChainError as error:
        return "does not name" in str(error), {"reason": str(error)[:160]}
    return False, {
        "outcomes": sorted({entry["outcome"] for entry in transcribed["entries"]}),
        "reason": "an unnamed outcome was given a default",
    }


# Each entry: the probe it backs, the check, and the single refusal that check
# depends on. `weakening` is the keyword `transcribe` accepts, set to False.
ADAPTER_CHECKS = (
    ("refuses-a-foreign-protocol", check_refuses_a_foreign_protocol, "refuse_foreign_protocol"),
    ("refuses-an-empty-stream", check_refuses_an_empty_stream, "refuse_empty"),
    ("refuses-a-malformed-row", check_refuses_a_malformed_row, "refuse_undecodable"),
    ("refuses-an-unnamed-outcome", check_refuses_an_unnamed_outcome, "refuse_unnamed_outcome"),
)


def mutation_probes(stream: str) -> list[dict[str, Any]]:
    """Weaken one adapter refusal at a time and require its check to go red."""
    probes = []
    for name, check, weakening in ADAPTER_CHECKS:
        strict, strict_detail = check(adapt_conformance, stream)
        weakened, weakened_detail = check(
            lambda raw, key=weakening: transcribe(raw, **{key: False}), stream
        )
        probes.append(
            {
                "probe": name,
                "weakening": f"transcribe({weakening}=False)",
                "strict": strict,
                "weakened": weakened,
                "detected": bool(strict) and not weakened,
                "detail": {"strict": strict_detail, "weakened": weakened_detail},
            }
        )
    return probes


# ---------------------------------------------------------------------------
# The chain
# ---------------------------------------------------------------------------


class Chain:
    """Seal, retain and verify, entirely through the pinned Quoin CLI."""

    # The crate's own version, for the tools this repository ships. Read from
    # Cargo.toml rather than written twice.
    @staticmethod
    def crate_version() -> str:
        for line in (ROOT / "Cargo.toml").read_text(encoding="utf-8").splitlines():
            if line.startswith("version = "):
                return line.split('"')[1]
        raise ChainError("Cargo.toml declares no package version")

    def observe_tool_versions(self) -> dict[str, str]:
        """One observed version per declared tool identity.

        A tool whose version cannot be observed raises. The alternative is a
        sealed attestation naming a version nobody measured, and an attestation
        is only worth its weakest field.
        """
        crate = self.crate_version()
        probes = {
            "cargo": lambda: semantic_version(tool_version(["cargo", "--version"])),
            "quire": lambda: semantic_version(
                (self.environment.get("quire") or "").split(" ")[0] or None
            ),
        }
        versions: dict[str, str] = {}
        for proof in self.declaration["record"]["definition"]["proof_obligations"]:
            identity = proof["tool_identity"]
            if identity in versions:
                continue
            observed = probes[identity]() if identity in probes else crate
            if observed is None:
                raise ChainError(
                    f"the version of {identity} could not be observed; an attestation "
                    "will not be sealed naming a version nobody measured"
                )
            versions[identity] = observed
        return versions

    def __init__(self, candidate_revision: str, store: Path) -> None:
        self.revision = candidate_revision
        self.store = store
        self.environment = observe_environment()
        self.declaration = json.loads(DECLARATION.read_text(encoding="utf-8"))
        self.observed_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        self.tool_versions = self.observe_tool_versions()

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
                "version": self.tool_versions[proof["tool_identity"]],
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


def _worst(results: list[str]) -> str:
    for candidate in RESULT_PRECEDENCE:
        if candidate in results:
            return candidate
    raise ChainError("a producer result stream carried no outcome at all")


def _rows_result(rows: list[dict[str, Any]], where: str) -> str:
    if not rows:
        raise ChainError(
            f"{where} carries no rows. A producer that reported nothing is vacuous, "
            "and vacuous is not passed."
        )
    results = []
    for index, row in enumerate(rows):
        outcome = row.get("outcome")
        if outcome not in ROW_RESULTS:
            raise ChainError(f"{where} row {index} declares outcome {outcome!r}, which is not named")
        results.append(ROW_RESULTS[outcome])
    return _worst(results)


def derive_result(proof_id: str, path: Path) -> str:
    """Read the producer's own structured verdict out of the bytes it wrote.

    This is the difference between an attestation that states what happened and
    one that states what the caller hoped. Nothing here parses a transcript for
    words: every producer this repository owns emits a declared structured
    result, and `cargo` emits its own JSON message stream, so the verdict is read
    from a field in every case.

    A producer whose output cannot be read at all raises rather than defaulting.
    An attestation that says `passed` because its input was unreadable is the
    single worst failure this file could have.
    """
    raw = path.read_text(encoding="utf-8")
    if proof_id == "PROOF-corpus-conformance":
        rows = [json.loads(line) for line in raw.splitlines() if line.strip()]
        return _rows_result(rows, path.name)
    if proof_id in ("PROOF-corpus-oracle", "PROOF-feature-boundary"):
        return _rows_result(json.loads(raw)["entries"], path.name)
    if proof_id == "PROOF-quire-static-export":
        export = json.loads(raw)
        # Quire's export is a static fact set, not a run, so it has no outcome
        # field. What it can be held to is that it actually contains the facts
        # the impact snapshot claims: an empty document is `not_computed`, which
        # is a different answer from a clean export and must not read as one.
        if not isinstance(export, dict) or not export:
            return "not_computed"
        populated = any(
            isinstance(export.get(key), (list, dict)) and export.get(key)
            for key in export
        )
        return "passed" if populated else "not_computed"
    if proof_id == "PROOF-msrv":
        # `cargo --message-format=json` emits one JSON object per line and ends
        # with `build-finished`. The verdict is that object's `success` field.
        messages = [json.loads(line) for line in raw.splitlines() if line.strip()]
        finished = [item for item in messages if item.get("reason") == "build-finished"]
        if not finished:
            # The build did not report finishing. That is not a failure and it is
            # certainly not a pass; it is a run whose result was not computed.
            return "not_computed"
        if any(
            item.get("reason") == "compiler-message"
            and item.get("message", {}).get("level") == "error"
            for item in messages
        ):
            return "failed"
        return "passed" if finished[-1].get("success") is True else "failed"
    raise ChainError(f"no result rule is declared for {proof_id}")


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

    def scenario(name: str, state: str | None, matched: bool, detail: Any) -> None:
        """Record a scenario. `state` is None when it demonstrates no outcome.

        A scenario that is not about one of the twelve outcomes must say so,
        rather than borrow a label and inflate the census.
        """
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
    #
    # The attested result is read out of the bytes the producer wrote, never
    # assumed. A loop that sealed `passed` for everything would report a green
    # chain over a red repository, which is the failure this whole migration is
    # supposed to make impossible.
    selections: dict[str, str] = {}
    observed_results: dict[str, str] = {}
    for proof_id, path in inputs.items():
        media_type = INPUTS[proof_id][1]
        observed = derive_result(proof_id, path)
        observed_results[proof_id] = observed
        body = chain.attestation_body(record_digest, proof_id, observed)
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

    # Every producer at this candidate revision reported success, and the
    # attestations say so because the bytes did, not because the loop assumed it.
    # If a producer had failed, this scenario is where the chain says so.
    scenario(
        "attested-results-are-read-from-producer-output",
        None,
        all(result == "passed" for result in observed_results.values()),
        observed_results,
    )

    # -- 2. the receipt, and re-verifying it ---------------------------------
    status, receipt = chain.receipt(record_digest, selections, decisions)
    verified_status, _ = chain.verify_receipt(receipt)
    # No ix-flow decision exists, so an `incomplete` receipt is the correct
    # answer, and the reason it gives must be the missing decision specifically.
    # Asserting only "not valid" would be satisfied by a receipt that was invalid
    # because every proof failed.
    scenario(
        "receipt-reports-the-absent-human-decision",
        "partial",
        status == 1
        and receipt["outcome"] == "incomplete"
        and "decision_missing" in receipt["reasons"]
        and receipt["checks"]["review"]["outcome"] == "incomplete"
        and receipt["decision_event"] is None,
        {
            "outcome": receipt["outcome"],
            "exit": status,
            "reasons": receipt["reasons"],
            "review": receipt["checks"]["review"],
        },
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
        "attested-failed",
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
                "attested-failed",
                not set(rows["PROOF-corpus-oracle"]["reasons"]) & set(expected_reason.values()),
                {"corpus_oracle_reasons": rows["PROOF-corpus-oracle"]["reasons"]},
            )

    # The three non-success states must be pairwise distinguishable. Each being
    # non-passing individually would still be satisfied by collapsing all three.
    distinct = len({frozenset(value) for value in observed_reasons.values()}) == 3
    scenario(
        "non-success-states-stay-distinguishable",
        # Not a state demonstration: this is the assertion that the three states
        # demonstrated above are not the same answer wearing three names.
        None,
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

    # Every control must name a scenario that actually ran. A `pairs_with` naming
    # nothing is a control that pairs with nothing, and a test asserting the
    # dangling name would be satisfied by the typo rather than by the pairing.
    names = {item["scenario"] for item in scenarios}
    dangling = sorted(
        item["control"] for item in controls if item["pairs_with"] not in names
    )
    if dangling:
        raise ChainError(
            f"these controls name a scenario that does not exist: {dangling}. "
            f"Scenarios present: {sorted(names)}"
        )

    return {
        "record_digest": record_digest,
        "candidate_revision": candidate_revision,
        "impact_snapshot_digest": chain.record["impact_snapshot"]["digest"],
        "quire_export": str(
            (ASSURANCE_DIR / INPUTS["PROOF-quire-static-export"][0]).relative_to(ROOT)
        ),
        "attested_results": observed_results,
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
    downgraded = adapt_conformance(derive_not_computed_stream(stream))
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

    # Probes 5 to 8: the adapter's four refusals, each run through the same
    # predicate `--mutation-probes` uses. Sharing the predicate is the point: a
    # probe that passes here and a weakened adapter that the mutation run
    # detects are then statements about one piece of code, not two.
    refusal_states = {
        "refuses-a-foreign-protocol": "unsupported",
        "refuses-an-empty-stream": "vacuous",
        "refuses-a-malformed-row": "malformed",
        "refuses-an-unnamed-outcome": "unsupported",
    }
    for name, check, _ in ADAPTER_CHECKS:
        matched, detail = check(adapt_conformance, stream)
        results.append(
            {
                "probe": name,
                "state": refusal_states[name],
                "matched": matched,
                "detail": detail,
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
        "--mutation-probes",
        action="store_true",
        help=(
            "weaken one adapter refusal at a time and require the check that "
            "guards it to go red; exits 1 if any weakening goes undetected"
        ),
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

    if arguments.mutation_probes:
        # The producer stream is required. Deriving the mutation cases from a
        # hand-written stream would probe the adapter against bytes no producer
        # ever emitted, which is a different and weaker claim.
        try:
            stream = require_inputs()["PROOF-corpus-conformance"].read_text(encoding="utf-8")
            probes = mutation_probes(stream)
        except (ChainError, OSError) as error:
            print(str(error), file=sys.stderr)
            return 2
        for probe in probes:
            print(
                f"mutation {probe['probe']} [{probe['weakening']}]: "
                f"{'detected' if probe['detected'] else 'UNDETECTED'}"
            )
        if arguments.json:
            print(json.dumps({"mutation_probes": probes}, indent=2, sort_keys=True))
        undetected = [probe["probe"] for probe in probes if not probe["detected"]]
        if undetected:
            print(
                f"these adapter checks did not go red when the refusal they guard was "
                f"switched off: {undetected}. A check that cannot be made to fail is a "
                "check that was never checking.",
                file=sys.stderr,
            )
            return 1
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
        # Only a case that ran and matched counts. A scenario that failed did not
        # demonstrate its state, and one that demonstrates no state says so with
        # a null rather than borrowing a label.
        "states_demonstrated": sorted(
            {
                item["state"]
                for group in (chain["scenarios"], probes)
                for item in group
                if item["matched"] and item.get("state") is not None
            }
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
