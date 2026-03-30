// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Discovery Configuration
//!
//! HSM-specific discovery configuration that extends the canonical base.

use super::super::config::domains::discovery::DiscoveryConfig;
use serde::{Deserialize, Serialize};

/// HSM-specific discovery configuration
///
/// Wraps the canonical `DiscoveryConfig` with HSM-specific hardware detection flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmDiscoveryConfig {
    /// Base discovery configuration
    pub base: DiscoveryConfig,

    // HSM Hardware Type Discovery Flags
    /// Enable cloud KMS discovery (AWS KMS, Azure Key Vault, Google Cloud KMS)
    pub enable_cloud_kms: bool,

    /// Enable network HSM discovery (HSMs accessible over network)
    pub enable_network_hsm: bool,

    /// Enable USB HSM discovery (`YubiKey`, Nitrokey, etc.)
    pub enable_usb_hsm: bool,

    /// Enable software HSM discovery (software-based cryptographic modules)
    pub enable_software_hsm: bool,

    /// Enable mobile HSM discovery (Android `StrongBox`, iOS Secure Enclave)
    pub enable_mobile_hsm: bool,

    /// Enable TPM discovery (Trusted Platform Module)
    pub enable_tpm: bool,

    /// Enable PKCS#11 discovery
    pub enable_pkcs11_discovery: bool,

    // Capability Detection
    /// Enable automatic capability detection for discovered HSMs
    pub enable_capability_detection: bool,

    // Entropy & Quality
    /// Enable human entropy elevation (for hybrid entropy systems)
    pub enable_human_entropy_elevation: bool,

    /// Minimum entropy quality threshold (0.0 - 1.0)
    pub minimum_entropy_quality: f64,
}

impl Default for HsmDiscoveryConfig {
    fn default() -> Self {
        Self {
            base: DiscoveryConfig::default(),
            enable_cloud_kms: true,
            enable_network_hsm: true,
            enable_usb_hsm: true,
            enable_software_hsm: true,
            enable_mobile_hsm: true,
            enable_tpm: true,
            enable_pkcs11_discovery: true,
            enable_capability_detection: true,
            enable_human_entropy_elevation: true,
            minimum_entropy_quality: 0.8,
        }
    }
}

impl HsmDiscoveryConfig {
    /// Create a restrictive configuration (only essential HSM types)
    pub fn restrictive() -> Self {
        Self {
            enable_cloud_kms: false,
            enable_network_hsm: true, // Essential
            enable_usb_hsm: true,     // Essential
            enable_software_hsm: false,
            enable_mobile_hsm: false,
            enable_tpm: true, // Essential
            enable_pkcs11_discovery: true,
            minimum_entropy_quality: 0.9, // Higher threshold
            ..Default::default()
        }
    }

    /// Create a permissive configuration (all HSM types)
    pub fn permissive() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn default_permissive_and_restrictive() {
        let d = HsmDiscoveryConfig::default();
        let p = HsmDiscoveryConfig::permissive();
        assert!(d.enable_software_hsm);
        assert!(p.enable_software_hsm);
        let r = HsmDiscoveryConfig::restrictive();
        assert!(!r.enable_software_hsm);
        assert!(r.enable_network_hsm);
        assert!(r.minimum_entropy_quality >= d.minimum_entropy_quality);
    }

    #[test]
    fn serde_json_roundtrip() {
        let c = HsmDiscoveryConfig::default();
        let j = serde_json::to_string(&c).expect("serialize HsmDiscoveryConfig");
        let back: HsmDiscoveryConfig = serde_json::from_str(&j).expect("deserialize");
        assert_eq!(c.enable_tpm, back.enable_tpm);
        assert_eq!(c.minimum_entropy_quality, back.minimum_entropy_quality);
        assert_eq!(c.enable_cloud_kms, back.enable_cloud_kms);
    }

    #[test]
    fn debug_clone_smoke() {
        let c = HsmDiscoveryConfig::restrictive();
        let s = format!("{c:?}");
        assert!(
            s.contains("base") || s.contains("HsmDiscoveryConfig"),
            "{s}"
        );
    }
}
