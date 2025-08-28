use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConfigurationMigrationRegistry {
    pub migration_map: HashMap<String, String>,

    pub deprecated_configs: Vec<String>,

    pub canonical_configs: Vec<String>,
}

impl Default for ConfigurationMigrationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurationMigrationRegistry {
    pub fn new() -> Self {
        let mut migration_map = HashMap::with_capacity(16);
        let mut deprecated_configs = Vec::new();

        migration_map.insert(
            "NetworkConfig".to_string(),
            "UnifiedNetworkConfig".to_string(),
        );
        migration_map.insert(
            "NetworkPortsConfig".to_string(),
            "UnifiedNetworkConfig".to_string(),
        );
        migration_map.insert(
            "NetworkScanConfig".to_string(),
            "UnifiedNetworkConfig".to_string(),
        );
        migration_map.insert(
            "NetworkResourceConfig".to_string(),
            "UnifiedNetworkConfig".to_string(),
        );

        migration_map.insert(
            "SecurityConfig".to_string(),
            "UnifiedSecurityConfig".to_string(),
        );
        migration_map.insert(
            "SecurityLevelConfig".to_string(),
            "UnifiedSecurityConfig".to_string(),
        );
        migration_map.insert(
            "SecuritySentinelConfig".to_string(),
            "UnifiedSecurityConfig".to_string(),
        );
        migration_map.insert(
            "SecurityProcessorConfig".to_string(),
            "UnifiedSecurityConfig".to_string(),
        );

        migration_map.insert(
            "MonitoringConfig".to_string(),
            "UnifiedMonitoringConfig".to_string(),
        );
        migration_map.insert(
            "MetricCollectionConfig".to_string(),
            "UnifiedMonitoringConfig".to_string(),
        );
        migration_map.insert(
            "AlertProcessingConfig".to_string(),
            "UnifiedMonitoringConfig".to_string(),
        );
        migration_map.insert(
            "PrometheusConfig".to_string(),
            "UnifiedMonitoringConfig".to_string(),
        );

        migration_map.insert(
            "PerformanceConfig".to_string(),
            "UnifiedPerformanceConfig".to_string(),
        );
        migration_map.insert(
            "PerformanceRoutingConfig".to_string(),
            "UnifiedPerformanceConfig".to_string(),
        );
        migration_map.insert(
            "LoadTestConfiguration".to_string(),
            "UnifiedPerformanceConfig".to_string(),
        );

        migration_map.insert(
            "DatabaseConfig".to_string(),
            "UnifiedDatabaseConfig".to_string(),
        );

        deprecated_configs.extend([
            "BiomeOSAuthConfig".to_string(),
            "BiomeOSConnectionConfig".to_string(),
            "SongBirdConfig".to_string(),
            "UniversalAdapterConfig".to_string(), // Multiple instances
            "HealthCheckConfig".to_string(),      // Multiple instances
            "CircuitBreakerConfig".to_string(),   // Multiple instances
            "LoadBalancerConfig".to_string(),
            "HealthMonitorConfig".to_string(), // Multiple instances
            "KubernetesConfig".to_string(),
            "VendorHsmConfig".to_string(),
            "BridgeConfig".to_string(),
            "DetectorConfig".to_string(),
            "EvolutionConfig".to_string(),
            "ExtensibleAdapterConfig".to_string(),
            "DiscoveryEngineConfig".to_string(),
            "TlsConfig".to_string(), // Multiple instances
            "RouterConfig".to_string(),
            "ModelConfig".to_string(),
            "CircuitConfig".to_string(),
            "VaultConfig".to_string(),
            "CapabilityConfig".to_string(),
            "OAuth2Config".to_string(),
            "BearDogEcosystemConfig".to_string(),
            "DiscoveryConfig".to_string(),       // Multiple instances
            "ThreatDetectionConfig".to_string(), // Multiple instances
            "MlEngineConfig".to_string(),
        ]);

        let canonical_configs = vec![
            "UnifiedNetworkConfig".to_string(),
            "UnifiedSecurityConfig".to_string(),
            "UnifiedMonitoringConfig".to_string(),
            "UnifiedPerformanceConfig".to_string(),
            "UnifiedDatabaseConfig".to_string(),
            "UnifiedComplianceConfig".to_string(),
            "UnifiedDiscoveryConfig".to_string(),
            "UnifiedProductionConfig".to_string(),
            "UnifiedPlatformConfig".to_string(),
            "UnifiedConfigManager".to_string(),
        ];

        Self {
            migration_map,
            deprecated_configs,
            canonical_configs,
        }
    }

