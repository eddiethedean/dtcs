//! Lineage analysis (SPEC Chapter 10 §11).

mod analysis;

pub use analysis::{
    analyze, analyze_with_options, LineageAnalysisReport, LineageEdge, LineageGovernance,
};
