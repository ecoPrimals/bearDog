// SPDX-License-Identifier: AGPL-3.0-or-later

//! # 🚀 tarpc Types and Service Trait for `BearDog`
//!
//! **HIGH-PERFORMANCE CRYPTO RPC** (v1.0.0)
//!
//! Provides shared types and service trait for tarpc-based cryptographic operations.
//! This mirrors Songbird's `tarpc_types.rs` pattern for protocol-agnostic architecture.
//!
//! ## Performance
//! - ~10-20 μs latency (vs 50-100 μs for JSON-RPC)
//! - ~100K requests/sec (vs 10K for JSON-RPC)
//! - Compact binary serialization (e.g. postcard) for IPC payloads
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

/// tarpc service trait for `BearDog` cryptographic operations
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
    /// Application-defined error code (mirrors JSON-RPC style semantics).
    pub code: i32,
    /// Human-readable failure reason.
    pub message: String,
}

/// Keypair (public + private)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    /// Encoded public key bytes for the selected algorithm.
    pub public_key: Vec<u8>,
    /// Encoded private key bytes (handle as secret material).
    pub private_key: Vec<u8>,
}

/// Sign request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    /// Message or digest to sign.
    pub data: Vec<u8>,
    /// Private key bytes in the server's expected encoding.
    pub private_key: Vec<u8>,
}

/// Sign response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignResponse {
    /// Raw signature bytes.
    pub signature: Vec<u8>,
}

/// Verify request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    /// Original signed payload.
    pub data: Vec<u8>,
    /// Signature produced by [`SignResponse::signature`].
    pub signature: Vec<u8>,
    /// Public key bytes matching the signer.
    pub public_key: Vec<u8>,
}

/// Key exchange request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeRequest {
    /// Local ephemeral or static private key.
    pub our_private_key: Vec<u8>,
    /// Remote party's public key.
    pub their_public_key: Vec<u8>,
}

/// Shared secret from key exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSecret {
    /// Derived shared secret (e.g. DH output).
    pub secret: Vec<u8>,
}

/// Encrypt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptRequest {
    /// Plaintext to protect.
    pub plaintext: Vec<u8>,
    /// Symmetric key bytes.
    pub key: Vec<u8>,
    /// Optional nonce/IV; generated server-side when omitted for algorithms that allow it.
    pub nonce: Option<Vec<u8>>,
    /// Optional associated authenticated data for AEAD modes.
    pub aad: Option<Vec<u8>>,
}

/// Encrypt response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResponse {
    /// Ciphertext without tag when tag is separate (implementation-defined packing).
    pub ciphertext: Vec<u8>,
    /// Nonce/IV used for decryption.
    pub nonce: Vec<u8>,
    /// Authentication tag for AEAD schemes.
    pub tag: Vec<u8>,
}

/// Decrypt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    /// Ciphertext from [`EncryptResponse::ciphertext`].
    pub ciphertext: Vec<u8>,
    /// Symmetric key bytes.
    pub key: Vec<u8>,
    /// Nonce/IV matching encryption.
    pub nonce: Vec<u8>,
    /// Authentication tag from [`EncryptResponse::tag`].
    pub tag: Vec<u8>,
    /// Optional AAD matching encryption.
    pub aad: Option<Vec<u8>>,
}

/// Decrypt response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    /// Recovered plaintext.
    pub plaintext: Vec<u8>,
}

/// Hash response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResponse {
    /// Digest output bytes.
    pub hash: Vec<u8>,
}

/// HMAC request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacRequest {
    /// Message input to MAC.
    pub data: Vec<u8>,
    /// Symmetric MAC key.
    pub key: Vec<u8>,
}

/// TLS secrets derivation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSecretsRequest {
    /// Handshake shared secret (e.g. ECDH output).
    pub shared_secret: Vec<u8>,
    /// Transcript hash covering negotiated messages.
    pub transcript_hash: Vec<u8>,
    /// TLS cipher suite identifier (IANA or internal string token).
    pub cipher_suite: String,
}

/// TLS derived secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSecrets {
    /// Client traffic protection secret (TLS 1.3 style labeling).
    pub client_traffic_secret: Vec<u8>,
    /// Server traffic protection secret.
    pub server_traffic_secret: Vec<u8>,
}

