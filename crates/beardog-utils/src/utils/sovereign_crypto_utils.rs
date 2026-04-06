// SPDX-License-Identifier: AGPL-3.0-or-later

// Sovereign Crypto Utils
//
// This module replaces traditional crypto utilities with human-owned entropy.
// Instead of machine-only randomness, all cryptographic operations now use
// BearDog's sovereign entropy hierarchy, giving humans ownership over their
// cryptographic randomness.

use beardog_core::migration::{MigrationPhase, SovereignEntropyMigrationManager};
use beardog_errors::BearDogError;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};


type HmacSha256 = Hmac<Sha256>;

/// Sovereign crypto utilities with human-owned entropy
pub struct SovereignCryptoUtils {
    migration_manager: Arc<RwLock<SovereignEntropyMigrationManager>>,
}

impl SovereignCryptoUtils {
    /// Create new sovereign crypto utilities
    /// Creates a new instance
    pub async fn new(migration_manager: Arc<RwLock<SovereignEntropyMigrationManager>>) -> Self {
        info!("🔐 Initializing Sovereign Crypto Utils");
        info!("🎯 Mission: Replace machine randomness with human-owned entropy");

        Self { migration_manager }
    }

    /// Generate cryptographically secure random bytes using human-owned entropy
    pub fn secure_random_bytes(
        &self,
        size: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "🎲 Generating {} secure random bytes with sovereign entropy",
            size
        );

        let manager = self.migration_manager.read();
        manager
            .migrate_crypto_key_generation("secure_random_bytes", size, human_identity)
    }

    /// Generate cryptographic salt using human-owned entropy
    pub fn generate_salt(
        &self,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        info!("🧂 Generating 256-bit cryptographic salt with sovereign entropy");

        let manager = self.migration_manager.read();
        manager
            .migrate_crypto_key_generation(
                "salt_generation",
                32, // 256-bit salt
                human_identity,
            )
    }

    /// Generate cryptographic nonce using human-owned entropy
    pub fn generate_nonce(
        &self,
        size: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "🔢 Generating {}-byte cryptographic nonce with sovereign entropy",
            size
        );

        let manager = self.migration_manager.read();
        manager
            .migrate_random_data_generation("nonce_generation", size, human_identity)
    }

    /// Generate encryption key using human-owned entropy
    pub fn generate_encryption_key(
        &self,
        key_size_bytes: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "🔑 Generating {}-byte encryption key with sovereign entropy",
            key_size_bytes
        );

        let manager = self.migration_manager.read();
        manager
            .migrate_crypto_key_generation("encryption_keys", key_size_bytes, human_identity)
    }

    /// Generate signing key using human-owned entropy
    pub fn generate_signing_key(
        &self,
        key_size_bytes: usize,
        human_identity: Option<&str>,
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "✍️  Generating {}-byte signing key with sovereign entropy",
            key_size_bytes
        );

        let manager = self.migration_manager.read();
        manager
            .migrate_crypto_key_generation("signature_generation", key_size_bytes, human_identity)
    }

    /// SHA-256 hash (deterministic, no randomness needed)
    /// 
    /// DEPRECATED: Use beardog_security::crypto_utils::BearDogCrypto::sha256_hash instead
    #[deprecated(
        since = "3.0.1",
        note = "Use beardog_security::crypto_utils::BearDogCrypto::sha256_hash instead. \
                Crypto functions should be in beardog-security crate. Removal planned for v3.3.0 (Q1 2026)."
    )]
    pub fn sha256_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        bytes_to_hex(&hasher.finalize())
    }

    /// HMAC-SHA256 with existing key (deterministic)
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<String, BearDogError> {
        let mut mac = HmacSha256::new_from_slice(key).map_err(|e| BearDogError::Crypto {
            message: format!("Invalid HMAC key: {e}"),
        })?;
        mac.update(data);
        Ok(bytes_to_hex(&mac.finalize().into_bytes()))
    }

    /// Verify HMAC-SHA256 signature (deterministic)
    pub fn verify_hmac_sha256(
        key: &[u8],
        data: &[u8],
        signature: &str,
    ) -> Result<bool, BearDogError> {
        let computed = Self::hmac_sha256(key, data)?;
        Ok(constant_time_compare(
            computed.as_bytes(),
            signature.as_bytes(),
        ))
    }

    /// Generate secure password using human-owned entropy
    pub fn generate_password(
        &self,
        length: usize,
        human_identity: Option<&str>,
    ) -> Result<String, BearDogError> {
        info!(
            "🔐 Generating {}-character password with sovereign entropy",
            length
        );

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";

        let random_bytes = self.secure_random_bytes(length, human_identity)?;
        let mut password = String::with_capacity(length);

        for byte in random_bytes {
            let idx = (byte as usize) % CHARSET.len();
            password.push(CHARSET[idx] as char);
        }

        Ok(password)
    }

    /// Generate UUID using human-owned entropy
    pub fn generate_uuid(
        &self,
        human_identity: Option<&str>,
    ) -> Result<String, BearDogError> {
        info!("🆔 Generating UUID with sovereign entropy");

        let random_bytes = self.secure_random_bytes(16, human_identity)?;

        // Format as UUID v4
        let uuid = format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-4{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            random_bytes[0], random_bytes[1], random_bytes[2], random_bytes[3],
            random_bytes[4], random_bytes[5],
            random_bytes[6] & 0x0f, random_bytes[7],
            (random_bytes[8] & 0x3f) | 0x80, random_bytes[9],
            random_bytes[10], random_bytes[11], random_bytes[12], random_bytes[13], random_bytes[14], random_bytes[15]
        );

        Ok(uuid)
    }

    /// Generate session token using human-owned entropy
    pub fn generate_session_token(
        &self,
        token_length: usize,
        human_identity: Option<&str>,
    ) -> Result<String, BearDogError> {
        info!("🎫 Generating session token with sovereign entropy");

        let random_bytes = self
            .secure_random_bytes(token_length, human_identity)
            ?;
        Ok(base64::encode(random_bytes))
    }

    /// Get migration statistics
    /// Gets migration_statistics
    pub fn get_migration_statistics(
        &self,
    ) -> Result<beardog_core::migration::MigrationStatistics, BearDogError> {
        let manager = self.migration_manager.read();
        Ok(manager.get_migration_statistics())
    }
}

/// Constant-time comparison to prevent timing attacks (deterministic)
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        diff |= byte_a ^ byte_b;
    }
    diff == 0
}

/// Convert bytes to hexadecimal string (deterministic)
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// Legacy module removed Nov 11, 2025 - Zero active usage confirmed
// Migration: Use SovereignCryptoUtils methods directly
