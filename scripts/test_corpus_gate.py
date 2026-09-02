#!/usr/bin/env python3
"""Behavioral mutation tests for independently derived corpus outcomes."""

from __future__ import annotations

import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "validate_corpus", ROOT / "scripts" / "validate_corpus.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def main() -> int:
    original = MODULE.CORPUS
    with tempfile.TemporaryDirectory() as temporary:
        corpus = Path(temporary) / "corpus"
        shutil.copytree(original, corpus)
        manifest_path = corpus / "manifest.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        fixture = next(
            item for item in manifest["fixtures"] if item["expected_validation"] == "valid"
        )
        fixture["expected_closed_trace"] = not fixture["expected_closed_trace"]
        manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
        actual = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "validate_corpus.py"),
             "--corpus", str(corpus)], check=False, capture_output=True,
        )
        assert actual.returncode != 0, "corpus validator exit contract accepted a mutation"
        MODULE.CORPUS = corpus
        try:
            MODULE.validate()
        except AssertionError as error:
            assert "derived" in str(error), error
        else:
            raise AssertionError("a mutated closed-trace oracle was accepted")
        finally:
            MODULE.CORPUS = original
    print("derived corpus oracle behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
