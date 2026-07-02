# Testing Plan

Tests should be written against `SPEC.md`.

Required test categories:

- Parse valid YAML
- Parse valid JSON
- Reject malformed documents
- Reject missing required fields
- Reject duplicate identifiers
- Validate logical types
- Validate Semantic Action references
- Validate Function references
- Validate Rule references
- Preserve extensions
- Generate deterministic diagnostics

Add snapshot tests for diagnostics where useful.
