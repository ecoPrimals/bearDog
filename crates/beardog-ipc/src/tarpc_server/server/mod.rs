// SPDX-License-Identifier: AGPL-3.0-or-later

//! `BearDog` tarpc server type and TCP accept loop.

mod aead;
mod genetic;
mod hashing;
mod introspection;
mod kdf;
mod key_exchange;
mod keygen;
mod signatures;
mod tls;

use std::net::SocketAddr;

use crate::tarpc_types::{
    BearDogCrypto, Capability, CryptoResult, DecryptRequest, DecryptResponse, EncryptRequest,
    EncryptResponse, EntropyMixRequest, HashResponse, HealthStatus, HmacRequest,
    KeyExchangeRequest, KeyPair, LineageKey, LineageRequest, MethodInfo, MixedEntropy, PrimalInfo,
    SharedSecret, SignRequest, SignResponse, TlsSecrets, TlsSecretsRequest, TlsSignRequest,
    VerifyRequest,
};
use futures::StreamExt;
use tarpc::{
    context::Context,
    server::{self, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::TcpListener;
use tracing::{debug, error, info};

/// `BearDog` Crypto Server implementation
///
/// Implements the `BearDogCrypto` tarpc service trait.
/// All methods delegate to the same crypto implementations used by JSON-RPC handlers.
#[derive(Clone)]
pub struct BearDogCryptoServer {
    /// Server start time for uptime tracking
    start_time: std::time::Instant,

    /// Primal name (self-knowledge)
    primal_name: String,

    /// Primal family label (e.g. ecosystem name)
    primal_family: String,

    /// Version
    version: String,
}

impl BearDogCryptoServer {
    /// Create a new crypto server with compile-time defaults (no environment I/O).
    pub fn new() -> Self {
        Self::with_identity(env!("CARGO_PKG_NAME"), "ecoPrimals")
    }

    /// Create with explicit identity (for tests and dependency injection).
    pub fn with_identity(primal_name: impl Into<String>, primal_family: impl Into<String>) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            primal_name: primal_name.into(),
            primal_family: primal_family.into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Create reading `PRIMAL_NAME` and `PRIMAL_FAMILY` from the environment.
    pub fn from_env() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            primal_name: beardog_errors::process_env::var("PRIMAL_NAME")
                .unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string()),
            primal_family: beardog_errors::process_env::var("PRIMAL_FAMILY")
                .unwrap_or_else(|_| "ecoPrimals".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Start the tarpc server on the specified address.
    ///
    /// # Arguments
    /// * `addr` - Socket address resolved at runtime via capability-based
    ///   port discovery (see `beardog-config::domains::port_discovery`)
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

/// Implementation of the `BearDogCrypto` tarpc service trait.
/// The `#[tarpc::service]` attribute on the trait in `tarpc_types.rs` generates
/// the necessary server plumbing - we just implement the trait normally.
impl BearDogCrypto for BearDogCryptoServer {
    async fn generate_ed25519(self, _: Context) -> CryptoResult<KeyPair> {
        keygen::generate_ed25519().await
    }

    async fn generate_x25519_ephemeral(self, _: Context) -> CryptoResult<KeyPair> {
        keygen::generate_x25519_ephemeral().await
    }

    async fn generate_ecdh_p256(self, _: Context) -> CryptoResult<KeyPair> {
        keygen::generate_ecdh_p256().await
    }

    async fn generate_ecdh_p384(self, _: Context) -> CryptoResult<KeyPair> {
        keygen::generate_ecdh_p384().await
    }

    async fn sign_ed25519(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        signatures::sign_ed25519(request).await
    }

    async fn verify_ed25519(self, _: Context, request: VerifyRequest) -> CryptoResult<bool> {
        signatures::verify_ed25519(request).await
    }

    async fn sign_ecdsa_p256(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        signatures::sign_ecdsa_p256(request).await
    }

    async fn sign_ecdsa_p384(self, _: Context, request: SignRequest) -> CryptoResult<SignResponse> {
        signatures::sign_ecdsa_p384(request).await
    }

    async fn x25519_key_exchange(
        self,
        _: Context,
        request: KeyExchangeRequest,
    ) -> CryptoResult<SharedSecret> {
        key_exchange::x25519_key_exchange(request).await
    }

    async fn ecdh_p256_key_exchange(
        self,
        _: Context,
        request: KeyExchangeRequest,
    ) -> CryptoResult<SharedSecret> {
        key_exchange::ecdh_p256_key_exchange(request).await
    }

    async fn chacha20_poly1305_encrypt(
        self,
        _: Context,
        request: EncryptRequest,
    ) -> CryptoResult<EncryptResponse> {
        aead::chacha20_poly1305_encrypt(request).await
    }

    async fn chacha20_poly1305_decrypt(
        self,
        _: Context,
        request: DecryptRequest,
    ) -> CryptoResult<DecryptResponse> {
        aead::chacha20_poly1305_decrypt(request).await
    }

    async fn aes256_gcm_encrypt(
        self,
        _: Context,
        request: EncryptRequest,
    ) -> CryptoResult<EncryptResponse> {
        aead::aes256_gcm_encrypt(request).await
    }

    async fn aes256_gcm_decrypt(
        self,
        _: Context,
        request: DecryptRequest,
    ) -> CryptoResult<DecryptResponse> {
        aead::aes256_gcm_decrypt(request).await
    }

    async fn blake3_hash(self, _: Context, data: Vec<u8>) -> CryptoResult<HashResponse> {
        hashing::blake3_hash(data).await
    }

    async fn sha256_hash(self, _: Context, data: Vec<u8>) -> CryptoResult<HashResponse> {
        hashing::sha256_hash(data).await
    }

    async fn hmac_sha256(self, _: Context, request: HmacRequest) -> CryptoResult<HashResponse> {
        hashing::hmac_sha256(request).await
    }

    async fn tls_derive_handshake_secrets(
        self,
        _: Context,
        request: TlsSecretsRequest,
    ) -> CryptoResult<TlsSecrets> {
        kdf::tls_derive_handshake_secrets(request).await
    }

    async fn tls_derive_application_secrets(
        self,
        _: Context,
        request: TlsSecretsRequest,
    ) -> CryptoResult<TlsSecrets> {
        kdf::tls_derive_application_secrets(request).await
    }

    async fn tls_sign_handshake(
        self,
        _: Context,
        request: TlsSignRequest,
    ) -> CryptoResult<SignResponse> {
        tls::tls_sign_handshake(request).await
    }

    async fn genetic_derive_lineage_key(
        self,
        _: Context,
        request: LineageRequest,
    ) -> CryptoResult<LineageKey> {
        kdf::genetic_derive_lineage_key(request).await
    }

    async fn genetic_mix_entropy(
        self,
        _: Context,
        request: EntropyMixRequest,
    ) -> CryptoResult<MixedEntropy> {
        genetic::genetic_mix_entropy(request).await
    }

    async fn primal_info(self, _: Context) -> PrimalInfo {
        introspection::primal_info(&self).await
    }

    async fn rpc_methods(self, _: Context) -> Vec<MethodInfo> {
        introspection::rpc_methods().await
    }

    async fn primal_capabilities(self, _: Context) -> Vec<Capability> {
        introspection::primal_capabilities().await
    }

    async fn health(self, _: Context) -> HealthStatus {
        introspection::health(&self).await
    }
}

#[cfg(test)]
mod tests;
