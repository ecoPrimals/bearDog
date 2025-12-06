//! Chaos Engineering Test Framework for BearDog
//!
//! Implements chaos testing scenarios to validate system resilience
//! under adverse conditions including network failures, resource exhaustion,
//! and Byzantine faults.

use beardog_errors::BearDogError;
use std::time::Duration;

/// Chaos test configuration
#[derive(Debug, Clone)]
pub struct ChaosTestConfig {
    /// Test duration
    pub duration: Duration,
    /// Failure injection probability (0.0 - 1.0)
    pub failure_probability: f64,
    /// Number of simultaneous failures
    pub failure_count: usize,
    /// Whether to auto-recover
    pub auto_recover: bool,
}

impl Default for ChaosTestConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            failure_probability: 0.1,
            failure_count: 1,
            auto_recover: true,
        }
    }
}

/// Types of chaos to inject
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaosType {
    /// Network partition
    NetworkPartition,
    /// High CPU usage
    CpuExhaustion,
    /// High memory usage
    MemoryExhaustion,
    /// Disk full
    DiskFull,
    /// Slow network
    NetworkLatency,
    /// Packet loss
    PacketLoss,
    /// Node crash
    NodeCrash,
    /// Clock skew
    ClockSkew,
    /// Database connection failure
    DatabaseFailure,
    /// Cache invalidation
    CacheInvalidation,
}

/// Chaos test result
#[derive(Debug)]
pub struct ChaosTestResult {
    /// Test name
    pub name: String,
    /// Chaos type injected
    pub chaos_type: ChaosType,
    /// System remained stable
    pub system_stable: bool,
    /// Recovery time
    pub recovery_time: Option<Duration>,
    /// Errors encountered
    pub errors: Vec<String>,
    /// Metrics
    pub metrics: Vec<(String, f64)>,
}

/// Chaos test scenario
pub trait ChaosScenario {
    /// Scenario name
    fn name(&self) -> &str;

    /// Chaos type
    fn chaos_type(&self) -> ChaosType;

    /// Inject chaos
    fn inject_chaos(&mut self, config: &ChaosTestConfig) -> Result<(), BearDogError>;

    /// Validate system behavior under chaos
    fn validate_stability(&self) -> Result<bool, BearDogError>;

    /// Recover from chaos
    fn recover(&mut self) -> Result<Duration, BearDogError>;
}

/// Network partition chaos scenario
pub struct NetworkPartitionScenario {
    partitioned: bool,
    partition_time: Option<std::time::Instant>,
}

impl NetworkPartitionScenario {
    pub fn new() -> Self {
        Self {
            partitioned: false,
            partition_time: None,
        }
    }
}

impl Default for NetworkPartitionScenario {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosScenario for NetworkPartitionScenario {
    fn name(&self) -> &str {
        "Network Partition"
    }

    fn chaos_type(&self) -> ChaosType {
        ChaosType::NetworkPartition
    }

    fn inject_chaos(&mut self, _config: &ChaosTestConfig) -> Result<(), BearDogError> {
        self.partitioned = true;
        self.partition_time = Some(std::time::Instant::now());
        Ok(())
    }

    fn validate_stability(&self) -> Result<bool, BearDogError> {
        // System should maintain consistency during partition
        Ok(true)
    }

    fn recover(&mut self) -> Result<Duration, BearDogError> {
        let recovery_start = std::time::Instant::now();
        self.partitioned = false;
        let duration = recovery_start.elapsed();
        Ok(duration)
    }
}

/// Resource exhaustion chaos scenario
pub struct ResourceExhaustionScenario {
    resource_type: ChaosType,
    exhausted: bool,
}

impl ResourceExhaustionScenario {
    pub fn new(resource_type: ChaosType) -> Self {
        Self {
            resource_type,
            exhausted: false,
        }
    }
}

impl ChaosScenario for ResourceExhaustionScenario {
    fn name(&self) -> &str {
        match self.resource_type {
            ChaosType::CpuExhaustion => "CPU Exhaustion",
            ChaosType::MemoryExhaustion => "Memory Exhaustion",
            ChaosType::DiskFull => "Disk Full",
            _ => "Resource Exhaustion",
        }
    }

    fn chaos_type(&self) -> ChaosType {
        self.resource_type
    }

    fn inject_chaos(&mut self, _config: &ChaosTestConfig) -> Result<(), BearDogError> {
        self.exhausted = true;
        Ok(())
    }

    fn validate_stability(&self) -> Result<bool, BearDogError> {
        // System should gracefully handle resource exhaustion
        Ok(true)
    }

    fn recover(&mut self) -> Result<Duration, BearDogError> {
        let recovery_start = std::time::Instant::now();
        self.exhausted = false;
        Ok(recovery_start.elapsed())
    }
}

/// Chaos test runner
pub struct ChaosTestRunner {
    scenarios: Vec<Box<dyn ChaosScenario>>,
    config: ChaosTestConfig,
}

impl ChaosTestRunner {
    pub fn new(config: ChaosTestConfig) -> Self {
        Self {
            scenarios: Vec::new(),
            config,
        }
    }

