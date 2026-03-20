// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for SecurityConfig
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: All builder methods, validation, environment loading, edge cases

#[cfg(test)]
mod tests {
    use crate::domains::security::{
        DEFAULT_MIN_TLS_VERSION_FALLBACK, SecurityConfig, SecurityConfigBuilder,
    };

    // ============================================================================
    // SecurityConfig Default and Construction Tests
    // ============================================================================

    #[test]
    fn test_const_defaults() {
        let config = SecurityConfig::const_defaults();

        assert!(!config.strict_mode);
        assert!(!config.require_mtls);
        assert!(config.min_tls_version.is_empty()); // const_defaults has empty string
        assert!(config.allow_localhost_bypass);
        assert!(config.enable_audit_log);
        assert!(config.enable_rate_limiting);
        assert!(!config.auto_block_suspicious_ips);
        assert!(config.require_authentication);
    }

    #[test]
    fn test_default_has_tls_version() {
        let config = SecurityConfig::default();

        assert_eq!(config.min_tls_version, DEFAULT_MIN_TLS_VERSION_FALLBACK);
    }

    #[test]
    fn test_from_env_no_variables() {
        let config = SecurityConfig::from_env();

        assert_eq!(config.min_tls_version, DEFAULT_MIN_TLS_VERSION_FALLBACK);
        assert!(!config.strict_mode);
        assert!(config.enable_audit_log);
    }

    #[test]
    fn test_tls_version_via_builder() {
        let config = SecurityConfig::builder()
            .min_tls_version("1.3".to_string())
            .build();

        assert_eq!(config.min_tls_version, "1.3");
    }

    // ============================================================================
    // Builder Pattern Comprehensive Tests
    // ============================================================================

    #[test]
    fn test_builder_new() {
        let builder = SecurityConfigBuilder::new();
        let config = builder.build();

        assert_eq!(config, SecurityConfig::default());
    }

    #[test]
    fn test_builder_default() {
        let builder = SecurityConfigBuilder::default();
        let config = builder.build();

        assert_eq!(config, SecurityConfig::default());
    }

    #[test]
    fn test_builder_strict_mode_enabled() {
        let config = SecurityConfig::builder().strict_mode(true).build();

        assert!(config.strict_mode);
    }

    #[test]
    fn test_builder_strict_mode_disabled() {
        let config = SecurityConfig::builder().strict_mode(false).build();

        assert!(!config.strict_mode);
    }

    #[test]
    fn test_builder_require_mtls_enabled() {
        let config = SecurityConfig::builder().require_mtls(true).build();

        assert!(config.require_mtls);
    }

    #[test]
    fn test_builder_require_mtls_disabled() {
        let config = SecurityConfig::builder().require_mtls(false).build();

        assert!(!config.require_mtls);
    }

    #[test]
    fn test_builder_min_tls_version_1_2() {
        let config = SecurityConfig::builder()
            .min_tls_version(DEFAULT_MIN_TLS_VERSION_FALLBACK.to_string())
            .build();

        assert_eq!(config.min_tls_version, DEFAULT_MIN_TLS_VERSION_FALLBACK);
    }

    #[test]
    fn test_builder_min_tls_version_1_3() {
        let config = SecurityConfig::builder()
            .min_tls_version("1.3".to_string())
            .build();

        assert_eq!(config.min_tls_version, "1.3");
    }

    #[test]
    fn test_builder_allow_localhost_bypass_enabled() {
        let config = SecurityConfig::builder()
            .allow_localhost_bypass(true)
            .build();

        assert!(config.allow_localhost_bypass);
    }

    #[test]
    fn test_builder_allow_localhost_bypass_disabled() {
        let config = SecurityConfig::builder()
            .allow_localhost_bypass(false)
            .build();

        assert!(!config.allow_localhost_bypass);
    }

    #[test]
    fn test_builder_enable_audit_log_enabled() {
        let config = SecurityConfig::builder().enable_audit_log(true).build();

        assert!(config.enable_audit_log);
    }

    #[test]
    fn test_builder_enable_audit_log_disabled() {
        let config = SecurityConfig::builder().enable_audit_log(false).build();

        assert!(!config.enable_audit_log);
    }

    #[test]
    fn test_builder_enable_rate_limiting_enabled() {
        let config = SecurityConfig::builder().enable_rate_limiting(true).build();

        assert!(config.enable_rate_limiting);
    }

    #[test]
    fn test_builder_enable_rate_limiting_disabled() {
        let config = SecurityConfig::builder()
            .enable_rate_limiting(false)
            .build();

        assert!(!config.enable_rate_limiting);
    }

    #[test]
    fn test_builder_auto_block_suspicious_ips_enabled() {
        let config = SecurityConfig::builder()
            .auto_block_suspicious_ips(true)
            .build();

        assert!(config.auto_block_suspicious_ips);
    }

