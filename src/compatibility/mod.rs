//! Compatibility evaluation (SPEC Chapter 11–12).

mod classify;
mod compare;
mod evolution;
mod report;
mod types;

pub use classify::analyze;
pub use evolution::analyze_evolution;
pub use report::{CompatibilityReport, ContractChange, EvolutionReport};
pub use types::{ChangeCategory, ComparisonScope, CompatibilityLevel};