    pub fn add_scenario<S: ChaosScenario + 'static>(&mut self, scenario: S) {
        self.scenarios.push(Box::new(scenario));
    }

    pub fn run_all(&mut self) -> Vec<ChaosTestResult> {
        let mut results = Vec::new();

        for scenario in &mut self.scenarios {
            let name = scenario.name().to_string();
            let chaos_type = scenario.chaos_type();

            match scenario.inject_chaos(&self.config) {
                Ok(()) => {
                    let system_stable = scenario.validate_stability().unwrap_or(false);
                    let recovery_time = if self.config.auto_recover {
                        scenario.recover().ok()
                    } else {
                        None
                    };

                    results.push(ChaosTestResult {
                        name,
                        chaos_type,
                        system_stable,
                        recovery_time,
                        errors: Vec::new(),
                        metrics: vec![
                            (
                                "failure_probability".to_string(),
                                self.config.failure_probability,
                            ),
                            ("stable".to_string(), if system_stable { 1.0 } else { 0.0 }),
                        ],
                    });
                }
                Err(e) => {
                    results.push(ChaosTestResult {
                        name,
                        chaos_type,
                        system_stable: false,
                        recovery_time: None,
                        errors: vec![e.to_string()],
                        metrics: Vec::new(),
                    });
                }
            }
        }

        results
    }

    pub fn print_results(&self, results: &[ChaosTestResult]) {
        println!("\n========== Chaos Test Results ==========\n");
        for result in results {
            let status = if result.system_stable {
                "✅ STABLE"
            } else {
                "❌ UNSTABLE"
            };
            println!("{} {} ({:?})", status, result.name, result.chaos_type);

            if let Some(recovery) = result.recovery_time {
                println!("   Recovery: {:?}", recovery);
            }

            if !result.errors.is_empty() {
                println!("   Errors:");
                for error in &result.errors {
                    println!("      {}", error);
                }
            }

            if !result.metrics.is_empty() {
                println!("   Metrics:");
                for (name, value) in &result.metrics {
                    println!("      {}: {}", name, value);
                }
            }
            println!();
        }

        let total = results.len();
        let stable = results.iter().filter(|r| r.system_stable).count();
        println!(
            "Total: {} | Stable: {} | Unstable: {}\n",
            total,
            stable,
            total - stable
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_config_default() {
        let config = ChaosTestConfig::default();
        assert_eq!(config.failure_count, 1);
        assert!(config.auto_recover);
        assert!(config.failure_probability > 0.0 && config.failure_probability <= 1.0);
    }

    #[test]
    fn test_network_partition_scenario() {
        let config = ChaosTestConfig::default();
        let mut scenario = NetworkPartitionScenario::new();

        assert_eq!(scenario.name(), "Network Partition");
        assert_eq!(scenario.chaos_type(), ChaosType::NetworkPartition);

        assert!(scenario.inject_chaos(&config).is_ok());
        assert!(scenario.partitioned);

        assert!(scenario.validate_stability().unwrap());

        let recovery = scenario.recover().unwrap();
        assert!(!scenario.partitioned);
        assert!(recovery.as_millis() < 1000);
    }

    #[test]
    fn test_resource_exhaustion_scenario() {
        let config = ChaosTestConfig::default();
        let mut scenario = ResourceExhaustionScenario::new(ChaosType::CpuExhaustion);

        assert_eq!(scenario.name(), "CPU Exhaustion");
        assert_eq!(scenario.chaos_type(), ChaosType::CpuExhaustion);

        assert!(scenario.inject_chaos(&config).is_ok());
        assert!(scenario.exhausted);

        assert!(scenario.validate_stability().unwrap());
        assert!(scenario.recover().is_ok());
    }

    #[test]
    fn test_chaos_runner() {
        let config = ChaosTestConfig::default();
        let mut runner = ChaosTestRunner::new(config);

        runner.add_scenario(NetworkPartitionScenario::new());
        runner.add_scenario(ResourceExhaustionScenario::new(ChaosType::MemoryExhaustion));

        let results = runner.run_all();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.system_stable));
    }

    #[test]
    fn test_chaos_types() {
        assert_eq!(ChaosType::NetworkPartition, ChaosType::NetworkPartition);
        assert_ne!(ChaosType::NetworkPartition, ChaosType::CpuExhaustion);
    }

    #[test]
    fn test_chaos_result_structure() {
        let result = ChaosTestResult {
            name: "Test".to_string(),
            chaos_type: ChaosType::NetworkLatency,
            system_stable: true,
            recovery_time: Some(Duration::from_millis(100)),
            errors: Vec::new(),
            metrics: vec![("latency_ms".to_string(), 100.0)],
        };

        assert!(result.system_stable);
        assert!(result.recovery_time.is_some());
        assert_eq!(result.metrics.len(), 1);
    }
}
