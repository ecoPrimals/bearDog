// Canonical Performance Configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalPerformanceConfig {
    /// Whether feature is enabled
    pub enabled: bool,
    /// The optimization level value
    pub optimization_level: OptimizationLevel,
    /// Whether caching is enabled
    pub caching_enabled: bool,
    /// Whether parallel processing is enabled
    /// Whether `parallel_processing` is enabled
    pub parallel_processing: bool,
    /// Resource usage limits
    /// The resource limits value
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum OptimizationLevel {
    Low,
    /// Medium optimization (default, balanced)
    #[default]
    /// Represents medium variant
    Medium,
    High,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// Maximum memory usage in megabytes
    /// Number of `max_memory_mb`
    pub max_memory_mb: u64,
    /// Maximum CPU cores to use
    /// Number of `max_cpu_cores`
    pub max_cpu_cores: u32,
    /// Maximum concurrent connections
    /// Number of `max_connections`
    pub max_connections: u32,
}

pub type PerformanceConfig = CanonicalPerformanceConfig;