    #[test]
    fn test_builder_auto_block_suspicious_ips_disabled() {
        let config = SecurityConfig::builder()
            .auto_block_suspicious_ips(false)
            .build();

        assert!(!config.auto_block_suspicious_ips);
    }

    #[test]
    fn test_builder_require_authentication_enabled() {
        let config = SecurityConfig::builder()
            .require_authentication(true)
            .build();

        assert!(config.require_authentication);
    }

    #[test]
    fn test_builder_require_authentication_disabled() {
        let config = SecurityConfig::builder()
            .require_authentication(false)
            .build();

        assert!(!config.require_authentication);
    }

    #[test]
    fn test_builder_all_fields() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .min_tls_version("1.3".to_string())
            .allow_localhost_bypass(false)
            .enable_audit_log(true)
            .enable_rate_limiting(true)
            .auto_block_suspicious_ips(true)
            .require_authentication(true)
            .build();

        assert!(config.strict_mode);
        assert!(config.require_mtls);
        assert_eq!(config.min_tls_version, "1.3");
        assert!(!config.allow_localhost_bypass);
        assert!(config.enable_audit_log);
        assert!(config.enable_rate_limiting);
        assert!(config.auto_block_suspicious_ips);
        assert!(config.require_authentication);
    }

    // ============================================================================
    // Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_default_config() {
        let config = SecurityConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_strict_config() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .allow_localhost_bypass(false)
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_permissive_config() {
        let config = SecurityConfig::builder()
            .strict_mode(false)
            .require_mtls(false)
            .allow_localhost_bypass(true)
            .require_authentication(false)
            .build();

        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Configuration Scenarios
    // ============================================================================

    #[test]
    fn test_development_config() {
        let config = SecurityConfig::builder()
            .strict_mode(false)
            .require_mtls(false)
            .allow_localhost_bypass(true)
            .enable_audit_log(false)
            .auto_block_suspicious_ips(false)
            .build();

        assert!(!config.strict_mode);
        assert!(!config.require_mtls);
        assert!(config.allow_localhost_bypass);
        assert!(!config.enable_audit_log);
        assert!(!config.auto_block_suspicious_ips);
    }

    #[test]
    fn test_staging_config() {
        let config = SecurityConfig::builder()
            .strict_mode(false)
            .require_mtls(true)
            .min_tls_version(DEFAULT_MIN_TLS_VERSION_FALLBACK.to_string())
            .allow_localhost_bypass(false)
            .enable_audit_log(true)
            .enable_rate_limiting(true)
            .auto_block_suspicious_ips(false)
            .build();

        assert!(!config.strict_mode);
        assert!(config.require_mtls);
        assert!(!config.allow_localhost_bypass);
        assert!(config.enable_audit_log);
    }

    #[test]
    fn test_maximum_security_config() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .min_tls_version("1.3".to_string())
            .allow_localhost_bypass(false)
            .enable_audit_log(true)
            .enable_rate_limiting(true)
            .auto_block_suspicious_ips(true)
            .require_authentication(true)
            .build();

        assert!(config.strict_mode);
        assert!(config.require_mtls);
        assert_eq!(config.min_tls_version, "1.3");
        assert!(!config.allow_localhost_bypass);
        assert!(config.auto_block_suspicious_ips);
        assert!(config.require_authentication);
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[test]
    fn test_serialization() {
        let config = SecurityConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: SecurityConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_serialization_with_custom_values() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .min_tls_version("1.3".to_string())
            .build();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: SecurityConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.strict_mode, deserialized.strict_mode);
        assert_eq!(config.min_tls_version, deserialized.min_tls_version);
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_clone() {
        let config1 = SecurityConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_debug() {
        let config = SecurityConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("SecurityConfig"));
    }

    #[test]
    fn test_partial_eq() {
        let config1 = SecurityConfig::default();
        let config2 = SecurityConfig::default();
        let config3 = SecurityConfig::builder().strict_mode(true).build();

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    // ============================================================================
    // Builder Chaining Tests
    // ============================================================================

    #[test]
    fn test_builder_chaining_multiple_calls() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .enable_audit_log(true)
            .enable_rate_limiting(true)
            .build();

        assert!(config.strict_mode);
        assert!(config.require_mtls);
        assert!(config.enable_audit_log);
        assert!(config.enable_rate_limiting);
    }

    #[test]
    fn test_builder_partial_configuration() {
        let config = SecurityConfig::builder().strict_mode(true).build();

        // Unset fields should use defaults
        assert!(config.strict_mode);
        assert!(!config.require_mtls); // default
        assert_eq!(config.min_tls_version, DEFAULT_MIN_TLS_VERSION_FALLBACK); // default
    }
}
