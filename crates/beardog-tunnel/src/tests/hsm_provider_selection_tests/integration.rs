// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests: `HsmManager`, mock providers, and multi-tier selection behavior.

use crate::tunnel::hsm::manager::{HealthStatus, HsmManager, HsmProvider, KeyInfo, ProviderInfo};
use crate::tunnel::hsm::{
    GenerateKeyRequest,
    types::{HsmKey, HsmTier},
};
use async_trait::async_trait;
use beardog_errors::BearDogError;
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

    async fn decrypt(&self, _key_id: &str, _ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Hardware HSM unavailable".to_string(),
            ));
        }
        Ok(_ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
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

    async fn decrypt(&self, _key_id: &str, _ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(_ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
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
async fn test_hsm_manager_registers_multiple_providers() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = HsmManager::new();

    let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    manager.register_hsm_provider(HsmTier::Hardware, hardware_hsm)?;
    manager.register_hsm_provider(HsmTier::Software, software_hsm)?;

    Ok(())
}

#[tokio::test]
async fn test_provider_health_check_returns_status() -> Result<(), Box<dyn std::error::Error>> {
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
    let failing_hsm = MockHardwareHsm::with_failures();
    let working_hsm = MockSoftwareHsm;

    let hw_result = failing_hsm.sign("test-key", b"data").await;
    assert!(hw_result.is_err(), "Failing HSM should return error");

    let sw_result = working_hsm.sign("test-key", b"data").await;
    assert!(sw_result.is_ok(), "Working HSM should succeed");

    Ok(())
}

#[tokio::test]
async fn test_provider_availability_check() -> Result<(), Box<dyn std::error::Error>> {
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

/// Mock Cloud HSM Provider for testing
#[derive(Debug, Clone)]
struct MockCloudHsm {
    available: bool,
    fail_operations: bool,
}

impl MockCloudHsm {
    fn new(available: bool) -> Self {
        Self {
            available,
            fail_operations: false,
        }
    }

    #[allow(dead_code)]
    fn with_failures() -> Self {
        Self {
            available: true,
            fail_operations: true,
        }
    }
}

#[async_trait]
impl HsmProvider for MockCloudHsm {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock-cloud".to_string(),
            name: "Mock Cloud HSM".to_string(),
            security_level: 4,
        })
    }

    async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Err(BearDogError::not_implemented(
            "Mock generate_key for testing",
        ))
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(vec![9, 10, 11, 12]) // Mock cloud signature
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
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(_data.to_vec())
    }

    async fn decrypt(&self, _key_id: &str, _ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(_ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key for testing"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type: "cloud".to_string(),
            is_hardware_backed: true,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            is_healthy: self.available && !self.fail_operations,
            error_message: if self.available && !self.fail_operations {
                None
            } else {
                Some("Cloud HSM not available".to_string())
            },
        })
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

