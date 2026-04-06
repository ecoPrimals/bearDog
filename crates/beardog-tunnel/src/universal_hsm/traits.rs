// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Trait Definitions
//!
//! Core trait definitions for the universal HSM system.

use crate::tunnel::hsm::types::{HsmCapability, KeyType};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Universal HSM Provider trait
///
/// This trait defines the interface that all HSM providers must implement
pub trait UniversalHsmProvider: Send + Sync {
    /// Get provider information
    fn get_provider_info(&self) -> ProviderInfo;

    /// Generate a new key
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails.
    fn generate_key(&self, key_type: KeyType) -> Result<Vec<u8>, BearDogError>;

    /// Sign data with a key
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails.
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails.
    fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError>;

    /// Get provider capabilities
    fn get_capabilities(&self) -> Vec<HsmCapability>;
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Provider version
    pub version: String,
    /// Capabilities verified
    pub capabilities_verified: bool,
}

/// Provider type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Mobile hardware security
    MobileHardware,
    /// Desktop hardware security
    DesktopHardware,
    /// Software HSM
    Software,
    /// PKCS#11 hardware token
    Pkcs11,
    /// TPM 2.0+
    Tpm,
    /// Cloud HSM
    Cloud,
    /// USB security token
    UsbToken,
    /// Network HSM
    NetworkHsm,
    /// Custom provider
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MobileHardware => write!(f, "Mobile Hardware Security"),
            Self::DesktopHardware => write!(f, "Desktop Hardware Security"),
            Self::Software => write!(f, "Software HSM"),
            Self::Pkcs11 => write!(f, "PKCS#11 HSM"),
            Self::Tpm => write!(f, "TPM 2.0+"),
            Self::Cloud => write!(f, "Cloud HSM"),
            Self::UsbToken => write!(f, "USB Security Token"),
            Self::NetworkHsm => write!(f, "Network HSM"),
            Self::Custom => write!(f, "Custom HSM"),
        }
    }
}

/// Platform enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    /// Mobile platform
    Mobile,
    /// Desktop platform
    Desktop,
    /// Server platform
    Server,
    /// Embedded platform
    Embedded,
    /// WebAssembly
    Wasm,
    /// Universal (all platforms)
    Universal,
}

impl Default for ProviderInfo {
    fn default() -> Self {
        Self {
            name: "Unknown Provider".to_string(),
            provider_type: ProviderType::Software,
            version: "0.0.0".to_string(),
            capabilities_verified: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn provider_info_default_and_clone() {
        let a = ProviderInfo::default();
        let b = a.clone();
        assert_eq!(a.name, "Unknown Provider");
        assert_eq!(a.provider_type, ProviderType::Software);
        assert_eq!(a.version, "0.0.0");
        assert!(!a.capabilities_verified);
        assert_eq!(a.name, b.name);
        assert_eq!(a.provider_type, b.provider_type);
    }

    #[test]
    fn provider_type_display_all_variants() {
        let cases = [
            (ProviderType::MobileHardware, "Mobile Hardware Security"),
            (ProviderType::DesktopHardware, "Desktop Hardware Security"),
            (ProviderType::Software, "Software HSM"),
            (ProviderType::Pkcs11, "PKCS#11 HSM"),
            (ProviderType::Tpm, "TPM 2.0+"),
            (ProviderType::Cloud, "Cloud HSM"),
            (ProviderType::UsbToken, "USB Security Token"),
            (ProviderType::NetworkHsm, "Network HSM"),
            (ProviderType::Custom, "Custom HSM"),
        ];
        for (p, expected) in cases {
            assert_eq!(
                format!("{p}"),
                expected,
                "ProviderType display mismatch for {p:?}"
            );
        }
    }

    #[test]
    fn provider_info_serde_json_roundtrip() {
        let info = ProviderInfo {
            name: "Test".to_string(),
            provider_type: ProviderType::Pkcs11,
            version: "1.2.3".to_string(),
            capabilities_verified: true,
        };
        let json = serde_json::to_string(&info).expect("serialize ProviderInfo");
        let back: ProviderInfo = serde_json::from_str(&json).expect("deserialize ProviderInfo");
        assert_eq!(info.name, back.name);
        assert_eq!(info.provider_type, back.provider_type);
        assert_eq!(info.version, back.version);
        assert_eq!(info.capabilities_verified, back.capabilities_verified);
    }

    #[test]
    fn platform_enum_serde_roundtrip() {
        for p in [
            Platform::Mobile,
            Platform::Desktop,
            Platform::Server,
            Platform::Embedded,
            Platform::Wasm,
            Platform::Universal,
        ] {
            let json = serde_json::to_string(&p).expect("serialize Platform");
            let back: Platform = serde_json::from_str(&json).expect("deserialize Platform");
            assert_eq!(p, back);
        }
    }
}
