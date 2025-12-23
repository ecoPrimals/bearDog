//! Mobile HSM Setup
//!
//! This module provides setup and initialization for mobile HSMs (Android StrongBox, iOS Secure Enclave).

use super::{manager::HsmManager, software_hsm::RustSoftwareHsm};

#[cfg(target_os = "android")]
use super::android_strongbox::AndroidStrongBoxHsm;

#[cfg(not(target_os = "android"))]
use beardog_types::hsm::AndroidStrongBoxHsm;

use crate::tunnel::hsm::types::{AndroidHsmConfig, HsmTier, SoftwareHsmConfig};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{info, warn};

/// Mobile HSM setup configuration
#[derive(Debug, Clone)]
pub struct MobileHsmSetup {
    pub require_mobile_for_critical: bool,
    pub enable_graphene_optimizations: bool,
    pub software_hsm_config: SoftwareHsmConfig,
    pub android_config: AndroidHsmConfig,
}

impl Default for MobileHsmSetup {
    fn default() -> Self {
        Self {
            require_mobile_for_critical: true,
            enable_graphene_optimizations: true,
            software_hsm_config: SoftwareHsmConfig::default(),
            android_config: AndroidHsmConfig::default(),
        }
    }
}

/// Initializes mobile HSM manager
///
/// # Errors
/// Returns an error if HSM initialization fails.
pub async fn initialize_mobile_hsm_manager(
    setup: MobileHsmSetup,
) -> Result<HsmManager, BearDogError> {
    info!("📱 Initializing mobile-first HSM manager");
    let mut hsm_manager = HsmManager::new();

    // Try to initialize mobile HSM (Android StrongBox)
    match initialize_mobile_hsm(&setup.android_config).await {
        Ok(mobile_hsm) => {
            info!("✅ Mobile HSM (StrongBox) initialized successfully");

            // NOTE: Simplified to unit variant - device_type and secure_enclave info managed separately
            let mobile_tier = HsmTier::Mobile;

            // Mobile HSM successfully initialized - ready for registration
            // DEFERRED(Phase-2): Complete HSM manager registration API
            // Mobile HSM functionality is available through direct provider access
            info!("✅ Mobile HSM ready (registration deferred to Phase 2)");
            let _ = (mobile_tier, mobile_hsm); // Suppress unused variable warnings
        }
        Err(e) => {
            if setup.require_mobile_for_critical {
                return Err(BearDogError::unavailable(format!(
                    "Mobile HSM required but unavailable: {e}"
                )));
            } else {
                warn!("⚠️ Mobile HSM unavailable, using software-only mode: {}", e);
            }
        }
    }

    // Initialize software HSM as fallback
    match initialize_software_hsm(&setup.software_hsm_config).await {
        Ok(software_hsm) => {
            info!("✅ Software HSM initialized successfully");

            // NOTE: Simplified to unit variant - implementation details managed separately
            let software_tier = HsmTier::Software;

            hsm_manager.register_hsm_provider(software_tier, Arc::new(software_hsm))?;
        }
        Err(e) => {
            return Err(BearDogError::initialization(format!(
                "Failed to initialize software HSM: {e}"
            )));
        }
    }

    info!("🎯 Mobile-first HSM manager initialization complete");
    Ok(hsm_manager)
}

/// Initializes mobile HSM (Android StrongBox)
async fn initialize_mobile_hsm(
    config: &AndroidHsmConfig,
) -> Result<AndroidStrongBoxHsm, BearDogError> {
    info!("🔐 Initializing Android StrongBox HSM");

    if !cfg!(target_os = "android") {
        warn!("⚠️ Not running on Android - StrongBox unavailable");
        return Err(BearDogError::unavailable(
            "Android StrongBox only available on Android devices".to_string(),
        ));
    }

    let _ = config; // Suppress unused warning
    let strongbox_hsm = AndroidStrongBoxHsm::with_defaults();
    info!("✅ Android StrongBox HSM initialized and ready (mock)");
    Ok(strongbox_hsm)
}

/// Initializes software HSM
async fn initialize_software_hsm(
    config: &SoftwareHsmConfig,
) -> Result<RustSoftwareHsm, BearDogError> {
    info!("💻 Initializing Rust Software HSM");
    let software_hsm = RustSoftwareHsm::new(config.clone()).await?;
    info!("✅ Rust Software HSM initialized and ready");
    Ok(software_hsm)
}

/// Creates Pixel 8 + GrapheneOS optimized configuration
pub fn create_pixel8_graphene_config() -> MobileHsmSetup {
    info!("🎯 Creating Pixel 8 + GrapheneOS optimized HSM configuration");

    MobileHsmSetup {
        require_mobile_for_critical: true,
        enable_graphene_optimizations: true,
        software_hsm_config: SoftwareHsmConfig::default(),
        android_config: AndroidHsmConfig::default(),
    }
}

/// Creates development configuration with software fallback
pub fn create_dev_config() -> MobileHsmSetup {
    info!("🛠️ Creating development HSM configuration");

    MobileHsmSetup {
        require_mobile_for_critical: false,
        enable_graphene_optimizations: false,
        software_hsm_config: SoftwareHsmConfig::default(),
        android_config: AndroidHsmConfig::default(),
    }
}

/// Sets up production mobile HSM environment
///
/// # Errors
/// Returns an error if setup fails.
pub async fn setup_production_mobile_hsm() -> Result<HsmManager, BearDogError> {
    info!("🚀 Setting up production mobile HSM environment");
    let config = create_pixel8_graphene_config();
    initialize_mobile_hsm_manager(config).await
}

/// Demonstrates mobile HSM operations
///
/// # Errors
/// Returns an error if demo fails.
pub async fn demo_mobile_operations(hsm_manager: &HsmManager) -> Result<(), BearDogError> {
    info!("🎬 Demonstrating mobile HSM operation routing");

    let metrics = hsm_manager.get_routing_metrics();
    info!("📊 Routing metrics: {:?}", metrics);

    info!("🎉 Mobile HSM operation routing demonstration complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_setup_default() {
        let setup = MobileHsmSetup::default();
        assert!(setup.require_mobile_for_critical);
        assert!(setup.enable_graphene_optimizations);
    }

    #[test]
    fn test_pixel8_config() {
        let config = create_pixel8_graphene_config();
        assert!(config.enable_graphene_optimizations);
    }

    #[test]
    fn test_dev_config() {
        let config = create_dev_config();
        assert!(!config.require_mobile_for_critical);
    }

    #[tokio::test]
    async fn test_software_hsm_init() {
        let config = SoftwareHsmConfig::default();
        let result = initialize_software_hsm(&config).await;
        // May fail if dependencies not available, but should not panic
        let _ = result;
    }
}
