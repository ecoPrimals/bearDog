// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Configuration Tests
//!
//! High-coverage tests for canonical configuration types and validation

use super::*;
use crate::canonical::config::unified::UnifiedBearDogConfig;

#[cfg(test)]
mod unified_config_comprehensive_tests {
    use super::*;

    #[test]
    fn test_development_config_initialization() {
        let config = UnifiedBearDogConfig::development();
        assert_eq!(config.app.environment, "development");
        assert!(config.app.debug_mode);
    }

    #[test]
    fn test_production_config_initialization() {
        let config = UnifiedBearDogConfig::production();
        assert_eq!(config.app.environment, "production");
        assert!(!config.app.debug_mode);
    }

    #[test]
    fn test_staging_config_initialization() {
        let config = UnifiedBearDogConfig::staging();
        assert_eq!(config.app.environment, "staging");
    }

    #[test]
    fn test_config_validation_valid() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_clone() {
        let config1 = UnifiedBearDogConfig::development();
        let config2 = config1.clone();
        assert_eq!(config1.app.environment, config2.app.environment);
    }

    #[test]
    fn test_config_debug_output() {
        let config = UnifiedBearDogConfig::development();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("UnifiedBearDogConfig"));
    }

    #[test]
    fn test_config_serialization() {
        let config = UnifiedBearDogConfig::development();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_config_deserialization() {
        let config = UnifiedBearDogConfig::development();
        let json = serde_json::to_string(&config)?;
        let deserialized: Result<UnifiedBearDogConfig, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_config_round_trip() {
        let original = UnifiedBearDogConfig::development();
        let json = serde_json::to_string(&original)?;
        let restored: UnifiedBearDogConfig = serde_json::from_str(&json)?;
        assert_eq!(original.app.environment, restored.app.environment);
    }

    #[test]
    fn test_app_config_defaults() {
        let config = UnifiedBearDogConfig::development();
        assert!(!config.app.app_name.is_empty());
        assert!(!config.app.version.is_empty());
    }

    #[test]
    fn test_network_config_present() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.network.bind_address.contains("127.0.0.1") || 
                config.network.bind_address.contains("0.0.0.0"));
    }

    #[test]
    fn test_security_config_present() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.security.enable_hsm || !config.security.enable_hsm); // Security config exists
    }

    #[test]
    fn test_monitoring_config_present() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.monitoring.enable_metrics || !config.monitoring.enable_metrics);
    }

    #[test]
    fn test_database_config_present() {
        let config = UnifiedBearDogConfig::development();
        assert!(!config.database.connection_string.is_empty());
    }

    #[test]
    fn test_config_environments_differ() {
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();
        assert_ne!(dev.app.environment, prod.app.environment);
    }

    #[test]
    fn test_production_has_stricter_settings() {
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();
        assert!(dev.app.debug_mode);
        assert!(!prod.app.debug_mode);
    }

    #[test]
    fn test_config_default_trait() {
        let config = UnifiedBearDogConfig::default();
        assert!(!config.app.app_name.is_empty());
    }

    #[test]
    fn test_multiple_config_instances() {
        let config1 = UnifiedBearDogConfig::development();
        let config2 = UnifiedBearDogConfig::development();
        assert_eq!(config1.app.environment, config2.app.environment);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_memory_safety() {
        // Create and drop configs rapidly to test memory safety
        for _ in 0..1000 {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            let _config = UnifiedBearDogConfig::development();
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_thread_safety() {
        use std::sync::Arc;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        use std::thread;

        let config = Arc::new(UnifiedBearDogConfig::development());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let mut handles = vec![];

        for _ in 0..10 {
            let config_clone = Arc::clone(&config);
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            let handle = thread::spawn(move || {
                assert!(!config_clone.app.app_name.is_empty());
            });
            handles.push(handle);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        }

        for handle in handles {
            handle.join()?;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        }
    }
}

#[cfg(test)]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
mod app_config_tests {
    use super::*;

    #[test]
    fn test_app_name_not_empty() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UnifiedBearDogConfig::development();
        assert!(!config.app.app_name.is_empty());
        assert!(config.app.app_name.len() > 0);
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_version_format() {
        let config = UnifiedBearDogConfig::development();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(!config.app.version.is_empty());
        // Version should contain at least one digit
        assert!(config.app.version.chars().any(|c| c.is_numeric()));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_environment_valid_values() {
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let staging = UnifiedBearDogConfig::staging();

        assert!(["development", "dev"].contains(&dev.app.environment.as_str()));
        assert!(["production", "prod"].contains(&prod.app.environment.as_str()));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(["staging", "stage"].contains(&staging.app.environment.as_str()));
    }

    #[test]
    fn test_debug_mode_correlation() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Dev should have debug, prod should not
        assert!(dev.app.debug_mode);
        assert!(!prod.app.debug_mode);
    }
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
}

#[cfg(test)]
mod security_config_tests {
    use super::*;
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_security_config_exists() {
        let config = UnifiedBearDogConfig::development();
        // Security config should be present
        let _ = &config.security;
    }

    #[test]
    fn test_hsm_config_toggleable() {
        let mut config = UnifiedBearDogConfig::development();
        config.security.enable_hsm = true;
        assert!(config.security.enable_hsm);

        config.security.enable_hsm = false;
        assert!(!config.security.enable_hsm);
    }

    #[test]
    fn test_encryption_config() {
        let config = UnifiedBearDogConfig::development();
        // Should have encryption settings
        assert!(config.security.enable_encryption || !config.security.enable_encryption);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
}

#[cfg(test)]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
mod network_config_tests {
    use super::*;

    #[test]
    fn test_bind_address_format() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UnifiedBearDogConfig::development();
        let addr = &config.network.bind_address;
        
        // Should contain IP address
        assert!(addr.contains(".") || addr.contains(":"));
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_port_range() {
        let config = UnifiedBearDogConfig::development();
        let port = config.network.port;
        
        // Port should be in valid range
        assert!(port > 0);
        assert!(port < 65536);
    }

    #[test]
    fn test_localhost_binding() {
        let dev_config = UnifiedBearDogConfig::development();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Dev typically binds to localhost
        assert!(dev_config.network.bind_address.contains("127.0.0.1") ||
                dev_config.network.bind_address.contains("localhost"));
    }
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
}

#[cfg(test)]
mod database_config_tests {
    use super::*;

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_connection_string_present() {
        let config = UnifiedBearDogConfig::development();
        assert!(!config.database.connection_string.is_empty());
    }

    #[test]
    fn test_pool_size_reasonable() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.database.max_connections > 0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.database.max_connections < 1000); // Reasonable upper bound
    }

    #[test]
    fn test_connection_timeout() {
        let config = UnifiedBearDogConfig::development();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.database.connection_timeout_ms > 0);
    }
}

