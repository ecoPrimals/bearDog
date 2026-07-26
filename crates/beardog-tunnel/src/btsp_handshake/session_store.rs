// SPDX-License-Identifier: AGPL-3.0-or-later

//! Persistent server-side BTSP session store.
//!
//! Holds ephemeral handshake state between `btsp.server.create_session` and
//! `btsp.server.verify`, then promotes verified sessions to active state
//! with derived session keys.
//!
//! Designed for the "handshake-as-a-service" RPC surface that downstream
//! springs (healthSpring, hotSpring, neuralSpring, ludoSpring) call to
//! establish authenticated sessions via JSON-RPC.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroize;

use super::session::BtspCipher;
use beardog_errors::BearDogError;
use beardog_types::constants::domains::network::timeouts::TLS_HANDSHAKE_TIMEOUT;

/// Maximum time a pending handshake can sit before it expires.
const PENDING_HANDSHAKE_TTL: Duration = TLS_HANDSHAKE_TIMEOUT;

/// Maximum number of concurrent pending handshakes (`DoS` guard).
const MAX_PENDING_SESSIONS: usize = 256;

/// Maximum active sessions (bounded to prevent unbounded memory growth).
const MAX_ACTIVE_SESSIONS: usize = 1024;

/// Server-side handshake state awaiting client verification.
struct PendingSession {
    server_secret: StaticSecret,
    server_pub: PublicKey,
    handshake_key: [u8; 32],
    challenge: [u8; 32],
    created_at: Instant,
}

impl Drop for PendingSession {
    fn drop(&mut self) {
        self.handshake_key.zeroize();
        self.challenge.zeroize();
    }
}

/// A verified, active BTSP session with derived keys.
pub struct ActiveSession {
    /// Hex session ID.
    pub session_id: String,
    /// Negotiated cipher.
    pub cipher: BtspCipher,
    /// Requesting primal's ephemeral public key.
    pub client_pub: [u8; 32],
    /// Server-to-client key.
    pub encrypt_key: [u8; 32],
    /// Client-to-server key.
    pub decrypt_key: [u8; 32],
    /// When the session was verified.
    pub verified_at: Instant,
}

impl Drop for ActiveSession {
    fn drop(&mut self) {
        self.encrypt_key.zeroize();
        self.decrypt_key.zeroize();
    }
}

/// Thread-safe, time-bounded session store for BTSP server RPC.
#[derive(Clone)]
pub struct BtspSessionStore {
    pending: Arc<RwLock<HashMap<String, PendingSession>>>,
    active: Arc<RwLock<HashMap<String, ActiveSession>>>,
}

impl Default for BtspSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BtspSessionStore {
    /// Create an empty session store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
            active: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new pending session: generate ephemeral keys, derive handshake
    /// key, generate challenge. Returns `(session_token, server_pub, challenge)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the family seed derivation fails or the store is
    /// at capacity (denial-of-service guard).
    pub async fn create_session(
        &self,
        family_seed: &[u8],
    ) -> Result<(String, [u8; 32], [u8; 32]), BearDogError> {
        use super::crypto::{derive_handshake_key, generate_ephemeral_keypair};
        use rand::RngCore;

        self.evict_expired_pending().await;

        {
            let pending = self.pending.read().await;
            if pending.len() >= MAX_PENDING_SESSIONS {
                return Err(BearDogError::system(
                    "BTSP session store at capacity (too many pending handshakes)".to_string(),
                ));
            }
        }

        let handshake_key = derive_handshake_key(family_seed)?;
        let (server_secret, server_pub) = generate_ephemeral_keypair();

        let mut challenge = [0u8; 32];
        rand::rng().fill_bytes(&mut challenge);

        let session_token = uuid::Uuid::new_v4().to_string();

        let pending_session = PendingSession {
            server_secret,
            server_pub,
            handshake_key,
            challenge,
            created_at: Instant::now(),
        };

        self.pending
            .write()
            .await
            .insert(session_token.clone(), pending_session);

        Ok((session_token, *server_pub.as_bytes(), challenge))
    }

