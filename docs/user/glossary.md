# Glossary

| Term | Meaning |
|------|---------|
| **COM** | Canonical Object Model — in-memory contract after parse |
| **Contract** | Transformation Contract document (YAML/JSON) |
| **`dtcsVersion`** | Document version string; must be `"1.0.0"` today |
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
| **SPEC** | Normative DTCS document (`SPEC.md`, currently draft) |

See also SPEC Chapter 1 §9 and Chapter 2.
