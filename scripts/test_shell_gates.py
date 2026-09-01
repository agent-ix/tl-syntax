#!/usr/bin/env python3
"""Exercise shell and remaining Python gate entry points against corrupt inputs."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
from collections.abc import Callable
from contextlib import contextmanager
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "check_evidence_shell_contract", ROOT / "scripts" / "check_evidence_shell_contract.py"
)
assert SPEC is not None and SPEC.loader is not None
CONTRACT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CONTRACT)
PROFILE_SPEC = importlib.util.spec_from_file_location(
    "evidence_profile", ROOT / "scripts" / "evidence_profile.py"
)
assert PROFILE_SPEC is not None and PROFILE_SPEC.loader is not None
PROFILE = importlib.util.module_from_spec(PROFILE_SPEC)
PROFILE_SPEC.loader.exec_module(PROFILE)

LIVE_GATE_PATHS = (
    Path("scripts/evidence_profile.py"),
    Path("scripts/test_shell_gates.py"),
    Path("scripts/tool_identity.py"),
    Path("scripts/verify_evidence.sh"),
)


def replace_manifest_digest(path: Path, relative: str, digest: str) -> None:
    lines = path.read_text(encoding="utf-8").splitlines()
    suffix = f"  {relative}"
    matches = [index for index, line in enumerate(lines) if line.endswith(suffix)]
    assert len(matches) == 1, f"expected one manifest entry for {relative}"
    lines[matches[0]] = f"{digest}{suffix}"
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sync_live_gate_sources(tree: Path) -> None:
    """Overlay this suite's live gate sources, including deletions."""
    for relative in LIVE_GATE_PATHS:
        source = ROOT / relative
        destination = tree / relative
        if source.is_file():
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, destination)
        elif destination.exists():
            destination.unlink()