    /// Verify a client's challenge response and promote to active session.
    ///
    /// Consumes the pending session (single-use). On success, derives session
    /// keys via X25519 + HKDF and stores the active session.
    ///
    /// # Errors
    ///
    /// Returns an error if the session token is unknown/expired, the client
    /// response fails HMAC verification, or key derivation fails.
    pub async fn verify_session(
        &self,
        session_token: &str,
        client_pub_bytes: &[u8; 32],
        client_response: &[u8],
        preferred_cipher: BtspCipher,
    ) -> Result<(String, BtspCipher), BearDogError> {
        let pending = self
            .pending
            .write()
            .await
            .remove(session_token)
            .ok_or_else(|| {
                BearDogError::system(format!(
                    "BTSP session token not found or expired: {session_token}"
                ))
            })?;

        if pending.created_at.elapsed() > PENDING_HANDSHAKE_TTL {
            return Err(BearDogError::system(
                "BTSP pending session expired".to_string(),
            ));
        }

        super::crypto::verify_challenge_response(
            &pending.handshake_key,
            &pending.challenge,
            client_pub_bytes,
            pending.server_pub.as_bytes(),
            client_response,
        )?;

        let their_pub = x25519_dalek::PublicKey::from(*client_pub_bytes);
        let shared_secret = super::crypto::x25519_shared_secret(&pending.server_secret, &their_pub);

        let mut session_id_bytes = [0u8; 16];
        rand::RngCore::fill_bytes(&mut rand::rng(), &mut session_id_bytes);
        let session_id = hex::encode(session_id_bytes);

        let keys = super::crypto::derive_session_keys(&shared_secret, session_id.as_bytes())?;

        {
            let mut active = self.active.write().await;
            if active.len() >= MAX_ACTIVE_SESSIONS {
                self.evict_oldest_active(&mut active);
            }
            active.insert(
                session_id.clone(),
                ActiveSession {
                    session_id: session_id.clone(),
                    cipher: preferred_cipher,
                    client_pub: *client_pub_bytes,
                    encrypt_key: keys.server_to_client,
                    decrypt_key: keys.client_to_server,
                    verified_at: Instant::now(),
                },
            );
        }

        Ok((session_id, preferred_cipher))
    }

    /// Re-negotiate cipher for an active session.
    ///
    /// # Errors
    ///
    /// Returns an error if the session ID is not found.
    pub async fn negotiate_cipher(
        &self,
        session_id: &str,
        cipher: BtspCipher,
    ) -> Result<BtspCipher, BearDogError> {
        let mut active = self.active.write().await;
        let session = active.get_mut(session_id).ok_or_else(|| {
            BearDogError::system(format!("BTSP active session not found: {session_id}"))
        })?;
        session.cipher = cipher;
        Ok(cipher)
    }

    /// Export session keys for an active session (for the relay path).
    ///
    /// Returns `(encrypt_key, decrypt_key, cipher)` if the session exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the session ID is not found.
    pub async fn export_session_keys(
        &self,
        session_id: &str,
    ) -> Result<([u8; 32], [u8; 32], BtspCipher), BearDogError> {
        let active = self.active.read().await;
        let session = active.get(session_id).ok_or_else(|| {
            BearDogError::system(format!("BTSP active session not found: {session_id}"))
        })?;
        Ok((session.encrypt_key, session.decrypt_key, session.cipher))
    }

    /// Summary statistics for `btsp.server.status`.
    pub async fn status(&self) -> SessionStoreStatus {
        let pending = self.pending.read().await;
        let active = self.active.read().await;
        SessionStoreStatus {
            pending_sessions: pending.len(),
            active_sessions: active.len(),
            max_pending: MAX_PENDING_SESSIONS,
            max_active: MAX_ACTIVE_SESSIONS,
        }
    }

    /// Remove expired pending sessions.
    async fn evict_expired_pending(&self) {
        let mut pending = self.pending.write().await;
        pending.retain(|_, s| s.created_at.elapsed() < PENDING_HANDSHAKE_TTL);
    }

    /// Drop the oldest active session to make room.
    fn evict_oldest_active(&self, active: &mut HashMap<String, ActiveSession>) {
        if let Some(oldest_key) = active
            .iter()
            .min_by_key(|(_, s)| s.verified_at)
            .map(|(k, _)| k.clone())
        {
            active.remove(&oldest_key);
        }
    }
}

