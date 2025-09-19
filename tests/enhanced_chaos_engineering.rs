

use beardog_types::config::*;
use beardog_errors::*;
use beardog_errors::{BearDogError, *};
use beardog_security::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;

mod chaos;
pub use chaos::*;

pub struct EnhancedChaosFramework {
    pub fault_injectors: Vec<Box<dyn FaultInjector + Send + Sync>>,
    pub recovery_validators: Vec<Box<dyn RecoveryValidator + Send + Sync>>,
    pub byzantine_scenarios: Vec<ByzantineScenario>,
    pub performance_baselines: HashMap<String, PerformanceBaseline>,
    pub test_harness: Arc<RwLock<ChaosTestHarness>>,
}

#[derive(Debug, Clone)]
pub struct ByzantineScenario {
    pub description: String,
    pub faulty_nodes: Vec<String>,
    pub fault_behavior: ByzantineFaultBehavior,
    pub duration: Duration,
    pub expected_recovery_time: Duration,
}

#[derive(Debug, Clone)]
pub struct PerformanceBaseline {
    pub avg_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub success_rate: f64,
    pub throughput_ops_per_sec: f64,
}

pub struct ChaosTestHarness {
    pub security_manager: MemoryKeyManager,
    pub active_faults: HashMap<String, ActiveFault>,
    pub recovery_metrics: RecoveryMetrics,
    pub byzantine_nodes: HashMap<String, ByzantineNode>,
}

