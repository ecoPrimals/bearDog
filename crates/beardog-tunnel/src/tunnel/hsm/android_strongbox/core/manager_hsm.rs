// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration with the tunnel `HsmManager` (`ManagerHsmProvider`).

use super::AndroidStrongBoxHsm;
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::{KeyGenerationSpec, KeyUsage};
use std::collections::HashMap;
use std::future::Future;
use tracing::info;

use crate::tunnel::hsm::manager::implementation::{
    HealthStatus, HsmProvider as ManagerHsmProvider, KeyInfo as ManagerKeyInfo, ProviderInfo,
};
use crate::tunnel::hsm::types::{HsmKey, KeyHealthStatus, KeyMaterial, KeyMetadata};

/// Implementation of the manager's HsmProvider trait for AndroidStrongBoxHsm
/// This allows AndroidStrongBoxHsm to be registered with HsmManager
impl ManagerHsmProvider for AndroidStrongBoxHsm {
    fn get_info(&self) -> impl Future<Output = Result<ProviderInfo, BearDogError>> + Send {
        let this = self.clone();
        async move {
            Ok(ProviderInfo {
                id: format!("android-strongbox-{}", this.device_info.model),
                name: "Android StrongBox HSM".to_string(),
                security_level: 5, // StrongBox is highest security level
            })
        }
    }

    fn generate_key(
        &self,
        request: crate::tunnel::hsm::GenerateKeyRequest,
    ) -> impl Future<Output = Result<HsmKey, BearDogError>> + Send {
        let this = self.clone();
        async move {
            // Convert GenerateKeyRequest to KeyGenerationSpec
            let spec = KeyGenerationSpec {
                key_id: request.key_id,
                key_type: request.key_type,
                key_size: 256, // Default for StrongBox
                key_usage: vec![KeyUsage::Sign, KeyUsage::Verify], // Default
                extractable: false, // StrongBox keys are non-extractable by design
            };

            // Generate the key
            let key_info = this.generate_strongbox_key(&spec).await?;

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
    }

    fn sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        let data = data.to_vec();
        async move {
            info!("🔐 Signing with StrongBox key: {}", key_id);
            this.validate_key_access(&key_id)?;
            this.keystore.sign(&key_id, &data).await
        }
    }

    fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        let data = data.to_vec();
        let signature = signature.to_vec();
        async move {
            info!("🔐 Verifying signature with StrongBox key: {}", key_id);
            this.validate_key_access(&key_id)?;
            this.keystore.verify(&key_id, &data, &signature).await
        }
    }

    fn encrypt(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        let data = data.to_vec();
        async move {
            info!("🔐 Encrypting with StrongBox key: {}", key_id);
            this.validate_key_access(&key_id)?;
            this.keystore.encrypt(&key_id, &data).await
        }
    }

    fn decrypt(
        &self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        let ciphertext = ciphertext.to_vec();
        async move {
            info!("🔐 Decrypting with StrongBox key: {}", key_id);
            this.validate_key_access(&key_id)?;
            this.keystore.decrypt(&key_id, &ciphertext).await
        }
    }

    fn import_key(
        &self,
        key_data: &[u8],
        key_id: &str,
    ) -> impl Future<Output = Result<HsmKey, BearDogError>> + Send {
        let _key_data = key_data.to_vec();
        let _key_id = key_id.to_string();
        async move {
            // StrongBox doesn't support key import - keys are hardware-generated
            Err(BearDogError::unsupported_operation(
                "Key import not supported in StrongBox - keys are hardware-generated",
            ))
        }
    }

    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        async move {
            info!("🗑️ Deleting StrongBox key: {}", key_id);
            this.keystore.delete_key(&key_id).await
        }
    }

    fn get_key_info(
        &self,
        key_id: &str,
    ) -> impl Future<Output = Result<ManagerKeyInfo, BearDogError>> + Send {
        let this = self.clone();
        let key_id = key_id.to_string();
        async move {
            let cache = this.key_cache.read().await;
            if let Some(cached) = cache.get(&key_id) {
                // Use ManagerKeyInfo (from manager::implementation) which has: key_id, key_type (String), is_hardware_backed
                Ok(ManagerKeyInfo {
                    key_id: key_id.clone(),
                    key_type: format!("{:?}", cached.key_type), // Convert KeyType enum to String
                    is_hardware_backed: true, // StrongBox is always hardware-backed
                })
            } else {
                Err(BearDogError::not_found(format!(
                    "Key not found: {}",
                    key_id
                )))
            }
        }
    }

    fn health_check(&self) -> impl Future<Output = Result<HealthStatus, BearDogError>> + Send {
        let this = self.clone();
        async move {
            let is_healthy =
                this.keystore.is_strongbox_available() && this.health_monitor.is_healthy().await;

            Ok(HealthStatus {
                is_healthy,
                error_message: if is_healthy {
                    None
                } else {
                    Some("StrongBox HSM unhealthy".to_string())
                },
            })
        }
    }

    fn is_available(&self) -> bool {
        self.keystore.is_strongbox_available()
    }
}