/// TLS sign request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSignRequest {
    /// Transcript hash to sign (handshake context).
    pub transcript_hash: Vec<u8>,
    /// Private key for the certificate key pair.
    pub private_key: Vec<u8>,
    /// Signature algorithm name (e.g. `ed25519`, `ecdsa_p256`).
    pub algorithm: String,
}

/// Lineage key derivation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRequest {
    /// High-entropy family seed material.
    pub family_seed: Vec<u8>,
    /// Generation counter within the lineage chain.
    pub generation: u32,
    /// Domain separation string for KDF context.
    pub context: String,
}

/// Lineage key response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageKey {
    /// Derived lineage key bytes.
    pub key: Vec<u8>,
    /// Echo of the requested generation.
    pub generation: u32,
}

/// Entropy mixing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyMixRequest {
    /// Independent entropy sources to combine.
    pub sources: Vec<Vec<u8>>,
    /// Context string binding the mix to a purpose.
    pub context: String,
}

/// Mixed entropy response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedEntropy {
    /// Fixed-length mixed output suitable for downstream KDFs.
    pub entropy: Vec<u8>,
}

/// Primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    /// Process or primal display name.
    pub name: String,
    /// Semantic version of the running binary.
    pub version: String,
    /// Deployment family or tenant grouping.
    pub family: String,
    /// Advertised capability identifiers.
    pub capabilities: Vec<String>,
}

/// Method information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    /// RPC method name.
    pub name: String,
    /// Short human description for introspection UIs.
    pub description: String,
    /// Parameter names or type hints as strings.
    pub params: Vec<String>,
}

/// Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Capability identifier (stable string).
    pub name: String,
    /// Capability contract version.
    pub version: String,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Coarse status (`ok`, `degraded`, etc.).
    pub status: String,
    /// Software version reporting health.
    pub version: String,
    /// Seconds since process start.
    pub uptime_seconds: u64,
}

