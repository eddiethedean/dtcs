//! Runtime input and execution plan validation.

use crate::compile::{validate as validate_compiled_plan, ExecutionPlan};
use crate::diagnostics::DiagnosticReport;
use crate::model::{Input, Schema};
use crate::runtime::model::{Dataset, RuntimeInputs};

/// Validate an execution plan before runtime execution.
#[must_use]
pub fn validate_execution_plan(plan: &ExecutionPlan) -> DiagnosticReport {
    validate_compiled_plan(plan)
}

/// Validate runtime inputs against declared schemas.
pub fn validate_inputs(inputs: &[Input], provided: &RuntimeInputs) -> Result<(), String> {
    for input in inputs {
        if input.optional && !provided.contains_key(&input.id) {
            continue;
        }
        let dataset = if input.optional {
            match provided.get(&input.id) {
                Some(dataset) => dataset,
                None => continue,
            }
        } else {
            provided
                .get(&input.id)
                .ok_or_else(|| format!("missing required input '{}'", input.id))?
        };
        if !input.optional && dataset.is_empty() {
            return Err(format!("required input '{}' has no rows", input.id));
        }
        if let Some(schema) = &input.schema {
            validate_dataset_schema(&input.id, dataset, schema)?;
        }
    }

    // Unequal input lengths are allowed (for example joins). Materialize paths
    // that align by index handle short inputs by producing null/missing cells.
    Ok(())
}

fn validate_dataset_schema(
    interface_id: &str,
    dataset: &Dataset,
    schema: &Schema,
) -> Result<(), String> {
    for (row_index, row) in dataset.iter().enumerate() {
        for field in &schema.fields {
            let value = row.get(&field.name);
            let invalid_for_required = match value {
                None => true,
                Some(v) => !field.nullable && (v.is_null() || v.is_missing() || v.is_invalid()),
            };
            if invalid_for_required {
                return Err(format!(
                    "input '{interface_id}' row {row_index} field '{}' is null/missing/invalid but not nullable",
                    field.name
                ));
            }
        }
    }
    Ok(())
}
