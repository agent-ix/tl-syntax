#!/usr/bin/env python3
"""Behavioral self-test for required JSON Schema format enforcement."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GATE = ROOT / "scripts" / "validate_json_schema.py"


def main() -> int:
    with tempfile.TemporaryDirectory() as temporary:
        directory = Path(temporary)
        schema = directory / "schema.json"
        instance = directory / "instance.json"
        schema.write_text(
            json.dumps(
                {
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "type": "string",
                    "format": "date-time",
                }
            ),
            encoding="utf-8",
        )
        instance.write_text(json.dumps("not-a-date-time"), encoding="utf-8")
        result = subprocess.run(
            [sys.executable, str(GATE), str(schema), str(instance)],
            text=True,
            capture_output=True,
            check=False,
        )
        if result.returncode not in {1, 125}:
            print(
                "format gate falsely accepted an invalid date-time or returned an "
                f"undocumented status {result.returncode}: {result.stdout}{result.stderr}",
                file=sys.stderr,
            )
            return 1
        if result.returncode == 125 and "unavailable" not in result.stderr:
            print("missing format support was not classified as unavailable", file=sys.stderr)
            return 1
    print("required JSON Schema format behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
