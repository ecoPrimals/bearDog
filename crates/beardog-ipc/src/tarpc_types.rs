//! # 🚀 tarpc Types and Service Trait for BearDog
//!
//! **HIGH-PERFORMANCE CRYPTO RPC** (v1.0.0)
//!
//! Provides shared types and service trait for tarpc-based cryptographic operations.
//! This mirrors Songbird's tarpc_types.rs pattern for protocol-agnostic architecture.
//!
//! ## Performance
//! - ~10-20 μs latency (vs 50-100 μs for JSON-RPC)
//! - ~100K requests/sec (vs 10K for JSON-RPC)
//! - Zero-copy binary serialization with bincode
//! - Type-safe at compile time
//!
//! ## Philosophy: Walk → Run (Protocol Graduation)
//! - **JSON-RPC** (walking): Flexible, text-based, easy debugging, first interactions
//! - **tarpc** (running): Fast, binary, second nature once patterns stabilize
//!
//! Both protocols expose the SAME operations - tarpc is just the "compiled" version
//! of what was learned through JSON-RPC experimentation.
//!
//! ## Protocol Priority
//! 1. **tarpc** (PRIMARY) - High-performance binary RPC for established patterns
//! 2. **JSON-RPC** (SECONDARY) - Flexible text-based for new/evolving operations
//! 3. **HTTP** (FALLBACK) - Universal compatibility

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// tarpc service trait for BearDog cryptographic operations
///
/// This trait defines the async RPC interface using tarpc.
/// Both client and server implementations use this trait.
///
/// # Design: Parallel with JSON-RPC
/// Every method here has a corresponding JSON-RPC handler.
/// tarpc is the "fast path" for operations that have stabilized.
#[cfg(feature = "tarpc")]
#[tarpc::service]
pub trait BearDogCrypto {
    // ============================================================
    // Key Generation (Asymmetric)
    // ============================================================

    /// Generate Ed25519 keypair
    async fn generate_ed25519() -> CryptoResult<KeyPair>;

    /// Generate X25519 ephemeral keypair for key exchange
    async fn generate_x25519_ephemeral() -> CryptoResult<KeyPair>;

    /// Generate ECDH P-256 keypair
    async fn generate_ecdh_p256() -> CryptoResult<KeyPair>;

    /// Generate ECDH P-384 keypair
    async fn generate_ecdh_p384() -> CryptoResult<KeyPair>;

    // ============================================================
    // Signatures
    // ============================================================

    /// Sign data with Ed25519
    async fn sign_ed25519(request: SignRequest) -> CryptoResult<SignResponse>;

    /// Verify Ed25519 signature
    async fn verify_ed25519(request: VerifyRequest) -> CryptoResult<bool>;

    /// Sign data with ECDSA P-256
    async fn sign_ecdsa_p256(request: SignRequest) -> CryptoResult<SignResponse>;

    /// Sign data with ECDSA P-384
    async fn sign_ecdsa_p384(request: SignRequest) -> CryptoResult<SignResponse>;

    // ============================================================
    // Key Exchange
    // ============================================================

    /// Perform X25519 Diffie-Hellman key exchange
    async fn x25519_key_exchange(request: KeyExchangeRequest) -> CryptoResult<SharedSecret>;

    /// Perform ECDH P-256 key exchange
    async fn ecdh_p256_key_exchange(request: KeyExchangeRequest) -> CryptoResult<SharedSecret>;

    // ============================================================
    // Symmetric Encryption (AEAD)
    // ============================================================

    /// Encrypt with ChaCha20-Poly1305
    async fn chacha20_poly1305_encrypt(request: EncryptRequest) -> CryptoResult<EncryptResponse>;

    /// Decrypt with ChaCha20-Poly1305
    async fn chacha20_poly1305_decrypt(request: DecryptRequest) -> CryptoResult<DecryptResponse>;

    /// Encrypt with AES-256-GCM
    async fn aes256_gcm_encrypt(request: EncryptRequest) -> CryptoResult<EncryptResponse>;

    /// Decrypt with AES-256-GCM
    async fn aes256_gcm_decrypt(request: DecryptRequest) -> CryptoResult<DecryptResponse>;

    // ============================================================
    // Hashing
    // ============================================================

    /// Hash data with BLAKE3
    async fn blake3_hash(data: Vec<u8>) -> CryptoResult<HashResponse>;

    /// Hash data with SHA-256
    async fn sha256_hash(data: Vec<u8>) -> CryptoResult<HashResponse>;

    /// Compute HMAC-SHA256
    async fn hmac_sha256(request: HmacRequest) -> CryptoResult<HashResponse>;

