// SPDX-License-Identifier: AGPL-3.0-or-later

//! tarpc binary RPC service (G64 Cephalization).
//!
//! Exposes a `.tarpc.sock` sibling socket alongside the primary JSON-RPC
//! `.sock`. Hot-path crypto operations use bincode framing — no JSON
//! serde roundtrip, no string encoding overhead.
//!
//! ## Socket Convention
//!
//! ```text
//! beardog-family123.sock          ← JSON-RPC  (always present, bootstrap + diagnostic)
//! beardog-family123.tarpc.sock    ← tarpc      (optional, high-perf intra-gate)
//! ```
//!
//! ## Ecosystem Alignment
//!
//! biomeOS `SecurityRpc` trait expects `sign`, `verify`, `get_jwt_secret`,
//! `verify_lineage`. This service covers those operations plus BLAKE3 hash
//! (provenance hot-path). Additional methods can be added without breaking
//! existing consumers.

use std::sync::Arc;

use beardog_crypto::{hash_blake3, sign_ed25519, verify_ed25519};
use beardog_types::primal_identity::PrimalIdentity;
use tracing::{debug, info, warn};

/// Derives the tarpc socket path from a JSON-RPC socket path.
///
/// Follows the ecosystem convention from `biomeos-primal-sdk::tarpc_transport`:
/// `beardog.sock` → `beardog.tarpc.sock`.
#[must_use]
pub fn tarpc_socket_path(jsonrpc_socket: &str) -> String {
    if let Some(base) = jsonrpc_socket.strip_suffix(".sock") {
        format!("{base}.tarpc.sock")
    } else {
        format!("{jsonrpc_socket}.tarpc")
    }
}

/// Health status returned by the tarpc health check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TarpcHealthStatus {
    /// Whether the service is healthy.
    pub healthy: bool,
    /// Primal name.
    pub primal: String,
    /// Crate version.
    pub version: String,
    /// Number of JSON-RPC methods served.
    pub method_count: u32,
}

/// Result of an Ed25519 signing operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignResult {
    /// Base64-encoded signature (64 bytes raw).
    pub signature: Vec<u8>,
    /// Public key corresponding to the signing key.
    pub public_key: Vec<u8>,
}

#[expect(
    missing_docs,
    reason = "tarpc::service macro generates the trait and helper types; docs on individual methods below"
)]
#[tarpc::service]
pub trait BearDogRpc {
    /// Health check — returns primal status without JSON overhead.
    async fn health_check() -> TarpcHealthStatus;

    /// BLAKE3 hash — provenance hot-path (CAS, data braids).
    async fn blake3_hash(data: Vec<u8>) -> Vec<u8>;

    /// Ed25519 sign — derived key from KDF(FAMILY_SEED, key_id, "signing").
    /// Returns `Err` if key derivation or signing fails.
    async fn sign_ed25519(key_id: String, message: Vec<u8>) -> Result<SignResult, String>;

    /// Ed25519 verify.
    async fn verify_ed25519(
        public_key: Vec<u8>,
        message: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<bool, String>;

    /// SHA-256 hash.
    async fn sha256(data: Vec<u8>) -> Vec<u8>;

    /// HMAC-SHA256.
    async fn hmac_sha256(key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, String>;

    /// Crate version string.
    async fn version() -> String;
}

/// Server-side implementation of `BearDogRpc`.
#[derive(Clone)]
pub struct BearDogRpcServer {
    identity: Arc<PrimalIdentity>,
}

impl BearDogRpcServer {
    /// Creates a new tarpc service backed by the given primal identity.
    #[must_use]
    pub fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self { identity }
    }

    fn derive_ed25519_keypair(
        &self,
        key_id: &str,
    ) -> Result<(ed25519_dalek::SigningKey, ed25519_dalek::VerifyingKey), String> {
        let family_seed = beardog_errors::process_env::var(
            beardog_config::env_keys::ENV_FAMILY_SEED_PREFIXED,
        )
        .or_else(|_| beardog_errors::process_env::var(beardog_config::env_keys::ENV_FAMILY_SEED))
        .map_err(|_| "FAMILY_SEED not set — cannot derive signing key".to_string())?;

        let derived = blake3::derive_key(
            &format!("beardog-ed25519:{key_id}:signing"),
            family_seed.as_bytes(),
        );

        let signing_key = ed25519_dalek::SigningKey::from_bytes(&derived);
        let verifying_key = signing_key.verifying_key();
        Ok((signing_key, verifying_key))
    }
}

impl BearDogRpc for BearDogRpcServer {
    async fn health_check(self, _: tarpc::context::Context) -> TarpcHealthStatus {
        debug!("tarpc health_check");
        TarpcHealthStatus {
            healthy: true,
            primal: self.identity.primal_name().to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            method_count: 236,
        }
    }

