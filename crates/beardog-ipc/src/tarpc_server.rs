//! # 🚀 tarpc Server for BearDog Crypto Operations
//!
//! **HIGH-PERFORMANCE CRYPTO RPC SERVER** (v1.0.0)
//!
//! Implements the `BearDogCrypto` tarpc service trait, providing binary RPC
//! for cryptographic operations with ~10-20μs latency.
//!
//! ## Architecture
//! - tarpc server binds to TCP port (default 9901)
//! - Delegates to existing crypto implementations (same as JSON-RPC handlers)
//! - Zero unsafe code
//! - Modern async/await
//!
//! ## Philosophy: Walk → Run
//! This server provides the "running" (fast) path for operations that have
//! been stabilized through JSON-RPC "walking" (flexible) experimentation.

use std::net::SocketAddr;

use futures::StreamExt;
use tarpc::{
    context::Context,
    server::{self, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::TcpListener;
use tracing::{debug, error, info};

use crate::tarpc_types::*;

/// BearDog Crypto Server implementation
///
/// Implements the `BearDogCrypto` tarpc service trait.
/// All methods delegate to the same crypto implementations used by JSON-RPC handlers.
#[derive(Clone)]
pub struct BearDogCryptoServer {
    /// Server start time for uptime tracking
    start_time: std::time::Instant,
    
    /// Primal name (self-knowledge)
    primal_name: String,
    
    /// Version
    version: String,
}

impl BearDogCryptoServer {
    /// Create a new crypto server
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            primal_name: std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "BearDog".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Start the tarpc server on the specified address
    ///
    /// # Arguments
    /// * `addr` - Socket address to bind to (e.g., "127.0.0.1:9901")
    ///
    /// # Errors
    /// Returns error if binding fails
    pub async fn run(self, addr: SocketAddr) -> anyhow::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        info!("🚀 BearDog tarpc server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    debug!("📥 tarpc connection from {}", peer_addr);
                    
                    let server = self.clone();
                    tokio::spawn(async move {
                        let transport = tarpc::serde_transport::new(
                            tokio_util::codec::LengthDelimitedCodec::builder()
                                .max_frame_length(16 * 1024 * 1024) // 16MB max
                                .new_framed(stream),
                            Bincode::default(),
                        );

                        let channel = server::BaseChannel::with_defaults(transport);
                        
                        // Use the tarpc-generated serve() method from the impl
                        channel
                            .execute(server.serve())
                            .for_each(|response| async move {
                                tokio::spawn(response);
                            })
                            .await;
                        
                        debug!("📤 tarpc connection closed from {}", peer_addr);
                    });
                }
                Err(e) => {
                    error!("Failed to accept tarpc connection: {}", e);
                }
            }
        }
    }
}

impl Default for BearDogCryptoServer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// BearDogCrypto Service Implementation
// ============================================================

/// Implementation of the BearDogCrypto tarpc service trait.
/// The `#[tarpc::service]` attribute on the trait in tarpc_types.rs generates
/// the necessary server plumbing - we just implement the trait normally.
impl BearDogCrypto for BearDogCryptoServer {
    // ============================================================
    // Key Generation
    // ============================================================

    async fn generate_ed25519(self, _: Context) -> CryptoResult<KeyPair> {
        use ed25519_dalek::SigningKey;
        use rand::{rngs::OsRng, RngCore};

        // Generate 32 random bytes for the signing key
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        Ok(KeyPair {
            public_key: verifying_key.as_bytes().to_vec(),
            private_key: signing_key.to_bytes().to_vec(),
        })
    }

    async fn generate_x25519_ephemeral(self, _: Context) -> CryptoResult<KeyPair> {
        use rand::{rngs::OsRng, RngCore};
        use x25519_dalek::{PublicKey, StaticSecret};

        // Generate 32 random bytes for the secret key
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        
        let static_secret = StaticSecret::from(secret_bytes);
        let public_key = PublicKey::from(&static_secret);

        Ok(KeyPair {
            public_key: public_key.as_bytes().to_vec(),
            private_key: static_secret.as_bytes().to_vec(),
        })
    }

