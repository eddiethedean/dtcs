# DTCS Specification Authoring Guide

Version: 1.0 Draft

## 1. Purpose

This guide defines the editorial, structural, and normative conventions
for all DTCS specifications and companion standards. Its goal is to
ensure that every document reads as a coherent standards publication
regardless of author.

## 2. Guiding Principles

-   Semantics over implementation.
-   Contracts over code.
-   Deterministic wording.
-   Stable terminology.
-   Vendor neutrality.
-   Backward compatibility whenever practical.

## 3. Normative Language

Use RFC 2119 / RFC 8174 keywords only in ALL CAPS.

Preferred keywords:

- SHALL / SHALL NOT
- SHOULD / SHOULD NOT
- MAY
- MUST / MUST NOT (reserved for absolute requirements when appropriate)
- OPTIONAL
- RECOMMENDED

Avoid lowercase forms for normative requirements.

## 4. Normative vs. Informative Content

Normative:

- Requirements
- Definitions
- Conformance
- Registries
- Required behavior

Informative:

- Notes
- Examples
- Rationale
- Design discussion
- Diagrams

Never mix normative requirements into informative notes.

## 5. Controlled Vocabulary

Canonical terms:

- Transformation Contract
- Canonical Object Model
- Transformation Plan
- Execution Plan
- Semantic Action
- Expression
- Function
- Rule
- Runtime
- Engine
- Planner
- Optimizer
- Compiler
- Validator
- Analyzer
- Diagnostic

Avoid introducing synonyms without defining them.

## 6. Capitalization

Capitalize canonical DTCS concepts exactly as defined.

Examples: - Transformation Plan - Execution Plan - Semantic Action

Use lower case for generic concepts: - transformation - function -
compiler (when not referring to the DTCS component)

## 7. Chapter Template

Each chapter SHOULD follow:

1.  Purpose
2.  Design Goals
3.  Core Concepts
4.  Normative Requirements
5.  Conformance (if applicable)
6.  Summary

## 8. Section Numbering

Use decimal numbering:

-   Chapter
-   x.y section
-   x.y.z subsection only when necessary

## 9. Writing Style

-   Active voice.
-   One concept per paragraph.
-   Define before use.
-   Avoid ambiguous pronouns.
-   Prefer short declarative sentences.

## 10. Examples

Examples SHALL:

- Be marked as informative.
- Demonstrate semantics.
- Avoid implying implementation requirements.

## 11. Diagrams

Prefer ASCII diagrams.

Show conceptual architecture instead of implementation details.

## 12. Code Blocks

Code blocks are illustrative unless explicitly declared normative.

## 13. Cross References

Reference chapters and sections numerically.

Do not duplicate normative definitions across chapters.

## 14. Identifier Conventions

Standard identifiers: - dtcs:\*

Vendor identifiers: - vendor:* - company:*

Identifiers MUST be stable.

## 15. Registry Templates

Every registry entry SHOULD include:

-   Identifier
-   Purpose
-   Parameters
-   Type Rules
-   Null Behavior
-   Lineage
-   Compatibility
-   Examples

## 16. Diagnostics

Diagnostics should include:

-   Identifier
-   Severity
-   Stage
-   Message
-   Suggested remediation

## 17. YAML and JSON

YAML is the preferred authoring format.

JSON is the preferred interchange format.

The Object Model is canonical.

## 18. Diagrams

Maintain the canonical architecture:

Transformation Contract → Canonical Object Model → Transformation Plan →
Execution Plan → Runtime

## 19. Conformance Wording

Use: - SHALL for mandatory behavior. - SHOULD for recommendations. - MAY
for optional behavior.

## 20. Editorial Rules

Avoid: - "simply" - "just" - "obviously" - implementation-specific
language in normative sections.

## 21. Future Evolution

Add new concepts without redefining existing semantics.

Preserve identifiers whenever practical.

## 22. Glossary Maintenance

Every new normative term SHOULD be added to the glossary.

## 23. Appendix Guidance

Appendices SHOULD contain: - references - rationale - historical notes -
migration guidance

Appendices are informative unless explicitly stated.

## 24. Quality Checklist

Before publication verify:

-   Consistent terminology
-   Stable identifiers
-   Correct RFC keyword usage
-   No duplicated definitions
-   Clear normative/informative separation
-   Cross references validated
-   Examples labeled
-   Diagrams consistent
-   Conformance language present
-   Summary included
