from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import pytest

import dtcs

PACKAGE_ROOT = Path(__file__).resolve().parent
REPO_ROOT = Path(__file__).resolve().parents[2]
FIXTURES = REPO_ROOT / "tests" / "fixtures"
EXAMPLE = REPO_ROOT / "examples" / "customer_normalize.dtcs.yaml"
MANIFEST = REPO_ROOT / "tests" / "fixture_expectations.json"


def _fixture_dir() -> Path:
    return FIXTURES


def _fixture(name: str) -> bytes:
    return _fixture_dir().joinpath(name).read_bytes()


def _fixture_format(name: str) -> str:
    return "json" if name.endswith(".json") else "yaml"


def _load_manifest() -> list[dict]:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))["fixtures"]


def test_spec_version() -> None:
    assert dtcs.SPEC_VERSION.endswith("draft")
    assert dtcs.__version__


def test_parse_valid_yaml_fixture() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yaml")
    assert dtcs.is_valid(result["report"])
    contract = result["contract"]
    assert contract is not None
    assert contract["id"] == "customer.normalize"


def test_parse_valid_json_fixture() -> None:
    result = dtcs.parse(_fixture("valid_minimal.json"), "json")
    assert dtcs.is_valid(result["report"])
    contract = result["contract"]
    assert contract is not None
    assert contract["id"] == "json.example"


def test_parse_and_validate_repo_example() -> None:
    content = EXAMPLE.read_bytes()
    report = dtcs.parse_and_validate(content, "yaml")
    assert dtcs.is_valid(report)


def test_parse_file_repo_example() -> None:
    result = dtcs.parse_file(str(EXAMPLE))
    assert dtcs.is_valid(result["report"])
    contract = result["contract"]
    assert contract is not None
    assert contract["id"] == "customer.normalize"


def test_validate_result_merges_parse_and_validation_diagnostics() -> None:
    result = dtcs.parse(_fixture("missing_lineage.yaml"), "yaml")
    report = dtcs.validate_result(result)
    assert not dtcs.is_valid(report)
    ids = {diagnostic["id"] for diagnostic in report["diagnostics"]}
    assert "dtcs:missing-lineage" in ids


def test_validate_contract_round_trip() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yaml")
    report = dtcs.validate(result["contract"])
    assert dtcs.is_valid(report)


def test_validate_none_contract_raises() -> None:
    with pytest.raises(TypeError, match="contract must be a dict"):
        dtcs.validate(None)


def test_parse_malformed_yaml_has_no_contract() -> None:
    result = dtcs.parse(_fixture("malformed.yaml"), "yaml")
    assert result["contract"] is None
    assert any(d["id"] == "dtcs:parse-error" for d in result["report"]["diagnostics"])


def test_parse_malformed_json_has_no_contract() -> None:
    result = dtcs.parse(_fixture("malformed.json"), "json")
    assert result["contract"] is None
    assert any(d["id"] == "dtcs:parse-error" for d in result["report"]["diagnostics"])


def test_parse_yml_format_alias() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yml")
    assert dtcs.is_valid(result["report"])


def test_parse_bytearray_content() -> None:
    result = dtcs.parse(bytearray(_fixture("valid_customer.yaml")), "yaml")
    assert dtcs.is_valid(result["report"])


def test_inspect_summary() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yaml")
    summary = dtcs.inspect(result["contract"])
    assert "customer.normalize" in summary
    assert "inputs:" in summary


def test_metadata_validate_matches_full_validate() -> None:
    result = dtcs.parse(_fixture("invalid_metadata_timestamp.yaml"), "yaml")
    contract = result["contract"]
    metadata_report = dtcs.metadata_validate(contract)
    full_report = dtcs.validate(contract)
    assert not dtcs.is_valid(metadata_report)
    assert any(d["id"] == "dtcs:invalid-metadata" for d in metadata_report["diagnostics"])
    assert any(d["id"] == "dtcs:invalid-metadata" for d in full_report["diagnostics"])


def test_metadata_validate_is_subset_of_metadata_codes() -> None:
    result = dtcs.parse(_fixture("missing_lineage.yaml"), "yaml")
    contract = result["contract"]
    metadata_report = dtcs.metadata_validate(contract)
    full_report = dtcs.validate(contract)
    metadata_ids = {d["id"] for d in metadata_report["diagnostics"]}
    full_ids = {d["id"] for d in full_report["diagnostics"]}
    assert metadata_ids.issubset(full_ids)
    assert "dtcs:missing-lineage" in full_ids


def test_preserves_extension_fields() -> None:
    yaml = b"""
dtcsVersion: "1.0.0"
id: "ext.example"
name: "Extension Example"
version: "0.1.0"
acme:featureFlag: true
inputs:
  - id: "in"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
outputs:
  - id: "out"
    schema:
      fields:
        - name: "value"
          type: "string"
          nullable: false
lineage:
  mappings:
    - output: "out"
      inputs: ["in"]
"""
    result = dtcs.parse(yaml, "yaml")
    contract = result["contract"]
    assert contract is not None
    assert "acme:featureFlag" in contract
    assert dtcs.is_valid(dtcs.validate(contract))


