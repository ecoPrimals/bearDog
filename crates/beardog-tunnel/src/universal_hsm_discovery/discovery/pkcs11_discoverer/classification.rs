// SPDX-License-Identifier: AGPL-3.0-only

//! Manufacturer-based token classification for PKCS#11 profiles.

use super::discoverer::Pkcs11Discoverer;
use super::types::Pkcs11TokenInfo;
use crate::tunnel::hsm::types::capability::*;
use crate::universal_hsm_discovery::*;

impl Pkcs11Discoverer {
    /// Classify token and determine capabilities
    pub(crate) fn classify_token(
        &self,
        token_info: &Pkcs11TokenInfo,
    ) -> (HsmType, HsmTier, UniversalHsmCapabilities) {
        // Classify based on manufacturer
        match token_info.manufacturer_id.to_lowercase().as_str() {
            s if s.contains("thales") || s.contains("ncipher") => (
                HsmType::Hardware,
                HsmTier::Tier1,
                self.create_enterprise_hsm_capabilities(),
            ),
            s if s.contains("utimaco") => (
                HsmType::Hardware,
                HsmTier::Tier1,
                self.create_enterprise_hsm_capabilities(),
            ),
            s if s.contains("amazon") || s.contains("cloudhsm") => (
                HsmType::Cloud,
                HsmTier::Tier1,
                self.create_cloud_hsm_capabilities(),
            ),
            s if s.contains("yubico") => (
                HsmType::Hardware,
                HsmTier::Tier2,
                self.create_yubikey_pkcs11_capabilities(),
            ),
            s if s.contains("tpm") => (
                HsmType::Hardware,
                HsmTier::Tier2,
                self.create_tpm_pkcs11_capabilities(),
            ),
            s if s.contains("softhsm") => (
                HsmType::Software,
                HsmTier::Tier3,
                self.create_softhsm_capabilities(),
            ),
            s if s.contains("opensc") => (
                HsmType::Hardware,
                HsmTier::Tier3,
                self.create_smartcard_capabilities(),
            ),
            _ => (
                HsmType::Software,
                HsmTier::Tier4,
                self.create_generic_pkcs11_capabilities(),
            ),
        }
    }
}
