// SPDX-License-Identifier: AGPL-3.0-or-later

//! Session creation and verification (`btsp.server.create_session`, `btsp.server.verify`).

use base64::Engine;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::Aead};
use rand::RngCore;
use serde::Deserialize;
use tracing::{info, warn};
use x25519_dalek::{PublicKey, StaticSecret};

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
    /// Create a server-side BTSP session for a calling primal.
    ///
    /// Generates ephemeral X25519 keys, derives the handshake key from the
    /// caller's family seed, stores state, and returns the server public key
    /// plus a challenge for the client to prove family membership.
    pub(super) async fn handle_server_create_session(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for btsp.server.create_session")?;
        let create_params = beardog_types::btsp::SessionCreateParams::deserialize(params_value)
            .map_err(|e| format!("Invalid create_session params: {e}"))?;

        let family_seed = base64::engine::general_purpose::STANDARD
            .decode(&create_params.family_seed)
            .map_err(|e| format!("Invalid family_seed base64: {e}"))?;

        let (session_token, server_pub, challenge) = self
            .session_store
            .create_session(&family_seed)
            .await
            .map_err(|e| format!("Session creation failed: {e}"))?;

        info!(
            session_token = %session_token,
            "BTSP server session created"
        );

        let resp = beardog_types::btsp::SessionCreateResponse {
            server_ephemeral_pub: base64::engine::general_purpose::STANDARD.encode(server_pub),
            challenge: base64::engine::general_purpose::STANDARD.encode(challenge),
            session_token,
        };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }

    /// Verify a client's challenge response and derive session keys.
    ///
    /// Consumes the pending handshake (single-use token). On success, the
    /// session is promoted to active with derived `ChaCha20-Poly1305` keys.
    pub(super) async fn handle_server_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for btsp.server.verify")?;
        let verify_params = beardog_types::btsp::SessionVerifyParams::deserialize(params_value)
            .map_err(|e| format!("Invalid server.verify params: {e}"))?;

        info!(
            session_token = %verify_params.session_token,
            "BTSP server.verify requested"
        );

        let client_pub_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
            .decode(&verify_params.client_ephemeral_pub)
            .map_err(|e| format!("Invalid client_ephemeral_pub: {e}"))?
            .try_into()
            .map_err(|_| "client_ephemeral_pub must be exactly 32 bytes".to_string())?;

        let response_bytes = base64::engine::general_purpose::STANDARD
            .decode(&verify_params.response)
            .map_err(|e| format!("Invalid response: {e}"))?;

        let cipher =
            crate::btsp_handshake::BtspCipher::from_wire_name(&verify_params.preferred_cipher)
                .map_err(|e| format!("Invalid cipher: {e}"))?;

        match self
            .session_store
            .verify_session(
                &verify_params.session_token,
                &client_pub_bytes,
                &response_bytes,
                cipher,
            )
            .await
        {
            Ok((session_id, negotiated_cipher)) => {
                info!(session_id = %session_id, "BTSP server session verified");
                let resp = beardog_types::btsp::SessionVerifyResponse {
                    verified: true,
                    session_id: Some(session_id),
                    cipher: Some(negotiated_cipher.wire_name().to_string()),
                    error: None,
                };
                serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into)
            }
            Err(e) => {
                warn!(error = %e, "BTSP server session verification failed");
                let resp = beardog_types::btsp::SessionVerifyResponse {
                    verified: false,
                    session_id: None,
                    cipher: None,
                    error: Some(e.to_string()),
                };
                serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into)
            }
        }
    }

    /// Export session keys for a verified session, wrapped under the caller's
    /// X25519 ephemeral public key.
    ///
    /// This completes the BTSP relay path (BTSP-BARRACUDA-WIRE): after
    /// `btsp.server.verify` succeeds, the relay primal calls `export_keys`
    /// to retrieve the session keys encrypted so they never appear in
    /// plaintext in a JSON-RPC response.
    ///
    /// Wrapping scheme:
    /// 1. `BearDog` generates a fresh X25519 ephemeral keypair
    /// 2. DH with the caller's ephemeral pub → shared secret
    /// 3. HKDF(shared, "btsp-key-export-v1") → wrapping key (32 bytes)
    /// 4. ChaCha20-Poly1305 encrypts `encrypt_key || decrypt_key` (64 bytes)
    /// 5. Returns `nonce || ciphertext` as `wrapped_keys` (base64)
    pub(super) async fn handle_server_export_keys(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for btsp.server.export_keys")?;
        let export_params = beardog_types::btsp::SessionExportKeysParams::deserialize(params_value)
            .map_err(|e| format!("Invalid export_keys params: {e}"))?;

        let caller_pub_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
            .decode(&export_params.caller_ephemeral_pub)
            .map_err(|e| format!("Invalid caller_ephemeral_pub: {e}"))?
            .try_into()
            .map_err(|_| "caller_ephemeral_pub must be exactly 32 bytes".to_string())?;

        let (encrypt_key, decrypt_key, cipher) = self
            .session_store
            .export_session_keys(&export_params.session_id)
            .await
            .map_err(|e| format!("Session lookup failed: {e}"))?;

        let mut plaintext = [0u8; 64];
        plaintext[..32].copy_from_slice(&encrypt_key);
        plaintext[32..].copy_from_slice(&decrypt_key);

        let mut wrapper_secret_bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut wrapper_secret_bytes);
        let wrapper_secret = StaticSecret::from(wrapper_secret_bytes);
        let wrapper_pub = PublicKey::from(&wrapper_secret);

        let caller_pub = PublicKey::from(caller_pub_bytes);
        let shared = *wrapper_secret.diffie_hellman(&caller_pub).as_bytes();

        let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(b"btsp-key-export-v1"), &shared);
        let mut wrapping_key = [0u8; 32];
        hk.expand(b"wrap", &mut wrapping_key)
            .map_err(|e| format!("HKDF wrapping key derivation failed: {e}"))?;

        let aead = ChaCha20Poly1305::new((&wrapping_key).into());
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = aead
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| format!("Key wrapping encryption failed: {e}"))?;

        let mut wrapped = Vec::with_capacity(12 + ciphertext.len());
        wrapped.extend_from_slice(&nonce_bytes);
        wrapped.extend_from_slice(&ciphertext);

        info!(
            session_id = %export_params.session_id,
            cipher = %cipher.wire_name(),
            "BTSP session keys exported (wrapped)"
        );

        let resp = beardog_types::btsp::SessionExportKeysResponse {
            wrapped_keys: base64::engine::general_purpose::STANDARD.encode(&wrapped),
            wrapper_ephemeral_pub: base64::engine::general_purpose::STANDARD
                .encode(wrapper_pub.as_bytes()),
            cipher: cipher.wire_name().to_string(),
        };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }
}