/// Protocol information for negotiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Transport or RPC protocol label (`tarpc`, `jsonrpc`, `http`, ...).
    pub name: String,
    /// Listening TCP/UDP port when applicable (from discovery or config; values in docs/tests are illustrative).
    pub port: u16,
    /// Whether this protocol endpoint is currently accepting work.
    pub enabled: bool,
    /// Optional key/value metadata (TLS requirements, paths, feature flags).
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
        let json = serde_json::to_string(&kp).expect("serialize KeyPair");
        let deserialized: KeyPair =
            serde_json::from_str(&json).expect("deserialize KeyPair from JSON");
        assert_eq!(kp.public_key, deserialized.public_key);
    }

    #[test]
    fn test_keypair_roundtrip() {
        let kp = KeyPair {
            public_key: vec![1, 2, 3],
            private_key: vec![4, 5, 6],
        };
        let json = serde_json::to_string(&kp).expect("serialize KeyPair for roundtrip");
        let restored: KeyPair = serde_json::from_str(&json).expect("deserialize KeyPair roundtrip");
        assert_eq!(kp.public_key, restored.public_key);
        assert_eq!(kp.private_key, restored.private_key);
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
    fn test_crypto_error_serialization() {
        let err = CryptoError {
            code: -32600,
            message: "Bad request".to_string(),
        };
        let json = serde_json::to_string(&err).expect("serialize CryptoError");
        let restored: CryptoError = serde_json::from_str(&json).expect("deserialize CryptoError");
        assert_eq!(err.code, restored.code);
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

    #[test]
    fn test_sign_request_serialization() {
        let req = SignRequest {
            data: vec![1, 2, 3],
            private_key: vec![4, 5, 6],
        };
        let json = serde_json::to_string(&req).expect("serialize SignRequest");
        let restored: SignRequest = serde_json::from_str(&json).expect("deserialize SignRequest");
        assert_eq!(req.data, restored.data);
    }

    #[test]
    fn test_verify_request_serialization() {
        let req = VerifyRequest {
            data: vec![1],
            signature: vec![2],
            public_key: vec![3],
        };
        let json = serde_json::to_string(&req).expect("serialize VerifyRequest");
        let restored: VerifyRequest =
            serde_json::from_str(&json).expect("deserialize VerifyRequest");
        assert_eq!(req.public_key, restored.public_key);
    }

    #[test]
    fn test_key_exchange_request_serialization() {
        let req = KeyExchangeRequest {
            our_private_key: vec![1; 32],
            their_public_key: vec![2; 32],
        };
        let json = serde_json::to_string(&req).expect("serialize KeyExchangeRequest");
        let restored: KeyExchangeRequest =
            serde_json::from_str(&json).expect("deserialize KeyExchangeRequest");
        assert_eq!(req.our_private_key.len(), restored.our_private_key.len());
    }

    #[test]
    fn test_decrypt_request_serialization() {
        let req = DecryptRequest {
            ciphertext: vec![1],
            key: vec![0; 32],
            nonce: vec![0; 12],
            tag: vec![0; 16],
            aad: None,
        };
        let json = serde_json::to_string(&req).expect("serialize DecryptRequest");
        let restored: DecryptRequest =
            serde_json::from_str(&json).expect("deserialize DecryptRequest");
        assert_eq!(req.ciphertext, restored.ciphertext);
    }

    #[test]
    fn test_hmac_request_serialization() {
        let req = HmacRequest {
            data: vec![1, 2, 3],
            key: vec![4, 5, 6],
        };
        let json = serde_json::to_string(&req).expect("serialize HmacRequest");
        let restored: HmacRequest = serde_json::from_str(&json).expect("deserialize HmacRequest");
        assert_eq!(req.key, restored.key);
    }

    #[test]
    fn test_primal_info_serialization() {
        let info = PrimalInfo {
            name: "beardog".to_string(),
            version: "1.0".to_string(),
            family: "ecoPrimals".to_string(),
            capabilities: vec!["crypto".to_string()],
        };
        let json = serde_json::to_string(&info).expect("serialize PrimalInfo");
        let restored: PrimalInfo = serde_json::from_str(&json).expect("deserialize PrimalInfo");
        assert_eq!(info.name, restored.name);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus {
            status: "healthy".to_string(),
            version: "1.0".to_string(),
            uptime_seconds: 42,
        };
        let json = serde_json::to_string(&status).expect("serialize HealthStatus");
        let restored: HealthStatus = serde_json::from_str(&json).expect("deserialize HealthStatus");
        assert_eq!(status.uptime_seconds, restored.uptime_seconds);
    }

    #[test]
    fn test_protocol_info_serialization() {
        let mut meta = HashMap::new();
        meta.insert("port".to_string(), "9901".to_string());
        let info = ProtocolInfo {
            name: "tarpc".to_string(),
            port: 9901,
            enabled: true,
            metadata: meta,
        };
        let json = serde_json::to_string(&info).expect("serialize ProtocolInfo");
        let restored: ProtocolInfo = serde_json::from_str(&json).expect("deserialize ProtocolInfo");
        assert_eq!(info.port, restored.port);
    }

    #[test]
    fn test_protocol_info_default_metadata() {
        let json = r#"{"name":"tarpc","port":9901,"enabled":true}"#;
        let info: ProtocolInfo =
            serde_json::from_str(json).expect("deserialize ProtocolInfo with default metadata");
        assert!(info.metadata.is_empty());
    }

    #[test]
    fn test_lineage_request_serialization() {
        let req = LineageRequest {
            family_seed: vec![1, 2, 3],
            generation: 1,
            context: "test".to_string(),
        };
        let json = serde_json::to_string(&req).expect("serialize LineageRequest");
        let restored: LineageRequest =
            serde_json::from_str(&json).expect("deserialize LineageRequest");
        assert_eq!(req.generation, restored.generation);
    }

    #[test]
    fn test_tls_secrets_request_serialization() {
        let req = TlsSecretsRequest {
            shared_secret: vec![1; 32],
            transcript_hash: vec![2; 32],
            cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        };
        let json = serde_json::to_string(&req).expect("serialize TlsSecretsRequest");
        let restored: TlsSecretsRequest =
            serde_json::from_str(&json).expect("deserialize TlsSecretsRequest");
        assert_eq!(req.cipher_suite, restored.cipher_suite);
    }

    #[test]
    fn test_capability_serialization() {
        let cap = Capability {
            name: "crypto.signatures".to_string(),
            version: "1.0".to_string(),
        };
        let json = serde_json::to_string(&cap).expect("serialize Capability");
        let restored: Capability = serde_json::from_str(&json).expect("deserialize Capability");
        assert_eq!(cap.name, restored.name);
    }
}