    // ============================================================
    // TLS Support
    // ============================================================

    /// Derive TLS 1.3 handshake secrets
    async fn tls_derive_handshake_secrets(request: TlsSecretsRequest) -> CryptoResult<TlsSecrets>;

    /// Derive TLS 1.3 application secrets
    async fn tls_derive_application_secrets(request: TlsSecretsRequest)
        -> CryptoResult<TlsSecrets>;

    /// Sign TLS handshake
    async fn tls_sign_handshake(request: TlsSignRequest) -> CryptoResult<SignResponse>;

    // ============================================================
    // Genetic Lineage (Dark Forest)
    // ============================================================

    /// Derive lineage key from family seed
    async fn genetic_derive_lineage_key(request: LineageRequest) -> CryptoResult<LineageKey>;

    /// Mix entropy for genetic operations
    async fn genetic_mix_entropy(request: EntropyMixRequest) -> CryptoResult<MixedEntropy>;

    // ============================================================
    // Introspection (Self-Knowledge)
    // ============================================================

    /// Get primal info
    async fn primal_info() -> PrimalInfo;

    /// List available methods
    async fn rpc_methods() -> Vec<MethodInfo>;

    /// Get capabilities
    async fn primal_capabilities() -> Vec<Capability>;

    /// Health check
    async fn health() -> HealthStatus;
}

// ============================================================
// Request/Response Types
// ============================================================

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CryptoError>;

/// Crypto operation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoError {
    pub code: i32,
    pub message: String,
}

/// Keypair (public + private)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

/// Sign request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    pub data: Vec<u8>,
    pub private_key: Vec<u8>,
}

/// Sign response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignResponse {
    pub signature: Vec<u8>,
}

/// Verify request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Key exchange request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeRequest {
    pub our_private_key: Vec<u8>,
    pub their_public_key: Vec<u8>,
}

/// Shared secret from key exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSecret {
    pub secret: Vec<u8>,
}

/// Encrypt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub plaintext: Vec<u8>,
    pub key: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
    pub aad: Option<Vec<u8>>,
}

/// Encrypt response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: Vec<u8>,
}

/// Decrypt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub ciphertext: Vec<u8>,
    pub key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: Vec<u8>,
    pub aad: Option<Vec<u8>>,
}

/// Decrypt response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub plaintext: Vec<u8>,
}

/// Hash response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResponse {
    pub hash: Vec<u8>,
}

/// HMAC request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacRequest {
    pub data: Vec<u8>,
    pub key: Vec<u8>,
}

/// TLS secrets derivation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSecretsRequest {
    pub shared_secret: Vec<u8>,
    pub transcript_hash: Vec<u8>,
    pub cipher_suite: String,
}

/// TLS derived secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSecrets {
    pub client_traffic_secret: Vec<u8>,
    pub server_traffic_secret: Vec<u8>,
}

/// TLS sign request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSignRequest {
    pub transcript_hash: Vec<u8>,
    pub private_key: Vec<u8>,
    pub algorithm: String,
}

/// Lineage key derivation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRequest {
    pub family_seed: Vec<u8>,
    pub generation: u32,
    pub context: String,
}

/// Lineage key response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageKey {
    pub key: Vec<u8>,
    pub generation: u32,
}

/// Entropy mixing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyMixRequest {
    pub sources: Vec<Vec<u8>>,
    pub context: String,
}

/// Mixed entropy response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedEntropy {
    pub entropy: Vec<u8>,
}

/// Primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    pub name: String,
    pub version: String,
    pub family: String,
    pub capabilities: Vec<String>,
}

/// Method information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub description: String,
    pub params: Vec<String>,
}

/// Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub version: String,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

/// Protocol information for negotiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    pub name: String,
    pub port: u16,
    pub enabled: bool,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_serialization() {
        let kp = KeyPair {
            public_key: vec![1, 2, 3],
            private_key: vec![4, 5, 6],
        };
        let json = serde_json::to_string(&kp).unwrap();
        let deserialized: KeyPair = serde_json::from_str(&json).unwrap();
        assert_eq!(kp.public_key, deserialized.public_key);
    }

    #[test]
    fn test_crypto_error() {
        let err = CryptoError {
            code: -32000,
            message: "Invalid key".to_string(),
        };
        assert_eq!(err.code, -32000);
    }

    #[test]
    fn test_encrypt_request_optional_fields() {
        let req = EncryptRequest {
            plaintext: vec![1, 2, 3],
            key: vec![0; 32],
            nonce: None,
            aad: None,
        };
        assert!(req.nonce.is_none());
        assert!(req.aad.is_none());
    }
}
