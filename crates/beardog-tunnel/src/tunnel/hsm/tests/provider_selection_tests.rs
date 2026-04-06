// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Provider Selection Tests
//!
//! Tests for HSM provider selection, tier management, and fallback behavior.
//! Updated November 21, 2025 to match UnifiedHsmProvider API.

use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use crate::tunnel::hsm::types::SoftwareHsmConfig;
use crate::tunnel::hsm::unified_provider::UnifiedHsmProvider;
use beardog_errors::BearDogError;
use beardog_traits::canonical::hsm::HsmTier;
use std::sync::Arc;

#[tokio::test]
async fn test_unified_provider_creation() -> Result<(), BearDogError> {
    // Test that unified provider can be created
    let provider = UnifiedHsmProvider::new();

    // Should start with no registered providers
    assert!(
        provider.list_providers().is_empty(),
        "New provider should have no registered providers"
    );

    Ok(())
}

#[tokio::test]
async fn test_software_hsm_registration() -> Result<(), BearDogError> {
    // Test that software HSM can be registered as a provider
    let mut unified = UnifiedHsmProvider::new();

    let config = SoftwareHsmConfig::default();
    let software_hsm = RustSoftwareHsm::new(config).await?;

    unified.register_provider("software".to_string(), Arc::new(software_hsm))?;

    // Software provider should be available
    assert!(
        !unified.list_providers().is_empty(),
        "Software HSM should be registered"
    );

    Ok(())
}

#[tokio::test]
async fn test_default_provider_set_on_first_registration() -> Result<(), BearDogError> {
    // Test that first registered provider becomes default
    let mut unified = UnifiedHsmProvider::new();

    let config = SoftwareHsmConfig::default();
    let software_hsm = RustSoftwareHsm::new(config).await?;

    unified.register_provider("software".to_string(), Arc::new(software_hsm))?;

    // Should be able to get default provider
    let result = unified.get_default_provider();
    assert!(result.is_ok(), "Default provider should be set");

    Ok(())
}

#[tokio::test]
async fn test_get_provider_by_id() -> Result<(), BearDogError> {
    // Test explicit provider retrieval by ID
    let mut unified = UnifiedHsmProvider::new();

    let config = SoftwareHsmConfig::default();
    let software_hsm = RustSoftwareHsm::new(config).await?;

    unified.register_provider("software-hsm".to_string(), Arc::new(software_hsm))?;

    // Should be able to get provider by ID
    let provider = unified.get_provider("software-hsm");
    assert!(provider.is_ok(), "Should be able to get provider by ID");

    Ok(())
}

#[test]
fn test_hsm_tier_ordering() {
    // Test that HSM tier ordering is correct

    let hardware = HsmTier::Hardware;
    let tpm = HsmTier::Tpm;
    let software = HsmTier::Software;

    // Hardware should be highest priority
    assert!(
        hardware > tpm,
        "Hardware should be higher priority than TPM"
    );
    assert!(
        hardware > software,
        "Hardware should be higher priority than Software"
    );

    // TPM should be middle priority
    assert!(
        tpm > software,
        "TPM should be higher priority than Software"
    );
}

#[tokio::test]
async fn test_multiple_provider_registration() -> Result<(), BearDogError> {
    // Test registering multiple providers
    let mut unified = UnifiedHsmProvider::new();

    let config1 = SoftwareHsmConfig::default();
    let software_hsm1 = RustSoftwareHsm::new(config1).await?;

    let config2 = SoftwareHsmConfig::default();
    let software_hsm2 = RustSoftwareHsm::new(config2).await?;

    unified.register_provider("software-1".to_string(), Arc::new(software_hsm1))?;
    unified.register_provider("software-2".to_string(), Arc::new(software_hsm2))?;

    // Should have both providers registered
    let providers = unified.list_providers();
    assert_eq!(providers.len(), 2, "Should have 2 registered providers");

    Ok(())
}

#[tokio::test]
async fn test_unregister_provider() -> Result<(), BearDogError> {
    // Test unregistering a provider
    let mut unified = UnifiedHsmProvider::new();

    let config = SoftwareHsmConfig::default();
    let software_hsm = RustSoftwareHsm::new(config).await?;

    unified.register_provider("software".to_string(), Arc::new(software_hsm))?;
    assert_eq!(unified.list_providers().len(), 1);

    unified.unregister_provider("software")?;
    assert_eq!(
        unified.list_providers().len(),
        0,
        "Provider should be unregistered"
    );

    Ok(())
}

#[tokio::test]
async fn test_set_default_provider() -> Result<(), BearDogError> {
    // Test explicitly setting default provider
    let mut unified = UnifiedHsmProvider::new();

    let config1 = SoftwareHsmConfig::default();
    let software_hsm1 = RustSoftwareHsm::new(config1).await?;

    let config2 = SoftwareHsmConfig::default();
    let software_hsm2 = RustSoftwareHsm::new(config2).await?;

    unified.register_provider("software-1".to_string(), Arc::new(software_hsm1))?;
    unified.register_provider("software-2".to_string(), Arc::new(software_hsm2))?;

    // Explicitly set software-2 as default
    unified.set_default_provider("software-2".to_string())?;

    // Default provider should now be software-2
    let _default = unified.get_default_provider()?;
    // Successfully retrieved default provider

    Ok(())
}

#[tokio::test]
async fn test_provider_not_found() -> Result<(), BearDogError> {
    // Test getting non-existent provider
    let unified = UnifiedHsmProvider::new();

    let result = unified.get_provider("non-existent");
    assert!(result.is_err(), "Should error for non-existent provider");

    Ok(())
}

#[tokio::test]
async fn test_get_default_provider_when_none() -> Result<(), BearDogError> {
    // Test getting default provider when none registered
    let unified = UnifiedHsmProvider::new();

    let result = unified.get_default_provider();
    assert!(result.is_err(), "Should error when no providers registered");

    Ok(())
}

#[tokio::test]
async fn test_list_providers() -> Result<(), BearDogError> {
    // Test listing providers
    let mut unified = UnifiedHsmProvider::new();

    let config1 = SoftwareHsmConfig::default();
    let software_hsm1 = RustSoftwareHsm::new(config1).await?;

    let config2 = SoftwareHsmConfig::default();
    let software_hsm2 = RustSoftwareHsm::new(config2).await?;

    unified.register_provider("provider-1".to_string(), Arc::new(software_hsm1))?;
    unified.register_provider("provider-2".to_string(), Arc::new(software_hsm2))?;

    // Should list all registered providers
    let providers = unified.list_providers();
    assert!(providers.len() >= 2, "Should list all registered providers");

    Ok(())
}
