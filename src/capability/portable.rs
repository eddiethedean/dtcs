//! Portable engine capability manifests (proposal §13 / Ch 14).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::registry::default_registry;

use super::builtin::{
    reference_profile, REFERENCE_ENGINE_ID, REFERENCE_LANGUAGE_FEATURES, REFERENCE_OPERATORS,
    REFERENCE_OPTIMIZATION, REFERENCE_RUNTIME_FEATURES,
};
use super::model::EngineCapabilityDeclaration;

/// Per-entry capability detail for a registry identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryCapability {
    /// Whether the entry is supported.
    pub supported: bool,
    /// Entry version this declaration covers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Semantic modes (for example filter invalid handling).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_modes: Vec<String>,
    /// Known limits.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub limits: IndexMap<String, String>,
    /// Notes / unsupported optional features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// Certification tier: `certified`, `experimental`, or `unsupported`.
    #[serde(default = "default_tier", skip_serializing_if = "String::is_empty")]
    pub tier: String,
}

fn default_tier() -> String {
    "certified".into()
}

/// Machine-readable portable capability manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableCapabilityManifest {
    /// Portable profile this declaration targets.
    pub profile: String,
    /// Implementation class (Compiler, Runtime, …).
    pub implementation_class: String,
    /// Engine identifier.
    pub engine: String,
    /// Engine version.
    pub engine_version: String,
    /// Supported plan serialization profiles.
    pub supported_plan_profiles: Vec<String>,
    /// Actions keyed by registry id.
    pub actions: IndexMap<String, EntryCapability>,
    /// Operators keyed by registry id or short name.
    pub operators: IndexMap<String, EntryCapability>,
    /// Functions keyed by registry id.
    pub functions: IndexMap<String, EntryCapability>,
    /// Types keyed by logical type name.
    pub types: IndexMap<String, EntryCapability>,
    /// Semantic modes at engine level.
    pub semantic_modes: IndexMap<String, String>,
    /// Engine limits.
    pub limits: IndexMap<String, String>,
    /// Legacy flat capability declaration for compatibility.
    pub legacy: EngineCapabilityDeclaration,
}

/// Features the reference runtime does **not** certify (false-claim gate).
pub const REFERENCE_UNSUPPORTED_CLAIMS: &[&str] = &["unnest", "dstCalendar"];

/// Features available experimentally on the DTCS 3.0 reference surface.
pub const REFERENCE_EXPERIMENTAL_FEATURES: &[&str] = &[
    "ianaTimezone",
    "explode",
    "map_entries",
    "pivot",
    "statistics",
    "windowV2",
    "seededNondeterminism",
];

/// Features the reference runtime certifies for portable profiles.
pub const REFERENCE_CERTIFIED_FEATURES: &[&str] = &[
    "windowFramesRows",
    "windowFramesRange",
    "firstValue",
    "lastValue",
    "fixedOffsetTimezone",
    "dateTruncExtract",
    "complexAccessOps",
    "betweenTernary",
    "joinCollisionPolicy",
    "joinPredicate",
    "sortExpr",
    "unionDuplicatePolicy",
    "groupByExpr",
];

