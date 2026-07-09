//! Embedded `dtcs:` registry catalog.

use indexmap::IndexMap;

use crate::diagnostics::codes;
use crate::model::{
    ExtensionCompatibility, RegistryCategory, RegistryDocument, RegistryEntry, RegistryEntryStatus,
    RegistryPublicationStatus,
};

use super::load;
use crate::parser::DocumentFormat;

/// Builds the embedded standard registry for this implementation.
#[must_use]
pub fn builtin_registry() -> RegistryDocument {
    let mut entries = IndexMap::new();

    insert_entry(
        &mut entries,
        entry(
            "dtcs",
            "DTCS Namespace",
            RegistryCategory::ExtensionNamespace,
            "Reserved namespace for standardized DTCS identifiers",
        ),
    );

    for code in codes::ALL_CODES {
        let name = code.strip_prefix("dtcs:").unwrap_or(code).replace('-', " ");
        insert_entry(
            &mut entries,
            entry(code, &title_case(&name), RegistryCategory::Diagnostic, code),
        );
    }

    let mut registry = RegistryDocument {
        id: "dtcs:builtin".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        governing_specification: crate::SPEC_VERSION.into(),
        publication_status: RegistryPublicationStatus::Standard,
        entries,
    };

    // Merge the embedded standard libraries (Phase 0.5).
    merge_builtin_doc(
        &mut registry,
        include_bytes!("builtin/semantic_actions.yaml"),
        "builtin/semantic_actions.yaml",
    );
    merge_builtin_doc(
        &mut registry,
        include_bytes!("builtin/functions.yaml"),
        "builtin/functions.yaml",
    );
    merge_builtin_doc(
        &mut registry,
        include_bytes!("builtin/rules.yaml"),
        "builtin/rules.yaml",
    );

    registry
}

fn entry(id: &str, name: &str, category: RegistryCategory, definition: &str) -> RegistryEntry {
    let compatibility = matches!(category, RegistryCategory::ExtensionNamespace)
        .then_some(ExtensionCompatibility::Optional);
    RegistryEntry {
        id: id.into(),
        name: name.into(),
        category,
        version: "1.0.0".into(),
        status: RegistryEntryStatus::Standard,
        compatibility,
        definition: Some(definition.into()),
        references: Vec::new(),
        supported: true,
    }
}

fn insert_entry(entries: &mut IndexMap<String, RegistryEntry>, entry: RegistryEntry) {
    entries.insert(entry.id.clone(), entry);
}

fn merge_builtin_doc(registry: &mut RegistryDocument, bytes: &[u8], name: &'static str) {
    let doc = load::load_bytes(bytes, DocumentFormat::Yaml).unwrap_or_else(|report| {
        let summary = report
            .diagnostics
            .iter()
            .map(|d| format!("{}: {}", d.id, d.message))
            .collect::<Vec<_>>()
            .join("; ");
        panic!("invalid embedded registry doc '{name}': {summary}");
    });
    registry.merge_trusted(&doc);
}

fn title_case(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
