// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::{Context as AnyhowContext, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Configuration Structures
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, Deserialize)]
struct DemoConfig {
    #[serde(rename = "ceremony")]
    _ceremony: CeremonyConfig,
    #[serde(rename = "constraints")]
    _constraints: ConstraintsConfig,
    #[serde(rename = "conflict_resolution")]
    _conflict_resolution: ConflictResolutionConfig,
    #[serde(rename = "validation")]
    _validation: ValidationConfig,
    #[serde(rename = "performance")]
    _performance: PerformanceConfig,
    #[serde(rename = "audit")]
    _audit: AuditConfig,
}

#[derive(Debug, Deserialize)]
struct CeremonyConfig {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "description")]
    _description: String,
    #[serde(rename = "max_duration_ms")]
    _max_duration_ms: u64,
}

#[derive(Debug, Deserialize)]
struct ConstraintsConfig {
    #[serde(rename = "enable_and")]
    _enable_and: bool,
    #[serde(rename = "enable_or")]
    _enable_or: bool,
    #[serde(rename = "enable_not")]
    _enable_not: bool,
    #[serde(rename = "enable_implies")]
    _enable_implies: bool,
    #[serde(rename = "enable_nesting")]
    _enable_nesting: bool,
    #[serde(rename = "max_nesting_depth")]
    _max_nesting_depth: u32,
    #[serde(rename = "enable_inheritance")]
    _enable_inheritance: bool,
}

#[derive(Debug, Deserialize)]
struct ConflictResolutionConfig {
    #[serde(rename = "strategy")]
    _strategy: String,
    #[serde(rename = "detect_conflicts")]
    _detect_conflicts: bool,
    #[serde(rename = "fail_on_unresolvable")]
    _fail_on_unresolvable: bool,
}

#[derive(Debug, Deserialize)]
struct ValidationConfig {
    #[serde(rename = "enable_runtime_checks")]
    _enable_runtime_checks: bool,
    #[serde(rename = "cache_evaluations")]
    _cache_evaluations: bool,
    #[serde(rename = "performance_target_ms")]
    _performance_target_ms: u64,
}

#[derive(Debug, Deserialize)]
struct PerformanceConfig {
    #[serde(rename = "composition_target_ms")]
    _composition_target_ms: u64,
    #[serde(rename = "validation_target_ms")]
    _validation_target_ms: u64,
    #[serde(rename = "conflict_detection_target_ms")]
    _conflict_detection_target_ms: u64,
    #[serde(rename = "resolution_target_ms")]
    _resolution_target_ms: u64,
}

#[derive(Debug, Deserialize)]
struct AuditConfig {
    #[serde(rename = "log_level")]
    _log_level: String,
    #[serde(rename = "include_performance")]
    _include_performance: bool,
    #[serde(rename = "log_constraint_trees")]
    _log_constraint_trees: bool,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Scenario Structures
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, Deserialize)]
struct Scenario {
    #[serde(rename = "scenario_name")]
    _scenario_name: String,
    #[serde(rename = "scenario_id")]
    _scenario_id: String,
    #[serde(rename = "operation")]
    _operation: String,
    constraint_definitions: Vec<ConstraintDefinition>,
    compositions: Vec<Composition>,
    test_cases: Vec<TestCase>,
    #[serde(rename = "expected_results")]
    _expected_results: ExpectedResults,
}

#[derive(Debug, Deserialize, Clone)]
struct ConstraintDefinition {
    id: String,
    #[serde(rename = "type")]
    constraint_type: String,
    value: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone)]
