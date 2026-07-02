"""Command-line interface for the DTCS Python package."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from dtcs import (
    SPEC_VERSION,
    __version__,
    compat_analyze,
    evolve_analyze,
    inspect,
    is_valid,
    lineage_analyze,
    parse_file,
    validate_result,
)


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
        if object_ref := diagnostic.get("objectRef"):
            print(f"  at: {object_ref}")
        if remediation := diagnostic.get("remediation"):
            print(f"  hint: {remediation}")

    if mode == "validate" and is_valid(report):
        print("valid")


def _load_valid_contract(path: Path) -> dict:
    try:
        result = parse_file(str(path))
    except ValueError as error:
        raise SystemExit(str(error)) from error
    report = validate_result(result)
    if not is_valid(report):
        _render_report(report, json_output=False, mode="validate")
        raise SystemExit(f"validation failed for {path}")
    contract = result.get("contract")
    if contract is None:
        raise SystemExit(f"no contract in {path}")
    return contract


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="dtcs",
        description="Validate and analyze DTCS transformation contracts",
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

    compat_parser = subparsers.add_parser("compat", help="Compare contract compatibility")
    compat_parser.add_argument("source", type=Path)
    compat_parser.add_argument("target", type=Path)
    compat_parser.add_argument("--scope", default="")
    compat_parser.add_argument("--json", action="store_true")

    evolve_parser = subparsers.add_parser("evolve", help="Analyze contract evolution")
    evolve_parser.add_argument("older", type=Path)
    evolve_parser.add_argument("newer", type=Path)
    evolve_parser.add_argument("--json", action="store_true")

    lineage_parser = subparsers.add_parser("lineage", help="Analyze contract lineage")
    lineage_parser.add_argument("path", type=Path)
    lineage_parser.add_argument("--impact", default=None)
    lineage_parser.add_argument("--dependency", default=None)
    lineage_parser.add_argument("--json", action="store_true")

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

    if args.command == "compat":
        source = _load_valid_contract(args.source)
        target = _load_valid_contract(args.target)
        scope = [part for part in args.scope.split(",") if part] if args.scope else None
        try:
            report = compat_analyze(source, target, scope)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 2
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            print(f"compatibility: {report.get('level')}")
        level = report.get("level")
        return 0 if level and level != "incompatible" else 1

    if args.command == "evolve":
        older = _load_valid_contract(args.older)
        newer = _load_valid_contract(args.newer)
        report = evolve_analyze(older, newer)
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            print(
                f"evolution: {report.get('compatibility')} "
                f"(same identity: {report.get('sameIdentity')})"
            )
        return 0 if report.get("sameIdentity") and report.get("compatibility") != "incompatible" else 1

    if args.command == "lineage":
        contract = _load_valid_contract(args.path)
        report = lineage_analyze(contract, args.impact, args.dependency)
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            for edge in report.get("graph", []):
                print(f"{edge['output']} <- {edge['inputs']}")
            if impact := report.get("impact"):
                print(f"impact {impact['input']}: {impact['outputs']}")
            if dependency := report.get("dependency"):
                print(f"dependency {dependency['output']}: {dependency['inputs']}")
            governance = report.get("governance") or {}
            if owner := governance.get("owner"):
                print(f"governance owner: {owner}")
            if steward := governance.get("steward"):
                print(f"governance steward: {steward}")
        return 0

    path = args.path
    try:
        result = parse_file(str(path))
    except ValueError as error:
        print(str(error), file=sys.stderr)
        return 1
    report = validate_result(result)

    if args.command == "validate":
        _render_report(report, json_output=args.json, mode="validate")
        return 0 if is_valid(report) else 1

    if args.command == "diagnostics":
        _render_report(report, json_output=args.json, mode="diagnostics")
        return 0 if is_valid(report) else 1

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
    try:
        from importlib.metadata import version

        return version("dtcs")
    except Exception:
        return __version__


if __name__ == "__main__":
    sys.exit(main())
