// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for HSM Configuration
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: All HSM provider configurations, validation, scenarios

#[cfg(test)]
mod tests {
    use crate::domains::hsm::HsmConfig;
    use std::path::PathBuf;

    // ============================================================================
    // Default Configuration Tests
    // ============================================================================

    #[test]
    fn test_default_hsm_config() {
        let config = HsmConfig::default();

        assert!(config.auto_detect);
        assert!(config.prefer_hardware);
        assert!(config.enable_softhsm);
        assert!(!config.enable_yubihsm);
        assert!(!config.enable_tpm);
        assert!(!config.enable_strongbox);
        assert!(config.softhsm_config.is_none());
        assert!(config.yubihsm_connector.is_none());
        assert!(!config.provider_order.is_empty());
    }

    #[test]
    fn test_default_provider_order() {
        let config = HsmConfig::default();

        // Should have a default provider order
        assert!(!config.provider_order.is_empty());
    }

    #[test]
    fn test_validate_default_config() {
        let config = HsmConfig::default();
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Provider Configuration Tests
    // ============================================================================

    #[test]
    fn test_enable_all_providers() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            enable_softhsm: true,
            enable_yubihsm: true,
            enable_tpm: true,
            enable_strongbox: true,
            ..Default::default()
        };

        assert!(config.enable_softhsm);
        assert!(config.enable_yubihsm);
        assert!(config.enable_tpm);
        assert!(config.enable_strongbox);
    }

    #[test]
    fn test_disable_all_providers() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            enable_softhsm: false,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            ..Default::default()
        };

        assert!(!config.enable_softhsm);
        assert!(!config.enable_yubihsm);
        assert!(!config.enable_tpm);
        assert!(!config.enable_strongbox);
    }

    #[test]
    fn test_hardware_only_config() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            enable_softhsm: false,
            enable_yubihsm: true,
            enable_tpm: true,
            prefer_hardware: true,
            ..Default::default()
        };

        assert!(!config.enable_softhsm);
        assert!(config.enable_yubihsm);
        assert!(config.enable_tpm);
        assert!(config.prefer_hardware);
    }

    #[test]
    fn test_software_only_config() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            enable_softhsm: true,
            enable_yubihsm: false,
            enable_tpm: false,
            enable_strongbox: false,
            prefer_hardware: false,
            ..Default::default()
        };

        assert!(config.enable_softhsm);
        assert!(!config.prefer_hardware);
    }

    // ============================================================================
    // Auto-Detection Tests
    // ============================================================================

    #[test]
    fn test_auto_detect_enabled() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            auto_detect: true,
            ..Default::default()
        };

        assert!(config.auto_detect);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_auto_detect_disabled() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            auto_detect: false,
            ..Default::default()
        };

        assert!(!config.auto_detect);
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Path and Connector Configuration Tests
    // ============================================================================

    #[test]
    fn test_softhsm_config_path() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            softhsm_config: Some(PathBuf::from("/etc/softhsm2/softhsm2.conf")),
            ..Default::default()
        };

        assert_eq!(
            config.softhsm_config,
            Some(PathBuf::from("/etc/softhsm2/softhsm2.conf"))
        );
    }

    #[test]
    fn test_yubihsm_connector_url() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            yubihsm_connector: Some("http://localhost:12345".to_string()),
            ..Default::default()
        };

        assert_eq!(
            config.yubihsm_connector,
            Some("http://localhost:12345".to_string())
        );
    }

    #[test]
    fn test_multiple_configs_set() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            softhsm_config: Some(PathBuf::from("/path/to/softhsm.conf")),
            yubihsm_connector: Some("http://yubihsm:12345".to_string()),
            ..Default::default()
        };

        assert!(config.softhsm_config.is_some());
        assert!(config.yubihsm_connector.is_some());
    }

    // ============================================================================
    // Provider Order Tests
    // ============================================================================

    #[test]
    fn test_custom_provider_order() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            provider_order: vec![
                "yubihsm".to_string(),
                "tpm".to_string(),
                "softhsm".to_string(),
            ],
            ..Default::default()
        };

        assert_eq!(config.provider_order.len(), 3);
        assert_eq!(config.provider_order[0], "yubihsm");
    }

    #[test]
    fn test_empty_provider_order() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            provider_order: vec![],
            ..Default::default()
        };

        assert!(config.provider_order.is_empty());
    }

    #[test]
    fn test_single_provider_order() {
        // ✅ Idiomatic: Initialize with struct expression
        let config = HsmConfig {
            provider_order: vec!["softhsm".to_string()],
            ..Default::default()
        };

        assert_eq!(config.provider_order.len(), 1);
        assert_eq!(config.provider_order[0], "softhsm");
    }

    // ============================================================================
    // Realistic Configuration Scenarios
    // ============================================================================

    #[test]
    fn test_production_yubihsm_config() {
        // ✅ Idiomatic: Initialize all fields in one expression
        let config = HsmConfig {
            auto_detect: false,
            prefer_hardware: true,
            enable_softhsm: false,
            enable_yubihsm: true,
            enable_tpm: false,
            yubihsm_connector: Some("http://yubihsm-connector:12345".to_string()),
            provider_order: vec!["yubihsm".to_string()],
            ..Default::default()
        };

        assert!(!config.auto_detect);
        assert!(config.enable_yubihsm);
        assert!(config.yubihsm_connector.is_some());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_development_softhsm_config() {
        // ✅ Idiomatic: Initialize all fields in one expression
        let config = HsmConfig {
            auto_detect: true,
            prefer_hardware: false,
            enable_softhsm: true,
            enable_yubihsm: false,
            // Don't set softhsm_config path to avoid file existence check
            ..Default::default()
        };

        assert!(config.auto_detect);
        assert!(!config.prefer_hardware);
        assert!(config.enable_softhsm);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_android_strongbox_config() {
        // ✅ Idiomatic: Initialize all fields in one expression
        let config = HsmConfig {
            auto_detect: false,
            prefer_hardware: true,
            enable_strongbox: true,
            enable_softhsm: false,
            provider_order: vec!["strongbox".to_string()],
            ..Default::default()
        };

        assert!(config.enable_strongbox);
        assert!(config.prefer_hardware);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_tpm_config() {
        let config = HsmConfig {
            enable_tpm: true,
            enable_softhsm: false,
            prefer_hardware: true,
            provider_order: vec!["tpm".to_string()],
            ..Default::default()
        };

        assert!(config.enable_tpm);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_multi_provider_fallback_config() {
        let config = HsmConfig {
            auto_detect: true,
            prefer_hardware: true,
            enable_yubihsm: true,
            enable_tpm: true,
            enable_softhsm: true,
            provider_order: vec![
                "yubihsm".to_string(),
                "tpm".to_string(),
                "softhsm".to_string(),
            ],
            ..Default::default()
        };

        assert_eq!(config.provider_order.len(), 3);
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_clone() {
        let config1 = HsmConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.auto_detect, config2.auto_detect);
        assert_eq!(config1.enable_softhsm, config2.enable_softhsm);
    }

    #[test]
    fn test_debug() {
        let config = HsmConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("HsmConfig"));
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[test]
    fn test_serialization() {
        let config = HsmConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: HsmConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.auto_detect, deserialized.auto_detect);
        assert_eq!(config.enable_softhsm, deserialized.enable_softhsm);
    }

    #[test]
    fn test_serialization_with_paths() {
        let config = HsmConfig {
            softhsm_config: Some(PathBuf::from("/test/path")),
            yubihsm_connector: Some("http://test:12345".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: HsmConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.softhsm_config, deserialized.softhsm_config);
        assert_eq!(config.yubihsm_connector, deserialized.yubihsm_connector);
    }
}
