// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for adapter configuration

use super::adapter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_adapter_config_default() {
        let config = UnifiedAdapterConfig::default();
        assert_eq!(config.core.adapter_id, "default-adapter");
        assert_eq!(config.core.adapter_type, AdapterType::Universal);
        assert!(config.security.enabled);
        assert!(config.monitoring.enabled);
    }

    #[test]
    fn test_adapter_config_validation() {
        let config = UnifiedAdapterConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config.clone();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        invalid_config.core.adapter_id = String::new();
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_development_config() -> Result<(), Box<dyn std::error::Error>> {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UnifiedAdapterConfig::development()?;
        assert!(!config.security.auth_required);
        assert!(!config.security.encryption_in_transit);
        assert!(!config.monitoring.enabled);
        assert_eq!(config.optimization.level, 1);
        Ok(())
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_production_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = UnifiedAdapterConfig::production()?;
        assert_eq!(config.security.auth_level, AuthLevel::MultiFactor);
        assert!(config.security.encryption_at_rest);
        assert_eq!(config.optimization.level, 5);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.service_mesh.enabled);
        Ok(())
    }

    #[test]
    fn test_config_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let config = UnifiedAdapterConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let toml_str = config.to_toml()?;
        assert!(toml_str.contains("[core]"));
        assert!(toml_str.contains("[discovery]"));
        assert!(toml_str.contains("[security]"));
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_migration_utilities() {
        let migrated = migration::migrate_legacy_adapter_config("test-adapter".to_string(), 50, 15);
        assert_eq!(migrated.core.adapter_id, "test-adapter");
        assert_eq!(migrated.core.max_connections, 50);
        assert_eq!(migrated.core.connection_timeout, Duration::from_secs(15));
    }
}