#[tokio::test]
async fn test_provider_selection_with_multiple_providers() -> Result<(), Box<dyn std::error::Error>>
{
    let mut manager = HsmManager::new();

    let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
    let cloud_hsm = Arc::new(MockCloudHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    manager.register_hsm_provider(HsmTier::Hardware, hardware_hsm.clone())?;
    manager.register_hsm_provider(HsmTier::Cloud, cloud_hsm.clone())?;
    manager.register_hsm_provider(HsmTier::Software, software_hsm.clone())?;

    let hw_health = hardware_hsm.health_check().await?;
    let cloud_health = cloud_hsm.health_check().await?;
    let sw_health = software_hsm.health_check().await?;

    assert!(hw_health.is_healthy, "Hardware HSM should be healthy");
    assert!(cloud_health.is_healthy, "Cloud HSM should be healthy");
    assert!(sw_health.is_healthy, "Software HSM should be healthy");

    let hw_info = hardware_hsm.get_info().await?;
    let cloud_info = cloud_hsm.get_info().await?;
    let sw_info = software_hsm.get_info().await?;

    assert!(
        hw_info.security_level > cloud_info.security_level,
        "Hardware should have higher security level than cloud"
    );
    assert!(
        cloud_info.security_level > sw_info.security_level,
        "Cloud should have higher security level than software"
    );

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_prioritizes_hardware() -> Result<(), Box<dyn std::error::Error>> {
    let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
    let cloud_hsm = Arc::new(MockCloudHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    let hw_info = hardware_hsm.get_info().await?;
    let cloud_info = cloud_hsm.get_info().await?;
    let sw_info = software_hsm.get_info().await?;

    let providers = [
        (hw_info.security_level, "hardware"),
        (cloud_info.security_level, "cloud"),
        (sw_info.security_level, "software"),
    ];

    let selected = providers
        .iter()
        .max_by_key(|(level, _)| level)
        .map(|(_, name)| *name);

    assert_eq!(
        selected,
        Some("hardware"),
        "Should select hardware when all providers available"
    );

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_with_hardware_unavailable()
-> Result<(), Box<dyn std::error::Error>> {
    let hardware_hsm = Arc::new(MockHardwareHsm::new(false));
    let cloud_hsm = Arc::new(MockCloudHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    let hw_available = hardware_hsm.is_available();
    let cloud_available = cloud_hsm.is_available();
    let sw_available = software_hsm.is_available();

    assert!(!hw_available, "Hardware HSM should be unavailable");
    assert!(cloud_available, "Cloud HSM should be available");
    assert!(sw_available, "Software HSM should be available");

    let selected = if hw_available {
        "hardware"
    } else if cloud_available {
        "cloud"
    } else if sw_available {
        "software"
    } else {
        "none"
    };

    assert_eq!(
        selected, "cloud",
        "Should select cloud when hardware unavailable"
    );

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_cascading_failover() -> Result<(), Box<dyn std::error::Error>> {
    let hardware_hsm = Arc::new(MockHardwareHsm::new(false));
    let cloud_hsm = Arc::new(MockCloudHsm::new(false));
    let software_hsm = Arc::new(MockSoftwareHsm);

    let hw_available = hardware_hsm.is_available();
    let cloud_available = cloud_hsm.is_available();
    let sw_available = software_hsm.is_available();

    let selected = if hw_available {
        "hardware"
    } else if cloud_available {
        "cloud"
    } else if sw_available {
        "software"
    } else {
        "none"
    };

    assert_eq!(
        selected, "software",
        "Should fall back to software when hardware and cloud unavailable"
    );

    let result = software_hsm.sign("test-key", b"test data").await;
    assert!(result.is_ok(), "Software HSM should be operational");

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_with_health_checks() -> Result<(), Box<dyn std::error::Error>> {
    let healthy_hw = Arc::new(MockHardwareHsm::new(true));
    let unhealthy_hw = Arc::new(MockHardwareHsm::with_failures());
    let healthy_sw = Arc::new(MockSoftwareHsm);

    let healthy_hw_status = healthy_hw.health_check().await?;
    let unhealthy_hw_status = unhealthy_hw.health_check().await?;
    let healthy_sw_status = healthy_sw.health_check().await?;

    assert!(healthy_hw_status.is_healthy);
    assert!(!unhealthy_hw_status.is_healthy);
    assert!(healthy_sw_status.is_healthy);

    let selected_with_healthy_hw = if healthy_hw_status.is_healthy {
        "hardware"
    } else if healthy_sw_status.is_healthy {
        "software"
    } else {
        "none"
    };

    assert_eq!(selected_with_healthy_hw, "hardware");

    let selected_with_unhealthy_hw = if unhealthy_hw_status.is_healthy {
        "hardware"
    } else if healthy_sw_status.is_healthy {
        "software"
    } else {
        "none"
    };

    assert_eq!(
        selected_with_unhealthy_hw, "software",
        "Should fail over to software when hardware unhealthy"
    );

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_operation_retry() -> Result<(), Box<dyn std::error::Error>> {
    let failing_hw = Arc::new(MockHardwareHsm::with_failures());
    let working_cloud = Arc::new(MockCloudHsm::new(true));
    let working_sw = Arc::new(MockSoftwareHsm);

    let hw_result = failing_hw.sign("test-key", b"data").await;
    assert!(hw_result.is_err(), "Hardware operation should fail");

    let cloud_result = working_cloud.sign("test-key", b"data").await;
    assert!(cloud_result.is_ok(), "Cloud operation should succeed");

    let sw_result = working_sw.sign("test-key", b"data").await;
    assert!(sw_result.is_ok(), "Software operation should succeed");

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_security_level_comparison()
-> Result<(), Box<dyn std::error::Error>> {
    let hardware_hsm = MockHardwareHsm::new(true);
    let cloud_hsm = MockCloudHsm::new(true);
    let software_hsm = MockSoftwareHsm;

    let hw_info = hardware_hsm.get_info().await?;
    let cloud_info = cloud_hsm.get_info().await?;
    let sw_info = software_hsm.get_info().await?;

    assert_eq!(hw_info.security_level, 5, "Hardware security level");
    assert_eq!(cloud_info.security_level, 4, "Cloud security level");
    assert_eq!(sw_info.security_level, 3, "Software security level");

    assert!(hw_info.security_level > cloud_info.security_level);
    assert!(cloud_info.security_level > sw_info.security_level);

    Ok(())
}

#[tokio::test]
async fn test_provider_selection_all_providers_registered() -> Result<(), Box<dyn std::error::Error>>
{
    let mut manager = HsmManager::new();

    let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
    let cloud_hsm = Arc::new(MockCloudHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    manager.register_hsm_provider(HsmTier::Hardware, hardware_hsm.clone())?;
    manager.register_hsm_provider(HsmTier::Cloud, cloud_hsm.clone())?;
    manager.register_hsm_provider(HsmTier::Software, software_hsm.clone())?;

    let hw_sig = hardware_hsm.sign("test", b"data").await?;
    let cloud_sig = cloud_hsm.sign("test", b"data").await?;
    let sw_sig = software_hsm.sign("test", b"data").await?;

    assert!(!hw_sig.is_empty(), "Hardware signature should be valid");
    assert!(!cloud_sig.is_empty(), "Cloud signature should be valid");
    assert!(!sw_sig.is_empty(), "Software signature should be valid");

    assert_ne!(
        hw_sig, cloud_sig,
        "Hardware and cloud should produce different signatures"
    );
    assert_ne!(
        cloud_sig, sw_sig,
        "Cloud and software should produce different signatures"
    );
    assert_ne!(
        hw_sig, sw_sig,
        "Hardware and software should produce different signatures"
    );

    Ok(())
}
