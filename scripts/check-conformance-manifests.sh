#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

src="src/conformance/manifest.json"
tests="tests/conformance/manifest.json"

if ! cmp -s "$src" "$tests"; then
  echo "ERROR: conformance manifests differ:" >&2
  echo "  $src" >&2
  echo "  $tests" >&2
  echo "Keep them byte-identical (see CONTRIBUTING.md)." >&2
  exit 1
fi

echo "OK: conformance manifests match ($src == $tests)"
