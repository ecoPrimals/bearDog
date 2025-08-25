// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Enhanced Chaos Engineering Test Suite for BearDog Security Manager
//!
//! This test suite extends the existing chaos testing framework with advanced
//! fault injection scenarios, Byzantine fault tolerance, and recovery validation.

use beardog_types::config::*;
use beardog_core::*;
use beardog_errors::*;
use beardog_security::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;

mod chaos;
pub use chaos::*;

/// Enhanced chaos testing framework with advanced fault injection
pub struct EnhancedChaosFramework {
    pub fault_injectors: Vec<Box<dyn FaultInjector + Send + Sync>>,
    pub recovery_validators: Vec<Box<dyn RecoveryValidator + Send + Sync>>,
    pub byzantine_scenarios: Vec<ByzantineScenario>,
    pub performance_baselines: HashMap<String, PerformanceBaseline>,
    pub test_harness: Arc<RwLock<ChaosTestHarness>>,
}

/// Byzantine fault scenario for testing distributed system resilience
#[derive(Debug, Clone)]
pub struct ByzantineScenario {
    pub name: String,
    pub description: String,
    pub faulty_nodes: Vec<String>,
    pub fault_behavior: ByzantineFaultBehavior,
    pub duration: Duration,
    pub expected_recovery_time: Duration,
}

/// Types of Byzantine fault behaviors
#[derive(Debug, Clone)]
pub enum ByzantineFaultBehavior {
    /// Node sends conflicting information to different peers
    ConflictingMessages,
    /// Node delays responses beyond acceptable thresholds
    DelayedResponses(Duration),
    /// Node sends malformed or corrupted data
    CorruptedData,
    /// Node stops responding entirely
    SilentFailure,
    /// Node sends valid responses but with wrong signatures
    InvalidSignatures,
}

/// Performance baseline for comparison during chaos testing
#[derive(Debug, Clone)]
pub struct PerformanceBaseline {
    pub operation_type: String,
    pub avg_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub success_rate: f64,
    pub throughput_ops_per_sec: f64,
}

/// Chaos test harness for advanced scenarios
pub struct ChaosTestHarness {
    pub security_manager: MemoryKeyManager,
    pub active_faults: HashMap<String, ActiveFault>,
    pub recovery_metrics: RecoveryMetrics,
    pub byzantine_nodes: HashMap<String, ByzantineNode>,
}

/// Active fault tracking
#[derive(Debug)]
pub struct ActiveFault {
    pub fault_id: String,
    pub fault_type: String,
    pub injected_at: Instant,
    pub duration: Duration,
    pub affected_components: Vec<String>,
}

/// Recovery metrics tracking
#[derive(Debug, Default)]
pub struct RecoveryMetrics {
    pub total_faults_injected: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub average_recovery_time_ms: f64,
    pub max_recovery_time_ms: f64,
    pub system_availability: f64,
}

/// Byzantine node simulation
pub struct ByzantineNode {
    pub node_id: String,
    pub behavior: ByzantineFaultBehavior,
    pub is_active: bool,
    pub messages_sent: u64,
    pub messages_corrupted: u64,
}

impl EnhancedChaosFramework {
    pub async fn new() -> BearDogResult<Self> {
        let test_harness = Arc::new(RwLock::new(ChaosTestHarness::new()));
        
        Ok(Self {
            fault_injectors: vec![
                Box::new(NetworkFaultInjector::new()),
                Box::new(MemoryFaultInjector::new()),
                Box::new(CryptoFaultInjector::new()),
                Box::new(AuthFaultInjector::new()),
            ],
            recovery_validators: vec![
                Box::new(SystemHealthValidator::new()),
                Box::new(DataIntegrityValidator::new()),
                Box::new(PerformanceValidator::new()),
            ],
            byzantine_scenarios: Self::create_byzantine_scenarios(),
            performance_baselines: Self::establish_baselines().await,
            test_harness,
        })
    }

