# Glossary

| Term | Meaning |
|------|---------|
| **COM** | Canonical Object Model — in-memory contract after parse |
| **Contract** | Transformation Contract document (YAML/JSON) |
| **`dtcsVersion`** | Spec version this document targets. Prefer `"2.0.0"`; `"1.0.0"` / `"1.0.0-draft"` still accepted |
| **Diagnostic** | Structured finding with severity, code, and stage |
| **Engine capability profile** | Declared engine features used by `match` |
| **Execution plan** | Compiled step list for a runtime backend |
| **Flow** | Lineage information-flow kind (`preserved`, `derived`, `filtered`, …) |
| **Function** | Named COM function, often backed by a `dtcs:` registry entry |
| **Lineage mapping** | Declares which inputs contribute to an output |
| **Missing** | Distinct from null; JSON `{"$dtcs":"missing"}` |
| **Invalid** | Distinct from null; JSON `{"$dtcs":"invalid"}` |
| **Operation** | Lineage mapping operation id (default `dtcs:derive`) |
| **Plan** | Transformation plan IR after lowering |
| **Reference runtime** | In-memory executor in this repo (not a warehouse) |
| **Registry** | Catalog of standard/vendor identifiers |
| **Rule** | Constraint evaluated at a phase (pre/execution/post) |
| **Semantic action** | Declared transform (`dtcs:lowercase`, `dtcs:project`, …) |
| **Portable Relational** | Spec 2.0 profile family for joins, aggs, windows, datetime, and complex access |
| **Portable plan** | Serializable transform-plan IR (`dtcs.transform-plan/1`) for engine interchange |
| **SPEC** | Normative DTCS document (`SPEC.md`, currently draft 2.0.0) |

See also [versioning.md](versioning.md) and SPEC Chapter 1 §9 / Chapter 2.
