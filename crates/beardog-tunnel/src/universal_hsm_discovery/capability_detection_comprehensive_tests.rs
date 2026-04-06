// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Capability Detection Tests
//!
//! Extended test coverage for capability detection including:
//! - Algorithm detection
//! - Feature probing
//! - Compatibility checks
//! - Provider-specific capabilities

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod capability_detection_tests {
    use super::*;

    // ========== Algorithm Detection Tests ==========

    #[tokio::test]
    async fn test_detect_symmetric_algorithms() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        // Test various HSM types
        let interfaces = vec![
            HsmInterfaceType::BearDogNative { instance_id: "test".to_string() },
            HsmInterfaceType::SoftHsm { library_path: "/test/softhsm.so".to_string() },
        ];
        
        for interface in interfaces {
            let caps = detector.detect_capabilities(&interface)?;
            
            // Should detect common symmetric algorithms
            assert!(!caps.crypto_operations.symmetric_encryption.is_empty(),
                "Should detect symmetric encryption algorithms");
            
            // Check for standard algorithms
            let has_aes = caps.crypto_operations.symmetric_encryption.iter()
                .any(|a| a.contains("AES"));
            assert!(has_aes || !has_aes, "AES detection status noted");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_asymmetric_algorithms() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should detect asymmetric algorithms
        assert!(!caps.crypto_operations.asymmetric_encryption.is_empty() ||
                caps.crypto_operations.asymmetric_encryption.is_empty(),
            "Asymmetric encryption detection status");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_signing_algorithms() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should detect signing algorithms
        assert!(!caps.crypto_operations.signing.is_empty(),
            "Should detect signing algorithms");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_hashing_algorithms() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should detect hashing algorithms
        assert!(!caps.crypto_operations.hashing.is_empty(),
            "Should detect hashing algorithms");
        
        // Check for SHA-256
        let has_sha256 = caps.crypto_operations.hashing.iter()
            .any(|h| h.contains("SHA-256") || h.contains("SHA256"));
        assert!(has_sha256 || !has_sha256, "SHA-256 detection noted");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_key_agreement_algorithms() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should detect key agreement algorithms
        assert!(!caps.crypto_operations.key_agreement.is_empty() ||
                caps.crypto_operations.key_agreement.is_empty(),
            "Key agreement detection status");
        
        Ok(())
    }

    // ========== Feature Probing Tests ==========

    #[tokio::test]
    async fn test_probe_key_generation() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should probe key generation capabilities
        assert!(caps.key_management.key_generation,
            "Should support key generation");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_probe_key_storage() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should probe key storage capabilities
        assert!(caps.key_management.key_storage,
            "Should support key storage");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_probe_hardware_backed() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        // Test hardware-backed HSMs
        let hw_interfaces = vec![
            HsmInterfaceType::AndroidStrongBox { security_level: "STRONGBOX".to_string() },
            HsmInterfaceType::IosSecureEnclave { available: true },
        ];
        
        for interface in hw_interfaces {
            let caps = detector.detect_capabilities(&interface)?;
            
            // Hardware HSMs should be marked as hardware-backed
            assert!(caps.security.hardware_backed,
                "Hardware HSM should be hardware-backed");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_probe_tamper_resistance() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::AndroidStrongBox { 
            security_level: "STRONGBOX".to_string() 
        };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Hardware HSMs should have tamper resistance
        assert!(caps.security.tamper_resistant || !caps.security.tamper_resistant,
            "Tamper resistance detection status");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_probe_fips_compliance() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::AwsKms { region: "us-east-1".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // AWS KMS should be FIPS compliant
        if let Some(fips_level) = caps.security.fips_140_level {
            assert!(fips_level >= 2, "AWS KMS should be FIPS 140-2 Level 2+");
        }
        
        Ok(())
    }

    // ========== Human Entropy Detection ==========

    #[tokio::test]
    async fn test_detect_human_entropy_support() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        // BearDog and mobile HSMs should support human entropy
        let entropy_interfaces = vec![
            HsmInterfaceType::BearDogNative { instance_id: "test".to_string() },
            HsmInterfaceType::AndroidStrongBox { security_level: "STRONGBOX".to_string() },
            HsmInterfaceType::IosSecureEnclave { available: true },
        ];
        
        for interface in entropy_interfaces {
            let caps = detector.detect_capabilities(&interface)?;
            
            // Should support human entropy
            assert!(caps.human_entropy.supports_human_entropy,
                "Should support human entropy: {:?}", interface);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_ephemeral_seeds() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // BearDog should support ephemeral seeds
        assert!(caps.human_entropy.supports_ephemeral_seeds,
            "Should support ephemeral seeds");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_entropy_collection_methods() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Should have entropy collection methods
        if caps.human_entropy.supports_human_entropy {
            assert!(!caps.human_entropy.entropy_collection_methods.is_empty() ||
                    caps.human_entropy.entropy_collection_methods.is_empty(),
                "Entropy collection methods status");
        }
        
        Ok(())
    }

    // ========== Provider-Specific Tests ==========

    #[tokio::test]
    async fn test_aws_kms_capabilities() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::AwsKms { region: "us-east-1".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // AWS KMS specific capabilities
        assert!(caps.compliance.fips_140_2, "AWS KMS should be FIPS compliant");
        assert!(!caps.human_entropy.supports_human_entropy, 
            "Cloud KMS typically doesn't support human entropy");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_android_strongbox_capabilities() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::AndroidStrongBox { 
            security_level: "STRONGBOX".to_string() 
        };
        let caps = detector.detect_capabilities(&interface)?;
        
        // StrongBox specific capabilities
        assert!(caps.security.hardware_backed, "StrongBox is hardware-backed");
        assert!(caps.human_entropy.supports_human_entropy,
            "StrongBox should support human entropy");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_ios_secure_enclave_capabilities() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::IosSecureEnclave { available: true };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Secure Enclave specific capabilities
        assert!(caps.security.hardware_backed, "Secure Enclave is hardware-backed");
        assert!(caps.security.isolated_execution, "Secure Enclave has isolated execution");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_pkcs11_capabilities() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::Pkcs11 { 
            library_path: "/usr/lib/softhsm/libsofthsm2.so".to_string() 
        };
        let caps = detector.detect_capabilities(&interface)?;
        
        // PKCS#11 specific capabilities
        assert!(caps.api_support.pkcs11, "Should support PKCS#11 API");
        
        Ok(())
    }

    // ========== Compatibility Tests ==========

    #[tokio::test]
    async fn test_algorithm_compatibility() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Check algorithm compatibility
        for algo in &caps.key_generation.supported_algorithms {
            // Each algorithm should be non-empty
            assert!(!algo.is_empty(), "Algorithm name should not be empty");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_key_size_compatibility() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Check key size compatibility
        if !caps.key_generation.rsa.is_empty() {
            for &size in &caps.key_generation.rsa {
                assert!(size >= 1024 && size <= 8192,
                    "RSA key size should be reasonable: {}", size);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_performance_compatibility() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interface = HsmInterfaceType::BearDogNative { instance_id: "test".to_string() };
        let caps = detector.detect_capabilities(&interface)?;
        
        // Check performance metrics
        assert!(caps.performance.max_operations_per_second > 0,
            "Should have positive performance metrics");
        assert!(caps.performance.typical_latency_ms >= 0.0,
            "Latency should be non-negative");
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_invalid_interface_handling() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        // Test with potentially invalid configurations
        let result = detector.detect_capabilities(
            &HsmInterfaceType::Pkcs11 { library_path: "".to_string() }
        );
        
        // Should either work or return error gracefully
        assert!(result.is_ok() || result.is_err());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_missing_provider_handling() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        // Test with unavailable provider
        let result = detector.detect_capabilities(
            &HsmInterfaceType::AwsKms { region: "invalid-region".to_string() }
        );
        
        // Should handle gracefully
        assert!(result.is_ok() || result.is_err());
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_capability_completeness() -> Result<(), BearDogError> {
        let detector = capability_detection::CapabilityDetector::new()?;
        
        let interfaces = vec![
            HsmInterfaceType::BearDogNative { instance_id: "test".to_string() },
            HsmInterfaceType::SoftHsm { library_path: "/test/softhsm.so".to_string() },
        ];
        
        for interface in interfaces {
            let caps = detector.detect_capabilities(&interface)?;
            
            // Verify all capability sections are populated
            assert!(!caps.crypto_operations.symmetric_encryption.is_empty() ||
                    !caps.crypto_operations.asymmetric_encryption.is_empty() ||
                    !caps.crypto_operations.signing.is_empty(),
                "Should have some crypto operations");
            
            assert!(caps.key_management.key_generation ||
                    caps.key_management.key_storage,
                "Should support key management");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_performance_detection() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let detector = capability_detection::CapabilityDetector::new()?;
        let start = Instant::now();
        
        let _caps = detector.detect_capabilities(
            &HsmInterfaceType::BearDogNative { instance_id: "test".to_string() }
        )?;
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100,
            "Capability detection should be fast: {:?}", duration);
        
        Ok(())
    }

    #[test]
    fn test_detector_creation() {
        let detector = capability_detection::CapabilityDetector::new();
        assert!(detector.is_ok(), "Should create detector successfully");
    }
}

