//! Test Scenarios and Load Test Types
//!
//! This module defines the various test scenarios and load test types
//! that can be configured for BearDog performance testing.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::operations::*;
use super::profiles::*;

/// Test scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Test type
    pub test_type: LoadTestType,
    /// Load profile
    pub load_profile: LoadProfile,
    /// Target configuration
    pub target: TestTarget,
    /// Success criteria
    pub success_criteria: SuccessCriteria,
    /// Test duration
    pub duration: Duration,
}

/// Load test type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadTestType {
    /// Database performance test
    Database {
        /// Database operations to test
        operations: Vec<DatabaseOperation>,
        /// Connection patterns
        connection_patterns: Vec<ConnectionPattern>,
    },
    /// Memory optimization test
    Memory {
        /// Memory operations to test
        operations: Vec<MemoryOperation>,
        /// Memory patterns
        memory_patterns: Vec<MemoryPattern>,
    },
    /// Caching performance test
    Caching {
        /// Cache operations to test
        operations: Vec<CacheOperation>,
        /// Cache patterns
        cache_patterns: Vec<CachePattern>,
    },
    /// SIMD crypto acceleration test
    SIMDCrypto {
        /// Cryptographic operations to test
        operations: Vec<CryptoOperation>,
        /// Crypto patterns
        crypto_patterns: Vec<CryptoPattern>,
    },
    /// Genetic algorithm performance test
    GeneticAlgorithm {
        /// Genetic operations to test
        operations: Vec<GeneticOperation>,
        /// Genetic patterns
        genetic_patterns: Vec<GeneticPattern>,
    },
    /// End-to-end system test
    EndToEnd {
        /// System operations to test
        operations: Vec<SystemOperation>,
        /// Workflow patterns
        workflow_patterns: Vec<WorkflowPattern>,
    },
    /// Stress test
    Stress {
        /// Stress operations
        operations: Vec<StressOperation>,
        /// Stress patterns
        stress_patterns: Vec<StressPattern>,
    },
    /// Scalability test
    Scalability {
        /// Scalability operations
        operations: Vec<ScalabilityOperation>,
        /// Scaling patterns
        scaling_patterns: Vec<ScalingPattern>,
    },
}

/// Test target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTarget {
    /// Target host
    pub host: String,
    /// Target port
    pub port: u16,
    /// Target protocol
    pub protocol: String,
    /// Target path
    pub path: String,
    /// Target authentication
    pub authentication: Option<TestAuthentication>,
}

/// Test authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAuthentication {
    /// Authentication type
    pub auth_type: AuthenticationType,
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Additional parameters
    pub parameters: std::collections::HashMap<String, String>,
}

/// Authentication type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationType {
    /// Basic authentication
    Basic,
    /// Bearer token
    Bearer,
    /// OAuth2
    OAuth2,
    /// Custom authentication
    Custom { method: String },
}

/// Success criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriteria {
    /// Maximum response time
    pub max_response_time: Duration,
    /// Minimum throughput
    pub min_throughput: f64,
    /// Maximum error rate
    pub max_error_rate: f64,
    /// Resource utilization limits
    pub resource_limits: ResourceLimits,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU usage
    pub max_cpu_usage: f64,
    /// Maximum memory usage
    pub max_memory_usage: f64,
    /// Maximum disk usage
    pub max_disk_usage: f64,
    /// Maximum network usage
    pub max_network_usage: f64,
}

impl Default for TestScenario {
    fn default() -> Self {
        Self {
            name: "Default Test Scenario".to_string(),
            description: "Default test scenario for load testing".to_string(),
            test_type: LoadTestType::Database {
                operations: Vec::new(),
                connection_patterns: Vec::new(),
            },
            load_profile: LoadProfile::default(),
            target: TestTarget::default(),
            success_criteria: SuccessCriteria::default(),
            duration: Duration::from_secs(60),
        }
    }
}

impl Default for TestTarget {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            path: "/".to_string(),
            authentication: None,
        }
    }
}

impl Default for SuccessCriteria {
    fn default() -> Self {
        Self {
            max_response_time: Duration::from_millis(1000),
            min_throughput: 100.0,
            max_error_rate: 0.01,
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_usage: 0.8,
            max_memory_usage: 0.8,
            max_disk_usage: 0.8,
            max_network_usage: 0.8,
        }
    }
}