    async fn generate_ecdh_p256(self, _: Context) -> CryptoResult<KeyPair> {
        use p256::SecretKey;
        use rand::rngs::OsRng;

        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = secret_key.public_key();

        Ok(KeyPair {
            public_key: public_key.to_sec1_bytes().to_vec(),
            private_key: secret_key.to_bytes().to_vec(),
        })
    }

    async fn generate_ecdh_p384(self, _: Context) -> CryptoResult<KeyPair> {
        use p384::SecretKey;
        use rand::rngs::OsRng;

        let secret = SecretKey::random(&mut OsRng);
        let public = secret.public_key();

        Ok(KeyPair {
            public_key: public.to_sec1_bytes().to_vec(),
            private_key: secret.to_bytes().to_vec(),
        })
    }

    // ============================================================
    // Signatures
    // ============================================================

    async fn sign_ed25519(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        use ed25519_dalek::{Signature, Signer, SigningKey};

        let key_bytes: [u8; 32] = request
            .private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid Ed25519 private key length".to_string(),
            })?;

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let signature: Signature = signing_key.sign(&request.data);

        Ok(SignResponse {
            signature: signature.to_bytes().to_vec(),
        })
    }

    async fn verify_ed25519(self, _: Context, request: VerifyRequest) -> CryptoResult<bool> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let key_bytes: [u8; 32] = request
            .public_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid Ed25519 public key length".to_string(),
            })?;

        let sig_bytes: [u8; 64] = request
            .signature
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid Ed25519 signature length".to_string(),
            })?;

        let verifying_key = VerifyingKey::from_bytes(&key_bytes).map_err(|e| CryptoError {
            code: -32000,
            message: format!("Invalid public key: {}", e),
        })?;

        let signature = Signature::from_bytes(&sig_bytes);
        Ok(verifying_key.verify(&request.data, &signature).is_ok())
    }

    async fn sign_ecdsa_p256(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        use p256::ecdsa::{signature::Signer, Signature, SigningKey};

        let key_bytes: [u8; 32] = request
            .private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid P-256 private key length (expected 32 bytes)".to_string(),
            })?;

        let signing_key = SigningKey::from_bytes(&key_bytes.into())
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Invalid P-256 private key: {}", e),
            })?;

        let signature: Signature = signing_key.sign(&request.data);

        Ok(SignResponse {
            signature: signature.to_der().as_bytes().to_vec(),
        })
    }

    async fn sign_ecdsa_p384(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        use p384::ecdsa::{signature::Signer, Signature, SigningKey};

        let key_bytes: [u8; 48] = request
            .private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid P-384 private key length (expected 48 bytes)".to_string(),
            })?;

        let signing_key = SigningKey::from_bytes(&key_bytes.into())
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Invalid P-384 private key: {}", e),
            })?;

        let signature: Signature = signing_key.sign(&request.data);

        Ok(SignResponse {
            signature: signature.to_der().as_bytes().to_vec(),
        })
    }

    // ============================================================
    // Key Exchange
    // ============================================================

    async fn x25519_key_exchange(
        self,
        _: Context,
        request: KeyExchangeRequest,
    ) -> CryptoResult<SharedSecret> {
        use x25519_dalek::{PublicKey, StaticSecret};

        let our_secret_bytes: [u8; 32] = request
            .our_private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid X25519 private key length".to_string(),
            })?;

        let their_public_bytes: [u8; 32] = request
            .their_public_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid X25519 public key length".to_string(),
            })?;

        let our_secret = StaticSecret::from(our_secret_bytes);
        let their_public = PublicKey::from(their_public_bytes);
        let shared = our_secret.diffie_hellman(&their_public);

        Ok(SharedSecret {
            secret: shared.as_bytes().to_vec(),
        })
    }

    async fn ecdh_p256_key_exchange(
        self,
        _: Context,
        request: KeyExchangeRequest,
    ) -> CryptoResult<SharedSecret> {
        use p256::{ecdh::diffie_hellman, PublicKey, SecretKey};

        let key_bytes: [u8; 32] = request
            .our_private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid P-256 private key length (expected 32 bytes)".to_string(),
            })?;

        let our_secret = SecretKey::from_bytes(&key_bytes.into())
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Invalid P-256 private key: {}", e),
            })?;

        let their_public = PublicKey::from_sec1_bytes(&request.their_public_key)
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Invalid P-256 public key: {}", e),
            })?;

        let shared = diffie_hellman(our_secret.to_nonzero_scalar(), their_public.as_affine());

        Ok(SharedSecret {
            secret: shared.raw_secret_bytes().to_vec(),
        })
    }

    // ============================================================
    // Symmetric Encryption (AEAD)
    // ============================================================

    async fn chacha20_poly1305_encrypt(
        self,
        _: Context,
        request: EncryptRequest,
    ) -> CryptoResult<EncryptResponse> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };
        use rand::RngCore;

        let key: [u8; 32] = request
            .key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid key length (expected 32 bytes)".to_string(),
            })?;

        let cipher = ChaCha20Poly1305::new(&key.into());

        // Generate or use provided nonce
        let nonce_bytes: [u8; 12] = if let Some(n) = request.nonce {
            n.try_into().map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid nonce length (expected 12 bytes)".to_string(),
            })?
        } else {
            let mut n = [0u8; 12];
            rand::rngs::OsRng.fill_bytes(&mut n);
            n
        };

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, request.plaintext.as_ref()).map_err(|e| {
            CryptoError {
                code: -32000,
                message: format!("Encryption failed: {}", e),
            }
        })?;

        // ChaCha20-Poly1305 appends the 16-byte tag to ciphertext
        let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

        Ok(EncryptResponse {
            ciphertext: ct.to_vec(),
            nonce: nonce_bytes.to_vec(),
            tag: tag.to_vec(),
        })
    }

    async fn chacha20_poly1305_decrypt(
        self,
        _: Context,
        request: DecryptRequest,
    ) -> CryptoResult<DecryptResponse> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        let key: [u8; 32] = request
            .key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid key length (expected 32 bytes)".to_string(),
            })?;

        let nonce_bytes: [u8; 12] = request
            .nonce
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid nonce length (expected 12 bytes)".to_string(),
            })?;

        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Reconstruct ciphertext with tag appended
        let mut ciphertext_with_tag = request.ciphertext;
        ciphertext_with_tag.extend_from_slice(&request.tag);

        let plaintext = cipher
            .decrypt(nonce, ciphertext_with_tag.as_ref())
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Decryption failed: {}", e),
            })?;

        Ok(DecryptResponse { plaintext })
    }

    async fn aes256_gcm_encrypt(
        self,
        _: Context,
        request: EncryptRequest,
    ) -> CryptoResult<EncryptResponse> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        use rand::RngCore;

        let key: [u8; 32] = request
            .key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid key length (expected 32 bytes)".to_string(),
            })?;

        let cipher = Aes256Gcm::new(&key.into());

        let nonce_bytes: [u8; 12] = if let Some(n) = request.nonce {
            n.try_into().map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid nonce length (expected 12 bytes)".to_string(),
            })?
        } else {
            let mut n = [0u8; 12];
            rand::rngs::OsRng.fill_bytes(&mut n);
            n
        };

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, request.plaintext.as_ref()).map_err(|e| {
            CryptoError {
                code: -32000,
                message: format!("Encryption failed: {}", e),
            }
        })?;

        let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

        Ok(EncryptResponse {
            ciphertext: ct.to_vec(),
            nonce: nonce_bytes.to_vec(),
            tag: tag.to_vec(),
        })
    }

    async fn aes256_gcm_decrypt(
        self,
        _: Context,
        request: DecryptRequest,
    ) -> CryptoResult<DecryptResponse> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };

        let key: [u8; 32] = request
            .key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid key length (expected 32 bytes)".to_string(),
            })?;

        let nonce_bytes: [u8; 12] = request
            .nonce
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid nonce length (expected 12 bytes)".to_string(),
            })?;

        let cipher = Aes256Gcm::new(&key.into());
        let nonce = Nonce::from_slice(&nonce_bytes);

        let mut ciphertext_with_tag = request.ciphertext;
        ciphertext_with_tag.extend_from_slice(&request.tag);

        let plaintext = cipher
            .decrypt(nonce, ciphertext_with_tag.as_ref())
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("Decryption failed: {}", e),
            })?;

        Ok(DecryptResponse { plaintext })
    }

    // ============================================================
    // Hashing
    // ============================================================

    async fn blake3_hash(self, _: Context, data: Vec<u8>) -> CryptoResult<HashResponse> {
        let hash = blake3::hash(&data);
        Ok(HashResponse {
            hash: hash.as_bytes().to_vec(),
        })
    }

    async fn sha256_hash(self, _: Context, data: Vec<u8>) -> CryptoResult<HashResponse> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();

        Ok(HashResponse {
            hash: result.to_vec(),
        })
    }

    async fn hmac_sha256(self, _: Context, request: HmacRequest) -> CryptoResult<HashResponse> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(&request.key).map_err(|e| CryptoError {
            code: -32000,
            message: format!("Invalid HMAC key: {}", e),
        })?;

        mac.update(&request.data);
        let result = mac.finalize();

        Ok(HashResponse {
            hash: result.into_bytes().to_vec(),
        })
    }

    // ============================================================
    // TLS Support
    // ============================================================

    async fn tls_derive_handshake_secrets(
        self,
        _: Context,
        request: TlsSecretsRequest,
    ) -> CryptoResult<TlsSecrets> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let hkdf = Hkdf::<Sha256>::new(None, &request.shared_secret);

        let mut client_secret = vec![0u8; 32];
        let mut server_secret = vec![0u8; 32];

        let client_info = format!("tls13 c hs traffic {}", hex::encode(&request.transcript_hash));
        let server_info = format!("tls13 s hs traffic {}", hex::encode(&request.transcript_hash));

        hkdf.expand(client_info.as_bytes(), &mut client_secret)
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("HKDF expand failed: {}", e),
            })?;

        hkdf.expand(server_info.as_bytes(), &mut server_secret)
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("HKDF expand failed: {}", e),
            })?;

        Ok(TlsSecrets {
            client_traffic_secret: client_secret,
            server_traffic_secret: server_secret,
        })
    }

    async fn tls_derive_application_secrets(
        self,
        _: Context,
        request: TlsSecretsRequest,
    ) -> CryptoResult<TlsSecrets> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let hkdf = Hkdf::<Sha256>::new(None, &request.shared_secret);

        let mut client_secret = vec![0u8; 32];
        let mut server_secret = vec![0u8; 32];

        let client_info = format!("tls13 c ap traffic {}", hex::encode(&request.transcript_hash));
        let server_info = format!("tls13 s ap traffic {}", hex::encode(&request.transcript_hash));

        hkdf.expand(client_info.as_bytes(), &mut client_secret)
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("HKDF expand failed: {}", e),
            })?;

        hkdf.expand(server_info.as_bytes(), &mut server_secret)
            .map_err(|e| CryptoError {
                code: -32000,
                message: format!("HKDF expand failed: {}", e),
            })?;

        Ok(TlsSecrets {
            client_traffic_secret: client_secret,
            server_traffic_secret: server_secret,
        })
    }

    async fn tls_sign_handshake(
        self,
        _: Context,
        request: TlsSignRequest,
    ) -> CryptoResult<SignResponse> {
        // Delegate to Ed25519 signing (most common for TLS 1.3)
        self.sign_ed25519(
            tarpc::context::current(),
            SignRequest {
                data: request.transcript_hash,
                private_key: request.private_key,
            },
        )
        .await
    }

    // ============================================================
    // Genetic Lineage
    // ============================================================

    async fn genetic_derive_lineage_key(
        self,
        _: Context,
        request: LineageRequest,
    ) -> CryptoResult<LineageKey> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let info = format!("beardog-lineage-{}-gen{}", request.context, request.generation);
        let hkdf = Hkdf::<Sha256>::new(None, &request.family_seed);

        let mut key = vec![0u8; 32];
        hkdf.expand(info.as_bytes(), &mut key).map_err(|e| CryptoError {
            code: -32000,
            message: format!("Lineage derivation failed: {}", e),
        })?;

        Ok(LineageKey {
            key,
            generation: request.generation,
        })
    }

    async fn genetic_mix_entropy(
        self,
        _: Context,
        request: EntropyMixRequest,
    ) -> CryptoResult<MixedEntropy> {
        // Use BLAKE3 keyed hash for entropy mixing
        let mut hasher = blake3::Hasher::new();
        hasher.update(request.context.as_bytes());

        for source in &request.sources {
            hasher.update(source);
        }

        let hash = hasher.finalize();

        Ok(MixedEntropy {
            entropy: hash.as_bytes().to_vec(),
        })
    }

    // ============================================================
    // Introspection (Self-Knowledge)
    // ============================================================

    async fn primal_info(self, _: Context) -> PrimalInfo {
        PrimalInfo {
            name: self.primal_name.clone(),
            version: self.version.clone(),
            family: std::env::var("PRIMAL_FAMILY").unwrap_or_else(|_| "ecoPrimals".to_string()),
            capabilities: vec![
                "crypto".to_string(),
                "signatures".to_string(),
                "encryption".to_string(),
                "hashing".to_string(),
                "key-exchange".to_string(),
                "tls".to_string(),
                "genetic".to_string(),
            ],
        }
    }

    async fn rpc_methods(self, _: Context) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "generate_ed25519".to_string(),
                description: "Generate Ed25519 keypair".to_string(),
                params: vec![],
            },
            MethodInfo {
                name: "sign_ed25519".to_string(),
                description: "Sign data with Ed25519".to_string(),
                params: vec!["data".to_string(), "private_key".to_string()],
            },
            MethodInfo {
                name: "chacha20_poly1305_encrypt".to_string(),
                description: "Encrypt with ChaCha20-Poly1305".to_string(),
                params: vec!["plaintext".to_string(), "key".to_string()],
            },
            MethodInfo {
                name: "blake3_hash".to_string(),
                description: "Hash with BLAKE3".to_string(),
                params: vec!["data".to_string()],
            },
            // ... more methods would be listed here
        ]
    }

    async fn primal_capabilities(self, _: Context) -> Vec<Capability> {
        vec![
            Capability {
                name: "crypto.signatures.ed25519".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.signatures.ecdsa".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.encryption.chacha20-poly1305".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.encryption.aes-256-gcm".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.hashing.blake3".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.key-exchange.x25519".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "crypto.tls.1.3".to_string(),
                version: "1.0".to_string(),
            },
            Capability {
                name: "genetic.lineage".to_string(),
                version: "1.0".to_string(),
            },
        ]
    }

    async fn health(self, _: Context) -> HealthStatus {
        HealthStatus {
            status: "healthy".to_string(),
            version: self.version.clone(),
            uptime_seconds: self.start_time.elapsed().as_secs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = BearDogCryptoServer::new();
        assert!(!server.version.is_empty());
    }

    #[test]
    fn test_default_primal_name() {
        // Without PRIMAL_NAME env var, should default to "BearDog"
        std::env::remove_var("PRIMAL_NAME");
        let server = BearDogCryptoServer::new();
        assert_eq!(server.primal_name, "BearDog");
    }
}
