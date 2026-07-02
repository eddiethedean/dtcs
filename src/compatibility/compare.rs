//! Scoped COM comparison (SPEC Chapter 11).

use std::collections::{HashMap, HashSet};

use crate::diagnostics::{codes, Diagnostic, DiagnosticCategory, DiagnosticStage, Severity};
use crate::model::{
    parse_logical_type, type_compatible, Expression, Field, Function, Input, Lineage, Output, Rule,
    SemanticAction, TransformationContract, TypeCompatibility,
};

use super::report::ContractChange;
use super::types::{ChangeCategory, ComparisonScope, DiffKind};

/// Internal comparison outcome used for classification.
#[derive(Debug, Default)]
pub(crate) struct ComparisonOutcome {
    pub diffs: Vec<InternalDiff>,
    pub changes: Vec<ContractChange>,
}

#[derive(Debug, Clone)]
pub(crate) struct InternalDiff {
    pub kind: DiffKind,
    pub category: ChangeCategory,
    pub message: String,
    pub object_ref: Option<String>,
}

impl ComparisonOutcome {
    pub(crate) fn push(&mut self, diff: InternalDiff) {
        if diff.kind != DiffKind::Neutral {
            self.changes.push(ContractChange {
                category: diff.category,
                message: diff.message.clone(),
                object_ref: diff.object_ref.clone(),
            });
        }
        self.diffs.push(diff);
    }

    pub(crate) fn has_breaking(&self) -> bool {
        self.diffs
            .iter()
            .any(|d| matches!(d.kind, DiffKind::Breaking))
    }

    pub(crate) fn has_conditional(&self) -> bool {
        self.diffs
            .iter()
            .any(|d| matches!(d.kind, DiffKind::Conditional))
    }

    pub(crate) fn has_additive(&self) -> bool {
        self.diffs
            .iter()
            .any(|d| matches!(d.kind, DiffKind::Additive))
    }

    pub(crate) fn is_clean(&self) -> bool {
        self.diffs
            .iter()
            .all(|d| matches!(d.kind, DiffKind::Neutral))
    }
}

/// Compare `source` to `target` within the given scope.
#[must_use]
pub fn compare_contracts(
    source: &TransformationContract,
    target: &TransformationContract,
    scope: ComparisonScope,
) -> ComparisonOutcome {
    let mut outcome = ComparisonOutcome::default();

    if source.dtcs_version != target.dtcs_version {
        outcome.push(InternalDiff {
            kind: DiffKind::Neutral,
            category: ChangeCategory::Metadata,
            message: format!(
                "dtcsVersion differs ('{}' vs '{}'); spec version alone does not determine compatibility",
                source.dtcs_version, target.dtcs_version
            ),
            object_ref: Some("dtcsVersion".into()),
        });
    }

    if scope.interfaces || scope.types {
        compare_interfaces(source, target, scope, &mut outcome);
    }
    if scope.semantics {
        compare_semantics(source, target, &mut outcome);
    }
    if scope.lineage {
        compare_lineage(source, target, &mut outcome);
    }
    if scope.metadata {
        compare_metadata(source, target, &mut outcome);
    }
    if scope.extensions {
        compare_extensions(source, target, &mut outcome);
    }

    outcome
}

fn compare_interfaces(
    source: &TransformationContract,
    target: &TransformationContract,
    scope: ComparisonScope,
    outcome: &mut ComparisonOutcome,
) {
    compare_input_set(&source.inputs, &target.inputs, outcome);
    compare_output_set(&source.outputs, &target.outputs, scope.types, outcome);
}

fn compare_input_set(source: &[Input], target: &[Input], outcome: &mut ComparisonOutcome) {
    let source_map: HashMap<_, _> = source.iter().map(|i| (i.id.as_str(), i)).collect();
    let target_map: HashMap<_, _> = target.iter().map(|i| (i.id.as_str(), i)).collect();

    for (id, src) in &source_map {
        let Some(tgt) = target_map.get(id) else {
            if src.optional {
                outcome.push(InternalDiff {
                    kind: DiffKind::Additive,
                    category: ChangeCategory::Interface,
                    message: format!("optional input '{id}' present in source but not in target"),
                    object_ref: Some(format!("inputs.{id}")),
                });
                continue;
            }
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Interface,
                message: format!("required input '{id}' removed in target"),
                object_ref: Some(format!("inputs.{id}")),
            });
            continue;
        };
        if src.optional != tgt.optional && !tgt.optional {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Interface,
                message: format!("input '{id}' changed from optional to required"),
                object_ref: Some(format!("inputs.{id}.optional")),
            });
        }
    }

    for (id, tgt) in &target_map {
        if !source_map.contains_key(id) {
            outcome.push(InternalDiff {
                kind: if tgt.optional {
                    DiffKind::Additive
                } else {
                    DiffKind::Breaking
                },
                category: ChangeCategory::Interface,
                message: format!("input '{id}' added in target"),
                object_ref: Some(format!("inputs.{id}")),
            });
        }
    }

    for (id, src) in &source_map {
        if let Some(tgt) = target_map.get(id) {
            compare_schemas(
                &format!("inputs.{id}.schema"),
                src.schema.as_ref(),
                tgt.schema.as_ref(),
                outcome,
            );
        }
    }
}

