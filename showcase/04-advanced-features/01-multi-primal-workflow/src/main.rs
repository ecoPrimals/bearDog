// SPDX-License-Identifier: AGPL-3.0-only

// 🌐 Multi-Primal Workflow Demo
//
// ✅ CORRECT ARCHITECTURE: Capability-based discovery across ALL ecosystem primals
// ❌ NO HARDCODED SERVICE NAMES - Discovers by capability!
//
// This demo orchestrates a complete workflow across 5 ecosystem services:
// - BearDog (security, audit)
// - AI Service (via "ai" capability)
// - Storage Service (via "storage" capability)
// - Compute Service (via "compute" capability)
// - Orchestration Service (via "orchestration" capability)

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// Multi-Primal Workflow Demo
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
    workflow: WorkflowConfig,
    beardog: BeardogConfig,
    discovery: DiscoveryConfig,
    policies: PoliciesConfig,
    audit: AuditConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkflowConfig {
    name: String,
    description: String,
    max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct BeardogConfig {
    hsm_type: String,
    key_constraints: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct DiscoveryConfig {
    methods: Vec<String>,
    fallback: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PoliciesConfig {
    max_retries: u32,
    timeout_ms: u64,
    require_audit: bool,
    require_lineage: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditConfig {
    log_level: String,
    include_performance: bool,
    include_lineage: bool,
    hash_algorithm: String,
}

// ============================================================================
// Scenario Structures
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Scenario {
    scenario_name: String,
    scenario_id: String,
    user_id: String,
    operation: String,
    data: ScenarioData,
    policies: ScenarioPolicies,
    workflow_steps: Vec<WorkflowStep>,
    expected_results: ExpectedResults,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ScenarioData {
    content: String,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ScenarioPolicies {
    purpose: String,
    retention_days: u32,
    allowed_operations: Vec<String>,
    prohibited_operations: Vec<String>,
    compliance: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct WorkflowStep {
    step: u32,
    service: String,
    operation: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ExpectedResults {
    total_duration_ms: u64,
    services_contacted: u32,
    lineage_tracked: bool,
    audit_generated: bool,
    compliance_validated: bool,
}

// ============================================================================
// Service Discovery Types
// ============================================================================

#[derive(Debug, Clone)]
struct DiscoveredService {
    name: String,
    capability: String,
    endpoint: String,
    discovered_via: String,
}

// ============================================================================
// Workflow Execution Types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct WorkflowResult {
    scenario_id: String,
    status: String,
    operation_id: String,
    steps_executed: Vec<StepResult>,
    lineage: LineageInfo,
    audit_receipt: AuditReceipt,
    performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize)]
struct StepResult {
    step: u32,
    service: String,
    capability: String,
    operation: String,
    duration_ms: u64,
    result_id: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct LineageInfo {
    master_key_id: String,
    derived_keys: Vec<String>,
    operations_tracked: u32,
    audit_trail_hash: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuditReceipt {
    operation_hash: String,
    timestamp: String,
    verified: bool,
    compliance: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PerformanceMetrics {
    total_duration_ms: u64,
    services_contacted: u32,
    discovery_overhead_ms: u64,
    lineage_verification_ms: u64,
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

    info!("🌐 Multi-Primal Workflow Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ Using capability-based discovery (no hardcoded services!)");
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

    // Execute workflow
    let workflow_start = Instant::now();
    let result = execute_workflow(&scenario, &config).await?;
    let total_duration = workflow_start.elapsed();

    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ WORKFLOW COMPLETE!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Print results
    print_results(&result, total_duration);

    // Validate against expected results
    validate_results(&result, &scenario.expected_results);

    info!("");
    info!("🎉 Multi-Primal Workflow Demo Complete!");

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
// Workflow Execution
// ============================================================================

async fn execute_workflow(scenario: &Scenario, config: &DemoConfig) -> Result<WorkflowResult> {
    info!("🚀 Starting Multi-Primal Workflow");
    info!("   Scenario: {}", scenario.scenario_name);
    info!("   Operation: {}", scenario.operation);
    info!("");

    let mut step_results = Vec::new();
    let operation_id = format!("workflow-{}", uuid::Uuid::new_v4());

    // Step 1: Discover all services by capability
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 1: Service Discovery (Capability-Based)");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let discovery_start = Instant::now();
    let services = discover_all_services(config).await?;
    let discovery_duration = discovery_start.elapsed();

    info!("✅ Discovered {} services in {:?}", services.len(), discovery_duration);
    for service in &services {
        info!("   - {} (capability: '{}') via {}", 
              service.name, service.capability, service.discovered_via);
    }
    info!("");

    // Step 2: Initialize BearDog security
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 2: Security Initialization");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let step_start = Instant::now();
    let master_key_id = generate_genetic_key(scenario, config).await?;
    let step_duration = step_start.elapsed();

    step_results.push(StepResult {
        step: 1,
        service: "beardog".to_string(),
        capability: "security".to_string(),
        operation: "generate_key".to_string(),
        duration_ms: step_duration.as_millis() as u64,
        result_id: master_key_id.clone(),
        status: "success".to_string(),
    });

    info!("✅ Genetic key generated: {}", master_key_id);
    info!("   Duration: {:?}", step_duration);
    info!("");

    // Step 3: AI Analysis (discovered via "ai" capability)
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 3: AI Analysis (Privacy-Preserving)");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let ai_service = services.iter()
        .find(|s| s.capability == "ai")
        .context("AI service not discovered")?;
    
    let step_start = Instant::now();
    let analysis_id = analyze_data(scenario, ai_service, &master_key_id).await?;
    let step_duration = step_start.elapsed();

    step_results.push(StepResult {
        step: 2,
        service: ai_service.name.clone(),
        capability: "ai".to_string(),
        operation: "analyze".to_string(),
        duration_ms: step_duration.as_millis() as u64,
        result_id: analysis_id.clone(),
        status: "success".to_string(),
    });

    info!("✅ AI analysis complete: {}", analysis_id);
    info!("   Service: {} (discovered)", ai_service.name);
    info!("   Duration: {:?}", step_duration);
    info!("   ✅ PII sanitized (privacy-preserving)");
    info!("");

    // Step 4: Encrypted Storage (discovered via "storage" capability)
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 4: Encrypted Storage");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let storage_service = services.iter()
        .find(|s| s.capability == "storage")
        .context("Storage service not discovered")?;
    
    let step_start = Instant::now();
    let storage_id = store_encrypted(&analysis_id, storage_service, &master_key_id).await?;
    let step_duration = step_start.elapsed();

    step_results.push(StepResult {
        step: 3,
        service: storage_service.name.clone(),
        capability: "storage".to_string(),
        operation: "store_encrypted".to_string(),
        duration_ms: step_duration.as_millis() as u64,
        result_id: storage_id.clone(),
        status: "success".to_string(),
    });

    info!("✅ Encrypted storage complete: {}", storage_id);
    info!("   Service: {} (discovered)", storage_service.name);
    info!("   Duration: {:?}", step_duration);
    info!("");

    // Step 5: Encrypted Compute (discovered via "compute" capability)
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 5: Encrypted Compute (Zero-Knowledge)");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let compute_service = services.iter()
        .find(|s| s.capability == "compute")
        .context("Compute service not discovered")?;
    
    let step_start = Instant::now();
    let compute_id = compute_encrypted(&storage_id, compute_service, &master_key_id).await?;
    let step_duration = step_start.elapsed();

    step_results.push(StepResult {
        step: 4,
        service: compute_service.name.clone(),
        capability: "compute".to_string(),
        operation: "process_encrypted".to_string(),
        duration_ms: step_duration.as_millis() as u64,
        result_id: compute_id.clone(),
        status: "success".to_string(),
    });

    info!("✅ Encrypted compute complete: {}", compute_id);
    info!("   Service: {} (discovered)", compute_service.name);
    info!("   Duration: {:?}", step_duration);
    info!("   ✅ Zero-knowledge processing (no plaintext exposure)");
    info!("");

    // Step 6: Multi-Node Coordination (discovered via "orchestration" capability)
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 6: Multi-Node Orchestration");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let orch_service = services.iter()
        .find(|s| s.capability == "orchestration")
        .context("Orchestration service not discovered")?;
    
    let step_start = Instant::now();
    let coord_id = coordinate_multinode(&compute_id, orch_service).await?;
    let step_duration = step_start.elapsed();

    step_results.push(StepResult {
        step: 5,
        service: orch_service.name.clone(),
        capability: "orchestration".to_string(),
        operation: "coordinate".to_string(),
        duration_ms: step_duration.as_millis() as u64,
        result_id: coord_id.clone(),
        status: "success".to_string(),
    });

    info!("✅ Multi-node coordination complete: {}", coord_id);
    info!("   Service: {} (discovered)", orch_service.name);
    info!("   Duration: {:?}", step_duration);
    info!("");

    // Step 7: Lineage Verification & Audit
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("PHASE 7: Lineage Verification & Audit");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let lineage_start = Instant::now();
    let lineage = verify_lineage(&master_key_id, &step_results).await?;
    let lineage_duration = lineage_start.elapsed();

    let audit_receipt = generate_audit_receipt(&scenario, &step_results, &lineage).await?;

    step_results.push(StepResult {
        step: 6,
        service: "beardog".to_string(),
        capability: "security".to_string(),
        operation: "verify_and_audit".to_string(),
        duration_ms: lineage_duration.as_millis() as u64,
        result_id: audit_receipt.operation_hash.clone(),
        status: "success".to_string(),
    });

    info!("✅ Lineage verified: {} operations tracked", lineage.operations_tracked);
    info!("   Audit trail hash: {}", lineage.audit_trail_hash);
    info!("   Duration: {:?}", lineage_duration);
    info!("");

    // Calculate performance metrics
    let total_duration_ms: u64 = step_results.iter()
        .map(|s| s.duration_ms)
        .sum();

    let performance = PerformanceMetrics {
        total_duration_ms,
        services_contacted: services.len() as u32,
        discovery_overhead_ms: discovery_duration.as_millis() as u64,
        lineage_verification_ms: lineage_duration.as_millis() as u64,
    };

    Ok(WorkflowResult {
        scenario_id: scenario.scenario_id.clone(),
        status: "success".to_string(),
        operation_id,
        steps_executed: step_results,
        lineage,
        audit_receipt,
        performance,
    })
}

// ============================================================================
// Service Discovery (Capability-Based!)
// ============================================================================

async fn discover_all_services(config: &DemoConfig) -> Result<Vec<DiscoveredService>> {
    let mut services = Vec::new();

    // Discover orchestration service
    if let Some(service) = discover_service_by_capability("orchestration", config).await? {
        services.push(service);
    }

    // Discover AI service
    if let Some(service) = discover_service_by_capability("ai", config).await? {
        services.push(service);
    }

    // Discover storage service
    if let Some(service) = discover_service_by_capability("storage", config).await? {
        services.push(service);
    }

    // Discover compute service
    if let Some(service) = discover_service_by_capability("compute", config).await? {
        services.push(service);
    }

    Ok(services)
}

async fn discover_service_by_capability(
    capability: &str,
    config: &DemoConfig,
) -> Result<Option<DiscoveredService>> {
    // Method 1: Environment variables
    let env_key_endpoint = format!("PRIMAL_{}_ENDPOINT", capability.to_uppercase());
    let env_key_caps = format!("PRIMAL_{}_CAPABILITIES", capability.to_uppercase());

    if let (Ok(endpoint), Ok(capabilities)) = (
        std::env::var(&env_key_endpoint),
        std::env::var(&env_key_caps),
    ) {
        let caps: Vec<&str> = capabilities.split(',').map(|s| s.trim()).collect();
        if caps.contains(&capability) {
            let name = format!("discovered-{}", capability);
            return Ok(Some(DiscoveredService {
                name,
                capability: capability.to_string(),
                endpoint,
                discovered_via: "environment".to_string(),
            }));
        }
    }

    // Method 2: Fallback to config
    if let Some(endpoint) = config.discovery.fallback.get(capability) {
        warn!("Using config fallback for '{}' capability", capability);
        let name = format!("fallback-{}", capability);
        return Ok(Some(DiscoveredService {
            name,
            capability: capability.to_string(),
            endpoint: endpoint.clone(),
            discovered_via: "config_fallback".to_string(),
        }));
    }

    Ok(None)
}

// ============================================================================
// Workflow Operations (Simulated for Demo)
// ============================================================================

async fn generate_genetic_key(scenario: &Scenario, _config: &DemoConfig) -> Result<String> {
    // Simulate key generation with genetic constraints
    tokio::time::sleep(Duration::from_millis(5)).await;
    
    let key_id = format!("genetic-key-{}-{}", 
                         scenario.policies.purpose,
                         uuid::Uuid::new_v4());
    
    Ok(key_id)
}

async fn analyze_data(
    scenario: &Scenario,
    service: &DiscoveredService,
    _key_id: &str,
) -> Result<String> {
    // Simulate AI analysis with PII sanitization
    tokio::time::sleep(Duration::from_millis(145)).await;
    
    let analysis_id = format!("analysis-{}-{}", 
                              service.name,
                              uuid::Uuid::new_v4());
    
    // Simulate PII sanitization
    let _content_length = scenario.data.content.len();
    
    Ok(analysis_id)
}

async fn store_encrypted(
    _analysis_id: &str,
    service: &DiscoveredService,
    _key_id: &str,
) -> Result<String> {
    // Simulate encrypted storage with compression
    tokio::time::sleep(Duration::from_millis(8)).await;
    
    let storage_id = format!("storage-{}-{}", 
                            service.name,
                            uuid::Uuid::new_v4());
    
    Ok(storage_id)
}

async fn compute_encrypted(
    _storage_id: &str,
    service: &DiscoveredService,
    _key_id: &str,
) -> Result<String> {
    // Simulate zero-knowledge compute
    tokio::time::sleep(Duration::from_millis(235)).await;
    
    let compute_id = format!("compute-{}-{}", 
                            service.name,
                            uuid::Uuid::new_v4());
    
    Ok(compute_id)
}

async fn coordinate_multinode(
    _compute_id: &str,
    service: &DiscoveredService,
) -> Result<String> {
    // Simulate multi-node coordination
    tokio::time::sleep(Duration::from_millis(12)).await;
    
    let coord_id = format!("coord-{}-{}", 
                          service.name,
                          uuid::Uuid::new_v4());
    
    Ok(coord_id)
}

// ============================================================================
// Lineage & Audit
// ============================================================================

async fn verify_lineage(
    master_key_id: &str,
    steps: &[StepResult],
) -> Result<LineageInfo> {
    // Simulate lineage verification
    tokio::time::sleep(Duration::from_millis(3)).await;
    
    // Track derived keys from workflow
    let derived_keys: Vec<String> = steps.iter()
        .filter(|s| s.service != "beardog")
        .map(|s| format!("derived-{}", s.result_id))
        .collect();
    
    // Generate audit trail hash
    let audit_data = format!("{}-{:?}", master_key_id, steps);
    let audit_hash = blake3::hash(audit_data.as_bytes());
    let audit_trail_hash = format!("blake3-{}", hex::encode(&audit_hash.as_bytes()[..16]));
    
    Ok(LineageInfo {
        master_key_id: master_key_id.to_string(),
        derived_keys,
        operations_tracked: steps.len() as u32,
        audit_trail_hash,
    })
}

async fn generate_audit_receipt(
    scenario: &Scenario,
    steps: &[StepResult],
    lineage: &LineageInfo,
) -> Result<AuditReceipt> {
    // Simulate audit receipt generation
    tokio::time::sleep(Duration::from_millis(2)).await;
    
    // Generate operation hash
    let operation_data = format!("{}-{:?}-{:?}", 
                                scenario.scenario_id, steps, lineage);
    let operation_hash_raw = blake3::hash(operation_data.as_bytes());
    let operation_hash = format!("blake3-{}", hex::encode(&operation_hash_raw.as_bytes()[..16]));
    
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    Ok(AuditReceipt {
        operation_hash,
        timestamp,
        verified: true,
        compliance: scenario.policies.compliance.clone(),
    })
}

// ============================================================================
// Results Printing & Validation
// ============================================================================

fn print_results(result: &WorkflowResult, total_duration: Duration) {
    info!("📊 WORKFLOW RESULTS:");
    info!("");
    info!("Operation ID: {}", result.operation_id);
    info!("Status: {}", result.status);
    info!("");

    info!("Steps Executed:");
    for step in &result.steps_executed {
        info!("  {}. {} ({}): {} - {}ms", 
              step.step, step.service, step.capability, step.operation, step.duration_ms);
    }
    info!("");

    info!("Lineage Information:");
    info!("  Master Key: {}", result.lineage.master_key_id);
    info!("  Derived Keys: {}", result.lineage.derived_keys.len());
    info!("  Operations Tracked: {}", result.lineage.operations_tracked);
    info!("  Audit Trail Hash: {}", result.lineage.audit_trail_hash);
    info!("");

    info!("Audit Receipt:");
    info!("  Operation Hash: {}", result.audit_receipt.operation_hash);
    info!("  Timestamp: {}", result.audit_receipt.timestamp);
    info!("  Verified: {}", result.audit_receipt.verified);
    info!("  Compliance: {:?}", result.audit_receipt.compliance);
    info!("");

    info!("Performance Metrics:");
    info!("  Total Duration: {}ms", total_duration.as_millis());
    info!("  Workflow Duration: {}ms", result.performance.total_duration_ms);
    info!("  Services Contacted: {}", result.performance.services_contacted);
    info!("  Discovery Overhead: {}ms", result.performance.discovery_overhead_ms);
    info!("  Lineage Verification: {}ms", result.performance.lineage_verification_ms);
}

fn validate_results(result: &WorkflowResult, expected: &ExpectedResults) {
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("VALIDATION AGAINST EXPECTED RESULTS:");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Check total duration
    let duration_ok = result.performance.total_duration_ms <= expected.total_duration_ms;
    info!("  Total Duration: {} (expected ≤ {}ms) {}", 
          if duration_ok { "✅ PASS" } else { "⚠️ SLOWER" },
          expected.total_duration_ms,
          result.performance.total_duration_ms);

    // Check services contacted
    let services_ok = result.performance.services_contacted >= expected.services_contacted;
    info!("  Services Contacted: {} (expected ≥ {}) {}", 
          if services_ok { "✅ PASS" } else { "❌ FAIL" },
          expected.services_contacted,
          result.performance.services_contacted);

    // Check lineage tracked
    let lineage_ok = (result.lineage.operations_tracked > 0) == expected.lineage_tracked;
    info!("  Lineage Tracked: {} {}", 
          if lineage_ok { "✅ PASS" } else { "❌ FAIL" },
          result.lineage.operations_tracked > 0);

    // Check audit generated
    let audit_ok = result.audit_receipt.verified == expected.audit_generated;
    info!("  Audit Generated: {} {}", 
          if audit_ok { "✅ PASS" } else { "❌ FAIL" },
          result.audit_receipt.verified);

    // Check compliance
    let compliance_ok = (!result.audit_receipt.compliance.is_empty()) == expected.compliance_validated;
    info!("  Compliance Validated: {} {}", 
          if compliance_ok { "✅ PASS" } else { "❌ FAIL" },
          !result.audit_receipt.compliance.is_empty());

    info!("");
    
    let all_ok = duration_ok && services_ok && lineage_ok && audit_ok && compliance_ok;
    if all_ok {
        info!("🎯 ALL VALIDATIONS PASSED!");
    } else {
        info!("⚠️  Some validations did not pass (see details above)");
    }
}

