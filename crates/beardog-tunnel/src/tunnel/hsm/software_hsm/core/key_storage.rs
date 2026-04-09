// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key generation, derivation, storage, and metadata helpers for [`super::RustSoftwareHsm`].

use super::super::super::types::KeyType;
use super::super::types::{AuditLogEntry, AuditLogger, ProtectedMemory, SoftwareKey};
use super::RustSoftwareHsm;
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::types::{HsmKey, KeyHealthStatus, KeyMaterial, KeyMetadata};
use beardog_errors::BearDogError;
use beardog_types::hsm::HsmAlgorithm;
use bytes::Bytes;
use chrono::Utc;
use tracing::info;

impl RustSoftwareHsm {
    /// Generate a software key
    pub(super) async fn generate_software_key(
        &self,
        request: &GenerateKeyRequest,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔑 Generating software key: {}", request.key_id);

        let key_material = self
            .crypto_provider
            .generate_key_material(&request.key_type)
            .await?;

        let protected_bytes = self.memory_protector.protect(&key_material).await?;
        let buf = Bytes::from(protected_bytes);
        let encrypted_data = buf.to_vec();
        let protected_material = ProtectedMemory::from_bytes(buf, true);

        let key_id = request.key_id.clone();
        let key_type = request.key_type.clone();
        let metadata = KeyMetadata::new(key_id.clone(), key_type.clone());
        let software_key = SoftwareKey {
            id: key_id.clone(),
            key_material: protected_material,
            key_type: key_type.clone(),
            created_at: Utc::now(),
            metadata: metadata.clone(),
        };

        let key_store = self.key_store.write().await;
        key_store.store_key(software_key).await?;

        self.audit_logger
            .log_operation(&AuditLogEntry::success("generate_key", &request.key_id))
            .await?;

        let hsm_key = HsmKey {
            id: key_id,
            hsm_type: "SoftwareHsm".to_string(),
            key_type,
            metadata,
            key_material: KeyMaterial::Encrypted {
                encrypted_data,
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Software key generated successfully: {}", request.key_id);
        Ok(hsm_key)
    }

    /// Derive key from root key using HKDF
    async fn derive_key_from_root(
        &self,
        root_key_id: &str,
        derivation_data: &[u8],
    ) -> Result<HsmKey, BearDogError> {
        info!("🔑 Deriving key from software root key: {}", root_key_id);

        let key_store = self.key_store.read().await;
        let root_key = key_store.get_key(root_key_id).await?;

        let mut root_key_material = self
            .memory_protector
            .unprotect(root_key.key_material.data())
            .await?;

        let derived_material = self
            .crypto_provider
            .derive_key(&root_key_material, derivation_data)
            .await?;

        let protected_bytes = self.memory_protector.protect(&derived_material).await?;
        let buf = Bytes::from(protected_bytes);
        let encrypted_derived_data = buf.to_vec();
        let protected_derived = ProtectedMemory::from_bytes(buf, true);

        self.memory_protector
            .zeroize(&mut root_key_material)
            .await?;

        let derived_key_id = format!(
            "{}_{}",
            root_key_id,
            hex::encode(&derivation_data[..8.min(derivation_data.len())])
        );

        let derived_key_type = root_key.key_type.clone();
        let derived_meta = KeyMetadata::new(derived_key_id.clone(), derived_key_type.clone());
        let derived_key = SoftwareKey {
            id: derived_key_id.clone(),
            key_material: protected_derived,
            key_type: derived_key_type.clone(),
            created_at: Utc::now(),
            metadata: derived_meta.clone(),
        };

        drop(key_store);
        let key_store = self.key_store.write().await;
        key_store.store_key(derived_key).await?;

        let hsm_key = HsmKey {
            id: derived_key_id,
            hsm_type: "SoftwareHsm".to_string(),
            key_type: derived_key_type,
            metadata: derived_meta,
            key_material: KeyMaterial::Encrypted {
                encrypted_data: encrypted_derived_data,
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Key derived successfully: {}", hsm_key.id);
        Ok(hsm_key)
    }

    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    /// Derive a key from a root key
    pub async fn derive_key(
        &self,
        root_key_id: &str,
        derivation_data: &[u8],
    ) -> Result<HsmKey, BearDogError> {
        self.derive_key_from_root(root_key_id, derivation_data)
            .await
    }

    /// Map an [`HsmAlgorithm`] to the internal [`KeyType`] used by the legacy key store.
    pub(super) fn algorithm_to_key_type(algorithm: HsmAlgorithm) -> KeyType {
        match algorithm {
            HsmAlgorithm::Aes256Gcm | HsmAlgorithm::HmacSha256 => KeyType::Aes,
            HsmAlgorithm::ChaCha20Poly1305 => KeyType::ChaCha20,
            HsmAlgorithm::Ed25519 | HsmAlgorithm::X25519 => KeyType::Ed25519,
            HsmAlgorithm::EcdsaP256 | HsmAlgorithm::EcdsaP384 => KeyType::EllipticCurve,
            HsmAlgorithm::Rsa2048 | HsmAlgorithm::Rsa4096 => KeyType::Rsa,
        }
    }
}