fn compare_output_set(
    source: &[Output],
    target: &[Output],
    compare_types: bool,
    outcome: &mut ComparisonOutcome,
) {
    let source_map: HashMap<_, _> = source.iter().map(|o| (o.id.as_str(), o)).collect();
    let target_map: HashMap<_, _> = target.iter().map(|o| (o.id.as_str(), o)).collect();

    for id in source_map.keys() {
        if !target_map.contains_key(id) {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Interface,
                message: format!("output '{id}' removed in target"),
                object_ref: Some(format!("outputs.{id}")),
            });
        }
    }

    for id in target_map.keys() {
        if !source_map.contains_key(id) {
            outcome.push(InternalDiff {
                kind: DiffKind::Additive,
                category: ChangeCategory::Interface,
                message: format!("output '{id}' added in target"),
                object_ref: Some(format!("outputs.{id}")),
            });
        }
    }

    if compare_types {
        for (id, src) in &source_map {
            if let Some(tgt) = target_map.get(id) {
                compare_schemas(
                    &format!("outputs.{id}.schema"),
                    src.schema.as_ref(),
                    tgt.schema.as_ref(),
                    outcome,
                );
            }
        }
    }
}

fn compare_schemas(
    object_ref: &str,
    source: Option<&crate::model::Schema>,
    target: Option<&crate::model::Schema>,
    outcome: &mut ComparisonOutcome,
) {
    match (source, target) {
        (None, None) => {}
        (Some(_), None) | (None, Some(_)) => {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Type,
                message: format!("schema presence changed at {object_ref}"),
                object_ref: Some(object_ref.into()),
            });
        }
        (Some(src_schema), Some(tgt_schema)) => {
            compare_fields(object_ref, &src_schema.fields, &tgt_schema.fields, outcome);
        }
    }
}

fn compare_fields(
    object_ref: &str,
    source: &[Field],
    target: &[Field],
    outcome: &mut ComparisonOutcome,
) {
    let source_map: HashMap<_, _> = source.iter().map(|f| (f.name.as_str(), f)).collect();
    let target_map: HashMap<_, _> = target.iter().map(|f| (f.name.as_str(), f)).collect();

    for (name, src) in &source_map {
        let Some(tgt) = target_map.get(name) else {
            outcome.push(InternalDiff {
                kind: if src.nullable {
                    DiffKind::Additive
                } else {
                    DiffKind::Breaking
                },
                category: ChangeCategory::Type,
                message: format!("field '{name}' removed from {object_ref}"),
                object_ref: Some(format!("{object_ref}.fields.{name}")),
            });
            continue;
        };
        compare_field_types(object_ref, name, src, tgt, outcome);
    }

    for name in target_map.keys() {
        if !source_map.contains_key(name) {
            outcome.push(InternalDiff {
                kind: DiffKind::Additive,
                category: ChangeCategory::Type,
                message: format!("field '{name}' added to {object_ref}"),
                object_ref: Some(format!("{object_ref}.fields.{name}")),
            });
        }
    }
}

fn compare_field_types(
    object_ref: &str,
    name: &str,
    source: &Field,
    target: &Field,
    outcome: &mut ComparisonOutcome,
) {
    let src_type = parse_logical_type(&source.type_name).ok();
    let tgt_type = parse_logical_type(&target.type_name).ok();

    if source.type_name != target.type_name {
        if let (Some(src_t), Some(tgt_t)) = (&src_type, &tgt_type) {
            match type_compatible(src_t, tgt_t) {
                TypeCompatibility::Identical => {}
                TypeCompatibility::Compatible => {
                    outcome.push(InternalDiff {
                        kind: DiffKind::Additive,
                        category: ChangeCategory::Type,
                        message: format!(
                            "field '{name}' type widened from '{}' to '{}'",
                            source.type_name, target.type_name
                        ),
                        object_ref: Some(format!("{object_ref}.fields.{name}.type")),
                    });
                }
                TypeCompatibility::Incompatible => {
                    outcome.push(InternalDiff {
                        kind: DiffKind::Breaking,
                        category: ChangeCategory::Type,
                        message: format!(
                            "field '{name}' type changed from '{}' to '{}'",
                            source.type_name, target.type_name
                        ),
                        object_ref: Some(format!("{object_ref}.fields.{name}.type")),
                    });
                }
            }
        } else {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Type,
                message: format!(
                    "field '{name}' type changed from '{}' to '{}'",
                    source.type_name, target.type_name
                ),
                object_ref: Some(format!("{object_ref}.fields.{name}.type")),
            });
        }
    }

    if source.nullable && !target.nullable {
        outcome.push(InternalDiff {
            kind: DiffKind::Breaking,
            category: ChangeCategory::Type,
            message: format!("field '{name}' changed from nullable to non-nullable"),
            object_ref: Some(format!("{object_ref}.fields.{name}.nullable")),
        });
    } else if !source.nullable && target.nullable {
        outcome.push(InternalDiff {
            kind: DiffKind::Additive,
            category: ChangeCategory::Type,
            message: format!("field '{name}' changed from non-nullable to nullable"),
            object_ref: Some(format!("{object_ref}.fields.{name}.nullable")),
        });
    }
}

