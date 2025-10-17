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
        assert!(matches!(level, OptimizationLevel::Medium));
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory_mb, 0);
        assert_eq!(limits.max_cpu_cores, 0);
        assert_eq!(limits.max_connections, 0);
    }

    #[test]
    fn test_performance_config_serialization() {
        let config = CanonicalPerformanceConfig {
            enabled: true,
            optimization_level: OptimizationLevel::High,
            caching_enabled: true,
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

    #[test]
    fn test_type_alias() {
        // Ensure type alias works
        let _config: PerformanceConfig = CanonicalPerformanceConfig::default();
    }
}
