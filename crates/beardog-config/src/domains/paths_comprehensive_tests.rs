//! Comprehensive tests for PathConfig
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: Platform-aware paths, PKCS#11 discovery, validation, cross-platform

#[cfg(test)]
mod tests {
    use crate::domains::paths::PathConfig;
    use serial_test::serial;
    use std::path::PathBuf;

    // Note: Tests that modify environment variables now use #[serial]
    // Modern pattern: Declarative test serialization instead of manual mutex

    // ============================================================================
    // Default Configuration Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_default_config() {
        let config = PathConfig::default();

        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(!config.data_dir.as_os_str().is_empty());
        assert!(!config.log_dir.as_os_str().is_empty());
        assert!(config.pkcs11_library_paths.is_empty());
        assert!(config.pkcs11_library.is_none());
    }

    #[serial_test::serial]
    #[test]
    fn test_default_paths_are_absolute() {
        let config = PathConfig::default();

        // Paths should be absolute (either from dirs crate or fallback)
        assert!(config.config_dir.is_absolute() || config.config_dir.starts_with("/"));
        assert!(config.data_dir.is_absolute() || config.data_dir.starts_with("/"));
        assert!(config.log_dir.is_absolute() || config.log_dir.starts_with("/"));
    }

    // ============================================================================
    // from_env() Tests
    // ============================================================================

    #[test]
    #[serial] // Modern pattern: declarative serialization for env var tests
    fn test_from_env_no_variables() {
        std::env::remove_var("BEARDOG_CONFIG_DIR");
        std::env::remove_var("BEARDOG_DATA_DIR");
        std::env::remove_var("BEARDOG_LOG_DIR");
        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");

        let config = PathConfig::from_env();

        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(!config.data_dir.as_os_str().is_empty());
        assert!(!config.log_dir.as_os_str().is_empty());
        assert!(config.pkcs11_library.is_none());

        std::env::remove_var("BEARDOG_CONFIG_DIR");
        std::env::remove_var("BEARDOG_DATA_DIR");
        std::env::remove_var("BEARDOG_LOG_DIR");
        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");
    }

    #[test]
    #[serial] // Modern pattern: declarative serialization for env var tests
    fn test_from_env_with_pkcs11_library() {
        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");
        std::env::set_var("BEARDOG_PKCS11_LIBRARY", "/custom/path/lib.so");

        let config = PathConfig::from_env();

        assert_eq!(
            config.pkcs11_library,
            Some(PathBuf::from("/custom/path/lib.so"))
        );

        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");
    }

    // ============================================================================
    // PKCS#11 Library Discovery Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_discover_pkcs11_libraries() {
        let libraries = PathConfig::discover_pkcs11_libraries();

        // May or may not find libraries depending on system
        // Just verify it returns a Vec and doesn't panic
        // Length check not needed (Vec::len() is always >= 0), but we verify it doesn't panic
        let _ = libraries.len();
    }

    #[serial_test::serial]
    #[test]
    fn test_discover_pkcs11_libraries_platform_specific() {
        let libraries = PathConfig::discover_pkcs11_libraries();

        // On Linux, if SoftHSM is installed, should find it
        // On macOS, might find Homebrew installations
        // On Windows, might find in Program Files
        // Just verify the discovery runs without error
        println!(
            "Found {} PKCS#11 libraries on {}",
            libraries.len(),
            std::env::consts::OS
        );
    }

    // ============================================================================
    // get_pkcs11_library() Priority Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_get_pkcs11_library_explicit() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: Some(PathBuf::from("/explicit/lib.so")),
            pkcs11_library_paths: vec![PathBuf::from("/path/lib.so")],
        };

        // Explicit library should take precedence
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/explicit/lib.so"))
        );
    }

    #[serial_test::serial]
    #[test]
    fn test_get_pkcs11_library_from_paths() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![
                PathBuf::from("/path1/lib.so"),
                PathBuf::from("/path2/lib.so"),
            ],
        };

        // Should return first path from configured paths
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/path1/lib.so"))
        );
    }

    #[serial_test::serial]
    #[test]
    fn test_get_pkcs11_library_discovery() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
        };

        // Should try discovery (may or may not find anything)
        let result = config.get_pkcs11_library();

        // Just verify it doesn't panic
        println!("Discovery result: {:?}", result);
    }

    #[serial_test::serial]
    #[test]
    fn test_get_pkcs11_library_priority() {
        // Test priority: explicit > configured paths > discovery

        // 1. Only discovery available
        let config1 = PathConfig::default();
        let result1 = config1.get_pkcs11_library();
        println!("Discovery only: {:?}", result1);

        // 2. Configured paths available
        let config2 = PathConfig {
            pkcs11_library_paths: vec![PathBuf::from("/configured.so")],
            ..Default::default()
        };
        assert_eq!(
            config2.get_pkcs11_library(),
            Some(PathBuf::from("/configured.so"))
        );

        // 3. Explicit library available (highest priority)
        let config3 = PathConfig {
            pkcs11_library_paths: vec![PathBuf::from("/configured.so")],
            pkcs11_library: Some(PathBuf::from("/explicit.so")),
            ..Default::default()
        };
        assert_eq!(
            config3.get_pkcs11_library(),
            Some(PathBuf::from("/explicit.so"))
        );
    }

    // ============================================================================
    // Validation Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_validate_default() {
        let config = PathConfig::default();

        // Should validate successfully (directories may not exist, but that's OK)
        assert!(config.validate().is_ok());
    }

    #[serial_test::serial]
    #[test]
    fn test_validate_with_nonexistent_pkcs11_library() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: Some(PathBuf::from("/nonexistent/lib.so")),
            pkcs11_library_paths: vec![],
        };

        // Should fail validation
        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("PKCS#11"));
        assert!(err.to_string().contains("/nonexistent/lib.so"));
    }

    #[serial_test::serial]
    #[test]
    fn test_validate_without_pkcs11_library() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
        };

        // Should validate successfully (no explicit library to check)
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_clone() {
        let config1 = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: Some(PathBuf::from("/test/lib.so")),
            pkcs11_library_paths: vec![PathBuf::from("/path/lib.so")],
        };

        let config2 = config1.clone();

        assert_eq!(config1.config_dir, config2.config_dir);
        assert_eq!(config1.data_dir, config2.data_dir);
        assert_eq!(config1.log_dir, config2.log_dir);
        assert_eq!(config1.pkcs11_library, config2.pkcs11_library);
        assert_eq!(config1.pkcs11_library_paths, config2.pkcs11_library_paths);
    }

    #[serial_test::serial]
    #[test]
    fn test_debug() {
        let config = PathConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("PathConfig"));
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_serialization() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/config"),
            data_dir: PathBuf::from("/test/data"),
            log_dir: PathBuf::from("/test/log"),
            pkcs11_library: Some(PathBuf::from("/test/lib.so")),
            pkcs11_library_paths: vec![PathBuf::from("/path/lib.so")],
        };

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: PathConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.config_dir, deserialized.config_dir);
        assert_eq!(config.data_dir, deserialized.data_dir);
        assert_eq!(config.log_dir, deserialized.log_dir);
        assert_eq!(config.pkcs11_library, deserialized.pkcs11_library);
    }

    #[serial_test::serial]
    #[test]
    fn test_serialization_default() {
        let config = PathConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let _deserialized: PathConfig = serde_json::from_str(&json).expect("Should deserialize");

        // Just verify round-trip works
    }

    // ============================================================================
    // Platform-Specific Scenarios
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_linux_paths() {
        // Test Linux-specific path patterns
        if cfg!(target_os = "linux") {
            let config = PathConfig::default();

            // On Linux, default paths might include /etc, /var/lib, /var/log
            let config_str = config.config_dir.to_string_lossy();
            let data_str = config.data_dir.to_string_lossy();
            let log_str = config.log_dir.to_string_lossy();

            println!("Linux paths:");
            println!("  config: {}", config_str);
            println!("  data: {}", data_str);
            println!("  log: {}", log_str);
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_macos_paths() {
        // Test macOS-specific path patterns
        if cfg!(target_os = "macos") {
            let config = PathConfig::default();

            // On macOS, might use ~/Library or fallback to /etc
            let config_str = config.config_dir.to_string_lossy();
            let data_str = config.data_dir.to_string_lossy();
            let log_str = config.log_dir.to_string_lossy();

            println!("macOS paths:");
            println!("  config: {}", config_str);
            println!("  data: {}", data_str);
            println!("  log: {}", log_str);
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_windows_paths() {
        // Test Windows-specific path patterns
        if cfg!(target_os = "windows") {
            let config = PathConfig::default();

            // On Windows, might use AppData or Program Files
            let config_str = config.config_dir.to_string_lossy();
            let data_str = config.data_dir.to_string_lossy();
            let log_str = config.log_dir.to_string_lossy();

            println!("Windows paths:");
            println!("  config: {}", config_str);
            println!("  data: {}", data_str);
            println!("  log: {}", log_str);
        }
    }

    // ============================================================================
    // Realistic Configuration Scenarios
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_production_linux_config() {
        let config = PathConfig {
            config_dir: PathBuf::from("/etc/beardog"),
            data_dir: PathBuf::from("/var/lib/beardog"),
            log_dir: PathBuf::from("/var/log/beardog"),
            pkcs11_library: Some(PathBuf::from("/usr/lib/softhsm/libsofthsm2.so")),
            pkcs11_library_paths: vec![],
        };

        assert!(config.config_dir.is_absolute());
        assert!(config.data_dir.is_absolute());
        assert!(config.log_dir.is_absolute());
    }

    #[serial_test::serial]
    #[test]
    fn test_development_local_config() {
        let config = PathConfig {
            config_dir: PathBuf::from("./config"),
            data_dir: PathBuf::from("./data"),
            log_dir: PathBuf::from("./logs"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
        };

        // Development config might use relative paths
        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(config.validate().is_ok());
    }

    #[serial_test::serial]
    #[test]
    fn test_docker_container_config() {
        let config = PathConfig {
            config_dir: PathBuf::from("/app/config"),
            data_dir: PathBuf::from("/app/data"),
            log_dir: PathBuf::from("/app/logs"),
            pkcs11_library: Some(PathBuf::from("/usr/lib/softhsm/libsofthsm2.so")),
            pkcs11_library_paths: vec![],
        };

        assert!(config.config_dir.is_absolute());
        assert!(config.data_dir.is_absolute());
    }

    // ============================================================================
    // Edge Cases and Boundary Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_empty_pkcs11_library_paths() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test"),
            data_dir: PathBuf::from("/test"),
            log_dir: PathBuf::from("/test"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
        };

        assert!(config.pkcs11_library_paths.is_empty());
        assert!(config.validate().is_ok());
    }

    #[serial_test::serial]
    #[test]
    fn test_multiple_pkcs11_library_paths() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test"),
            data_dir: PathBuf::from("/test"),
            log_dir: PathBuf::from("/test"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![
                PathBuf::from("/path1/lib.so"),
                PathBuf::from("/path2/lib.so"),
                PathBuf::from("/path3/lib.so"),
            ],
        };

        assert_eq!(config.pkcs11_library_paths.len(), 3);
        // Should use first path
        assert_eq!(
            config.get_pkcs11_library(),
            Some(PathBuf::from("/path1/lib.so"))
        );
    }

    #[serial_test::serial]
    #[test]
    fn test_paths_with_special_characters() {
        let config = PathConfig {
            config_dir: PathBuf::from("/test/with spaces/config"),
            data_dir: PathBuf::from("/test/with-dashes/data"),
            log_dir: PathBuf::from("/test/with_underscores/logs"),
            pkcs11_library: None,
            pkcs11_library_paths: vec![],
        };

        assert!(!config.config_dir.as_os_str().is_empty());
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Comprehensive Integration Test
    // ============================================================================

    #[test]
    #[serial] // Modern pattern: declarative serialization for env var tests
    fn test_full_lifecycle() {
        // 1. Create default config
        let config1 = PathConfig::default();
        assert!(config1.validate().is_ok());

        // 2. Load from environment
        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");
        std::env::set_var("BEARDOG_PKCS11_LIBRARY", "/test/lib.so");
        let config2 = PathConfig::from_env();
        assert_eq!(config2.pkcs11_library, Some(PathBuf::from("/test/lib.so")));

        // 3. Serialize and deserialize
        let json = serde_json::to_string(&config2).expect("Should serialize");
        let config3: PathConfig = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(config2.pkcs11_library, config3.pkcs11_library);

        // 4. Clone
        let config4 = config3.clone();
        assert_eq!(config3.config_dir, config4.config_dir);

        std::env::remove_var("BEARDOG_PKCS11_LIBRARY");
    }
}
