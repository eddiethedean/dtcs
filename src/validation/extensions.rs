//! Extension validation phase.

use crate::model::TransformationContract;

use super::context::{validate_extension_keys, ValidationContext};

pub(crate) fn validate_extensions(ctx: &mut ValidationContext, contract: &TransformationContract) {
    validate_extension_keys(ctx, contract);
}