def test_diagnostics_are_deterministic() -> None:
    content = _fixture("invalid_type.yaml")
    first = dtcs.parse_and_validate(content, "yaml")
    second = dtcs.parse_and_validate(content, "yaml")
    assert first["diagnostics"] == second["diagnostics"]


@pytest.mark.parametrize("entry", _load_manifest(), ids=lambda entry: entry["file"])
def test_fixture_expectations(entry: dict) -> None:
    name = entry["file"]
    content = _fixture(name)
    doc_format = _fixture_format(name)
    result = dtcs.parse(content, doc_format)
    assert dtcs.is_valid(result["report"]) is entry["parse_valid"]
    assert (result["contract"] is not None) is entry["contract"]
    if result["contract"] is not None:
        report = dtcs.validate_result(result)
    else:
        report = result["report"]
    assert dtcs.is_valid(report) is entry["validate_valid"]
    if codes := entry.get("codes"):
        ids = {diagnostic["id"] for diagnostic in report["diagnostics"]}
        for code in codes:
            assert code in ids


def _python_dtcs(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, "-m", "dtcs", *args],
        capture_output=True,
        text=True,
        check=False,
    )


def test_cli_validate_succeeds_on_example() -> None:
    output = _python_dtcs("validate", str(EXAMPLE))
    assert output.returncode == 0
    assert "valid" in output.stdout


def test_cli_validate_succeeds_on_phase_0_2_fixture() -> None:
    path = _fixture_dir() / "valid_metadata.yaml"
    output = _python_dtcs("validate", str(path))
    assert output.returncode == 0
    assert "valid" in output.stdout


def test_cli_validate_fails_on_invalid_contract() -> None:
    path = _fixture_dir() / "missing_lineage.yaml"
    output = _python_dtcs("validate", str(path))
    assert output.returncode != 0


def test_cli_inspect_fails_on_invalid_contract() -> None:
    path = _fixture_dir() / "unresolved_reference.yaml"
    output = _python_dtcs("inspect", str(path))
    assert output.returncode != 0


def test_cli_inspect_succeeds_on_valid_contract() -> None:
    path = _fixture_dir() / "valid_customer.yaml"
    output = _python_dtcs("inspect", str(path))
    assert output.returncode == 0
    assert "customer.normalize" in output.stdout


def test_cli_diagnostics_json_output() -> None:
    path = _fixture_dir() / "missing_lineage.yaml"
    output = _python_dtcs("diagnostics", "--json", str(path))
    assert output.returncode != 0
    payload = json.loads(output.stdout)
    assert payload["diagnostics"]


def test_cli_version_json_output() -> None:
    output = _python_dtcs("version", "--json")
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["crateVersion"] == dtcs.__version__
    assert payload["specVersion"] == dtcs.SPEC_VERSION


def test_unsupported_format_raises() -> None:
    with pytest.raises(ValueError, match="unsupported format"):
        dtcs.parse(_fixture("valid_customer.yaml"), "xml")


def test_parse_file_missing_path_raises() -> None:
    with pytest.raises(ValueError):
        dtcs.parse_file("/tmp/does-not-exist-dtcs-fixture.yaml")


def test_cli_missing_file_exits_cleanly() -> None:
    output = _python_dtcs("validate", "/tmp/does-not-exist-dtcs-fixture.yaml")
    assert output.returncode == 1
    assert "traceback" not in output.stderr.lower()
    assert output.stderr.strip()


def test_cli_validate_json_output() -> None:
    output = _python_dtcs("validate", "--json", str(EXAMPLE))
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["valid"] is True
    assert isinstance(payload["diagnostics"], list)


def test_cli_inspect_json_output() -> None:
    output = _python_dtcs("inspect", "--json", str(EXAMPLE))
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["id"] == "customer.normalize"
    assert payload["inputs"] >= 1


def test_compat_analyze_backward_compatible() -> None:
    old = _fixture_dir() / "compatibility" / "backward_old.yaml"
    new = _fixture_dir() / "compatibility" / "backward_new.yaml"
    source = dtcs.parse_file(str(old))["contract"]
    target = dtcs.parse_file(str(new))["contract"]
    report = dtcs.compat_analyze(source, target)
    assert report["level"] == "backwardCompatible"


def test_cli_compat_json_output() -> None:
    old = _fixture_dir() / "compatibility" / "backward_old.yaml"
    new = _fixture_dir() / "compatibility" / "backward_new.yaml"
    output = _python_dtcs("compat", "--json", str(old), str(new))
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["level"] == "backwardCompatible"


def test_cli_evolve_json_output() -> None:
    rev1 = _fixture_dir() / "compatibility" / "evolution" / "rev1.yaml"
    rev2 = _fixture_dir() / "compatibility" / "evolution" / "rev2.yaml"
    output = _python_dtcs("evolve", "--json", str(rev1), str(rev2))
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["sameIdentity"] is True


