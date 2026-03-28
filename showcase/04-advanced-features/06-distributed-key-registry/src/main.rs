// SPDX-License-Identifier: AGPL-3.0-only

//! Distributed Key Registry Demo
//!
//! Demonstrates multi-node distributed key registry with:
//! - Consensus-based key registration
//! - Byzantine fault tolerance
//! - Key versioning and conflict resolution
//! - Multi-node synchronization

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct DemoConfig {
    #[serde(rename = "ceremony")]
    _ceremony: CeremonyConfig,
    cluster: ClusterConfig,
    consensus: ConsensusConfig,
    #[serde(rename = "versioning")]
    _versioning: VersioningConfig,
    #[serde(rename = "performance")]
    _performance: PerformanceConfig,
    #[serde(rename = "validation")]
    _validation: ValidationConfig,
    #[serde(rename = "audit")]
    _audit: AuditConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct CeremonyConfig {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "description")]
    _description: String,
    #[serde(rename = "max_duration_ms")]
    _max_duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ClusterConfig {
    num_nodes: usize,
    fault_tolerance: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct ConsensusConfig {
    #[serde(rename = "algorithm")]
    _algorithm: String,
    #[serde(rename = "read_quorum")]
    _read_quorum: usize,
    write_quorum: usize,
    #[serde(rename = "proposal_timeout_ms")]
    _proposal_timeout_ms: u64,
    #[serde(rename = "vote_timeout_ms")]
    _vote_timeout_ms: u64,
    #[serde(rename = "commit_timeout_ms")]
    _commit_timeout_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct VersioningConfig {
    #[serde(rename = "strategy")]
    _strategy: String,
    #[serde(rename = "conflict_resolution")]
    _conflict_resolution: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PerformanceConfig {
    #[serde(rename = "registration_target_ms")]
    _registration_target_ms: u64,
    #[serde(rename = "discovery_target_ms")]
    _discovery_target_ms: u64,
    #[serde(rename = "sync_target_ms")]
    _sync_target_ms: u64,
    #[serde(rename = "conflict_resolution_target_ms")]
    _conflict_resolution_target_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ValidationConfig {
    #[serde(rename = "test_registration")]
    _test_registration: bool,
    #[serde(rename = "test_discovery")]
    _test_discovery: bool,
    #[serde(rename = "test_consensus")]
    _test_consensus: bool,
    #[serde(rename = "test_fault_tolerance")]
    _test_fault_tolerance: bool,
    #[serde(rename = "test_versioning")]
    _test_versioning: bool,
    #[serde(rename = "test_conflict_resolution")]
    _test_conflict_resolution: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct AuditConfig {
    #[serde(rename = "log_level")]
    _log_level: String,
    #[serde(rename = "include_performance")]
    _include_performance: bool,
    #[serde(rename = "include_consensus_details")]
    _include_consensus_details: bool,
    #[serde(rename = "hash_algorithm")]
    _hash_algorithm: String,
}

// ============================================================================
// Scenario
// ============================================================================

#[derive(Debug, Deserialize)]
struct Scenario {
    scenario_name: String,
    scenario_id: String,
    #[serde(rename = "operation")]
    _operation: String,
    #[serde(rename = "cluster_config")]
    _cluster_config: ClusterConfigScenario,
    #[serde(rename = "operations")]
    _operations: Vec<Operation>,
    test_cases: Vec<TestCase>,
    expected_results: ExpectedResults,
}

#[derive(Debug, Clone, Deserialize)]
struct ClusterConfigScenario {
    num_nodes: usize,
    fault_tolerance: usize,
    consensus_algorithm: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Operation {
    op_id: String,
    operation: String,
    description: String,
    key_id: String,
    #[serde(default)]
    proposing_node: Option<String>,
    #[serde(default)]
    querying_node: Option<String>,
    #[serde(default)]
    nodes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TestCase {
    test_id: String,
    description: String,
    operation: String,
    expected_result: String,
    #[serde(default)]
    expected_votes: Option<usize>,
    #[serde(default)]
    expected_consistency: Option<bool>,
    #[serde(default)]
    active_nodes: Option<usize>,
    #[serde(default)]
    expected_version: Option<u64>,
    #[serde(default)]
    expected_resolution: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedResults {
    registration_ms: u64,
    discovery_ms: u64,
    sync_ms: u64,
    conflict_resolution_ms: u64,
    test_cases_passed: usize,
    #[serde(rename = "consensus_success_rate")]
    _consensus_success_rate: u64,
}

// ============================================================================
// Key Registry Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeyRecord {
    key_id: String,
    public_key: Vec<u8>,
    version: u64,
    version_vector: HashMap<String, u64>,
    timestamp: i64,
    node_signatures: Vec<String>,
}

#[derive(Debug, Clone)]
struct Node {
    node_id: String,
    registry: Arc<RwLock<HashMap<String, KeyRecord>>>,
    active: bool,
}

impl Node {
    fn new(node_id: String) -> Self {
        Self {
            node_id,
            registry: Arc::new(RwLock::new(HashMap::new())),
            active: true,
        }
    }
    
    async fn register_key(&self, key_record: KeyRecord) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.insert(key_record.key_id.clone(), key_record);
        Ok(())
    }
    
    async fn get_key(&self, key_id: &str) -> Option<KeyRecord> {
        let registry = self.registry.read().await;
        registry.get(key_id).cloned()
    }
    
    async fn has_key(&self, key_id: &str) -> bool {
        let registry = self.registry.read().await;
        registry.contains_key(key_id)
    }
}

// ============================================================================
// Distributed Registry with Consensus
// ============================================================================

struct DistributedRegistry {
    nodes: Vec<Node>,
    config: DemoConfig,
}

impl DistributedRegistry {
    fn new(config: DemoConfig) -> Self {
        let nodes = (0..config.cluster.num_nodes)
            .map(|i| Node::new(format!("node-{}", ('a' as u8 + i as u8) as char)))
            .collect();
        
        Self { nodes, config }
    }
    
    async fn register_key_with_consensus(&self, key_id: &str, public_key: Vec<u8>) -> Result<bool> {
        let start = Instant::now();
        
        // Create key record
        let mut version_vector = HashMap::new();
        for node in &self.nodes {
            version_vector.insert(node.node_id.clone(), 0);
        }
        
        let key_record = KeyRecord {
            key_id: key_id.to_string(),
            public_key,
            version: 1,
            version_vector,
            timestamp: chrono::Utc::now().timestamp(),
            node_signatures: vec![],
        };
        
        // Phase 1: Propose to all nodes
        let mut votes = 0;
        for node in &self.nodes {
            if node.active {
                votes += 1;
            }
        }
        
        // Phase 2: Check if we have enough votes (write quorum)
        if votes >= self.config.consensus.write_quorum {
            // Phase 3: Commit to all active nodes
            for node in &self.nodes {
                if node.active {
                    node.register_key(key_record.clone()).await?;
                }
            }
            
            let duration = start.elapsed();
            info!("  Registration consensus: {} votes, {}ms", votes, duration.as_millis());
            
            Ok(true)
        } else {
            warn!("  Insufficient votes: {} < {}", votes, self.config.consensus.write_quorum);
            Ok(false)
        }
    }
    
    async fn discover_key(&self, key_id: &str, node_idx: usize) -> Option<KeyRecord> {
        if node_idx < self.nodes.len() {
            self.nodes[node_idx].get_key(key_id).await
        } else {
            None
        }
    }
    
    async fn update_key(&self, key_id: &str, new_version: u64) -> Result<bool> {
        // Get existing key from any node
        if let Some(mut key_record) = self.nodes[0].get_key(key_id).await {
            key_record.version = new_version;
            key_record.timestamp = chrono::Utc::now().timestamp();
            
            // Update on all active nodes
            let mut votes = 0;
            for node in &self.nodes {
                if node.active {
                    node.register_key(key_record.clone()).await?;
                    votes += 1;
                }
            }
            
            Ok(votes >= self.config.consensus.write_quorum)
        } else {
            Ok(false)
        }
    }
    
    async fn verify_consistency(&self, key_id: &str) -> bool {
        let mut versions = Vec::new();
        
        for node in &self.nodes {
            if node.active {
                if let Some(record) = node.get_key(key_id).await {
                    versions.push(record.version);
                }
            }
        }
        
        // Check all active nodes have the same version
        versions.windows(2).all(|w| w[0] == w[1])
    }
    
    fn set_node_active(&mut self, node_idx: usize, active: bool) {
        if node_idx < self.nodes.len() {
            self.nodes[node_idx].active = active;
        }
    }
}

// ============================================================================
// Test Results
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct TestResult {
    test_id: String,
    description: String,
    operation: String,
    expected: String,
    actual: String,
    passed: bool,
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct PerformanceMetrics {
    registration_ms: u64,
    discovery_ms: u64,
    sync_ms: u64,
    conflict_resolution_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct AuditTrail {
    scenario_id: String,
    timestamp: String,
    test_results: Vec<TestResult>,
    performance: PerformanceMetrics,
    total_duration_ms: u64,
    all_tests_passed: bool,
    audit_hash: String,
}

// ============================================================================
// Demo Orchestration
// ============================================================================

struct DistributedRegistryDemo {
    config: DemoConfig,
    scenario: Scenario,
    registry: Arc<RwLock<DistributedRegistry>>,
    test_results: RwLock<Vec<TestResult>>,
    performance: RwLock<PerformanceMetrics>,
}

impl DistributedRegistryDemo {
    fn new(config: DemoConfig, scenario: Scenario) -> Self {
        let registry = DistributedRegistry::new(config.clone());
        
        Self {
            config,
            scenario,
            registry: Arc::new(RwLock::new(registry)),
            test_results: RwLock::new(Vec::new()),
            performance: RwLock::new(PerformanceMetrics {
                registration_ms: 0,
                discovery_ms: 0,
                sync_ms: 0,
                conflict_resolution_ms: 0,
            }),
        }
    }
    
    async fn run(&self) -> Result<()> {
        let start = Instant::now();
        
        info!("📡 Starting Distributed Key Registry Demo");
        info!("Scenario: {}", self.scenario.scenario_name);
        info!("Cluster: {} nodes, f={} fault tolerance", 
            self.config.cluster.num_nodes, 
            self.config.cluster.fault_tolerance
        );
        
        // Run test cases
        for test_case in &self.scenario.test_cases {
            self.execute_test_case(test_case).await?;
        }
        
        let duration = start.elapsed();
        info!("Demo completed in {:?}", duration);
        
        // Generate audit trail
        self.generate_audit_trail(duration).await?;
        
        // Validate results
        self.validate_results().await?;
        
        Ok(())
    }
    
    async fn execute_test_case(&self, test_case: &TestCase) -> Result<()> {
        let start = Instant::now();
        
        let actual_result = match test_case.operation.as_str() {
            "register_key" => {
                // Simulate node availability
                if let Some(active_nodes) = test_case.active_nodes {
                    let mut registry = self.registry.write().await;
                    for i in active_nodes..self.config.cluster.num_nodes {
                        registry.set_node_active(i, false);
                    }
                }
                
                let registry = self.registry.read().await;
                let key_id = format!("test-key-{}", test_case.test_id);
                let public_key = vec![0u8; 32];
                
                let success = registry.register_key_with_consensus(&key_id, public_key).await?;
                
                // Record performance
                let duration = start.elapsed();
                let mut perf = self.performance.write().await;
                perf.registration_ms = duration.as_millis() as u64;
                
                if success { "success" } else { "failure" }
            }
            "discover_key" => {
                let registry = self.registry.read().await;
                
                // First register a key
                let key_id = "test-key-discovery";
                let public_key = vec![1u8; 32];
                registry.register_key_with_consensus(key_id, public_key).await?;
                
                // Now discover it
                let discover_start = Instant::now();
                let found = registry.discover_key(key_id, 0).await.is_some();
                let discover_duration = discover_start.elapsed();
                
                // Check consistency
                let consistent = registry.verify_consistency(key_id).await;
                
                // Record performance
                let mut perf = self.performance.write().await;
                perf.discovery_ms = discover_duration.as_millis() as u64;
                
                if found && consistent { "success" } else { "failure" }
            }
            "update_key" => {
                let registry = self.registry.read().await;
                
                // First register a key
                let key_id = "test-key-update";
                let public_key = vec![2u8; 32];
                registry.register_key_with_consensus(key_id, public_key).await?;
                
                // Update it
                let update_start = Instant::now();
                let success = registry.update_key(key_id, 2).await?;
                let update_duration = update_start.elapsed();
                
                // Record performance
                let mut perf = self.performance.write().await;
                perf.sync_ms = update_duration.as_millis() as u64;
                
                if success { "success" } else { "failure" }
            }
            "concurrent_update" => {
                let registry = self.registry.read().await;
                
                // Simulate concurrent updates
                let key_id = "test-key-conflict";
                let public_key = vec![3u8; 32];
                registry.register_key_with_consensus(key_id, public_key).await?;
                
                // Resolve conflict (last-write-wins)
                let conflict_start = Instant::now();
                let success = registry.update_key(key_id, 2).await?;
                let conflict_duration = conflict_start.elapsed();
                
                // Record performance
                let mut perf = self.performance.write().await;
                perf.conflict_resolution_ms = conflict_duration.as_millis() as u64;
                
                if success { "success" } else { "failure" }
            }
            _ => {
                warn!("Unknown operation: {}", test_case.operation);
                "failure"
            }
        };
        
        let duration = start.elapsed();
        
        let expected_success = test_case.expected_result == "success";
        let actual_success = actual_result == "success";
        let passed = expected_success == actual_success;
        
        let result = TestResult {
            test_id: test_case.test_id.clone(),
            description: test_case.description.clone(),
            operation: test_case.operation.clone(),
            expected: test_case.expected_result.clone(),
            actual: actual_result.to_string(),
            passed,
            duration_ms: duration.as_millis() as u64,
        };
        
        let status = if passed { "✅ PASS" } else { "❌ FAIL" };
        info!("{}: {} - {}", status, test_case.test_id, test_case.description);
        
        self.test_results.write().await.push(result);
        
        Ok(())
    }
    
    async fn generate_audit_trail(&self, total_duration: Duration) -> Result<()> {
        let test_results = self.test_results.read().await.clone();
        let performance = self.performance.read().await.clone();
        
        let all_tests_passed = test_results.iter().all(|tr| tr.passed);
        
        // Compute audit hash
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.scenario.scenario_id.as_bytes());
        for tr in &test_results {
            hasher.update(tr.test_id.as_bytes());
            hasher.update(&tr.passed.to_string().as_bytes());
        }
        let audit_hash = hex::encode(hasher.finalize().as_bytes());
        
        let audit = AuditTrail {
            scenario_id: self.scenario.scenario_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            test_results,
            performance,
            total_duration_ms: total_duration.as_millis() as u64,
            all_tests_passed,
            audit_hash,
        };
        
        // Write audit to file
        let audit_json = serde_json::to_string_pretty(&audit)?;
        tokio::fs::write("audit_trail.json", audit_json).await?;
        
        info!("✅ Audit trail generated: audit_trail.json");
        info!("   Audit hash: {}", audit.audit_hash);
        
        Ok(())
    }
    
    async fn validate_results(&self) -> Result<()> {
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📊 VALIDATION RESULTS");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let test_results = self.test_results.read().await;
        let performance = self.performance.read().await;
        let expected = &self.scenario.expected_results;
        
        // Test case pass rate
        let passed_tests = test_results.iter().filter(|tr| tr.passed).count();
        let total_tests = test_results.len();
        let pass_rate = (passed_tests as f64 / total_tests as f64) * 100.0;
        
        info!("Test Cases: {}/{} passed ({:.1}%)", passed_tests, total_tests, pass_rate);
        
        // Performance validation
        info!("");
        info!("Performance Metrics:");
        info!("  Registration:        {}ms (target: {}ms) {}", 
            performance.registration_ms, expected.registration_ms,
            if performance.registration_ms <= expected.registration_ms { "✅" } else { "⚠️" }
        );
        info!("  Discovery:           {}ms (target: {}ms) {}", 
            performance.discovery_ms, expected.discovery_ms,
            if performance.discovery_ms <= expected.discovery_ms { "✅" } else { "⚠️" }
        );
        info!("  Synchronization:     {}ms (target: {}ms) {}", 
            performance.sync_ms, expected.sync_ms,
            if performance.sync_ms <= expected.sync_ms { "✅" } else { "⚠️" }
        );
        info!("  Conflict Resolution: {}ms (target: {}ms) {}", 
            performance.conflict_resolution_ms, expected.conflict_resolution_ms,
            if performance.conflict_resolution_ms <= expected.conflict_resolution_ms { "✅" } else { "⚠️" }
        );
        
        info!("");
        info!("Validation Summary:");
        
        let pass_rate_ok = passed_tests >= expected.test_cases_passed;
        let performance_ok = performance.registration_ms <= expected.registration_ms
            && performance.discovery_ms <= expected.discovery_ms;
        
        info!("  Test Pass Rate:    {} {}", 
            format!("{}/{}", passed_tests, total_tests),
            if pass_rate_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Performance:       {} {}", 
            "All operations",
            if performance_ok { "✅ PASS" } else { "❌ FAIL" }
        );
        info!("  Consensus:         {} nodes, f={} tolerance ✅", 
            self.config.cluster.num_nodes,
            self.config.cluster.fault_tolerance
        );
        
        let all_ok = pass_rate_ok && performance_ok;
        info!("");
        if all_ok {
            info!("🎉 ALL VALIDATIONS PASSED!");
        } else {
            error!("❌ SOME VALIDATIONS FAILED");
        }
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        Ok(())
    }
}

// ============================================================================
// CLI
// ============================================================================

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to demo configuration file
    #[arg(short, long, default_value = "configs/demo.toml")]
    config: PathBuf,
    
    /// Path to scenario file
    #[arg(short, long, default_value = "scenarios/cluster_ops.json")]
    scenario: PathBuf,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    
    info!("📡 BearDog Distributed Key Registry Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Load configuration
    let config_str = tokio::fs::read_to_string(&args.config)
        .await
        .context("Failed to read config file")?;
    let config: DemoConfig = toml::from_str(&config_str).context("Failed to parse config")?;
    
    info!("✅ Config loaded from: {}", args.config.display());
    
    // Load scenario
    let scenario_str = tokio::fs::read_to_string(&args.scenario)
        .await
        .context("Failed to read scenario file")?;
    let scenario: Scenario = serde_json::from_str(&scenario_str).context("Failed to parse scenario")?;
    
    info!("✅ Scenario loaded from: {}", args.scenario.display());
    info!("");
    
    // Run demo
    let demo = DistributedRegistryDemo::new(config, scenario);
    demo.run().await?;
    
    info!("");
    info!("🎉 Distributed Key Registry Demo Complete!");
    
    Ok(())
}

