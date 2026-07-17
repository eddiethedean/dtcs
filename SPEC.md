# DTCS 3.0 Specification

**Status:** Draft\
**Version:** 3.0.0

# Chapter 1 --- Introduction

## 1. Purpose

The **Data Transformation Contract Standard (DTCS)** defines a
vendor-neutral, implementation-independent specification for expressing
the semantics of data transformations.

A DTCS Transformation Contract describes **what** a transformation
means. It does not prescribe **how** that transformation is implemented,
optimized, compiled, or executed.

Unless explicitly identified as informative, every requirement in this
specification is normative.

### 1.1 Specification series

This document is **DTCS 3.0.0**. It supersedes DTCS 2.0.0 as the governing
specification for conforming implementations that claim
`dtcsVersion: "3.0.0"`.

DTCS 3.0 incorporates Rich Portable Analytics: bounded lambda Expressions,
advanced string and regular-expression Functions, strict conversions,
statistical aggregates, complex-value transforms, reshape and set actions,
IANA temporal semantics, controlled nondeterminism, structural nested-field
evolution, expanded capability declarations, and canonical Transformation
Plan serialization (`dtcs.transform-plan/2`). These additions are organized
as independently claimable semantic-family profiles (Chapter 27; Appendix
A.9).

DTCS 3.0 retains the meanings of identifiers published by DTCS 2.0. Valid
DTCS 2.0 contracts and `dtcs.transform-plan/1` artifacts remain readable by
compatible implementations. Migration to `dtcs.transform-plan/2` SHALL NOT
change their semantics. Contracts authored for DTCS 1.0.0 remain valid under
implementations that continue to accept `dtcsVersion: "1.0.0"`, provided
published registry-entry semantics are preserved.

## 2. Design Goals

DTCS is designed to:

-   Standardize transformation semantics.
-   Preserve semantic meaning across heterogeneous execution
    environments.
-   Enable deterministic validation, analysis, planning, and
    compilation.
-   Support portable execution through multiple backend runtimes.
-   Promote interoperability among independent implementations.
-   Enable long-term ecosystem evolution through stable contracts.

## 3. Scope

This specification defines:

-   Transformation Contracts
-   The Canonical Object Model
-   Transformation Plans
-   Execution Plans
-   Semantic Actions
-   Expressions
-   Functions
-   Rules
-   Validation
-   Diagnostics
-   Lineage
-   Compatibility
-   Runtime Conformance
-   Extensibility
-   Governance

This specification SHALL NOT define:

-   Programming languages
-   SQL dialects
-   Storage technologies
-   Execution engine internals
-   Workflow orchestration
-   User interface behavior

## 4. Architectural Overview

The DTCS processing model consists of five logical stages.

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

Each stage SHALL preserve the semantic meaning established by the
originating Transformation Contract.

## 5. Foundational Principles

### 5.1 Contract-First

Every transformation SHALL be represented by a Transformation Contract
before execution.

### 5.2 Semantic-First

Semantic meaning SHALL take precedence over implementation details.

### 5.3 Implementation Independence

A conforming Transformation Contract SHALL remain portable across
conforming implementations.

### 5.4 Determinism

Equivalent Transformation Contracts SHALL represent equivalent
semantics.

### 5.5 Interoperability

Conforming implementations SHOULD exchange DTCS artifacts without
semantic loss.

### 5.6 Extensibility

Implementations MAY extend DTCS through the standardized extensibility
model without redefining standardized semantics.

## 6. Intended Audience

This specification is intended for:

-   Data engineers
-   Platform engineers
-   Compiler authors
-   Runtime implementers
-   Tool vendors
-   Standards organizations
-   Open-source contributors

## 7. Relationship to Other Standards

DTCS complements standards governing datasets, data quality, lineage,
orchestration, and governance.

DTCS standardizes transformation semantics rather than execution
technologies.

## 8. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL
NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and
**OPTIONAL** are to be interpreted as described in RFC 2119 and RFC 8174
when, and only when, they appear in all capital letters.

## 9. Terminology

For the purposes of this specification:

-   **Transformation Contract** --- A declarative specification
    describing the intended semantics of a data transformation.
-   **Canonical Object Model** --- The implementation-independent
    representation reconstructed from a DTCS document.
-   **Transformation Plan** --- The canonical semantic intermediate
    representation derived from a validated Transformation Contract.
-   **Execution Plan** --- A backend-specific representation generated
    from a Transformation Plan.
-   **Semantic Action** --- A standardized operation that transforms
    datasets.
-   **Expression** --- A computation that produces a value.
-   **Function** --- A reusable computation invoked by an Expression.
-   **Rule** --- A declarative invariant evaluated before, during, or
    after execution.
-   **Runtime** --- A component responsible for executing an Execution
    Plan while preserving DTCS semantics.

These definitions are normative.

## 10. Conformance

Conformance requirements are specified in Chapter 23.

Software claiming DTCS conformance SHALL satisfy all mandatory
requirements applicable to its declared conformance profile.

## 11. Summary

DTCS establishes a contract-first, semantic foundation for portable data
transformation.

The remainder of this specification defines the Canonical Object Model,
semantic model, validation framework, planning architecture, compiler
interfaces, runtime requirements, extensibility mechanisms, conformance
model, security considerations, and governance processes required to
build interoperable DTCS implementations.

---

# Chapter 2 --- Core Concepts

## 1. Purpose

This chapter defines the fundamental concepts that form the conceptual
foundation of the Data Transformation Contract Standard (DTCS).

These concepts establish the normative vocabulary used throughout this
specification. Every conforming implementation SHALL interpret these
concepts consistently.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS conceptual model is designed to be:

-   Semantic-first
-   Contract-first
-   Deterministic
-   Implementation-independent
-   Extensible
-   Composable

Core concepts SHALL remain stable across compatible revisions of the
specification.

------------------------------------------------------------------------

## 3. Transformation Contract

A **Transformation Contract** is the authoritative declaration of the
intended semantics of a data transformation.

A Transformation Contract SHALL define:

-   transformation identity
-   inputs
-   outputs
-   semantic actions
-   expressions
-   rules
-   guarantees
-   metadata

A Transformation Contract SHALL NOT prescribe implementation details.

------------------------------------------------------------------------

## 4. Canonical Object Model

The **Canonical Object Model** is the implementation-independent
representation of a Transformation Contract.

All supported document formats SHALL deserialize into an equivalent
Canonical Object Model.

The Canonical Object Model is the authoritative representation for
validation, analysis, planning, and compilation.

------------------------------------------------------------------------

## 5. Transformation Plan

A **Transformation Plan** is the canonical semantic intermediate
representation (IR) derived from a validated Transformation Contract.

A Transformation Plan SHALL:

-   preserve semantic meaning
-   preserve lineage
-   preserve logical types
-   remain independent of execution engines

Transformation Plans are defined in detail in Chapter 13.

------------------------------------------------------------------------

## 6. Execution Plan

An **Execution Plan** is a backend-specific representation produced from
a Transformation Plan.

Execution Plans MAY target different execution engines while preserving
identical observable semantics.

Generation of Execution Plans is implementation specific.

------------------------------------------------------------------------

## 7. Runtime

A **Runtime** executes an Execution Plan.

A conforming Runtime SHALL preserve:

-   transformation semantics
-   logical types
-   contractual guarantees
-   lineage
-   deterministic behavior for deterministic transformations

Runtime requirements are specified in Chapter 16.

------------------------------------------------------------------------

## 8. Semantic Actions

A **Semantic Action** is an implementation-independent operation that
changes the logical state of a dataset.

Semantic Actions define *what changes*.

They SHALL NOT define *how the change is implemented*.

The normative Semantic Action Library is defined in Chapter 17.

------------------------------------------------------------------------

## 9. Expressions

An **Expression** computes a value.

Expressions MAY invoke Functions and Operators.

Expressions SHALL NOT directly modify datasets.

------------------------------------------------------------------------

## 10. Functions

A **Function** is a reusable computation invoked by an Expression.

Functions SHALL:

-   declare parameter types
-   declare return types
-   define null behavior
-   define determinism

The Standard Function Library is defined in Chapter 18.

------------------------------------------------------------------------

## 11. Rules

A **Rule** is a declarative invariant evaluated before, during, or after
execution.

Rules express truth conditions.

Rules SHALL NOT modify datasets.

The Standard Rule Library is defined in Chapter 19.

------------------------------------------------------------------------

## 12. Diagnostics

A **Diagnostic** is structured information produced during processing.

Diagnostics MAY be generated by:

-   readers
-   validators
-   analyzers
-   planners
-   optimizers
-   compilers
-   runtimes

Diagnostics SHALL NOT alter transformation semantics.

------------------------------------------------------------------------

## 13. Conceptual Relationships

The DTCS conceptual model is illustrated below.

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

Semantic Actions
        │
        ▼
Expressions
        │
        ▼
Functions

Rules ─────────────► Validation
Diagnostics ◄────── All Processing Stages
```

This conceptual model is normative.

------------------------------------------------------------------------

## 14. Conformance

Implementations claiming DTCS conformance SHALL interpret the concepts
defined in this chapter consistently with the remainder of this
specification.

Implementations SHALL NOT redefine the semantics of these core concepts.

------------------------------------------------------------------------

## 15. Summary

The concepts defined in this chapter establish the common vocabulary and
architectural foundation of DTCS.

Subsequent chapters refine these concepts into formal object models,
semantic definitions, planning rules, compilation interfaces, runtime
requirements, and conformance obligations while preserving the
conceptual relationships established herein.

---

# Chapter 3 --- Canonical Object Model

## 1. Purpose

This chapter defines the **Canonical Object Model (COM)** of the Data
Transformation Contract Standard (DTCS).

The Canonical Object Model is the authoritative,
implementation-independent representation of a DTCS Transformation
Contract. All normative processing defined by this specification SHALL
operate on the Canonical Object Model rather than on any serialized
document format.

------------------------------------------------------------------------

## 2. Design Goals

The Canonical Object Model SHALL be:

-   implementation independent
-   deterministic
-   serialization independent
-   machine readable
-   extensible
-   stable across conforming implementations

The Canonical Object Model SHALL preserve the complete semantic meaning
of a Transformation Contract.

------------------------------------------------------------------------

## 3. Architectural Role

The Canonical Object Model occupies the architectural boundary between
document parsing and semantic analysis.

``` text
Transformation Contract
        │
        ▼
Canonical Object Model
        │
        ▼
Validation
        │
        ▼
