// SPDX-License-Identifier: AGPL-3.0-or-later

#![cfg(test)]
#![allow(missing_docs)]

//! Test-only [`super::manager::HsmProvider`] implementations for enum dispatch.
//!
//! This module is gated behind `#[cfg(test)]` both here (inner attribute) and
//! at the `mod` declaration in `hsm/mod.rs` (outer attribute). None of this
//! code is compiled into release builds.

use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::manager::implementation::{HealthStatus, KeyInfo, ProviderInfo};
use crate::tunnel::hsm::types::HsmKey;
use beardog_errors::BearDogError;

// ── manager/tests.rs: MockHsmProvider ────────────────────────────────────────

/// Test-only HSM provider with configurable failure modes.
pub struct MockHsmProvider {
    /// Whether the mock provider reports itself as available.
    pub available: bool,
    /// Whether key generation operations should fail.
    pub fail_generate: bool,
    /// Whether key deletion operations should fail.
    pub fail_delete: bool,
}

impl Default for MockHsmProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MockHsmProvider {
    /// Create a new mock provider in the default (available, non-failing) state.
    pub fn new() -> Self {
        Self {
            available: true,
            fail_generate: false,
            fail_delete: false,
        }
    }

    pub fn unavailable() -> Self {
        Self {
            available: false,
            fail_generate: false,
            fail_delete: false,
        }
    }

    pub fn failing_generate() -> Self {
        Self {
            available: true,
            fail_generate: true,
            fail_delete: false,
        }
    }

    pub fn failing_delete() -> Self {
        Self {
            available: true,
            fail_generate: false,
            fail_delete: true,
        }
    }
}

impl HsmProvider for MockHsmProvider {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock".to_string(),
            name: "Mock HSM".to_string(),
            security_level: 3,
        })
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        use crate::tunnel::hsm::{KeyHealthStatus, KeyMaterial, KeyMetadata};
        use chrono::Utc;

        if self.fail_generate {
            return Err(BearDogError::system(
                "Mock generate_key failure".to_string(),
            ));
        }

        Ok(HsmKey {
            id: request.key_id.clone(),
            hsm_type: "MockHSM".to_string(),
            key_type: request.key_type.clone(),
            metadata: KeyMetadata::new(request.key_id, request.key_type),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: vec![1, 2, 3, 4],
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        })
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn decrypt(&self, _key_id: &str, _ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        if self.fail_delete {
            return Err(BearDogError::system("Mock delete_key failure".to_string()));
        }
        Ok(())
    }

    async fn get_key_info(&self, _key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: "test".to_string(),
            key_type: "AES".to_string(),
            is_hardware_backed: false,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            is_healthy: true,
            error_message: None,
        })
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

// ── implementation.rs tests: DefaultManagerMockProvider ────────────────────

pub struct DefaultManagerMockProvider {
    pub id: String,
}

impl HsmProvider for DefaultManagerMockProvider {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: self.id.clone(),
            name: "Mock Provider".to_string(),
            security_level: 1,
        })
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        use crate::tunnel::hsm::types::key::{
            KeyAttestation, KeyHealthStatus, KeyMaterial, KeyMetadata,
        };
        use chrono::Utc;

        Ok(HsmKey {
            id: request.key_id.clone(),
            hsm_type: "mock".to_string(),
            key_type: request.key_type.clone(),
            metadata: KeyMetadata::new(request.key_id.clone(), request.key_type),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: vec![0u8; 32],
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: Some(KeyAttestation {
                certificate_chain: vec![],
                attestation_statement: vec![],
                format: "mock".to_string(),
                timestamp: Utc::now(),
            }),
            created_at: Utc::now(),
        })
    }

    async fn sign(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data.to_vec())
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn encrypt(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(ciphertext.to_vec())
    }

    async fn import_key(&self, key_data: &[u8], key_id: &str) -> Result<HsmKey, BearDogError> {
        use crate::tunnel::hsm::types::key::{
            KeyAttestation, KeyHealthStatus, KeyMaterial, KeyMetadata, KeyType,
        };
        use chrono::Utc;

        let key_id = key_id.to_string();
        let _key_data = key_data.len();
        Ok(HsmKey {
            id: key_id.clone(),
            hsm_type: "mock".to_string(),
            key_type: KeyType::Ed25519,
            metadata: KeyMetadata::new(key_id.clone(), KeyType::Ed25519),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: vec![0u8; 32],
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: Some(KeyAttestation {
                certificate_chain: vec![],
                attestation_statement: vec![],
                format: "mock".to_string(),
                timestamp: Utc::now(),
            }),
            created_at: Utc::now(),
        })
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type: "Ed25519".to_string(),
            is_hardware_backed: false,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            is_healthy: true,
            error_message: None,
        })
    }

    fn is_available(&self) -> bool {
        true
    }
}

