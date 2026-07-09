"""Command-line interface for the DTCS Python package."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from dtcs import (
    analyze,
    SPEC_VERSION,
    __version__,
    compat_analyze,
    evolve_analyze,
    inspect,
    is_valid,
    lineage_analyze,
    parse_file,
    plan_lower,
    registry_list,
    registry_resolve,
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


def _load_valid_contract(
    path: Path,
    *,
    json_output: bool = False,
    registry_path: str | None = None,
) -> dict:
    try:
        result = parse_file(str(path))
    except ValueError as error:
        raise SystemExit(str(error)) from error
    report = validate_result(result, registry_path)
    if not is_valid(report):
        _render_report(report, json_output=json_output, mode="validate")
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
    validate_parser.add_argument("--registry", type=Path, default=None)
    validate_parser.add_argument("--json", action="store_true")

    analyze_parser = subparsers.add_parser("analyze", help="Analyze semantics and expressions")
    analyze_parser.add_argument("path", type=Path)
    analyze_parser.add_argument("--registry", type=Path, default=None)
    analyze_parser.add_argument("--json", action="store_true")

    plan_parser = subparsers.add_parser("plan", help="Lower a contract to a transformation plan")
    plan_parser.add_argument("path", type=Path)
    plan_parser.add_argument("--registry", type=Path, default=None)
    plan_parser.add_argument("--json", action="store_true")

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
    compat_parser.add_argument("--registry", type=Path, default=None)
    # Accept both comma-delimited and repeated flags: `--scope a,b` or `--scope a --scope b`.
    compat_parser.add_argument("--scope", action="append", default=[])
    compat_parser.add_argument("--json", action="store_true")

    evolve_parser = subparsers.add_parser("evolve", help="Analyze contract evolution")
    evolve_parser.add_argument("older", type=Path)
    evolve_parser.add_argument("newer", type=Path)
    evolve_parser.add_argument("--registry", type=Path, default=None)
    evolve_parser.add_argument("--json", action="store_true")

    lineage_parser = subparsers.add_parser("lineage", help="Analyze contract lineage")
    lineage_parser.add_argument("path", type=Path)
    lineage_parser.add_argument("--registry", type=Path, default=None)
    lineage_parser.add_argument("--impact", default=None)
    lineage_parser.add_argument("--dependency", default=None)
    lineage_parser.add_argument("--json", action="store_true")

    version_parser = subparsers.add_parser("version", help="Print package versions")
    version_parser.add_argument("--json", action="store_true")

    registry_parser = subparsers.add_parser("registry", help="Inspect the identifier registry")
    registry_sub = registry_parser.add_subparsers(dest="registry_command", required=True)

    registry_list_parser = registry_sub.add_parser("list", help="List registry entries")
    registry_list_parser.add_argument("--registry", type=Path, default=None)
    registry_list_parser.add_argument("--json", action="store_true")

    registry_resolve_parser = registry_sub.add_parser("resolve", help="Resolve a registry identifier")
    registry_resolve_parser.add_argument("id")
    registry_resolve_parser.add_argument("--registry", type=Path, default=None)
    registry_resolve_parser.add_argument("--json", action="store_true")

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

    if args.command == "registry":
        registry_path = str(args.registry) if args.registry else None
        if args.registry_command == "list":
            try:
                entries = registry_list(registry_path)
            except ValueError as error:
                print(str(error), file=sys.stderr)
                return 1
            if args.json:
                print(json.dumps(entries, indent=2))
            else:
                for entry in entries:
                    print(
                        f"{entry.get('id')}  [{entry.get('category')}]  "
                        f"{entry.get('name')}  ({entry.get('status')})"
                    )
            return 0
        if args.registry_command == "resolve":
            try:
                entry = registry_resolve(args.id, registry_path)
            except ValueError as error:
                print(str(error), file=sys.stderr)
                return 1
            if entry is None:
                if args.json:
                    print("null")
                else:
                    print(f"unresolved registry entry: {args.id}", file=sys.stderr)
                return 1
            if args.json:
                print(json.dumps(entry, indent=2))
            else:
                print(f"id: {entry.get('id')}")
                print(f"name: {entry.get('name')}")
                print(f"category: {entry.get('category')}")
                print(f"version: {entry.get('version')}")
                print(f"status: {entry.get('status')}")
                if definition := entry.get("definition"):
                    print(f"definition: {definition}")
                if compatibility := entry.get("compatibility"):
                    print(f"compatibility: {compatibility}")
                print(f"supported: {entry.get('supported')}")
            return 0

    if args.command == "analyze":
        registry_path = str(args.registry) if args.registry else None
        contract = _load_valid_contract(
            args.path,
            json_output=args.json,
            registry_path=registry_path,
        )
        try:
            result = analyze(contract, registry_path)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if args.json:
            print(json.dumps(result, indent=2))
        else:
            validation = result.get("validation", {})
            analysis_report = result.get("analysis", {})
            if validation.get("diagnostics"):
                print("validation diagnostics:")
                _render_report(validation, json_output=False, mode="diagnostics")
            if analysis_report.get("diagnostics"):
                print("analysis diagnostics:")
                _render_report(analysis_report, json_output=False, mode="diagnostics")
            else:
                print("no analysis diagnostics")
        valid = is_valid(result.get("validation", {})) and is_valid(result.get("analysis", {}))
        return 0 if valid else 1

    if args.command == "plan":
        registry_path = str(args.registry) if args.registry else None
        contract = _load_valid_contract(
            args.path,
            json_output=args.json,
            registry_path=registry_path,
        )
        try:
            result = plan_lower(contract, registry_path)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if not is_valid({"diagnostics": result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": result.get("diagnostics", [])},
                json_output=args.json,
                mode="diagnostics",
            )
            return 1
        plan = result.get("plan")
        if args.json:
            print(json.dumps(plan, indent=2))
        else:
            print(f"plan: {plan.get('identity', {}).get('id', '')}")
            print(f"nodes: {len(plan.get('nodes', []))}")
            print(f"dependencies: {len(plan.get('dependencies', []))}")
        return 0

    if args.command == "compat":
        registry_path = str(args.registry) if args.registry else None
        source = _load_valid_contract(
            args.source,
            json_output=args.json,
            registry_path=registry_path,
        )
        target = _load_valid_contract(
            args.target,
            json_output=args.json,
            registry_path=registry_path,
        )
        scope_tokens: list[str] = []
        for item in args.scope or []:
            scope_tokens.extend(part.strip() for part in str(item).split(","))
        scope_tokens = [token for token in scope_tokens if token]
        scope = scope_tokens or None
        try:
            report = compat_analyze(source, target, scope)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 2
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            print(f"compatibility: {report.get('level')}")
            for aspect in report.get("aspects", []) or []:
                print(f"  {aspect.get('aspect')}: {aspect.get('message')}")
            for diagnostic in report.get("diagnostics", []) or []:
                severity = diagnostic.get("severity", "error")
                code = diagnostic.get("id", "dtcs:unknown")
                message = diagnostic.get("message", "")
                print(f"[{severity}] {code} - {message}")
        level = report.get("level")
        return 0 if level and level != "incompatible" else 1

    if args.command == "evolve":
        registry_path = str(args.registry) if args.registry else None
        older = _load_valid_contract(
            args.older,
            json_output=args.json,
            registry_path=registry_path,
        )
        newer = _load_valid_contract(
            args.newer,
            json_output=args.json,
            registry_path=registry_path,
        )
        report = evolve_analyze(older, newer)
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            print(
                f"evolution: {report.get('compatibility')} "
                f"(same identity: {report.get('sameIdentity')})"
            )
            for change in report.get("changes", []) or []:
                print(f"  [{change.get('category')}] {change.get('message')}")
            for hint in report.get("migrationHints", []) or []:
                print(f"  hint: {hint}")
        return 0 if report.get("sameIdentity") and report.get("compatibility") != "incompatible" else 1

    if args.command == "lineage":
        registry_path = str(args.registry) if args.registry else None
        contract = _load_valid_contract(
            args.path,
            json_output=args.json,
            registry_path=registry_path,
        )
        report = lineage_analyze(contract, args.impact, args.dependency)
        if args.json:
            print(json.dumps(report, indent=2))
        else:
            for edge in report.get("graph", []):
                print(f"{edge['output']} <- {edge['inputs']}")
            if impact := report.get("impact"):
                print(f"impact {impact['input']} -> {impact['outputs']}")
            if dependency := report.get("dependency"):
                print(f"dependency {dependency['output']} <- {dependency['inputs']}")
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
    registry_path = str(args.registry) if getattr(args, "registry", None) else None
    report = validate_result(result, registry_path)

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