/// Build the reference portable capability manifest for a profile.
#[must_use]
pub fn reference_portable_manifest(profile: &str) -> PortableCapabilityManifest {
    let legacy = reference_profile();
    let registry = default_registry();
    let mut actions = IndexMap::new();
    let mut functions = IndexMap::new();
    let mut operators = IndexMap::new();
    let mut types = IndexMap::new();

    for entry in registry.entries.values() {
        match entry.category {
            crate::model::RegistryCategory::SemanticAction => {
                let (supported, tier, notes) = action_support(&entry.id);
                actions.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported,
                        version: Some(entry.version.clone()),
                        semantic_modes: Vec::new(),
                        limits: IndexMap::new(),
                        notes,
                        tier,
                    },
                );
            }
            crate::model::RegistryCategory::Function => {
                let experimental = matches!(
                    entry.status,
                    crate::model::RegistryEntryStatus::Experimental
                        | crate::model::RegistryEntryStatus::Candidate
                );
                let (supported, tier, notes) = function_support(&entry.id, experimental);
                functions.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported,
                        version: Some(entry.version.clone()),
                        semantic_modes: if experimental {
                            vec!["experimental".into()]
                        } else {
                            Vec::new()
                        },
                        limits: function_limits(&entry.id),
                        notes,
                        tier,
                    },
                );
            }
            crate::model::RegistryCategory::Operator => {
                operators.insert(
                    entry.id.clone(),
                    EntryCapability {
                        supported: true,
                        version: Some(entry.version.clone()),
                        semantic_modes: Vec::new(),
                        limits: IndexMap::new(),
                        notes: Vec::new(),
                        tier: "certified".into(),
                    },
                );
            }
            _ => {}
        }
    }
    for op in REFERENCE_OPERATORS {
        operators
            .entry((*op).to_string())
            .or_insert(EntryCapability {
                supported: true,
                version: Some("1.0.0".into()),
                semantic_modes: Vec::new(),
                limits: IndexMap::new(),
                notes: Vec::new(),
                tier: "certified".into(),
            });
    }
    for ty in &legacy.categories.logical_types {
        types.insert(
            ty.clone(),
            EntryCapability {
                supported: true,
                version: Some("1.0.0".into()),
                semantic_modes: Vec::new(),
                limits: IndexMap::new(),
                notes: Vec::new(),
                tier: "certified".into(),
            },
        );
    }

    let mut semantic_modes = IndexMap::new();
    semantic_modes.insert("filterInvalid".into(), "fail".into());
    semantic_modes.insert("joinNullKeys".into(), "neverMatch".into());
    semantic_modes.insert("missingGroupingKeys".into(), "distinctFromNull".into());
    semantic_modes.insert("timezone".into(), "fixedOffset".into());
    semantic_modes.insert("windowFrames".into(), "rowsAndRange".into());
    for feature in REFERENCE_LANGUAGE_FEATURES {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }
    for feature in REFERENCE_OPTIMIZATION {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }
    for feature in REFERENCE_RUNTIME_FEATURES {
        semantic_modes.insert((*feature).to_string(), "supported".into());
    }
    for feature in REFERENCE_CERTIFIED_FEATURES {
        semantic_modes.insert((*feature).to_string(), "certified".into());
    }
    for feature in REFERENCE_EXPERIMENTAL_FEATURES {
        semantic_modes.insert((*feature).to_string(), "experimental".into());
    }
    for feature in REFERENCE_UNSUPPORTED_CLAIMS {
        semantic_modes.insert((*feature).to_string(), "unsupported".into());
    }

    let mut limits = IndexMap::new();
    limits.insert(
        "maxPortablePlanBytes".into(),
        crate::plan::MAX_PORTABLE_PLAN_BYTES.to_string(),
    );
    limits.insert(
        "maxPortablePlanDepth".into(),
        crate::plan::MAX_PORTABLE_PLAN_DEPTH.to_string(),
    );
    limits.insert(
        "maxPortablePlanNodes".into(),
        crate::plan::MAX_PORTABLE_PLAN_NODES.to_string(),
    );

    PortableCapabilityManifest {
        profile: profile.to_string(),
        implementation_class: "Compiler".into(),
        engine: REFERENCE_ENGINE_ID.into(),
        engine_version: env!("CARGO_PKG_VERSION").into(),
        supported_plan_profiles: vec![crate::plan::TRANSFORM_PLAN_IDENTITY.into()],
        actions,
        operators,
        functions,
        types,
        semantic_modes,
        limits,
        legacy,
    }
}

