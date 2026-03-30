// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical HSM provider types for the unified `HsmKeyProvider` trait.
//!
//! These types are shared across all HSM backends (software, Android `StrongBox`,
//! iOS Secure Enclave, PKCS#11, TPM) and form the vocabulary of the
//! provider-agnostic HSM abstraction.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Identifies the kind of HSM backend behind an `HsmKeyProvider` implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmProviderType {
    /// Pure-Rust in-process key store (`RustCrypto`). Always available.
    Software,
    /// Android Keystore backed by `StrongBox` Keymaster hardware.
    AndroidStrongBox,
    /// iOS Secure Enclave (T2/Apple Silicon).
    IosSecureEnclave,
    /// PKCS#11 token (Yubikey, Nitrokey, `CloudHSM` PKCS#11 bridge, etc.).
    Pkcs11,
    /// Trusted Platform Module 2.0.
    Tpm,
}

impl std::fmt::Display for HsmProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Software => write!(f, "software"),
            Self::AndroidStrongBox => write!(f, "android-strongbox"),
            Self::IosSecureEnclave => write!(f, "ios-secure-enclave"),
            Self::Pkcs11 => write!(f, "pkcs11"),
            Self::Tpm => write!(f, "tpm"),
        }
    }
}

/// Cryptographic algorithm identifier for key generation and operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmAlgorithm {
    /// AES-256-GCM authenticated encryption.
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption.
    ChaCha20Poly1305,
    /// Ed25519 signatures.
    Ed25519,
    /// ECDSA on P-256 (NIST).
    EcdsaP256,
    /// ECDSA on P-384 (NIST).
    EcdsaP384,
    /// X25519 key agreement.
    X25519,
    /// HMAC-SHA256.
    HmacSha256,
    /// RSA-2048 (sign/verify only).
    Rsa2048,
    /// RSA-4096 (sign/verify only).
    Rsa4096,
}

impl std::fmt::Display for HsmAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aes256Gcm => write!(f, "AES-256-GCM"),
            Self::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::EcdsaP256 => write!(f, "ECDSA-P256"),
            Self::EcdsaP384 => write!(f, "ECDSA-P384"),
            Self::X25519 => write!(f, "X25519"),
            Self::HmacSha256 => write!(f, "HMAC-SHA256"),
            Self::Rsa2048 => write!(f, "RSA-2048"),
            Self::Rsa4096 => write!(f, "RSA-4096"),
        }
    }
}

/// Parameters for key generation via `HsmKeyProvider::generate_key`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenParams {
    /// Algorithm to use for the generated key.
    pub algorithm: HsmAlgorithm,
    /// Optional human-readable label stored alongside the key.
    pub label: Option<String>,
    /// Whether the key should be extractable (software providers only;
    /// hardware providers silently ignore this and always store non-extractably).
    pub extractable: bool,
}

impl KeyGenParams {
    /// Shorthand: generate a non-extractable key with the given algorithm.
    #[must_use]
    pub fn new(algorithm: HsmAlgorithm) -> Self {
        Self {
            algorithm,
            label: None,
            extractable: false,
        }
    }
}

/// Opaque handle returned after key generation or import.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyHandle {
    /// Provider-scoped identifier used in subsequent operations.
    pub key_id: String,
    /// Algorithm the key was generated with.
    pub algorithm: HsmAlgorithm,
    /// `true` when the key lives in tamper-resistant hardware.
    pub hardware_backed: bool,
    /// When the key was created (milliseconds since UNIX epoch).
    pub created_at_ms: u64,
}

/// Declares which algorithms and operations a provider supports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilitySet {
    /// Set of supported algorithms.
    pub algorithms: HashSet<HsmAlgorithm>,
    /// Whether the provider stores keys in hardware.
    pub hardware_backed: bool,
    /// Whether key material can be exported/imported.
    pub supports_key_export: bool,
    /// Maximum number of resident keys (0 = unlimited).
    pub max_keys: u32,
}

impl Default for HsmCapabilitySet {
    fn default() -> Self {
        Self {
            algorithms: HashSet::new(),
            hardware_backed: false,
            supports_key_export: true,
            max_keys: 0,
        }
    }
}

impl HsmCapabilitySet {
    /// Returns `true` if the given algorithm is advertised.
    #[must_use]
    pub fn supports(&self, algorithm: HsmAlgorithm) -> bool {
        self.algorithms.contains(&algorithm)
    }
}

/// Preference passed to `HsmProviderRegistry::select` to control backend choice.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionPreference {
    /// Use the highest-security available backend (hardware > software).
    #[default]
    PreferHardware,
    /// Fail if no hardware-backed provider is available.
    RequireHardware,
    /// Always use the software provider regardless of hardware availability.
    SoftwareOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_type_display() {
        assert_eq!(HsmProviderType::Software.to_string(), "software");
        assert_eq!(
            HsmProviderType::AndroidStrongBox.to_string(),
            "android-strongbox"
        );
    }

    #[test]
    fn algorithm_display() {
        assert_eq!(HsmAlgorithm::Aes256Gcm.to_string(), "AES-256-GCM");
        assert_eq!(HsmAlgorithm::Ed25519.to_string(), "Ed25519");
    }

    #[test]
    fn key_gen_params_new() {
        let p = KeyGenParams::new(HsmAlgorithm::Ed25519);
        assert_eq!(p.algorithm, HsmAlgorithm::Ed25519);
        assert!(!p.extractable);
        assert!(p.label.is_none());
    }

    #[test]
    fn capability_set_supports() {
        let mut caps = HsmCapabilitySet::default();
        caps.algorithms.insert(HsmAlgorithm::Aes256Gcm);
        assert!(caps.supports(HsmAlgorithm::Aes256Gcm));
        assert!(!caps.supports(HsmAlgorithm::Ed25519));
    }

    #[test]
    fn selection_preference_default_is_prefer_hardware() {
        assert_eq!(
            SelectionPreference::default(),
            SelectionPreference::PreferHardware
        );
    }

    #[test]
    fn key_handle_serde_roundtrip() {
        let h = KeyHandle {
            key_id: "k1".into(),
            algorithm: HsmAlgorithm::ChaCha20Poly1305,
            hardware_backed: false,
            created_at_ms: 1_711_612_800_000,
        };
        let json = serde_json::to_string(&h).expect("serialize KeyHandle");
        let back: KeyHandle = serde_json::from_str(&json).expect("deserialize KeyHandle");
        assert_eq!(back.key_id, "k1");
        assert!(!back.hardware_backed);
    }

    #[test]
    fn provider_type_serde_roundtrip() {
        for pt in [
            HsmProviderType::Software,
            HsmProviderType::AndroidStrongBox,
            HsmProviderType::IosSecureEnclave,
            HsmProviderType::Pkcs11,
            HsmProviderType::Tpm,
        ] {
            let json = serde_json::to_string(&pt).expect("serialize");
            let back: HsmProviderType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(pt, back);
        }
    }
}
