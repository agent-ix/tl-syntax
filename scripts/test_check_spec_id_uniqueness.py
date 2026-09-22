#!/usr/bin/env python3
"""Regression tests for scripts/check_spec_id_uniqueness.py frontmatter parsing.

Reproduces three cases found by code review (TL-199, spec/reviews/SR-110) where
a genuine cross-document id collision went completely unreported: a trailing
YAML comment on an `id:` line, a quoted id value, and a file that fails UTF-8
decoding. Also checks that an undecodable file now prints a visible warning
instead of being silently dropped, and that the unrelated Task-NNN
per-plan-bundle namespace logic still isn't flagging cross-bundle reuse.
"""

from __future__ import annotations

import contextlib
import importlib.util
import io
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "check_spec_id_uniqueness", ROOT / "scripts" / "check_spec_id_uniqueness.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def _run(tmp_root: Path) -> tuple[int, list[str]]:
    """Run main() against tmp_root/spec, with module ROOT/SPEC_DIR swapped in
    for the duration of the call and always restored, output captured."""
    original_root, original_spec_dir = MODULE.ROOT, MODULE.SPEC_DIR
    MODULE.ROOT = tmp_root
    MODULE.SPEC_DIR = tmp_root / "spec"
    buffer = io.StringIO()
    try:
        with contextlib.redirect_stdout(buffer), contextlib.redirect_stderr(buffer):
            code = MODULE.main(["check_spec_id_uniqueness.py"])
    finally:
        MODULE.ROOT, MODULE.SPEC_DIR = original_root, original_spec_dir
    return code, buffer.getvalue().splitlines()


def _write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def test_trailing_comment_collision_is_caught(tmp_root: Path) -> None:
    """A trailing YAML comment on an id: line must not hide a real collision."""
    _write(tmp_root / "spec/requirements/a.md", "---\nid: FR-010  # renumbered\n---\n")
    _write(tmp_root / "spec/requirements/b.md", "---\nid: FR-010\n---\n")
    code, _lines = _run(tmp_root)
    assert code == 1, "trailing YAML comment on an id: line let a real collision through"


def test_double_quoted_id_collision_is_caught(tmp_root: Path) -> None:
    """A double-quoted id value must collide with the same unquoted id."""
    _write(tmp_root / "spec/requirements/a.md", '---\nid: "FR-020"\n---\n')
    _write(tmp_root / "spec/requirements/b.md", "---\nid: FR-020\n---\n")
    code, _lines = _run(tmp_root)
    assert code == 1, "a double-quoted id value let a real collision through"


def test_single_quoted_id_collision_is_caught(tmp_root: Path) -> None:
    """A single-quoted id value must collide with the same unquoted id."""
    _write(tmp_root / "spec/requirements/a.md", "---\nid: 'FR-021'\n---\n")
    _write(tmp_root / "spec/requirements/b.md", "---\nid: FR-021\n---\n")
    code, _lines = _run(tmp_root)
    assert code == 1, "a single-quoted id value let a real collision through"


def test_undecodable_file_warns_and_is_excluded(tmp_root: Path) -> None:
    """A file that fails UTF-8 decoding must print a visible warning naming it
    (never be silently dropped), while remaining excluded from the id it would
    otherwise have declared -- it still doesn't fail the run by itself."""
    bad = tmp_root / "spec/requirements/bad-encoding.md"
    bad.parent.mkdir(parents=True, exist_ok=True)
    bad.write_bytes(b"---\nid: FR-030\n---\n# body\nbad byte: \xff\xfe\n")
    _write(tmp_root / "spec/requirements/clean.md", "---\nid: FR-030\n---\n")
    code, lines = _run(tmp_root)
    assert code == 0, "an undecodable file must not itself fail the run"
    warnings = [line for line in lines if line.startswith("warning: could not read")]
    assert warnings, "an undecodable file was excluded with no visible warning"
    assert "bad-encoding.md" in warnings[0]


def test_comment_only_id_line_means_no_id(tmp_root: Path) -> None:
    """`id: # nothing here` declares no real value: must not crash or collide."""
    _write(tmp_root / "spec/requirements/a.md", "---\nid: # nothing here\n---\n")
    code, _lines = _run(tmp_root)
    assert code == 0, "a comment-only id: line was treated as declaring an id"


def test_cross_bundle_task_ids_still_not_flagged(tmp_root: Path) -> None:
    """Guard against a regression in the unrelated Task-NNN per-plan-bundle
    namespace logic while touching extract_id()."""
    _write(tmp_root / "spec/plans/PLAN-A/tasks/Task-001-a.md", "---\nid: Task-001\n---\n")
    _write(tmp_root / "spec/plans/PLAN-B/tasks/Task-001-b.md", "---\nid: Task-001\n---\n")
    code, _lines = _run(tmp_root)
    assert code == 0, "Task-001 reused across two different plan bundles was flagged"


TESTS = (
    test_trailing_comment_collision_is_caught,
    test_double_quoted_id_collision_is_caught,
    test_single_quoted_id_collision_is_caught,
    test_undecodable_file_warns_and_is_excluded,
    test_comment_only_id_line_means_no_id,
    test_cross_bundle_task_ids_still_not_flagged,
)


def main() -> int:
    for test in TESTS:
        with tempfile.TemporaryDirectory() as tmp:
            test(Path(tmp))
        print(f"ok - {test.__name__}")
    print(f"check_spec_id_uniqueness regression tests: {len(TESTS)} passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
