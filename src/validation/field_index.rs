//! Scoped schema field index for reference resolution.

use std::collections::{HashMap, HashSet};

use crate::model::{Field, TransformationContract};

/// A resolved schema field location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldLocation {
    /// Declaring interface identifier.
    pub interface_id: String,
    /// Field name.
    pub field_name: String,
    /// Logical type expression.
    pub type_name: String,
    /// Whether the field is nullable.
    pub nullable: bool,
    /// `true` when declared on an input.
    pub is_input: bool,
}

/// Result of resolving a target reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetResolution<'a> {
    /// Resolved schema field.
    Field(&'a FieldLocation),
    /// Resolved interface identifier without a field target.
    Interface {
        /// Interface identifier.
        id: String,
        /// `true` when the interface is an input.
        is_input: bool,
    },
    /// Target matches multiple schema fields.
    Ambiguous(Vec<FieldLocation>),
    /// Target could not be resolved.
    NotFound,
}

/// A qualified field path collision between two declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedFieldCollision {
    /// Colliding qualified path (`interface.field`).
    pub qualified: String,
    /// First declaring interface.
    pub first_interface: String,
    /// Second declaring interface.
    pub second_interface: String,
}

/// Index of declared interfaces and schema fields.
#[derive(Debug, Default)]
pub struct FieldIndex {
    qualified: HashMap<String, FieldLocation>,
    by_name: HashMap<String, Vec<FieldLocation>>,
    input_ids: HashSet<String>,
    output_ids: HashSet<String>,
    qualified_collisions: Vec<QualifiedFieldCollision>,
}

impl FieldIndex {
    /// Build a field index from a transformation contract.
    #[must_use]
    pub fn from_contract(contract: &TransformationContract) -> Self {
        let mut index = Self::default();
        for input in &contract.inputs {
            index.input_ids.insert(input.id.clone());
            if let Some(schema) = &input.schema {
                index.insert_fields(&input.id, schema.fields.iter(), true);
            }
        }
        for output in &contract.outputs {
            index.output_ids.insert(output.id.clone());
            if let Some(schema) = &output.schema {
                index.insert_fields(&output.id, schema.fields.iter(), false);
            }
        }
        index
    }

    fn insert_fields<'a>(
        &mut self,
        interface_id: &str,
        fields: impl Iterator<Item = &'a Field>,
        is_input: bool,
    ) {
        for field in fields {
            let qualified = format!("{interface_id}.{}", field.name);
            if let Some(existing) = self.qualified.get(&qualified) {
                self.qualified_collisions.push(QualifiedFieldCollision {
                    qualified: qualified.clone(),
                    first_interface: existing.interface_id.clone(),
                    second_interface: interface_id.to_string(),
                });
                continue;
            }
            let location = FieldLocation {
                interface_id: interface_id.to_string(),
                field_name: field.name.clone(),
                type_name: field.type_name.clone(),
                nullable: field.nullable,
                is_input,
            };
            self.qualified.insert(qualified, location.clone());
            self.by_name
                .entry(field.name.clone())
                .or_default()
                .push(location);
        }
    }

    /// Qualified field path collisions detected while building the index.
    #[must_use]
    pub fn qualified_collisions(&self) -> &[QualifiedFieldCollision] {
        &self.qualified_collisions
    }

    /// Returns all declared interface identifiers.
    #[must_use]
    pub fn interface_ids(&self) -> Vec<&str> {
        self.input_ids
            .iter()
            .chain(self.output_ids.iter())
            .map(String::as_str)
            .collect()
    }

    /// Returns `true` when an identifier is declared on both inputs and outputs.
    #[must_use]
    pub fn has_io_id_collision(&self) -> bool {
        self.input_ids
            .intersection(&self.output_ids)
            .next()
            .is_some()
    }

    /// Resolve a target against declared interfaces and schema fields.
    #[must_use]
    pub fn resolve<'a>(&'a self, target: &str) -> TargetResolution<'a> {
        let target = target.trim();
        if target.is_empty() {
            return TargetResolution::NotFound;
        }

        if let Some(location) = self.qualified.get(target) {
            return TargetResolution::Field(location);
        }

        if let Some(qualified) = resolve_qualified_path(target, self.interface_ids().iter().copied())
        {
            if let Some(location) = self.qualified.get(&qualified) {
                return TargetResolution::Field(location);
            }
            return TargetResolution::NotFound;
        }

        if let Some(matches) = self.by_name.get(target) {
            return match matches.len() {
                0 => TargetResolution::NotFound,
                1 => TargetResolution::Field(&matches[0]),
                _ => TargetResolution::Ambiguous(matches.clone()),
            };
        }

        if self.input_ids.contains(target) {
            return TargetResolution::Interface {
                id: target.to_string(),
                is_input: true,
            };
        }
        if self.output_ids.contains(target) {
            return TargetResolution::Interface {
                id: target.to_string(),
                is_input: false,
            };
        }

        TargetResolution::NotFound
    }

    /// Returns field names that appear in more than one interface schema.
    #[must_use]
    pub fn ambiguous_field_names(&self) -> Vec<String> {
        self.by_name
            .iter()
            .filter(|(_, locations)| locations.len() > 1)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// Resolve a qualified field path using longest-prefix interface matching.
#[must_use]
pub fn resolve_qualified_path<'a>(
    target: &str,
    interface_ids: impl IntoIterator<Item = &'a str>,
) -> Option<String> {
    let target = target.trim();
    if !target.contains('.') {
        return None;
    }
    let mut best: Option<(&str, usize)> = None;
    for id in interface_ids {
        let prefix = format!("{id}.");
        if target.starts_with(&prefix) {
            let len = id.len();
            if best.map_or(true, |(_, bl)| len > bl) {
                best = Some((id, len));
            }
        }
    }
    best.map(|_| target.to_string())
}
