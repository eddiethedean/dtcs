#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "Checking Rust API documentation..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked

echo "Checking Markdown links..."
python3 scripts/check-markdown-links.py

# Suppress Material-for-MkDocs advisory banner about unreleased MkDocs 2.0.
export NO_MKDOCS_2_WARNING=true

build_mkdocs() {
  mkdocs build --strict
}

if command -v mkdocs >/dev/null 2>&1; then
  echo "Building MkDocs site..."
  build_mkdocs
elif [[ -x .venv/bin/mkdocs ]]; then
  echo "Building MkDocs site..."
  .venv/bin/mkdocs build --strict
elif [[ -x .docs-venv/bin/mkdocs ]]; then
  echo "Building MkDocs site..."
  .docs-venv/bin/mkdocs build --strict
else
  echo "MkDocs not installed; installing docs requirements into .docs-venv..."
  python3 -m venv .docs-venv
  # shellcheck disable=SC1091
  source .docs-venv/bin/activate
  pip install -q -r docs/requirements.txt
  build_mkdocs
  deactivate
fi

echo "Documentation checks passed."