#[derive(Debug, Clone)]
pub struct ActiveFault {
    pub fault_type: String,
    pub injected_at: Instant,
    pub duration: Duration,
    pub affected_components: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RecoveryMetrics {
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub average_recovery_time_ms: f64,
    pub max_recovery_time_ms: f64,
    pub system_availability: f64,
}

pub struct ByzantineNode {
    pub node_id: String,
    pub behavior: ByzantineFaultBehavior,
    pub is_active: bool,
    pub messages_sent: u64,
    pub messages_corrupted: u64,
}

impl EnhancedChaosFramework {
    pub async fn new() -> Result<Self, BearDogError> {
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
            performance_baselines: Self::establish_baselines(),
            test_harness,
        })
    }

    fn create_byzantine_scenarios() -> Vec<ByzantineScenario> {
        vec![
            ByzantineScenario {
                name: "conflicting_auth_responses".to_string(),
                description:  "Auth nodes send conflicting authorization decisions".to_string(),
                faulty_nodes: vec![ "auth_node_1".to_string(),
                duration: Duration::from_secs(30),
                expected_recovery_time: Duration::from_secs(10),
            },
            ByzantineScenario {
                name:  "delayed_crypto_operations".to_string(),
                description:  "Crypto nodes introduce significant delays".to_string(),
                faulty_nodes: vec![ "crypto_node_1".to_string()],
                fault_behavior: ByzantineFaultBehavior::DelayedResponses(Duration::from_millis(5000)),
                duration: Duration::from_secs(45),
                expected_recovery_time: Duration::from_secs(15),
            },
            ByzantineScenario {
                name:  "corrupted_key_material".to_string(),
                description:  "Key management nodes return corrupted keys".to_string(),
                faulty_nodes: vec![ "key_node_1".to_string(),
                duration: Duration::from_secs(60),
                expected_recovery_time: Duration::from_secs(20),
            },
        ]
    }

    fn establish_baselines() -> HashMap<String, PerformanceBaseline> {
        let mut baselines = HashMap::with_capacity(16);
        
        baselines.insert( "key_generation".to_string(), PerformanceBaseline {
            operation_type:  "key_generation".to_string(),
            expected_duration_ms: 10.0,
            max_response_time_ms: 50.0,
            success_rate: 99.9,
            throughput_ops_per_sec: 1000.0,
        });
        
        baselines.insert( "encryption".to_string(), PerformanceBaseline {
            operation_type:  "encryption".to_string(),
            expected_duration_ms: 5.0,
            max_response_time_ms: 25.0,
            success_rate: 99.95,
            throughput_ops_per_sec: 2000.0,
        });
        
        baselines.insert( "authentication".to_string(), PerformanceBaseline {
            operation_type:  "authentication".to_string(),
            expected_duration_ms: 20.0,
            max_response_time_ms: 100.0,
            success_rate: 99.5,
            throughput_ops_per_sec: 500.0,
        });
        
        baselines
    }

    pub fn execute_scenario(&self, scenario_name: &str) -> Result<ChaosTestResults, BearDogError> {
        let start_time = Instant::now();
        let scenario = self.byzantine_scenarios.iter()
            .find(|s| s.name == scenario_name)
            .ok_or_else(|| BearDogError::internal(format!("Scenario not found: {:?}", scenario.name))?;
        println!("📝 Description: {}", scenario.description);

        let baseline_metrics = self.measure_baseline_performance()?;

        self.inject_byzantine_faults(scenario)?;

        let fault_metrics = self.monitor_system_under_fault(scenario.duration)?;

        let recovery_metrics = self.validate_recovery(scenario)?;

        let performance_impact = self.analyze_performance_impact(&baseline_metrics, &fault_metrics)?;

        let total_duration = start_time.elapsed();
        
        Ok(ChaosTestResults {
            scenario_name: scenario.name.clone(recovery_metrics.recovery_successful,
        })
    }

    fn measure_baseline_performance(&self) -> Result<SystemMetrics, BearDogError> {
        println!("📊 Measuring baseline performance...");
        
        let start_time = Instant::now();
        let mut successful_ops = 0;
        let mut failed_ops = 0;
        let num_operations = 100;

        for i in 0..num_operations {

            let key_result = BearDogCrypto::generate_secure_random(32);
            if key_result.is_ok() {
                successful_ops += 1;
            } else {
                failed_ops += 1;
            }

            if let Ok(key) = &key_result {
                let encrypt_result = BearDogCrypto::encrypt_aes_gcm(key, b "test data", None);
                if encrypt_result.is_ok() {
                    successful_ops += 1;
                } else {
                    failed_ops += 1;
                }
            }

            tokio::time::sleep(Duration::from_millis(num_operations * 2, // key gen + encryption
            successful_operations: successful_ops,
            failed_operations: failed_ops,
            average_response_time_ms: duration.as_millis() as f64 / (num_operations * 2) as f64,
            throughput_ops_per_sec: (num_operations * 2) as f64 / duration.as_secs_f64(),
        })
    }

    async fn inject_byzantine_faults(&self, scenario: &ByzantineScenario) -> Result<(), BearDogError> {
        println!("💉 Injecting Byzantine faults...");
        
        let mut harness = self.test_harness.write();
        
        for node_id in &scenario.faulty_nodes {
            let byzantine_node = ByzantineNode {
                node_id: node_id.clone(),
                behavior: scenario.fault_behavior.clone(true,
                messages_sent: 0,
                messages_corrupted: 0,
            };
            
                    harness.byzantine_nodes.insert(format!("byzantine_{}_{:?}", scenario.name, node_id), ByzantineNode {
            node_id: node_id.clone(),
            fault_type: format!("{:?}", scenario.fault_behavior),
                injected_at: Instant::now(scenario.duration,
                affected_components: vec![node_id.clone()],
            };
            
            harness.active_faults.insert(fault.fault_id.clone(), fault);
        }
        
        harness.recovery_metrics.total_faults_injected += scenario.faulty_nodes.len() as u64;
        
        Ok(())
    }

    fn monitor_system_under_fault(&self, duration: Duration) -> Result<SystemMetrics, BearDogError> {
        println!("🔍 Monitoring system under fault for {:?}...", duration);
        
        let start_time = Instant::now();
        let mut successful_ops = 0;
        let mut failed_ops = 0;
        let mut total_ops = 0;

        let end_time = start_time + duration;
        
        while Instant::now() < end_time {
            total_ops += 1;

            let operation_result = self.simulate_operation_under_fault();
            
            if operation_result.is_ok() {
                successful_ops += 1;
            } else {
                failed_ops += 1;
            }
            
            tokio::time::sleep(Duration::from_millis(actual_duration,
            total_operations: total_ops,
            successful_operations: successful_ops,
            failed_operations: failed_ops,
            average_response_time_ms: actual_duration.as_millis() as f64 / total_ops as f64,
            throughput_ops_per_sec: total_ops as f64 / actual_duration.as_secs_f64(),
        })
    }

    async fn simulate_operation_under_fault(&self) -> Result<(), BearDogError> {
        let harness = self.test_harness.read();

        for (node_id, node) in &harness.byzantine_nodes {
            if !node.is_active {
                continue;
            }
            
            match &node.behavior {
                ByzantineFaultBehavior::ConflictingMessages => {

                    if rand::random::<f64>() < 0.3 {
                        return Err(BearDogError::internal(format!("Conflicting response from {:?}", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::DelayedResponses(delay) => {

                    tokio::time::sleep(*delay).await;
                },
                ByzantineFaultBehavior::CorruptedData => {

                    if rand::random::<f64>() < 0.4 {
                        return Err(BearDogError::internal(format!("Corrupted data from {:?}", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::SilentFailure => {

                    if rand::random::<f64>() < 0.5 {
                        return Err(BearDogError::Timeout {
                            message: format!("No response from {:?}", node_id),
                        });
                    }
                },
                ByzantineFaultBehavior::InvalidSignatures => {

                    if rand::random::<f64>() < 0.2 {
                        return Err(BearDogError::authentication(format!("Invalid signature from {:?}", node_id),
                        });
                    }
                },
            }
        }

        Ok(())
    }

    async fn validate_recovery(&self, scenario: &ByzantineScenario) -> Result<RecoveryMetrics, BearDogError> {
        println!("🔄 Validating system recovery...");

        let mut harness = self.test_harness.write();
        for node_id in &scenario.faulty_nodes {
            if let Some(node) = harness.byzantine_nodes.get_mut(node_id) {
                node.is_active = false;
            }
        }
        drop(harness);
        
        let recovery_start = Instant::now();
        let mut recovery_successful = false;
        let max_recovery_time = scenario.expected_recovery_time * 3; // Allow 3x expected time

        while recovery_start.elapsed() < max_recovery_time {
            let test_result = self.test_system_health();
            
            if test_result.is_ok() {
                recovery_successful = true;
                break;
            }
            
            tokio::time::sleep(Duration::from_millis(actual_recovery_time,
            expected_recovery_time: scenario.expected_recovery_time,
            health_checks_passed: if recovery_successful { 1 } else { 0 },
            health_checks_failed: if recovery_successful { 0 } else { 1 },
        })
    }

    fn test_system_health(&self) -> Result<(), BearDogError> {

        let key = BearDogCrypto::generate_secure_random(32)?;
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, b "health check ", None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
        
        if decrypted != b "health check " {
            return Err(BearDogError::internal( "Health check failed: decryption mismatch "));
        }
        
        Ok(&SystemMetrics,
        under_fault: &SystemMetrics,
    ) -> Result<PerformanceImpact, BearDogError> {
        let throughput_degradation = 
            ((baseline.throughput_ops_per_sec - under_fault.throughput_ops_per_sec) / baseline.throughput_ops_per_sec) * 100.0;
        
        let response_time_increase = 
            ((under_fault.average_response_time_ms - baseline.average_response_time_ms) / baseline.average_response_time_ms) * 100.0;
        
        let error_rate_increase = 
            (under_fault.failed_operations as f64 / under_fault.total_operations as f64) * 100.0 -
            (baseline.failed_operations as f64 / baseline.total_operations as f64) * 100.0;
        
        Ok(throughput_degradation,
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
            active_faults: HashMap::with_capacity(16),
            recovery_metrics: RecoveryMetrics::default(),
            byzantine_nodes: HashMap::with_capacity(Duration,
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub average_response_time_ms: f64,
    pub throughput_ops_per_sec: f64,
}

#[derive(Debug, Clone)]
    pub recovery_time: Duration,
    pub expected_recovery_time: Duration,
    pub health_checks_passed: u32,
    pub health_checks_failed: u32,
}

#[derive(Debug, Clone)]
    pub response_time_increase_percent: f64,
    pub error_rate_increase_percent: f64,
    pub availability_impact_percent: f64,
}

#[derive(Debug, Clone)]
    pub total_duration: Duration,
    pub baseline_metrics: SystemMetrics,
    pub fault_metrics: SystemMetrics,
    pub recovery_metrics: RecoveryMetrics,
    pub performance_impact: PerformanceImpact,
    pub success: bool,
}

pub trait FaultInjector {
    fn inject_fault(&self, fault_type: &str) -> Result<String, BearDogError>;
    fn remove_fault(&self, fault_id: &str) -> Result<(), BearDogError>;
}

pub trait RecoveryValidator {
    fn validate_recovery(&self) -> Result<bool, BearDogError>;
}

pub struct NetworkFaultInjector;
pub struct MemoryFaultInjector;
pub struct CryptoFaultInjector;
pub struct AuthFaultInjector;

impl NetworkFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for NetworkFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> Result<String, BearDogError> {
        Ok(format!("network_fault_{:?}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

impl MemoryFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for MemoryFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> Result<String, BearDogError> {
        Ok(format!("memory_fault_{:?}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

impl CryptoFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for CryptoFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> Result<String, BearDogError> {
        Ok(format!("crypto_fault_{:?}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

impl AuthFaultInjector {
    pub fn new() -> Self { Self }
}

impl FaultInjector for AuthFaultInjector {
    fn inject_fault(&self, fault_type: &str) -> Result<String, BearDogError> {
        Ok(format!("auth_fault_{:?}", fault_type))
    }
    
    fn remove_fault(&self, _fault_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

pub struct SystemHealthValidator;
pub struct DataIntegrityValidator;
pub struct PerformanceValidator;

impl SystemHealthValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for SystemHealthValidator {
    fn validate_recovery(&self) -> Result<bool, BearDogError> {

        Ok(true)
    }
}

impl DataIntegrityValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for DataIntegrityValidator {
    fn validate_recovery(&self) -> Result<bool, BearDogError> {

        Ok(true)
    }
}

impl PerformanceValidator {
    pub fn new() -> Self { Self }
}

impl RecoveryValidator for PerformanceValidator {
    fn validate_recovery(&self) -> Result<bool, BearDogError> {

        Ok(true)
    }
}

#[tokio::test]
async fn test_byzantine_conflicting_messages() -> Result<(), BearDogError> {
    let framework = EnhancedChaosFramework::new()?;
    let results = framework.execute_scenario( "conflicting_auth_responses")?;
    
    println!("🔥 Byzantine Conflicting Messages Test Results:");
    println!("  Scenario: {}", results.scenario_name);
    println!("  Duration: {:?}", results.total_duration);
    println!("  Recovery Successful: {}", results.recovery_metrics.recovery_successful);
    println!("  Throughput Degradation: {:.2}%", results.performance_impact.throughput_degradation_percent);
    println!("  Response Time Increase: {:.2}%", results.performance_impact.response_time_increase_percent);
    
    assert!(results.success,  "Byzantine fault scenario should recover successfully ");
    assert!(results.performance_impact.throughput_degradation_percent < 50.0,  "Throughput degradation should be manageable ");
    
    println!("✅ Byzantine conflicting messages test passed ");
    Ok(())
}

#[tokio::test]
async fn test_byzantine_delayed_responses() -> Result<(), BearDogError> {
    let framework = EnhancedChaosFramework::new()?;
    let results = framework.execute_scenario( "delayed_crypto_operations")?;
    
    println!("🔥 Byzantine Delayed Responses Test Results:");
    println!("  Scenario: {}", results.scenario_name);
    println!("  Recovery Time: {:?}", results.recovery_metrics.recovery_time);
    println!("  Expected Recovery: {:?}", results.recovery_metrics.expected_recovery_time);
    println!("  Error Rate Increase: {:.2}%", results.performance_impact.error_rate_increase_percent);
    
    assert!(results.success,  "Delayed response scenario should recover successfully ");
    assert!(results.recovery_metrics.recovery_time <= results.recovery_metrics.expected_recovery_time * 2, 
             "Recovery should complete within reasonable time ");
    
    println!("✅ Byzantine delayed responses test passed ");
    Ok(())
}

#[tokio::test]
async fn test_byzantine_corrupted_data() -> Result<(), BearDogError> {
    let framework = EnhancedChaosFramework::new()?;
    let results = framework.execute_scenario( "corrupted_key_material")?;
    
    println!("🔥 Byzantine Corrupted Data Test Results:");
    println!("  Baseline Throughput: {:.2} ops/sec ", results.baseline_metrics.throughput_ops_per_sec);
    println!("  Under Fault Throughput: {:.2} ops/sec ", results.fault_metrics.throughput_ops_per_sec);
    println!("  Availability Impact: {:.2}%", results.performance_impact.availability_impact_percent);
    
    assert!(results.success,  "Corrupted data scenario should recover successfully ");
    assert!(results.performance_impact.availability_impact_percent < 80.0,  "System should maintain reasonable availability ");
    
    println!("✅ Byzantine corrupted data test passed ");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_multiple_byzantine_faults() -> Result<(), BearDogError> {
    let framework = EnhancedChaosFramework::new()?;

    let scenarios = vec![
         "conflicting_auth_responses",
         "delayed_crypto_operations",
         "corrupted_key_material",
    ];
    
    let mut handles = Vec::new();
    
    for scenario in scenarios {
        let framework_clone = &framework;
        let handle = tokio::spawn(async move {
            framework_clone.execute_scenario(scenario)
        });
        handles.push(handle);
    }
    
    let mut all_successful = true;
    let mut total_duration = Duration::from_secs(0);
    
    for handle in handles {
        match handle {
            Ok(Ok(results)) => {
                println!("✅ Scenario {} completed successfully ", results.scenario_name);
                total_duration = std::cmp::max({:?}", e);
                all_successful = false;
            },
            Err({:?}", e);
                all_successful = false;
            }
        }
    }
    
    println!("🔥 Concurrent Multiple Byzantine Faults Test Results:");
    println!("  All Scenarios Successful: {}", all_successful);
    println!("  Total Test Duration: {:?}", total_duration);
    
    assert!(all_successful,  "All Byzantine fault scenarios should recover successfully ");
    assert!(total_duration < Duration::from_secs(300),  "Concurrent tests should complete within reasonable time ");
    
    println!("✅ Concurrent multiple Byzantine faults test passed ");
    Ok(())
}

#[tokio::test]
fn test_system_resilience_under_extreme_load() -> Result<(), BearDogError> {
    let framework = EnhancedChaosFramework::new(format!("extreme_load_fault_{:?}", i),
            fault_type:  "extreme_load".to_string(),
            injected_at: Instant::now(),
            duration: Duration::from_secs(vec![format!("component_{:?}", i)],
        };
        harness.active_faults.insert(fault.fault_id.clone(), fault);
    }
    
    drop(harness);

    let start_time = Instant::now();
    let mut operations_completed = 0;
    let mut operations_failed = 0;

    while start_time.elapsed() < Duration::from_secs(30) {
        let operation_result = timeout(
            Duration::from_millis(100),
            framework.simulate_operation_under_fault()
        );
        
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

    assert!(success_rate > 0.2,  "System should maintain minimum functionality under extreme load ");
    assert!(total_operations > 100,  "System should continue processing operations ");
    
    println!( "System resilience under extreme load test passed ");
    Ok(())
} 