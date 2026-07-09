//! One-off helper to regenerate optimized plan golden files.
//! Run: cargo run --example write_optimize_goldens

use std::fs;
use std::path::PathBuf;

use dtcs::{analysis, optimize, parse, plan, validate, DocumentFormat};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixtures = [
        "optimize_constant_fold.yaml",
        "optimize_algebraic.yaml",
        "optimize_action_fusion.yaml",
        "optimize_function_inline.yaml",
        "optimize_rule_dedup.yaml",
        "optimize_dead_expr.yaml",
    ];

    let out_dir = root.join("tests/fixtures/plans_optimized");
    fs::create_dir_all(&out_dir).expect("create output dir");

    for file in fixtures {
        let path = root.join("tests/fixtures").join(file);
        let content = fs::read(&path).expect("read fixture");
        let contract = parse(&content, DocumentFormat::Yaml)
            .into_contract()
            .expect("contract");
        let report = validate(&contract);
        assert!(report.is_valid(), "{file}: {:?}", report.diagnostics);
        let analysis = analysis::check_contract(&contract, None);
        let lowered = plan::lower(&contract, None, Some(&analysis));
        let original = lowered.plan.expect("plan");
        let optimized = optimize(&original).plan.expect("optimized");
        let golden_name = file.replace(".yaml", ".plan.json");
        let golden_path = out_dir.join(golden_name);
        let json = serde_json::to_string_pretty(&optimized).expect("serialize");
        fs::write(&golden_path, json).expect("write golden");
        println!("wrote {}", golden_path.display());
    }
}
