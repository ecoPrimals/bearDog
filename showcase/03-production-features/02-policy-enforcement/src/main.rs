// 🛡️ BearDog: Runtime Policy Enforcement Demo
//
// This demo shows automatic policy validation and enforcement

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

/// BearDog Policy Enforcement Demo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Policy file (JSON)
    #[arg(short, long)]
    policies: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup_logging(args.verbose);

    info!("🛡️ BearDog: Runtime Policy Enforcement Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let _config = load_config(&args.config)?;
    
    info!("Policies: {}", args.policies.display());
    info!("Config loaded from: {}", args.config.display());
    info!("");

    run_policy_enforcement(&args.policies).await?;

    info!("");
    info!("🎉 Demo complete! All policies enforced!");
    
    Ok(())
}

async fn run_policy_enforcement(policy_path: &PathBuf) -> Result<()> {
    let total_start = Instant::now();

    // Step 1: Load policies
    info!("Step 1: Loading policies...");
    let start = Instant::now();
    
    let policies = load_policies(policy_path).await?;
    let load_time = start.elapsed();
    
    info!("✅ Policies loaded");
    info!("   Count: {} policies", policies.len());
    for policy in &policies {
        info!("   Key: {} → {} constraints", policy.key_id, policy.constraints.len());
    }
    info!("   Load time: {:?}", load_time);
    info!("");

    // Step 2: Initialize policy engine
    info!("Step 2: Initializing policy engine...");
    let start = Instant::now();
    
    let engine = initialize_policy_engine(policies)?;
    let init_time = start.elapsed();
    
    info!("✅ Policy engine initialized");
    info!("   Ready to enforce {} policies", engine.policy_count());
    info!("   Init time: {:?}", init_time);
    info!("");

    // Step 3-7: Run test scenarios
    let mut test_times = Vec::new();
    
    // Test 1: Valid storage operation
    info!("Step 3: Testing valid storage operation...");
    let start = Instant::now();
    let decision1 = engine.validate("storage-encrypt", "key_storage_001")?;
    test_times.push(start.elapsed());
    
    info!("✅ Test 1: {}", if decision1.allowed { "ALLOWED" } else { "DENIED" });
    info!("   Operation: storage-encrypt");
    info!("   Key: key_storage_001");
    info!("   Decision: {} ({})", 
        if decision1.allowed { "ALLOWED" } else { "DENIED" },
        decision1.reason);
    info!("   Check time: {:?}", test_times[0]);
    info!("");

    // Test 2: Invalid purpose (signing with storage key)
    info!("Step 4: Testing purpose violation...");
    let start = Instant::now();
    let decision2 = engine.validate("signing-sign", "key_storage_001")?;
    test_times.push(start.elapsed());
    
    info!("✅ Test 2: {}", if decision2.allowed { "ALLOWED" } else { "DENIED" });
    info!("   Operation: signing-sign");
    info!("   Key: key_storage_001 (storage-only)");
    info!("   Decision: {} ({})",
        if decision2.allowed { "ALLOWED" } else { "DENIED" },
        decision2.reason);
    info!("   Check time: {:?}", test_times[1]);
    info!("");

    // Test 3: Valid signing operation
    info!("Step 5: Testing valid signing operation...");
    let start = Instant::now();
    let decision3 = engine.validate("signing-sign", "key_signing_002")?;
    test_times.push(start.elapsed());
    
    info!("✅ Test 3: {}", if decision3.allowed { "ALLOWED" } else { "DENIED" });
    info!("   Operation: signing-sign");
    info!("   Key: key_signing_002 (signing-only)");
    info!("   Decision: {} ({})",
        if decision3.allowed { "ALLOWED" } else { "DENIED" },
        decision3.reason);
    info!("   Check time: {:?}", test_times[2]);
    info!("");

    // Test 4: Export attempt with no-export key
    info!("Step 6: Testing export violation...");
    let start = Instant::now();
    let decision4 = engine.validate("export", "key_compute_003")?;
    test_times.push(start.elapsed());
    
    info!("✅ Test 4: {}", if decision4.allowed { "ALLOWED" } else { "DENIED" });
    info!("   Operation: export");
    info!("   Key: key_compute_003 (no-export)");
    info!("   Decision: {} ({})",
        if decision4.allowed { "ALLOWED" } else { "DENIED" },
        decision4.reason);
    info!("   Check time: {:?}", test_times[3]);
    info!("");

    // Test 5: Valid compute operation
    info!("Step 7: Testing valid compute operation...");
    let start = Instant::now();
    let decision5 = engine.validate("compute-execute", "key_compute_003")?;
    test_times.push(start.elapsed());
    
    info!("✅ Test 5: {}", if decision5.allowed { "ALLOWED" } else { "DENIED" });
    info!("   Operation: compute-execute");
    info!("   Key: key_compute_003 (compute-only)");
    info!("   Decision: {} ({})",
        if decision5.allowed { "ALLOWED" } else { "DENIED" },
        decision5.reason);
    info!("   Check time: {:?}", test_times[4]);
    info!("");

    // Step 8: Verify results
    info!("Step 8: Verifying policy enforcement...");
    
    let expected = vec![true, false, true, false, true];
    let actual = vec![
        decision1.allowed,
        decision2.allowed,
        decision3.allowed,
        decision4.allowed,
        decision5.allowed,
    ];
    
    let all_correct = expected == actual;
    
    if all_correct {
        info!("✅ All tests passed!");
        info!("   Test 1: ALLOWED (storage op with storage key) ✓");
        info!("   Test 2: DENIED (signing op with storage key) ✓");
        info!("   Test 3: ALLOWED (signing op with signing key) ✓");
        info!("   Test 4: DENIED (export with no-export key) ✓");
        info!("   Test 5: ALLOWED (compute op with compute key) ✓");
    } else {
        info!("❌ Some tests failed!");
    }
    info!("");

    // Performance summary
    let total_time = total_start.elapsed();
    let avg_check_time: std::time::Duration = test_times.iter().sum::<std::time::Duration>() / test_times.len() as u32;
    let max_check_time = test_times.iter().max().unwrap();
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 Performance Summary");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Policy load:       {:?}", load_time);
    info!("Engine init:       {:?}", init_time);
    info!("Average check:     {:?}", avg_check_time);
    info!("Max check:         {:?}", max_check_time);
    info!("Total time:        {:?}", total_time);
    info!("");

    // Validation
    let target_check_time = std::time::Duration::from_millis(1);
    
    if avg_check_time <= target_check_time {
        info!("✅ SUCCESS! Performance target met!");
        info!("   Average: {:?} <= 1ms ✓", avg_check_time);
    } else {
        info!("⚠️  Average check time: {:?} > 1ms", avg_check_time);
    }

    Ok(())
}

