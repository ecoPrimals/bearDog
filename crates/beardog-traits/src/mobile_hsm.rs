// SPDX-License-Identifier: AGPL-3.0-or-later

//! Vendor-agnostic mobile HSM capability trait.
//!
//! Captures the common capabilities of mobile hardware security modules
//! across Android (StrongBox), iOS (Secure Enclave), and future platforms.
//! Implementations compose with [`HsmKeyProvider`](super::hsm::HsmKeyProvider)
//! for the full key lifecycle.

use beardog_types::hsm::HsmAlgorithm;
use serde::{Deserialize, Serialize};

/// Classifies the secure element hardware backing a mobile HSM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureElementType {
    /// Google Titan M2 / Qualcomm SPU / Samsung Knox (Android `StrongBox`).
    AndroidStrongBox,
    /// Apple T2 / Apple Silicon Secure Enclave.
    IosSecureEnclave,
    /// Software-only fallback (no hardware isolation).
    SoftwareFallback,
    /// Hardware present but type not yet classified.
    Unknown,
}

impl std::fmt::Display for SecureElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AndroidStrongBox => write!(f, "android-strongbox"),
            Self::IosSecureEnclave => write!(f, "ios-secure-enclave"),
            Self::SoftwareFallback => write!(f, "software-fallback"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Vendor-agnostic mobile HSM capability surface.
///
/// Any mobile HSM backend (`StrongBox`, Secure Enclave, future platforms)
/// implements this to advertise its hardware-level capabilities. The
/// orchestrator uses these queries to select the best provider at runtime
/// without coupling to vendor-specific APIs.
pub trait MobileHsmCapability: Send + Sync {
    /// Whether the device can produce hardware attestation certificates
    /// proving key material is inside the secure element.
    fn hardware_attestation_available(&self) -> bool;

    /// Whether biometric authentication (fingerprint / face) can gate
    /// key usage on this device.
    fn biometric_gate_available(&self) -> bool;

    /// The type of secure element backing this provider.
    fn secure_element_type(&self) -> SecureElementType;

    /// Algorithms supported by the hardware secure element.
    fn supported_algorithms(&self) -> Vec<HsmAlgorithm>;
}

/// Abstraction for sealing a master key to platform-specific hardware.
///
/// On Android, the master key can be wrapped by Keystore2 hardware keys.
/// On iOS, it can be protected by Keychain with `kSecAttrAccessibleAfterFirstUnlock`.
/// On all platforms, a software HKDF fallback is available.
pub trait MasterKeySealer: Send + Sync {
    /// Seal (wrap) `plaintext` so it can only be recovered on this device.
    ///
    /// # Errors
    /// Returns an error if the sealing hardware/API is unavailable.
    fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>, beardog_errors::BearDogError>;

    /// Unseal (unwrap) `ciphertext` previously produced by [`Self::seal`].
    ///
    /// # Errors
    /// Returns an error if the ciphertext is corrupt or was sealed on a different device.
    fn unseal(&self, ciphertext: &[u8]) -> Result<Vec<u8>, beardog_errors::BearDogError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubMobileHsm;

    impl MobileHsmCapability for StubMobileHsm {
        fn hardware_attestation_available(&self) -> bool {
            false
        }
        fn biometric_gate_available(&self) -> bool {
            false
        }
        fn secure_element_type(&self) -> SecureElementType {
            SecureElementType::SoftwareFallback
        }
        fn supported_algorithms(&self) -> Vec<HsmAlgorithm> {
            vec![HsmAlgorithm::Aes256Gcm]
        }
    }

    struct StubSealer;

    impl MasterKeySealer for StubSealer {
        fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>, beardog_errors::BearDogError> {
            Ok(plaintext.to_vec())
        }
        fn unseal(&self, ciphertext: &[u8]) -> Result<Vec<u8>, beardog_errors::BearDogError> {
            Ok(ciphertext.to_vec())
        }
    }

    #[test]
    fn stub_mobile_hsm_defaults() {
        let hsm = StubMobileHsm;
        assert!(!hsm.hardware_attestation_available());
        assert!(!hsm.biometric_gate_available());
        assert_eq!(hsm.secure_element_type(), SecureElementType::SoftwareFallback);
        assert_eq!(hsm.supported_algorithms(), vec![HsmAlgorithm::Aes256Gcm]);
    }

    #[test]
    fn stub_sealer_roundtrip() {
        let sealer = StubSealer;
        let plaintext = b"master-key-material";
        let sealed = sealer.seal(plaintext).unwrap();
        let unsealed = sealer.unseal(&sealed).unwrap();
        assert_eq!(plaintext.as_slice(), unsealed.as_slice());
    }

    #[test]
    fn secure_element_type_display() {
        assert_eq!(SecureElementType::AndroidStrongBox.to_string(), "android-strongbox");
        assert_eq!(SecureElementType::IosSecureEnclave.to_string(), "ios-secure-enclave");
        assert_eq!(SecureElementType::SoftwareFallback.to_string(), "software-fallback");
        assert_eq!(SecureElementType::Unknown.to_string(), "unknown");
    }
}
