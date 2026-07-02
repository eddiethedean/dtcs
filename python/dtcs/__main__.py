"""Command-line interface for the DTCS Python package."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from dtcs import SPEC_VERSION, inspect, is_valid, parse_and_validate, parse_file


def _render_report(report: dict, *, json_output: bool, mode: str) -> None:
    if json_output:
        if mode == "validate":
            payload = {"valid": is_valid(report), "diagnostics": report.get("diagnostics", [])}
        else:
            payload = {"diagnostics": report.get("diagnostics", [])}
        print(json.dumps(payload, indent=2))
        return

    diagnostics = report.get("diagnostics", [])
    if not diagnostics:
        print("valid" if mode == "validate" else "no diagnostics")
        return

    for diagnostic in diagnostics:
        severity = diagnostic.get("severity", "error")
        code = diagnostic.get("id", "dtcs:unknown")
        category = diagnostic.get("category", "syntax")
        message = diagnostic.get("message", "")
        print(f"[{severity}] {code} ({category}) - {message}")
        if object_ref := diagnostic.get("object_ref"):
            print(f"  at: {object_ref}")
        if remediation := diagnostic.get("remediation"):
            print(f"  hint: {remediation}")

    if mode == "validate" and is_valid(report):
        print("valid")


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="dtcs",
        description="Validate DTCS transformation contracts",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    validate_parser = subparsers.add_parser("validate", help="Parse and validate a contract")
    validate_parser.add_argument("path", type=Path)
    validate_parser.add_argument("--json", action="store_true")

    inspect_parser = subparsers.add_parser("inspect", help="Print a contract summary")
    inspect_parser.add_argument("path", type=Path)
    inspect_parser.add_argument("--json", action="store_true")

    diagnostics_parser = subparsers.add_parser(
        "diagnostics",
        help="Print validation diagnostics",
    )
    diagnostics_parser.add_argument("path", type=Path)
    diagnostics_parser.add_argument("--json", action="store_true")

    version_parser = subparsers.add_parser("version", help="Print package versions")
    version_parser.add_argument("--json", action="store_true")

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)

    if args.command == "version":
        if args.json:
            print(
                json.dumps(
                    {
                        "crateVersion": _package_version(),
                        "specVersion": SPEC_VERSION,
                    },
                    indent=2,
                )
            )
        else:
            print(f"dtcs {_package_version()}")
            print(f"spec {SPEC_VERSION}")
        return 0

    path = args.path
    if args.command == "validate":
        report = _validate_path(path)
        _render_report(report, json_output=args.json, mode="validate")
        return 0 if is_valid(report) else 1

    if args.command == "diagnostics":
        report = _validate_path(path)
        _render_report(report, json_output=args.json, mode="diagnostics")
        return 0 if is_valid(report) else 1

    result = parse_file(str(path))
    report = _validate_path(path)
    if not is_valid(report):
        _render_report(report, json_output=args.json, mode="diagnostics")
        return 1

    contract = result.get("contract")
    if contract is None:
        _render_report(report, json_output=args.json, mode="diagnostics")
        return 1

    if args.json:
        print(
            json.dumps(
                {
                    "id": contract["id"],
                    "name": contract["name"],
                    "version": contract["version"],
                    "dtcsVersion": contract["dtcsVersion"],
                    "inputs": len(contract.get("inputs", [])),
                    "outputs": len(contract.get("outputs", [])),
                    "semanticActions": len(contract.get("semanticActions", [])),
                    "rules": len(contract.get("rules", [])),
                    "expressions": len(contract.get("expressions", [])),
                    "functions": len(contract.get("functions", [])),
                },
                indent=2,
            )
        )
    else:
        print(inspect(contract), end="")
    return 0


def _package_version() -> str:
    from importlib.metadata import version

    return version("dtcs")


def _validate_path(path: Path) -> dict:
    content = path.read_bytes()
    if path.suffix == ".json":
        doc_format = "json"
    else:
        doc_format = "yaml"
    return parse_and_validate(content, doc_format)


if __name__ == "__main__":
    sys.exit(main())
