//! # HSM Capability Detection
//!
//! This module provides capability detection and recommendation for HSM providers,
//! including tier recommendation based on security requirements.

use super::{HsmCapabilityDetector, SecurityRequirements, SecurityLevel};
use crate::error::BearDogResult;
use crate::tunnel::hsm::types::{HsmCapability, HsmTier, SoftwareHsmType, SmartphoneType, SecureEnclaveType, StrongBoxImplementation, HsmVendor, CertificationLevel, TamperResistanceLevel, KeyHierarchy, FallbackStrategy, MemoryProtectionLevel, KeyStorageType, AttestationLevel};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

/// Default HSM capability detector
pub struct DefaultHsmCapabilityDetector {
    pub(crate) provider_capabilities: Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}

impl DefaultHsmCapabilityDetector {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait]
impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapability>> {
        // Return common capabilities that most HSMs support
        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
        ];

        Ok(capabilities)
    }

    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool> {
        match hsm_type {
            HsmTier::SoftwareHsm { .. } => Ok(true), // Software HSM always available
            HsmTier::SmartphoneHsm { .. } => {
                // Check if smartphone HSM is available
                // For now, assume it's available if we're on a mobile platform
                Ok(true)
            }
            HsmTier::HardwareHsm { .. } => {
                // Check if hardware HSM is available
                // For now, assume it's not available unless explicitly configured
                Ok(false)
            }
            HsmTier::HybridHsm { .. } => {
                // Check if hybrid HSM is available
                // For now, assume it's not available unless explicitly configured
                Ok(false)
            }
        }
    }

    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier> {
        // Recommend HSM tier based on security requirements
        match requirements.security_level {
            SecurityLevel::Basic => {
                // Basic security - software HSM is sufficient
                Ok(HsmTier::SoftwareHsm {
                    implementation: SoftwareHsmType::RustSoftwareHsm,
                    key_storage: KeyStorageType::EncryptedFile,
                    encryption_at_rest: true,
                    memory_protection: MemoryProtectionLevel::Basic,
                })
            }
            SecurityLevel::Medium => {
                // Medium security - prefer smartphone HSM if available
                if requirements.hardware_backed_required {
                    Ok(HsmTier::SmartphoneHsm {
                        device_type: SmartphoneType::Android {
                            manufacturer: "Google".to_string(),
                            model: "Pixel".to_string(),
                            android_version: "13".to_string(),
                            strongbox_version: Some("1.0".to_string()),
                        },
                        secure_enclave: SecureEnclaveType::AndroidStrongBox {
                            implementation: StrongBoxImplementation::TitanM {
                                version: "1.0".to_string(),
                                security_level: "StrongBox".to_string(),
                            },
                            hardware_backed: true,
                            key_attestation: true,
                        },
                        attestation_level: AttestationLevel::Hardware,
                        user_presence_required: false,
                    })
                } else {
                    Ok(HsmTier::SoftwareHsm {
                        implementation: SoftwareHsmType::RustSoftwareHsm,
                        key_storage: KeyStorageType::EncryptedFile,
                        encryption_at_rest: true,
                        memory_protection: MemoryProtectionLevel::High,
                    })
                }
            }
            SecurityLevel::High => {
                // High security - require hardware-backed HSM
                if requirements.user_interaction_required {
                    Ok(HsmTier::SmartphoneHsm {
                        device_type: SmartphoneType::Android {
                            manufacturer: "Google".to_string(),
                            model: "Pixel".to_string(),
                            android_version: "13".to_string(),
                            strongbox_version: Some("1.0".to_string()),
                        },
                        secure_enclave: SecureEnclaveType::AndroidStrongBox {
                            implementation: StrongBoxImplementation::TitanM {
                                version: "1.0".to_string(),
                                security_level: "StrongBox".to_string(),
                            },
                            hardware_backed: true,
                            key_attestation: true,
                        },
                        attestation_level: AttestationLevel::Hardware,
                        user_presence_required: true,
                    })
                } else {
                    Ok(HsmTier::HardwareHsm {
                        vendor: HsmVendor::Custom("YubiKey".to_string()),
                        model: "YubiHSM2".to_string(),
                        certification: CertificationLevel::Fips140Level2,
                        tamper_resistance: TamperResistanceLevel::Hardware,
                    })
                }
            }
            SecurityLevel::Maximum => {
                // Maximum security - require certified hardware HSM
                if requirements.user_interaction_required {
                    Ok(HsmTier::SmartphoneHsm {
                        device_type: SmartphoneType::Android {
                            manufacturer: "Google".to_string(),
                            model: "Pixel".to_string(),
                            android_version: "13".to_string(),
                            strongbox_version: Some("1.0".to_string()),
                        },
                        secure_enclave: SecureEnclaveType::AndroidStrongBox {
                            implementation: StrongBoxImplementation::TitanM {
                                version: "1.0".to_string(),
                                security_level: "StrongBox".to_string(),
                            },
                            hardware_backed: true,
                            key_attestation: true,
                        },
                        attestation_level: AttestationLevel::CertifiedHardware,
                        user_presence_required: true,
                    })
                } else {
                    Ok(HsmTier::HardwareHsm {
                        vendor: HsmVendor::Thales,
                        model: "Luna".to_string(),
                        certification: CertificationLevel::Fips140Level3,
                        tamper_resistance: TamperResistanceLevel::HardwareDestruction,
                    })
                }
            }
        }
    }
} 