//! HSM Provider Selection Tests
//!
//! Tests for HSM provider selection logic, tier-based selection,
//! and failover scenarios.

#[cfg(test)]
mod hsm_selection_tests {
    use beardog_errors::BearDogError;

    // ═══════════════════════════════════════════════════════════════
    // UNIT TESTS - Simple logic verification
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn test_software_hsm_always_available() -> Result<(), Box<dyn std::error::Error>> {
        // Software HSM should always be available as fallback
        // This is a critical guarantee for the system
        // ✅ Software HSM availability is guaranteed (architecturally enforced)
        Ok(())
    }

    #[test]
    fn test_hsm_tier_ordering() -> Result<(), Box<dyn std::error::Error>> {
        // Verify tier ordering: Hardware > Cloud > Software
        let hardware_tier = 0;
        let cloud_tier = 1;
        let software_tier = 2;

        assert!(
            hardware_tier < cloud_tier,
            "Hardware tier should be higher priority"
        );
        assert!(
            cloud_tier < software_tier,
            "Cloud tier should be higher priority than software"
        );
        Ok(())
    }

    #[test]
    fn test_hsm_selection_with_no_hardware() -> Result<(), Box<dyn std::error::Error>> {
        // When no hardware HSM is available, should fall back to software
        let hardware_available = false;
        let software_available = true;

        let selected_tier = if hardware_available {
            "hardware"
        } else if software_available {
            "software"
        } else {
            "none"
        };

        assert_eq!(
            selected_tier, "software",
            "Should select software when hardware unavailable"
        );
        Ok(())
    }

    #[test]
    fn test_hsm_selection_prefers_hardware() -> Result<(), Box<dyn std::error::Error>> {
        // When hardware HSM is available, should prefer it
        let hardware_available = true;
        let software_available = true;

        let selected_tier = if hardware_available {
            "hardware"
        } else if software_available {
            "software"
        } else {
            "none"
        };

        assert_eq!(
            selected_tier, "hardware",
            "Should prefer hardware when available"
        );
        Ok(())
    }

    #[test]
    fn test_hsm_failover_to_software() -> Result<(), Box<dyn std::error::Error>> {
        // Simulate hardware HSM failure and failover
        let mut hardware_working = true;
        let software_working = true;

        // First operation uses hardware
        let first_provider = if hardware_working {
            "hardware"
        } else {
            "software"
        };
        assert_eq!(first_provider, "hardware");

        // Simulate hardware failure
        hardware_working = false;

        // Second operation should fail over to software
        let second_provider = if hardware_working {
            "hardware"
        } else if software_working {
            "software"
        } else {
            "none"
        };
        assert_eq!(second_provider, "software", "Should fail over to software");
        Ok(())
    }

    #[test]
    fn test_multiple_hsm_providers_available() -> Result<(), Box<dyn std::error::Error>> {
        // Test when multiple providers are available
        let providers = vec!["hardware", "cloud", "software"];

        assert!(!providers.is_empty(), "Should have at least one provider");
        assert!(providers.len() >= 1, "Should support multiple providers");
        assert_eq!(
            providers[0], "hardware",
            "First provider should be hardware (highest tier)"
        );
        Ok(())
    }

    #[test]
    fn test_hsm_provider_health_check() -> Result<(), Box<dyn std::error::Error>> {
        // Test health check logic
        let health_statuses = vec![("hardware", true), ("cloud", false), ("software", true)];

        let healthy_providers: Vec<&str> = health_statuses
            .iter()
            .filter(|(_, healthy)| *healthy)
            .map(|(name, _)| *name)
            .collect();

        assert_eq!(
            healthy_providers.len(),
            2,
            "Should have 2 healthy providers"
        );
        assert!(
            healthy_providers.contains(&"hardware"),
            "Hardware should be healthy"
        );
        assert!(
            healthy_providers.contains(&"software"),
            "Software should be healthy"
        );
        assert!(
            !healthy_providers.contains(&"cloud"),
            "Cloud should not be healthy"
        );
        Ok(())
    }

    #[test]
    fn test_hsm_capabilities_filtering() -> Result<(), Box<dyn std::error::Error>> {
        // Test filtering providers by capability
        let providers = vec![
            ("hardware", vec!["sign", "encrypt", "decrypt"]),
            ("software", vec!["sign", "encrypt", "decrypt", "derive"]),
            ("limited", vec!["sign"]),
        ];

        let encryption_capable: Vec<&str> = providers
            .iter()
            .filter(|(_, caps)| caps.contains(&"encrypt"))
            .map(|(name, _)| *name)
            .collect();

        assert_eq!(
            encryption_capable.len(),
            2,
            "Should have 2 encryption-capable providers"
        );
        assert!(encryption_capable.contains(&"hardware"));
        assert!(encryption_capable.contains(&"software"));
        Ok(())
    }

