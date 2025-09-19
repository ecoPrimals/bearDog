// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
// async_trait no longer needed - using native fn
use beardog_errors::BearDogError;

// Local definition of HsmTier since it's not available in beardog_types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HsmTier {
    /// Represents hardware variant
    Hardware,
    /// Represents software variant
    Software,
    /// Represents cloud variant
    Cloud,
    /// Represents hybrid variant
    Hybrid,
}
use beardog_types::canonical::hsm::{HsmKey, KeyMetadata};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Types of hsm key
pub enum HsmKeyType {
    /// Represents symmetric variant
    Symmetric,
    /// Represents asymmetric public variant
    AsymmetricPublic,
    /// Represents asymmetric private variant
    AsymmetricPrivate,
    /// Currently signing
    Signing,
    /// Represents encryption variant
    Encryption,
}

// Define HsmKeyInfo locally since it's not available in the unified system
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    pub key_id: String,
    /// The key type value
    pub key_type: HsmKeyType,
    /// The metadata value
    pub metadata: KeyMetadata,
    /// Number of usage
    pub usage_count: u64,
}

// Additional canonical HSM types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyPolicy {
    /// Collection of allowed operations
    pub allowed_operations: Vec<String>,
    /// Optional expiration
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of minimum_key_size
    pub minimum_key_size: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyUsageLog {
    /// The operation value
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    /// The result value
    pub result: String,
}

pub trait HsmProvider: BaseProvider {
    fn generate_key(
        key_type: HsmKeyType,
        metadata: KeyMetadata,
    ) -> impl std::future::Future<Output = Result<HsmKey, BearDogError>> + Send;

    fn sign_data(
        key_id: &str,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    fn encrypt_data(
        key_id: &str,
        plaintext: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn decrypt_data(
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn list_keys(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<HsmKeyInfo>, BearDogError>> + Send;

    /// Removes key
    fn delete_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets key_info
    fn get_key_info(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<HsmKeyInfo, BearDogError>> + Send;

    fn backup_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    fn restore_key(
        &self,
        backup_data: &[u8],
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn rotate_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<HsmKey, BearDogError>> + Send;

    /// Sets key_policy
    fn set_key_policy(
        key_id: &str,
        policy: KeyPolicy,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets key_policy
    fn get_key_policy(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<KeyPolicy, BearDogError>> + Send;

    fn audit_key_usage(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<KeyUsageLog>, BearDogError>> + Send;
}
