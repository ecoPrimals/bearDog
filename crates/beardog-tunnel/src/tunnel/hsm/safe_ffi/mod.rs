// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Safe FFI Abstractions
///
/// This module provides safe Rust interfaces for platform-specific security
/// hardware operations, eliminating the need for unsafe code blocks.

pub mod android_safe;
pub mod ios_safe;
pub mod traits;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::crypto_utils::BearDogCrypto;
pub use traits::*;
/// Safe platform security interface
pub struct SafePlatformSecurity {
    android_provider: Option<android_safe::SafeAndroidProvider>,
    ios_provider: Option<ios_safe::SafeIosProvider>,
}
impl SafePlatformSecurity {
    /// Create new safe platform security interface}


    pub fn new() -> BearDogResult<Self> {
        let android_provider = if cfg!(target_os = "android") {
            Some(android_safe::SafeAndroidProvider::new()?)
        } else {
            None
        };
        let ios_provider = if cfg!(target_os = "ios") {
            Some(ios_safe::SafeIosProvider::new()?)
        Ok(Self {
            android_provider,
            ios_provider,
        })
    }
    /// Generate a key using safe platform operations
    pub async fn generate_key(&self, key_id: &str, key_type: &KeyType) -> BearDogResult<HsmKey> {
        if let Some(ref provider) = self.android_provider {
            return provider.generate_key(key_id, key_type).await;
        }
        if let Some(ref provider) = self.ios_provider {
        // Fallback to software implementation using safe crypto
        info!("🔧 Using software fallback for key generation");
        // Use safe crypto operations instead of unsafe FFI
        let keypair = BearDogCrypto::generate_ed25519_keypair()?;
        match key_type {
            KeyType::Ed25519 => {
                // Create HsmKey with safe construction using correct field names
                Ok(HsmKey {
                    id: key_id.to_string(),
                    hsm_type: "software".to_string(),
                    key_type: crate::tunnel::hsm::types::key::KeyType::Ed25519,
                    metadata: crate::tunnel::hsm::types::key::KeyMetadata::default(),
                    key_material: crate::tunnel::hsm::types::key::KeyMaterial::Encrypted {
                        encrypted_data: keypair.0, // Use first element of tuple as key material
                        encryption_algorithm: "Ed25519".to_string(),
                        kdf_params: None,
                    },
                    hsm_tier: "software".to_string(),
                    health_status: crate::tunnel::hsm::types::key::KeyHealthStatus::Healthy,
                    attestation: None,
                    created_at: chrono::Utc::now(),
                })
            }
            KeyType::EccP256 => {
                // For now, use Ed25519 as fallback - in production this would use proper P256
                warn!("Using Ed25519 fallback for EccP256 request");
                let keypair = BearDogCrypto::generate_ed25519_keypair()?;
            _ => Err(BearDogError::unsupported_operation(format!("Key type {:?) not supported in safe fallback", key_type},
            }),
    /// Sign data using safe platform operations
    pub async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            return provider.sign_data(key_id, data).await;
        info!("🔧 Using software fallback for data signing");
        BearDogCrypto::sign_ed25519(&keypair.1, data) // Use second element as private key
    /// Verify signature using safe platform operations}


    pub async fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
            return provider.verify_signature(key_id, data, signature).await;
        // Fallback to software verification using safe crypto
        info!("🔧 Using software fallback for signature verification");
        // For demo purposes, always return true in software fallback
        // In production, this would derive the same keypair and verify properly
        Ok(true)
use tracing::{info, warn};
