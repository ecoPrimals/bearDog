// SPDX-License-Identifier: AGPL-3.0-only

//! Domain capability vectors: network, storage, compute, performance, environment, system, entropy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::defaults;

use super::compliance_discovery::ComplianceCapabilities;
use super::discovery::{SecurityCapabilities, SecurityLevel};

/// Network capabilities and connectivity options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCapabilities {
    /// Supported protocols
    /// Collection of protocols
    pub protocols: Vec<String>,
    /// Maximum bandwidth in Mbps
    pub max_bandwidth: f64,
    /// Encryption support
    /// Whether `encryption_support` is enabled
    pub encryption_support: bool,
    /// VPN support enabled
    /// Whether `vpn_support` is enabled
    pub vpn_support: bool,
}

impl Default for NetworkCapabilities {
    fn default() -> Self {
        Self {
            protocols: vec![
                "HTTP".to_string(),
                "HTTPS".to_string(),
                "WebSocket".to_string(),
            ],
            max_bandwidth: std::env::var("BEARDOG_MAX_BANDWIDTH_MBPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000.0),
            encryption_support: true,
            vpn_support: true,
        }
    }
}

/// Storage capabilities and data management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Storage types supported
    /// Collection of storage types
    pub storage_types: Vec<String>,
    /// Maximum storage capacity in GB
    /// The max capacity value
    pub max_capacity: f64,
    /// Encryption at rest
    /// Whether `encryption_at_rest` is enabled
    pub encryption_at_rest: bool,
    /// Backup capabilities
    /// Whether `backup_support` is enabled
    pub backup_support: bool,
}

impl Default for StorageCapabilities {
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics averaging; precision loss acceptable for display values"
    )]
    fn default() -> Self {
        Self {
            storage_types: vec!["SSD".to_string(), "NVMe".to_string(), "Cloud".to_string()],
            max_capacity: std::env::var("BEARDOG_STORAGE_MAX_CAPACITY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults::DEFAULT_MAX_ENTRIES as f64),
            encryption_at_rest: true,
            backup_support: true,
        }
    }
}

/// Compute capabilities and processing power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {
    /// CPU architectures supported
    /// Collection of cpu architectures
    pub cpu_architectures: Vec<String>,
    /// Number of cores available
    /// Number of core
    pub core_count: u32,
    /// Memory capacity in GB
    /// The memory gb value
    pub memory_gb: f64,
    /// GPU acceleration available
    /// Whether `gpu_acceleration` is enabled
    pub gpu_acceleration: bool,
}

impl Default for ComputeCapabilities {
    fn default() -> Self {
        Self {
            cpu_architectures: vec!["x86_64".to_string(), "ARM64".to_string()],
            core_count: std::env::var("BEARDOG_DEFAULT_CORE_COUNT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8),
            memory_gb: std::env::var("BEARDOG_DEFAULT_MEMORY_GB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32.0),
            gpu_acceleration: false,
        }
    }
}

/// Throughput and latency characteristics used to match workloads to infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    /// Maximum operations per second
    /// The max ops per second value
    pub max_ops_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Throughput optimization enabled
    /// Whether `throughput_optimization` is enabled
    pub throughput_optimization: bool,
    /// Load balancing support
    /// Whether `load_balancing` is enabled
    pub load_balancing: bool,
}

impl Default for PerformanceCapabilities {
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics averaging; precision loss acceptable for display values"
    )]
    fn default() -> Self {
        Self {
            max_ops_per_second: std::env::var("BEARDOG_PERF_MAX_OPS_PER_SECOND")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(defaults::DEFAULT_MAX_ENTRIES as f64),
            avg_response_time_ms: std::env::var("BEARDOG_PERF_AVG_RESPONSE_TIME_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50.0),
            throughput_optimization: true,
            load_balancing: true,
        }
    }
}

/// Environmental capabilities and sustainability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCapabilities {
    /// Power consumption in watts
    /// The power consumption watts value
    pub power_consumption_watts: f64,
    /// Carbon footprint optimization
    /// Whether `carbon_optimization` is enabled
    pub carbon_optimization: bool,
    /// Renewable energy usage
    /// Whether `renewable_energy` is enabled
    pub renewable_energy: bool,
    /// Environmental certifications
    /// Collection of certifications
    pub certifications: Vec<String>,
}

