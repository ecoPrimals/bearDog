// SPDX-License-Identifier: AGPL-3.0-or-later

//! iOS Secure Enclave integration for the BearDog ecosystem.
//!
//! On iOS, this module uses `security-framework` to call into Apple's
//! Security.framework for hardware-backed P-256 key generation and ECDSA
//! signing via the Secure Enclave. On non-Apple platforms, all operations
//! return errors (fail-closed, Silicon Atheism principle).

use crate::tunnel::hsm::types::{HsmKey, KeyType};
#[cfg(target_os = "ios")]
use crate::tunnel::hsm::types::{KeyHealthStatus, KeyMaterial, KeyMetadata};
use beardog_errors::BearDogError;
use tracing::info;

/// Safe wrapper around iOS Secure Enclave hardware operations.
///
/// On iOS devices with A7+ chips, this provides real hardware-backed P-256
/// key generation and ECDSA-SHA256 signing via the Secure Enclave processor.
/// Private keys never leave the Secure Enclave hardware.
pub struct SafeSecureEnclave {
    available: bool,
}

impl SafeSecureEnclave {
    /// Creates a new Secure Enclave interface.
    ///
    /// # Errors
    /// Returns an error if platform detection fails.
    pub async fn new() -> Result<Self, BearDogError> {
        let available = Self::check_availability();
        if available {
            info!("Secure Enclave hardware detected and available");
        } else {
            info!("Secure Enclave not available on this platform");
        }
        Ok(Self { available })
    }

    fn check_availability() -> bool {
        cfg!(target_os = "ios")
    }

    /// Generate a hardware-backed P-256 key in the Secure Enclave.
    ///
    /// On iOS, this calls `SecKeyCreateRandomKey` with
    /// `kSecAttrTokenIDSecureEnclave` to ensure the private key is
    /// generated inside the Secure Enclave processor and never leaves it.
    ///
    /// The Secure Enclave only supports P-256 (EllipticCurve). Other key
    /// types are rejected.
    ///
    /// # Errors
    /// Returns an error if the Secure Enclave is unavailable or key generation fails.
    pub async fn safe_generate_key(
        &self,
        key_id: &str,
        key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        if !self.available {
            return Err(BearDogError::unsupported_platform(
                "Secure Enclave not available on this platform",
            ));
        }

        if !matches!(key_type, KeyType::EllipticCurve) {
            return Err(BearDogError::security(format!(
                "Secure Enclave only supports EllipticCurve (P-256) keys, got {key_type:?}. \
                 Use KeyType::EllipticCurve for hardware-backed keys."
            )));
        }

        #[cfg(target_os = "ios")]
        {
            self.ios_generate_key(key_id, key_type).await
        }

        #[cfg(not(target_os = "ios"))]
        {
            let _ = (key_id, key_type);
            Err(BearDogError::unsupported_platform(
                "Secure Enclave requires iOS",
            ))
        }
    }

    /// Sign data using a Secure Enclave-backed key.
    ///
    /// # Errors
    /// Returns an error if the key is not found or signing fails.
    pub async fn safe_sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.available {
            return Err(BearDogError::unsupported_platform(
                "Secure Enclave not available for signing",
            ));
        }

        #[cfg(target_os = "ios")]
        {
            self.ios_sign(key_id, data).await
        }

