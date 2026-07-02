from __future__ import annotations

from pathlib import Path

import dtcs

FIXTURES = Path(__file__).resolve().parents[2] / "tests" / "fixtures"
EXAMPLE = Path(__file__).resolve().parents[2] / "examples" / "customer_normalize.dtcs.yaml"


def _fixture(name: str) -> bytes:
    return FIXTURES.joinpath(name).read_bytes()


def test_spec_version() -> None:
    assert dtcs.SPEC_VERSION == "1.0.0-draft"


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


def test_validate_contract_round_trip() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yaml")
    report = dtcs.validate(result["contract"])
    assert dtcs.is_valid(report)


def test_invalid_fixture_reports_error() -> None:
    report = dtcs.parse_and_validate(_fixture("missing_lineage.yaml"), "yaml")
    assert not dtcs.is_valid(report)
    assert any(d["severity"] == "error" for d in report["diagnostics"])


def test_inspect_summary() -> None:
    result = dtcs.parse(_fixture("valid_customer.yaml"), "yaml")
    summary = dtcs.inspect(result["contract"])
    assert "customer.normalize" in summary
    assert "inputs:" in summary