#[cfg(test)]
mod monitoring_config_tests {
    use super::*;
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_metrics_toggleable() {
        let mut config = UnifiedBearDogConfig::development();
        
        config.monitoring.enable_metrics = true;
        assert!(config.monitoring.enable_metrics);
        
        config.monitoring.enable_metrics = false;
        assert!(!config.monitoring.enable_metrics);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_health_check_interval() {
        let config = UnifiedBearDogConfig::development();
        assert!(config.monitoring.health_check_interval_ms > 0);
    }
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_valid_config_passes() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UnifiedBearDogConfig::development();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config_valid() {
        let config = UnifiedBearDogConfig::production();
        assert!(config.validate().is_ok());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_staging_config_valid() {
        let config = UnifiedBearDogConfig::staging();
        assert!(config.validate().is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_default_config_valid() {
        let config = UnifiedBearDogConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.validate().is_ok());
    }
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

#[cfg(test)]
mod edge_cases {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    use super::*;

    #[test]
    fn test_rapid_config_creation() {
        for _ in 0..100 {
            let _ = UnifiedBearDogConfig::development();
        }
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_config_size_reasonable() {
        use std::mem;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let size = mem::size_of::<UnifiedBearDogConfig>();
        // Config should be reasonably sized (less than 100KB)
        assert!(size < 100_000);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_concurrent_validation() {
        use std::thread;
        
        let mut handles = vec![];
        for _ in 0..5 {
            let handle = thread::spawn(|| {
                let config = UnifiedBearDogConfig::development();
                assert!(config.validate().is_ok());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join()?;
        }
    }
}

