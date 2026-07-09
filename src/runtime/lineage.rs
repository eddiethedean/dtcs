//! Output materialization from lineage mappings.

use std::collections::BTreeMap;

use crate::model::{Field, Input, Output};
use crate::runtime::conversion::apply_matching_conversion;
use crate::runtime::model::{Dataset, Row, RuntimeValue};

/// Materialize an output dataset from contributing inputs.
pub fn materialize_output(
    output: &Output,
    input_ids: &[String],
    plan_inputs: &[Input],
    workspaces: &BTreeMap<String, Dataset>,
) -> Result<Dataset, String> {
    let primary_input = input_ids
        .first()
        .ok_or_else(|| "materialize requires at least one input".to_string())?;
    let source_rows = workspaces
        .get(primary_input)
        .ok_or_else(|| format!("unknown input interface '{primary_input}'"))?;

    let input_field_map = build_input_field_map(plan_inputs);

    let output_fields: Vec<String> = output
        .schema
        .as_ref()
        .map(|schema| schema.fields.iter().map(|f| f.name.clone()).collect())
        .unwrap_or_else(|| {
            source_rows
                .first()
                .map(|row| row.keys().cloned().collect())
                .unwrap_or_default()
        });

    let output_field_defs: BTreeMap<String, &Field> = output
        .schema
        .as_ref()
        .map(|schema| schema.fields.iter().map(|f| (f.name.clone(), f)).collect())
        .unwrap_or_default();

    let mut out_rows = Vec::with_capacity(source_rows.len());
    for (row_index, source_row) in source_rows.iter().enumerate() {
        let mut out_row = Row::new();
        for field_name in &output_fields {
            let mut value = resolve_field_value(field_name, input_ids, workspaces, row_index)
                .unwrap_or_else(|| {
                    source_row
                        .get(field_name)
                        .cloned()
                        .unwrap_or(RuntimeValue::Null)
                });

            if let Some(output_field) = output_field_defs.get(field_name) {
                for input_id in input_ids {
                    if let Some(source_field) =
                        input_field_map.get(&(input_id.as_str(), field_name.as_str()))
                    {
                        if !source_field.conversions.is_empty() {
                            value = apply_matching_conversion(
                                &value,
                                &source_field.conversions,
                                &output_field.type_name,
                            )?;
                            break;
                        }
                    }
                }
            }

            out_row.insert(field_name.to_string(), value);
        }
        out_rows.push(out_row);
    }
    Ok(out_rows)
}

fn build_input_field_map(inputs: &[Input]) -> BTreeMap<(&str, &str), &Field> {
    let mut map = BTreeMap::new();
    for input in inputs {
        if let Some(schema) = &input.schema {
            for field in &schema.fields {
                map.insert((input.id.as_str(), field.name.as_str()), field);
            }
        }
    }
    map
}

fn resolve_field_value(
    field_name: &str,
    input_ids: &[String],
    workspaces: &BTreeMap<String, Dataset>,
    row_index: usize,
) -> Option<RuntimeValue> {
    for input_id in input_ids {
        if let Some(rows) = workspaces.get(input_id) {
            if let Some(row) = rows.get(row_index) {
                if let Some(value) = row.get(field_name) {
                    return Some(value.clone());
                }
            }
        }
    }
    None
}
