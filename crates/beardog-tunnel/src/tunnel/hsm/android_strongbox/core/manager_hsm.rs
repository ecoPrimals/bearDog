// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration with the tunnel [`HsmManager`](crate::tunnel::hsm::manager::implementation::DefaultHsmManager) (`ManagerHsmProvider`).

use super::AndroidStrongBoxHsm;
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::{KeyGenerationSpec, KeyUsage};
use std::collections::HashMap;
use tracing::info;

use crate::tunnel::hsm::manager::implementation::{
    HealthStatus, HsmProvider as ManagerHsmProvider, KeyInfo as ManagerKeyInfo, ProviderInfo,
};
use crate::tunnel::hsm::types::{HsmKey, KeyHealthStatus, KeyMaterial, KeyMetadata};

/// Implementation of the manager's [`ManagerHsmProvider`] trait for [`AndroidStrongBoxHsm`].
///
/// This allows [`AndroidStrongBoxHsm`] to be registered with [`HsmManager`](crate::tunnel::hsm::manager::implementation::DefaultHsmManager).
impl ManagerHsmProvider for AndroidStrongBoxHsm {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: format!("android-strongbox-{}", self.device_info.model),
            name: "Android StrongBox HSM".to_string(),
            security_level: 5, // StrongBox is highest security level
        })
    }

    async fn generate_key(
        &self,
        request: crate::tunnel::hsm::GenerateKeyRequest,
    ) -> Result<HsmKey, BearDogError> {
        // Convert GenerateKeyRequest to KeyGenerationSpec
        let spec = KeyGenerationSpec {
            key_id: request.key_id,
            key_type: request.key_type,
            key_size: 256, // Default for StrongBox
            key_usage: vec![KeyUsage::Sign, KeyUsage::Verify], // Default
            extractable: false, // StrongBox keys are non-extractable by design
        };

        // Generate the key
        let key_info = self.generate_strongbox_key(&spec).await?;

        // Convert KeyInfo to UniversalKey (HsmKey)
        Ok(HsmKey {
            id: key_info.key_id.clone(),
            hsm_type: "AndroidStrongBox".to_string(),
            key_type: key_info.key_type.clone(),
            metadata: KeyMetadata {
                key_id: key_info.key_id,
                key_type: key_info.key_type,
                alias: None,
                created_at: chrono::Utc::now(),
                expires_at: None,
                tags: HashMap::new(),
            },
            key_material: KeyMaterial::HardwareReference {
                reference: spec.key_id,
                hsm_location: "android_strongbox".to_string(),
            },
            hsm_tier: "production".to_string(),
            created_at: chrono::Utc::now(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
        })
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔐 Signing with StrongBox key: {key_id}");
        self.validate_key_access(key_id)?;
        self.keystore.sign(key_id, data).await
    }

    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!("🔐 Verifying signature with StrongBox key: {key_id}");
        self.validate_key_access(key_id)?;
        self.keystore.verify(key_id, data, signature).await
    }

    async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔐 Encrypting with StrongBox key: {key_id}");
        self.validate_key_access(key_id)?;
        self.keystore.encrypt(key_id, data).await
    }

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔐 Decrypting with StrongBox key: {key_id}");
        self.validate_key_access(key_id)?;
        self.keystore.decrypt(key_id, ciphertext).await
    }

    async fn import_key(
        &self,
        _key_data: &[u8],
        _key_id: &str,
    ) -> Result<HsmKey, BearDogError> {
        // StrongBox doesn't support key import - keys are hardware-generated
        Err(BearDogError::unsupported_operation(
            "Key import not supported in StrongBox - keys are hardware-generated",
        ))
    }

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting StrongBox key: {key_id}");
        self.keystore.delete_key(key_id).await
    }

    async fn get_key_info(&self, key_id: &str) -> Result<ManagerKeyInfo, BearDogError> {
        let cache = self.key_cache.read().await;
        if let Some(cached) = cache.get(key_id) {
            // Use ManagerKeyInfo (from manager::implementation) which has: key_id, key_type (String), is_hardware_backed
            Ok(ManagerKeyInfo {
                key_id: key_id.to_string(),
                key_type: format!("{:?}", cached.key_type), // Convert KeyType enum to String
                is_hardware_backed: self.keystore.is_strongbox_available(),
            })
        } else {
            Err(BearDogError::not_found(format!("Key not found: {key_id}")))
        }
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        let health_status = self.health_monitor.get_health_status().await;
        let is_healthy = self.keystore.is_strongbox_available()
            && health_status.map(|h| h.is_healthy).unwrap_or(false);

        Ok(HealthStatus {
            is_healthy,
            error_message: if is_healthy {
                None
            } else {
                Some("StrongBox HSM unhealthy".to_string())
            },
        })
    }

    fn is_available(&self) -> bool {
        self.keystore.is_strongbox_available()
    }
}