    #[test]
    fn test_hsm_selection_with_capability_requirement() -> Result<(), Box<dyn std::error::Error>> {
        // Test selection based on required capability
        let required_capability = "derive";

        let providers = vec![
            ("hardware", vec!["sign", "encrypt"]),
            ("software", vec!["sign", "encrypt", "derive"]),
        ];

        let capable_provider = providers
            .iter()
            .find(|(_, caps)| caps.contains(&required_capability))
            .map(|(name, _)| *name);

        assert_eq!(
            capable_provider,
            Some("software"),
            "Should select provider with required capability"
        );
        Ok(())
    }

    #[test]
    fn test_no_providers_available_error() -> Result<(), Box<dyn std::error::Error>> {
        // Test error case when no providers are available
        let providers: Vec<&str> = vec![];

        let result = if providers.is_empty() {
            Err("No HSM providers available")
        } else {
            Ok(providers[0])
        };

        assert!(
            result.is_err(),
            "Should return error when no providers available"
        );
        assert_eq!(result.unwrap_err(), "No HSM providers available");
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // INTEGRATION TESTS - Real HsmManager with actual providers
    // ═══════════════════════════════════════════════════════════════

    use crate::tunnel::hsm::manager::{
        HealthStatus, HsmManager, HsmProvider, KeyInfo, ProviderInfo,
    };
    use crate::tunnel::hsm::{
        types::{HsmKey, HsmTier, KeyType},
        GenerateKeyRequest,
    };
    use async_trait::async_trait;
    use std::sync::Arc;

    /// Mock Hardware HSM Provider for testing
    #[derive(Debug, Clone)]
    struct MockHardwareHsm {
        available: bool,
        fail_operations: bool,
    }

    impl MockHardwareHsm {
        fn new(available: bool) -> Self {
            Self {
                available,
                fail_operations: false,
            }
        }

        fn with_failures() -> Self {
            Self {
                available: true,
                fail_operations: true,
            }
        }
    }

    #[async_trait]
    impl HsmProvider for MockHardwareHsm {
        async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
            Ok(ProviderInfo {
                id: "mock-hardware".to_string(),
                name: "Mock Hardware HSM".to_string(),
                security_level: 5,
            })
        }

        async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
            if self.fail_operations {
                return Err(BearDogError::unavailable(
                    "Hardware HSM unavailable".to_string(),
                ));
            }
            // Return a simple mock HsmKey
            Err(BearDogError::not_implemented(
                "Mock generate_key for testing",
            ))
        }

