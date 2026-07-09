#!/usr/bin/env python3
"""Verify relative Markdown links resolve to files in the repository."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
SKIP_PREFIXES = ("http://", "https://", "mailto:", "#")


def main() -> int:
    errors: list[str] = []

    for md in sorted(ROOT.rglob("*.md")):
        if any(part in {".git", "target", ".venv"} for part in md.parts):
            continue

        text = md.read_text(encoding="utf-8")
        for match in LINK_RE.finditer(text):
            target = match.group(1).strip()
            if not target or target.startswith(SKIP_PREFIXES):
                continue

            path_part = target.split()[0]
            if "#" in path_part:
                path_part = path_part.split("#", 1)[0]
            if not path_part:
                continue

            resolved = (md.parent / path_part).resolve()
            try:
                resolved.relative_to(ROOT.resolve())
            except ValueError:
                errors.append(f"{md.relative_to(ROOT)}: [{target}] escapes repository")
                continue

            if not resolved.exists():
                errors.append(f"{md.relative_to(ROOT)}: [{target}] not found")

    if errors:
        print("Broken Markdown links:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("Markdown links OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
