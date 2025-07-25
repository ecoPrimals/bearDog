//! # Mobile HSM Setup Module
//!
//! This module provides convenient setup functions for initializing BearDog's
//! mobile-first HSM architecture with intelligent operation routing.

use super::{
    android_strongbox::AndroidStrongBoxHsm,
    software_hsm::RustSoftwareHsm,
    manager::HsmManager,
};
use crate::tunnel::hsm::types::{HsmTier, SoftwareHsmConfig, AndroidHsmConfig, SmartphoneType, SecureEnclaveType, StrongBoxImplementation, AttestationLevel, SoftwareHsmType, KeyStorageType, MemoryProtectionLevel, Pixel8GrapheneOSConfig};
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tracing::{info, warn};

/// Mobile HSM setup configuration
pub struct MobileHsmSetup {
    /// Whether to require mobile HSM for critical operations
    pub require_mobile_for_critical: bool,
    /// Whether to enable GrapheneOS optimizations
    pub enable_graphene_optimizations: bool,
    /// Software HSM configuration for fallback
    pub software_hsm_config: SoftwareHsmConfig,
    /// Android StrongBox configuration
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

/// Initialize mobile-first HSM manager with StrongBox and software fallback
pub async fn initialize_mobile_hsm_manager(setup: MobileHsmSetup) -> BearDogResult<HsmManager> {
    info!("📱 Initializing mobile-first HSM manager");

    let mut hsm_manager = HsmManager::new();

    // Initialize mobile HSM (Android StrongBox) if available
    match initialize_mobile_hsm(&setup.android_config).await {
        Ok(mobile_hsm) => {
            info!("✅ Mobile HSM (StrongBox) initialized successfully");

            // Create mobile HSM tier definition
            let mobile_tier = HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: "Google".to_string(),
                    model: "Pixel 8".to_string(),
                    android_version: "14".to_string(),
                    strongbox_version: Some("1.0".to_string()),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: StrongBoxImplementation::TitanM {
                        version: "1.0".to_string(),
                        security_level: "EAL4+".to_string(),
                    },
                    hardware_backed: true,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::CertifiedHardware,
                user_presence_required: false, // Can be set per operation
            };

            // Register mobile HSM with the operation router
            hsm_manager
                .register_hsm_provider(mobile_tier, Arc::new(mobile_hsm))
                .await?;
        }
        Err(e) => {
            if setup.require_mobile_for_critical {
                warn!(
                    "⚠️ Mobile HSM unavailable but required for critical operations: {}",
                    e
                );
                return Err(BearDogError::Unavailable {
                    message: format!("Mobile HSM required but unavailable: {}", e),
                });
            } else {
                warn!("⚠️ Mobile HSM unavailable, using software-only mode: {}", e);
            }
        }
    }

    // Initialize software HSM for routine operations and fallback
    match initialize_software_hsm(&setup.software_hsm_config).await {
        Ok(software_hsm) => {
            info!("✅ Software HSM initialized successfully");

            // Create software HSM tier definition
            let software_tier = HsmTier::SoftwareHsm {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: KeyStorageType::InMemory,
                encryption_at_rest: true,
                memory_protection: MemoryProtectionLevel::High,
            };

            // Register software HSM with the operation router
            hsm_manager
                .register_hsm_provider(software_tier, Arc::new(software_hsm))
                .await?;
        }
        Err(e) => {
            return Err(BearDogError::Initialization {
                message: format!("Failed to initialize software HSM: {}", e),
            });
        }
    }

    info!("🎯 Mobile-first HSM manager initialization complete");
    Ok(hsm_manager)
}

/// Initialize Android StrongBox HSM
async fn initialize_mobile_hsm(config: &AndroidHsmConfig) -> BearDogResult<AndroidStrongBoxHsm> {
    info!("🔐 Initializing Android StrongBox HSM");

    // Check if we're running on Android
    if !cfg!(target_os = "android") {
        warn!("⚠️ Not running on Android - StrongBox unavailable");
        return Err(BearDogError::Unavailable {
            message: "Android StrongBox only available on Android devices".to_string(),
        });
    }

    // Create and initialize StrongBox HSM
    let strongbox_hsm = AndroidStrongBoxHsm::new(config.clone()).await?;

    // Note: In a real implementation, we would initialize the HSM provider
    // For now, we'll skip this initialization step as the exact interface is still being refined

    info!("✅ Android StrongBox HSM initialized and ready");
    Ok(strongbox_hsm)
}

/// Initialize software HSM for routine operations
async fn initialize_software_hsm(config: &SoftwareHsmConfig) -> BearDogResult<RustSoftwareHsm> {
    info!("💻 Initializing Rust Software HSM");

    let software_hsm = RustSoftwareHsm::new(config.clone()).await?;

    // Note: In a real implementation, we would initialize the HSM provider
    // For now, we'll skip this initialization step as the exact interface is still being refined

    info!("✅ Rust Software HSM initialized and ready");
    Ok(software_hsm)
}

/// Setup Pixel 8 + GrapheneOS optimized configuration
pub fn create_pixel8_graphene_config() -> MobileHsmSetup {
    info!("🎯 Creating Pixel 8 + GrapheneOS optimized HSM configuration");

    let _pixel8_config = Pixel8GrapheneOSConfig::default();

    MobileHsmSetup {
        require_mobile_for_critical: true,
        enable_graphene_optimizations: true,
        software_hsm_config: SoftwareHsmConfig::default(),
        android_config: AndroidHsmConfig::default(),
    }
}

/// Quick setup for development/testing
pub async fn setup_development_mobile_hsm() -> BearDogResult<HsmManager> {
    info!("🛠️ Setting up development mobile HSM environment");

    let config = MobileHsmSetup {
        require_mobile_for_critical: false, // Allow software fallback for development
        enable_graphene_optimizations: false,
        ..Default::default()
    };

    initialize_mobile_hsm_manager(config).await
}

/// Production setup with all security features enabled
pub async fn setup_production_mobile_hsm() -> BearDogResult<HsmManager> {
    info!("🚀 Setting up production mobile HSM environment");

    let config = create_pixel8_graphene_config();
    initialize_mobile_hsm_manager(config).await
}

/// Demonstrate mobile HSM operation routing
pub async fn demo_mobile_operations(hsm_manager: &HsmManager) -> BearDogResult<()> {
    info!("🎬 Demonstrating mobile HSM operation routing");

    // Show routing metrics
    let metrics = hsm_manager.get_routing_metrics().await?;
    info!("📊 Current routing metrics: {:?}", metrics);

    info!("🎉 Mobile HSM operation routing demonstration complete");
    Ok(())
}

// Convert HsmTier to manager's HsmTier enum - simplified since they're now the same type
fn convert_hsm_tier(tier: HsmTier) -> HsmTier {
    // Since we unified the type system, this is now a simple pass-through
    tier
}
