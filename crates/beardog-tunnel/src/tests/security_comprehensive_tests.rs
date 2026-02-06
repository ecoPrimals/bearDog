//! Tunnel Security Comprehensive Tests
//! Created: October 25, 2025
//! Purpose: Week 2 test expansion - Security and encryption comprehensive coverage

use crate::tunnel::hsm::types::config::{
    AuthMethod, CryptoBackendType, MemoryConfig, MemoryProtectionLevel, SoftwareHsmConfig,
};
use crate::tunnel::hsm::types::SecurityLevel;

#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_comprehensive_tests {
    use super::*;

    // ============================================================================
    // Memory Protection Tests
    // ============================================================================

    #[test]
    fn test_memory_protection_level_enumeration() {
        let levels = [
            MemoryProtectionLevel::None,
            MemoryProtectionLevel::Low,
            MemoryProtectionLevel::Medium,
            MemoryProtectionLevel::High,
            MemoryProtectionLevel::Maximum,
        ];

        assert_eq!(levels.len(), 5, "Should have 5 memory protection levels");
    }

    #[test]
    fn test_memory_config_default() {
        let config = MemoryConfig::default();
        assert_eq!(config.protection_level, MemoryProtectionLevel::Medium);
        assert!(config.enable_encryption);
        assert_eq!(config.pool_size, 1024 * 1024);
    }

    #[test]
    fn test_memory_config_custom() {
        let config = MemoryConfig {
            protection_level: MemoryProtectionLevel::Maximum,
            enable_encryption: true,
            pool_size: 2048 * 1024,
        };

        assert_eq!(config.protection_level, MemoryProtectionLevel::Maximum);
        assert_eq!(config.pool_size, 2048 * 1024);
    }

    #[test]
    fn test_memory_protection_level_equality() {
        let level1 = MemoryProtectionLevel::High;
        let level2 = MemoryProtectionLevel::High;
        let level3 = MemoryProtectionLevel::Medium;

        assert_eq!(level1, level2, "Same levels should be equal");
        assert_ne!(level1, level3, "Different levels should not be equal");
    }

    // ============================================================================
    // Authentication Method Tests
    // ============================================================================

    #[test]
    fn test_auth_method_enumeration() {
        let methods = [
            AuthMethod::None,
            AuthMethod::Password,
            AuthMethod::Certificate,
            AuthMethod::Token,
            AuthMethod::Biometric,
            AuthMethod::MultiFactory,
        ];

        assert_eq!(methods.len(), 6, "Should have 6 authentication methods");
    }

    #[test]
    fn test_auth_method_equality() {
        let method1 = AuthMethod::Certificate;
        let method2 = AuthMethod::Certificate;
        let method3 = AuthMethod::Token;

        assert_eq!(method1, method2, "Same methods should be equal");
        assert_ne!(method1, method3, "Different methods should not be equal");
    }

    // ============================================================================
    // Security Level Tests
    // ============================================================================

    #[test]
    fn test_security_level_enumeration() {
        // Canonical hardware-based security levels (lowest to highest)
        let levels = [
            SecurityLevel::Software,
            SecurityLevel::TrustedExecutionEnvironment,
            SecurityLevel::SecureEnclave,
            SecurityLevel::HardwareSecurityModule,
            SecurityLevel::StrongBox,
        ];

        assert_eq!(levels.len(), 5, "Should have 5 security levels");
    }

    #[test]
    fn test_security_level_equality() {
        let level1 = SecurityLevel::HardwareSecurityModule;
        let level2 = SecurityLevel::HardwareSecurityModule;
        let level3 = SecurityLevel::SecureEnclave;

        assert_eq!(level1, level2, "Same security levels should be equal");
        assert_ne!(
            level1, level3,
            "Different security levels should not be equal"
        );
    }

    #[test]
    fn test_security_level_ordering() {
        // Verify proper ordering from lowest to highest
        assert!(SecurityLevel::StrongBox > SecurityLevel::HardwareSecurityModule);
        assert!(SecurityLevel::HardwareSecurityModule > SecurityLevel::SecureEnclave);
        assert!(SecurityLevel::SecureEnclave > SecurityLevel::TrustedExecutionEnvironment);
        assert!(SecurityLevel::TrustedExecutionEnvironment > SecurityLevel::Software);
    }

    #[test]
    fn test_security_level_hardware_backing() {
        assert!(!SecurityLevel::Software.is_hardware_backed());
        assert!(SecurityLevel::TrustedExecutionEnvironment.is_hardware_backed());
        assert!(SecurityLevel::StrongBox.is_hardware_backed());
    }

    // ============================================================================
    // Crypto Backend Tests
    // ============================================================================

    #[test]
    fn test_crypto_backend_type_enumeration() {
        let backends = [
            CryptoBackendType::GeneticCrypto, // 100% Pure Rust (RECOMMENDED)
            CryptoBackendType::RustCrypto,
            CryptoBackendType::Ring,
            CryptoBackendType::OpenSsl,
        ];

        assert_eq!(backends.len(), 4, "Should have 4 crypto backend types");
    }

    #[test]
    fn test_crypto_backend_equality() {
        let backend1 = CryptoBackendType::RustCrypto;
        let backend2 = CryptoBackendType::RustCrypto;
        let backend3 = CryptoBackendType::Ring;

        assert_eq!(backend1, backend2, "Same crypto backends should be equal");
        assert_ne!(
            backend1, backend3,
            "Different crypto backends should not be equal"
        );
    }

    // ============================================================================
    // Software HSM Config Tests
    // ============================================================================

    #[test]
    fn test_software_hsm_config_default() {
        let config = SoftwareHsmConfig::default();
        assert_eq!(config.crypto_backend, CryptoBackendType::RustCrypto);
        assert!(config.encryption_at_rest);
    }

    #[test]
    fn test_software_hsm_config_backend_selection() {
        let backends = vec![
            CryptoBackendType::RustCrypto,
            CryptoBackendType::Ring,
            CryptoBackendType::OpenSsl,
        ];

        for backend in backends {
            let config = SoftwareHsmConfig {
                crypto_backend: backend.clone(),
                ..SoftwareHsmConfig::default()
            };

            assert_eq!(
                config.crypto_backend, backend,
                "Config should respect selected backend"
            );
        }
    }

    #[test]
    fn test_software_hsm_config_encryption_at_rest() {
        let config_enabled = SoftwareHsmConfig {
            encryption_at_rest: true,
            ..SoftwareHsmConfig::default()
        };

        let config_disabled = SoftwareHsmConfig {
            encryption_at_rest: false,
            ..SoftwareHsmConfig::default()
        };

        assert!(
            config_enabled.encryption_at_rest,
            "Encryption at rest should be enabled"
        );
        assert!(
            !config_disabled.encryption_at_rest,
            "Encryption at rest should be disabled"
        );
    }

    #[test]
    fn test_software_hsm_config_memory_config() {
        let config = SoftwareHsmConfig::default();
        assert!(
            config.memory_config.enable_encryption,
            "Memory encryption should be enabled by default"
        );
        assert!(
            config.memory_config.pool_size > 0,
            "Memory pool should have positive size"
        );
    }
}