impl Default for EnvironmentalCapabilities {
    fn default() -> Self {
        Self {
            power_consumption_watts: std::env::var("BEARDOG_POWER_CONSUMPTION_WATTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(500.0),
            carbon_optimization: true,
            renewable_energy: true,
            certifications: vec!["Energy Star".to_string(), "Green Computing".to_string()],
        }
    }
}

/// Aggregated capability vectors for a node or deployment (security, network, compute, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemCapabilities {
    /// Security capabilities
    /// The security capabilities value
    pub security_capabilities: SecurityCapabilities,
    /// Compliance capabilities
    /// The compliance capabilities value
    pub compliance_capabilities: ComplianceCapabilities,
    /// Network capabilities
    /// The network capabilities value
    pub network_capabilities: NetworkCapabilities,
    /// Storage capabilities
    /// The storage capabilities value
    pub storage_capabilities: StorageCapabilities,
    /// Compute capabilities
    /// The compute capabilities value
    pub compute_capabilities: ComputeCapabilities,
    /// Advertised throughput, latency, and scaling features for this system profile.
    pub performance_capabilities: PerformanceCapabilities,
    /// Environmental capabilities
    /// The environmental capabilities value
    pub environmental_capabilities: EnvironmentalCapabilities,
}

impl SystemCapabilities {
    /// Builds a [`SystemCapabilities`] populated from [`Default`] (suitable for tests and bootstrapping).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the overall security level of the system
    #[must_use]
    pub const fn security_level(&self) -> &SecurityLevel {
        &self.security_capabilities.security_level
    }

    /// Check if the system meets minimum security requirements
    #[must_use]
    pub fn meets_security_requirements(&self) -> bool {
        self.security_capabilities.rbac
            && self.security_capabilities.audit_logging
            && !self.security_capabilities.secure_protocols.is_empty()
    }

    /// Get total storage capacity
    #[must_use]
    pub const fn total_storage_capacity(&self) -> f64 {
        self.storage_capabilities.max_capacity
    }

    /// Returns `true` when carbon and renewable-energy optimizations are both enabled in the profile.
    #[must_use]
    pub const fn is_environmentally_optimized(&self) -> bool {
        self.environmental_capabilities.carbon_optimization
            && self.environmental_capabilities.renewable_energy
    }
}

/// Hard and soft requirements a consumer states when requesting capability discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Minimum security level required
    /// The min security level value
    pub min_security_level: SecurityLevel,
    /// Required security capabilities
    /// The security requirements value
    pub security_requirements: SecurityCapabilities,
    /// Minimum performance envelope (ops/s, latency) the selected provider must satisfy.
    pub performance_requirements: PerformanceCapabilities,
    /// Required compliance standards
    /// Collection of compliance requirements
    pub compliance_requirements: Vec<String>,
    /// Hardware requirements
    /// Mapping of hardware requirements
    pub hardware_requirements: HashMap<String, String>,
    /// Software requirements
    /// Mapping of software requirements
    pub software_requirements: HashMap<String, String>,
    /// Network requirements
    /// Mapping of network requirements
    pub network_requirements: HashMap<String, String>,
    /// Optional features
    /// Collection of optional features
    pub optional_features: Vec<String>,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: SecurityLevel::Standard,
            security_requirements: SecurityCapabilities::default(),
            performance_requirements: PerformanceCapabilities::default(),
            compliance_requirements: vec!["SOC2".to_string()],
            hardware_requirements: HashMap::new(),
            software_requirements: HashMap::new(),
            network_requirements: HashMap::new(),
            optional_features: Vec::new(),
        }
    }
}

/// Human entropy generation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    /// Supported input methods
    /// Collection of input methods
    pub input_methods: Vec<String>,
    /// Minimum user interaction duration (ms) before entropy samples are considered valid.
    pub min_interaction_time_ms: u32,
    /// Upper bound (ms) on collection window; longer sessions may be truncated or re-sampled.
    pub max_interaction_time_ms: u32,
    /// Entropy quality scoring
    /// Whether `quality_scoring` is enabled
    pub quality_scoring: bool,
    /// Biometric integration
    /// Whether `biometric_integration` is enabled
    pub biometric_integration: bool,
    /// Mouse movement tracking
    /// Whether `mouse_tracking` is enabled
    pub mouse_tracking: bool,
    /// Keyboard timing analysis
    /// Whether `keyboard_timing` is enabled
    pub keyboard_timing: bool,
    /// Touch pattern analysis
    /// Whether `touch_patterns` is enabled
    pub touch_patterns: bool,
    /// Voice pattern analysis
    /// Whether `voice_patterns` is enabled
    pub voice_patterns: bool,
    /// Behavioral analysis
    /// Whether `behavioral_analysis` is enabled
    pub behavioral_analysis: bool,
}

impl Default for HumanEntropyCapabilities {
    fn default() -> Self {
        Self {
            input_methods: vec![
                "mouse".to_string(),
                "keyboard".to_string(),
                "touch".to_string(),
            ],
            min_interaction_time_ms: std::env::var("BEARDOG_MIN_INTERACTION_TIME_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            max_interaction_time_ms: std::env::var("BEARDOG_MAX_INTERACTION_TIME_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30000),
            quality_scoring: true,
            biometric_integration: false,
            mouse_tracking: true,
            keyboard_timing: true,
            touch_patterns: true,
            voice_patterns: false,
            behavioral_analysis: true,
        }
    }
}
