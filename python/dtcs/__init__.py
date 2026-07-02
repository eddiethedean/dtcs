"""Reference implementation of the Data Transformation Contract Standard (DTCS)."""

from __future__ import annotations

from dtcs._native import inspect as _inspect
from dtcs._native import parse_document as _parse_document
from dtcs._native import parse_path as _parse_path
from dtcs._native import spec_version as _spec_version
from dtcs._native import validate_contract as _validate_contract
from dtcs._native import validate_document as _validate_document

SPEC_VERSION = _spec_version()


def parse(content: str | bytes, format: str = "yaml") -> dict:
    """Parse a DTCS document from YAML or JSON text."""
    return _parse_document(content, format)


def parse_file(path: str) -> dict:
    """Parse a DTCS document from a file path."""
    return _parse_path(path)


def validate(contract: dict) -> dict:
    """Validate a parsed transformation contract."""
    return _validate_contract(contract)


def parse_and_validate(content: str | bytes, format: str = "yaml") -> dict:
    """Parse and validate a DTCS document in one step."""
    return _validate_document(content, format)


def inspect(contract: dict) -> str:
    """Return a short human-readable contract summary."""
    return _inspect(contract)


def is_valid(report: dict) -> bool:
    """Return True when a diagnostic report contains no error-level diagnostics."""
    return not any(
        diagnostic.get("severity") == "error"
        for diagnostic in report.get("diagnostics", [])
    )


__all__ = [
    "SPEC_VERSION",
    "inspect",
    "is_valid",
    "parse",
    "parse_and_validate",
    "parse_file",
    "validate",
]
