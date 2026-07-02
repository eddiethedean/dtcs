//! Function model (declarations only for MVP).

use serde::{Deserialize, Serialize};

/// A reusable function definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    /// Stable function identifier.
    pub id: String,
    /// Function registry identifier.
    pub function: String,
}