def test_cli_lineage_json_output() -> None:
    path = _fixture_dir() / "lineage_multi.yaml"
    output = _python_dtcs("lineage", "--json", "--impact", "customers", str(path))
    assert output.returncode == 0
    payload = json.loads(output.stdout)
    assert payload["impact"]["outputs"]


def test_diagnostics_json_uses_camel_case_object_ref() -> None:
    path = _fixture_dir() / "missing_lineage.yaml"
    output = _python_dtcs("diagnostics", "--json", str(path))
    payload = json.loads(output.stdout)
    diagnostic = payload["diagnostics"][0]
    assert "objectRef" in diagnostic or diagnostic.get("objectRef") is None
    assert "object_ref" not in diagnostic


def test_is_valid_treats_missing_severity_as_error() -> None:
    report = {"diagnostics": [{"id": "dtcs:unknown", "message": "boom"}]}
    assert not dtcs.is_valid(report)


def test_registry_resolve_builtin_action() -> None:
    entry = dtcs.registry_resolve("dtcs:lowercase")
    assert entry is not None
    assert entry["id"] == "dtcs:lowercase"
    assert entry["category"] == "semanticAction"


def test_registry_list_includes_standard_entries() -> None:
    entries = dtcs.registry_list()
    ids = {entry["id"] for entry in entries}
    assert "dtcs:lowercase" in ids
    assert "dtcs:not_null" in ids
    assert "dtcs:parse-error" in ids


def test_registry_resolve_missing_returns_none() -> None:
    assert dtcs.registry_resolve("dtcs:does-not-exist") is None


def test_registry_load_vendor_catalog() -> None:
    path = _fixture_dir() / "registry" / "vendor_catalog.yaml"
    catalog = dtcs.registry_load(str(path))
    assert catalog["id"] == "vendor.catalog"
    assert "acme:transform" in catalog["entries"]


def test_cli_registry_list_and_resolve() -> None:
    listed = _python_dtcs("registry", "list", "--json")
    assert listed.returncode == 0
    entries = json.loads(listed.stdout)
    assert any(entry["id"] == "dtcs:lowercase" for entry in entries)

    resolved = _python_dtcs("registry", "resolve", "dtcs:lowercase", "--json")
    assert resolved.returncode == 0
    entry = json.loads(resolved.stdout)
    assert entry["id"] == "dtcs:lowercase"

    missing = _python_dtcs("registry", "resolve", "dtcs:does-not-exist")
    assert missing.returncode == 1


def test_compat_analyze_rejects_invalid_scope() -> None:
    source = dtcs.parse_file(str(_fixture_dir() / "compatibility/backward_old.yaml"))["contract"]
    target = dtcs.parse_file(str(_fixture_dir() / "compatibility/backward_new.yaml"))["contract"]
    with pytest.raises(ValueError, match="invalid scope"):
        dtcs.compat_analyze(source, target, ["not-a-scope"])


def test_cli_compat_rejects_invalid_scope() -> None:
    old = _fixture_dir() / "compatibility" / "backward_old.yaml"
    new = _fixture_dir() / "compatibility" / "backward_new.yaml"
    output = _python_dtcs("compat", "--scope", "not-a-scope", str(old), str(new))
    assert output.returncode == 2


def test_contract_from_py_rejects_nan() -> None:
    contract = {
        "dtcsVersion": "1.0.0",
        "id": "nan.example",
        "name": "NaN Example",
        "version": "0.1.0",
        "inputs": [
            {
                "id": "in",
                "schema": {
                    "fields": [
                        {"name": "value", "type": "integer", "nullable": False},
                    ]
                },
            }
        ],
        "outputs": [
            {
                "id": "out",
                "schema": {
                    "fields": [
                        {"name": "value", "type": "integer", "nullable": False},
                    ]
                },
            }
        ],
        "lineage": {"mappings": [{"output": "out", "inputs": ["in"]}]},
        "acme:ratio": float("nan"),
    }
    with pytest.raises(ValueError, match="non-finite"):
        dtcs.validate(contract)


def test_contract_from_py_preserves_non_nan_dump_errors() -> None:
    contract = {
        "dtcsVersion": "1.0.0",
        "id": "dump_error.example",
        "name": "Dump Error Example",
        "version": "0.1.0",
        "inputs": [
            {
                "id": "in",
                "schema": {
                    "fields": [
                        {"name": "value", "type": "integer", "nullable": False},
                    ]
                },
            }
        ],
        "outputs": [
            {
                "id": "out",
                "schema": {
                    "fields": [
                        {"name": "value", "type": "integer", "nullable": False},
                    ]
                },
            }
        ],
        "lineage": {"mappings": [{"output": "out", "inputs": ["in"]}]},
        "acme:notSerializable": {1, 2, 3},
    }
    with pytest.raises(TypeError, match="not JSON serializable"):
        dtcs.validate(contract)