    pub fn get_canonical_type(&self, config_type: &str) -> Option<&String> {
        self.migration_map.get(config_type)
    }

    pub fn is_deprecated(&self, config_type: &str) -> bool {
        self.deprecated_configs.contains(&config_type.to_string())
    }

    pub fn canonical_types(&self) -> &Vec<String> {
        &self.canonical_configs
    }

    pub fn generate_migration_report(&self) -> ConfigurationMigrationReport {
        ConfigurationMigrationReport {
            total_deprecated: self.deprecated_configs.len(),
            total_canonical: self.canonical_configs.len(),
            migration_mappings: self.migration_map.len(),
            consolidation_ratio: self.deprecated_configs.len() as f32
                / self.canonical_configs.len() as f32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMigrationReport {
    pub total_deprecated: usize,

    pub total_canonical: usize,

    pub migration_mappings: usize,

    pub consolidation_ratio: f32,
}

pub struct ConfigurationMigrator;

impl ConfigurationMigrator {
    pub fn migrate_to_canonical<T, U>(legacy_config: T) -> Result<U, String>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let json = serde_json::to_string(&legacy_config)
            .map_err(|e| format_args!("Failed to serialize legacy config: {e}").to_string())?;

        let canonical = serde_json::from_str::<U>(&json).map_err(|e| {
            format_args!("Failed to deserialize to canonical config: {e}").to_string()
        })?;

        Ok(canonical)
    }

    pub fn validate_canonical_config<T>(config: &T) -> Result<(), String>
    where
        T: Serialize,
    {
        serde_json::to_string(config)
            .map_err(|e| format_args!("Configuration validation failed: {e}").to_string())?;

        Ok(())
    }

    pub fn generate_migration_script(
        crate_name: &str,
        registry: &ConfigurationMigrationRegistry,
    ) -> String {
        let mut script = format_args!("// Migration script for {crate_name}\n\n").to_string();

        script.push_str("// Replace fragmented imports with canonical imports:\n");
        for (deprecated, canonical) in &registry.migration_map {
            script.push_str(&format!(
                "// {deprecated} -> use crate::config::{canonical};\n"
            ));
        }

        script.push_str("\n// Remove deprecated configuration structs:\n");
        for deprecated in &registry.deprecated_configs {
            script.push_str(
                &format_args!("// Remove: pub struct {deprecated} {{ ... }}\n").to_string(),
            );
        }

        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_registry_creation() {
        let registry = ConfigurationMigrationRegistry::new();
        assert!(!registry.migration_map.is_empty());
        assert!(!registry.deprecated_configs.is_empty());
        assert!(!registry.canonical_configs.is_empty());
    }

    #[test]
    fn test_canonical_type_lookup() {
        let registry = ConfigurationMigrationRegistry::new();
        assert_eq!(
            registry.get_canonical_type("NetworkConfig"),
            Some(&"UnifiedNetworkConfig".to_string())
        );
        assert_eq!(
            registry.get_canonical_type("SecurityConfig"),
            Some(&"UnifiedSecurityConfig".to_string())
        );
    }

    #[test]
    fn test_deprecated_config_check() {
        let registry = ConfigurationMigrationRegistry::new();
        assert!(registry.is_deprecated("BiomeOSAuthConfig"));
        assert!(registry.is_deprecated("SongBirdConfig"));
        assert!(!registry.is_deprecated("UnifiedNetworkConfig"));
    }

    #[test]
    fn test_migration_report() {
        let registry = ConfigurationMigrationRegistry::new();
        let report = registry.generate_migration_report();
        assert!(report.total_deprecated > 0);
        assert!(report.total_canonical > 0);
        assert!(report.consolidation_ratio > 1.0); // More deprecated than canonical
    }
}