    async fn blake3_hash(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        hash_blake3(&data)
    }

    async fn sign_ed25519(
        self,
        _: tarpc::context::Context,
        key_id: String,
        message: Vec<u8>,
    ) -> Result<SignResult, String> {
        let (signing_key, verifying_key) = self.derive_ed25519_keypair(&key_id)?;

        let mut expanded = [0u8; 64];
        expanded[..32].copy_from_slice(signing_key.as_bytes());
        expanded[32..].copy_from_slice(verifying_key.as_bytes());

        let signature =
            sign_ed25519(&message, &expanded).map_err(|e| format!("signing failed: {e}"))?;

        Ok(SignResult {
            signature,
            public_key: verifying_key.to_bytes().to_vec(),
        })
    }

    async fn verify_ed25519(
        self,
        _: tarpc::context::Context,
        public_key: Vec<u8>,
        message: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<bool, String> {
        verify_ed25519(&message, &signature, &public_key)
            .map_err(|e| format!("verification failed: {e}"))
    }

    async fn sha256(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        beardog_crypto::hash_sha256(&data)
    }

    async fn hmac_sha256(
        self,
        _: tarpc::context::Context,
        key: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        beardog_crypto::hmac_sha256(&key, &data).map_err(|e| format!("HMAC failed: {e}"))
    }

    async fn version(self, _: tarpc::context::Context) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

/// Spawns the tarpc listener on the `.tarpc.sock` sibling socket.
///
/// This is fire-and-forget: if the listener fails to bind, it logs a warning
/// and returns without blocking the JSON-RPC server.
///
/// # Errors
///
/// Returns `Err` if the Unix socket cannot be bound (e.g. path too long,
/// permissions). The caller should treat this as non-fatal.
pub async fn spawn_tarpc_listener(
    jsonrpc_socket_path: &str,
    identity: Arc<PrimalIdentity>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use futures::StreamExt;
    use tarpc::server::{BaseChannel, Channel};

    let tarpc_path = tarpc_socket_path(jsonrpc_socket_path);

    if std::path::Path::new(&tarpc_path).exists() {
        std::fs::remove_file(&tarpc_path).ok();
    }

    let incoming = tarpc::serde_transport::unix::listen(
        &tarpc_path,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;
    info!(path = %tarpc_path, "tarpc listener bound");

    let server = BearDogRpcServer::new(identity);

    tokio::spawn(async move {
        futures::pin_mut!(incoming);
        while let Some(result) = incoming.next().await {
            match result {
                Ok(transport) => {
                    let channel = BaseChannel::with_defaults(transport);
                    let handler = server.clone();
                    tokio::spawn(channel.execute(handler.serve()).for_each(|resp| async {
                        tokio::spawn(resp);
                    }));
                }
                Err(e) => {
                    warn!(error = %e, "tarpc accept failed");
                }
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarpc_socket_path_convention() {
        assert_eq!(
            tarpc_socket_path("/tmp/beardog.sock"),
            "/tmp/beardog.tarpc.sock"
        );
        assert_eq!(
            tarpc_socket_path("/run/user/1000/biomeos/beardog-family123.sock"),
            "/run/user/1000/biomeos/beardog-family123.tarpc.sock"
        );
    }

    #[test]
    fn tarpc_socket_path_no_sock_suffix() {
        assert_eq!(
            tarpc_socket_path("/tmp/beardog"),
            "/tmp/beardog.tarpc"
        );
    }
}