Transformation Plan
```

All subsequent processing SHALL consume the Canonical Object Model.

------------------------------------------------------------------------

## 4. Canonical Representation

A conforming implementation SHALL reconstruct an equivalent Canonical
Object Model from every supported serialization.

Equivalent serialized documents SHALL produce semantically equivalent
Canonical Object Models.

Differences in formatting, whitespace, comments, or key ordering SHALL
NOT change the resulting object model.

------------------------------------------------------------------------

## 5. Object Identity

Every identifiable object MAY possess a stable identifier.

Identifiers SHALL:

-   uniquely identify the object within its scope
-   remain stable during processing
-   preserve referential integrity

Identifiers SHALL NOT derive semantic meaning solely from serialization
order.

------------------------------------------------------------------------

## 6. Object Composition

The Canonical Object Model SHALL support hierarchical composition.

Objects MAY contain:

-   child objects
-   references to other objects
-   collections
-   metadata

Composition SHALL preserve semantic meaning.

------------------------------------------------------------------------

## 7. References

Relationships between objects SHALL use stable references rather than
positional information.

Reference resolution SHALL be deterministic.

Invalid references SHALL produce diagnostics.

------------------------------------------------------------------------

## 8. Metadata

Every object MAY contain metadata.

Metadata SHALL supplement object semantics and SHALL NOT redefine
standardized behavior.

Unknown metadata MAY be preserved according to Chapter 22.

------------------------------------------------------------------------

## 9. Logical Components

A Transformation Contract SHALL be representable using the following
logical components:

-   Metadata
-   Inputs
-   Outputs
-   Semantic Actions
-   Expressions
-   Rules
-   Lineage
-   Compatibility
-   Extensions

Additional logical components MAY be introduced through future revisions
or standardized extensions.

------------------------------------------------------------------------

## 10. Object Integrity

A valid Canonical Object Model SHALL satisfy:

-   structural integrity
-   valid references
-   unique identities where required
-   type consistency
-   semantic completeness

Integrity validation SHALL precede planning.

------------------------------------------------------------------------

## 11. Immutability

The Canonical Object Model SHOULD be treated as immutable after
successful validation.

Implementations MAY create derived representations provided semantic
equivalence is preserved.

------------------------------------------------------------------------

## 12. Serialization Independence

The Canonical Object Model is normative.

YAML, JSON, TOML, XML, Protocol Buffers, and future encodings are
representations of the same underlying model.

Serialization SHALL NOT introduce or remove semantics.

------------------------------------------------------------------------

## 13. Transformation Plan Relationship

The Transformation Plan SHALL be derived from a validated Canonical
Object Model.

A Transformation Plan SHALL preserve:

-   semantic actions
-   expressions
-   rules
-   lineage
-   logical types
-   guarantees

------------------------------------------------------------------------

## 14. Extensibility

The Canonical Object Model MAY be extended through the DTCS
Extensibility Model.

Extensions SHALL:

-   use namespaced identifiers
-   preserve standardized semantics
-   remain machine readable

Extensions SHALL NOT redefine standardized object semantics.

------------------------------------------------------------------------

## 15. Conformance

A conforming DTCS implementation SHALL:

-   reconstruct a Canonical Object Model from every supported document
    format
-   preserve semantic equivalence during parsing
-   preserve object identity and references
-   reject invalid object structures through standardized diagnostics

------------------------------------------------------------------------

## 16. Summary

The Canonical Object Model is the normative foundation of DTCS
processing.

By separating semantic representation from document serialization, DTCS
enables consistent validation, analysis, planning, optimization,
compilation, and execution across heterogeneous implementations while
preserving identical observable semantics.

---

# Chapter 4 --- Type System

## 1. Purpose

This chapter defines the normative DTCS Type System.

The Type System establishes the logical value domains used by
Transformation Contracts. It provides a stable,
implementation-independent type model that SHALL be preserved throughout
validation, planning, compilation, and execution.

## 2. Design Goals

The DTCS Type System SHALL be:

-   implementation independent
-   deterministic
-   portable
-   extensible
-   machine readable
-   semantically stable

Logical types SHALL describe data semantics rather than storage or
programming language representations.

## 3. Type Model

A logical type defines the set of values and operations permitted for a
field or expression.

Implementations MAY use different native representations internally,
provided the observable logical type remains unchanged.

## 4. Primitive Types

DTCS defines the following primitive logical types:

-   boolean
-   integer
-   decimal
-   string
-   binary
-   date
-   time
-   datetime
-   duration

Future versions MAY introduce additional primitive types through
additive changes.

## 5. Composite Types

DTCS defines the following composite types:

-   list
-   map
-   object
-   tuple

Composite types SHALL declare the logical types of their contained
values.

## 6. Nullability

Nullability SHALL be explicit.

A nullable type and a non-nullable type are distinct logical types.

Implementations SHALL preserve nullability throughout processing.

## 7. Type Identity

Logical type identity SHALL be determined by:

-   type kind
-   parameters
-   nullability
-   constraints where applicable

Equivalent native types SHALL map to the same logical type when
semantics are identical.

## 8. Type Compatibility

Type compatibility SHALL be evaluated using logical semantics rather
than implementation-specific representations.

Compatibility analysis SHALL distinguish:

-   identical types
-   compatible types
-   incompatible types

Compatibility requirements are further defined in Chapter 11.

## 9. Type Conversion

Conversions MAY be explicit or implicit where permitted by this
specification.

Conversions SHALL preserve semantic correctness.

Lossy conversions SHALL be explicitly identified.

## 10. Type Inference

Implementations MAY infer logical types.

Inferred types SHALL be semantically equivalent to explicitly declared
types.

Inference SHALL NOT weaken contractual guarantees.

## 11. Expression Typing

Every Expression SHALL have a well-defined logical type.

Function parameter types, return types, and operator behavior SHALL be
type-consistent.

Type errors SHALL produce diagnostics.

## 12. Collection Typing

Collections SHALL declare their element or member types.

Nested collections SHALL recursively preserve logical typing.

## 13. Extension Types

Extensions MAY define additional logical types.

Extension types SHALL:

-   use namespaced identifiers where required
-   define compatibility behavior
-   define conversion behavior
-   preserve interoperability

## 14. Conformance

A conforming implementation SHALL:

-   preserve logical types
-   preserve nullability
-   perform type validation
-   reject invalid type usage through standardized diagnostics

## 15. Summary

The DTCS Type System provides a stable semantic foundation for
expressing values independently of programming languages and execution
engines. By standardizing logical types, nullability, compatibility, and
conversion behavior, DTCS enables portable validation, planning,
compilation, and execution while preserving consistent transformation
semantics.

---

# Chapter 5 --- Metadata

## 1. Purpose

This chapter defines the normative metadata model for the Data
Transformation Contract Standard (DTCS).

Metadata provides descriptive information about a Transformation
Contract and its constituent objects. Metadata enriches understanding,
governance, discovery, and interoperability while preserving the
semantic behavior of the contract.

Unless explicitly stated otherwise, metadata SHALL NOT alter
transformation semantics.

## 2. Design Goals

The DTCS metadata model SHALL be:

-   implementation independent
-   machine readable
-   extensible
-   deterministic
-   portable
-   semantically neutral

Metadata SHALL supplement, but SHALL NOT redefine, standardized DTCS
semantics.

## 3. Metadata Model

Metadata MAY be associated with:

-   Transformation Contracts
-   inputs
-   outputs
-   Semantic Actions
-   Expressions
-   Functions
-   Rules
-   other identifiable objects

Metadata associations SHALL preserve object identity.

## 4. Standard Metadata

DTCS defines the following standard metadata categories:

-   identity
-   descriptive information
-   ownership
-   lifecycle
-   documentation
-   governance
-   provenance
-   classification

Additional categories MAY be introduced through future specification
revisions.

## 5. Identity Metadata

Identity metadata SHOULD include, where applicable:

-   identifier
-   name
-   version
-   description

Identifiers SHALL remain stable for the lifetime of the object they
identify.

## 6. Documentation Metadata

Documentation metadata MAY include:

-   summary
-   detailed description
-   examples
-   references
-   external documentation

Documentation metadata is informative.

## 7. Governance Metadata

Governance metadata MAY include:

-   owner
-   steward
-   approval status
-   review date
-   policy references

Governance metadata SHALL NOT modify execution behavior.

## 8. Provenance Metadata

Metadata MAY describe provenance, including:

-   author
-   creation timestamp
-   modification timestamp
-   originating system

Operational provenance supplements, but does not replace, contractual
lineage defined elsewhere in this specification.

## 9. Classification Metadata

Implementations MAY classify objects for governance purposes.

Examples include:

-   public
-   internal
-   confidential
-   restricted

Classification metadata SHALL be treated as descriptive unless another
specification explicitly defines behavioral requirements.

## 10. Custom Metadata

Organizations MAY define additional metadata.

Custom metadata SHALL:

-   use namespaced identifiers where appropriate
-   avoid redefining standard metadata
-   preserve interoperability

## 11. Serialization

Metadata SHALL be preserved across supported DTCS serializations.

Serialization format SHALL NOT change metadata meaning.

## 12. Validation

Metadata SHALL be validated for:

-   structural correctness
-   identifier consistency
-   reference integrity

Unknown metadata MAY be preserved in accordance with Chapter 22.

## 13. Conformance

A conforming DTCS implementation SHALL:

-   preserve standardized metadata
-   preserve metadata identity
-   reject invalid standardized metadata through diagnostics
-   avoid assigning semantic meaning to descriptive metadata unless
    explicitly defined by this specification

## 14. Summary

The DTCS metadata model provides a consistent mechanism for describing
Transformation Contracts and related objects without altering their
semantics. By separating descriptive information from behavioral
definitions, DTCS supports governance, documentation, discovery, and
interoperability while maintaining a clear contract-first semantic
architecture.

---

# Chapter 6 --- Inputs and Outputs

## 1. Purpose

This chapter defines the normative model for declaring inputs and
outputs within a DTCS Transformation Contract.

Inputs and outputs establish the observable interface of a
transformation. They define the datasets consumed and produced by a
transformation without prescribing storage technologies, execution
engines, or transport mechanisms.

Unless explicitly stated otherwise, declarations in this chapter are
normative.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS input and output model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   portable
-   extensible
-   semantically complete

Input and output declarations SHALL describe contractual interfaces
rather than implementation details.

------------------------------------------------------------------------

## 3. Input Model

Every Transformation Contract SHALL declare its required inputs.

Each input SHALL represent a logical dataset or data stream consumed by
the transformation.

Input declarations SHALL be independent of physical storage location.

------------------------------------------------------------------------

## 4. Output Model

Every Transformation Contract SHALL declare its outputs.

Each output SHALL represent a logical dataset or data stream produced by
the transformation.

Output declarations SHALL describe the contractual result of execution.

------------------------------------------------------------------------

## 5. Dataset Identity

Each declared input and output SHALL possess a stable logical identity.

Logical identities SHALL remain stable across document serializations,
validation, planning, compilation, and execution.

Identifiers SHALL NOT depend upon ordering within a document.

------------------------------------------------------------------------

## 6. Schema Association

Inputs and outputs SHOULD declare their logical schema.

Schemas SHALL use the DTCS Type System defined in Chapter 4.

Schema declarations SHALL describe logical structure rather than native
implementation types.

------------------------------------------------------------------------

## 7. Preconditions

Inputs MAY define preconditions.

Preconditions express assumptions that SHALL hold before execution.

Examples include:

-   required fields
-   minimum record counts
-   required Rules
-   schema compatibility

Preconditions SHALL be evaluated before execution when possible.

------------------------------------------------------------------------

## 8. Postconditions

Outputs MAY define postconditions.

Postconditions express guarantees provided by the Transformation
Contract.

Examples include:

-   schema conformance
-   uniqueness guarantees
-   required Rules
-   expected logical types

Postconditions SHALL be verifiable.

------------------------------------------------------------------------

## 9. Lineage

Every output SHALL be traceable to one or more declared inputs.

Lineage SHALL preserve logical provenance independent of execution
technology.

Lineage behavior is further specified in Chapter 10.

------------------------------------------------------------------------

## 10. Multiplicity

Transformation Contracts MAY declare multiple inputs and multiple
outputs.

Input and output ordering SHALL NOT imply semantic meaning unless
explicitly defined by the contract.

------------------------------------------------------------------------

## 11. Optional Inputs

Inputs MAY be declared optional.

Optionality SHALL be explicit.

Implementations SHALL distinguish between omitted optional inputs and
invalid required inputs.

------------------------------------------------------------------------

## 12. Streaming Interfaces

Inputs and outputs MAY represent bounded or unbounded datasets.

Streaming behavior SHALL be explicitly declared.

Streaming declarations SHALL preserve the same logical interface model
used for bounded datasets.

------------------------------------------------------------------------

## 13. Extensibility

Input and output declarations MAY be extended using the DTCS
Extensibility Model.

Extensions SHALL:

-   preserve standardized semantics
-   use stable identifiers
-   remain machine readable

Extensions SHALL NOT redefine the meaning of standardized properties.

------------------------------------------------------------------------

## 14. Validation

Implementations SHALL validate:

-   required inputs
-   required outputs
-   identifier uniqueness
-   schema consistency
-   reference integrity
-   declared preconditions and postconditions where applicable

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 15. Conformance

A conforming DTCS implementation SHALL:

-   preserve declared inputs and outputs
-   preserve logical identities
-   preserve associated schemas
-   preserve contractual preconditions and postconditions
-   reject invalid interface definitions through standardized
    Diagnostics

------------------------------------------------------------------------

## 16. Summary

The DTCS input and output model defines the contractual interface of a
Transformation Contract.

By expressing logical datasets, schemas, preconditions, postconditions,
and lineage independently of implementation technologies, DTCS
establishes a stable interface for validation, planning, compilation,
and execution while preserving consistent transformation semantics
across heterogeneous environments.

---

# Chapter 7 --- Transformation Semantics

## 1. Purpose

This chapter defines the normative semantic model for transformations in
the Data Transformation Contract Standard (DTCS).

Transformation semantics describe the logical meaning of a
transformation independently of any execution engine, programming
language, storage technology, or optimization strategy.

The semantic meaning defined by a Transformation Contract SHALL remain
invariant throughout validation, planning, compilation, and execution.

------------------------------------------------------------------------

## 2. Design Goals

Transformation semantics SHALL be:

-   implementation independent
-   deterministic
-   composable
-   analyzable
-   portable
-   extensible

Semantics SHALL define observable behavior rather than implementation
strategy.

------------------------------------------------------------------------

## 3. Transformation Definition

A transformation is a declarative description of how one or more logical
inputs are converted into one or more logical outputs.

A transformation SHALL be represented by a Transformation Contract.

Transformation behavior SHALL be expressed using Semantic Actions,
Expressions, Functions, and Rules.

------------------------------------------------------------------------

## 4. Semantic Identity

Every Transformation Contract SHALL define exactly one semantic meaning.

Equivalent implementations MAY differ internally but SHALL preserve
identical observable behavior.

------------------------------------------------------------------------

## 5. Observable Behavior

Observable behavior consists of:

-   logical outputs
-   logical types
-   contractual guarantees
-   declared lineage
-   declared Rules

Execution performance, optimization strategy, and internal data
structures are not part of observable behavior unless explicitly
declared by a DTCS profile.

------------------------------------------------------------------------

## 6. Semantic Composition

Transformations MAY be composed from multiple Semantic Actions.

Composition SHALL preserve the semantics of each constituent action.

The resulting Transformation Plan SHALL represent the composed semantics
as a single canonical intermediate representation.

------------------------------------------------------------------------

## 7. Determinism

A Transformation Contract MAY declare whether it is deterministic.

For deterministic transformations, identical inputs and identical
execution context SHALL produce semantically equivalent outputs.

------------------------------------------------------------------------

## 8. Purity

A transformation MAY be declared pure.

Pure transformations SHALL NOT produce externally observable side
effects beyond their declared outputs.

Impure transformations SHALL explicitly declare any externally
observable side effects.

------------------------------------------------------------------------

## 9. Preconditions

Transformations MAY declare preconditions.

Preconditions express assumptions that SHALL hold before execution.

Violation of a mandatory precondition SHALL produce a standardized
Diagnostic.

------------------------------------------------------------------------

## 10. Postconditions

Transformations MAY declare postconditions.

Postconditions define guarantees that SHALL hold after successful
execution.

Postconditions SHALL be objectively verifiable.

------------------------------------------------------------------------

## 11. Information Preservation

Semantic Actions MAY preserve, derive, aggregate, or discard
information.

Information loss SHALL be explicit.

Lineage SHALL accurately describe the effect of each transformation on
source information.

------------------------------------------------------------------------

## 12. Ordering

Logical ordering SHALL NOT be assumed unless explicitly declared by the
Transformation Contract.

Implementations SHALL NOT infer semantic ordering from document
structure.

------------------------------------------------------------------------

## 13. Semantic Equivalence

Two transformations are semantically equivalent when they produce
identical observable behavior for all valid inputs under equivalent
execution contexts.

Semantic equivalence SHALL be independent of execution technology.

------------------------------------------------------------------------

## 14. Validation

Implementations SHALL validate that transformation semantics are:

-   internally consistent
-   type correct
-   reference complete
-   compatible with declared Rules
-   compatible with declared inputs and outputs

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 15. Conformance

A conforming DTCS implementation SHALL:

-   preserve transformation semantics
-   preserve declared guarantees
-   preserve lineage
-   preserve logical types
-   reject semantically invalid Transformation Contracts

Implementations SHALL NOT alter semantic meaning during planning,
optimization, compilation, or execution.

------------------------------------------------------------------------

## 16. Summary

The DTCS semantic model establishes the implementation-independent
meaning of a Transformation Contract.

By defining transformations in terms of observable behavior rather than
execution strategy, DTCS enables consistent validation, optimization,
compilation, and execution across heterogeneous platforms while
preserving identical semantic results.

---

# Chapter 8 --- Expression Language

## 1. Purpose

This chapter defines the normative expression model of the Data
Transformation Contract Standard (DTCS).

Expressions compute values. Expressions SHALL NOT directly modify
datasets. Dataset modification SHALL occur only through Semantic Actions
defined by this specification.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS Expression Language SHALL be:

-   implementation independent
-   deterministic by default
-   strongly typed
-   composable
-   machine readable
-   portable

Expression semantics SHALL be preserved across all conforming
implementations.

------------------------------------------------------------------------

## 3. Expression Model

An Expression is a declarative computation that evaluates to exactly one
logical value.

Expressions MAY reference:

-   literals
-   fields
-   parameters
-   Functions
-   Operators
-   other Expressions

Expressions SHALL NOT directly produce side effects.

### 3.1 Structured expression nodes

Portable plans and the optional COM `body` field SHALL represent
expressions as structured nodes. Authoring documents MAY supply a string
`expr` as sugar; when both `expr` and `body` are present, `body` is
authoritative.

Structured nodes SHALL use a `kind` discriminator. The kernel node kinds
are:

-   `literal` --- boolean, string, integer, or decimal value
-   `fieldRef` --- field reference (`target`)
-   `unary` --- unary operator (`op`, `expr`)
-   `binary` --- binary operator (`op`, `left`, `right`)
-   `call` --- function invocation (`callee`, `args`)

Operator identities in portable capability declarations MAY use registry
identifiers (`dtcs:eq`) or short names (`eq`). Opaque host-language
objects and `repr()` serialization are prohibited.

------------------------------------------------------------------------

## 4. Evaluation Context

Expression evaluation SHALL occur within an explicit evaluation context.

The evaluation context MAY include:

-   current record
-   current dataset
-   transformation parameters
-   execution context (where explicitly permitted)

Hidden implementation state SHALL NOT influence expression semantics.

------------------------------------------------------------------------

## 5. Expression Identity

Every Expression SHALL have:

-   a logical type
-   deterministic semantics unless explicitly declared otherwise
-   well-defined null behavior

------------------------------------------------------------------------

## 6. Type System Integration

Expression typing SHALL conform to Chapter 4.

Every Expression SHALL evaluate to a single logical type.

Type violations SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 7. Functions

Expressions MAY invoke standard or extension Functions.

Function invocation SHALL preserve:

-   parameter ordering
-   declared type rules
-   null behavior
-   determinism

Function semantics are defined in Chapter 18.

------------------------------------------------------------------------

## 8. Operators

Operators provide primitive computations used by Expressions.

Standard operator categories include:

-   arithmetic
-   comparison
-   logical
-   string
-   collection

Operator behavior SHALL be deterministic for identical operands.

------------------------------------------------------------------------

## 9. Null Semantics

Every Expression SHALL define null behavior explicitly.

Implementations SHALL distinguish between:

-   null values
-   missing values
-   invalid values

Implicit null handling is prohibited.

------------------------------------------------------------------------

## 10. Composition

Expressions MAY be nested.

Nested Expressions SHALL preserve the semantics of each constituent
Expression.

Evaluation order SHALL NOT alter observable semantics except where
explicitly defined.

------------------------------------------------------------------------

## 11. Determinism

Expressions are deterministic unless explicitly documented otherwise.

Non-deterministic Expressions SHALL declare the source of
non-determinism.

------------------------------------------------------------------------

## 12. Constant Expressions

Implementations MAY evaluate constant Expressions during planning or
compilation.

Constant evaluation SHALL preserve semantic equivalence.

------------------------------------------------------------------------

## 13. Validation

Implementations SHALL validate:

-   type correctness
-   reference resolution
-   function signatures
-   operator compatibility
-   null semantics

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 14. Optimization

Implementations MAY optimize Expressions through semantics-preserving
rewrites.

Examples include:

-   constant folding
-   algebraic simplification
-   dead expression elimination

Optimization SHALL NOT alter observable behavior.

------------------------------------------------------------------------

## 15. Conformance

A conforming DTCS implementation SHALL:

-   preserve expression semantics
-   preserve logical types
-   preserve null semantics
-   preserve determinism
-   reject invalid Expressions through standardized Diagnostics

------------------------------------------------------------------------

## 16. Summary

The DTCS Expression Language provides the implementation-independent
mechanism for computing values within a Transformation Contract.

By separating value computation from dataset transformation, DTCS
enables portable validation, optimization, planning, compilation, and
execution while preserving identical observable semantics across
heterogeneous execution environments.

---

# Chapter 9 --- Validation

## 1. Purpose

This chapter defines the normative validation model for the Data
Transformation Contract Standard (DTCS).

Validation determines whether a Transformation Contract conforms to the
requirements of this specification before planning, compilation, or
execution. Validation SHALL operate on the Canonical Object Model.

------------------------------------------------------------------------

## 2. Design Goals

Validation SHALL be:

-   deterministic
-   implementation independent
-   repeatable
-   machine readable
-   comprehensive
-   extensible

Validation SHALL detect specification violations without altering
transformation semantics.

------------------------------------------------------------------------

## 3. Validation Model

Validation is the process of evaluating a Transformation Contract
against the normative requirements of DTCS.

A conforming implementation SHALL perform validation before constructing
a Transformation Plan.

------------------------------------------------------------------------

## 4. Validation Phases

Validation SHALL be performed in the following logical phases:

1.  Document Validation
2.  Canonical Object Model Validation
3.  Structural Validation
4.  Type Validation
5.  Reference Validation
6.  Semantic Validation
7.  Extension Validation

Implementations MAY combine phases internally provided the observable
results remain equivalent.

------------------------------------------------------------------------

## 5. Document Validation

Document Validation SHALL verify:

-   supported serialization
-   document well-formedness
-   specification version
-   required top-level elements

------------------------------------------------------------------------

## 6. Structural Validation

Structural Validation SHALL verify:

-   required objects
-   required properties
-   identifier uniqueness
-   object composition
-   cardinality constraints

------------------------------------------------------------------------

## 7. Type Validation

Type Validation SHALL verify:

-   logical type correctness
-   nullability
-   function signatures
-   expression typing
-   collection typing

Type evaluation SHALL conform to Chapter 4.

------------------------------------------------------------------------

## 8. Reference Validation

Reference Validation SHALL verify:

-   object references
-   identifier resolution
-   dependency integrity
-   extension references

Unresolved references SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 9. Semantic Validation

Semantic Validation SHALL verify:

-   Transformation semantics
-   Semantic Actions
-   Expressions
-   Rules
-   Preconditions
-   Postconditions
-   Lineage consistency

Semantic Validation SHALL preserve the meaning of the Transformation
Contract.

------------------------------------------------------------------------

## 10. Extension Validation

Extensions SHALL be validated according to Chapter 22.

Unsupported mandatory extensions SHALL prevent successful validation.

Unsupported optional extensions MAY generate Diagnostics while allowing
validation to continue.

------------------------------------------------------------------------

## 11. Validation Results

Successful validation SHALL indicate that a Transformation Contract
satisfies the applicable normative requirements of this specification.

Successful validation SHALL NOT imply successful execution.

------------------------------------------------------------------------

## 12. Diagnostics

Validation failures SHALL produce standardized Diagnostics as defined in
Chapter 20.

Diagnostics SHOULD identify:

-   violated requirement
-   affected object
-   validation phase
-   severity
-   suggested remediation where practical

------------------------------------------------------------------------

## 13. Failure Handling

Implementations MAY continue validation after non-fatal failures in
order to report additional Diagnostics.

Fatal validation failures MAY terminate further processing.

------------------------------------------------------------------------

## 14. Conformance

A conforming DTCS implementation SHALL:

-   validate every Transformation Contract before planning
-   evaluate all mandatory validation phases
-   preserve semantic meaning during validation
-   report validation failures through standardized Diagnostics

------------------------------------------------------------------------

## 15. Summary

The DTCS Validation Model provides a deterministic,
implementation-independent process for verifying that a Transformation
Contract satisfies the requirements of this specification.

By validating structure, types, references, semantics, and extensions
before planning and execution, DTCS enables reliable analysis,
optimization, compilation, and runtime behavior while preserving the
contract-first semantic architecture.

---

# Chapter 10 --- Lineage

## 1. Purpose

This chapter defines the normative lineage model for the Data
Transformation Contract Standard (DTCS).

Lineage describes the semantic relationships between declared inputs,
intermediate transformations, and declared outputs. Lineage SHALL be
represented independently of execution technology and SHALL preserve the
logical provenance established by a Transformation Contract.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS lineage model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   analyzable
-   portable
-   extensible

Lineage SHALL describe semantic provenance rather than physical
execution history.

------------------------------------------------------------------------

## 3. Lineage Model

Lineage is the directed graph of relationships that explains how output
data is derived from input data.

A Transformation Contract SHALL define lineage sufficient to describe
the provenance of every declared output.

------------------------------------------------------------------------

## 4. Scope

Lineage MAY describe relationships between:

-   inputs
-   outputs
-   Semantic Actions
-   Expressions
-   Rules
-   Transformation Plans

Physical storage locations, execution nodes, and runtime scheduling are
outside the scope of DTCS lineage.

------------------------------------------------------------------------

## 5. Lineage Identity

Every lineage relationship SHOULD possess a stable logical identity.

Lineage identities SHALL remain stable across validation, planning,
compilation, and execution.

------------------------------------------------------------------------

## 6. Provenance Relationships

A lineage relationship SHALL identify:

-   one or more source objects
-   one or more destination objects
-   the semantic operation relating them

Relationship semantics SHALL be preserved throughout processing.

------------------------------------------------------------------------

## 7. Information Flow

Lineage SHALL represent the logical flow of information through a
transformation.

Information MAY be:

-   preserved
-   derived
-   aggregated
-   filtered
-   partitioned
-   discarded

Information loss SHALL be explicit where it affects contractual
guarantees.

------------------------------------------------------------------------

## 8. Semantic Preservation

Planning, optimization, compilation, and execution SHALL preserve
declared lineage.

Equivalent implementations MAY represent lineage differently internally
provided observable lineage remains equivalent.

------------------------------------------------------------------------

## 9. Granularity

Implementations MAY support lineage at multiple logical levels,
including:

-   dataset
-   object
-   field
-   expression

Supported lineage granularity SHOULD be declared through implementation
capabilities.

------------------------------------------------------------------------

## 10. Validation

Implementations SHALL validate:

-   lineage references
-   lineage completeness
-   reference integrity
-   semantic consistency

Invalid lineage SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Analysis

Lineage MAY be analyzed to support:

-   impact analysis
-   dependency analysis
-   compatibility analysis
-   governance
-   documentation

Analytical results SHALL NOT modify declared lineage.

------------------------------------------------------------------------

## 12. Extensions

Extensions MAY introduce additional lineage metadata or relationship
types.

Extensions SHALL preserve standardized lineage semantics and SHALL NOT
redefine the meaning of existing lineage relationships.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   preserve declared lineage
-   preserve provenance relationships
-   validate lineage integrity
-   report lineage violations through standardized Diagnostics

Implementations SHALL NOT alter declared lineage semantics during
planning, optimization, compilation, or execution.

------------------------------------------------------------------------

## 14. Summary

The DTCS lineage model provides an implementation-independent
representation of the logical provenance of data throughout a
Transformation Contract.

By standardizing semantic lineage independently of execution
technologies, DTCS enables consistent governance, impact analysis,
compatibility analysis, planning, and execution while preserving the
contract-first architecture established by this specification.

---

# Chapter 11 --- Compatibility

## 1. Purpose

This chapter defines the normative compatibility model for the Data
Transformation Contract Standard (DTCS).

Compatibility determines whether two Transformation Contracts, or
revisions of the same Transformation Contract, can interoperate without
violating declared semantics or contractual guarantees.

Compatibility analysis SHALL operate on the Canonical Object Model.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS compatibility model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   analyzable
-   extensible
-   version aware

Compatibility SHALL be evaluated using semantic meaning rather than
document structure.

------------------------------------------------------------------------

## 3. Compatibility Model

Compatibility evaluates the relationship between a source Transformation
Contract and a target Transformation Contract.

The outcome of compatibility analysis SHALL be based on observable
semantics, not implementation details.

------------------------------------------------------------------------

## 4. Compatibility Levels

DTCS defines the following compatibility classifications:

-   Identical
-   Backward Compatible
-   Forward Compatible
-   Conditionally Compatible
-   Incompatible

Every compatibility result SHALL resolve to exactly one classification.

------------------------------------------------------------------------

## 5. Compatibility Scope

Compatibility analysis MAY evaluate:

-   inputs
-   outputs
-   logical types
-   Semantic Actions
-   Expressions
-   Rules
-   lineage
-   metadata with defined behavioral semantics
-   extensions

Each evaluated aspect SHALL contribute to the overall compatibility
result.

------------------------------------------------------------------------

## 6. Semantic Compatibility

Two Transformation Contracts are semantically compatible when all
required observable behavior is preserved.

Observable behavior includes:

-   logical outputs
-   logical types
-   contractual guarantees
-   declared lineage
-   Rule behavior

------------------------------------------------------------------------

## 7. Type Compatibility

Logical type compatibility SHALL conform to Chapter 4.

Type compatibility SHALL be evaluated independently of native
implementation types.

------------------------------------------------------------------------

## 8. Interface Compatibility

Input and output compatibility SHALL evaluate:

-   required interfaces
-   optional interfaces
-   schema compatibility
-   contractual preconditions
-   contractual postconditions

Interface compatibility SHALL preserve contractual obligations.

------------------------------------------------------------------------

## 9. Behavioral Compatibility

Behavioral compatibility SHALL evaluate changes to:

-   Semantic Actions
-   Expressions
-   Functions
-   Rules

Equivalent implementations SHALL produce equivalent observable behavior.

------------------------------------------------------------------------

## 10. Extension Compatibility

Extensions SHALL declare their compatibility requirements.

Unsupported mandatory extensions SHALL produce an incompatible result
unless explicitly accommodated by the implementation.

------------------------------------------------------------------------

## 11. Version Compatibility

Specification version compatibility SHALL be evaluated independently
from contract compatibility.

Support for a DTCS specification version SHALL NOT imply compatibility
between arbitrary Transformation Contracts.

------------------------------------------------------------------------

## 12. Compatibility Analysis

Compatibility analysis SHALL be deterministic.

Equivalent inputs to the analysis process SHALL produce equivalent
compatibility results.

Analysis SHALL NOT modify either Transformation Contract.

------------------------------------------------------------------------

## 13. Diagnostics

Compatibility violations SHALL produce standardized Diagnostics.

Diagnostics SHOULD identify:

-   affected object
-   violated compatibility rule
-   compatibility classification
-   suggested remediation where practical

------------------------------------------------------------------------

## 14. Conformance

A conforming DTCS implementation SHALL:

-   evaluate compatibility using the Canonical Object Model
-   preserve semantic meaning during analysis
-   classify compatibility using the standardized model
-   report incompatibilities through standardized Diagnostics

------------------------------------------------------------------------

## 15. Summary

The DTCS compatibility model provides an implementation-independent
framework for evaluating whether Transformation Contracts can
interoperate while preserving semantics and contractual guarantees.

By standardizing compatibility analysis independently of execution
technologies, DTCS enables reliable contract evolution, impact analysis,
governance, planning, and deployment across heterogeneous
implementations.

---

# Chapter 12 --- Evolution

## 1. Purpose

This chapter defines the normative evolution model for the Data
Transformation Contract Standard (DTCS).

Evolution describes how a Transformation Contract MAY change over time
while preserving interoperability, semantic integrity, and compatibility
where applicable. Evolution SHALL be evaluated independently of any
specific implementation or deployment environment.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS evolution model SHALL be:

-   implementation independent
-   deterministic
-   version aware
-   analyzable
-   backward-compatible where practical
-   extensible

Evolution SHALL preserve semantic intent unless an incompatible change
is explicitly introduced.

------------------------------------------------------------------------

## 3. Evolution Model

A Transformation Contract MAY evolve through one or more revisions.

Each revision SHALL represent a complete and internally consistent
Transformation Contract.

Evolution SHALL NOT rely upon undocumented behavioral changes.

------------------------------------------------------------------------

## 4. Contract Identity

A Transformation Contract SHALL possess a stable logical identity.

Revisions SHALL retain the same logical identity unless a new contract
is intentionally created.

Identity SHALL be independent of document serialization.

------------------------------------------------------------------------

## 5. Versioning

Each revision SHOULD declare version information.

Version identifiers SHALL uniquely distinguish revisions of the same
Transformation Contract.

Version identifiers SHALL NOT be used as the sole basis for
compatibility decisions.

------------------------------------------------------------------------

## 6. Change Categories

Contract changes MAY include:

-   metadata changes
-   interface changes
-   type changes
-   semantic changes
-   rule changes
-   lineage changes
-   extension changes

Each change category SHALL be evaluated independently during
compatibility analysis.

------------------------------------------------------------------------

## 7. Compatible Evolution

Compatible evolution preserves required observable behavior.

Examples MAY include:

-   documentation improvements
-   additional optional metadata
-   additive optional interfaces
-   backward-compatible extensions

Compatibility SHALL be determined according to Chapter 11.

------------------------------------------------------------------------

## 8. Incompatible Evolution

Incompatible evolution changes observable behavior or contractual
guarantees.

Examples MAY include:

-   removing required inputs
-   changing required logical types
-   altering mandatory Semantic Actions
-   weakening declared guarantees

Such changes SHALL be classified as incompatible unless explicitly
accommodated by a profile.

------------------------------------------------------------------------

## 9. Migration Guidance

Implementations SHOULD provide migration guidance when incompatible
revisions are detected.

Migration guidance is informative and SHALL NOT modify contract
semantics.

------------------------------------------------------------------------

## 10. Deprecation

Objects MAY be deprecated prior to removal.

Deprecation SHOULD identify:

-   the deprecated object
-   recommended replacement
-   anticipated removal

Deprecated objects remain valid until removed by a subsequent revision.

------------------------------------------------------------------------

## 11. Evolution Analysis

Evolution analysis SHALL compare revisions using the Canonical Object
Model.

Analysis SHALL preserve semantic meaning and SHALL NOT modify either
revision.

------------------------------------------------------------------------

## 12. Diagnostics

Evolution analysis SHALL produce standardized Diagnostics for:

-   incompatible revisions
-   deprecated features
-   unsupported changes
-   invalid version metadata

Diagnostics SHOULD identify affected objects and recommended remediation
where practical.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   preserve contract identity across revisions
-   distinguish revisions from distinct contracts
-   evaluate evolution independently of serialization
-   report evolution issues through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS evolution model provides a deterministic framework for managing
the lifecycle of Transformation Contracts.

By separating revision management from compatibility analysis while
preserving semantic intent, DTCS enables controlled contract evolution,
governance, migration planning, and long-term interoperability across
independent implementations.

---

# Chapter 13 --- Transformation Plan

## 1. Purpose

This chapter defines the normative **Transformation Plan** model for the
Data Transformation Contract Standard (DTCS).

A Transformation Plan is the canonical semantic intermediate
representation (IR) derived from a validated Canonical Object Model. It
is the authoritative representation consumed by optimizers and
compilers.

The Transformation Plan SHALL preserve the complete semantic meaning of
the originating Transformation Contract.

------------------------------------------------------------------------

## 2. Design Goals

The Transformation Plan SHALL be:

-   implementation independent
-   deterministic
-   semantically complete
-   machine readable
-   analyzable
-   optimizable
-   portable

------------------------------------------------------------------------

## 3. Architectural Role

The Transformation Plan occupies the boundary between semantic analysis
and backend compilation.

``` text
Transformation Contract
        │
        ▼
