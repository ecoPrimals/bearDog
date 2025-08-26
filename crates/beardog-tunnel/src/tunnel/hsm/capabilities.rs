

use super::{
    HsmCapabilityDetector, HsmCapability, HsmTier, SecurityRequirements, SecurityLevel,
    SoftwareHsmType, KeyStorageType, MemoryProtectionLevel, SmartphoneType, SecureEnclaveType,
    StrongBoxImplementation, AttestationLevel,
};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DefaultHsmCapabilityDetector {
    provider_capabilities: Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}
impl DefaultHsmCapabilityDetector {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapability>> {

        Ok(vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
            HsmCapability::Signing,
            HsmCapability::Verification,
        ])}

    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool> {
        match hsm_type {
            HsmTier::SoftwareHsm { .. } => Ok(true), // Always available
            HsmTier::SmartphoneHsm { .. } => {

                Ok(std::env::consts::OS == "android")
            }
            HsmTier::HardwareHsm { .. } => Ok(false), // Not implemented
            HsmTier::HybridHsm { .. } => Ok(false),   // Not implemented
        }
    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier> {
        match requirements.security_level {
            SecurityLevel::Basic => Ok(HsmTier::SoftwareHsm {}

                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: KeyStorageType::Memory,
                encryption_at_rest: true,
                memory_protection: MemoryProtectionLevel::High,
            }),
            SecurityLevel::Medium | SecurityLevel::High => {
                if requirements.user_interaction_required {
                    Ok(HsmTier::SmartphoneHsm {
                        device_type: SmartphoneType::Android {
                            manufacturer: "Google".to_string(),
                            model: "Pixel 8a".to_string(),
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
                        attestation_level: AttestationLevel::Hardware,
                        user_presence_required: true,
                    })
                } else {
                    Ok(HsmTier::SoftwareHsm {
                        implementation: SoftwareHsmType::RustSoftwareHsm,
                        key_storage: KeyStorageType::EncryptedFile,
                        encryption_at_rest: true,
                        memory_protection: MemoryProtectionLevel::Maximum,
                }
            SecurityLevel::Maximum => {

                Ok(HsmTier::SmartphoneHsm {
                    device_type: SmartphoneType::Android {
                        manufacturer: "Google".to_string(),}

                        model: "Pixel 8a".to_string(),
                        android_version: "14".to_string(),
                        strongbox_version: Some("1.0".to_string()),
                    },
                    secure_enclave: SecureEnclaveType::AndroidStrongBox {
                        implementation: StrongBoxImplementation::TitanM {
                            version: "1.0".to_string(),
                            security_level: "EAL4+".to_string(),
                        hardware_backed: true,
                        key_attestation: true,
                    attestation_level: AttestationLevel::Hardware,
                    user_presence_required: true,
                })
} 
