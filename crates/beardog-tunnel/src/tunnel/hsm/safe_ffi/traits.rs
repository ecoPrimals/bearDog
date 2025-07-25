//! Safe Hardware Provider Traits
//!
//! Defines common traits for safe hardware security providers

use async_trait::async_trait;
use beardog_errors::BearDogResult;
use crate::tunnel::hsm::types::{KeyType, HsmKey};

/// Safe hardware security provider trait
#[async_trait]
pub trait SafeHardwareProvider: Send + Sync {
    /// Generate a key safely
    async fn generate_key(&self, key_id: &str, key_type: &KeyType) -> BearDogResult<HsmKey>;

    /// Sign data safely
    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Verify signature safely
    async fn verify_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool>;

    /// Check if this provider is hardware-backed
    fn is_hardware_backed(&self) -> bool;
} 