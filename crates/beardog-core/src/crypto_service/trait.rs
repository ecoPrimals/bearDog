// SPDX-License-Identifier: AGPL-3.0-only

//! Protocol-agnostic `CryptoService` trait
//!
//! This trait defines all cryptographic operations in a transport-neutral way.
//! Implementations can expose these via any protocol (HTTP, JSON-RPC, tarpc, gRPC, etc.).
//!
//! ## Design Philosophy
//!
//! - **Async-first**: All operations are async for scalability
//! - **Owned types**: Avoids lifetime complexity across async boundaries  
//! - **Result-based**: All operations can fail gracefully
//! - **Auditable**: Operations include context for audit trails
//! - **Zero unsafe**: Pure safe Rust

use async_trait::async_trait;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, EncryptedData, HealthStatus, KeyAlgorithm,
    KeyGenOptions, KeyInfo, ServiceCapabilities, SignOptions, Signature, SignatureAlgorithm,
    VerifyOptions,
};

use super::Result;

/// Protocol-agnostic cryptographic service operations
///
/// This trait defines the contract for all crypto operations.
/// It is deliberately protocol-agnostic - no HTTP status codes,
/// no RPC-specific types, just pure crypto operations.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` to support concurrent access.
///
/// # Error Handling
///
/// All operations return `Result<T>` with `BearDogError` for consistent error handling
/// across protocols. Protocol adapters can map these to protocol-specific errors.
#[async_trait]
pub trait CryptoService: Send + Sync {
    /// Encrypt data using specified algorithm
    ///
    /// # Arguments
    ///
    /// * `data` - Plaintext data to encrypt
    /// * `algorithm` - Encryption algorithm (AES-256-GCM, ChaCha20-Poly1305, etc.)
    /// * `options` - Encryption options (`key_id`, AAD, context, etc.)
    ///
    /// # Returns
    ///
    /// Encrypted data with metadata (nonce, tag, timestamp, algorithm used)
    ///
    /// # Errors
    ///
    /// - Key not found
    /// - Invalid algorithm for key type
    /// - Data too large
    /// - HSM communication error
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: CryptoAlgorithm,
        options: EncryptOptions,
    ) -> Result<EncryptedData>;

    /// Decrypt previously encrypted data
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted data with metadata
    /// * `options` - Decryption options (`key_id`, AAD, etc.)
    ///
    /// # Returns
    ///
    /// Original plaintext data
    ///
    /// # Errors
    ///
    /// - Key not found
    /// - Authentication failed (wrong key or tampered data)
    /// - Invalid nonce or tag
    async fn decrypt(&self, encrypted: &EncryptedData, options: DecryptOptions) -> Result<Vec<u8>>;

    /// Sign data using specified signature algorithm
    ///
    /// # Arguments
    ///
    /// * `data` - Data to sign
    /// * `algorithm` - Signature algorithm (Ed25519, ECDSA-P256, etc.)
    /// * `options` - Signing options (`key_id`, context, etc.)
    ///
    /// # Returns
    ///
    /// Digital signature with metadata
    ///
    /// # Errors
    ///
    /// - Key not found
    /// - Invalid algorithm for key type
    /// - HSM signing error
    async fn sign(
        &self,
        data: &[u8],
        algorithm: SignatureAlgorithm,
        options: SignOptions,
    ) -> Result<Signature>;

    /// Verify a digital signature
    ///
    /// # Arguments
    ///
    /// * `data` - Original data that was signed
    /// * `signature` - Signature to verify
    /// * `options` - Verification options (public key, context, etc.)
    ///
    /// # Returns
    ///
    /// `true` if signature is valid, `false` otherwise
    ///
    /// # Errors
    ///
    /// - Invalid public key format
    /// - Invalid signature format
    /// - Algorithm mismatch
    async fn verify(
        &self,
        data: &[u8],
        signature: &Signature,
        options: VerifyOptions,
    ) -> Result<bool>;

    /// Generate a new cryptographic key
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Key algorithm (AES-256, Ed25519, ECDSA-P256, etc.)
    /// * `options` - Key generation options (size, HSM, genetic mixing, etc.)
    ///
    /// # Returns
    ///
    /// Key metadata (NOT the key material itself - security best practice)
    ///
    /// # Security Note
    ///
    /// This method returns metadata only. The actual key material is stored
    /// securely and never exposed directly.
    ///
    /// # Errors
    ///
    /// - Unsupported algorithm
    /// - HSM not available (if required)
    /// - Insufficient entropy
    async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        options: KeyGenOptions,
    ) -> Result<KeyInfo>;

    /// Get service capabilities
    ///
    /// Returns what algorithms, features, and HSMs this service supports.
    /// Used for protocol discovery, capability negotiation, and client adaptation.
    ///
    /// # Capability-Based Discovery
    ///
    /// This enables primals to discover each other's capabilities at runtime
    /// without hardcoding assumptions about what algorithms are available.
    ///
    /// # Returns
    ///
    /// Service capabilities including:
    /// - Supported algorithms (discovered at runtime)
    /// - Available HSM providers
    /// - Feature flags (genetic mixing, audit logging, etc.)
    /// - Performance characteristics
    async fn get_capabilities(&self) -> Result<ServiceCapabilities>;

    /// Get service health status
    ///
    /// Returns current health metrics for monitoring and operations.
    ///
    /// # Returns
    ///
    /// Health status including:
    /// - Service state (healthy, degraded, unhealthy)
    /// - HSM connection status
    /// - Key count and storage
    /// - Operation statistics
    /// - Recent errors
    async fn get_health(&self) -> Result<HealthStatus>;
}