Canonical Object Model
        │
        ▼
Validation
        │
        ▼
Transformation Plan
        │
        ▼
Optimization
        │
        ▼
Execution Plan
```

All backend compilation SHALL begin from a Transformation Plan.

------------------------------------------------------------------------

## 4. Plan Model

A Transformation Plan SHALL represent the complete semantic intent of a
Transformation Contract.

The plan SHALL include, directly or by reference:

-   declared inputs
-   declared outputs
-   Semantic Actions
-   Expressions
-   Rules
-   logical types
-   lineage
-   contractual guarantees

No required semantic information SHALL be omitted.

------------------------------------------------------------------------

## 5. Semantic Preservation

Construction of a Transformation Plan SHALL preserve:

-   observable behavior
-   logical types
-   lineage
-   preconditions
-   postconditions
-   determinism

Plan construction SHALL NOT introduce or remove semantics.

------------------------------------------------------------------------

## 6. Canonical Representation

Equivalent Transformation Contracts SHALL produce semantically
equivalent Transformation Plans.

Internal implementation details MAY differ provided observable semantics
remain identical.

------------------------------------------------------------------------

## 7. Plan Identity

A Transformation Plan MAY possess a stable logical identifier.

Plan identity SHALL remain associated with the originating
Transformation Contract.

------------------------------------------------------------------------

## 8. Dependency Graph

A Transformation Plan SHALL define the logical dependency relationships
required for semantic evaluation.

Dependency relationships SHALL be acyclic unless explicitly permitted by
a DTCS profile.

------------------------------------------------------------------------

## 9. Optimization Boundary

The Transformation Plan is the input to optimization.

Optimizations SHALL transform a Transformation Plan into a semantically
equivalent Transformation Plan.

Optimization SHALL NOT directly generate an Execution Plan.

------------------------------------------------------------------------

## 10. Compiler Boundary

Compilers SHALL consume a Transformation Plan and produce one or more
Execution Plans.

Compilation SHALL preserve all declared semantics and contractual
guarantees.

------------------------------------------------------------------------

## 11. Validation

Implementations SHALL validate that a Transformation Plan:

-   is semantically complete
-   preserves contract semantics
-   contains valid dependencies
-   preserves logical types
-   preserves lineage

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 12. Serialization

Implementations MAY serialize Transformation Plans.

Serialized plans SHALL preserve semantic equivalence with the in-memory
representation.

Serialization format is implementation specific unless defined by a DTCS
profile.

### 12.1 Portable plan serialization profile

DTCS 3.0 defines the canonical serialization identity
`dtcs.transform-plan/2`. Implementations conforming to DTCS 3.0 SHALL also
read valid `dtcs.transform-plan/1` documents and SHALL migrate them to version
2 without semantic change.

A portable plan document SHALL include:

-   `profile` --- a portable profile identifier (for example
    `dtcs:profile/portable-relational-kernel/2`)
-   `specificationVersion` --- governing DTCS specification version
-   `registryVersions` --- map of registry category to pinned version
    (`actions`, `functions`, `operators`, `types`)
-   `transformation` --- originating contract identity
-   `inputs`, `parameters`, `actions`, `outputs`, `rules`, `lineage`
-   `requirements` --- capability requirements
-   `extensions` --- vendor extensions (optional)

Portable plans SHALL:

-   use data-only representation (no executable objects, host-language
    `repr()`, live handles, resolved secrets, or raw SQL)
-   apply canonical key ordering and meaningful sequence ordering
-   bound bytes, depth, node count, and literal/collection size
-   reject unknown mandatory constructs
-   produce identical semantic fingerprints for equivalent plans under
    the same profile and registry versions

Version 2 portable plans MAY contain bounded `lambda` Expression nodes,
generator actions, explicit semantic modes, and the resource and environment
requirements defined in Chapter 27. They SHALL use canonical serialization
for all such constructs. Every semantic input, including registry and profile
versions, lambda scope, grammar versions, timezone-data requirements, random
algorithms and seeds, error modes, ordering policies, collision policies, and
observable resource limits, SHALL contribute to the semantic fingerprint.
Display names, comments, source locations, and non-semantic explanatory
metadata SHALL NOT contribute to the semantic fingerprint.

Internal COM plan IR (`nodes` / `dependencies`) MAY continue to be used as an
implementation representation. Export to `dtcs.transform-plan/2` SHALL be
deterministic.

Security budgets for portable plans (default reference limits):

-   maximum document bytes: 8 MiB
-   maximum nesting depth: 128
-   maximum expression/action nodes: 100_000

------------------------------------------------------------------------

## 13. Extensibility

Transformation Plans MAY be extended through the DTCS Extensibility
Model.

Extensions SHALL preserve standardized semantics and SHALL NOT redefine
the meaning of existing plan constructs.

------------------------------------------------------------------------

## 14. Conformance

A conforming DTCS implementation SHALL:

-   construct a Transformation Plan from a validated Canonical Object
    Model
-   preserve semantic meaning
-   preserve logical types
-   preserve lineage
-   preserve contractual guarantees
-   reject invalid plans through standardized Diagnostics

------------------------------------------------------------------------

## 15. Summary

The Transformation Plan is the canonical semantic intermediate
representation of DTCS.

By separating semantic planning from backend compilation, DTCS enables
multiple optimizers, compilers, and execution engines to interoperate
while preserving identical observable behavior and contractual
guarantees across heterogeneous platforms.

---

# Chapter 14 --- Engine Capability Model

## 1. Purpose

This chapter defines the normative Engine Capability Model for the Data
Transformation Contract Standard (DTCS).

The Engine Capability Model describes the features, constraints, and
execution characteristics supported by an execution engine. Capability
declarations enable deterministic planning and compilation without
embedding engine-specific behavior into a Transformation Contract.

Capability declarations SHALL describe execution capabilities and SHALL
NOT modify the semantics of a Transformation Contract.

------------------------------------------------------------------------

## 2. Design Goals

The Engine Capability Model SHALL be:

-   implementation independent
-   machine readable
-   deterministic
-   extensible
-   version aware
-   analyzable

------------------------------------------------------------------------

## 3. Capability Model

An engine capability declaration identifies the semantic features an
execution engine supports.

Capabilities MAY describe:

-   logical types
-   Semantic Actions
-   Functions
-   operators
-   optimization features
-   execution characteristics
-   extension support

------------------------------------------------------------------------

## 4. Capability Identity

Each capability declaration SHALL identify:

-   engine identifier
-   engine version
-   capability version

Capability identifiers SHALL be stable.

------------------------------------------------------------------------

## 5. Capability Categories

Capabilities SHOULD be grouped into categories including:

-   language features
-   type support
-   function support
-   Semantic Action support
-   optimization support
-   runtime features
-   extension support

Additional categories MAY be introduced by future revisions.

------------------------------------------------------------------------

## 6. Capability Matching

Compilers SHALL compare Transformation Plan requirements against engine
capabilities.

Unsupported mandatory capabilities SHALL prevent successful compilation
unless an applicable profile explicitly permits an alternative.

------------------------------------------------------------------------

## 7. Capability Profiles

Implementations MAY publish capability profiles.

Profiles SHALL provide a machine-readable summary of supported
capabilities.

------------------------------------------------------------------------

## 8. Versioning

Capability declarations SHOULD include version information.

Version identifiers SHALL supplement, but SHALL NOT replace, capability
analysis.

------------------------------------------------------------------------

## 9. Capability Discovery

Implementations SHOULD expose capability declarations programmatically.

Discovery mechanisms are implementation specific.

------------------------------------------------------------------------

## 10. Extensions

Extensions MAY define additional capabilities.

Extension capabilities SHALL use namespaced identifiers and SHALL
preserve standardized semantics.

------------------------------------------------------------------------

## 11. Validation

Implementations SHALL validate:

-   declaration structure
-   identifier consistency
-   version metadata
-   capability references

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 12. Conformance

A conforming implementation SHALL:

-   publish an accurate capability declaration
-   preserve declared capability semantics
-   reject invalid capability declarations through standardized
    Diagnostics

------------------------------------------------------------------------

## 13. Summary

The Engine Capability Model provides a standardized mechanism for
describing execution engine features independently of any specific
runtime.

By separating capability declarations from transformation semantics,
DTCS enables planners and compilers to target multiple execution engines
while preserving identical observable behavior and contractual
guarantees.

---

# Chapter 15 --- Compilation

## 1. Purpose

This chapter defines the normative compilation model for the Data
Transformation Contract Standard (DTCS).

Compilation transforms a validated **Transformation Plan** into one or
more backend-specific **Execution Plans**. Compilation SHALL preserve
the semantic meaning, contractual guarantees, logical types, and lineage
defined by the originating Transformation Contract.

------------------------------------------------------------------------

## 2. Design Goals

The compilation model SHALL be:

-   implementation independent
-   deterministic
-   capability aware
-   semantically preserving
-   extensible
-   portable

Compilation SHALL target execution engines without modifying
transformation semantics.

------------------------------------------------------------------------

## 3. Compilation Model

Compilation consumes:

-   a validated Transformation Plan
-   an Engine Capability Model
-   applicable profiles and extensions

Compilation produces one or more Execution Plans suitable for a specific
execution engine.

------------------------------------------------------------------------

## 4. Compiler Responsibilities

A conforming compiler SHALL:

-   evaluate engine capabilities
-   preserve observable behavior
-   preserve logical types
-   preserve lineage
-   preserve contractual guarantees
-   generate standardized Diagnostics

------------------------------------------------------------------------

## 5. Execution Plan

An Execution Plan is a backend-specific representation of a
Transformation Plan.

Execution Plans MAY differ structurally across engines but SHALL remain
semantically equivalent.

Execution Plans SHALL NOT become the normative representation of a
transformation.

------------------------------------------------------------------------

## 6. Capability Evaluation

Compilers SHALL evaluate required Transformation Plan features against
the selected Engine Capability Model.

Unsupported mandatory capabilities SHALL prevent successful compilation
unless otherwise permitted by a DTCS profile.

------------------------------------------------------------------------

## 7. Semantic Preservation

Compilation SHALL preserve:

-   Semantic Actions
-   Expressions
-   Rules
-   Preconditions
-   Postconditions
-   Lineage
-   Determinism where declared

Backend optimizations SHALL NOT alter observable semantics.

------------------------------------------------------------------------

## 8. Multiple Targets

A Transformation Plan MAY be compiled for multiple execution engines.

Each generated Execution Plan SHALL preserve equivalent observable
behavior.

------------------------------------------------------------------------

## 9. Optimization Boundary

Optimization MAY occur before, during, or after compilation provided
semantic equivalence is maintained.

Compilation SHALL NOT depend upon a particular optimization strategy.

------------------------------------------------------------------------

## 10. Diagnostics

Compilation failures SHALL produce standardized Diagnostics.

Diagnostics SHOULD identify:

-   unsupported capabilities
-   invalid plans
-   extension incompatibilities
-   compilation failures

------------------------------------------------------------------------

## 11. Validation

Compilers SHALL validate generated Execution Plans for structural
completeness and engine compatibility before execution.

------------------------------------------------------------------------

## 12. Extensibility

Compilation MAY be extended through compiler plugins and standardized
extensions.

Extensions SHALL preserve standardized DTCS semantics.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS compiler SHALL:

-   consume validated Transformation Plans
-   produce valid Execution Plans
-   preserve semantic meaning
-   preserve contractual guarantees
-   reject un-compilable plans through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS compilation model defines the standardized process for lowering
a Transformation Plan into backend-specific Execution Plans.

By separating semantic planning from engine-specific compilation, DTCS
enables independent compiler implementations to target diverse execution
environments while preserving identical observable behavior, lineage,
logical types, and contractual guarantees.

---

# Chapter 16 --- Runtime

## 1. Purpose

This chapter defines the normative Runtime Model for the Data
Transformation Contract Standard (DTCS).

A Runtime executes one or more Execution Plans produced from a validated
Transformation Plan. Execution SHALL preserve the semantic meaning,
contractual guarantees, logical types, and lineage defined by the
originating Transformation Contract.

------------------------------------------------------------------------

## 2. Design Goals

The Runtime Model SHALL be:

-   implementation independent
-   deterministic where declared
-   capability aware
-   observable
-   extensible
-   semantically preserving

The Runtime SHALL execute transformations without redefining DTCS
semantics.

------------------------------------------------------------------------

## 3. Runtime Model

A Runtime consumes:

-   one or more validated Execution Plans
-   declared inputs
-   runtime parameters
-   implementation-specific resources

A Runtime produces the declared outputs and any standardized
Diagnostics.

Execution SHALL operate within the semantic boundaries established by
the Transformation Contract.

------------------------------------------------------------------------

## 4. Runtime Responsibilities

A conforming Runtime SHALL:

-   execute valid Execution Plans
-   preserve observable behavior
-   preserve logical types
-   preserve lineage
-   enforce preconditions and postconditions
-   generate standardized Diagnostics

------------------------------------------------------------------------

## 5. Execution Semantics

Execution SHALL preserve:

-   Semantic Actions
-   Expressions
-   Rules
-   contractual guarantees
-   declared determinism
-   declared interfaces

Execution SHALL NOT introduce observable semantic changes.

------------------------------------------------------------------------

## 6. Runtime Context

The Runtime MAY maintain implementation-specific execution state.

Implementation-specific state SHALL NOT alter the semantic meaning of a
Transformation Contract unless explicitly permitted by a DTCS profile.

------------------------------------------------------------------------

## 7. Inputs and Outputs

The Runtime SHALL validate required inputs before execution.

Upon successful completion, the Runtime SHALL produce outputs that
satisfy all declared postconditions.

------------------------------------------------------------------------

## 8. Failure Handling

Execution failures SHALL produce standardized Diagnostics.

Failures MAY include:

-   precondition violations
-   runtime resource failures
-   unsupported capabilities
-   execution errors

Failure reporting SHALL preserve diagnostic information without altering
contract semantics.

------------------------------------------------------------------------

## 9. Determinism

When a Transformation Contract declares deterministic execution,
identical valid inputs under equivalent execution contexts SHALL produce
semantically equivalent outputs.

Implementations SHALL identify declared sources of non-determinism where
applicable.

------------------------------------------------------------------------

## 10. Observability

Implementations MAY expose runtime metrics, tracing, logging, and
execution statistics.

Observability data is informative unless otherwise standardized by a
DTCS profile.

Observability SHALL NOT redefine transformation semantics.

------------------------------------------------------------------------

## 11. Runtime Validation

The Runtime SHALL verify:

-   Execution Plan integrity
-   input availability
-   precondition satisfaction
-   required runtime capabilities

Validation failures SHALL prevent execution.

------------------------------------------------------------------------

## 12. Extensions

Runtime behavior MAY be extended through the DTCS Extensibility Model.

Extensions SHALL preserve standardized semantics and SHALL NOT redefine
mandatory Runtime behavior.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS Runtime SHALL:

-   execute valid Execution Plans
-   preserve semantic meaning
-   preserve contractual guarantees
-   preserve logical types
-   preserve lineage
-   report runtime failures through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS Runtime Model defines the standardized execution behavior for
Transformation Contracts.

By separating execution from planning, optimization, and compilation,
DTCS enables multiple independent runtimes to execute equivalent
Execution Plans while preserving identical observable behavior,
contractual guarantees, logical types, and lineage across heterogeneous
execution environments.

---

# Chapter 17 --- Semantic Actions

## 1. Purpose

This chapter defines the normative Semantic Action model for the Data
Transformation Contract Standard (DTCS).

Semantic Actions are the fundamental operations that modify the logical
state of data within a Transformation Contract. They define **what
transformation occurs**, independent of **how** an implementation
performs that transformation.

Semantic Actions SHALL be the only standardized mechanism for modifying
datasets within DTCS.

------------------------------------------------------------------------

## 2. Design Goals

The Semantic Action model SHALL be:

-   implementation independent
-   deterministic where declared
-   composable
-   machine readable
-   analyzable
-   extensible

Semantic Actions SHALL define behavior through semantics rather than
algorithms.

------------------------------------------------------------------------

## 3. Semantic Action Model

A Semantic Action is a declarative operation with well-defined semantic
meaning.

Each Semantic Action SHALL define:

-   identifier
-   semantic definition
-   input requirements
-   output behavior
-   logical type behavior
-   lineage behavior
-   determinism characteristics
-   validation requirements

------------------------------------------------------------------------

## 4. Action Identity

Every standardized Semantic Action SHALL possess a stable identifier.

Standard identifiers SHALL use the `dtcs:` namespace.

Vendor-defined Semantic Actions SHALL use namespaced identifiers as
defined in Chapter 22.

Identifiers SHALL remain stable across compatible specification
revisions.

------------------------------------------------------------------------

## 5. Standard Semantic Actions

DTCS defines a standard library of Semantic Actions.

Representative categories include:

-   projection
-   selection
-   transformation
-   aggregation
-   grouping
-   joining
-   sorting
-   union
-   partitioning
-   filtering

Future versions MAY add Semantic Actions through backward-compatible
revisions.

------------------------------------------------------------------------

## 6. Composition

Transformation Contracts MAY compose multiple Semantic Actions.

Composition SHALL preserve the semantics of each constituent action.

The resulting Transformation Plan SHALL represent the composed semantics
without semantic loss.

------------------------------------------------------------------------

## 7. Type Semantics

Each Semantic Action SHALL define its effect on logical types.

Semantic Actions SHALL preserve or transform logical types only as
explicitly defined.

Implicit type changes are prohibited.

------------------------------------------------------------------------

## 8. Lineage Semantics

Each Semantic Action SHALL define its effect on lineage.

Lineage behavior SHALL describe how information flows from inputs to
outputs.

Lineage semantics SHALL be preserved throughout planning, compilation,
and execution.

------------------------------------------------------------------------

## 9. Determinism

Each Semantic Action SHALL declare whether it is deterministic.

Deterministic Semantic Actions SHALL produce semantically equivalent
outputs for equivalent inputs under equivalent execution contexts.

------------------------------------------------------------------------

## 10. Validation

Implementations SHALL validate:

-   Semantic Action identity
-   required parameters
-   logical type compatibility
-   composition rules
-   extension compatibility

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Optimization

Implementations MAY optimize Semantic Actions.

Optimization SHALL preserve:

-   observable behavior
-   logical types
-   lineage
-   contractual guarantees

Optimization SHALL NOT redefine Semantic Action semantics.

------------------------------------------------------------------------

## 12. Extensions

Organizations MAY define additional Semantic Actions.

Extension Semantic Actions SHALL:

-   use namespaced identifiers
-   define complete semantic behavior
-   define lineage behavior
-   define type behavior
-   preserve interoperability

Extensions SHALL NOT redefine standardized Semantic Actions.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   recognize standardized Semantic Actions
-   preserve Semantic Action semantics
-   preserve declared lineage behavior
-   preserve logical type behavior
-   reject invalid Semantic Actions through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

Semantic Actions are the fundamental semantic building blocks of DTCS.

By defining dataset transformations in terms of standardized,
implementation-independent operations, DTCS enables planners,
optimizers, compilers, and runtimes to preserve identical observable
behavior while supporting diverse execution technologies and independent
implementations.

---

# Chapter 18 --- Function Model

## 1. Purpose

This chapter defines the normative Function Model for the Data
Transformation Contract Standard (DTCS).

Functions provide reusable computations that MAY be invoked by
Expressions. Functions compute values but SHALL NOT directly modify
datasets. Dataset modification SHALL occur only through Semantic
Actions.

------------------------------------------------------------------------

## 2. Design Goals

The Function Model SHALL be:

-   implementation independent
-   strongly typed
-   machine readable
-   composable
-   deterministic where declared
-   extensible

Function semantics SHALL be preserved across all conforming
implementations.

------------------------------------------------------------------------

## 3. Function Model

A Function is a named computation with a well-defined semantic meaning.

Every Function SHALL define:

-   identifier
-   parameters
-   return type
-   null behavior
-   determinism characteristics
-   semantic definition

Functions SHALL evaluate to exactly one logical value.

------------------------------------------------------------------------

## 4. Function Identity

Every standardized Function SHALL possess a stable identifier.

Standard identifiers SHALL use the `dtcs:` namespace.

Vendor-defined Functions SHALL use namespaced identifiers as defined in
Chapter 22.

Identifiers SHALL remain stable across compatible DTCS revisions.

------------------------------------------------------------------------

## 5. Parameters

Every Function SHALL declare:

-   parameter order
-   parameter types
-   cardinality
-   optional parameters, where applicable

Argument validation SHALL occur before evaluation.

------------------------------------------------------------------------

## 6. Return Types

Every Function SHALL declare exactly one logical return type.

Return types SHALL conform to the DTCS Type System defined in Chapter 4.

------------------------------------------------------------------------

## 7. Null Semantics

Every Function SHALL explicitly define its behavior when one or more
arguments are null.

Implementations SHALL preserve declared null semantics.

Implicit null handling is prohibited.

------------------------------------------------------------------------

## 8. Determinism

Every Function SHALL declare whether it is deterministic.

Deterministic Functions SHALL produce semantically equivalent results
for equivalent inputs.

------------------------------------------------------------------------

## 9. Evaluation

Functions SHALL be evaluated within the Expression evaluation model
defined in Chapter 8.

Function evaluation SHALL NOT modify datasets or observable
transformation state.

------------------------------------------------------------------------

## 10. Validation

Implementations SHALL validate:

-   Function identity
-   parameter count
-   parameter types
-   return type consistency
-   null behavior

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Optimization

Implementations MAY optimize Function evaluation.

Optimization SHALL preserve:

-   semantic meaning
-   return values
-   null behavior
-   determinism

------------------------------------------------------------------------

## 12. Extensions

Organizations MAY define additional Functions.

Extension Functions SHALL:

-   use namespaced identifiers
-   define complete semantic behavior
-   define parameter and return types
-   define null semantics
-   preserve interoperability

Extensions SHALL NOT redefine standardized Functions.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   recognize standardized Functions
-   preserve Function semantics
-   preserve declared type behavior
-   preserve null semantics
-   reject invalid Function invocations through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS Function Model defines reusable, implementation-independent
computations for use within Expressions.

By separating value computation from dataset transformation, DTCS
enables planners, analyzers, optimizers, compilers, and runtimes to
preserve consistent semantic behavior while supporting multiple
execution technologies.

---

# Chapter 19 --- Rule Model

## 1. Purpose

This chapter defines the normative Rule Model for the Data
Transformation Contract Standard (DTCS).

Rules express declarative invariants that SHALL be evaluated before,
during, or after transformation execution. Rules define conditions that
MUST hold for a Transformation Contract without prescribing
implementation algorithms.

Rules SHALL validate or constrain behavior. Rules SHALL NOT directly
modify datasets.

------------------------------------------------------------------------

## 2. Design Goals

The Rule Model SHALL be:

-   implementation independent
-   declarative
-   deterministic
-   machine readable
-   analyzable
-   extensible

Rule semantics SHALL be preserved across all conforming implementations.

------------------------------------------------------------------------

## 3. Rule Model

A Rule is a declarative assertion that evaluates to either satisfied or
violated.

Every Rule SHALL define:

-   identifier
-   semantic definition
-   evaluation phase
-   evaluation scope
-   failure behavior
-   diagnostic behavior

Rule evaluation SHALL NOT alter transformation semantics.

------------------------------------------------------------------------

## 4. Rule Identity

Every standardized Rule SHALL possess a stable identifier.

Standard Rule identifiers SHALL use the `dtcs:` namespace.

Vendor-defined Rules SHALL use namespaced identifiers as defined in
Chapter 22.

Identifiers SHALL remain stable across compatible DTCS revisions.

------------------------------------------------------------------------

## 5. Evaluation Phases

Rules MAY be evaluated:

-   before execution (preconditions)
-   during execution
-   after execution (postconditions)

The evaluation phase SHALL be explicitly defined.

------------------------------------------------------------------------

## 6. Evaluation Scope

Rules MAY apply to:

-   Transformation Contracts
-   inputs
-   outputs
-   Expressions
-   Semantic Actions
-   Transformation Plans
-   Execution Plans

The scope of every Rule SHALL be explicitly declared.

------------------------------------------------------------------------

## 7. Rule Outcomes

Rule evaluation SHALL produce one of the following outcomes:

-   satisfied
-   violated
-   indeterminate (where explicitly permitted)

Violation of a mandatory Rule SHALL produce a standardized Diagnostic.

------------------------------------------------------------------------

## 8. Type Integration

Rules SHALL evaluate values according to the DTCS Type System defined in
Chapter 4.

Implementations SHALL preserve logical type semantics during Rule
evaluation.

------------------------------------------------------------------------

## 9. Determinism

Every Rule SHALL declare whether it is deterministic.

Deterministic Rules SHALL produce semantically equivalent results for
equivalent inputs under equivalent evaluation contexts.

------------------------------------------------------------------------

## 10. Validation

Implementations SHALL validate:

-   Rule identity
-   evaluation scope
-   evaluation phase
-   referenced objects
-   type consistency

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Optimization

Implementations MAY optimize Rule evaluation.

Optimization SHALL preserve:

-   Rule semantics
-   evaluation outcomes
-   diagnostic behavior

Optimization SHALL NOT redefine Rule meaning.

------------------------------------------------------------------------

## 12. Extensions

Organizations MAY define additional Rules.

Extension Rules SHALL:

-   use namespaced identifiers
-   define complete semantic behavior
-   define evaluation scope
-   define evaluation phase
-   preserve interoperability

Extensions SHALL NOT redefine standardized Rules.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   recognize standardized Rules
-   preserve Rule semantics
-   preserve declared evaluation behavior
-   reject invalid Rule definitions through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS Rule Model provides a standardized, declarative mechanism for
expressing contractual invariants independently of implementation
technologies.

By separating validation logic from transformation logic, DTCS enables
consistent governance, analysis, planning, compilation, and runtime
enforcement while preserving identical observable semantics across
conforming implementations.

---

# Chapter 20 --- Diagnostics

## 1. Purpose

This chapter defines the normative Diagnostics Model for the Data
Transformation Contract Standard (DTCS).

Diagnostics communicate errors, warnings, informational messages, and
other standardized observations produced during DTCS processing.
Diagnostics provide implementation-independent reporting and SHALL NOT
alter the semantic meaning of a Transformation Contract.

Unless explicitly stated otherwise, the requirements in this chapter are
normative.

------------------------------------------------------------------------

## 2. Design Goals

The Diagnostics Model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   actionable
-   extensible
-   semantically neutral

Diagnostics SHALL report processing outcomes without changing processing
behavior.

------------------------------------------------------------------------

## 3. Diagnostic Model

A Diagnostic is a structured record describing an observation produced
during parsing, validation, analysis, planning, optimization,
compilation, or execution.

Every Diagnostic SHALL define:

-   diagnostic identifier
-   severity
-   processing stage
-   message
-   affected object
-   diagnostic category

Diagnostics MAY include remediation guidance and implementation-specific
context.

------------------------------------------------------------------------

## 4. Diagnostic Identity

Every standardized Diagnostic SHALL possess a stable identifier.

Standard identifiers SHALL use the `dtcs:` namespace.

Vendor-defined Diagnostics SHALL use namespaced identifiers as defined
in Chapter 22.

Identifiers SHALL remain stable across compatible DTCS revisions.

------------------------------------------------------------------------

## 5. Severity Levels

DTCS defines the following standardized severity levels:

-   Error
-   Warning
-   Information

Profiles MAY define additional severity levels provided interoperability
is preserved.

Severity SHALL NOT change the semantic meaning of a Transformation
Contract.

------------------------------------------------------------------------

## 6. Processing Stages

Diagnostics MAY be produced during:

-   document parsing
-   Canonical Object Model construction
-   validation
-   analysis
-   planning
-   optimization
-   compilation
-   execution

The originating processing stage SHALL be identified.

------------------------------------------------------------------------

## 7. Diagnostic Categories

Standard categories include:

-   syntax
-   structure
-   type
-   reference
-   semantic
-   compatibility
-   capability
-   runtime
-   extension

Future revisions MAY introduce additional categories.

------------------------------------------------------------------------

## 8. Reporting Requirements

Implementations SHALL report every mandatory processing failure through
one or more Diagnostics.

Implementations MAY continue processing after non-fatal failures to
report additional Diagnostics.

------------------------------------------------------------------------

## 9. Determinism

Equivalent processing performed on equivalent Transformation Contracts
SHALL produce semantically equivalent Diagnostics.

Implementations MAY vary message wording provided the standardized
identifier, severity, and meaning are preserved.

------------------------------------------------------------------------

## 10. Validation

Diagnostic definitions SHALL be validated for:

-   identifier uniqueness
-   required fields
-   severity correctness
-   processing stage consistency

Invalid diagnostic definitions SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Extensions

Organizations MAY define additional Diagnostics.

Extension Diagnostics SHALL:

-   use namespaced identifiers
-   preserve standardized severity semantics
-   preserve interoperability

Extensions SHALL NOT redefine standardized Diagnostic identifiers.

------------------------------------------------------------------------

## 12. Conformance

A conforming DTCS implementation SHALL:

-   generate standardized Diagnostics for mandatory failures
-   preserve Diagnostic identifiers and severity
-   identify the originating processing stage
-   reject invalid Diagnostic definitions through standardized
    Diagnostics

------------------------------------------------------------------------

## 13. Summary

The DTCS Diagnostics Model provides a standardized mechanism for
communicating processing outcomes throughout the DTCS lifecycle.

By separating diagnostic reporting from transformation semantics, DTCS
enables consistent tooling, automated validation, conformance testing,
and interoperable implementations while preserving the contract-first
architecture of the specification.

---

# Chapter 21 --- Extensibility

## 1. Purpose

This chapter defines the normative Extensibility Model for the Data
Transformation Contract Standard (DTCS).

The Extensibility Model enables organizations to introduce new
capabilities while preserving interoperability with conforming DTCS
implementations. Extensions SHALL augment standardized behavior and
SHALL NOT redefine the semantics established by this specification.

Unless explicitly identified as informative, the requirements in this
chapter are normative.

------------------------------------------------------------------------

## 2. Design Goals

The Extensibility Model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   interoperable
-   version aware
-   forward extensible

Extensions SHALL preserve the contract-first and semantic-first
principles of DTCS.

------------------------------------------------------------------------

## 3. Extension Model

An extension is a standardized or vendor-defined addition to the DTCS
object model.

Extensions MAY introduce:

-   metadata
-   logical types
-   Semantic Actions
-   Functions
-   Rules
-   engine capabilities
-   profiles
-   registries

Extensions SHALL define complete semantic behavior for every construct
they introduce.

------------------------------------------------------------------------

## 4. Namespace Requirements

Every extension SHALL use a stable namespace identifier.

The `dtcs:` namespace is reserved for standardized extensions.

Vendor-defined extensions SHALL use globally unique namespaced
identifiers.

Namespace identifiers SHALL remain stable across compatible revisions.

------------------------------------------------------------------------

## 5. Extension Identity

Every extension SHALL define:

-   extension identifier
-   namespace
-   version
-   compatibility requirements
-   semantic definition

Extension identity SHALL be independent of serialization format.

------------------------------------------------------------------------

## 6. Extension Categories

Standard extension categories MAY include:

-   semantic
-   type
-   function
-   rule
-   runtime
-   compiler
-   capability
-   metadata

Future revisions MAY define additional categories.

------------------------------------------------------------------------

## 7. Compatibility

Extensions SHALL declare compatibility requirements.

Mandatory extensions unsupported by an implementation SHALL prevent
successful processing.

Optional extensions MAY be ignored only when doing so does not change
observable semantics.

------------------------------------------------------------------------

## 8. Processing Requirements

Implementations SHALL preserve recognized extension semantics.

Unknown optional extensions MAY be preserved without interpretation.

Unknown mandatory extensions SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 9. Validation

Implementations SHALL validate:

-   namespace correctness
-   identifier uniqueness
-   version metadata
-   semantic completeness
-   compatibility declarations

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 10. Registry Integration

Standardized extensions SHOULD be registered through the DTCS registry
process.

Registry identifiers SHALL remain stable.

Registration procedures are defined by DTCS governance.

------------------------------------------------------------------------

## 11. Evolution

Extensions MAY evolve independently of the core specification.

Evolution SHALL preserve published compatibility guarantees.

Breaking changes SHALL require a new major extension version.

------------------------------------------------------------------------

## 12. Conformance

A conforming DTCS implementation SHALL:

-   recognize standardized extension metadata
-   preserve supported extension semantics
-   report unsupported mandatory extensions through standardized
    Diagnostics
-   preserve extension identity and namespace information

------------------------------------------------------------------------

## 13. Summary

The DTCS Extensibility Model provides a standardized framework for
evolving the DTCS ecosystem without compromising interoperability.

By defining stable namespaces, explicit compatibility requirements,
deterministic processing rules, and standardized extension metadata,
DTCS enables independent innovation while preserving the semantic
integrity of Transformation Contracts across implementations.

---

# Chapter 22 --- Registries

## 1. Purpose

This chapter defines the normative Registry Model for the Data
Transformation Contract Standard (DTCS).

Registries provide authoritative, versioned catalogs of standardized
identifiers used throughout DTCS. Registries SHALL enable independent
implementations to interpret standardized constructs consistently
without redefining their semantics.

Unless explicitly identified as informative, every requirement in this
chapter is normative.

------------------------------------------------------------------------

## 2. Design Goals

The Registry Model SHALL be:

-   implementation independent
-   machine readable
-   deterministic
-   versioned
-   extensible
-   interoperable

Registries SHALL preserve long-term identifier stability.

------------------------------------------------------------------------

## 3. Registry Model

A registry is an authoritative collection of standardized entries
governed by DTCS.

Registry entries MAY define:

-   Semantic Actions
-   Functions
-   Rules
-   Logical Types
-   Diagnostics
-   Engine Capabilities
-   Profiles
-   Extension Namespaces
-   Other standardized identifiers

Each registry SHALL define the semantic meaning of every registered
entry.

------------------------------------------------------------------------

## 4. Registry Identity

Every registry SHALL possess:

-   registry identifier
-   registry version
-   governing specification
-   publication status

Registry identifiers SHALL remain stable across compatible revisions.

------------------------------------------------------------------------

## 5. Registry Entries

Every registry entry SHALL define:

-   stable identifier
-   human-readable name
-   semantic definition
-   version
-   status
-   compatibility information
-   normative references

Registry entries MAY include informative examples.

------------------------------------------------------------------------

## 6. Identifier Stability

Published standardized identifiers SHALL NOT be reassigned.

Deprecated identifiers SHALL remain reserved.

Removed identifiers SHALL NOT be reused for different semantics.

------------------------------------------------------------------------

## 7. Versioning

Registries SHALL evolve through versioned publications.

Additive entries SHOULD preserve backward compatibility.

Breaking changes SHALL require an appropriate registry version
increment.

------------------------------------------------------------------------

## 8. Processing Requirements

Implementations SHALL recognize the standardized identifiers they claim
to support.

Unknown standardized identifiers SHALL generate standardized Diagnostics
when required for processing.

Implementations MAY preserve unknown optional identifiers without
interpretation.

------------------------------------------------------------------------

## 9. Validation

Registry processing SHALL validate:

-   identifier uniqueness
-   namespace correctness
-   semantic completeness
-   version consistency
-   status consistency

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 10. Publication Status

Registry entries SHOULD declare one of the following statuses:

-   Draft
-   Experimental
-   Standard
-   Deprecated
-   Obsolete

Status values SHALL NOT modify semantic meaning.

------------------------------------------------------------------------

## 11. Governance

The DTCS governance process SHALL define:

-   registry publication
-   identifier allocation
-   review procedures
-   deprecation procedures
-   retirement procedures

Governance procedures are specified in Chapter 26.

------------------------------------------------------------------------

## 12. Conformance

A conforming DTCS implementation SHALL:

-   preserve standardized registry identifiers
-   interpret supported registry entries consistently
-   reject invalid registry definitions through standardized Diagnostics
-   preserve registry version information where applicable

------------------------------------------------------------------------

## 13. Summary

The DTCS Registry Model provides the authoritative mechanism for
managing standardized identifiers across the DTCS ecosystem.

By separating identifier governance from transformation semantics, DTCS
enables independent implementations to evolve while preserving
interoperability, long-term stability, and consistent interpretation of
standardized constructs.

---

# Chapter 23 --- Conformance

## 1. Purpose

This chapter defines the normative conformance model for the Data
Transformation Contract Standard (DTCS).

Conformance specifies the minimum requirements an implementation SHALL
satisfy to claim support for DTCS. Conformance requirements are
objective, testable, and implementation independent.

Unless explicitly identified as informative, all requirements in this
chapter are normative.

------------------------------------------------------------------------

## 2. Design Goals

The conformance model SHALL be:

-   objective
-   deterministic
-   implementation independent
-   testable
-   extensible
-   profile aware

Conformance SHALL evaluate observable behavior rather than
implementation strategy.

------------------------------------------------------------------------

## 3. Conformance Model

A DTCS implementation claims conformance by satisfying the mandatory
requirements applicable to its declared conformance profile.

An implementation SHALL NOT claim conformance if mandatory requirements
are knowingly omitted.

------------------------------------------------------------------------

## 4. Conformance Units

The following implementation classes MAY claim conformance:

-   Parser
-   Validator
-   Analyzer
-   Planner
-   Optimizer
-   Compiler
-   Runtime
-   Integrated Platform

Each class SHALL identify its supported capabilities.

------------------------------------------------------------------------

## 5. Conformance Profiles

A conformance profile defines a coherent set of DTCS capabilities.

Profiles SHALL identify:

-   supported specification version
-   supported registries
-   supported extensions
-   implementation class
-   optional capabilities

Profiles SHALL be machine readable.

### 5.1 Semantic-family profiles

In addition to implementation-class profiles, DTCS defines
semantic-family profiles for portable relational transformations
(Appendix A.8 and Appendix A.9):

-   `dtcs:profile/portable-relational-kernel/1`
-   `dtcs:profile/portable-relational/1`
-   `dtcs:profile/portable-window/1`
-   `dtcs:profile/portable-complex-types/1`
-   `dtcs:profile/portable-relational-kernel/2`
-   `dtcs:profile/portable-relational/2`
-   `dtcs:profile/portable-string-advanced/1`
-   `dtcs:profile/portable-conversion/1`
-   `dtcs:profile/portable-statistics/1`
-   `dtcs:profile/portable-complex-values/1`
-   `dtcs:profile/portable-reshape/1`
-   `dtcs:profile/portable-relational-extended/1`
-   `dtcs:profile/portable-temporal-iana/1`
-   `dtcs:profile/portable-nondeterministic/1`
-   `dtcs:profile/portable-window/2`

An implementation MAY claim both an implementation class and one or more
semantic-family profiles. Capability declarations for compilers SHALL be
keyed by exact registry entries (identifier and version) and MAY include
signatures, semantic modes, limits, and known unsupported optional
features.

DTCS 3.0 capability declarations SHALL identify accepted Transformation Plan
protocol versions; complete and partial profile claims; supported entry and
semantic-mode versions; regex, Unicode, timezone-data, random-algorithm, and
format-grammar versions; applicable resource budgets; ordering and
deterministic-execution guarantees; and compile-time or runtime conditions.
Capability analysis SHALL report the narrowest unsupported requirement.

A semantic family SHALL NOT advance from Experimental to Standard until
at least two independent compilers pass that family's conformance
fixtures. False claims invalidate conformance.

------------------------------------------------------------------------

## 6. Mandatory Requirements

A conforming implementation SHALL:

-   preserve Transformation Contract semantics
-   preserve logical types
-   preserve lineage
-   preserve contractual guarantees
-   generate standardized Diagnostics for mandatory failures
-   process supported standardized identifiers consistently

------------------------------------------------------------------------

## 7. Optional Capabilities

Implementations MAY support optional capabilities.

Unsupported optional capabilities SHALL NOT invalidate conformance
unless required by the declared profile.

Optional capability support SHALL be accurately declared.

------------------------------------------------------------------------

## 8. Conformance Testing

Conformance SHALL be verified using objective test procedures.

Test suites SHOULD verify:

-   semantic preservation
-   validation behavior
-   compatibility analysis
-   planning
-   compilation
-   runtime behavior
-   diagnostic generation

Equivalent observable behavior constitutes a successful result.

------------------------------------------------------------------------

## 9. Capability Declaration

Every conforming implementation SHALL publish a machine-readable
capability declaration.

Capability declarations SHALL accurately describe supported DTCS
functionality.

False capability claims invalidate conformance.

------------------------------------------------------------------------

## 10. Version Conformance

An implementation SHALL identify the DTCS specification version to which
it conforms.

Conformance to one specification version SHALL NOT imply conformance to
another version.

------------------------------------------------------------------------

## 11. Extensions

Supported extensions SHALL be declared.

Unsupported mandatory extensions SHALL be reported through standardized
Diagnostics.

Extensions SHALL NOT redefine mandatory DTCS semantics.

------------------------------------------------------------------------

## 12. Conformance Reporting

Implementations SHOULD provide conformance reports suitable for
automated verification.

Reports MAY include:

-   implementation identity
-   supported profiles
-   capability declarations
-   test results
-   supported extensions

------------------------------------------------------------------------

## 13. Governance Relationship

The DTCS governance process defines publication and maintenance of
conformance profiles, test suites, and reference implementations.

------------------------------------------------------------------------

## 14. Conformance

An implementation SHALL claim DTCS conformance only if it satisfies all
mandatory requirements applicable to its declared implementation class
and conformance profile.

------------------------------------------------------------------------

## 15. Summary

The DTCS Conformance Model establishes an objective,
implementation-independent framework for verifying interoperability
across the DTCS ecosystem.

By standardizing implementation classes, profiles, capability
declarations, and testable requirements, DTCS enables independent
implementations to demonstrate interoperable behavior while preserving
the semantic integrity of Transformation Contracts.

---

# Chapter 24 --- Security Considerations

## 1. Purpose

This chapter defines the normative security model for the Data
Transformation Contract Standard (DTCS).

DTCS specifies the semantics of data transformations. It does not
prescribe a complete security architecture. This chapter identifies the
security requirements that SHALL be preserved by conforming
implementations and distinguishes them from implementation-specific
security controls.

Unless explicitly identified as informative, all requirements in this
chapter are normative.

------------------------------------------------------------------------

## 2. Design Goals

The DTCS security model SHALL be:

-   implementation independent
-   risk aware
-   deterministic
-   interoperable
-   extensible
-   semantically neutral

Security mechanisms SHALL protect DTCS processing without redefining
Transformation Contract semantics.

------------------------------------------------------------------------

## 3. Security Model

DTCS defines security requirements for processing Transformation
Contracts.

Implementations remain responsible for selecting authentication,
authorization, encryption, networking, infrastructure, and operational
controls appropriate to their environments.

Security controls SHALL NOT change the semantic meaning of a
Transformation Contract.

------------------------------------------------------------------------

## 4. Security Scope

This specification addresses:

-   contract integrity
-   processing integrity
-   extension trust
-   registry authenticity
-   capability declaration integrity
-   diagnostic confidentiality where applicable

This specification does not mandate specific cryptographic algorithms or
identity systems.

------------------------------------------------------------------------

## 5. Contract Integrity

Implementations SHOULD verify that Transformation Contracts have not
been modified unexpectedly.

Integrity verification mechanisms are implementation specific.

Failed integrity verification SHALL prevent trusted processing where
required by policy.

------------------------------------------------------------------------

## 6. Trusted Extensions

Implementations SHALL distinguish standardized extensions from
vendor-defined extensions.

Organizations SHOULD establish trust policies governing extension
acceptance.

Unsupported or untrusted mandatory extensions SHALL produce standardized
Diagnostics.

------------------------------------------------------------------------

## 7. Registry Trust

Implementations SHOULD obtain standardized registry information from
trusted sources.

Registry authenticity mechanisms are implementation specific.

Compromised registry data SHALL NOT be treated as authoritative.

------------------------------------------------------------------------

## 8. Execution Isolation

Execution environments SHOULD isolate Transformation Contract execution
from unrelated workloads where practical.

Isolation mechanisms are outside the scope of this specification.

------------------------------------------------------------------------

## 9. Sensitive Information

Transformation Contracts SHOULD avoid embedding sensitive credentials or
secrets.

Implementations SHOULD provide secure mechanisms for supplying runtime
secrets independently of the contract.

------------------------------------------------------------------------

## 10. Diagnostics

Diagnostics SHALL avoid exposing sensitive implementation details unless
explicitly authorized by local policy.

Diagnostic reporting SHALL preserve interoperability while minimizing
unnecessary disclosure.

------------------------------------------------------------------------

## 11. Extensions

Security-related extensions MAY introduce additional controls.

Extensions SHALL preserve standardized DTCS semantics and SHALL NOT
redefine mandatory security requirements established by this
specification.

------------------------------------------------------------------------

## 12. Conformance

A conforming DTCS implementation SHALL:

-   preserve Transformation Contract semantics when applying security
    controls
-   preserve registry and extension identity
-   report mandatory security-related processing failures through
    standardized Diagnostics
-   distinguish standardized behavior from implementation-specific
    security mechanisms

------------------------------------------------------------------------

## 13. Summary

The DTCS Security Model establishes implementation-independent security
requirements while deliberately avoiding dependence on specific security
technologies.

By separating semantic requirements from operational security controls,
DTCS enables secure processing across diverse environments while
preserving interoperability, portability, and the contract-first
architecture defined by this specification.

---

# Chapter 25 --- Versioning

## 1. Purpose

This chapter defines the normative Versioning Model for the Data
Transformation Contract Standard (DTCS).

The Versioning Model establishes how DTCS specifications, Transformation
Contracts, registries, profiles, extensions, and related artifacts
identify revisions over time. Version identifiers enable interoperable
evolution while remaining distinct from compatibility analysis.

Unless explicitly identified as informative, all requirements in this
chapter are normative.

------------------------------------------------------------------------

## 2. Design Goals

The Versioning Model SHALL be:

-   implementation independent
-   deterministic
-   machine readable
-   interoperable
-   extensible
-   stable

Version identifiers SHALL communicate revision identity without altering
the semantic meaning of a Transformation Contract.

------------------------------------------------------------------------

## 3. Versioning Scope

DTCS versioning applies to:

-   DTCS specifications
-   Transformation Contracts
-   registries
-   profiles
-   extensions
-   capability declarations
-   implementation conformance profiles

Each artifact SHALL identify its version independently.

------------------------------------------------------------------------

## 4. Version Identity

A version identifier uniquely identifies a published revision of an
artifact.

A version identifier SHALL:

-   uniquely identify a revision
-   remain stable after publication
-   be treated as immutable

Published version identifiers SHALL NOT be reused.

------------------------------------------------------------------------

## 5. Specification Versions

Every DTCS specification SHALL declare its specification version.

Implementations claiming conformance SHALL identify the specification
version they implement.

Conformance to one specification version SHALL NOT imply conformance to
another.

An implementation claiming DTCS 3.0 conformance SHALL identify whether it
accepts `dtcs.transform-plan/1`, `dtcs.transform-plan/2`, or both. A DTCS 3.0
Planner, Compiler, Runtime, or Integrated Platform that accepts portable plans
SHALL accept both versions and SHALL preserve semantics during version 1 to
version 2 migration.

------------------------------------------------------------------------

## 6. Contract Versions

A Transformation Contract MAY define a contract version.

Contract version identifiers SHALL distinguish revisions of the same
logical contract.

Contract versions SHALL NOT be used as the sole basis for compatibility
decisions.

Compatibility SHALL be evaluated according to Chapter 11.

------------------------------------------------------------------------

## 7. Registry and Extension Versions

Registries and extensions SHALL define independent version identifiers.

A registry or extension MAY evolve without requiring a new DTCS
specification version, provided normative compatibility requirements are
satisfied.

------------------------------------------------------------------------

## 8. Version Evolution

Version identifiers SHOULD reflect meaningful published revisions.

Editorial revisions, additive revisions, and breaking revisions SHOULD
be distinguishable according to DTCS governance policies.

Specific version numbering conventions are implementation or governance
decisions unless standardized by a DTCS profile.

------------------------------------------------------------------------

## 9. Version Resolution

Implementations SHALL preserve declared version identifiers during
parsing, validation, planning, compilation, and execution.

Version resolution SHALL be deterministic.

Version resolution SHALL NOT modify Transformation Contract semantics.

------------------------------------------------------------------------

## 10. Validation

Implementations SHALL validate:

-   version identifier presence where required
-   version identifier format
-   version consistency
-   conflicting version declarations

Validation failures SHALL produce standardized Diagnostics.

------------------------------------------------------------------------

## 11. Relationship to Compatibility

Version identity and compatibility are distinct concepts.

Two artifacts MAY share the same version while remaining incompatible
under Chapter 11.

Two artifacts MAY possess different version identifiers while remaining
fully compatible.

Implementations SHALL evaluate compatibility independently of version
identifiers.

------------------------------------------------------------------------

## 12. Extensions

Extensions SHALL declare their own version identifiers.

Extension versioning SHALL preserve namespace identity and published
compatibility guarantees.

------------------------------------------------------------------------

## 13. Conformance

A conforming DTCS implementation SHALL:

-   preserve declared version identifiers
-   distinguish version identity from compatibility
-   validate required version metadata
-   report invalid version information through standardized Diagnostics

------------------------------------------------------------------------

## 14. Summary

The DTCS Versioning Model provides a stable, implementation-independent
framework for identifying revisions of DTCS artifacts.

By separating revision identity from semantic compatibility, DTCS
enables controlled evolution of specifications, Transformation
Contracts, registries, extensions, and implementations while preserving
interoperability and long-term architectural stability.

---

# Chapter 26 --- Governance

## 1. Purpose

This chapter defines the normative Governance Model for the Data
Transformation Contract Standard (DTCS).

The Governance Model establishes the processes by which the DTCS
specification, registries, profiles, extensions, and related standards
evolve over time. Governance SHALL preserve interoperability, semantic
stability, transparency, and long-term sustainability.

Unless explicitly identified as informative, all requirements in this
chapter are normative.

------------------------------------------------------------------------

## 2. Design Goals

The Governance Model SHALL be:

-   transparent
-   implementation independent
-   deterministic
-   community driven
-   extensible
-   accountable

Governance SHALL preserve the contract-first and semantic-first
principles established by this specification.

------------------------------------------------------------------------

## 3. Governance Scope

Governance applies to:

-   the DTCS specification
-   normative companion specifications
-   registries
-   conformance profiles
-   extension namespaces
-   reference implementations
-   conformance test suites

Each governed artifact SHALL define a responsible authority and a
published lifecycle.

------------------------------------------------------------------------

## 4. Governance Principles

The DTCS project SHALL be guided by the following principles:

-   semantic stability
-   backward compatibility where practical
-   open participation
-   documented decision making
-   objective technical evaluation
-   long-term identifier stability

------------------------------------------------------------------------

## 5. Change Management

All normative changes SHALL be proposed through a documented change
process.

Each proposal SHALL include:

-   problem statement
-   proposed solution
-   compatibility impact
-   migration considerations
-   affected artifacts

Normative changes SHALL undergo technical review before adoption.

------------------------------------------------------------------------

## 6. Specification Lifecycle

Every specification revision SHOULD progress through defined publication
states, including:

-   Draft
-   Candidate
-   Stable
-   Deprecated
-   Obsolete

Publication state SHALL be explicitly identified.

------------------------------------------------------------------------

## 7. Registry Governance

Registry publication SHALL follow documented review procedures.

Registry authorities SHALL preserve identifier stability and SHALL NOT
reuse published standardized identifiers for different semantics.

------------------------------------------------------------------------

## 8. Extension Governance

Standardized extensions SHOULD undergo technical review before
publication.

Extension namespaces SHALL be managed to prevent identifier conflicts.

Mandatory extensions SHALL publish compatibility requirements.

------------------------------------------------------------------------

## 9. Conformance Governance

Governance SHALL maintain:

-   conformance profiles
-   conformance test suites
-   reference test vectors
-   implementation guidance

Conformance criteria SHALL remain objective and publicly documented.

------------------------------------------------------------------------

## 10. Deprecation and Retirement

Governance SHALL define documented procedures for:

-   deprecation
-   retirement
-   replacement
-   migration

Deprecated artifacts SHOULD remain available for historical reference
where practical.

------------------------------------------------------------------------

## 11. Appeals and Review

Governance SHOULD provide a documented mechanism for requesting
technical review of published decisions.

Review procedures SHALL be transparent and publicly documented.

------------------------------------------------------------------------

## 12. Publication

Normative publications SHALL identify:

-   publication date
-   version
-   publication status
-   governing authority
-   applicable editorial baseline

Published documents SHALL be immutable.

------------------------------------------------------------------------

## 13. Conformance

A DTCS-governed artifact SHALL comply with the governance procedures
applicable to its artifact type before being represented as an official
DTCS publication.

------------------------------------------------------------------------

## 14. Summary

The DTCS Governance Model provides the institutional framework required
to evolve the DTCS ecosystem while preserving semantic integrity,
interoperability, and long-term stability.

By standardizing change management, publication, registry
administration, extension review, conformance maintenance, and lifecycle
management, DTCS enables an open, sustainable standards ecosystem
capable of supporting independent implementations for many years.


------------------------------------------------------------------------

# Chapter 27 --- Rich Portable Analytics

## 1. Purpose

This chapter defines the DTCS 3.0 semantic families required for rich,
portable dataframe-style analytics. The requirements remain independent of
Python, SQL, PySpark, and any backend-native expression language.

## 2. Design Goals

Rich Portable Analytics SHALL:

-   preserve implementation independence and data-only plans
-   make each advanced semantic family independently discoverable and
    testable
-   define value-state, type, error, ordering, and determinism behavior
-   support safe lowering to dataframe, distributed, and SQL Engines
-   reject unsupported behavior instead of approximating it
-   keep validation and planning bounded without reading source rows

Rich Portable Analytics SHALL NOT standardize arbitrary SQL text, backend
expression strings, language ASTs, executable user code, serialized closures,
physical partition counts, cache placement, orchestration, storage, resource
acquisition, or unbounded schema discovery from runtime data.

## 3. Lambda Expressions

DTCS 3.0 adds the structured Expression node kind `lambda`. A lambda node
SHALL contain `parameters`, an ordered non-empty array of unique parameter
names, and `body`, one structured Expression node.

A lambda Expression SHALL:

-   be data-only and contain no executable object
-   reference only its parameters, literals, plan parameters, and outer
    fields explicitly permitted by the invoking Function
-   use registered Operators and Functions
-   obey plan depth, node, byte, and evaluation budgets
-   occur only as an argument to a Function whose registry entry declares
    lambda support

Lambda parameters use lexical scope. An inner parameter shadows an outer
parameter with the same name. A `fieldRef` with `scope: "lambda"` SHALL resolve
to the nearest enclosing lambda parameter. Implementations SHALL evaluate
lambda arguments in declared parameter order and SHALL NOT capture undeclared
host or Runtime state. Conditional Operators and Functions SHALL evaluate only
the selected branch when their registry semantics declare short-circuit
behavior.

## 4. String and Regular-Expression Profile

Profile `dtcs:profile/portable-string-advanced/1` defines advanced string and
regular-expression Functions.

String operations SHALL measure positions and lengths in Unicode code points
unless an entry explicitly declares another unit. Case conversion and case
folding SHALL be locale independent and SHALL declare the Unicode data
version. The profile contains the Functions `dtcs:trim`, `dtcs:ltrim`,
`dtcs:rtrim`, `dtcs:normalize_whitespace`, `dtcs:split`,
`dtcs:join_strings`, `dtcs:pad_left`, `dtcs:pad_right`, `dtcs:repeat`,
`dtcs:reverse`, `dtcs:translate`, `dtcs:position`, `dtcs:lower_unicode`,
`dtcs:upper_unicode`, and `dtcs:casefold`.

The profile also contains `dtcs:regex_matches`, `dtcs:regex_contains`,
`dtcs:regex_extract`, `dtcs:regex_extract_all`, `dtcs:regex_replace`, and
`dtcs:regex_split`. A plan using these Functions SHALL identify a registered
portable regex grammar and Unicode version. The grammar SHALL define flags,
capture numbering, zero-length matches, replacement references, and invalid
pattern behavior. Unsupported constructs SHALL fail capability analysis.
Implementations SHALL enforce declared regex complexity and input-length
budgets.

### 4.1 Portable regex grammar and Unicode version

Version 1 of the portable regex grammar is `dtcs-regex/1`. It uses the
regular-language subset implemented by RE2-style engines: literals, Unicode
scalar escapes, character classes, grouping, alternation, quantifiers, named
and numbered captures, anchors, and flags `i`, `m`, and `s`. Look-around,
backreferences, recursive patterns, conditional patterns, and embedded
host-language code SHALL be rejected. The Unicode data version is `15.1`.

Capture group zero is the complete match; numbered captures are assigned by
opening-parenthesis order. `regex_extract` returns null when no match exists;
`regex_extract_all` returns an empty list. `regex_replace` interprets `$0` and
`$1` through `$99` as capture references. For an empty match, scanning advances
by one Unicode code point after emitting that match. The maximum pattern length
is 16 KiB and the maximum input length is 1 MiB by default.

## 5. Conversion and Parsing Profile

Profile `dtcs:profile/portable-conversion/1` contains `dtcs:cast`,
`dtcs:try_cast`, `dtcs:parse_date`, `dtcs:parse_time`,
`dtcs:parse_datetime`, `dtcs:format_date`, `dtcs:format_datetime`,
`dtcs:parse_decimal`, and `dtcs:parse_boolean`.

`dtcs:cast` performs strict conversion and applies the declared error mode to
invalid input. `dtcs:try_cast` performs tolerant conversion and requires an
explicit invalid-result policy. Every conversion SHALL define overflow,
underflow, rounding, precision, null, missing, invalid, NaN, and infinity
behavior. Date and time format strings SHALL identify a DTCS format grammar;
they SHALL NOT silently use backend-native tokens. Decimal and boolean parsing
SHALL be locale independent unless an explicit registered locale mode is
present.

## 6. Statistical Aggregate Profile

Profile `dtcs:profile/portable-statistics/1` contains `dtcs:variance`,
`dtcs:stddev`, `dtcs:covariance`, `dtcs:correlation`, `dtcs:median`,
`dtcs:quantile`, `dtcs:first`, `dtcs:last`, `dtcs:collect_list`, and
`dtcs:collect_set`.

Variance, standard deviation, and covariance SHALL declare population or
sample mode. Correlation SHALL declare its method; Pearson is the standard
method in version 1 of this profile. Quantiles SHALL declare exact or an
explicitly identified approximate mode. First, last, and collection
aggregates SHALL declare ordering and null policies.

Every statistical entry SHALL define degrees of freedom, numeric promotion,
decimal handling, empty and all-null input, NaN and infinity behavior,
overflow, result nullability, and order sensitivity. Approximate modes SHALL
also declare algorithm identity, error bounds, merge behavior, and
determinism. Collection aggregates SHALL enforce an explicit size limit.

### 6.1 Exact statistics decision

The initial portable statistics profile supports exact computation only.
`quantile` SHALL use nearest-rank values sorted in ascending total numeric
order, with `p` in `[0, 1]`; `p = 0` selects the first value and `p = 1` the
last. Approximate quantiles SHALL fail capability analysis. Variance and
covariance SHALL use the numerically stable two-pass algorithm. Population mode
uses divisor `n`; sample mode uses `n - 1` and returns null below two valid
observations. Correlation is Pearson and returns null when either variance is
zero. Null, missing, and invalid inputs are excluded; NaN and infinity produce
invalid. Median is `quantile(p = 0.5)`. `first`, `last`, and `collect_list`
require declared ordering; `collect_set` uses total-value equality and
canonical ascending order. The default collection limit is 1,000,000 values.

## 7. Complex-Value Profile

Profile `dtcs:profile/portable-complex-values/1` contains constructors
`dtcs:list`, `dtcs:map`, `dtcs:object`, and `dtcs:tuple`; accessors and
transforms `dtcs:size`, `dtcs:list_contains`, `dtcs:list_position`,
`dtcs:list_slice`, `dtcs:list_concat`, `dtcs:list_sort`, `dtcs:map_keys`,
`dtcs:map_values`, `dtcs:map_entries`, `dtcs:map_from_entries`,
`dtcs:map_concat`, `dtcs:object_names`, and `dtcs:object_values`; and
higher-order Functions `dtcs:transform`, `dtcs:filter_values`, `dtcs:exists`,
`dtcs:forall`, `dtcs:reduce`, `dtcs:zip_with`, `dtcs:map_filter`,
`dtcs:transform_keys`, and `dtcs:transform_values`.

The profile extends the compatible subset of
`dtcs:profile/portable-complex-types/1`. It SHALL define collection order,
zero-based indexing, negative-index behavior, out-of-bounds access, null
elements, duplicate map keys, object field order, type unification, lambda
evaluation, and resource budgets. A plan SHALL state its duplicate-key and
collision policies. An implementation SHALL NOT substitute host-language map
or object behavior when it differs from those policies.

### 7.1 Concrete collection decisions

Lists use zero-based indexes. Negative indexes count from the end; an index
outside the resulting range returns null for `element_at` and invalid for
`index`. Maps and objects preserve insertion order. Duplicate keys are invalid
for constructors and `map_from_entries`; `map_concat` requires an explicit
`collisionPolicy` of `error`, `left`, or `right`. Object field order follows
constructor order. A collection may contain at most 1,000,000 elements unless
a lower plan limit is declared.

## 8. Generator and Reshape Profile

Profile `dtcs:profile/portable-reshape/1` contains `dtcs:explode`,
`dtcs:pivot`, and `dtcs:unpivot`.

`dtcs:explode` SHALL declare its input Expression, output fields, whether it is
outer, and whether it emits a position. It SHALL define behavior for lists,
maps, null, missing, invalid, and empty values; output ordering; positional
origin; field collisions; and maximum row expansion.

Pivot categories SHALL be declared in the plan or bounded by a planning
artifact. Ordinary validation and planning SHALL NOT read source rows to
discover an unbounded output schema. Pivot and unpivot SHALL define category
ordering, output naming, duplicate-cell aggregation, null categories, and
collision behavior.

### 8.1 Pivot and reshape decisions

`pivot.categories` is required and SHALL be a non-empty ordered array of
literal scalar values. Runtime category discovery is not permitted. A pivot
column name is the canonical JSON scalar representation of its category unless
`categoryNames` supplies an equal-length array of unique names. Null categories
are rejected. The default duplicate-cell policy is `error`; a plan may select a
declared aggregate Function instead. A pivot SHALL declare `maxColumns`, which
defaults to 10,000. `unpivot` emits rows in declared `fields` order.

`explode` emits list elements in list order and map entries in insertion order.
For maps, the element is a two-item `[key, value]` list. Position begins at
zero. Null, missing, invalid, and empty inputs emit zero rows unless
`outer: true`, in which case they emit one row with null element and position
zero. Field collisions are errors.

## 9. Extended Relational Profile

Profile `dtcs:profile/portable-relational-extended/1` contains
`dtcs:intersect`, `dtcs:except`, `dtcs:sample`, `dtcs:random_split`,
`dtcs:repartition`, and `dtcs:coalesce_partitions`.

Intersect and except SHALL declare distinct or all mode and positional or
by-name alignment. Sampling SHALL declare fraction or count mode, replacement
policy, and seed policy. Random split SHALL declare weights, seed policy, and
assignment algorithm. Seeded operations SHALL identify a portable
pseudorandom algorithm, seed width, and stable row-identity requirement, and
SHALL be independent of physical partitioning. Without stable row identity,
the plan SHALL classify the result as nondeterministic or fail a determinism
requirement.

Repartition expresses a logical distribution requirement only.
`dtcs:coalesce_partitions` is an optional physical hint and SHALL NOT alter row
semantics.

## 10. Temporal Profile

Profile `dtcs:profile/portable-temporal-iana/1` defines IANA timezone
identifiers, timezone-data version requirements, ambiguous and nonexistent
local-time policies, calendar-aware arithmetic, ISO-week extraction,
explicit week-start modes, declared sub-minute precision, and interval
construction and comparison. It contains `dtcs:to_timezone`,
`dtcs:from_utc`, and `dtcs:to_utc`.

A plan SHALL record the required timezone-data version or compatible range.
An implementation unable to provide compatible data SHALL reject the
requirement rather than silently use host timezone data. Month-end, leap-day,
ambiguous-time, and nonexistent-time behavior SHALL be explicit.

### 10.1 IANA and calendar decisions

The required timezone-data release is IANA `2025b`. `to_utc` requires
`ambiguousTimePolicy` (`earlier`, `later`, or `error`) and
`nonexistentTimePolicy` (`shiftForward` or `error`); both default to `error`.
Calendar month and year addition clamps the day to the final day of the target
month. Leap-day addition therefore yields February 28 in a non-leap target
year. ISO weeks begin Monday. Temporal precision is nanoseconds.

## 11. Controlled Nondeterminism Profile

Profile `dtcs:profile/portable-nondeterministic/1` contains `dtcs:random`,
`dtcs:random_normal`, `dtcs:uuid`, `dtcs:run_id`, and
`dtcs:run_timestamp`.

Every call SHALL be classified as `deterministic`, `run-stable`,
`seeded-stable`, or `nondeterministic`. Random Functions SHALL identify their
algorithm, seed policy, and numeric domain. UUID generation SHALL identify its
UUID version and representation. Run identifiers and timestamps SHALL remain
stable within one logical run.

Determinism classification SHALL affect fingerprints, caching, retries,
incremental execution, and idempotency analysis. Optimizers SHALL NOT
duplicate, reorder, or eliminate a call when doing so changes observable
results.

### 11.1 Random, UUID, and run-context decisions

Seeded `random`, `random_normal`, `sample`, and `random_split` use
`xorshift64star/1` with an unsigned 64-bit seed. A seeded call is
`seeded-stable`; an unseeded random call is `nondeterministic`. `uuid` produces
canonical lower-case RFC 4122 UUID version 4 strings. `run_id` and
`run_timestamp` are `run-stable` for one Execution Plan invocation; the latter
is UTC RFC 3339 with nanosecond precision.

## 12. Window Profile Version 2

Profile `dtcs:profile/portable-window/2` extends the compatible subset of
`dtcs:profile/portable-window/1` with `dtcs:ntile`, `dtcs:percent_rank`,
`dtcs:cume_dist`, `dtcs:nth_value`, explicit `ignoreNulls`, named windows,
safe inheritance, and registered exclusion modes. It SHALL define peer, tie,
NaN, and range-frame semantics precisely. Unsupported exclusion or
backend-specific frame modes SHALL remain separate capability requirements.

## 13. Nested Schema Evolution

Nested field paths SHALL be represented as structural path segments rather
than ambiguous dotted strings. Actions `dtcs:with_nested_fields`,
`dtcs:rename_nested_fields`, and `dtcs:drop_nested_fields` SHALL define
missing-parent behavior, automatic parent creation, list traversal, field
order, collisions, nullability, and compatibility impact. Logical schema
changes SHALL remain distinct from physical storage-schema mutation.

### 13.1 Structural path decisions

A nested path is an ordered array of segments. A field segment is
`{"kind":"field","name":"name"}`; a list segment is
`{"kind":"index","index":0}`. Dotted strings are invalid. Negative list
indexes are invalid in schema actions. `with_nested_fields` defaults to
`createParents: false`; with it enabled only missing object parents are
created. Traversing a null, missing, or non-object parent is invalid. Rename
and drop preserve object field order; collisions are errors unless an action
declares `collisionPolicy: "replace"`.

## 14. Error and Value-State Semantics

Every registry entry introduced by this chapter SHALL define behavior for
present, null, missing, invalid, NaN, positive and negative infinity, type
mismatch, overflow, underflow, and resource-budget exhaustion where
applicable.

Expression error mode SHALL be one of `fail`, `invalid`, `null`, or `route`.
The `route` mode is valid only when the containing action declares an
invalid-output path. A plan SHALL select the mode explicitly. Implementations
SHALL NOT infer it from backend defaults.

## 15. Security and Resource Requirements

DTCS 3.0 retains the prohibition on executable objects and raw SQL. Plans
SHALL declare or inherit enforceable limits for regex complexity, lambda
evaluation, collection construction, generator expansion, pivot columns,
aggregate state, and collected values. Implementations SHALL validate random
seeds and the provenance of timezone, Unicode, locale, regex, and format data.

Plans, Diagnostics, conformance artifacts, and fingerprints SHALL NOT contain
resolved secrets or source rows. Diagnostics SHALL be bounded and SHALL NOT
echo sensitive input values.

## 16. Conformance

Each profile in this chapter SHALL publish machine-readable positive,
negative, boundary, and hostile-input fixtures covering canonicalization,
typing, Diagnostics, value states, numeric boundaries, Unicode, temporal
boundaries, determinism, and resource limits as applicable. Cross-Engine
differential results SHALL be included before graduation.

A new semantic family SHALL remain Experimental until two independent
Compilers pass all mandatory fixtures. Reference implementation self-tests do
not constitute two independent Compilers. Unsupported backend behavior SHALL
fail capability analysis without approximation.

## 17. Summary

Rich Portable Analytics expands DTCS while preserving a small portable
kernel. Independently claimable profiles allow Engines to advertise precise
support and fail closed when semantics, environmental data, or budgets cannot
be honored.

------------------------------------------------------------------------

# Appendix A --- Standard Library Catalog (Normative)

## A.1 Purpose

This appendix enumerates concrete `dtcs:` identifiers for Semantic
Actions, Functions, and Rules that constitute the DTCS standard
libraries.

Categories listed in Chapter 17 §5 map to the Semantic Action
identifiers defined in §A.3. Function and Rule identifiers in §A.4 and
§A.5 complete the standard libraries referenced by Chapters 18 and 19.

Implementations that claim support for the DTCS standard libraries
SHALL recognize these identifiers with the semantics described here and
in the corresponding chapters. Additional vendor identifiers MAY be
provided through registries as defined in Chapter 22.

------------------------------------------------------------------------

## A.2 Semantic Action parameters

SemanticAction instances MAY include a `parameters` map.

Parameter keys and value shapes are defined per action identifier.
Dataset-scoped actions use interface identifiers as targets.
Field-scoped actions use `interface.field` targets.

Implementations SHALL reject unknown required parameters and SHALL
preserve declared parameters through planning, compilation, and
execution without semantic loss.

------------------------------------------------------------------------

## A.3 Semantic Actions

Unless otherwise noted, every standard Semantic Action listed below is
deterministic.

Field transforms (category: transformation) operate on string-typed
field targets unless a registry definition states otherwise:

-   `dtcs:lowercase` --- no parameters; lineageBehavior `preserve`
-   `dtcs:uppercase` --- no parameters; lineageBehavior `preserve`
-   `dtcs:capitalize` --- no parameters; lineageBehavior `preserve`
-   `dtcs:trim` --- no parameters; lineageBehavior `preserve`
-   `dtcs:normalize_whitespace` --- no parameters; lineageBehavior
    `preserve`
-   `dtcs:hash_sha256` --- no parameters; lineageBehavior `derive`

Dataset operators:

-   `dtcs:project` (category: projection; entry version `2.0.0`) ---
    parameter `fields`: legacy form is an array of field names; rich
    form is an ordered array of names and/or `{expr, as}` objects. No
    unselected field is implicitly retained. lineageBehavior `preserve`
-   `dtcs:select` (category: selection; entry version `2.0.0`) ---
    legacy parameters `field`, optional `equals`; rich parameter
    `predicate` (Expression). lineageBehavior `filter`
-   `dtcs:filter` (category: filtering; entry version `2.0.0`) ---
    same parameters as `dtcs:select`. Predicate semantics: retain
    `true`; discard `false`, `null`, and `missing`; fail (or
    explicitly route) on `invalid`; never coerce `invalid` to
    `false`. lineageBehavior `filter`
-   `dtcs:with_fields` (category: projection) --- parameter
    `assignments` (ordered `{as, expr}`); add or replace fields.
    lineageBehavior `derive`
-   `dtcs:rename_fields` (category: projection) --- parameter
    `mapping` (`old` → `new`); rename without changing value or type.
    lineageBehavior `preserve`
-   `dtcs:drop_fields` (category: projection) --- parameters
    `fields`, optional `missingPolicy` (`error` default, or
    `ignore`). lineageBehavior `discard`
-   `dtcs:aggregate` (category: aggregation; entry version `2.0.0`) ---
    legacy parameters `groupBy`, `valueField`, `op`; rich parameters
    `groupBy` / `aggregates` expression lists, optional `filter`.
    Missing grouping keys are distinct from null. lineageBehavior
    `aggregate`
-   `dtcs:group` (category: grouping; entry version `2.0.0`) --- same
    parameters as `dtcs:aggregate`. lineageBehavior `aggregate`
-   `dtcs:join` (category: joining; entry version `2.0.0`) --- legacy
    parameters `right`, `leftKey`, optional `rightKey` (inner
    equi-join). Rich parameters: `type` (`inner`, `left`, `right`,
    `full`, `semi`, `anti`, `cross`), `on` / `predicate`, `nullSafe`,
    `collisionPolicy`. Under entry version `2.0.0`, ordinary equality
    SHALL NOT match null keys. lineageBehavior `derive`
-   `dtcs:sort` (category: sorting; entry version `2.0.0`) --- legacy
    parameters `field`, optional `descending`; rich parameter `keys`
    (`expr`, `descending`, `nulls`). Outputs remain unordered without
    sort. lineageBehavior `preserve`
-   `dtcs:union` (category: union; entry version `2.0.0`) --- legacy
    parameter `other` (positional append); rich parameters `mode`
    (`positional` \| `byName`), `missingColumnPolicy`,
    `duplicatePolicy`. lineageBehavior `derive`
-   `dtcs:distinct` (category: selection) --- optional `fields`;
    full-row distinct when omitted. lineageBehavior `filter`
-   `dtcs:deduplicate` (category: selection) --- parameters `keys`,
    `retain`; retain policy required; may be nondeterministic without
    ordering. lineageBehavior `filter`
-   `dtcs:limit` (category: selection) --- parameters `count`,
    optional `offset`; nondeterministic without prior sort.
    lineageBehavior `filter`
-   `dtcs:partition` (category: partitioning) --- parameters: `field`;
    lineageBehavior `partition`
-   `dtcs:window` (category: window) --- parameters `partitionBy`,
    `orderBy`, `frame`, `functions`; profile
    `dtcs:profile/portable-window/1`. Frame `type` SHALL be `rows` or
    `range`; bounds are `unboundedPreceding`, `nPreceding`,
    `currentRow`, `nFollowing`, `unboundedFollowing`. Default frame is
    rows from unbounded preceding through current row. Window functions
    include `row_number`, `rank`, `dense_rank`, `lag`, `lead`,
    `first_value`, `last_value`, and framed aggregates
    (`sum`/`count`/`average`/`min`/`max`).

Lineage default operation:

-   `dtcs:derive` (category: transformation) --- no parameters;
    lineageBehavior `derive`

`dtcs:derive` is the default lineage `operation` when a mapping omits an
explicit operation identifier (see §A.6 and Chapter 10 §6).

------------------------------------------------------------------------

## A.4 Functions

Standard Functions MAY be invoked from expressions using direct `dtcs:`
call syntax as defined in Chapter 8.

Each Function SHALL declare a null-behavior token. The tokens used by
the standard library are:

-   `propagate` --- a null or missing argument yields a null result
-   `defined` --- null and missing arguments are interpreted according
    to the Function's documented semantics (the result is not forced to
    null solely by argument absence)

The standard Function catalog is:

-   `dtcs:lower` --- nullBehavior `propagate`
-   `dtcs:upper` --- nullBehavior `propagate`
-   `dtcs:concat` --- nullBehavior `propagate`; minimum two arguments
-   `dtcs:concat_ws` --- nullBehavior `defined`; separator plus values
-   `dtcs:substr` --- nullBehavior `propagate`
-   `dtcs:replace` --- nullBehavior `propagate`
-   `dtcs:coalesce` (entry version `2.0.0`) --- nullBehavior `defined`;
    returns the first present non-null, non-missing argument (skips
    invalid); formerly incorrectly tagged `propagate`
-   `dtcs:if_null` --- nullBehavior `defined`; replace null/missing
-   `dtcs:null_if` --- nullBehavior `defined`; null when equal
-   `dtcs:case_when` --- nullBehavior `defined`; conditional
-   `dtcs:try_cast` --- nullBehavior `defined`; tolerant cast
-   `dtcs:is_invalid` --- nullBehavior `defined`; true when invalid
-   `dtcs:length` --- nullBehavior `propagate`
-   `dtcs:to_string` --- nullBehavior `propagate`
-   `dtcs:to_integer` --- nullBehavior `propagate`
-   `dtcs:to_decimal` --- nullBehavior `propagate`
-   `dtcs:abs` --- nullBehavior `propagate`; numeric absolute value
-   `dtcs:round`, `dtcs:floor`, `dtcs:ceil`, `dtcs:power`, `dtcs:sqrt`
    --- nullBehavior `propagate`
-   `dtcs:least`, `dtcs:greatest` --- nullBehavior `propagate`
-   `dtcs:min` --- nullBehavior `propagate`; variadic minimum
-   `dtcs:max` --- nullBehavior `propagate`; variadic maximum
-   `dtcs:contains` --- nullBehavior `propagate`; substring containment
-   `dtcs:starts_with`, `dtcs:ends_with` --- nullBehavior `propagate`
-   `dtcs:is_null` --- nullBehavior `defined`; true when the argument
    is null
-   `dtcs:is_missing` --- nullBehavior `defined`; true when the
    argument is missing
-   Aggregate family: `dtcs:count_all`, `dtcs:count`,
    `dtcs:count_distinct`, `dtcs:sum`, `dtcs:average`
-   Date/time family (fixed-offset timezone profile; reference clock
    MAY be fixed for determinism): `dtcs:current_date`,
    `dtcs:current_timestamp`, `dtcs:date_add`, `dtcs:date_diff`,
    `dtcs:date_trunc`, `dtcs:extract`, `dtcs:at_timezone`. Units for
    add/diff include day, month, year, hour, and minute. IANA/DST
    calendars are non-goals for the portable fixed-offset profile.
-   Window family (profile `dtcs:profile/portable-window/1`):
    `dtcs:row_number`, `dtcs:rank`, `dtcs:dense_rank`, `dtcs:lag`,
    `dtcs:lead`, `dtcs:first_value`, `dtcs:last_value`

### A.4.1 Operators

Operators are registered under `dtcs:stdlib.operators` and SHALL NOT
share ambiguous identities with Functions.

Comparison: `dtcs:eq`, `dtcs:not_eq`, `dtcs:lt`, `dtcs:lte`,
`dtcs:gt`, `dtcs:gte`, `dtcs:null_safe_eq`

Boolean: `dtcs:and`, `dtcs:or`, `dtcs:not`

Arithmetic: `dtcs:add`, `dtcs:subtract`, `dtcs:multiply`,
`dtcs:divide`, `dtcs:modulo`, `dtcs:negate`

Membership: `dtcs:in`, `dtcs:between` (arity 3; inclusive
`value between lower and upper`; null/missing operands yield null)

Access: `dtcs:field`, `dtcs:index`, `dtcs:element_at` (list/map/object
access; `element_at` returns null on out-of-bounds)

Each operator declares arity, accepted types, return type, promotion,
null/missing/invalid and NaN behavior, errors, determinism, and
optimizer-safe properties. Capability declarations MAY list short names
(`eq`) and/or registry identifiers (`dtcs:eq`).

All standard Functions listed above are deterministic unless marked
otherwise (date/time and window families may be nondeterministic or
non-run-stable).

------------------------------------------------------------------------

## A.5 Rules

Standard Rules MAY appear as preconditions, execution rules, or
postconditions subject to registry phase metadata (Chapter 19 §5).

The standard Rule catalog is:

-   `dtcs:not_null` --- no parameters; the target SHALL be present and
    non-null
-   `dtcs:min_length` --- parameter: `min`; string minimum length
-   `dtcs:max_length` --- parameter: `max`; string maximum length
-   `dtcs:regex_match` --- parameter: `pattern`; regular-expression
    match
-   `dtcs:range` --- optional parameters: `min`, `max`; inclusive
    numeric bounds
-   `dtcs:one_of` --- parameter: `values` (array); value membership
-   `dtcs:equals` --- parameter: `value`; equality comparison with
    `allowIndeterminate` true

Rule evaluation outcomes and indeterminate handling SHALL follow Chapter
19. Implementations SHALL NOT invent alternate identifiers for the same
standard semantics.

------------------------------------------------------------------------

## A.6 Lineage mapping fields

A lineage mapping object SHALL identify provenance from inputs to an
output as defined in Chapter 10.

Lineage mapping fields:

-   `id` (optional) --- stable mapping identity
-   `output` --- destination interface or field reference
-   `inputs` --- array of contributing source references
-   `operation` --- semantic operation identifier; defaults to
    `dtcs:derive` when omitted
-   `flow` --- information-flow kind; one of:

    -   `preserved`
    -   `derived`
    -   `aggregated`
    -   `filtered`
    -   `partitioned`
    -   `discarded`

Information loss SHALL be explicit where it affects contractual
guarantees (Chapter 10 §7).

------------------------------------------------------------------------

## A.7 Null semantics tokens

Runtime evaluation SHALL distinguish the following value kinds
(Chapter 8 §9):

-   null --- a present key with a null payload
-   missing --- an explicit missing token serialized as
    `{"$dtcs":"missing"}`
-   invalid --- an explicit invalid token serialized as
    `{"$dtcs":"invalid"}`, optionally with a `reason` field

Null, missing, and invalid are not interchangeable. Functions and
Semantic Actions SHALL apply their declared null-behavior with respect
to these distinctions. Implementations SHALL NOT coerce missing or
invalid values to null except where a standard library entry explicitly
defines that behavior.

------------------------------------------------------------------------

## A.8 Portable Relational Profiles

DTCS publishes semantic-family profiles orthogonal to implementation-class
profiles (Chapter 23). Registry document `dtcs:stdlib.profiles` lists:

-   `dtcs:profile/portable-relational-kernel/1` --- project, filter,
    field shaping (`with_fields`, `rename_fields`, `drop_fields`),
    scalar operators/functions, portable plan serialization
-   `dtcs:profile/portable-relational/1` --- extends kernel with joins,
    unions, grouping, aggregation, ordering, distinct, deduplicate,
    limit, and aggregate functions
-   `dtcs:profile/portable-window/1` --- windows and analytics
    (experimental until two independent compilers pass)
-   `dtcs:profile/portable-complex-types/1` --- arrays/lists, maps,
    structs/objects (experimental); type aliases `array`→`list` and
    `struct`→`object` are additive

DTCS 2.0 plan serialization identity: `dtcs.transform-plan/1`. DTCS 3.0 plan
serialization identity: `dtcs.transform-plan/2` (Chapter 13 §12.1). DTCS 3.0
implementations SHALL read version 1 plans and migrate them without semantic
change.

A semantic family advances from Experimental to Standard only after at
least two independent compilers pass that family's conformance fixtures.
False capability claims invalidate conformance.

SQL-lowering compilers that claim a portable profile SHALL use bound
parameters and safe identifiers; they SHALL NOT embed raw SQL or
executable objects in portable plans.

------------------------------------------------------------------------

## A.9 DTCS 3.0 Rich Portable Analytics Profiles

The DTCS 3.0 profile catalog is:

-   `dtcs:profile/portable-relational-kernel/2` --- Candidate until two-compiler
    graduation criteria are met
-   `dtcs:profile/portable-relational/2` --- Candidate until two-compiler
    graduation criteria are met
-   `dtcs:profile/portable-string-advanced/1` --- Experimental
-   `dtcs:profile/portable-conversion/1` --- Experimental
-   `dtcs:profile/portable-statistics/1` --- Experimental
-   `dtcs:profile/portable-complex-values/1` --- Experimental
-   `dtcs:profile/portable-reshape/1` --- Experimental
-   `dtcs:profile/portable-relational-extended/1` --- Experimental
-   `dtcs:profile/portable-temporal-iana/1` --- Experimental
-   `dtcs:profile/portable-nondeterministic/1` --- Experimental
-   `dtcs:profile/portable-window/2` --- Candidate pending conformance by two
    independent Compilers

Profile composition SHALL be explicit. Claiming an advanced profile SHALL NOT
implicitly claim an unrelated family. The normative identifiers belonging to
each profile are listed in Chapter 27. Every registry entry SHALL additionally
publish its purpose, parameters, type rules, value-state and error behavior,
lineage effect, determinism classification, compatibility rules, applicable
resource limits, and semantic entry version.

`dtcs:profile/portable-complex-types/1` maps to the compatible subset of
`dtcs:profile/portable-complex-values/1`. Constructors and lambda Functions
are additional capabilities. `dtcs:profile/portable-window/1` maps to the
compatible subset of `dtcs:profile/portable-window/2`.

Field Semantic Actions retain their published identities. Where a Function
shares a lexical identifier with a field Semantic Action, registry category
and structured plan position SHALL disambiguate the entry. Implementations
SHALL NOT infer the registry category from backend overload resolution.
