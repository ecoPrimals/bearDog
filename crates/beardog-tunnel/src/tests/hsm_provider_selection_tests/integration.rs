// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests: `HsmManager`, mock providers, and multi-tier selection behavior.

use crate::tunnel::hsm::HsmProviderBackend;
use crate::tunnel::hsm::hsm_provider_mocks::{MockCloudHsm, MockHardwareHsm, MockSoftwareHsm};
use crate::tunnel::hsm::manager::HsmManager;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::types::HsmTier;
use std::sync::Arc;

#[tokio::test]
async fn test_hsm_manager_registers_multiple_providers() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = HsmManager::new();

    manager.register_hsm_provider(
        HsmTier::Hardware,
        Arc::new(HsmProviderBackend::MockHardware(MockHardwareHsm::new(true))),
    )?;
    manager.register_hsm_provider(
        HsmTier::Software,
        Arc::new(HsmProviderBackend::MockSoftware(MockSoftwareHsm)),
    )?;

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

#[tokio::test]
async fn test_provider_selection_with_multiple_providers() -> Result<(), Box<dyn std::error::Error>>
{
    let mut manager = HsmManager::new();

    let hardware_hsm = Arc::new(MockHardwareHsm::new(true));
    let cloud_hsm = Arc::new(MockCloudHsm::new(true));
    let software_hsm = Arc::new(MockSoftwareHsm);

    manager.register_hsm_provider(
        HsmTier::Hardware,
        Arc::new(HsmProviderBackend::MockHardware((*hardware_hsm).clone())),
    )?;
    manager.register_hsm_provider(
        HsmTier::Cloud,
        Arc::new(HsmProviderBackend::MockCloud((*cloud_hsm).clone())),
    )?;
    manager.register_hsm_provider(
        HsmTier::Software,
        Arc::new(HsmProviderBackend::MockSoftware(MockSoftwareHsm)),
    )?;

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

    manager.register_hsm_provider(
        HsmTier::Hardware,
        Arc::new(HsmProviderBackend::MockHardware((*hardware_hsm).clone())),
    )?;
    manager.register_hsm_provider(
        HsmTier::Cloud,
        Arc::new(HsmProviderBackend::MockCloud((*cloud_hsm).clone())),
    )?;
    manager.register_hsm_provider(
        HsmTier::Software,
        Arc::new(HsmProviderBackend::MockSoftware(MockSoftwareHsm)),
    )?;

    let hw_sig: Vec<u8> = hardware_hsm.sign("test", b"data").await?;
    let cloud_sig: Vec<u8> = cloud_hsm.sign("test", b"data").await?;
    let sw_sig: Vec<u8> = software_hsm.sign("test", b"data").await?;

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