/// Validate that a manifest does not falsely claim unsupported features.
pub fn validate_capability_accuracy(
    manifest: &PortableCapabilityManifest,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for feature in REFERENCE_UNSUPPORTED_CLAIMS {
        if let Some(mode) = manifest.semantic_modes.get(*feature) {
            if mode == "supported" || mode == "certified" {
                errors.push(format!(
                    "false claim: '{feature}' marked '{mode}' but reference runtime does not support it"
                ));
            }
        }
    }
    for feature in REFERENCE_EXPERIMENTAL_FEATURES {
        if let Some(mode) = manifest.semantic_modes.get(*feature) {
            if mode == "certified" {
                errors.push(format!(
                    "false claim: '{feature}' marked certified but reference surface is experimental only"
                ));
            }
        }
    }
    for (id, entry) in &manifest.functions {
        if entry.supported && entry.tier == "certified" && matches!(id.as_str(), "dtcs:unnest") {
            errors.push(format!("false claim: certified support for '{id}'"));
        }
        if !entry.supported && entry.tier == "certified" {
            errors.push(format!(
                "inconsistent claim: '{id}' unsupported but tier=certified"
            ));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn action_support(id: &str) -> (bool, String, Vec<String>) {
    match id {
        "dtcs:window" => (
            true,
            "certified".into(),
            vec![
                "supports rows and range frames".into(),
                "first_value/last_value and framed aggregates".into(),
                "window/2 ntile/percent_rank/cume_dist/nth_value are candidate".into(),
            ],
        ),
        "dtcs:repartition" | "dtcs:coalesce_partitions" => (
            true,
            "experimental".into(),
            vec!["logical layout hint only; no row mutation in reference runtime".into()],
        ),
        "dtcs:pivot"
        | "dtcs:explode"
        | "dtcs:unpivot"
        | "dtcs:intersect"
        | "dtcs:except"
        | "dtcs:sample"
        | "dtcs:random_split"
        | "dtcs:with_nested_fields"
        | "dtcs:rename_nested_fields"
        | "dtcs:drop_nested_fields" => (
            true,
            "experimental".into(),
            vec!["DTCS 3.0 reference surface; dual-compiler certification pending".into()],
        ),
        _ => (true, "certified".into(), Vec::new()),
    }
}

fn function_support(id: &str, experimental: bool) -> (bool, String, Vec<String>) {
    let mut notes = function_notes(id, experimental);
    // Aggregate and window functions are not scalar-callable.
    if matches!(
        id,
        "dtcs:count_all"
            | "dtcs:count"
            | "dtcs:count_distinct"
            | "dtcs:sum"
            | "dtcs:average"
            | "dtcs:row_number"
            | "dtcs:rank"
            | "dtcs:dense_rank"
            | "dtcs:lag"
            | "dtcs:lead"
            | "dtcs:first_value"
            | "dtcs:last_value"
            | "dtcs:ntile"
            | "dtcs:percent_rank"
            | "dtcs:cume_dist"
            | "dtcs:nth_value"
            | "dtcs:variance"
            | "dtcs:stddev"
            | "dtcs:median"
            | "dtcs:collect_list"
            | "dtcs:collect_set"
    ) {
        notes.push("not callable as a scalar expression; use aggregate/window actions".into());
        let tier = if experimental
            || matches!(
                id,
                "dtcs:ntile"
                    | "dtcs:percent_rank"
                    | "dtcs:cume_dist"
                    | "dtcs:nth_value"
                    | "dtcs:variance"
                    | "dtcs:stddev"
                    | "dtcs:median"
                    | "dtcs:collect_list"
                    | "dtcs:collect_set"
            ) {
            "experimental".into()
        } else {
            "certified".into()
        };
        return (true, tier, notes);
    }
    let tier = if experimental {
        "experimental".into()
    } else {
        "certified".into()
    };
    (true, tier, notes)
}

fn function_limits(id: &str) -> IndexMap<String, String> {
    let mut limits = IndexMap::new();
    if matches!(
        id,
        "dtcs:current_date" | "dtcs:current_timestamp" | "dtcs:at_timezone"
    ) {
        limits.insert("timezone".into(), "fixedOffset".into());
        limits.insert("ianaTimezone".into(), "unsupported".into());
    }
    limits
}

fn function_notes(id: &str, experimental: bool) -> Vec<String> {
    let mut notes = Vec::new();
    if experimental {
        notes.push("experimental registry status".into());
    }
    if matches!(id, "dtcs:current_date" | "dtcs:current_timestamp") {
        notes.push("reference clock fixed at 2026-01-01 for determinism".into());
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_manifest_passes_accuracy_gate() {
        let manifest = reference_portable_manifest("dtcs:profile/portable-relational/1");
        validate_capability_accuracy(&manifest).expect("reference should be accurate");
        assert_eq!(
            manifest
                .semantic_modes
                .get("ianaTimezone")
                .map(String::as_str),
            Some("experimental")
        );
        assert_eq!(
            manifest
                .semantic_modes
                .get("windowFramesRows")
                .map(String::as_str),
            Some("certified")
        );
    }

    #[test]
    fn false_claim_is_rejected() {
        let mut manifest = reference_portable_manifest("dtcs:profile/portable-relational/1");
        manifest
            .semantic_modes
            .insert("ianaTimezone".into(), "certified".into());
        let err = validate_capability_accuracy(&manifest).expect_err("should fail");
        assert!(err.iter().any(|e| e.contains("ianaTimezone")));
    }
}
