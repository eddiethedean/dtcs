#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "Checking Rust API documentation..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked

echo "Checking Markdown links..."
python3 scripts/check-markdown-links.py

echo "Documentation checks passed."
