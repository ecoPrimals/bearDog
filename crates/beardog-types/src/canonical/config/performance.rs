//! Canonical Performance Configuration
//!
//! Performance tuning configuration for the BearDog security ecosystem.
//! Controls optimization levels, resource limits, caching, and parallel processing.

use serde::{Deserialize, Serialize};

/// Performance configuration for BearDog operations
///
/// Controls various performance-related settings including optimization levels,
/// caching strategies, parallel processing, and resource usage limits.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::performance::{CanonicalPerformanceConfig, OptimizationLevel};
///
/// let config = CanonicalPerformanceConfig {
///     enabled: true,
///     optimization_level: OptimizationLevel::High,
///     caching_enabled: true,
///     parallel_processing: true,
///     ..Default::default()
/// };
/// ```
///
/// # Performance Considerations
///
/// - **Caching**: Enables in-memory caching of frequently accessed data
/// - **Parallel Processing**: Allows multi-threaded operations where safe
/// - **Resource Limits**: Prevents resource exhaustion in production
///
/// # See Also
///
/// - [`OptimizationLevel`] - Available optimization levels
/// - [`ResourceLimits`] - Resource usage constraints
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalPerformanceConfig {
    /// Whether performance optimizations are enabled
    ///
    /// When disabled, BearDog uses safe defaults with minimal optimization.
    /// Enable in production for better performance.
    pub enabled: bool,

    /// The optimization level to apply
    ///
    /// Controls the trade-off between performance and resource usage.
    /// Higher levels provide better performance but may use more resources.
    pub optimization_level: OptimizationLevel,

    /// Whether in-memory caching is enabled
    ///
    /// Caches frequently accessed configuration and security data
    /// to reduce database queries and improve response times.
    pub caching_enabled: bool,

    /// Whether parallel processing is enabled
    ///
    /// Allows multi-threaded processing of independent operations.
    /// Improves throughput but increases CPU usage.
    pub parallel_processing: bool,

    /// Resource usage limits
    ///
    /// Defines maximum resource consumption to prevent exhaustion
    /// in production environments.
    pub resource_limits: ResourceLimits,
}

/// Optimization level for BearDog operations
///
/// Defines the intensity of performance optimizations applied.
/// Higher levels trade more resources for better performance.
///
/// # Levels
///
/// - **Low**: Minimal optimization, lowest resource usage
/// - **Medium**: Balanced optimization (default), good for most use cases
/// - **High**: Aggressive optimization, higher resource usage
/// - **Maximum**: Maximum optimization, highest resource usage
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::performance::OptimizationLevel;
///
/// let level = OptimizationLevel::High;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum OptimizationLevel {
    /// Low optimization - minimal resource usage
    ///
    /// Use for: Development, debugging, resource-constrained environments
    Low,

    /// Medium optimization - balanced performance and resources (default)
    ///
    /// Use for: Most production deployments, good balance of speed and efficiency
    #[default]
    Medium,

    /// High optimization - aggressive performance tuning
    ///
    /// Use for: High-throughput production, when performance is critical
    High,

    /// Maximum optimization - highest performance, highest resource usage
    ///
    /// Use for: Performance-critical paths, when resources are abundant
    Maximum,
}

/// Resource usage limits for BearDog operations
///
/// Defines maximum resource consumption to prevent resource exhaustion
/// and maintain system stability in production environments.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::performance::ResourceLimits;
///
/// let limits = ResourceLimits {
///     max_memory_mb: 2048,
///     max_cpu_cores: 4,
///     max_connections: 100,
/// };
/// ```
///
/// # Safety
///
/// Setting limits to 0 means unlimited (use with caution in production).
/// Always set reasonable limits based on available system resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// Maximum memory usage in megabytes
    ///
    /// BearDog will attempt to stay within this limit. Set to 0 for unlimited.
    /// Recommended: 1024-4096 MB for production deployments.
    pub max_memory_mb: u64,

    /// Maximum CPU cores to use
    ///
    /// Limits parallel processing to this many cores. Set to 0 for unlimited.
    /// Recommended: Leave 1-2 cores for other system processes.
    pub max_cpu_cores: u32,

    /// Maximum concurrent connections
    ///
    /// Limits the number of simultaneous active connections. Set to 0 for unlimited.
    /// Recommended: 100-1000 based on expected load and available memory.
    pub max_connections: u32,
}

pub type PerformanceConfig = CanonicalPerformanceConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_config_default() {
        let config = CanonicalPerformanceConfig::default();
        assert!(!config.enabled);
        assert!(!config.caching_enabled);
        assert!(!config.parallel_processing);
        assert!(matches!(
            config.optimization_level,
            OptimizationLevel::Medium
        ));
    }

    #[test]
    fn test_optimization_level_default() {
        let level = OptimizationLevel::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(matches!(level, OptimizationLevel::Medium));
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory_mb, 0);
        assert_eq!(limits.max_cpu_cores, 0);
        assert_eq!(limits.max_connections, 0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_performance_config_serialization() {
        let config = CanonicalPerformanceConfig {
            enabled: true,
            optimization_level: OptimizationLevel::High,
            caching_enabled: true,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            parallel_processing: true,
            resource_limits: ResourceLimits {
                max_memory_mb: 2048,
                max_cpu_cores: 4,
                max_connections: 100,
            },
        };

        let json = serde_json::to_string(&config).expect("Serialization failed");
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"max_memory_mb\":2048"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_performance_config_deserialization() {
        let json = r#"{
            "enabled": true,
            "optimization_level": "Maximum",
            "caching_enabled": true,
            "parallel_processing": false,
            "resource_limits": {
                "max_memory_mb": 4096,
                "max_cpu_cores": 8,
                "max_connections": 200
            }
        }"#;

        let config: CanonicalPerformanceConfig =
            serde_json::from_str(json).expect("Deserialization failed");

        assert!(config.enabled);
        assert!(matches!(
            config.optimization_level,
            OptimizationLevel::Maximum
        ));
        assert_eq!(config.resource_limits.max_memory_mb, 4096);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_type_alias() {
        // Ensure type alias works
        let _config: PerformanceConfig = CanonicalPerformanceConfig::default();
    }
}
