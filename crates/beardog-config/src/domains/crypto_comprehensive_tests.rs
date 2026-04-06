// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for `CryptoConfig`
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: Algorithm validation, security levels, FIPS mode, builder patterns

#[cfg(test)]
mod tests {
    use crate::domains::crypto::{CryptoConfig, CryptoConfigBuilder};

    // ============================================================================
    // Default and Construction Tests
    // ============================================================================

    #[test]
    fn test_const_defaults() {
        let config = CryptoConfig::const_defaults();

        assert_eq!(config.rsa_key_size, 2048);
        assert_eq!(config.ec_curve, "secp256r1");
        assert_eq!(config.aes_key_size, 256);
        assert_eq!(config.hash_algorithm, "sha256");
        assert_eq!(config.pbkdf2_iterations, 100_000);
        assert!(!config.fips_mode);
    }

    #[test]
    fn test_default_equals_const_defaults() {
        let default = CryptoConfig::default();
        let const_def = CryptoConfig::const_defaults();

        assert_eq!(default, const_def);
    }

    #[test]
    fn test_default_validates() {
        let config = CryptoConfig::default();
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // RSA Key Size Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_rsa_2048() {
        let config = CryptoConfigBuilder::new().rsa_key_size(2048).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_rsa_3072() {
        let config = CryptoConfigBuilder::new().rsa_key_size(3072).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_rsa_4096() {
        let config = CryptoConfigBuilder::new().rsa_key_size(4096).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_rsa_too_small() {
        let config = CryptoConfigBuilder::new().rsa_key_size(1024).build();

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("RSA key size"));
        assert!(err.to_string().contains("2048"));
    }

    #[test]
    fn test_validate_rsa_512_fails() {
        let config = CryptoConfigBuilder::new().rsa_key_size(512).build();

        assert!(config.validate().is_err());
    }

    // ============================================================================
    // AES Key Size Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_aes_128() {
        let config = CryptoConfigBuilder::new().aes_key_size(128).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_aes_192() {
        let config = CryptoConfigBuilder::new().aes_key_size(192).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_aes_256() {
        let config = CryptoConfigBuilder::new().aes_key_size(256).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_aes_512_fails() {
        let config = CryptoConfigBuilder::new().aes_key_size(512).build();

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("AES key size"));
        assert!(err.to_string().contains("128"));
    }

    #[test]
    fn test_validate_aes_64_fails() {
        let config = CryptoConfigBuilder::new().aes_key_size(64).build();

        assert!(config.validate().is_err());
    }

    // ============================================================================
    // EC Curve Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_ec_secp256r1() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("secp256r1".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_ec_secp384r1() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("secp384r1".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_ec_secp521r1() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("secp521r1".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_ec_ed25519() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("ed25519".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_ec_invalid_curve_fails() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("invalid_curve".to_string())
            .build();

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("ec_curve"));
    }

    #[test]
    fn test_validate_ec_secp256k1_fails() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("secp256k1".to_string()) // Not in allowed list
            .build();

        assert!(config.validate().is_err());
    }

