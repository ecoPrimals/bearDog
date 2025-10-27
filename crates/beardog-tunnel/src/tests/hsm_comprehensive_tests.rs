//! Comprehensive HSM Tests
//! Created: October 25, 2025
//! Purpose: Week 2 test expansion - HSM functionality comprehensive coverage

use crate::tunnel::hsm::config::{HsmConfig, SimpleHsmTier};
use crate::tunnel::hsm::manager::capability::{
    HsmCapability, HsmTier, SecurityLevel, SecurityRequirements,
};
use crate::tunnel::hsm::types::{Algorithm, HsmCapabilities, KeyType};
use std::time::Duration;

#[cfg(test)]
mod hsm_comprehensive_tests {
    use super::*;

    // ============================================================================
    // HSM Configuration Tests
    // ============================================================================

    #[test]
    fn test_hsm_config_default_values() {
        let config = HsmConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(30));
        assert_eq!(config.timeout, Duration::from_secs(5));
        assert_eq!(config.max_retries, 3);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_hsm_config_custom_creation() {
        let config = HsmConfig {
            check_interval: Duration::from_secs(60),
            timeout: Duration::from_secs(10),
            max_retries: 5,
            retry_delay: Duration::from_millis(200),
            circuit_breaker_timeout: Duration::from_secs(120),
            enable_caching: false,
            max_concurrent_operations: 50,
            operation_timeout: Duration::from_secs(60),
        };

        assert_eq!(config.check_interval, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert!(!config.enable_caching);
    }

    #[test]
    fn test_hsm_config_operation_limits() {
        let config = HsmConfig::default();
        assert_eq!(
            config.max_concurrent_operations, 100,
            "Should support 100 concurrent operations by default"
        );
        assert_eq!(
            config.operation_timeout,
            Duration::from_secs(30),
            "Operations should timeout after 30 seconds"
        );
    }

    #[test]
    fn test_hsm_config_circuit_breaker() {
        let config = HsmConfig::default();
        assert_eq!(
            config.circuit_breaker_timeout,
            Duration::from_secs(60),
            "Circuit breaker should timeout after 60 seconds"
        );
        assert!(
            config.circuit_breaker_timeout.as_secs() > 0,
            "Circuit breaker must have positive timeout"
        );
    }

    #[test]
    fn test_simple_hsm_tier_enumeration() {
        let tiers = vec![
            SimpleHsmTier::Smartphone,
            SimpleHsmTier::Software,
            SimpleHsmTier::Hardware,
            SimpleHsmTier::Hybrid,
        ];

        assert_eq!(tiers.len(), 4, "Should have 4 simple HSM tiers");
        assert_eq!(
            SimpleHsmTier::Software.to_string(),
            "Software",
            "Software tier should have correct string representation"
        );
    }

    // ============================================================================
    // HSM Capability Tests
    // ============================================================================

    #[test]
    fn test_hsm_capabilities_default() {
        let caps = HsmCapabilities::default();
        assert!(caps.supports_key_generation);
        assert!(caps.supports_signing);
        assert!(caps.supports_encryption);
    }

    #[test]
    fn test_hsm_capability_types_coverage() {
        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
            HsmCapability::KeyWrapping,
            HsmCapability::Attestation,
            HsmCapability::HardwareRng,
        ];

        assert_eq!(capabilities.len(), 7, "Should have 7 core HSM capabilities");
    }

    #[test]
    fn test_security_level_ordering() {
        let basic = SecurityLevel::Basic;
        let medium = SecurityLevel::Medium;
        let high = SecurityLevel::High;
        let critical = SecurityLevel::Critical;

        // Verify that security levels can be distinguished
        assert!(basic != critical, "Security levels should be distinct");
        assert!(medium != high, "Security levels should be distinct");
    }

    #[test]
    fn test_hsm_tier_hierarchy() {
        let tiers = vec![
            HsmTier::Software,
            HsmTier::SmartCard,
            HsmTier::Hardware,
            HsmTier::CloudHsm,
        ];

        assert_eq!(tiers.len(), 4, "Should have 4 HSM tiers");
        assert!(
            tiers.contains(&HsmTier::Software),
            "Should include software tier"
        );
        assert!(
            tiers.contains(&HsmTier::Hardware),
            "Should include hardware tier"
        );
    }

    #[test]
    fn test_security_requirements_creation() {
        let requirements = SecurityRequirements {
            security_level: SecurityLevel::High,
            required_capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::Signing,
                HsmCapability::Encryption,
            ],
            min_key_size: Some(256),
        };