        #[cfg(not(target_os = "ios"))]
        {
            let _ = (key_id, data);
            Err(BearDogError::unsupported_platform(
                "Secure Enclave signing requires iOS",
            ))
        }
    }

    /// Verify a signature using the Secure Enclave.
    ///
    /// # Errors
    /// Returns an error if the key is not found or verification fails.
    pub async fn safe_verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        if !self.available {
            return Err(BearDogError::unsupported_platform(
                "Secure Enclave not available for verification",
            ));
        }

        #[cfg(target_os = "ios")]
        {
            self.ios_verify(key_id, data, signature).await
        }

        #[cfg(not(target_os = "ios"))]
        {
            let _ = (key_id, data, signature);
            Err(BearDogError::unsupported_platform(
                "Secure Enclave verification requires iOS",
            ))
        }
    }

    /// Checks if Secure Enclave hardware is available.
    pub fn is_available(&self) -> bool {
        self.available
    }

    // ── iOS-specific implementations ────────────────────────────────

    #[cfg(target_os = "ios")]
    async fn ios_generate_key(
        &self,
        key_id: &str,
        key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        use security_framework::key::{
            GenerateKeyOptions, KeyType as SecKeyType, SecKey, Token,
        };

        info!("Generating P-256 key in Secure Enclave: {}", key_id);

        let mut opts = GenerateKeyOptions::default();
        opts.set_key_type(SecKeyType::ec());
        opts.set_token(Token::SecureEnclave);
        opts.set_label(key_id);

        let key = SecKey::generate(opts.to_dictionary())
            .map_err(|e| BearDogError::security(format!(
                "Secure Enclave key generation failed: {e}"
            )))?;

        let public_key_data = key
            .external_representation()
            .map(|d| d.to_vec())
            .unwrap_or_default();

        info!(
            "Secure Enclave key generated: {} ({} byte public key)",
            key_id,
            public_key_data.len()
        );

        let mut tags = std::collections::HashMap::new();
        tags.insert("hardware_backed".to_string(), "true".to_string());
        tags.insert("enclave".to_string(), "ios_secure_enclave".to_string());

        Ok(HsmKey {
            id: key_id.to_string(),
            hsm_type: "ios_secure_enclave".to_string(),
            key_type: key_type.clone(),
            metadata: KeyMetadata::new(key_id.to_string(), key_type.clone()),
            key_material: KeyMaterial::HardwareReference {
                reference: key_id.to_string(),
                hsm_location: "ios_secure_enclave".to_string(),
            },
            hsm_tier: "hardware".to_string(),
            created_at: chrono::Utc::now(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
        })
    }

    #[cfg(target_os = "ios")]
    fn lookup_key(&self, key_id: &str) -> Result<security_framework::key::SecKey, BearDogError> {
        use security_framework::item::{ItemClass, ItemSearchOptions, Reference, SearchResult};

        let results = ItemSearchOptions::new()
            .class(ItemClass::key())
            .label(key_id)
            .load_refs(true)
            .limit(1)
            .search()
            .map_err(|e| BearDogError::security(format!(
                "Keychain lookup failed for key '{key_id}': {e}"
            )))?;

        for result in &results {
            if let SearchResult::Ref(Reference::Key(key)) = result {
                return Ok(key.clone());
            }
        }

        Err(BearDogError::security(format!(
            "Key '{key_id}' not found in Secure Enclave keychain"
        )))
    }

    #[cfg(target_os = "ios")]
    async fn ios_sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use security_framework::key::Algorithm;

        let key = self.lookup_key(key_id)?;

        let signature = key
            .create_signature(Algorithm::ECDSASignatureMessageX962SHA256, data)
            .map_err(|e| BearDogError::security(format!(
                "Secure Enclave signing failed: {e}"
            )))?;

        info!(
            "Secure Enclave signed {} bytes -> {} byte signature",
            data.len(),
            signature.len()
        );
        Ok(signature)
    }

    #[cfg(target_os = "ios")]
    async fn ios_verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        use security_framework::key::Algorithm;

        let key = self.lookup_key(key_id)?;

        let public_key = key.public_key()
            .ok_or_else(|| BearDogError::security(
                "Failed to extract public key from Secure Enclave key".to_string()
            ))?;

        let valid = public_key
            .verify_signature(Algorithm::ECDSASignatureMessageX962SHA256, data, signature)
            .is_ok();

        info!("Secure Enclave signature verification: {}", valid);
        Ok(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_secure_enclave_creation() -> Result<(), BearDogError> {
        let enclave = SafeSecureEnclave::new().await?;
        if cfg!(target_os = "ios") {
            assert!(enclave.is_available());
        } else {
            assert!(!enclave.is_available());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_key_unavailable_on_non_ios() {
        let enclave = SafeSecureEnclave::new().await.unwrap();
        if !cfg!(target_os = "ios") {
            let result = enclave
                .safe_generate_key("test_key", &KeyType::EllipticCurve)
                .await;
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            assert!(
                err.contains("not available") || err.contains("requires iOS")
                    || err.contains("Unsupported platform"),
                "unexpected error: {err}"
            );
        }
    }

    #[tokio::test]
    async fn test_sign_unavailable_on_non_ios() {
        let enclave = SafeSecureEnclave::new().await.unwrap();
        if !cfg!(target_os = "ios") {
            let result = enclave.safe_sign("test_key", b"test data").await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_verify_unavailable_on_non_ios() {
        let enclave = SafeSecureEnclave::new().await.unwrap();
        if !cfg!(target_os = "ios") {
            let result = enclave
                .safe_verify("test_key", b"test data", b"fake_sig")
                .await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_rejects_non_p256_key_types() {
        let enclave = SafeSecureEnclave::new().await.unwrap();
        if !cfg!(target_os = "ios") {
            return;
        }
        let result = enclave
            .safe_generate_key("test", &KeyType::Ed25519)
            .await;
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("EllipticCurve"),
            "should suggest EllipticCurve"
        );
    }
}
