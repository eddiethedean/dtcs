# DTCS Editorial Baseline

**Version:** 1.0

## Status

Normative for all DTCS specifications and companion standards unless
superseded by a later editorial baseline.

## 1. Purpose

The DTCS Editorial Baseline establishes the mandatory editorial
conventions for authoring normative DTCS specifications. Its purpose is
to ensure consistency, clarity, interoperability, and
publication-quality documentation across the DTCS ecosystem.

## 2. Scope

This baseline applies to:

-   DTCS Specification
-   DPCS (Data Pipeline Contract Standard)
-   DEL (Data Expression Language)
-   Companion specifications
-   Registry specifications
-   Conformance specifications
-   Reference implementation documentation (where applicable)

## 3. Editorial Principles

Every specification SHALL:

-   Use a consistent standards-document structure.
-   Separate normative and informative content.
-   Use canonical DTCS terminology.
-   Preserve architectural consistency.
-   Use deterministic, testable language.
-   Avoid implementation-specific normative requirements.

## 4. Canonical Chapter Structure

Normative chapters SHOULD follow this structure:

1.  Purpose
2.  Design Goals
3.  Core Concepts
4.  Normative Requirements
5.  Conformance (when applicable)
6.  Summary

## 5. Normative Language

Normative requirements SHALL use RFC 2119 / RFC 8174 terminology.

Approved keywords:

-   MUST
-   MUST NOT
-   SHALL
-   SHALL NOT
-   SHOULD
-   SHOULD NOT
-   MAY
-   OPTIONAL
-   RECOMMENDED

## 6. Canonical Architecture

Every specification SHALL preserve the conceptual architecture:

``` text
Transformation Contract
        │
        ▼
Canonical Object Model
        │
        ▼
Transformation Plan
        │
        ▼
Execution Plan
        │
        ▼
Runtime
```

## 7. Canonical Terminology

Reserved DTCS terms include:

-   Transformation Contract
-   Canonical Object Model
-   Transformation Plan
-   Execution Plan
-   Semantic Action
-   Expression
-   Function
-   Rule
-   Validator
-   Analyzer
-   Planner
-   Optimizer
-   Compiler
-   Runtime
-   Engine
-   Diagnostic

Normative text SHOULD avoid synonyms for these concepts.

## 8. Object Model First

The Canonical Object Model is the authoritative representation of a DTCS
specification.

YAML, JSON, TOML, XML, and other encodings are representations of that
model.

## 9. Semantic First

Normative requirements SHALL define semantic behavior rather than
implementation strategy.

## 10. Editorial Review

Every chapter SHALL satisfy the DTCS Editorial Review Checklist before
publication.

## 11. Quality Goals

Published specifications SHOULD:

-   Read as a single cohesive work.
-   Maintain consistent terminology.
-   Avoid duplicate definitions.
-   Clearly distinguish normative and informative content.
-   Preserve long-term editorial consistency.

## 12. Baseline Evolution

Editorial baselines SHALL be versioned independently of the DTCS
specification.

Future revisions SHOULD preserve compatibility whenever practical and
document all editorial changes.
