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

//! # Configuration Migration Helper
//!
//! **CONFIGURATION CONSOLIDATION UTILITIES**
//! 
//! This module provides utilities to migrate from fragmented configuration
//! structs to canonical unified configurations, reducing 511 config structs
//! to <50 canonical types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import canonical configurations
use super::{
    UnifiedNetworkConfig, UnifiedSecurityConfig, UnifiedMonitoringConfig,
    UnifiedPerformanceConfig, DatabaseConfig, ComplianceConfig
};

/// Configuration Migration Registry
/// 
/// Tracks the consolidation of fragmented configurations into canonical types
pub struct ConfigurationMigrationRegistry {
    /// Maps old config type names to canonical types
    pub migration_map: HashMap<String, String>,
    /// Tracks deprecated configuration structs
    pub deprecated_configs: Vec<String>,
    /// Canonical configuration types
    pub canonical_configs: Vec<String>,
}

impl ConfigurationMigrationRegistry {
    /// Create a new migration registry with predefined mappings
    pub fn new() -> Self {
        let mut migration_map = HashMap::new();
        let mut deprecated_configs = Vec::new();
        
        // Network configuration mappings
        migration_map.insert("NetworkConfig".to_string(), "UnifiedNetworkConfig".to_string());
        migration_map.insert("NetworkPortsConfig".to_string(), "UnifiedNetworkConfig".to_string());
        migration_map.insert("NetworkScanConfig".to_string(), "UnifiedNetworkConfig".to_string());
        migration_map.insert("NetworkResourceConfig".to_string(), "UnifiedNetworkConfig".to_string());
        
        // Security configuration mappings
        migration_map.insert("SecurityConfig".to_string(), "UnifiedSecurityConfig".to_string());
        migration_map.insert("SecurityLevelConfig".to_string(), "UnifiedSecurityConfig".to_string());
        migration_map.insert("SecuritySentinelConfig".to_string(), "UnifiedSecurityConfig".to_string());
        migration_map.insert("SecurityProcessorConfig".to_string(), "UnifiedSecurityConfig".to_string());
        
        // Monitoring configuration mappings
        migration_map.insert("MonitoringConfig".to_string(), "UnifiedMonitoringConfig".to_string());
        migration_map.insert("MetricCollectionConfig".to_string(), "UnifiedMonitoringConfig".to_string());
        migration_map.insert("AlertProcessingConfig".to_string(), "UnifiedMonitoringConfig".to_string());
        migration_map.insert("PrometheusConfig".to_string(), "UnifiedMonitoringConfig".to_string());
        
        // Performance configuration mappings
        migration_map.insert("PerformanceConfig".to_string(), "UnifiedPerformanceConfig".to_string());
        migration_map.insert("PerformanceRoutingConfig".to_string(), "UnifiedPerformanceConfig".to_string());
        migration_map.insert("LoadTestConfiguration".to_string(), "UnifiedPerformanceConfig".to_string());
        
        // Database configuration mappings
        migration_map.insert("DatabaseConfig".to_string(), "UnifiedDatabaseConfig".to_string());
        
        // Mark fragmented configs as deprecated
        deprecated_configs.extend([
            "BiomeOSAuthConfig".to_string(),
            "BiomeOSConnectionConfig".to_string(),
            "SongBirdConfig".to_string(),
            "UniversalAdapterConfig".to_string(), // Multiple instances
            "HealthCheckConfig".to_string(), // Multiple instances
            "CircuitBreakerConfig".to_string(), // Multiple instances
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
            "DiscoveryConfig".to_string(), // Multiple instances
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
    
    /// Get the canonical configuration type for a given config
    pub fn get_canonical_type(&self, config_type: &str) -> Option<&String> {
        self.migration_map.get(config_type)
    }
    
    /// Check if a configuration type is deprecated
    pub fn is_deprecated(&self, config_type: &str) -> bool {
        self.deprecated_configs.contains(&config_type.to_string())
    }
    
    /// Get all canonical configuration types
    pub fn canonical_types(&self) -> &Vec<String> {
        &self.canonical_configs
    }
    
    /// Generate migration report
    pub fn generate_migration_report(&self) -> ConfigurationMigrationReport {
        ConfigurationMigrationReport {
            total_deprecated: self.deprecated_configs.len(),
            total_canonical: self.canonical_configs.len(),
            migration_mappings: self.migration_map.len(),
            consolidation_ratio: self.deprecated_configs.len() as f32 / self.canonical_configs.len() as f32,
        }
    }
}

/// Configuration migration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMigrationReport {
    /// Total number of deprecated configurations
    pub total_deprecated: usize,
    /// Total number of canonical configurations
    pub total_canonical: usize,
    /// Number of migration mappings
    pub migration_mappings: usize,
    /// Consolidation ratio (deprecated/canonical)
    pub consolidation_ratio: f32,
}

/// Configuration Migration Utilities
pub struct ConfigurationMigrator;

impl ConfigurationMigrator {
    /// Migrate a legacy configuration to its canonical equivalent
    pub fn migrate_to_canonical<T, U>(legacy_config: T) -> Result<U, String> 
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        // Serialize legacy config to JSON
        let json = serde_json::to_string(&legacy_config)
            .map_err(|e| format!("Failed to serialize legacy config: {}", e))?;
        
        // Deserialize to canonical config
        let canonical = serde_json::from_str::<U>(&json)
            .map_err(|e| format!("Failed to deserialize to canonical config: {}", e))?;
        
        Ok(canonical)
    }
    
    /// Validate canonical configuration
    pub fn validate_canonical_config<T>(config: &T) -> Result<(), String>
    where
        T: Serialize,
    {
        // Basic validation - ensure config can be serialized
        serde_json::to_string(config)
            .map_err(|e| format!("Configuration validation failed: {}", e))?;
        
        Ok(())
    }
    
    /// Generate migration script for a crate
    pub fn generate_migration_script(crate_name: &str, registry: &ConfigurationMigrationRegistry) -> String {
        let mut script = format!("// Migration script for {}\n\n", crate_name);
        
        script.push_str("// Replace fragmented imports with canonical imports:\n");
        for (deprecated, canonical) in &registry.migration_map {
            script.push_str(&format!(
                "// {} -> use beardog_types::config::{};\n", 
                deprecated, canonical
            ));
        }
        
        script.push_str("\n// Remove deprecated configuration structs:\n");
        for deprecated in &registry.deprecated_configs {
            script.push_str(&format!("// Remove: pub struct {} {{ ... }}\n", deprecated));
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