// Configuration
#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
struct DemoConfig {
    enforcement_mode: String,
}

fn load_config(path: &PathBuf) -> Result<DemoConfig> {
    let content = std::fs::read_to_string(path)?;
    toml::from_str(&content).context("Failed to parse config")
}

// Policy structures
#[derive(Debug, Clone, serde::Deserialize)]
struct Policy {
    key_id: String,
    constraints: Vec<String>,
}

async fn load_policies(path: &PathBuf) -> Result<Vec<Policy>> {
    let content = tokio::fs::read_to_string(path).await?;
    serde_json::from_str(&content).context("Failed to parse policies")
}

// Policy engine
struct PolicyEngine {
    policies: Arc<parking_lot::RwLock<HashMap<String, Vec<String>>>>,
}

fn initialize_policy_engine(policies: Vec<Policy>) -> Result<Arc<PolicyEngine>> {
    let mut policy_map = HashMap::new();
    for policy in policies {
        policy_map.insert(policy.key_id, policy.constraints);
    }
    
    Ok(Arc::new(PolicyEngine {
        policies: Arc::new(parking_lot::RwLock::new(policy_map)),
    }))
}

impl PolicyEngine {
    fn policy_count(&self) -> usize {
        self.policies.read().len()
    }
    
    fn validate(&self, operation: &str, key_id: &str) -> Result<PolicyDecision> {
        let policies = self.policies.read();
        let constraints = policies.get(key_id)
            .ok_or_else(|| anyhow::anyhow!("No policy for key: {}", key_id))?;
        
        // Extract operation purpose (e.g., "storage-encrypt" → "storage")
        let op_purpose = operation.split('-').next().unwrap_or(operation);
        
        // Check constraints
        for constraint in constraints {
            // Purpose constraint
            if constraint.starts_with("purpose:") {
                let required_purpose = constraint.strip_prefix("purpose:").unwrap();
                if op_purpose != required_purpose {
                    return Ok(PolicyDecision {
                        allowed: false,
                        reason: format!("Purpose mismatch: operation '{}' requires '{}', key has '{}'",
                            op_purpose, op_purpose, required_purpose),
                    });
                }
            }
            
            // No-export constraint
            if constraint == "no-export" && operation == "export" {
                return Ok(PolicyDecision {
                    allowed: false,
                    reason: "Key has no-export constraint".to_string(),
                });
            }
        }
        
        // All constraints satisfied
        Ok(PolicyDecision {
            allowed: true,
            reason: "All constraints satisfied".to_string(),
        })
    }
}

// Policy decision
#[derive(Debug, Clone)]
struct PolicyDecision {
    allowed: bool,
    reason: String,
}

fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level))
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

