// SPDX-License-Identifier: AGPL-3.0-only

//! Hardware and software HSM tiers, key metadata, and the canonical [`HsmProvider`] trait.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::{HsmKey, KeyMetadata};
use serde::{Deserialize, Serialize};

/// Relative assurance level of an HSM implementation (software through dedicated hardware).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum HsmTier {
    /// Represents software variant (lowest priority)
    Software,
    /// Represents TPM variant (medium priority)
    Tpm,
    /// Represents cloud variant (medium-high priority)
    Cloud,
    /// Represents hybrid variant (high priority)
    Hybrid,
    /// Represents hardware variant (highest priority)
    Hardware,
}

/// Role of a key material record inside the HSM.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// Inventory row returned by [`HsmProvider::list_keys`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    /// Stable key handle for subsequent crypto ops.
    pub key_id: String,
    /// The key type value
    pub key_type: HsmKeyType,
    /// The metadata value
    pub metadata: KeyMetadata,
    /// Number of usage
    pub usage_count: u64,
}

/// Policy constraints applied to a key (allowed ops, size floor, expiry).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyPolicy {
    /// Collection of allowed operations
    pub allowed_operations: Vec<String>,
    /// Optional expiration
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of `minimum_key_size`
    pub minimum_key_size: u32,
}

/// One row from an HSM audit log for a specific key.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyUsageLog {
    /// The operation value
    pub operation: String,
    /// When the operation was invoked.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Principal that triggered the operation.
    pub user_id: String,
    /// The result value
    pub result: String,
}

/// Key lifecycle and crypto operations backed by an HSM or HSM emulator.
///
/// # Migration (v0.10.0)
///
/// Superseded by [`crate::hsm::HsmKeyProvider`] which is object-safe and
/// used with `HsmProviderRegistry` for runtime provider selection.
/// This trait will be removed in a future release.
pub trait HsmProvider: BaseProvider {
    /// Creates a new key of `key_type` with vendor `metadata`.
    fn generate_key(
        key_type: HsmKeyType,
        metadata: KeyMetadata,
    ) -> impl std::future::Future<Output = Result<HsmKey, BearDogError>> + Send;

    /// Signs `data` with the private key identified by `key_id`.
    fn sign_data(
        key_id: &str,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Verifies a signature using the public half referenced by `key_id`.
    fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Encrypts `plaintext` with `key_id`.
    fn encrypt_data(
        key_id: &str,
        plaintext: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Decrypts `ciphertext` with `key_id`.
    fn decrypt_data(
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Lists resident keys and their inventory metadata.
    fn list_keys(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<HsmKeyInfo>, BearDogError>> + Send;

    /// Removes key
    fn delete_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `key_info`
    fn get_key_info(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<HsmKeyInfo, BearDogError>> + Send;

    /// Exports an encrypted backup blob for disaster recovery.
    fn backup_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Imports a key from [`Self::backup_key`] output.
    fn restore_key(
        &self,
        backup_data: &[u8],
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Rotates material while preserving logical `key_id` where supported.
    fn rotate_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<HsmKey, BearDogError>> + Send;

    /// Sets `key_policy`
    fn set_key_policy(
        key_id: &str,
        policy: KeyPolicy,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `key_policy`
    fn get_key_policy(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<KeyPolicy, BearDogError>> + Send;

    /// Returns recent audit lines for compliance review.
    fn audit_key_usage(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<KeyUsageLog>, BearDogError>> + Send;
}