// ── integration tests: hardware / software / cloud mocks ─────────────────────

#[derive(Debug, Clone)]
pub struct MockHardwareHsm {
    pub available: bool,
    pub fail_operations: bool,
}

impl MockHardwareHsm {
    pub fn new(available: bool) -> Self {
        Self {
            available,
            fail_operations: false,
        }
    }

    pub fn with_failures() -> Self {
        Self {
            available: true,
            fail_operations: true,
        }
    }
}

impl HsmProvider for MockHardwareHsm {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock-hardware".to_string(),
            name: "Mock Hardware HSM".to_string(),
            security_level: 5,
        })
    }

    async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Hardware HSM unavailable".to_string(),
            ));
        }
        Err(BearDogError::not_implemented(
            "Mock generate_key for testing",
        ))
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Hardware HSM unavailable".to_string(),
            ));
        }
        Ok(vec![1, 2, 3, 4])
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(!self.fail_operations)
    }

    async fn encrypt(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Hardware HSM unavailable".to_string(),
            ));
        }
        Ok(data.to_vec())
    }

    async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Hardware HSM unavailable".to_string(),
            ));
        }
        Ok(ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key for testing"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type: "hardware".to_string(),
            is_hardware_backed: true,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        let available = self.available;
        let fail = self.fail_operations;
        Ok(HealthStatus {
            is_healthy: available && !fail,
            error_message: if available && !fail {
                None
            } else {
                Some("Hardware HSM not available".to_string())
            },
        })
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

#[derive(Debug, Clone)]
pub struct MockSoftwareHsm;

impl HsmProvider for MockSoftwareHsm {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock-software".to_string(),
            name: "Mock Software HSM".to_string(),
            security_level: 3,
        })
    }

    async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented(
            "Mock generate_key for testing",
        ))
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![5, 6, 7, 8])
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn encrypt(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key for testing"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type: "software".to_string(),
            is_hardware_backed: false,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            is_healthy: true,
            error_message: None,
        })
    }

    fn is_available(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct MockCloudHsm {
    pub available: bool,
    pub fail_operations: bool,
}

impl MockCloudHsm {
    pub fn new(available: bool) -> Self {
        Self {
            available,
            fail_operations: false,
        }
    }

    pub fn with_failures() -> Self {
        Self {
            available: true,
            fail_operations: true,
        }
    }
}

impl HsmProvider for MockCloudHsm {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock-cloud".to_string(),
            name: "Mock Cloud HSM".to_string(),
            security_level: 4,
        })
    }

    async fn generate_key(&self, _request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Err(BearDogError::not_implemented(
            "Mock generate_key for testing",
        ))
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(vec![9, 10, 11, 12])
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(!self.fail_operations)
    }

    async fn encrypt(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(data.to_vec())
    }

    async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if self.fail_operations {
            return Err(BearDogError::unavailable(
                "Cloud HSM unavailable".to_string(),
            ));
        }
        Ok(ciphertext.to_vec())
    }

    async fn import_key(&self, _key_data: &[u8], _key_id: &str) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key for testing"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: key_id.to_string(),
            key_type: "cloud".to_string(),
            is_hardware_backed: true,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        let available = self.available;
        let fail = self.fail_operations;
        Ok(HealthStatus {
            is_healthy: available && !fail,
            error_message: if available && !fail {
                None
            } else {
                Some("Cloud HSM not available".to_string())
            },
        })
    }

    fn is_available(&self) -> bool {
        self.available
    }
}