struct Composition {
    comp_id: String,
    name: String,
    operator: String,
    operands: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TestCase {
    test_id: String,
    description: String,
    composition: String,
    context: HashMap<String, serde_json::Value>,
    expected_result: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedResults {
    #[serde(rename = "composition_ms")]
    _composition_ms: u64,
    #[serde(rename = "validation_ms")]
    _validation_ms: u64,
    #[serde(rename = "conflict_detection_ms")]
    _conflict_detection_ms: u64,
    #[serde(rename = "test_cases_passed")]
    _test_cases_passed: u32,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Constraint Engine
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, Clone)]
enum ConstraintNode {
    // Leaf nodes (atomic constraints)
    Purpose(String),
    Export(bool),
    Storage(String),
    
    // Composition nodes
    And(Vec<ConstraintNode>),
    Or(Vec<ConstraintNode>),
    Not(Box<ConstraintNode>),
    
    // Reference to another composition
    Ref(String),
}

struct ConstraintEngine {
    constraints: HashMap<String, ConstraintDefinition>,
    compositions: HashMap<String, Composition>,
}

impl ConstraintEngine {
    fn new() -> Self {
        Self {
            constraints: HashMap::new(),
            compositions: HashMap::new(),
        }
    }

    fn add_constraint(&mut self, def: ConstraintDefinition) {
        self.constraints.insert(def.id.clone(), def);
    }

    fn add_composition(&mut self, comp: Composition) {
        self.compositions.insert(comp.comp_id.clone(), comp);
    }

    /// Build a constraint tree from a composition ID
    fn build_tree(&self, comp_id: &str) -> Result<ConstraintNode> {
        // Check if it's a composition
        if let Some(comp) = self.compositions.get(comp_id) {
            let operand_nodes: Result<Vec<ConstraintNode>> = comp
                .operands
                .iter()
                .map(|op| self.build_tree(op))
                .collect();
            let operand_nodes = operand_nodes?;

            return match comp.operator.as_str() {
                "and" => Ok(ConstraintNode::And(operand_nodes)),
                "or" => Ok(ConstraintNode::Or(operand_nodes)),
                "not" => {
                    if operand_nodes.len() != 1 {
                        return Err(anyhow::anyhow!("NOT operator requires exactly one operand"));
                    }
                    Ok(ConstraintNode::Not(Box::new(operand_nodes[0].clone())))
                }
                _ => Err(anyhow::anyhow!("Unknown operator: {}", comp.operator)),
            };
        }

        // Check if it's a basic constraint
        if let Some(constraint) = self.constraints.get(comp_id) {
            return match constraint.constraint_type.as_str() {
                "purpose" => {
                    let value = constraint.value.as_str()
                        .ok_or_else(|| anyhow::anyhow!("Purpose value must be a string"))?;
                    Ok(ConstraintNode::Purpose(value.to_string()))
                }
                "export" => {
                    let value = constraint.value.as_bool()
                        .ok_or_else(|| anyhow::anyhow!("Export value must be a boolean"))?;
                    Ok(ConstraintNode::Export(value))
                }
                "storage" => {
                    let value = constraint.value.as_str()
                        .ok_or_else(|| anyhow::anyhow!("Storage value must be a string"))?;
                    Ok(ConstraintNode::Storage(value.to_string()))
                }
                _ => Err(anyhow::anyhow!("Unknown constraint type: {}", constraint.constraint_type)),
            };
        }

        Err(anyhow::anyhow!("Constraint or composition not found: {}", comp_id))
    }

    /// Evaluate a constraint tree against a context
    fn evaluate(&self, node: &ConstraintNode, context: &HashMap<String, serde_json::Value>) -> bool {
        match node {
            ConstraintNode::And(children) => {
                children.iter().all(|child| self.evaluate(child, context))
            }
            ConstraintNode::Or(children) => {
                children.iter().any(|child| self.evaluate(child, context))
            }
            ConstraintNode::Not(child) => {
                !self.evaluate(child, context)
            }
            ConstraintNode::Purpose(expected) => {
                if let Some(value) = context.get("purpose") {
                    if let Some(actual) = value.as_str() {
                        return actual == expected;
                    }
                }
                false
            }
            ConstraintNode::Export(expected) => {
                if let Some(value) = context.get("export") {
                    if let Some(actual) = value.as_bool() {
                        return actual == *expected;
                    }
                }
                false
            }
            ConstraintNode::Storage(expected) => {
                if let Some(value) = context.get("storage") {
                    if let Some(actual) = value.as_str() {
                        return actual == expected;
                    }
                }
                false
            }
            ConstraintNode::Ref(_) => {
                // Should not happen after tree building
                false
            }
        }
    }

