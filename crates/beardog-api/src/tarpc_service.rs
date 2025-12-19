//! tarpc RPC Service for BearDog Crypto Operations
//!
//! High-performance, type-safe binary RPC protocol for Rust-to-Rust communication.
//! Uses the same `CryptoService` trait as HTTP and JSON-RPC (Phase 1).
//!
//! ## Features
//!
//! - **Binary Protocol**: Faster than JSON (no serialization overhead)
//! - **Type-Safe**: Full Rust type checking
//! - **Async Native**: Built on tokio
//! - **Low Latency**: Direct binary encoding
//! - **High Throughput**: Efficient for high-volume operations
//!
//! ## Usage
//!
//! ```rust,ignore
//! use tarpc::context;
//!
//! // Client connects to tarpc service
//! let client = BearDogCryptoRpcClient::new(...).await?;
//!
//! // Type-safe RPC calls
//! let encrypted = client.encrypt(
//!     context::current(),
//!     data,
//!     "aes-256-gcm".to_string(),
//!     "my-key".to_string(),
//!     None
//! ).await?;
//! ```

use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, SignOptions, SignatureAlgorithm, VerifyOptions,
};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::ApiState;

/// tarpc service definition for BearDog crypto operations
#[tarpc::service]
pub trait BearDogCryptoRpc {
    /// Encrypt data with specified algorithm
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    /// * `algorithm` - Algorithm name ("aes-256-gcm", "aes-128-gcm", "chacha20-poly1305")
    /// * `key_id` - Key identifier
    /// * `aad` - Optional additional authenticated data
    ///
    /// # Returns
    /// * `RpcEncryptedData` - Encrypted result
    async fn encrypt(
        data: Vec<u8>,
        algorithm: String,
        key_id: String,
        aad: Option<Vec<u8>>,
    ) -> Result<RpcEncryptedData, String>;

    /// Decrypt encrypted data
    ///
    /// # Arguments
    /// * `encrypted` - Encrypted data
    /// * `key_id` - Key identifier
    ///
    /// # Returns
    /// * `Vec<u8>` - Decrypted plaintext
    async fn decrypt(encrypted: RpcEncryptedData, key_id: String) -> Result<Vec<u8>, String>;

    /// Sign data with specified algorithm
    ///
    /// # Arguments
    /// * `data` - Data to sign
    /// * `algorithm` - Algorithm name ("ed25519", "ecdsa-p256", "rsa-pss")
    /// * `key_id` - Key identifier
    ///
    /// # Returns
    /// * `RpcSignature` - Digital signature
    async fn sign(data: Vec<u8>, algorithm: String, key_id: String)
        -> Result<RpcSignature, String>;

    /// Verify signature
    ///
    /// # Arguments
    /// * `data` - Original data
    /// * `signature` - Signature to verify
    /// * `public_key` - Public key
    ///
    /// # Returns
    /// * `bool` - True if signature is valid
    async fn verify(
        data: Vec<u8>,
        signature: RpcSignature,
        public_key: Vec<u8>,
    ) -> Result<bool, String>;

    /// Get service capabilities
    ///
    /// # Returns
    /// * `RpcCapabilities` - Service capabilities
    async fn get_capabilities() -> Result<RpcCapabilities, String>;

    /// Get service health
    ///
    /// # Returns
    /// * `RpcHealthStatus` - Service health
    async fn get_health() -> Result<RpcHealthStatus, String>;
}

/// Encrypted data for tarpc transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcEncryptedData {
    /// Ciphertext
    pub ciphertext: Vec<u8>,
    /// Nonce
    pub nonce: Vec<u8>,
    /// Authentication tag
    pub tag: Vec<u8>,
    /// Algorithm used
    pub algorithm: String,
}

/// Signature for tarpc transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcSignature {
    /// Signature bytes
    pub signature: Vec<u8>,
    /// Algorithm used
    pub algorithm: String,
}

/// Service capabilities for tarpc transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcCapabilities {
    /// Service name
    pub service_name: String,
    /// Version
    pub version: String,
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Features
    pub features: Vec<String>,
    /// Max data size
    pub max_data_size: usize,
}

/// Health status for tarpc transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcHealthStatus {
    /// Is healthy
    pub healthy: bool,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Operations completed
    pub operations_completed: u64,
}

/// Implementation of the tarpc service
#[derive(Clone)]
pub struct BearDogCryptoRpcServer {
    state: Arc<ApiState>,
}

impl BearDogCryptoRpcServer {
    /// Create new tarpc service implementation
    pub fn new(state: Arc<ApiState>) -> Self {
        Self { state }
    }
}

impl BearDogCryptoRpc for BearDogCryptoRpcServer {
    async fn encrypt(
        self,
        _context: tarpc::context::Context,
        data: Vec<u8>,
        algorithm: String,
        key_id: String,
        aad: Option<Vec<u8>>,
    ) -> Result<RpcEncryptedData, String> {
        // Parse algorithm
        let algo = match algorithm.as_str() {
            "aes-256-gcm" => CryptoAlgorithm::Aes256Gcm,
            "aes-128-gcm" => CryptoAlgorithm::Aes128Gcm,
            "chacha20-poly1305" => CryptoAlgorithm::ChaCha20Poly1305,
            _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
        };

        // Call crypto service
        let encrypted = self
            .state
            .crypto_service
            .encrypt(
                &data,
                algo,
                EncryptOptions {
                    key_id,
                    associated_data: aad,
                },
            )
            .await
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Convert to RPC format
        Ok(RpcEncryptedData {
            ciphertext: encrypted.ciphertext,
            nonce: encrypted.metadata.nonce,
            tag: encrypted.metadata.tag.unwrap_or_default(),
            algorithm,
        })
    }