@contextmanager
def live_gate_worktree():
    temporary = tempfile.TemporaryDirectory(prefix="tl-syntax-shell-gate-")
    tree = Path(temporary.name) / "tree"
    retained_manifest = active_record(ROOT).with_suffix(".sha256")
    baseline_revision = subprocess.run(
        [
            "/usr/bin/git", "log", "-1", "--format=%H", "--diff-filter=A", "--",
            str(retained_manifest.relative_to(ROOT)),
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    assert baseline_revision, "cannot locate the active record's introduction commit"
    added = subprocess.run(
        [
            "/usr/bin/git", "worktree", "add", "--detach", str(tree),
            baseline_revision,
        ],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    assert added.returncode == 0, f"cannot create shell-gate worktree: {added.stderr}"
    try:
        # The evidence records and independent validators intentionally come from
        # the committed fixture. Only the live shell/profile sources owned by this
        # suite are overlaid, and a deletion is overlaid as a deletion.
        sync_live_gate_sources(tree)
        yield tree
    finally:
        removed = subprocess.run(
            ["/usr/bin/git", "worktree", "remove", "--force", str(tree)],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        temporary.cleanup()
        pruned = subprocess.run(
            ["/usr/bin/git", "worktree", "prune"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        assert removed.returncode == 0, f"cannot remove shell-gate worktree: {removed.stderr}"
        assert pruned.returncode == 0, f"cannot prune shell-gate worktrees: {pruned.stderr}"


def active_record(tree: Path) -> Path:
    registry = json.loads(
        (tree / "evidence" / "RETRACTIONS.json").read_text(encoding="utf-8")
    )
    retracted = set(registry["records"])
    records = sorted(
        path
        for path in (tree / "evidence").glob("tl-syntax-v01-*")
        if path.is_dir() and path.name not in retracted
    )
    assert len(records) == 1, f"expected exactly one active evidence record, got {records}"
    return records[0]


def run_shell_gate(tree: Path) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["/usr/bin/bash", "scripts/verify_evidence.sh"],
        cwd=tree,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def run_profile_gate(tree: Path) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["/usr/bin/python3", "scripts/evidence_profile.py", "--verify-census"],
        cwd=tree,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def repair_record_chain(tree: Path, record: Path, changed: tuple[Path, ...]) -> None:
    outer = record.with_suffix(".sha256")
    for path in changed:
        replace_manifest_digest(outer, str(path.relative_to(tree)), sha256(path))
    replace_manifest_digest(
        tree / "evidence" / "ANCHORS",
        str(outer.relative_to(tree)),
        sha256(outer),
    )


def disable_shell_command(tree: Path, command: str) -> None:
    script = tree / "scripts" / "verify_evidence.sh"
    text = script.read_text(encoding="utf-8")
    assert text.count(command) == 1
    script.write_text(text.replace(command, f"# {command}", 1), encoding="utf-8")


def disable_finalizer(tree: Path) -> None:
    disable_shell_command(
        tree,
        '/usr/bin/python3 scripts/finalize_collection.py --check "${checksum%.sha256}"',
    )


def disable_history_gate(tree: Path) -> None:
    # A retained manifest is immutable after introduction, so a negative fixture
    # cannot repair its digest chain without the orthogonal history gate reacting.
    # Disable only that earlier stage in the disposable worktree; the manifest and
    # anchor verifiers remain live, which proves the repaired chain reaches the
    # finalizer under test.
    disable_shell_command(tree, "/usr/bin/python3 scripts/verify_evidence_tree.py")


def restore_fixture(tree: Path, pristine_evidence: Path) -> None:
    shutil.rmtree(tree / "evidence")
    shutil.copytree(pristine_evidence, tree / "evidence")
    sync_live_gate_sources(tree)


def corrupt_inner_manifest(_tree: Path, record: Path) -> None:
    manifest = record / "evidence-manifest.json"
    manifest.write_bytes(manifest.read_bytes() + b"\n")


def corrupt_parameters_digest(_tree: Path, record: Path) -> None:
    envelope_path = record / "evidence-envelope.json"
    envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
    envelope["parametersDigest"]["value"] = "0" * 64
    envelope_path.write_text(
        json.dumps(envelope, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    summary_path = record / "collection-summary.json"
    summary = json.loads(summary_path.read_text(encoding="utf-8"))
    summary["finalEnvelopeSha256"] = sha256(envelope_path)
    summary_path.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    repair_record_chain(_tree, record, (envelope_path, summary_path))


def corrupt_tool_identity(tree: Path, record: Path) -> None:
    input_path = record / "collection-input.json"
    collection_input = json.loads(input_path.read_text(encoding="utf-8"))
    collection_input["tools"]["identities"]["cargo"]["sha256"] = "0" * 64
    input_path.write_text(
        json.dumps(collection_input, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    envelope_path = record / "evidence-envelope.json"
    envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
    envelope["inputs"][0]["contentDigest"]["value"] = sha256(input_path)
    envelope_path.write_text(
        json.dumps(envelope, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    summary_path = record / "collection-summary.json"
    summary = json.loads(summary_path.read_text(encoding="utf-8"))
    summary["finalEnvelopeSha256"] = sha256(envelope_path)
    summary_path.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    repair_record_chain(tree, record, (input_path, envelope_path, summary_path))


def corrupt_summary_projection(tree: Path, record: Path) -> None:
    summary_path = record / "collection-summary.json"
    summary = json.loads(summary_path.read_text(encoding="utf-8"))
    summary["outcomes"][0]["status"] = "failed"
    summary_path.write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    repair_record_chain(tree, record, (summary_path,))


def corrupt_retracted_record(tree: Path, _record: Path) -> None:
    registry = json.loads(
        (tree / "evidence" / "RETRACTIONS.json").read_text(encoding="utf-8")
    )
    retracted = tree / "evidence" / sorted(registry["records"])[0]
    artifact = retracted / "collection-input.json"
    artifact.write_bytes(artifact.read_bytes() + b"\n")


def plant_root_artifact(tree: Path, _record: Path) -> None:
    (tree / "evidence" / ".POLICY-SHELL-PROBE").write_text(
        "fabricated\n", encoding="utf-8"
    )


def retract_only_active_record(tree: Path, record: Path) -> None:
    registry_path = tree / "evidence" / "RETRACTIONS.json"
    registry = json.loads(registry_path.read_text(encoding="utf-8"))
    registry["records"][record.name] = {
        "reason": "policy probe must not permit zero active qualified records"
    }
    registry_path.write_text(
        json.dumps(registry, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    static = tree / "evidence" / "STATIC.sha256"
    replace_manifest_digest(
        static,
        "evidence/RETRACTIONS.json",
        hashlib.sha256(registry_path.read_bytes()).hexdigest(),
    )
    replace_manifest_digest(
        tree / "evidence" / "ANCHORS",
        "evidence/STATIC.sha256",
        hashlib.sha256(static.read_bytes()).hexdigest(),
    )


def activate_second_record(tree: Path, _record: Path) -> None:
    registry_path = tree / "evidence" / "RETRACTIONS.json"
    registry = json.loads(registry_path.read_text(encoding="utf-8"))
    second = "tl-syntax-v01-20987eed0128-20260831T223910Z"
    assert second in registry["records"]
    del registry["records"][second]
    registry_path.write_text(
        json.dumps(registry, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    static = tree / "evidence" / "STATIC.sha256"
    replace_manifest_digest(
        static, "evidence/RETRACTIONS.json", sha256(registry_path)
    )
    replace_manifest_digest(
        tree / "evidence" / "ANCHORS", "evidence/STATIC.sha256", sha256(static)
    )


def main() -> int:
    shell_text = (ROOT / "scripts" / "verify_evidence.sh").read_text(encoding="utf-8")
    for command in CONTRACT.REQUIRED:
        assert CONTRACT.inspect(shell_text.replace(command, "true", 1)), (
            f"evidence shell contract accepted removal of {command}"
        )
    with live_gate_worktree() as tree:
        with tempfile.TemporaryDirectory(
            prefix="tl-syntax-evidence-fixture-"
        ) as fixture_directory:
            pristine_evidence = Path(fixture_directory) / "evidence"
            shutil.copytree(tree / "evidence", pristine_evidence)
            baseline = run_shell_gate(tree)
            assert baseline.returncode == 0, (
                "unmodified evidence shell fixture is not green: "
                + baseline.stderr.decode(errors="replace")
            )

            scenarios: tuple[tuple[Callable[[Path, Path], None], str], ...] = (
                (plant_root_artifact, "planted root artifact"),
                (corrupt_inner_manifest, "corrupt active-record manifest"),
                (corrupt_retracted_record, "corrupt retracted record"),
                (retract_only_active_record, "zero-active census"),
                (activate_second_record, "two-active census"),
            )
            for mutator, description in scenarios:
                restore_fixture(tree, pristine_evidence)
                mutator(tree, active_record(tree))
                rejected = run_shell_gate(tree)
                assert rejected.returncode != 0, f"shell gate accepted {description}"

            for mutator, description in (
                (corrupt_parameters_digest, "mutated parametersDigest"),
                (corrupt_tool_identity, "mutated tool identity"),
                (corrupt_summary_projection, "mutated summary projection"),
            ):
                restore_fixture(tree, pristine_evidence)
                mutator(tree, active_record(tree))
                disable_history_gate(tree)
                rejected = run_shell_gate(tree)
                assert rejected.returncode != 0, f"finalizer accepted {description}"
                disable_finalizer(tree)
                bypassed = run_shell_gate(tree)
                assert bypassed.returncode == 0, (
                    f"{description} was rejected before the finalizer stage: "
                    + bypassed.stderr.decode(errors="replace")
                )

            restore_fixture(tree, pristine_evidence)
            source = active_record(tree) / "source-revision.txt"
            source.write_text("not-a-revision\n", encoding="utf-8")
            malformed = run_profile_gate(tree)
            assert malformed.returncode == 1, (
                "malformed source revision was not classified failed"
            )

            restore_fixture(tree, pristine_evidence)
            source = active_record(tree) / "source-revision.txt"
            source.write_text("0" * 40 + "\n", encoding="utf-8")
            unavailable = run_profile_gate(tree)
            assert unavailable.returncode == 2, (
                "missing source revision was not classified unavailable"
            )

            restore_fixture(tree, pristine_evidence)
            source = active_record(tree) / "source-revision.txt"
            roots = subprocess.run(
                ["/usr/bin/git", "rev-list", "--max-parents=0", "HEAD"], cwd=tree,
                check=True, capture_output=True, text=True,
            ).stdout.splitlines()
            assert len(roots) == 1, f"expected one repository root commit, got {roots}"
            root_revision = roots[0]
            source.write_text(root_revision + "\n", encoding="utf-8")
            invalid = run_profile_gate(tree)
            assert invalid.returncode == 1, (
                "reachable source without tools.lock was not classified failed"
            )

            restore_fixture(tree, pristine_evidence)
            try:
                PROFILE.qualification_census(tree, head=root_revision)
            except ValueError as error:
                assert "differs from the current source head" in str(error)
            else:
                raise AssertionError("stale active evidence was accepted for a divergent head")

    planted_rust = ROOT / "src" / ".policy_unsafe_probe.rs"
    planted_rust.write_text("fn probe() { unsafe { core::hint::unreachable_unchecked() } }\n", encoding="utf-8")
    try:
        result = subprocess.run(
            ["/usr/bin/bash", "scripts/check_unsafe_comments.sh"], cwd=ROOT,
            check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        assert result.returncode != 0, "unsafe-comment shell gate accepted an unreviewed block"
    finally:
        planted_rust.unlink(missing_ok=True)

    with tempfile.TemporaryDirectory() as directory:
        tree = Path(directory) / "tree.txt"
        tree.write_text("tl-syntax v0.1.0 (/x)\nforged v1.0.0\n", encoding="utf-8")
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "check_default_dependencies.py"),
             "--tree-output", str(tree)], check=False, capture_output=True,
        )
        assert result.returncode != 0, "default-dependency gate exit contract accepted a dependency"

    print("shell and Python gate entry-point behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