fn compare_semantics(
    source: &TransformationContract,
    target: &TransformationContract,
    outcome: &mut ComparisonOutcome,
) {
    compare_actions(&source.semantic_actions, &target.semantic_actions, outcome);
    compare_expressions(&source.expressions, &target.expressions, outcome);
    compare_functions(&source.functions, &target.functions, outcome);
    compare_rules(&source.rules, &target.rules, outcome);
}

fn compare_actions(
    source: &[SemanticAction],
    target: &[SemanticAction],
    outcome: &mut ComparisonOutcome,
) {
    diff_by_id(
        "semanticActions",
        source,
        target,
        |s, t| s.action == t.action && s.target == t.target,
        ChangeCategory::Semantic,
        outcome,
    );
}

fn compare_expressions(
    source: &[Expression],
    target: &[Expression],
    outcome: &mut ComparisonOutcome,
) {
    diff_by_id(
        "expressions",
        source,
        target,
        |s, t| s.expr == t.expr && s.type_name == t.type_name,
        ChangeCategory::Semantic,
        outcome,
    );
}

fn compare_functions(source: &[Function], target: &[Function], outcome: &mut ComparisonOutcome) {
    diff_by_id(
        "functions",
        source,
        target,
        |s, t| {
            s.function == t.function && s.type_name == t.type_name && s.parameters == t.parameters
        },
        ChangeCategory::Semantic,
        outcome,
    );
}

fn compare_rules(source: &[Rule], target: &[Rule], outcome: &mut ComparisonOutcome) {
    diff_by_id(
        "rules",
        source,
        target,
        |s, t| s.rule == t.rule && s.target == t.target && s.phase == t.phase,
        ChangeCategory::Rule,
        outcome,
    );
}

fn diff_by_id<T, F>(
    collection: &str,
    source: &[T],
    target: &[T],
    same: F,
    category: ChangeCategory,
    outcome: &mut ComparisonOutcome,
) where
    T: Identifiable,
    F: Fn(&T, &T) -> bool,
{
    let source_map: HashMap<_, _> = source.iter().map(|item| (item.id(), item)).collect();
    let target_map: HashMap<_, _> = target.iter().map(|item| (item.id(), item)).collect();

    for id in source_map.keys() {
        if !target_map.contains_key(id) {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category,
                message: format!("{collection} entry '{id}' removed in target"),
                object_ref: Some(format!("{collection}.{id}")),
            });
        }
    }

    for id in target_map.keys() {
        if !source_map.contains_key(id) {
            outcome.push(InternalDiff {
                kind: DiffKind::Additive,
                category,
                message: format!("{collection} entry '{id}' added in target"),
                object_ref: Some(format!("{collection}.{id}")),
            });
        }
    }

    for (id, src) in &source_map {
        if let Some(tgt) = target_map.get(id) {
            if !same(src, tgt) {
                outcome.push(InternalDiff {
                    kind: DiffKind::Breaking,
                    category,
                    message: format!("{collection} entry '{id}' changed"),
                    object_ref: Some(format!("{collection}.{id}")),
                });
            }
        }
    }
}

trait Identifiable {
    fn id(&self) -> &str;
}