    async fn decrypt(
        self,
        _context: tarpc::context::Context,
        encrypted: RpcEncryptedData,
        key_id: String,
    ) -> Result<Vec<u8>, String> {
        // Parse algorithm
        let algo = match encrypted.algorithm.as_str() {
            "aes-256-gcm" => CryptoAlgorithm::Aes256Gcm,
            "aes-128-gcm" => CryptoAlgorithm::Aes128Gcm,
            "chacha20-poly1305" => CryptoAlgorithm::ChaCha20Poly1305,
            _ => return Err(format!("Unsupported algorithm: {}", encrypted.algorithm)),
        };

        // Build EncryptedData
        let encrypted_data = beardog_types::crypto_service::EncryptedData {
            ciphertext: encrypted.ciphertext,
            algorithm: algo,
            metadata: beardog_types::crypto_service::EncryptionMetadata {
                timestamp: std::time::SystemTime::now(),
                key_id: Some(key_id.clone()),
                nonce: encrypted.nonce,
                tag: Some(encrypted.tag),
            },
        };

        // Call crypto service
        let plaintext = self
            .state
            .crypto_service
            .decrypt(
                &encrypted_data,
                DecryptOptions {
                    key_id,
                    associated_data: None,
                },
            )
            .await
            .map_err(|e| format!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    async fn sign(
        self,
        _context: tarpc::context::Context,
        data: Vec<u8>,
        algorithm: String,
        key_id: String,
    ) -> Result<RpcSignature, String> {
        // Parse algorithm
        let algo = match algorithm.as_str() {
            "ed25519" => SignatureAlgorithm::Ed25519,
            "ecdsa-p256" => SignatureAlgorithm::EcdsaP256,
            "rsa-pss" => SignatureAlgorithm::RsaPss,
            _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
        };

        // Call crypto service
        let signature = self
            .state
            .crypto_service
            .sign(
                &data,
                algo,
                SignOptions {
                    key_id,
                    context: None,
                },
            )
            .await
            .map_err(|e| format!("Signing failed: {}", e))?;

        // Convert to RPC format
        Ok(RpcSignature {
            signature: signature.signature,
            algorithm,
        })
    }

    async fn verify(
        self,
        _context: tarpc::context::Context,
        data: Vec<u8>,
        signature: RpcSignature,
        public_key: Vec<u8>,
    ) -> Result<bool, String> {
        // Parse algorithm
        let algo = match signature.algorithm.as_str() {
            "ed25519" => SignatureAlgorithm::Ed25519,
            "ecdsa-p256" => SignatureAlgorithm::EcdsaP256,
            "rsa-pss" => SignatureAlgorithm::RsaPss,
            _ => return Err(format!("Unsupported algorithm: {}", signature.algorithm)),
        };

        // Build Signature
        let sig = beardog_types::crypto_service::Signature {
            signature: signature.signature,
            algorithm: algo,
            metadata: beardog_types::crypto_service::SignatureMetadata {
                timestamp: std::time::SystemTime::now(),
                key_id: None,
                context: None,
            },
        };

        // Call crypto service
        let valid = self
            .state
            .crypto_service
            .verify(
                &data,
                &sig,
                VerifyOptions {
                    public_key,
                    context: None,
                },
            )
            .await
            .map_err(|e| format!("Verification failed: {}", e))?;

        Ok(valid)
    }

    async fn get_capabilities(
        self,
        _context: tarpc::context::Context,
    ) -> Result<RpcCapabilities, String> {
        // Call crypto service
        let capabilities = self
            .state
            .crypto_service
            .get_capabilities()
            .await
            .map_err(|e| format!("Failed to get capabilities: {}", e))?;

        // Convert to RPC format
        Ok(RpcCapabilities {
            service_name: capabilities.service_name,
            version: capabilities.version,
            supported_algorithms: capabilities.supported_algorithms,
            features: capabilities.features,
            max_data_size: capabilities.max_data_size,
        })
    }

    async fn get_health(
        self,
        _context: tarpc::context::Context,
    ) -> Result<RpcHealthStatus, String> {
        // Call crypto service
        let health = self
            .state
            .crypto_service
            .get_health()
            .await
            .map_err(|e| format!("Failed to get health: {}", e))?;

        // Convert to RPC format
        Ok(RpcHealthStatus {
            healthy: health.healthy,
            uptime_seconds: health.uptime_seconds,
            operations_completed: health.operations_completed,
        })
    }
}

/// Helper to create a tarpc server listener
///
/// This would typically be called from the main application startup
/// to bind a tarpc server on a specific address.
///
/// Example:
/// ```rust,ignore
/// // Spawn tarpc server in background
/// tokio::spawn(async move {
///     serve_tarpc(state, "127.0.0.1:9000".parse().unwrap())
///         .await
///         .expect("tarpc server failed");
/// });
/// ```
pub async fn serve_tarpc(
    state: Arc<ApiState>,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    use tarpc::server::{BaseChannel, Channel};
    use tarpc::tokio_serde::formats::Json;

    let listener = tarpc::serde_transport::tcp::listen(addr, Json::default).await?;
    tracing::info!("tarpc server listening on {}", addr);

    let server = BearDogCryptoRpcServer::new(state);

    // Process connections as a stream
    listener
        // Filter out connection errors
        .filter_map(|r| future::ready(r.ok()))
        // For each connection, create a channel and spawn a server task
        .for_each(|transport| {
            let server = server.clone();
            async move {
                let channel = BaseChannel::with_defaults(transport);
                // execute() returns a Stream of responses, spawn task to handle them
                tokio::spawn(channel.execute(server.serve()).for_each(|response| async move {
                    tokio::spawn(response);
                }));
            }
        })
        .await;

    Ok(())
}
