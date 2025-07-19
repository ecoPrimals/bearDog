//! Load Testing Configuration
//!
//! This module provides comprehensive load testing configuration for BearDog performance testing.
//! It includes test scenarios, execution environments, monitoring, and validation configurations.

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod execution;
pub mod monitoring;
pub mod operations;
pub mod profiles;
pub mod scenarios;
pub mod validation;

pub use execution::{
    EnvironmentType, ExecutionEnvironment, ResourceManagementConfig, ResultCollectionConfig,
    TestExecutionConfig, TestSchedulingConfig,
};
pub use monitoring::{
    AlertingConfig, LoadTestMonitoringConfig, MetricsCollection, MetricsStorage, ReportingConfig,
};
pub use operations::{
    CacheOperation, CryptoOperation, DatabaseOperation, GeneticOperation, MemoryOperation,
    ScalabilityOperation, StressOperation, SystemOperation,
};
pub use profiles::{
    CachePattern, ConnectionPattern, CryptoPattern, GeneticPattern, LoadParameters, LoadProfile,
    LoadProfileType, MemoryPattern, ScalingPattern, StressPattern, WorkflowPattern,
};
pub use scenarios::{
    AuthenticationType, LoadTestType, SuccessCriteria, TestAuthentication, TestScenario, TestTarget,
};
pub use validation::{
    DataValidationConfig, ErrorRateThresholds, PerformanceThresholds, PerformanceValidationConfig,
    ResourceUtilizationThresholds, ResponseTimeThresholds, SamplingMethod, ThroughputThresholds,
    ValidationRule, ValidationRuleType, ValidationSampling,
};

/// Load testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingConfig {
    /// Enable load testing
    pub enabled: bool,
    /// Test scenarios
    pub scenarios: Vec<TestScenario>,
    /// Load generation configuration
    pub load_generation: LoadGenerationConfig,
    /// Test execution configuration
    pub execution: TestExecutionConfig,
    /// Performance validation configuration
    pub validation: PerformanceValidationConfig,
    /// Monitoring integration
    pub monitoring: LoadTestMonitoringConfig,
}

/// Load generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadGenerationConfig {
    /// Number of virtual users
    pub virtual_users: usize,
    /// Ramp-up duration
    pub ramp_up_duration: Duration,
    /// Think time between requests
    pub think_time: Duration,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health checking enabled
    pub health_checking: bool,
    /// Failover configuration
    pub failover: FailoverConfig,
}

/// Load balancing algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round robin
    WeightedRoundRobin,
    /// Random
    Random,
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Failover enabled
    pub enabled: bool,
    /// Failover threshold
    pub threshold: f64,
    /// Failover delay
    pub delay: Duration,
    /// Recovery configuration
    pub recovery: RecoveryConfig,
}

/// Recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryConfig {
    /// Recovery strategy
    pub strategy: RecoveryStrategy,
    /// Recovery timeout
    pub timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
}

/// Recovery strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Immediate recovery
    Immediate,
    /// Gradual recovery
    Gradual,
    /// Manual recovery
    Manual,
}

/// Resource estimate for load testing
pub struct ResourceEstimate {
    /// Memory in MB
    pub memory_mb: u64,
    /// CPU cores
    pub cpu_cores: u32,
    /// Disk in GB
    pub disk_gb: u64,
    /// Network bandwidth in Mbps
    pub network_bandwidth_mbps: f64,
    /// Estimated cost per hour
    pub estimated_cost_per_hour: f64,
}

impl Default for LoadTestingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            scenarios: Vec::new(),
            load_generation: LoadGenerationConfig::default(),
            execution: TestExecutionConfig::default(),
            validation: PerformanceValidationConfig::default(),
            monitoring: LoadTestMonitoringConfig::default(),
        }
    }
}

impl Default for LoadGenerationConfig {
    fn default() -> Self {
        Self {
            virtual_users: 10,
            ramp_up_duration: Duration::from_secs(60),
            think_time: Duration::from_secs(1),
            load_balancing: LoadBalancingConfig::default(),
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_checking: true,
            failover: FailoverConfig::default(),
        }
    }
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 0.8,
            delay: Duration::from_secs(5),
            recovery: RecoveryConfig::default(),
        }
    }
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            strategy: RecoveryStrategy::Gradual,
            timeout: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(5),
        }
    }
}

impl LoadTestingConfig {
    /// Create a production configuration for load testing
    pub fn production() -> Self {
        Self {
            enabled: true,
            scenarios: vec![TestScenario::default()],
            load_generation: LoadGenerationConfig {
                virtual_users: 100,
                ramp_up_duration: Duration::from_secs(300),
                think_time: Duration::from_millis(500),
                load_balancing: LoadBalancingConfig::default(),
            },
            execution: TestExecutionConfig::default(),
            validation: PerformanceValidationConfig::default(),
            monitoring: LoadTestMonitoringConfig::default(),
        }
    }

    /// Create a development configuration for load testing
    pub fn development() -> Self {
        Self {
            enabled: false,
            scenarios: vec![TestScenario::default()],
            load_generation: LoadGenerationConfig {
                virtual_users: 10,
                ramp_up_duration: Duration::from_secs(60),
                think_time: Duration::from_secs(1),
                load_balancing: LoadBalancingConfig::default(),
            },
            execution: TestExecutionConfig::default(),
            validation: PerformanceValidationConfig::default(),
            monitoring: LoadTestMonitoringConfig::default(),
        }
    }
}
