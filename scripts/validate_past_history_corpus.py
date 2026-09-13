#!/usr/bin/env python3
"""Validate the closed paired past/history corpus schema."""

from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    from jsonschema.validators import validator_for
except ImportError as error:
    print(f"past/history corpus validation unavailable: {error}", file=sys.stderr)
    raise SystemExit(125) from error


ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / "corpus" / "past-history"


def main() -> int:
    try:
        schema = json.loads((CORPUS / "schema.json").read_text(encoding="utf-8"))
        cases = json.loads((CORPUS / "cases.json").read_text(encoding="utf-8"))
        validator_type = validator_for(schema)
        validator_type.check_schema(schema)
        errors = sorted(
            validator_type(schema).iter_errors(cases),
            key=lambda item: list(item.path),
        )
        if errors:
            first = errors[0]
            location = "/".join(str(item) for item in first.path) or "<root>"
            raise AssertionError(f"{location}: {first.message}")
        print("paired past/history corpus satisfies its closed schema")
        return 0
    except (AssertionError, OSError, json.JSONDecodeError, ValueError) as error:
        print(f"past/history corpus validation failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
