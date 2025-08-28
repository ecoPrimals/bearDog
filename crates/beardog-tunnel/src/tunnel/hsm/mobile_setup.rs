

use super::{
    android_strongbox::AndroidStrongBoxHsm, // Import the type
    manager::HsmManager,
    software_hsm::RustSoftwareHsm,
    HsmProvider, // Import HsmProvider trait
};

use crate::tunnel::hsm::android_strongbox::core; // Import implementation module
use crate::tunnel::hsm::types::{
    AndroidHsmConfig, AttestationLevel, HsmTier, KeyStorageType, MemoryProtectionLevel,
    Pixel8GrapheneOSConfig, SecureEnclaveType, SmartphoneType, SoftwareHsmConfig, SoftwareHsmType,
    StrongBoxImplementation,
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{info, warn};

pub struct MobileHsmSetup {

    pub require_mobile_for_critical: bool,

    pub enable_graphene_optimizations: bool,

    pub software_hsm_config: SoftwareHsmConfig,

    pub android_config: AndroidHsmConfig,
}
impl Default for MobileHsmSetup {}

    fn default() -> Self {
        Self {
            require_mobile_for_critical: true,
            enable_graphene_optimizations: true,
            software_hsm_config: SoftwareHsmConfig::default(),
            android_config: AndroidHsmConfig::default(),
        }
    }

pub async fn initialize_mobile_hsm_manager(setup: MobileHsmSetup) -> Result<HsmManager, BearDogError> {
    info!("📱 Initializing mobile-first HSM manager");
    let mut hsm_manager = HsmManager::new();

    match initialize_mobile_hsm(&setup.android_config).await {
        Ok(mobile_hsm) => {
            info!("✅ Mobile HSM (StrongBox) initialized successfully");

            let mobile_tier = HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: "Google".to_string(),}

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
                attestation_level: AttestationLevel::CertifiedHardware,
                user_presence_required: false, // Can be set per operation
            };

            hsm_manager
                .register_hsm_provider(mobile_tier, Arc::new(mobile_hsm))
                .await?;
        Err(e) => {
            if setup.require_mobile_for_critical {
                warn!(
                    "⚠️ Mobile HSM unavailable but required for critical operations: {}",
                    e
                );
                return Err(BearDogError::Unavailable {
                    message: format_args!("Mobile HSM required but unavailable: {}", e).to_string(),
                });
            } else {
                warn!("⚠️ Mobile HSM unavailable, using software-only mode: {}", e);
            }

    match initialize_software_hsm(&setup.software_hsm_config).await {
        Ok(software_hsm) => {
            info!("✅ Software HSM initialized successfully");

            let software_tier = HsmTier::SoftwareHsm {}

                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: KeyStorageType::InMemory,
                encryption_at_rest: true,
                memory_protection: MemoryProtectionLevel::High,

                .register_hsm_provider(software_tier, Arc::new(software_hsm))
            return Err(BearDogError::Initialization {
                message: format_args!("Failed to initialize software HSM: {}", e).to_string(),
            });
    info!("🎯 Mobile-first HSM manager initialization complete");
    Ok(hsm_manager)

async fn initialize_mobile_hsm(config: &AndroidHsmConfig) -> Result<AndroidStrongBoxHsm, BearDogError> {
    info!("🔐 Initializing Android StrongBox HSM");

    if !cfg!(target_os = "android") {
        warn!("⚠️ Not running on Android - StrongBox unavailable");
        return Err(BearDogError::Unavailable {
            message: "Android StrongBox only available on Android devices".to_string(),
        });

    let strongbox_hsm = AndroidStrongBoxHsm::new(config.clone()).await?;

    info!("✅ Android StrongBox HSM initialized and ready");
    Ok(strongbox_hsm)

async fn initialize_software_hsm(config: &SoftwareHsmConfig) -> Result<RustSoftwareHsm, BearDogError> {
    info!("💻 Initializing Rust Software HSM");
    let software_hsm = RustSoftwareHsm::new(config.clone()).await?;
    info!("✅ Rust Software HSM initialized and ready");
    Ok(software_hsm)

pub fn create_pixel8_graphene_config() -> MobileHsmSetup {
    info!("🎯 Creating Pixel 8 + GrapheneOS optimized HSM configuration");
    let _pixel8_config = Pixel8GrapheneOSConfig::default();
    MobileHsmSetup {
        require_mobile_for_critical: true,
        enable_graphene_optimizations: true,
        software_hsm_config: SoftwareHsmConfig::default(),
        android_config: AndroidHsmConfig::default(),

pub async fn setup_development_mobile_hsm() -> Result<HsmManager, BearDogError> {
    info!("🛠️ Setting up development mobile HSM environment");
    let config = MobileHsmSetup {
        require_mobile_for_critical: false, // Allow software fallback for development
        enable_graphene_optimizations: false,
        ..Default::default()
    };
    initialize_mobile_hsm_manager(config).await

pub async fn setup_production_mobile_hsm() -> Result<HsmManager, BearDogError> {
    info!("🚀 Setting up production mobile HSM environment");
    let config = create_pixel8_graphene_config();

pub async fn demo_mobile_operations(hsm_manager: &HsmManager) -> Result<(), BearDogError> {
    info!("🎬 Demonstrating mobile HSM operation routing");

    let metrics = hsm_manager.get_routing_metrics().await?;
    info!("📊 Current routing metrics: {:?}", metrics);
    info!("🎉 Mobile HSM operation routing demonstration complete");
    Ok(())

fn convert_hsm_tier(tier: HsmTier) -> HsmTier {

    tier