    // ============================================================================
    // Hash Algorithm Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_hash_sha256() {
        let config = CryptoConfigBuilder::new()
            .hash_algorithm("sha256".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_hash_sha384() {
        let config = CryptoConfigBuilder::new()
            .hash_algorithm("sha384".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_hash_sha512() {
        let config = CryptoConfigBuilder::new()
            .hash_algorithm("sha512".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_hash_md5_fails() {
        let config = CryptoConfigBuilder::new()
            .hash_algorithm("md5".to_string())
            .build();

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("hash_algorithm"));
    }

    #[test]
    fn test_validate_hash_sha1_fails() {
        let config = CryptoConfigBuilder::new()
            .hash_algorithm("sha1".to_string())
            .build();

        assert!(config.validate().is_err());
    }

    // ============================================================================
    // PBKDF2 Iterations Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_pbkdf2_100000() {
        let config = CryptoConfigBuilder::new()
            .pbkdf2_iterations(100_000)
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_pbkdf2_250000() {
        let config = CryptoConfigBuilder::new()
            .pbkdf2_iterations(250_000)
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_pbkdf2_too_low_fails() {
        let config = CryptoConfigBuilder::new().pbkdf2_iterations(50_000).build();

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("PBKDF2"));
        assert!(err.to_string().contains("100,000"));
    }

    #[test]
    fn test_validate_pbkdf2_1000_fails() {
        let config = CryptoConfigBuilder::new().pbkdf2_iterations(1_000).build();

        assert!(config.validate().is_err());
    }

    // ============================================================================
    // FIPS Mode Tests
    // ============================================================================

    #[test]
    fn test_fips_mode_disabled_by_default() {
        let config = CryptoConfig::default();
        assert!(!config.fips_mode);
    }

    #[test]
    fn test_fips_mode_enabled() {
        let config = CryptoConfigBuilder::new().fips_mode(true).build();

        assert!(config.fips_mode);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_fips_mode_disabled() {
        let config = CryptoConfigBuilder::new().fips_mode(false).build();

        assert!(!config.fips_mode);
    }

    // ============================================================================
    // Builder Pattern Tests
    // ============================================================================

    #[test]
    fn test_builder_all_fields() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(4096)
            .ec_curve("secp384r1".to_string())
            .aes_key_size(192)
            .hash_algorithm("sha384".to_string())
            .pbkdf2_iterations(150_000)
            .fips_mode(true)
            .build();

        assert_eq!(config.rsa_key_size, 4096);
        assert_eq!(config.ec_curve, "secp384r1");
        assert_eq!(config.aes_key_size, 192);
        assert_eq!(config.hash_algorithm, "sha384");
        assert_eq!(config.pbkdf2_iterations, 150_000);
        assert!(config.fips_mode);
    }

    #[test]
    fn test_builder_partial_fields() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(4096)
            .hash_algorithm("sha512".to_string())
            .build();

        assert_eq!(config.rsa_key_size, 4096);
        assert_eq!(config.hash_algorithm, "sha512");
        // Others should be defaults
        assert_eq!(config.aes_key_size, 256);
        assert_eq!(config.ec_curve, "secp256r1");
    }

    #[test]
    fn test_builder_empty() {
        let config = CryptoConfigBuilder::new().build();

        // Should use all defaults
        assert_eq!(config, CryptoConfig::default());
    }

    // ============================================================================
    // from_env() Tests
    // ============================================================================

    #[test]
    fn test_from_env_no_variables() {
        let config = CryptoConfig::from_env();

        assert_eq!(config, CryptoConfig::default());
    }

    #[test]
    fn test_rsa_key_size_via_builder() {
        let config = CryptoConfig::builder().rsa_key_size(4096).build();

        assert_eq!(config.rsa_key_size, 4096);
    }

    #[test]
    fn test_default_rsa_key_size() {
        let config = CryptoConfig::default();
        assert_eq!(config.rsa_key_size, 2048);
    }

    // ============================================================================
    // Security Level Scenarios
    // ============================================================================

    #[test]
    fn test_minimum_security_config() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(2048)
            .aes_key_size(128)
            .ec_curve("secp256r1".to_string())
            .hash_algorithm("sha256".to_string())
            .pbkdf2_iterations(100_000)
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_standard_security_config() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(2048)
            .aes_key_size(256)
            .ec_curve("secp256r1".to_string())
            .hash_algorithm("sha256".to_string())
            .pbkdf2_iterations(100_000)
            .build();

        assert!(config.validate().is_ok());
        assert_eq!(config, CryptoConfig::default());
    }

    #[test]
    fn test_high_security_config() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(4096)
            .aes_key_size(256)
            .ec_curve("secp521r1".to_string())
            .hash_algorithm("sha512".to_string())
            .pbkdf2_iterations(250_000)
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_fips_compliant_config() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(2048)
            .aes_key_size(256)
            .ec_curve("secp384r1".to_string())
            .hash_algorithm("sha384".to_string())
            .pbkdf2_iterations(100_000)
            .fips_mode(true)
            .build();

        assert!(config.validate().is_ok());
        assert!(config.fips_mode);
    }

    #[test]
    fn test_ed25519_modern_config() {
        let config = CryptoConfigBuilder::new()
            .ec_curve("ed25519".to_string())
            .aes_key_size(256)
            .hash_algorithm("sha512".to_string())
            .pbkdf2_iterations(100_000)
            .build();

        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_clone() {
        let config1 = CryptoConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_debug() {
        let config = CryptoConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("CryptoConfig"));
    }

    #[test]
    fn test_partial_eq() {
        let config1 = CryptoConfig::default();
        let config2 = CryptoConfig::default();
        let config3 = CryptoConfigBuilder::new().rsa_key_size(4096).build();

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[test]
    fn test_serialization() {
        let config = CryptoConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: CryptoConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_serialization_custom_values() {
        let config = CryptoConfigBuilder::new()
            .rsa_key_size(4096)
            .fips_mode(true)
            .build();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: CryptoConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.rsa_key_size, deserialized.rsa_key_size);
        assert_eq!(config.fips_mode, deserialized.fips_mode);
    }

    // ============================================================================
    // Edge Cases and Error Messages
    // ============================================================================

    #[test]
    fn test_validation_error_messages() {
        // RSA error
        let config1 = CryptoConfigBuilder::new().rsa_key_size(1024).build();
        let err1 = config1.validate().unwrap_err();
        assert!(err1.to_string().contains("crypto.rsa_key_size"));

        // AES error
        let config2 = CryptoConfigBuilder::new().aes_key_size(512).build();
        let err2 = config2.validate().unwrap_err();
        assert!(err2.to_string().contains("crypto.aes_key_size"));

        // EC curve error
        let config3 = CryptoConfigBuilder::new()
            .ec_curve("invalid".to_string())
            .build();
        let err3 = config3.validate().unwrap_err();
        assert!(err3.to_string().contains("crypto.ec_curve"));

        // Hash error
        let config4 = CryptoConfigBuilder::new()
            .hash_algorithm("md5".to_string())
            .build();
        let err4 = config4.validate().unwrap_err();
        assert!(err4.to_string().contains("crypto.hash_algorithm"));

        // PBKDF2 error
        let config5 = CryptoConfigBuilder::new().pbkdf2_iterations(1000).build();
        let err5 = config5.validate().unwrap_err();
        assert!(err5.to_string().contains("crypto.pbkdf2_iterations"));
    }

    #[test]
    fn test_builder_immutability() {
        let builder1 = CryptoConfigBuilder::new();
        let builder2 = builder1.rsa_key_size(4096);
        let config = builder2.build();

        assert_eq!(config.rsa_key_size, 4096);
    }
}