/// Snapshot of session store state for the status endpoint.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionStoreStatus {
    /// Number of handshakes awaiting verification.
    pub pending_sessions: usize,
    /// Number of verified, active sessions.
    pub active_sessions: usize,
    /// Maximum pending before rejection.
    pub max_pending: usize,
    /// Maximum active before eviction.
    pub max_active: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btsp_handshake::crypto::{
        compute_challenge_hmac, derive_handshake_key, generate_ephemeral_keypair,
    };

    const TEST_SEED: &[u8] = b"test-family-seed-for-btsp-store!";

    #[tokio::test]
    async fn create_and_verify_session_roundtrip() {
        let store = BtspSessionStore::new();

        let (token, server_pub, challenge) = store
            .create_session(TEST_SEED)
            .await
            .expect("create session");

        assert!(!token.is_empty());

        let handshake_key = derive_handshake_key(TEST_SEED).expect("derive key");
        let (_client_secret, client_pub) = generate_ephemeral_keypair();

        let response = compute_challenge_hmac(
            &handshake_key,
            &challenge,
            client_pub.as_bytes(),
            &server_pub,
        )
        .expect("compute hmac");

        let (session_id, cipher) = store
            .verify_session(
                &token,
                client_pub.as_bytes(),
                &response,
                BtspCipher::ChaCha20Poly1305,
            )
            .await
            .expect("verify session");

        assert!(!session_id.is_empty());
        assert_eq!(cipher, BtspCipher::ChaCha20Poly1305);

        let status = store.status().await;
        assert_eq!(status.pending_sessions, 0);
        assert_eq!(status.active_sessions, 1);
    }

    #[tokio::test]
    async fn verify_rejects_wrong_response() {
        let store = BtspSessionStore::new();

        let (token, _server_pub, _challenge) = store
            .create_session(TEST_SEED)
            .await
            .expect("create session");

        let (_client_secret, client_pub) = generate_ephemeral_keypair();
        let bad_response = [0xBB; 32];

        let result = store
            .verify_session(
                &token,
                client_pub.as_bytes(),
                &bad_response,
                BtspCipher::ChaCha20Poly1305,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn verify_consumes_pending_session() {
        let store = BtspSessionStore::new();

        let (token, server_pub, challenge) = store
            .create_session(TEST_SEED)
            .await
            .expect("create session");

        let handshake_key = derive_handshake_key(TEST_SEED).expect("derive key");
        let (_client_secret, client_pub) = generate_ephemeral_keypair();

        let response = compute_challenge_hmac(
            &handshake_key,
            &challenge,
            client_pub.as_bytes(),
            &server_pub,
        )
        .expect("compute hmac");

        store
            .verify_session(
                &token,
                client_pub.as_bytes(),
                &response,
                BtspCipher::ChaCha20Poly1305,
            )
            .await
            .expect("first verify");

        let result = store
            .verify_session(
                &token,
                client_pub.as_bytes(),
                &response,
                BtspCipher::ChaCha20Poly1305,
            )
            .await;

        assert!(result.is_err(), "second verify should fail (consumed)");
    }

    #[tokio::test]
    async fn negotiate_cipher_on_active_session() {
        let store = BtspSessionStore::new();

        let (token, server_pub, challenge) = store
            .create_session(TEST_SEED)
            .await
            .expect("create session");

        let handshake_key = derive_handshake_key(TEST_SEED).expect("derive key");
        let (_client_secret, client_pub) = generate_ephemeral_keypair();

        let response = compute_challenge_hmac(
            &handshake_key,
            &challenge,
            client_pub.as_bytes(),
            &server_pub,
        )
        .expect("compute hmac");

        let (session_id, _) = store
            .verify_session(
                &token,
                client_pub.as_bytes(),
                &response,
                BtspCipher::ChaCha20Poly1305,
            )
            .await
            .expect("verify");

        let negotiated = store
            .negotiate_cipher(&session_id, BtspCipher::HmacPlain)
            .await
            .expect("negotiate");

        assert_eq!(negotiated, BtspCipher::HmacPlain);
    }

    #[tokio::test]
    async fn status_reflects_store_state() {
        let store = BtspSessionStore::new();

        let status = store.status().await;
        assert_eq!(status.pending_sessions, 0);
        assert_eq!(status.active_sessions, 0);

        store
            .create_session(TEST_SEED)
            .await
            .expect("create session");

        let status = store.status().await;
        assert_eq!(status.pending_sessions, 1);
        assert_eq!(status.active_sessions, 0);
    }
}
