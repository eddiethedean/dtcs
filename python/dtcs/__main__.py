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
    capability_match,
    capability_reference_profile,
    compile_plan,
    compat_analyze,
    conformance_declare,
    conformance_run,
    evolve_analyze,
    inspect,
    is_valid,
    lineage_analyze,
    parse_file,
    plan_export_portable,
    plan_fingerprint,
    plan_lower,
    plan_optimize,
    plan_topological_order,
    registry_list,
    registry_resolve,
    runtime_execute,
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


REFERENCE_PROFILE = "dtcs:reference"


def _load_transformation_plan(
    path: Path,
    *,
    from_plan: bool = False,
    optimize: bool = False,
    registry_path: str | None = None,
) -> dict:
    if from_plan:
        try:
            plan = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            raise SystemExit(str(error)) from error
    else:
        contract = _load_valid_contract(path, registry_path=registry_path)
        try:
            lower_result = plan_lower(contract, registry_path)
        except ValueError as error:
            raise SystemExit(str(error)) from error
        if not is_valid({"diagnostics": lower_result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": lower_result.get("diagnostics", [])},
                json_output=False,
                mode="diagnostics",
            )
            raise SystemExit(f"plan lowering failed for {path}")
        plan = lower_result.get("plan")
        if plan is None:
            raise SystemExit(f"no plan produced for {path}")

    if optimize:
        try:
            optimize_result = plan_optimize(plan, registry_path)
        except ValueError as error:
            raise SystemExit(str(error)) from error
        if not is_valid({"diagnostics": optimize_result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": optimize_result.get("diagnostics", [])},
                json_output=False,
                mode="diagnostics",
            )
            raise SystemExit(f"plan optimization failed for {path}")
        plan = optimize_result.get("plan")
        if plan is None:
            raise SystemExit(f"no optimized plan produced for {path}")
    return plan


def _load_capability_profile(profile: str) -> dict:
    if profile == REFERENCE_PROFILE:
        return capability_reference_profile()
    raise SystemExit(f"unsupported capability profile '{profile}'")


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

    export_parser = subparsers.add_parser(
        "export-portable",
        help="Export a portable transformation plan (dtcs.transform-plan/2)",
    )
    export_parser.add_argument("path", type=Path)
    export_parser.add_argument("--registry", type=Path, default=None)
    export_parser.add_argument(
        "--profile",
        default="dtcs:profile/portable-relational-kernel/2",
        help="Portable profile identifier",
    )
    export_parser.add_argument(
        "--fingerprint",
        action="store_true",
        help="Emit only the semantic fingerprint",
    )

    optimize_parser = subparsers.add_parser(
        "optimize",
        help="Optimize a transformation plan",
    )
    optimize_parser.add_argument("path", type=Path)
    optimize_parser.add_argument(
        "--plan",
        action="store_true",
        help="Treat path as serialized plan JSON instead of a contract",
    )
    optimize_parser.add_argument("--registry", type=Path, default=None)
    optimize_parser.add_argument(
        "--no-validate",
        action="store_true",
        help="Skip validation of the optimized plan",
    )
    optimize_parser.add_argument("--json", action="store_true")

    match_parser = subparsers.add_parser(
        "match",
        help="Match a transformation plan against engine capabilities",
    )
    match_parser.add_argument("path", type=Path)
    match_parser.add_argument(
        "--plan",
        action="store_true",
        help="Treat path as serialized plan JSON instead of a contract",
    )
    match_parser.add_argument("--optimize", action="store_true")
    match_parser.add_argument("--registry", type=Path, default=None)
    match_parser.add_argument(
        "--profile",
        default=REFERENCE_PROFILE,
        help="Engine profile identifier (default: dtcs:reference)",
    )
    match_parser.add_argument("--json", action="store_true")

    compile_parser = subparsers.add_parser(
        "compile",
        help="Compile a transformation plan to an execution plan",
    )
    compile_parser.add_argument("path", type=Path)
    compile_parser.add_argument(
        "--plan",
        action="store_true",
        help="Treat path as serialized plan JSON instead of a contract",
    )
    compile_parser.add_argument("--optimize", action="store_true")
    compile_parser.add_argument("--registry", type=Path, default=None)
    compile_parser.add_argument(
        "--profile",
        default=REFERENCE_PROFILE,
        help="Engine profile identifier (default: dtcs:reference)",
    )
    compile_parser.add_argument("--json", action="store_true")

    run_parser = subparsers.add_parser(
        "run",
        help="Execute a contract using the reference runtime",
    )
    run_parser.add_argument("path", type=Path)
    run_parser.add_argument(
        "--input",
        type=Path,
        required=True,
        help="JSON file with runtime inputs keyed by interface id",
    )
    run_parser.add_argument("--optimize", action="store_true")
    run_parser.add_argument("--registry", type=Path, default=None)
    run_parser.add_argument("--json", action="store_true")

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

    conformance_parser = subparsers.add_parser(
        "conformance",
        help="Conformance profiles and offline certification suite",
    )
    conformance_sub = conformance_parser.add_subparsers(dest="conformance_command", required=True)

    conformance_declare_parser = conformance_sub.add_parser(
        "declare",
        help="Emit implementation capability declaration",
    )
    conformance_declare_parser.add_argument("--profile", default=None)
    conformance_declare_parser.add_argument("--json", action="store_true")

    conformance_run_parser = conformance_sub.add_parser(
        "run",
        help="Run offline conformance tests",
    )
    conformance_run_parser.add_argument(
        "--profile",
        default="integrated-platform",
        help="Profile id or 'all'",
    )
    conformance_run_parser.add_argument("--json", action="store_true")

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

    if args.command == "conformance":
        if args.conformance_command == "declare":
            try:
                declaration = conformance_declare(args.profile)
            except ValueError as error:
                print(str(error), file=sys.stderr)
                return 1
            if args.json:
                print(json.dumps(declaration, indent=2))
            else:
                print(f"implementation: {declaration.get('implementationId')}")
                print(f"version: {declaration.get('implementationVersion')}")
                print(f"spec: {declaration.get('dtcsVersion')}")
                print(f"primary profile: {declaration.get('primaryProfile')}")
                for profile in declaration.get("profiles", []):
                    print(
                        f"  {profile.get('id')} ({profile.get('implementationClass')})"
                    )
            return 0
        if args.conformance_command == "run":
            profile = args.profile
            try:
                report = conformance_run(None if profile == "all" else profile)
            except ValueError as error:
                print(str(error), file=sys.stderr)
                return 1
            if args.json:
                print(json.dumps(report, indent=2))
            else:
                status = "passed" if report.get("passed") else "failed"
                print(
                    f"conformance {status} ({report.get('implementationVersion', '')})"
                )
                for result in (report.get("results") or []) + (report.get("security") or []):
                    if not result.get("passed"):
                        print(
                            f"  FAIL {result.get('id')} [{result.get('profile')}]: "
                            f"{result.get('message', 'failed')}"
                        )
            return 0 if report.get("passed") else 1

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
            order = plan_topological_order(contract, plan)
            if order:
                print(f"order: {' -> '.join(order)}")
        return 0

    if args.command == "export-portable":
        registry_path = str(args.registry) if args.registry else None
        contract = _load_valid_contract(
            args.path,
            json_output=True,
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
                json_output=True,
                mode="diagnostics",
            )
            return 1
        plan = result.get("plan")
        if plan is None:
            print("plan lowering succeeded without a plan", file=sys.stderr)
            return 1
        try:
            portable = plan_export_portable(plan, args.profile)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if args.fingerprint:
            print(plan_fingerprint(portable))
        else:
            print(json.dumps(portable, indent=2))
        return 0

    if args.command == "optimize":
        registry_path = str(args.registry) if args.registry else None
        if args.plan:
            try:
                plan = json.loads(args.path.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError) as error:
                print(str(error), file=sys.stderr)
                return 1
        else:
            contract = _load_valid_contract(
                args.path,
                json_output=args.json,
                registry_path=registry_path,
            )
            try:
                lower_result = plan_lower(contract, registry_path)
            except ValueError as error:
                print(str(error), file=sys.stderr)
                return 1
            if not is_valid({"diagnostics": lower_result.get("diagnostics", [])}):
                _render_report(
                    {"diagnostics": lower_result.get("diagnostics", [])},
                    json_output=args.json,
                    mode="diagnostics",
                )
                return 1
            plan = lower_result.get("plan")
            if plan is None:
                print("no plan produced", file=sys.stderr)
                return 1
        try:
            result = plan_optimize(
                plan,
                registry_path,
                validate=not args.no_validate,
            )
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
        if args.json:
            print(json.dumps(result, indent=2))
        else:
            optimized = result.get("plan") or {}
            transforms = result.get("transforms") or []
            print(f"plan: {optimized.get('identity', {}).get('id', '')}")
            print(f"nodes: {len(optimized.get('nodes', []))}")
            print(f"transforms: {len(transforms)}")
        return 0

    if args.command == "match":
        registry_path = str(args.registry) if args.registry else None
        plan = _load_transformation_plan(
            args.path,
            from_plan=args.plan,
            optimize=args.optimize,
            registry_path=registry_path,
        )
        profile = _load_capability_profile(args.profile)
        try:
            report = capability_match(plan, profile)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        supported = report.get("supported", False)
        if args.json:
            print(json.dumps(report, indent=2))
        elif supported:
            print(f"supported: {plan.get('identity', {}).get('id', '')}")
            print(f"engine: {profile.get('engineId', '')}")
        else:
            _render_report(
                {"diagnostics": report.get("diagnostics", [])},
                json_output=False,
                mode="diagnostics",
            )
        return 0 if supported else 1

    if args.command == "compile":
        registry_path = str(args.registry) if args.registry else None
        plan = _load_transformation_plan(
            args.path,
            from_plan=args.plan,
            optimize=args.optimize,
            registry_path=registry_path,
        )
        profile = _load_capability_profile(args.profile)
        try:
            result = capability_match(plan, profile)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if not result.get("supported", False):
            _render_report(
                {"diagnostics": result.get("diagnostics", [])},
                json_output=args.json,
                mode="diagnostics",
            )
            return 1
        try:
            compile_result = compile_plan(plan)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if not is_valid({"diagnostics": compile_result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": compile_result.get("diagnostics", [])},
                json_output=args.json,
                mode="diagnostics",
            )
            return 1
        execution_plan = compile_result.get("plan")
        if execution_plan is None:
            print("no execution plan produced", file=sys.stderr)
            return 1
        if args.json:
            print(json.dumps(execution_plan, indent=2))
        else:
            print(f"execution plan: {execution_plan.get('identity', {}).get('id', '')}")
            print(f"target: {execution_plan.get('target', {}).get('engineId', '')}")
            print(f"steps: {len(execution_plan.get('steps', []))}")
        return 0

    if args.command == "run":
        registry_path = str(args.registry) if args.registry else None
        plan = _load_transformation_plan(
            args.path,
            from_plan=False,
            optimize=args.optimize,
            registry_path=registry_path,
        )
        try:
            compile_result = compile_plan(plan)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if not is_valid({"diagnostics": compile_result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": compile_result.get("diagnostics", [])},
                json_output=args.json,
                mode="diagnostics",
            )
            return 1
        execution_plan = compile_result.get("plan")
        if execution_plan is None:
            print("no execution plan produced", file=sys.stderr)
            return 1
        try:
            inputs = json.loads(args.input.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            print(str(error), file=sys.stderr)
            return 1
        try:
            execute_result = runtime_execute(execution_plan, inputs)
        except ValueError as error:
            print(str(error), file=sys.stderr)
            return 1
        if not is_valid({"diagnostics": execute_result.get("diagnostics", [])}):
            _render_report(
                {"diagnostics": execute_result.get("diagnostics", [])},
                json_output=args.json,
                mode="diagnostics",
            )
            return 1
        outputs = execute_result.get("outputs")
        if args.json:
            print(json.dumps(outputs, indent=2))
        else:
            for interface_id, dataset in (outputs or {}).items():
                print(f"{interface_id}: {len(dataset)} row(s)")
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
