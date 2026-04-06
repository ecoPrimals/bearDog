// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`crate::tunnel::hsm::manager::HsmProvider`] implementation: crypto and key lifecycle.

use super::super::super::types::KeyType;
use super::super::types::ProtectedMemory;
use super::super::types::*;
use super::RustSoftwareHsm;
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::crypto::{
    CryptoRequirements, DecryptionOptions, EncryptionOptions, SigningOptions, VerificationOptions,
};
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::types::HsmKey;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use beardog_types::hsm::AuditEvent;
use bytes::Bytes;
use chrono::Utc;
use tracing::{debug, info};

#[async_trait::async_trait]
impl HsmProvider for RustSoftwareHsm {
    fn is_available(&self) -> bool {
        true
    }

    async fn get_info(
        &self,
    ) -> Result<crate::tunnel::hsm::manager::implementation::ProviderInfo, BearDogError> {
        Ok(crate::tunnel::hsm::manager::implementation::ProviderInfo {
            id: "Software HSM".to_string(),
            name: "Rust Software HSM".to_string(),
            security_level: 3,
        })
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        self.generate_software_key(&request).await
    }

    async fn import_key(&self, key_data: &[u8], key_id: &str) -> Result<HsmKey, BearDogError> {
        info!("📥 Importing key: {}", key_id);

        let key_type = match key_data.len() {
            16 => KeyType::Aes,
            32 => KeyType::Aes,
            64 => KeyType::Ed25519,
            _ => KeyType::Aes,
        };
        info!(
            "   Key type inferred: {:?} (size: {} bytes)",
            key_type,
            key_data.len()
        );

        let protected_bytes = self.memory_protector.protect(key_data).await?;
        let buf = Bytes::from(protected_bytes);
        let encrypted_blob = buf.to_vec();
        let protected_material = ProtectedMemory::from_bytes(buf, true);

        let id = key_id.to_string();
        let key_metadata = KeyMetadata::new(id.clone(), key_type.clone());

        let software_key = SoftwareKey {
            id: id.clone(),
            key_material: protected_material,
            key_type: key_type.clone(),
            created_at: Utc::now(),
            metadata: key_metadata.clone(),
        };

        let key_store = self.key_store.write().await;
        key_store.store_key(software_key).await?;

        self.audit_logger
            .log_audit_event(AuditEvent::new("import_key"))
            .await?;

        let hsm_key = HsmKey {
            id,
            hsm_type: "SoftwareHsm".to_string(),
            key_type,
            metadata: key_metadata,
            key_material: KeyMaterial::Encrypted {
                encrypted_data: encrypted_blob,
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Key imported successfully: {}", hsm_key.id);
        Ok(hsm_key)
    }

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("🔒 Encrypting data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        let requirements = CryptoRequirements::from_key_metadata(&key.metadata);

        let provider = self.crypto_manager.select_provider(&requirements).await?;

        let mut key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let algorithm = requirements
            .algorithm
            .ok_or_else(|| BearDogError::internal("No algorithm in requirements".to_string()))?
            .as_symmetric()
            .map_err(|e| BearDogError::crypto_error(&e))?;

        let encrypted_data = provider
            .encrypt_symmetric(
                algorithm,
                &key_material,
                plaintext,
                &EncryptionOptions::default(),
            )
            .await?;

        self.memory_protector.zeroize(&mut key_material).await?;

        let mut result = Vec::new();
        if let Some(ref nonce) = encrypted_data.nonce {
            result.extend_from_slice(nonce);
        }
        result.extend_from_slice(&encrypted_data.ciphertext);

        debug!(
            "✅ Data encrypted successfully with {}",
            provider.provider_name()
        );
        Ok(result)
    }

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("🔓 Decrypting data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        let requirements = CryptoRequirements::from_key_metadata(&key.metadata);

        let provider = self.crypto_manager.select_provider(&requirements).await?;

        let mut key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let algorithm = requirements
            .algorithm
            .ok_or_else(|| BearDogError::internal("No algorithm in requirements".to_string()))?
            .as_symmetric()
            .map_err(|e| BearDogError::crypto_error(&e))?;

        use crate::tunnel::hsm::crypto::EncryptedData;
        let (nonce, actual_ciphertext) = if ciphertext.len() >= 12 {
            let (nonce_bytes, ct_bytes) = ciphertext.split_at(12);
            (Some(nonce_bytes.to_vec()), ct_bytes.to_vec())
        } else {
            (None, ciphertext.to_vec())
        };

        let encrypted_data = EncryptedData {
            algorithm: algorithm.to_string(),
            ciphertext: actual_ciphertext,
            nonce,
            tag: None,
        };

        let plaintext = provider
            .decrypt_symmetric(
                algorithm,
                &key_material,
                &encrypted_data,
                &DecryptionOptions::default(),
            )
            .await?;

        self.memory_protector.zeroize(&mut key_material).await?;

        debug!(
            "✅ Data decrypted successfully with {}",
            provider.provider_name()
        );
        Ok(plaintext)
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Signing data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        let requirements = CryptoRequirements::from_key_metadata(&key.metadata);

        let provider = self.crypto_manager.select_provider(&requirements).await?;

        let mut key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let algorithm = requirements
            .algorithm
            .ok_or_else(|| BearDogError::internal("No algorithm in requirements".to_string()))?
            .as_signature()
            .map_err(|e| BearDogError::crypto_error(&e))?;

        let signature = provider
            .sign(algorithm, &key_material, data, &SigningOptions::default())
            .await?;

        self.memory_protector.zeroize(&mut key_material).await?;

        debug!(
            "✅ Data signed successfully with {}",
            provider.provider_name()
        );
        Ok(signature.signature)
    }

    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!("🔍 Verifying signature with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        let requirements = CryptoRequirements::from_key_metadata(&key.metadata);

        let provider = self.crypto_manager.select_provider(&requirements).await?;

        let mut key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let algorithm = requirements
            .algorithm
            .ok_or_else(|| BearDogError::internal("No algorithm in requirements".to_string()))?
            .as_signature()
            .map_err(|e| BearDogError::crypto_error(&e))?;

        use crate::tunnel::hsm::crypto::Signature;
        let sig = Signature {
            algorithm: algorithm.to_string(),
            signature: signature.to_vec(),
        };

        let is_valid = provider
            .verify(
                algorithm,
                &key_material,
                data,
                &sig,
                &VerificationOptions::default(),
            )
            .await?;

        self.memory_protector.zeroize(&mut key_material).await?;

        debug!(
            "✅ Signature verification complete with {}: {}",
            provider.provider_name(),
            is_valid
        );
        Ok(is_valid)
    }

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting software key: {}", key_id);

        let mut key_store = self.key_store.write().await;
        key_store.delete_key(key_id).await?;

        self.audit_logger
            .log_operation(&AuditLogEntry::success("delete_key", key_id))
            .await?;

        info!("✅ Key deleted successfully: {}", key_id);
        Ok(())
    }

    async fn get_key_info(
        &self,
        key_id: &str,
    ) -> Result<crate::tunnel::hsm::manager::implementation::KeyInfo, BearDogError> {
        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        Ok(crate::tunnel::hsm::manager::implementation::KeyInfo {
            key_id: key_id.to_string(),
            key_type: format!("{:?}", key.key_type),
            is_hardware_backed: false,
        })
    }

    async fn health_check(
        &self,
    ) -> Result<crate::tunnel::hsm::manager::implementation::HealthStatus, BearDogError> {
        let health = self.health_monitor.check_health().await?;

        Ok(crate::tunnel::hsm::manager::implementation::HealthStatus {
            is_healthy: health.is_healthy,
            error_message: health.error_message,
        })
    }
}
