//! Runtime input and execution plan validation.

use crate::compile::{validate as validate_compiled_plan, ExecutionPlan};
use crate::diagnostics::DiagnosticReport;
use crate::model::{Input, Schema};
use crate::runtime::model::{Dataset, RuntimeInputs, RuntimeValue};

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

    validate_equal_row_counts(inputs, provided)
}

fn validate_equal_row_counts(inputs: &[Input], provided: &RuntimeInputs) -> Result<(), String> {
    let mut lengths = Vec::new();
    for input in inputs {
        if let Some(dataset) = provided.get(&input.id) {
            if !dataset.is_empty() {
                lengths.push((input.id.clone(), dataset.len()));
            }
        }
    }
    if lengths.len() < 2 {
        return Ok(());
    }
    let expected = lengths[0].1;
    for (id, len) in &lengths[1..] {
        if *len != expected {
            return Err(format!(
                "input '{}' has {len} rows but '{}' has {expected} rows; all provided inputs must have equal row counts",
                id, lengths[0].0
            ));
        }
    }
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
            if !field.nullable && value.map_or(true, RuntimeValue::is_null) {
                return Err(format!(
                    "input '{interface_id}' row {row_index} field '{}' is null but not nullable",
                    field.name
                ));
            }
        }
    }
    Ok(())
}