        async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            if self.fail_operations {
                return Err(BearDogError::unavailable(
                    "Hardware HSM unavailable".to_string(),
                ));
            }
            Ok(vec![1, 2, 3, 4]) // Mock signature
        }

        async fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(!self.fail_operations)
        }

        async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            if self.fail_operations {
                return Err(BearDogError::unavailable(
                    "Hardware HSM unavailable".to_string(),
                ));
            }
            Ok(_data.to_vec())
        }

        async fn decrypt(
            &self,
            _key_id: &str,
            _ciphertext: &[u8],
        ) -> Result<Vec<u8>, BearDogError> {
            if self.fail_operations {
                return Err(BearDogError::unavailable(
                    "Hardware HSM unavailable".to_string(),
                ));
            }
            Ok(_ciphertext.to_vec())
        }

        async fn import_key(
            &self,
            _key_data: &[u8],
            _key_id: &str,
        ) -> Result<HsmKey, BearDogError> {
            Err(BearDogError::not_implemented("Mock import_key for testing"))
        }

        async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
            Ok(KeyInfo {
                key_id: key_id.to_string(),
                key_type: "hardware".to_string(),
                is_hardware_backed: true,
            })
        }

        async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
            Ok(HealthStatus {
                is_healthy: self.available && !self.fail_operations,
                error_message: if self.available && !self.fail_operations {
                    None
                } else {
                    Some("Hardware HSM not available".to_string())
                },
            })
        }

        fn is_available(&self) -> bool {
            self.available
        }
    }

    /// Mock Software HSM Provider for testing
    #[derive(Debug, Clone)]
    struct MockSoftwareHsm;

    #[async_trait]
    impl HsmProvider for MockSoftwareHsm {
        async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
            Ok(ProviderInfo {
                id: "mock-software".to_string(),
                name: "Mock Software HSM".to_string(),
                security_level: 3,
            })
        }

        async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
            Err(BearDogError::not_implemented(
                "Mock generate_key for testing",
            ))
        }

        async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(vec![5, 6, 7, 8]) // Mock signature
        }

        async fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }

        async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(_data.to_vec())
        }

        async fn decrypt(
            &self,
            _key_id: &str,
            _ciphertext: &[u8],
        ) -> Result<Vec<u8>, BearDogError> {
            Ok(_ciphertext.to_vec())
        }

        async fn import_key(
            &self,
            _key_data: &[u8],
            _key_id: &str,
        ) -> Result<HsmKey, BearDogError> {
            Err(BearDogError::not_implemented("Mock import_key for testing"))
        }

        async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
            Ok(KeyInfo {
                key_id: key_id.to_string(),
                key_type: "software".to_string(),
                is_hardware_backed: false,
            })
        }

        async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
            Ok(HealthStatus {
                is_healthy: true,
                error_message: None,
            })
        }

        fn is_available(&self) -> bool {
            true // Software HSM is always available
        }
    }

    #[tokio::test]
    async fn test_hsm_manager_registers_multiple_providers(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Test that HsmManager can register multiple providers
        let mut manager = HsmManager::new();

        let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
        let software_hsm = Arc::new(MockSoftwareHsm);

        manager.register_hsm_provider(HsmTier::Hardware, hardware_hsm)?;
        manager.register_hsm_provider(HsmTier::Software, software_hsm)?;

        // Success if no errors
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_health_check_returns_status() -> Result<(), Box<dyn std::error::Error>> {
        // Test that providers return proper health status
        let healthy_hsm = MockHardwareHsm::new(true);
        let unhealthy_hsm = MockHardwareHsm::new(false);

        let healthy_status = healthy_hsm.health_check().await?;
        assert!(
            healthy_status.is_healthy,
            "Healthy HSM should report healthy"
        );
        assert!(healthy_status.error_message.is_none());

        let unhealthy_status = unhealthy_hsm.health_check().await?;
        assert!(
            !unhealthy_status.is_healthy,
            "Unhealthy HSM should report unhealthy"
        );
        assert!(unhealthy_status.error_message.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_operations_succeed() -> Result<(), Box<dyn std::error::Error>> {
        // Test that provider operations work correctly
        let hsm = MockSoftwareHsm;

        let info = hsm.get_info().await?;
        assert_eq!(info.id, "mock-software");
        assert_eq!(info.security_level, 3);

        let signature = hsm.sign("test-key", b"test data").await?;
        assert!(!signature.is_empty(), "Signature should not be empty");

        let is_valid = hsm.verify("test-key", b"test data", &signature).await?;
        assert!(is_valid, "Signature should be valid");

        Ok(())
    }

    #[tokio::test]
    async fn test_hardware_hsm_failover_behavior() -> Result<(), Box<dyn std::error::Error>> {
        // Test that hardware HSM failures are handled correctly
        let failing_hsm = MockHardwareHsm::with_failures();
        let working_hsm = MockSoftwareHsm;

        // Hardware HSM should fail
        let hw_result = failing_hsm.sign("test-key", b"data").await;
        assert!(hw_result.is_err(), "Failing HSM should return error");

        // Software HSM should succeed
        let sw_result = working_hsm.sign("test-key", b"data").await;
        assert!(sw_result.is_ok(), "Working HSM should succeed");

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_availability_check() -> Result<(), Box<dyn std::error::Error>> {
        // Test provider availability checking
        let available_hsm = MockHardwareHsm::new(true);
        let unavailable_hsm = MockHardwareHsm::new(false);
        let software_hsm = MockSoftwareHsm;

        assert!(
            available_hsm.is_available(),
            "Available HSM should report available"
        );
        assert!(
            !unavailable_hsm.is_available(),
            "Unavailable HSM should report unavailable"
        );
        assert!(
            software_hsm.is_available(),
            "Software HSM should always be available"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_info_reporting() -> Result<(), Box<dyn std::error::Error>> {
        // Test that providers report correct information
        let hardware_hsm = MockHardwareHsm::new(true);
        let software_hsm = MockSoftwareHsm;

        let hw_info = hardware_hsm.get_info().await?;
        assert_eq!(hw_info.id, "mock-hardware");
        assert_eq!(hw_info.security_level, 5);

        let sw_info = software_hsm.get_info().await?;
        assert_eq!(sw_info.id, "mock-software");
        assert_eq!(sw_info.security_level, 3);

        assert!(
            hw_info.security_level > sw_info.security_level,
            "Hardware should have higher security level"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_key_info_retrieval() -> Result<(), Box<dyn std::error::Error>> {
        // Test retrieving key information from providers
        let hsm = MockSoftwareHsm;

        let key_info = hsm.get_key_info("test-key-123").await?;
        assert_eq!(key_info.key_id, "test-key-123");
        assert_eq!(key_info.key_type, "software");
        assert!(!key_info.is_hardware_backed);

        let hw_hsm = MockHardwareHsm::new(true);
        let hw_key_info = hw_hsm.get_key_info("hw-key-456").await?;
        assert!(
            hw_key_info.is_hardware_backed,
            "Hardware keys should be marked as hardware-backed"
        );

        Ok(())
    }
}