impl Identifiable for SemanticAction {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identifiable for Expression {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identifiable for Function {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identifiable for Rule {
    fn id(&self) -> &str {
        &self.id
    }
}

fn compare_lineage(
    source: &TransformationContract,
    target: &TransformationContract,
    outcome: &mut ComparisonOutcome,
) {
    compare_lineage_maps(source.lineage.as_ref(), target.lineage.as_ref(), outcome);
}

fn compare_lineage_maps(
    source: Option<&Lineage>,
    target: Option<&Lineage>,
    outcome: &mut ComparisonOutcome,
) {
    let src_mappings = source.map(|l| &l.mappings).cloned().unwrap_or_default();
    let tgt_mappings = target.map(|l| &l.mappings).cloned().unwrap_or_default();
    let src_map: HashMap<_, _> = src_mappings
        .iter()
        .map(|m| (m.output.as_str(), m))
        .collect();
    let tgt_map: HashMap<_, _> = tgt_mappings
        .iter()
        .map(|m| (m.output.as_str(), m))
        .collect();

    for (output, src) in &src_map {
        let Some(tgt) = tgt_map.get(output) else {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Lineage,
                message: format!("lineage mapping for output '{output}' removed"),
                object_ref: Some(format!("lineage.mappings.{output}")),
            });
            continue;
        };
        let src_inputs: HashSet<_> = src.inputs.iter().map(String::as_str).collect();
        let tgt_inputs: HashSet<_> = tgt.inputs.iter().map(String::as_str).collect();
        if src_inputs != tgt_inputs {
            outcome.push(InternalDiff {
                kind: DiffKind::Breaking,
                category: ChangeCategory::Lineage,
                message: format!("lineage inputs changed for output '{output}'"),
                object_ref: Some(format!("lineage.mappings.{output}.inputs")),
            });
        }
    }

    for output in tgt_map.keys() {
        if !src_map.contains_key(output) {
            outcome.push(InternalDiff {
                kind: DiffKind::Additive,
                category: ChangeCategory::Lineage,
                message: format!("lineage mapping for output '{output}' added"),
                object_ref: Some(format!("lineage.mappings.{output}")),
            });
        }
    }
}

fn compare_metadata(
    source: &TransformationContract,
    target: &TransformationContract,
    outcome: &mut ComparisonOutcome,
) {
    let src = source.metadata.as_ref();
    let tgt = target.metadata.as_ref();

    let src_gov = src.and_then(|m| m.governance.as_ref());
    let tgt_gov = tgt.and_then(|m| m.governance.as_ref());

    if src_gov.map(|g| g.owner.as_deref()) != tgt_gov.map(|g| g.owner.as_deref()) {
        outcome.push(InternalDiff {
            kind: DiffKind::Neutral,
            category: ChangeCategory::Metadata,
            message: "governance owner differs".into(),
            object_ref: Some("metadata.governance.owner".into()),
        });
    }

    let src_doc = src.and_then(|m| m.documentation.as_ref());
    let tgt_doc = tgt.and_then(|m| m.documentation.as_ref());
    if src_doc.map(|d| &d.description) != tgt_doc.map(|d| &d.description) {
        outcome.push(InternalDiff {
            kind: DiffKind::Neutral,
            category: ChangeCategory::Metadata,
            message: "documentation description differs (non-breaking)".into(),
            object_ref: Some("metadata.documentation.description".into()),
        });
    }
}

fn compare_extensions(
    source: &TransformationContract,
    target: &TransformationContract,
    outcome: &mut ComparisonOutcome,
) {
    let src_keys: HashSet<_> = source.extensions.keys().collect();
    let tgt_keys: HashSet<_> = target.extensions.keys().collect();

    for key in src_keys.difference(&tgt_keys) {
        outcome.push(InternalDiff {
            kind: DiffKind::Conditional,
            category: ChangeCategory::Extension,
            message: format!("extension key '{key}' removed in target"),
            object_ref: Some(key.to_string()),
        });
    }

    for key in tgt_keys.difference(&src_keys) {
        outcome.push(InternalDiff {
            kind: DiffKind::Additive,
            category: ChangeCategory::Extension,
            message: format!("extension key '{key}' added in target"),
            object_ref: Some(key.to_string()),
        });
    }
}

/// Build compatibility diagnostics from comparison outcome.
#[must_use]
pub fn diagnostics_from_outcome(outcome: &ComparisonOutcome) -> Vec<Diagnostic> {
    outcome
        .diffs
        .iter()
        .filter(|d| d.kind != DiffKind::Neutral)
        .map(|d| {
            let (id, severity) = match d.kind {
                DiffKind::Breaking => (codes::INCOMPATIBLE_CONTRACT, Severity::Error),
                DiffKind::Conditional => (codes::CONDITIONAL_COMPATIBILITY, Severity::Warning),
                DiffKind::Additive => (codes::CONDITIONAL_COMPATIBILITY, Severity::Warning),
                DiffKind::Neutral => (codes::CONDITIONAL_COMPATIBILITY, Severity::Warning),
            };
            Diagnostic::new(
                id,
                severity,
                DiagnosticStage::Analysis,
                DiagnosticCategory::Compatibility,
                &d.message,
            )
            .with_object_ref(d.object_ref.clone().unwrap_or_default())
        })
        .collect()
}
