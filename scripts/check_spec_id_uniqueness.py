#!/usr/bin/env python3
"""Fail when two spec documents declare the same `id:` frontmatter value.

`quire validate` checks each document's own archetype structure; it does not
compare an `id:` value against every other document in the tree. Nothing
before this script did either, and that gap is exactly how TL-190 happened:
two independent pull requests each minted SR-066..072 in spec/reviews/ and it
went unnoticed until the collision was found by hand. This script is the
cross-document check that was missing.

Scope: every `spec/**/*.md` file that declares an `id:` in its YAML
frontmatter -- requirements, plans, task files, reviews, ADRs, and the rest of
the ecosystem/ and evidence/ trees alike. Anywhere a duplicate id can land and
go unnoticed is in scope; TL-190's incident was in spec/reviews/, but nothing
about the failure mode is specific to that directory.

One deliberate exception: `Task-NNN` ids. A plan bundle's tasks are numbered
from Task-001 within `spec/plans/<PLAN>/tasks/`, and every plan bundle starts
that count over again -- Task-001 exists once per plan by design, not by
accident. `relationships:` targets confirm the convention: a task's
`ix://.../Task-NNN` references always resolve to a task in its own plan
bundle, never a sibling plan's. So Task ids are checked for uniqueness within
each plan bundle's tasks/ directory, not across the whole tree. Every other id
prefix (FR, NFR, StR, SR, PLAN, ADR, IF, VO, ...) is a single flat namespace
and is checked globally.

Exit status: 0 when every id is unique in its namespace, 1 when a collision is
found (every collision is printed, not just the first), 2 on a usage error.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC_DIR = ROOT / "spec"

FRONTMATTER_ID_RE = re.compile(r"^id:\s*(\S+)\s*$", re.MULTILINE)
TASK_ID_RE = re.compile(r"^Task-\d+$")


def extract_id(path: Path) -> str | None:
    """Return the frontmatter `id:` value, or None if the doc declares none."""
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return None
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---", 4)
    frontmatter = text[4:end] if end != -1 else text[4:]
    match = FRONTMATTER_ID_RE.search(frontmatter)
    return match.group(1) if match else None


def namespace_key(path: Path, doc_id: str) -> tuple[str, str]:
    """Group ids into their uniqueness namespace: global, or per plan bundle."""
    if TASK_ID_RE.match(doc_id):
        try:
            tasks_dir = path.resolve().parent
            plan_dir = tasks_dir.parent
            if tasks_dir.name == "tasks" and plan_dir.parent == SPEC_DIR.resolve() / "plans":
                return (f"plan:{plan_dir.name}", doc_id)
        except (OSError, ValueError):
            pass
    return ("global", doc_id)


def find_duplicates() -> dict[tuple[str, str], list[Path]]:
    by_key: dict[tuple[str, str], list[Path]] = {}
    for path in sorted(SPEC_DIR.rglob("*.md")):
        doc_id = extract_id(path)
        if doc_id is None:
            continue
        key = namespace_key(path, doc_id)
        by_key.setdefault(key, []).append(path)
    return {key: paths for key, paths in by_key.items() if len(paths) > 1}


def main(argv: list[str]) -> int:
    if argv[1:]:
        print("usage: check_spec_id_uniqueness.py", file=sys.stderr)
        return 2
    if not SPEC_DIR.is_dir():
        print(f"no spec/ directory at {SPEC_DIR}", file=sys.stderr)
        return 2

    duplicates = find_duplicates()
    if not duplicates:
        print("spec id uniqueness: OK (no duplicate id: values)")
        return 0

    for (_scope, doc_id), paths in sorted(duplicates.items()):
        rel = [str(p.relative_to(ROOT)) for p in paths]
        print(f"duplicate id '{doc_id}' declared in {len(rel)} files:", file=sys.stderr)
        for r in rel:
            print(f"  - {r}", file=sys.stderr)

    print(
        f"spec id uniqueness: FAILED ({len(duplicates)} duplicate id value(s))",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