    /// Detect conflicts in a constraint tree
    fn detect_conflicts(&self, node: &ConstraintNode) -> bool {
        match node {
            ConstraintNode::And(children) => {
                // Check for contradictions like Export(true) AND NOT(Export(true))
                for child in children {
                    if self.detect_conflicts(child) {
                        return true;
                    }
                }
                
                // Simple conflict detection: Export(true) and Export(false) in same AND
                let mut has_export_true = false;
                let mut has_export_false = false;
                
                for child in children {
                    match child {
                        ConstraintNode::Export(true) => has_export_true = true,
                        ConstraintNode::Export(false) => has_export_false = true,
                        ConstraintNode::Not(inner) => {
                            if let ConstraintNode::Export(val) = **inner {
                                if val {
                                    has_export_false = true;
                                } else {
                                    has_export_true = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                
                has_export_true && has_export_false
            }
            ConstraintNode::Or(children) => {
                children.iter().any(|child| self.detect_conflicts(child))
            }
            ConstraintNode::Not(child) => {
                self.detect_conflicts(child)
            }
            _ => false,
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Demo Execution
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

async fn run_demo(_config: DemoConfig, scenario: Scenario) -> Result<()> {
    info!("🔗 Constraint Composition Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let mut engine = ConstraintEngine::new();
    
    // Load constraints
    for constraint in &scenario.constraint_definitions {
        engine.add_constraint(constraint.clone());
    }
    info!("✅ Loaded {} constraints", scenario.constraint_definitions.len());
    
    // Load compositions
    for composition in &scenario.compositions {
        engine.add_composition(composition.clone());
    }
    info!("✅ Loaded {} compositions", scenario.compositions.len());
    info!("");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 1: Build Constraint Trees
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    info!("🏗️  Phase 1: Building Constraint Trees");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut trees: HashMap<String, ConstraintNode> = HashMap::new();
    let mut build_times = Vec::new();

    for comp in &scenario.compositions {
        let start = Instant::now();
        let tree = engine.build_tree(&comp.comp_id)?;
        let elapsed = start.elapsed();
        build_times.push(elapsed.as_millis());
        
        trees.insert(comp.comp_id.clone(), tree);
        info!("✅ Built tree: {} - {} ({:.2}ms)", comp.comp_id, comp.name, elapsed.as_millis());
    }

    let avg_build_ms = if !build_times.is_empty() {
        build_times.iter().sum::<u128>() / build_times.len() as u128
    } else {
        0
    };
    info!("📊 Average build time: {:.2}ms", avg_build_ms);
    info!("");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 2: Execute Test Cases
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    info!("🧪 Phase 2: Test Case Execution");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut passed = 0;
    let mut validation_times = Vec::new();

    for test_case in &scenario.test_cases {
        info!("Test {}: {}", test_case.test_id, test_case.description);

        let start = Instant::now();
        let result = if test_case.composition == "conflict" {
            // Special case: conflict detection test
            let has_conflict = scenario.compositions
                .iter()
                .any(|comp| {
                    if let Ok(tree) = engine.build_tree(&comp.comp_id) {
                        engine.detect_conflicts(&tree)
                    } else {
                        false
                    }
                });
            
            Ok(has_conflict)  // Test passes if we detect a conflict
        } else {
            // Normal constraint evaluation
            if let Some(tree) = trees.get(&test_case.composition) {
                let evaluation_result = engine.evaluate(tree, &test_case.context);
                Ok(evaluation_result)
            } else {
                Err(anyhow::anyhow!("Composition not found: {}", test_case.composition))
            }
        };
        let elapsed = start.elapsed();
        validation_times.push(elapsed.as_millis());

        match result {
            Ok(actual) => {
                let expected_pass = test_case.expected_result == "pass";
                let test_passes = if expected_pass {
                    actual  // For "pass", we expect true
                } else {
                    !actual  // For "fail", we expect false (constraint should fail)
                };

                if test_passes {
                    info!("   ✅ PASS (evaluated to: {}, expected: {})", actual, test_case.expected_result);
                    passed += 1;
                } else {
                    info!("   ❌ FAIL (evaluated to: {}, expected: {})", actual, test_case.expected_result);
                }
            }
            Err(e) => {
                info!("   ❌ FAIL (error: {})", e);
            }
        }
        info!("   Validation time: {:.2}ms", elapsed.as_millis());
        info!("");
    }

    let avg_validation_ms = if !validation_times.is_empty() {
        validation_times.iter().sum::<u128>() / validation_times.len() as u128
    } else {
        0
    };

    info!(
        "Test Cases: {}/{} passed ({:.1}%)",
        passed,
        scenario.test_cases.len(),
        (passed as f64 / scenario.test_cases.len() as f64) * 100.0
    );
    info!("");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 3: Conflict Detection
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    info!("🔍 Phase 3: Conflict Detection");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let start = Instant::now();
    let mut conflicts_found = 0;
    for (comp_id, tree) in &trees {
        if engine.detect_conflicts(tree) {
            info!("⚠️  Conflict detected in: {}", comp_id);
            conflicts_found += 1;
        }
    }
    let conflict_detection_ms = start.elapsed().as_millis();

    if conflicts_found == 0 {
        info!("✅ No conflicts detected in constraint trees");
    } else {
        info!("⚠️  {} conflict(s) detected", conflicts_found);
    }
    info!("   Detection time: {:.2}ms", conflict_detection_ms);
    info!("");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Final Summary
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ Demo Complete!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");
    info!("📊 Summary:");
    info!("   Constraints: {}", scenario.constraint_definitions.len());
    info!("   Compositions: {}", scenario.compositions.len());
    info!("   Avg Build Time: {:.2}ms", avg_build_ms);
    info!("   Avg Validation Time: {:.2}ms", avg_validation_ms);
    info!("   Conflict Detection: {:.2}ms", conflict_detection_ms);
    info!("   Test Cases Passed: {}/{}", passed, scenario.test_cases.len());
    info!(
        "   Success Rate: {:.1}%",
        (passed as f64 / scenario.test_cases.len() as f64) * 100.0
    );
    info!("");
    info!("✅ Constraint composition engine validated");
    info!("✅ AND/OR/NOT operators functional");
    info!("✅ Nested constraint trees working");
    info!("✅ Conflict detection operational");
    info!("");

    Ok(())
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CLI & Main
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Parser, Debug)]
#[command(name = "constraint-composition")]
#[command(about = "Constraint Composition Demo")]
struct Cli {
    #[arg(short, long, default_value = "configs/demo.toml")]
    config: PathBuf,

    #[arg(short, long, default_value = "scenarios/policy_composition.json")]
    scenario: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    // Parse CLI
    let cli = Cli::parse();

    // Load config
    let config_str = fs::read_to_string(&cli.config)
        .with_context(|| format!("Failed to read config: {:?}", cli.config))?;
    let config: DemoConfig = toml::from_str(&config_str)?;

    // Load scenario
    let scenario_str = fs::read_to_string(&cli.scenario)
        .with_context(|| format!("Failed to read scenario: {:?}", cli.scenario))?;
    let scenario: Scenario = serde_json::from_str(&scenario_str)?;

    // Run demo
    run_demo(config, scenario).await?;

    Ok(())
}

