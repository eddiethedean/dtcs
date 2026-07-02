# DTCS Editorial & Style Guide

Version: 1.0 Draft

# Purpose

This guide defines the editorial, structural, and normative conventions
used throughout the DTCS specification. All chapters SHOULD follow this
guide to produce a consistent, publication-quality standard.

# Guiding Principles

-   Prefer precision over brevity.
-   Define semantics, not implementations.
-   Separate normative and informative content.
-   Avoid ambiguity.
-   Keep terminology consistent across chapters.

# Normative Language

Use RFC 2119 / RFC 8174 keywords only in ALL CAPS.

-   MUST
-   MUST NOT
-   SHALL
-   SHALL NOT
-   SHOULD
-   SHOULD NOT
-   MAY
-   OPTIONAL
-   RECOMMENDED

Do not mix lowercase variants into normative statements.

# Normative vs Informative

Normative content:

- Requirements
- Definitions
- Conformance
- Mandatory behavior

Informative content:

- Examples
- Notes
- Diagrams
- Rationale
- Design discussion

Clearly label informative notes where appropriate.

# Terminology

Use one canonical term throughout the specification.

Preferred terms include:

-   Transformation Contract
-   Transformation Plan
-   Execution Plan
-   Semantic Action
-   Expression
-   Function
-   Rule
-   Runtime
-   Engine
-   Planner
-   Compiler
-   Validator
-   Analyzer

Avoid introducing synonyms unless explicitly defined.

# Chapter Structure

Every chapter SHOULD follow this template:

1.  Purpose
2.  Design Goals
3.  Core Concepts
4.  Normative Requirements
5.  Conformance (if applicable)
6.  Summary

# Writing Style

-   Use active voice.
-   Prefer short declarative sentences.
-   One concept per paragraph.
-   Define terms before using them.
-   Avoid implementation-specific examples in normative text.

# Examples

Examples SHOULD:

-   Be concise.
-   Be clearly marked as informative.
-   Illustrate semantics rather than implementation.
-   Avoid implying required syntax.

# Diagrams

Prefer simple ASCII diagrams.

Keep diagrams conceptual rather than implementation-specific.

# Consistency Rules

-   "Transformation Plan" always capitalized.
-   "Execution Plan" always capitalized.
-   "Semantic Action" always capitalized when referring to the DTCS
    concept.
-   Namespace examples use `dtcs:` for standard identifiers.

# Object Model

The canonical Object Model is authoritative.

Documents, YAML, JSON, and other formats are representations only.

# Architecture

Always describe the architecture in this order:

Transformation Contract → Object Model → Transformation Plan → Execution
Plan → Runtime

# Numbering

-   Chapters: integer
-   Sections: x.y
-   Lists: ordered only when sequence matters

# Code Blocks

Code blocks are illustrative unless explicitly stated otherwise.

# Cross References

Reference chapters and sections by number where practical.

# Glossary

New technical terms SHOULD be added to the glossary and used
consistently thereafter.

# Future Evolution

New chapters and revisions SHOULD preserve existing terminology,
identifiers, and architectural principles whenever practical.