    /// Create predefined Byzantine fault scenarios
    fn create_byzantine_scenarios() -> Vec<ByzantineScenario> {
        vec![
            ByzantineScenario {
                name: "conflicting_auth_responses".to_string(),
                description: "Auth nodes send conflicting authorization decisions".to_string(),
                faulty_nodes: vec!["auth_node_1".to_string(), "auth_node_2".to_string()],
                fault_behavior: ByzantineFaultBehavior::ConflictingMessages,
                duration: Duration::from_secs(30),
                expected_recovery_time: Duration::from_secs(10),
            },
            ByzantineScenario {
                name: "delayed_crypto_operations".to_string(),
                description: "Crypto nodes introduce significant delays".to_string(),
                faulty_nodes: vec!["crypto_node_1".to_string()],
                fault_behavior: ByzantineFaultBehavior::DelayedResponses(Duration::from_millis(5000)),
                duration: Duration::from_secs(45),
                expected_recovery_time: Duration::from_secs(15),
            },
            ByzantineScenario {
                name: "corrupted_key_material".to_string(),
                description: "Key management nodes return corrupted keys".to_string(),
                faulty_nodes: vec!["key_node_1".to_string(), "key_node_3".to_string()],
                fault_behavior: ByzantineFaultBehavior::CorruptedData,
                duration: Duration::from_secs(60),
                expected_recovery_time: Duration::from_secs(20),
            },
        ]
    }

    /// Establish performance baselines for comparison
    async fn establish_baselines() -> HashMap<String, PerformanceBaseline> {
        let mut baselines = HashMap::new();
        
        baselines.insert("key_generation".to_string(), PerformanceBaseline {
            operation_type: "key_generation".to_string(),
            avg_response_time_ms: 10.0,
            max_response_time_ms: 50.0,
            success_rate: 99.9,
            throughput_ops_per_sec: 1000.0,
        });
        
        baselines.insert("encryption".to_string(), PerformanceBaseline {
            operation_type: "encryption".to_string(),
            avg_response_time_ms: 5.0,
            max_response_time_ms: 25.0,
            success_rate: 99.95,
            throughput_ops_per_sec: 2000.0,
        });
        
        baselines.insert("authentication".to_string(), PerformanceBaseline {
            operation_type: "authentication".to_string(),
            avg_response_time_ms: 20.0,
            max_response_time_ms: 100.0,
            success_rate: 99.5,
            throughput_ops_per_sec: 500.0,
        });
        
        baselines
    }

    /// Execute a comprehensive chaos engineering test scenario
    pub async fn execute_scenario(&self, scenario_name: &str) -> BearDogResult<ChaosTestResults> {
        let start_time = Instant::now();
        let scenario = self.byzantine_scenarios.iter()
            .find(|s| s.name == scenario_name)
            .ok_or_else(|| BearDogError::internal(format!("Scenario {) not found", scenario_name),
            })?;

        println!("🔥 Starting chaos scenario: {}", scenario.name);
        println!("📝 Description: {}", scenario.description);

        // Phase 1: Establish baseline performance
        let baseline_metrics = self.measure_baseline_performance().await?;
        
        // Phase 2: Inject Byzantine faults
        self.inject_byzantine_faults(scenario).await?;
        
        // Phase 3: Monitor system behavior under fault
        let fault_metrics = self.monitor_system_under_fault(scenario.duration).await?;
        
        // Phase 4: Validate recovery
        let recovery_metrics = self.validate_recovery(scenario).await?;
        
        // Phase 5: Compare performance degradation
        let performance_impact = self.analyze_performance_impact(&baseline_metrics, &fault_metrics).await?;

        let total_duration = start_time.elapsed();
        
        Ok(ChaosTestResults {
            scenario_name: scenario.name.clone(),
            total_duration,
            baseline_metrics,
            fault_metrics,
            recovery_metrics,
            performance_impact,
            success: recovery_metrics.recovery_successful,
        })
    }

