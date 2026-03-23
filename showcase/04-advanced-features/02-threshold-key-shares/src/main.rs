// SPDX-License-Identifier: AGPL-3.0-only

// 🔐 Threshold Key Shares Demo
//
// Demonstrates Shamir's Secret Sharing for distributed trust:
// - Split master key into N shares
// - Require M-of-N shares to reconstruct
// - Multi-party authorization workflows
// - Audit trail for all threshold operations

use anyhow::{Context, Result};
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sharks::{Share, Sharks};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// Threshold Key Shares Demo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scenario file (JSON)
    #[arg(short, long)]
    scenario: PathBuf,

    /// Configuration file (TOML)
    #[arg(short, long)]
    config: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Configuration Structures
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct DemoConfig {
    ceremony: CeremonyConfig,
    threshold: ThresholdConfig,
    beardog: BeardogConfig,
    security: SecurityConfig,
    audit: AuditConfig,
    scenarios: ScenariosConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct CeremonyConfig {
    name: String,
    description: String,
    max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ThresholdConfig {
    total_shares: u8,
    threshold: u8,
    share_holders: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BeardogConfig {
    hsm_type: String,
    key_constraints: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SecurityConfig {
    encrypt_shares: bool,
    verify_share_integrity: bool,
    audit_all_operations: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditConfig {
    log_level: String,
    include_performance: bool,
    include_share_usage: bool,
    hash_algorithm: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ScenariosConfig {
    test_insufficient_shares: bool,
    test_exact_threshold: bool,
    test_excess_shares: bool,
}

// ============================================================================
// Scenario Structures
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Scenario {
    scenario_name: String,
    scenario_id: String,
    operation: String,
    amount: String,
    destination: String,
    requester: String,
    approval_policy: ApprovalPolicy,
    test_cases: Vec<TestCase>,
    expected_results: ExpectedResults,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ApprovalPolicy {
    threshold: u8,
    total_shares: u8,
    share_holders: Vec<ShareHolder>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ShareHolder {
    id: String,
    name: String,
    title: String,
    authorization_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TestCase {
    test_id: String,
    description: String,
    shares_provided: Vec<String>,
    expected_result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ExpectedResults {
    share_generation_ms: u64,
    key_reconstruction_ms: u64,
    total_ceremony_ms: u64,
    test_cases_passed: u32,
    audit_trail_complete: bool,
}

// ============================================================================
// Threshold Cryptography Types
// ============================================================================

#[derive(Debug, Clone)]
struct MasterKey {
    id: String,
    secret: Vec<u8>,
    constraints: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Serialize)]
struct DistributedShare {
    share_id: String,
    holder_id: String,
    holder_name: String,
    share_index: u8,
    #[serde(skip)]
    share_object: Option<Share>,  // Actual share object
    created_at: String,
    integrity_hash: String,
}

impl std::fmt::Debug for DistributedShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DistributedShare")
            .field("share_id", &self.share_id)
            .field("holder_id", &self.holder_id)
            .field("holder_name", &self.holder_name)
            .field("share_index", &self.share_index)
            .field("share_object", &"<opaque>")
            .field("created_at", &self.created_at)
            .field("integrity_hash", &self.integrity_hash)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize)]
struct ReconstructionAttempt {
    attempt_id: String,
    test_case_id: String,
    shares_provided: Vec<String>,
    threshold_met: bool,
    reconstruction_successful: bool,
    duration_ms: u64,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CeremonyResult {
    ceremony_id: String,
    scenario_id: String,
    master_key_id: String,
    shares_generated: u8,
    threshold: u8,
    reconstruction_attempts: Vec<ReconstructionAttempt>,
    audit_trail: AuditTrail,
    performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize)]
struct AuditTrail {
    ceremony_hash: String,
    total_operations: u32,
    share_usage_log: Vec<ShareUsageLog>,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
struct ShareUsageLog {
    share_id: String,
    holder_id: String,
    operation: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
struct PerformanceMetrics {
    share_generation_ms: u64,
    key_reconstruction_ms: u64,
    total_ceremony_ms: u64,
    test_cases_executed: u32,
    test_cases_passed: u32,
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let level = if args.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("🔐 Threshold Key Shares Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Using Shamir's Secret Sharing for distributed trust");
    info!("");

    // Load configuration
    let config = load_config(&args.config)
        .context("Failed to load configuration")?;
    info!("📋 Configuration loaded from: {}", args.config.display());

    // Load scenario
    let scenario = load_scenario(&args.scenario)
        .context("Failed to load scenario")?;
    info!("📝 Scenario loaded: {}", scenario.scenario_name);
    info!("");

    // Execute threshold key ceremony
    let ceremony_start = Instant::now();
    let result = execute_threshold_ceremony(&scenario, &config).await?;
    let total_duration = ceremony_start.elapsed();

    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ CEREMONY COMPLETE!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Print results
    print_results(&result, total_duration);

    // Validate against expected results
    validate_results(&result, &scenario.expected_results);

    info!("");
    info!("🎉 Threshold Key Shares Demo Complete!");

    Ok(())
}

// ============================================================================
// Configuration Loading
// ============================================================================

fn load_config(path: &PathBuf) -> Result<DemoConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let config: DemoConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    
    Ok(config)
}

fn load_scenario(path: &PathBuf) -> Result<Scenario> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read scenario file: {}", path.display()))?;
    
    let scenario: Scenario = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse scenario file: {}", path.display()))?;
    
    Ok(scenario)
}

// ============================================================================
// Threshold Ceremony Execution
// ============================================================================

async fn execute_threshold_ceremony(
    scenario: &Scenario,
    config: &DemoConfig,
) -> Result<CeremonyResult> {
    let ceremony_id = format!("ceremony-{}", uuid::Uuid::new_v4());
    
    info!("🚀 Starting Threshold Key Ceremony");
    info!("   Scenario: {}", scenario.scenario_name);
    info!("   Threshold: {}-of-{}", scenario.approval_policy.threshold, scenario.approval_policy.total_shares);
    info!("");

    // Phase 1: Generate Master Key
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 1: Master Key Generation");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let master_key = generate_master_key(&scenario, &config).await?;
    info!("✅ Master key generated: {}", master_key.id);
    info!("   Constraints: {:?}", master_key.constraints);
    info!("");

    // Phase 2: Split into Shares (Shamir's Secret Sharing)
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 2: Share Generation (Shamir's Secret Sharing)");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let share_gen_start = Instant::now();
    let shares = generate_shares(
        &master_key,
        scenario.approval_policy.threshold,
        scenario.approval_policy.total_shares,
        &scenario.approval_policy.share_holders,
    )?;
    let share_gen_duration = share_gen_start.elapsed();

    info!("✅ {} shares generated in {:?}", shares.len(), share_gen_duration);
    for share in &shares {
        info!("   - Share {}: {} ({})", 
              share.share_index, share.holder_name, share.holder_id);
    }
    info!("");

    // Phase 3: Run Test Cases
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 3: Threshold Reconstruction Tests");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let mut reconstruction_attempts = Vec::new();
    let mut total_reconstruction_ms = 0u64;

    for test_case in &scenario.test_cases {
        info!("Test: {}", test_case.description);
        info!("  Shares: {:?}", test_case.shares_provided);

        let recon_start = Instant::now();
        let attempt = test_reconstruction(
            &master_key,
            &shares,
            &test_case,
            scenario.approval_policy.threshold,
        ).await?;
        let recon_duration = recon_start.elapsed();
        total_reconstruction_ms += recon_duration.as_millis() as u64;

        let status = if attempt.reconstruction_successful { "✅ PASS" } else { "❌ FAIL" };
        info!("  Result: {} ({}ms)", status, recon_duration.as_millis());
        if let Some(error) = &attempt.error {
            info!("  Error: {}", error);
        }
        info!("");

        reconstruction_attempts.push(attempt);
    }

    // Phase 4: Generate Audit Trail
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 4: Audit Trail Generation");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let audit_trail = generate_audit_trail(&ceremony_id, &shares, &reconstruction_attempts)?;
    info!("✅ Audit trail generated");
    info!("   Ceremony hash: {}", audit_trail.ceremony_hash);
    info!("   Total operations: {}", audit_trail.total_operations);
    info!("");

    // Calculate performance metrics
    let test_cases_passed = reconstruction_attempts.iter()
        .filter(|a| {
            let expected_success = scenario.test_cases.iter()
                .find(|tc| tc.test_id == a.test_case_id)
                .map(|tc| tc.expected_result == "success")
                .unwrap_or(false);
            a.reconstruction_successful == expected_success
        })
        .count() as u32;

    let performance = PerformanceMetrics {
        share_generation_ms: share_gen_duration.as_millis() as u64,
        key_reconstruction_ms: total_reconstruction_ms / reconstruction_attempts.len() as u64,
        total_ceremony_ms: (share_gen_duration.as_millis() + total_reconstruction_ms as u128) as u64,
        test_cases_executed: reconstruction_attempts.len() as u32,
        test_cases_passed,
    };

    Ok(CeremonyResult {
        ceremony_id,
        scenario_id: scenario.scenario_id.clone(),
        master_key_id: master_key.id.clone(),
        shares_generated: shares.len() as u8,
        threshold: scenario.approval_policy.threshold,
        reconstruction_attempts,
        audit_trail,
        performance,
    })
}

// ============================================================================
// Master Key Generation
// ============================================================================

async fn generate_master_key(
    scenario: &Scenario,
    config: &DemoConfig,
) -> Result<MasterKey> {
    // Simulate key generation
    tokio::time::sleep(Duration::from_millis(5)).await;
    
    // Generate a random 32-byte secret
    let mut rng = rand::thread_rng();
    let secret: Vec<u8> = (0..32).map(|_| rng.r#gen()).collect();
    
    let key_id = format!("genetic-key-{}-{}", 
                         scenario.operation,
                         uuid::Uuid::new_v4());
    
    Ok(MasterKey {
        id: key_id,
        secret,
        constraints: config.beardog.key_constraints.clone(),
        created_at: chrono::Utc::now(),
    })
}

// ============================================================================
// Shamir's Secret Sharing - Share Generation
// ============================================================================

fn generate_shares(
    master_key: &MasterKey,
    threshold: u8,
    total_shares: u8,
    holders: &[ShareHolder],
) -> Result<Vec<DistributedShare>> {
    // Create Sharks dealer for Shamir's Secret Sharing
    let sharks = Sharks(threshold);
    
    // Generate shares from the secret
    let dealer = sharks.dealer(&master_key.secret);
    let shares_raw: Vec<Share> = dealer.take(total_shares as usize).collect();
    
    // Distribute shares to holders
    let mut distributed_shares = Vec::new();
    for (i, (share_raw, holder)) in shares_raw.into_iter().zip(holders.iter()).enumerate() {
        // Use share index as hash basis (in production, use actual share data hash)
        let integrity_hash = format!("share-hash-{}", i + 1);
        
        let share = DistributedShare {
            share_id: format!("share-{}-{}", holder.id, uuid::Uuid::new_v4()),
            holder_id: holder.id.clone(),
            holder_name: holder.name.clone(),
            share_index: (i + 1) as u8,
            share_object: Some(share_raw),
            created_at: chrono::Utc::now().to_rfc3339(),
            integrity_hash,
        };
        
        distributed_shares.push(share);
    }
    
    Ok(distributed_shares)
}

// ============================================================================
// Threshold Reconstruction Testing
// ============================================================================

async fn test_reconstruction(
    master_key: &MasterKey,
    all_shares: &[DistributedShare],
    test_case: &TestCase,
    threshold: u8,
) -> Result<ReconstructionAttempt> {
    // Simulate reconstruction latency
    tokio::time::sleep(Duration::from_millis(10)).await;
    
    let attempt_id = format!("attempt-{}", uuid::Uuid::new_v4());
    
    // Collect shares for this test
    let selected_shares: Vec<&DistributedShare> = all_shares.iter()
        .filter(|s| test_case.shares_provided.contains(&s.holder_id))
        .collect();
    
    let shares_count = selected_shares.len() as u8;
    let threshold_met = shares_count >= threshold;
    
    // Attempt reconstruction
    let (reconstruction_successful, error) = if threshold_met {
        // Get the Share objects
        let shares_raw: Vec<&Share> = selected_shares.iter()
            .filter_map(|s| s.share_object.as_ref())
            .collect();
        
        if shares_raw.len() as u8 >= threshold {
            let sharks = Sharks(threshold);
            let shares_owned: Vec<_> = shares_raw.iter().map(|&s| s.clone()).collect();
            match sharks.recover(&shares_owned) {
                Ok(recovered_secret) => {
                    // Verify it matches the original
                    let matches = recovered_secret == master_key.secret;
                    (matches, if matches { None } else { Some("SecretMismatch".to_string()) })
                }
                Err(e) => (false, Some(format!("ReconstructionError: {}", e))),
            }
        } else {
            (false, Some("InvalidShares".to_string()))
        }
    } else {
        (false, Some("InsufficientShares".to_string()))
    };
    
    Ok(ReconstructionAttempt {
        attempt_id,
        test_case_id: test_case.test_id.clone(),
        shares_provided: test_case.shares_provided.clone(),
        threshold_met,
        reconstruction_successful,
        duration_ms: 10, // Simulated
        error,
    })
}

// ============================================================================
// Audit Trail Generation
// ============================================================================

fn generate_audit_trail(
    ceremony_id: &str,
    shares: &[DistributedShare],
    attempts: &[ReconstructionAttempt],
) -> Result<AuditTrail> {
    // Log share usage
    let mut share_usage_log = Vec::new();
    for attempt in attempts {
        for share_id in &attempt.shares_provided {
            if let Some(share) = shares.iter().find(|s| &s.holder_id == share_id) {
                share_usage_log.push(ShareUsageLog {
                    share_id: share.share_id.clone(),
                    holder_id: share.holder_id.clone(),
                    operation: "reconstruction_attempt".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
    }
    
    // Generate ceremony hash
    let ceremony_data = format!("{}-{:?}-{:?}", ceremony_id, shares, attempts);
    let ceremony_hash = compute_hash(ceremony_data.as_bytes());
    
    Ok(AuditTrail {
        ceremony_hash,
        total_operations: (shares.len() + attempts.len()) as u32,
        share_usage_log,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

// ============================================================================
// Utility Functions
// ============================================================================

fn compute_hash(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    format!("blake3-{}", hex::encode(&hash.as_bytes()[..16]))
}

// ============================================================================
// Results Printing & Validation
// ============================================================================

fn print_results(result: &CeremonyResult, total_duration: Duration) {
    info!("📊 CEREMONY RESULTS:");
    info!("");
    info!("Ceremony ID: {}", result.ceremony_id);
    info!("Master Key: {}", result.master_key_id);
    info!("Threshold: {}-of-{}", result.threshold, result.shares_generated);
    info!("");

    info!("Reconstruction Attempts:");
    for attempt in &result.reconstruction_attempts {
        let status = if attempt.reconstruction_successful { "✅ SUCCESS" } else { "❌ FAILED" };
        info!("  - {}: {} shares → {}", 
              attempt.test_case_id, attempt.shares_provided.len(), status);
        if let Some(error) = &attempt.error {
            info!("    Error: {}", error);
        }
    }
    info!("");

    info!("Audit Trail:");
    info!("  Ceremony Hash: {}", result.audit_trail.ceremony_hash);
    info!("  Total Operations: {}", result.audit_trail.total_operations);
    info!("  Share Usage Events: {}", result.audit_trail.share_usage_log.len());
    info!("");

    info!("Performance Metrics:");
    info!("  Share Generation: {}ms", result.performance.share_generation_ms);
    info!("  Key Reconstruction (avg): {}ms", result.performance.key_reconstruction_ms);
    info!("  Total Ceremony: {}ms", total_duration.as_millis());
    info!("  Test Cases: {}/{} passed", 
          result.performance.test_cases_passed, 
          result.performance.test_cases_executed);
}

fn validate_results(result: &CeremonyResult, expected: &ExpectedResults) {
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("VALIDATION AGAINST EXPECTED RESULTS:");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Check share generation time
    let share_gen_ok = result.performance.share_generation_ms <= expected.share_generation_ms;
    info!("  Share Generation: {} (expected ≤ {}ms) {}ms", 
          if share_gen_ok { "✅ PASS" } else { "⚠️ SLOWER" },
          expected.share_generation_ms,
          result.performance.share_generation_ms);

    // Check reconstruction time
    let recon_ok = result.performance.key_reconstruction_ms <= expected.key_reconstruction_ms;
    info!("  Key Reconstruction: {} (expected ≤ {}ms) {}ms", 
          if recon_ok { "✅ PASS" } else { "⚠️ SLOWER" },
          expected.key_reconstruction_ms,
          result.performance.key_reconstruction_ms);

    // Check total ceremony time
    let ceremony_ok = result.performance.total_ceremony_ms <= expected.total_ceremony_ms;
    info!("  Total Ceremony: {} (expected ≤ {}ms) {}ms", 
          if ceremony_ok { "✅ PASS" } else { "⚠️ SLOWER" },
          expected.total_ceremony_ms,
          result.performance.total_ceremony_ms);

    // Check test cases
    let tests_ok = result.performance.test_cases_passed >= expected.test_cases_passed;
    info!("  Test Cases Passed: {} (expected ≥ {}) {}/{}", 
          if tests_ok { "✅ PASS" } else { "❌ FAIL" },
          expected.test_cases_passed,
          result.performance.test_cases_passed,
          result.performance.test_cases_executed);

    // Check audit trail
    let audit_ok = !result.audit_trail.ceremony_hash.is_empty() == expected.audit_trail_complete;
    info!("  Audit Trail Complete: {} {}", 
          if audit_ok { "✅ PASS" } else { "❌ FAIL" },
          !result.audit_trail.ceremony_hash.is_empty());

    info!("");
    
    let all_ok = share_gen_ok && recon_ok && ceremony_ok && tests_ok && audit_ok;
    if all_ok {
        info!("🎯 ALL VALIDATIONS PASSED!");
    } else {
        info!("⚠️  Some validations did not pass (see details above)");
    }
}