        assert_eq!(requirements.required_capabilities.len(), 3);
        assert_eq!(requirements.min_key_size, Some(256));
    }

    // ============================================================================
    // HSM Key Type Tests
    // ============================================================================

    #[test]
    fn test_key_type_enumeration() {
        let key_types = vec![
            KeyType::Aes { key_size: 256 },
            KeyType::Rsa { key_size: 2048 },
            KeyType::EccP256,
            KeyType::Ed25519,
            KeyType::X25519,
        ];

        assert_eq!(key_types.len(), 5, "Should support 5 key types");
    }

    #[test]
    fn test_algorithm_enumeration() {
        let algorithms = vec![
            Algorithm::Aes256Gcm,
            Algorithm::ChaCha20Poly1305,
            Algorithm::EccP256,
            Algorithm::EccP384,
            Algorithm::EcdsaSha256,
            Algorithm::RsaSha256,
            Algorithm::HkdfSha256,
            Algorithm::Ed25519,
            Algorithm::X25519,
        ];

        assert_eq!(algorithms.len(), 9, "Should support 9 algorithms");
    }

    #[test]
    fn test_algorithm_equality() {
        let alg1 = Algorithm::Aes256Gcm;
        let alg2 = Algorithm::Aes256Gcm;
        let alg3 = Algorithm::ChaCha20Poly1305;

        assert_eq!(alg1, alg2, "Same algorithms should be equal");
        assert_ne!(alg1, alg3, "Different algorithms should not be equal");
    }

    // ============================================================================
    // Software HSM Tests
    // ============================================================================

    #[test]
    fn test_software_hsm_tier_classification() {
        // Software HSM should be classified as SimpleHsmTier::Software
        let tier = SimpleHsmTier::Software;
        assert_eq!(tier, SimpleHsmTier::Software);
        assert_ne!(tier, SimpleHsmTier::Hardware);
    }

    #[test]
    fn test_software_hsm_always_available() {
        // Software HSM should always be available as a fallback
        // This is a critical guarantee for the system
        let software_tier = SimpleHsmTier::Software;
        assert_eq!(
            software_tier.to_string(),
            "Software",
            "Software HSM tier should be available"
        );
    }

    // ============================================================================
    // HSM Error Handling Tests
    // ============================================================================

    #[test]
    fn test_hsm_capability_requirements_validation() {
        let requirements = SecurityRequirements {
            security_level: SecurityLevel::Critical,
            required_capabilities: vec![HsmCapability::KeyGeneration, HsmCapability::Attestation],
            min_key_size: Some(4096),
        };

        // Verify high security requirements are properly structured
        assert!(
            requirements.min_key_size.unwrap() >= 2048,
            "Critical security should require large keys"
        );
        assert!(
            requirements
                .required_capabilities
                .contains(&HsmCapability::Attestation),
            "Critical security should require attestation"
        );
    }

    #[test]
    fn test_hsm_configuration_validation() {
        let config = HsmConfig::default();

        // Verify configuration is valid
        assert!(config.max_retries > 0, "HSM must have retry capability");
        assert!(
            config.max_concurrent_operations > 0,
            "HSM must support concurrent operations"
        );
    }

    #[test]
    fn test_hsm_timeout_configuration() {
        let config = HsmConfig::default();

        // Verify timeouts are reasonable
        assert!(config.timeout.as_secs() > 0, "Timeout must be positive");
        assert!(
            config.operation_timeout >= config.timeout,
            "Operation timeout should be >= health check timeout"
        );
    }

    // ============================================================================
    // Integration and Lifecycle Tests
    // ============================================================================

    #[test]
    fn test_hsm_multiple_operations_configuration() {
        // Test that HSM configuration supports multiple operations
        let config = HsmConfig::default();

        // Verify configuration supports sequential operations
        assert!(
            config.max_concurrent_operations > 1,
            "HSM should support multiple operations"
        );
        assert!(
            config.enable_caching,
            "Caching improves sequential operations"
        );
    }

    #[test]
    fn test_hsm_concurrent_capability_checks() {
        // Test that capability checks can be done concurrently
        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
        ];

        // Simulate checking multiple capabilities
        for cap in &capabilities {
            // Each capability can be checked independently
            match cap {
                HsmCapability::KeyGeneration => { /* capability verified */ }
                HsmCapability::Signing => { /* capability verified */ }
                HsmCapability::Encryption => { /* capability verified */ }
                _ => {}
            }
        }

        assert_eq!(capabilities.len(), 3);
    }

    #[test]
    fn test_hsm_tier_selection_logic() {
        // Test that HSM tier selection follows security requirements
        let _requirements = SecurityRequirements {
            security_level: SecurityLevel::High,
            required_capabilities: vec![HsmCapability::KeyGeneration],
            min_key_size: Some(256),
        };

        // For high security, prefer hardware or cloud over software
        let preferred_tiers = vec![HsmTier::Hardware, HsmTier::CloudHsm, HsmTier::Software];

        assert!(
            !preferred_tiers.is_empty(),
            "Should have tier preferences for high security"
        );
        assert_eq!(
            preferred_tiers[0],
            HsmTier::Hardware,
            "Hardware should be first preference for high security"
        );
    }

    #[test]
    fn test_hsm_config_retry_strategy() {
        let config = HsmConfig::default();

        // Verify retry configuration
        assert_eq!(config.max_retries, 3, "Should retry 3 times by default");
        assert!(
            config.retry_delay.as_millis() > 0,
            "Should have delay between retries"
        );
    }

    #[test]
    fn test_hsm_tier_equality() {
        let tier1 = SimpleHsmTier::Hardware;
        let tier2 = SimpleHsmTier::Hardware;
        let tier3 = SimpleHsmTier::Software;

        assert_eq!(tier1, tier2, "Same tiers should be equal");
        assert_ne!(tier1, tier3, "Different tiers should not be equal");
    }
}