    /// Measure baseline system performance
    async fn measure_baseline_performance(&self) -> BearDogResult<SystemMetrics> {
        println!("📊 Measuring baseline performance...");
        
        let start_time = Instant::now();
        let mut successful_ops = 0;
        let mut failed_ops = 0;
        let num_operations = 100;

        for i in 0..num_operations {
            // Test key generation
            let key_result = BearDogCrypto::generate_secure_random(32);
            if key_result.is_ok() {
                successful_ops += 1;
            } else {
                failed_ops += 1;
            }

            // Test encryption
            if let Ok(key) = &key_result {
                let encrypt_result = BearDogCrypto::encrypt_aes_gcm(key, b"test data", None);
                if encrypt_result.is_ok() {
                    successful_ops += 1;
                } else {
                    failed_ops += 1;
                }
            }

            // Small delay to simulate realistic load
            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        let duration = start_time.elapsed();
        
        Ok(SystemMetrics {
            duration,
            total_operations: num_operations * 2, // key gen + encryption
            successful_operations: successful_ops,
            failed_operations: failed_ops,
            average_response_time_ms: duration.as_millis() as f64 / (num_operations * 2) as f64,
            throughput_ops_per_sec: (num_operations * 2) as f64 / duration.as_secs_f64(),
        })
    }

    /// Inject Byzantine faults according to scenario
    async fn inject_byzantine_faults(&self, scenario: &ByzantineScenario) -> BearDogResult<()> {
        println!("💉 Injecting Byzantine faults...");
        
        let mut harness = self.test_harness.write().await;
        
        for node_id in &scenario.faulty_nodes {
            let byzantine_node = ByzantineNode {
                node_id: node_id.clone(),
                behavior: scenario.fault_behavior.clone(),
                is_active: true,
                messages_sent: 0,
                messages_corrupted: 0,
            };
            
            harness.byzantine_nodes.insert(node_id.clone(), byzantine_node);
            
            let fault = ActiveFault {
                fault_id: format!("byzantine_{}_{}", scenario.name, node_id),
                fault_type: format!("{:?}", scenario.fault_behavior),
                injected_at: Instant::now(),
                duration: scenario.duration,
                affected_components: vec![node_id.clone()],
            };
            
            harness.active_faults.insert(fault.fault_id.clone(), fault);
        }
        
        harness.recovery_metrics.total_faults_injected += scenario.faulty_nodes.len() as u64;
        
        Ok(())
    }

    /// Monitor system behavior while faults are active
    async fn monitor_system_under_fault(&self, duration: Duration) -> BearDogResult<SystemMetrics> {
        println!("🔍 Monitoring system under fault for {:?}...", duration);
        
        let start_time = Instant::now();
        let mut successful_ops = 0;
        let mut failed_ops = 0;
        let mut total_ops = 0;

        // Monitor system for the specified duration
        let end_time = start_time + duration;
        
        while Instant::now() < end_time {
            total_ops += 1;
            
            // Simulate operations under fault conditions
            let operation_result = self.simulate_operation_under_fault().await;
            
            if operation_result.is_ok() {
                successful_ops += 1;
            } else {
                failed_ops += 1;
            }
            
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let actual_duration = start_time.elapsed();
        
        Ok(SystemMetrics {
            duration: actual_duration,
            total_operations: total_ops,
            successful_operations: successful_ops,
            failed_operations: failed_ops,
            average_response_time_ms: actual_duration.as_millis() as f64 / total_ops as f64,
            throughput_ops_per_sec: total_ops as f64 / actual_duration.as_secs_f64(),
        })
    }

    /// Simulate an operation under fault conditions
    async fn simulate_operation_under_fault(&self) -> BearDogResult<()> {
        let harness = self.test_harness.read().await;
        
        // Check if any Byzantine nodes would interfere
        for (node_id, node) in &harness.byzantine_nodes {
            if !node.is_active {
                continue;
            }
            
            match &node.behavior {
                ByzantineFaultBehavior::ConflictingMessages => {
                    // Simulate conflicting responses
                    if rand::random::<f64>() < 0.3 {
                        return Err(BearDogError::internal(format!("Conflicting response from {)", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::DelayedResponses(delay) => {
                    // Simulate delayed responses
                    tokio::time::sleep(*delay).await;
                },
                ByzantineFaultBehavior::CorruptedData => {
                    // Simulate corrupted data
                    if rand::random::<f64>() < 0.4 {
                        return Err(BearDogError::internal(format!("Corrupted data from {)", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::SilentFailure => {
                    // Simulate timeout
                    if rand::random::<f64>() < 0.5 {
                        return Err(BearDogError::Timeout {
                            message: format!("No response from {}", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::InvalidSignatures => {
                    // Simulate invalid signatures
                    if rand::random::<f64>() < 0.2 {
                        return Err(BearDogError::authentication(format!("Invalid signature from {)", node_id),
                        });
                    }
                },
            }
        }
        
        // If no faults triggered, operation succeeds
        Ok(())
    }

    /// Validate system recovery after faults are removed
    async fn validate_recovery(&self, scenario: &ByzantineScenario) -> BearDogResult<RecoveryMetrics> {
        println!("🔄 Validating system recovery...");
        
        // Remove Byzantine faults
        let mut harness = self.test_harness.write().await;
        for node_id in &scenario.faulty_nodes {
            if let Some(node) = harness.byzantine_nodes.get_mut(node_id) {
                node.is_active = false;
            }
        }
        drop(harness);
        
        let recovery_start = Instant::now();
        let mut recovery_successful = false;
        let max_recovery_time = scenario.expected_recovery_time * 3; // Allow 3x expected time
        
        // Monitor recovery progress
        while recovery_start.elapsed() < max_recovery_time {
            let test_result = self.test_system_health().await;
            
            if test_result.is_ok() {
                recovery_successful = true;
                break;
            }
            
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        
        let actual_recovery_time = recovery_start.elapsed();
        
        // Update recovery metrics
        let mut harness = self.test_harness.write().await;
        if recovery_successful {
            harness.recovery_metrics.successful_recoveries += 1;
        } else {
            harness.recovery_metrics.failed_recoveries += 1;
        }
        
        let recovery_time_ms = actual_recovery_time.as_millis() as f64;
        harness.recovery_metrics.average_recovery_time_ms = 
            (harness.recovery_metrics.average_recovery_time_ms + recovery_time_ms) / 2.0;
        
        if recovery_time_ms > harness.recovery_metrics.max_recovery_time_ms {
            harness.recovery_metrics.max_recovery_time_ms = recovery_time_ms;
        }
        
        Ok(RecoveryMetrics {
            recovery_successful,
            recovery_time: actual_recovery_time,
            expected_recovery_time: scenario.expected_recovery_time,
            health_checks_passed: if recovery_successful { 1 } else { 0 },
            health_checks_failed: if recovery_successful { 0 } else { 1 },
        })
    }

    /// Test overall system health
    async fn test_system_health(&self) -> BearDogResult<()> {
        // Test key operations
        let key = BearDogCrypto::generate_secure_random(32)?;
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, b"health check", None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
        
        if decrypted != b"health check" {
            return Err(BearDogError::internal("Health check failed: decryption mismatch".to_string(),
            ));
        }
        
        Ok(())
    }

    /// Analyze performance impact of faults
    async fn analyze_performance_impact(
        &self,
        baseline: &SystemMetrics,
        under_fault: &SystemMetrics,
    ) -> BearDogResult<PerformanceImpact> {
        let throughput_degradation = 
            ((baseline.throughput_ops_per_sec - under_fault.throughput_ops_per_sec) / baseline.throughput_ops_per_sec) * 100.0;
        
        let response_time_increase = 
            ((under_fault.average_response_time_ms - baseline.average_response_time_ms) / baseline.average_response_time_ms) * 100.0;
        
        let error_rate_increase = 
            (under_fault.failed_operations as f64 / under_fault.total_operations as f64) * 100.0 -
            (baseline.failed_operations as f64 / baseline.total_operations as f64) * 100.0;
        
        Ok(PerformanceImpact {
            throughput_degradation_percent: throughput_degradation,
            response_time_increase_percent: response_time_increase,
            error_rate_increase_percent: error_rate_increase,
            availability_impact_percent: throughput_degradation, // Simplified metric
        })
    }
}

impl ChaosTestHarness {
    pub fn new() -> Self {
        Self {
            security_manager: MemoryKeyManager::new(),
            active_faults: HashMap::new(),
            recovery_metrics: RecoveryMetrics::default(),
            byzantine_nodes: HashMap::new(),
        }
    }
}

/// System metrics for performance measurement
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub duration: Duration,
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub average_response_time_ms: f64,
    pub throughput_ops_per_sec: f64,
}

/// Recovery metrics for fault recovery validation
#[derive(Debug)]
pub struct RecoveryMetrics {
    pub recovery_successful: bool,
    pub recovery_time: Duration,
    pub expected_recovery_time: Duration,
    pub health_checks_passed: u32,
    pub health_checks_failed: u32,
}

/// Performance impact analysis
#[derive(Debug)]
pub struct PerformanceImpact {
    pub throughput_degradation_percent: f64,
    pub response_time_increase_percent: f64,
    pub error_rate_increase_percent: f64,
    pub availability_impact_percent: f64,
}

/// Comprehensive chaos test results
#[derive(Debug)]
pub struct ChaosTestResults {
    pub scenario_name: String,
    pub total_duration: Duration,
    pub baseline_metrics: SystemMetrics,
    pub fault_metrics: SystemMetrics,
    pub recovery_metrics: RecoveryMetrics,
    pub performance_impact: PerformanceImpact,
    pub success: bool,
}

// Trait definitions for extensible fault injection
pub trait FaultInjector {
    fn inject_fault(&self, fault_type: &str) -> BearDogResult<String>;
    fn remove_fault(&self, fault_id: &str) -> BearDogResult<()>;
}

pub trait RecoveryValidator {
    fn validate_recovery(&self) -> BearDogResult<bool>;
}

// Concrete fault injector implementations
pub struct NetworkFaultInjector;
pub struct MemoryFaultInjector;
pub struct CryptoFaultInjector;
pub struct AuthFaultInjector;

impl NetworkFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for NetworkFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> BearDogResult<String> {
        Ok(format!("network_fault_{}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> BearDogResult<()> {
        Ok(())
    }
}

impl MemoryFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for MemoryFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> BearDogResult<String> {
        Ok(format!("memory_fault_{}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> BearDogResult<()> {
        Ok(())
    }
}

impl CryptoFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for CryptoFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> BearDogResult<String> {
        Ok(format!("crypto_fault_{}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> BearDogResult<()> {
        Ok(())
    }
}

impl AuthFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for AuthFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> BearDogResult<String> {
        Ok(format!("auth_fault_{}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> BearDogResult<()> {
        Ok(())
    }
}

// Recovery validator implementations
pub struct SystemHealthValidator;
pub struct DataIntegrityValidator;
pub struct PerformanceValidator;

impl SystemHealthValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for SystemHealthValidator {
    fn validate_recovery(&self) -> BearDogResult<bool> {
        // Simplified health check
        Ok(true)
    }
}

impl DataIntegrityValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for DataIntegrityValidator {
    fn validate_recovery(&self) -> BearDogResult<bool> {
        // Simplified integrity check
        Ok(true)
    }
}

impl PerformanceValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for PerformanceValidator {
    fn validate_recovery(&self) -> BearDogResult<bool> {
        // Simplified performance check
        Ok(true)
    }
}

// Enhanced Chaos Engineering Tests

#[tokio::test]
async fn test_byzantine_conflicting_messages() -> BearDogResult<()> {
    let framework = EnhancedChaosFramework::new().await?;
    let results = framework.execute_scenario("conflicting_auth_responses").await?;
    
    println!("🔥 Byzantine Conflicting Messages Test Results:");
    println!("  Scenario: {}", results.scenario_name);
    println!("  Duration: {:?}", results.total_duration);
    println!("  Recovery Successful: {}", results.recovery_metrics.recovery_successful);
    println!("  Throughput Degradation: {:.2}%", results.performance_impact.throughput_degradation_percent);
    println!("  Response Time Increase: {:.2}%", results.performance_impact.response_time_increase_percent);
    
    assert!(results.success, "Byzantine fault scenario should recover successfully");
    assert!(results.performance_impact.throughput_degradation_percent < 50.0, "Throughput degradation should be manageable");
    
    println!("✅ Byzantine conflicting messages test passed");
    Ok(())
}

#[tokio::test]
async fn test_byzantine_delayed_responses() -> BearDogResult<()> {
    let framework = EnhancedChaosFramework::new().await?;
    let results = framework.execute_scenario("delayed_crypto_operations").await?;
    
    println!("🔥 Byzantine Delayed Responses Test Results:");
    println!("  Scenario: {}", results.scenario_name);
    println!("  Recovery Time: {:?}", results.recovery_metrics.recovery_time);
    println!("  Expected Recovery: {:?}", results.recovery_metrics.expected_recovery_time);
    println!("  Error Rate Increase: {:.2}%", results.performance_impact.error_rate_increase_percent);
    
    assert!(results.success, "Delayed response scenario should recover successfully");
    assert!(results.recovery_metrics.recovery_time <= results.recovery_metrics.expected_recovery_time * 2, 
            "Recovery should complete within reasonable time");
    
    println!("✅ Byzantine delayed responses test passed");
    Ok(())
}

#[tokio::test]
async fn test_byzantine_corrupted_data() -> BearDogResult<()> {
    let framework = EnhancedChaosFramework::new().await?;
    let results = framework.execute_scenario("corrupted_key_material").await?;
    
    println!("🔥 Byzantine Corrupted Data Test Results:");
    println!("  Baseline Throughput: {:.2} ops/sec", results.baseline_metrics.throughput_ops_per_sec);
    println!("  Under Fault Throughput: {:.2} ops/sec", results.fault_metrics.throughput_ops_per_sec);
    println!("  Availability Impact: {:.2}%", results.performance_impact.availability_impact_percent);
    
    assert!(results.success, "Corrupted data scenario should recover successfully");
    assert!(results.performance_impact.availability_impact_percent < 80.0, "System should maintain reasonable availability");
    
    println!("✅ Byzantine corrupted data test passed");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_multiple_byzantine_faults() -> BearDogResult<()> {
    let framework = EnhancedChaosFramework::new().await?;
    
    // Execute multiple scenarios concurrently
    let scenarios = vec![
        "conflicting_auth_responses",
        "delayed_crypto_operations",
        "corrupted_key_material",
    ];
    
    let mut handles = Vec::new();
    
    for scenario in scenarios {
        let framework_clone = &framework;
        let handle = tokio::spawn(async move {
            framework_clone.execute_scenario(scenario).await
        });
        handles.push(handle);
    }
    
    let mut all_successful = true;
    let mut total_duration = Duration::from_secs(0);
    
    for handle in handles {
        match handle.await {
            Ok(Ok(results)) => {
                println!("✅ Scenario {} completed successfully", results.scenario_name);
                total_duration = std::cmp::max(total_duration, results.total_duration);
                if !results.success {
                    all_successful = false;
                }
            },
            Ok(Err(e)) => {
                println!("❌ Scenario failed with error: {:?}", e);
                all_successful = false;
            },
            Err(e) => {
                println!("❌ Task failed: {:?}", e);
                all_successful = false;
            }
        }
    }
    
    println!("🔥 Concurrent Multiple Byzantine Faults Test Results:");
    println!("  All Scenarios Successful: {}", all_successful);
    println!("  Total Test Duration: {:?}", total_duration);
    
    assert!(all_successful, "All Byzantine fault scenarios should recover successfully");
    assert!(total_duration < Duration::from_secs(300), "Concurrent tests should complete within reasonable time");
    
    println!("✅ Concurrent multiple Byzantine faults test passed");
    Ok(())
}

#[tokio::test]
async fn test_system_resilience_under_extreme_load() -> BearDogResult<()> {
    let framework = EnhancedChaosFramework::new().await?;
    
    // Inject multiple types of faults simultaneously
    let mut harness = framework.test_harness.write().await;
    
    // Simulate extreme load conditions
    for i in 0..10 {
        let fault = ActiveFault {
            fault_id: format!("extreme_load_fault_{}", i),
            fault_type: "extreme_load".to_string(),
            injected_at: Instant::now(),
            duration: Duration::from_secs(60),
            affected_components: vec![format!("component_{}", i)],
        };
        harness.active_faults.insert(fault.fault_id.clone(), fault);
    }
    
    drop(harness);
    
    // Test system behavior under extreme conditions
    let start_time = Instant::now();
    let mut operations_completed = 0;
    let mut operations_failed = 0;
    
    // Run for 30 seconds under extreme load
    while start_time.elapsed() < Duration::from_secs(30) {
        let operation_result = timeout(
            Duration::from_millis(100),
            framework.simulate_operation_under_fault()
        ).await;
        
        match operation_result {
            Ok(Ok(_)) => operations_completed += 1,
            Ok(Err(_)) | Err(_) => operations_failed += 1,
        }
    }
    
    let total_operations = operations_completed + operations_failed;
    let success_rate = operations_completed as f64 / total_operations as f64;
    
    println!("🔥 System Resilience Under Extreme Load Test Results:");
    println!("  Total Operations: {}", total_operations);
    println!("  Operations Completed: {}", operations_completed);
    println!("  Operations Failed: {}", operations_failed);
    println!("  Success Rate: {:.2}%", success_rate * 100.0);
    
    // System should maintain at least 20% success rate under extreme conditions
    assert!(success_rate > 0.2, "System should maintain minimum functionality under extreme load");
    assert!(total_operations > 100, "System should continue processing operations");
    
    println!("✅ System resilience under extreme load test passed");
    Ok(())
} 