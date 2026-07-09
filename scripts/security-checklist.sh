#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> DTCS security checklist (automated probes)"
cargo run --quiet --bin dtcs -- conformance run --profile integrated-platform --json > /tmp/dtcs-security-report.json

python3 - <<'PY'
import json
import sys

with open("/tmp/dtcs-security-report.json") as f:
    report = json.load(f)

security = report.get("security", [])
failed = [item for item in security if not item.get("passed")]
if failed:
    print("FAILED automated security probes:", file=sys.stderr)
    for item in failed:
        print(f"  - {item['id']}: {item.get('message', 'failed')}", file=sys.stderr)
    sys.exit(1)

print(f"OK: {len(security)} automated security probes passed")
for item in security:
    note = item.get("message")
    if note:
        print(f"  - {item['id']}: {note}")
PY

echo "==> Manual review items remain documented in docs/adoption/security-checklist.md"
