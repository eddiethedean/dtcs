//! Embedded `dtcs:` registry catalog.

use indexmap::IndexMap;

use crate::diagnostics::codes;
use crate::model::{
    ExtensionCompatibility, RegistryCategory, RegistryDocument, RegistryEntry, RegistryEntryStatus,
    RegistryPublicationStatus,
};

/// Builds the embedded standard registry for this implementation.
#[must_use]
pub fn builtin_registry() -> RegistryDocument {
    let mut entries = IndexMap::new();

    insert_entry(
        &mut entries,
        entry(
            "dtcs:lowercase",
            "Lowercase",
            RegistryCategory::SemanticAction,
            "Lowercases a non-nullable string field",
        ),
    );
    insert_entry(
        &mut entries,
        entry(
            "dtcs:not_null",
            "Not Null",
            RegistryCategory::Rule,
            "Requires a non-nullable schema field",
        ),
    );
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

    RegistryDocument {
        id: "dtcs:builtin".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        governing_specification: crate::SPEC_VERSION.into(),
        publication_status: RegistryPublicationStatus::Standard,
        entries,
    }
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
