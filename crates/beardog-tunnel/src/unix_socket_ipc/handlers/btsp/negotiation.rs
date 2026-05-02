// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cipher negotiation and server session store status.
//!
//! - `btsp.server.negotiate` / `btsp.session.negotiate` — re-negotiate cipher on active session
//! - `btsp.negotiate` — Phase 3 encrypted channel negotiation (ChaCha20-Poly1305)
//! - `btsp.server.status` — session store health

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::RngCore;
use serde::Deserialize;
use tracing::{debug, info, warn};

use super::BtspHandler;

impl BtspHandler {
    /// Re-negotiate cipher suite for an active session (Phase 2 store-based).
    pub(super) async fn handle_server_negotiate(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for btsp.server.negotiate")?;
        let neg_params = beardog_types::btsp::SessionNegotiateParams::deserialize(params_value)
            .map_err(|e| format!("Invalid server.negotiate params: {e}"))?;

        info!(
            session_token = %neg_params.session_token,
            requested_cipher = %neg_params.cipher,
            "BTSP server.negotiate requested"
        );

        let cipher = crate::btsp_handshake::BtspCipher::from_wire_name(&neg_params.cipher)
            .unwrap_or(crate::btsp_handshake::BtspCipher::ChaCha20Poly1305);

        match self
            .session_store
            .negotiate_cipher(&neg_params.session_token, cipher)
            .await
        {
            Ok(negotiated) => {
                let resp = beardog_types::btsp::SessionNegotiateResponse {
                    accepted: true,
                    cipher: negotiated.wire_name().to_string(),
                };
                serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
            }
            Err(e) => {
                warn!(error = %e, "BTSP server negotiate failed");
                let resp = beardog_types::btsp::SessionNegotiateResponse {
                    accepted: false,
                    cipher: neg_params.cipher,
                };
                serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
            }
        }
    }

    /// Phase 3 encrypted channel negotiation.
    ///
    /// After a successful Phase 1 handshake, the client sends `btsp.negotiate`
    /// offering cipher suites and a client nonce. The server selects the best
    /// cipher, generates a server nonce, and both sides derive session keys via
    /// `HKDF-SHA256(handshake_key, client_nonce || server_nonce)`.
    ///
    /// Wire: `{"session_id":"...","ciphers":["chacha20-poly1305"],"client_nonce":"<b64>"}`
    /// Response: `{"cipher":"chacha20-poly1305","server_nonce":"<b64>"}`
    pub(super) async fn handle_phase3_negotiate(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or(
            "Missing params for btsp.negotiate — expected: \
             {\"session_id\":\"...\",\"ciphers\":[\"chacha20-poly1305\"],\"client_nonce\":\"<b64>\"}",
        )?;

        let session_id = params_value
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: session_id")?;

        let client_nonce_b64 = params_value
            .get("client_nonce")
            .and_then(|v| v.as_str())
            .ok_or("Missing required parameter: client_nonce (base64)")?;

        let client_nonce = BASE64
            .decode(client_nonce_b64)
            .map_err(|e| format!("Invalid base64 in client_nonce: {e}"))?;

        let offered_ciphers: Vec<String> = if let Some(ciphers_val) = params_value.get("ciphers") {
            serde_json::from_value(ciphers_val.clone())
                .map_err(|e| format!("Invalid ciphers array: {e}"))?
        } else if let Some(cipher) = params_value
            .get("preferred_cipher")
            .and_then(|v| v.as_str())
        {
            vec![cipher.to_string()]
        } else {
            return Err("Missing required parameter: ciphers (array of cipher names)".to_string());
        };

        let selected = select_best_cipher(&offered_ciphers);

        if selected == "null" {
            info!(
                session_id = session_id,
                "BTSP Phase 3: no supported cipher offered — returning null"
            );
            return Ok(serde_json::json!({
                "cipher": "null",
                "server_nonce": "",
            }));
        }

        let family_seed = load_family_seed()?;
        let handshake_key = crate::btsp_handshake::crypto::derive_handshake_key(&family_seed)
            .map_err(|e| format!("Failed to derive handshake key: {e}"))?;

        let mut server_nonce = [0u8; 32];
        rand::rng().fill_bytes(&mut server_nonce);
        let server_nonce_b64 = BASE64.encode(server_nonce);

        let _keys = crate::btsp_handshake::crypto::derive_phase3_session_keys(
            &handshake_key,
            &client_nonce,
            &server_nonce,
        )
        .map_err(|e| format!("Phase 3 key derivation failed: {e}"))?;

        debug!(
            session_id = session_id,
            cipher = selected,
            "BTSP Phase 3: negotiate complete — session keys derived (server-side)"
        );

        info!(
            "🔒 BTSP Phase 3: negotiated cipher '{}' for session '{}'",
            selected, session_id
        );

        Ok(serde_json::json!({
            "cipher": selected,
            "server_nonce": server_nonce_b64,
        }))
    }

    /// Report session store health and active session count.
    pub(super) async fn handle_server_status(&self) -> Result<serde_json::Value, String> {
        let status = self.session_store.status().await;
        info!(
            pending = status.pending_sessions,
            active = status.active_sessions,
            "BTSP server status"
        );
        serde_json::to_value(status).map_err(|e| format!("Serialize: {e}"))
    }
}

/// Select the best cipher from the client's offered list.
///
/// Preference order: `chacha20-poly1305` > `hmac-plain` > `null`.
fn select_best_cipher(offered: &[String]) -> &'static str {
    if offered.iter().any(|c| c == "chacha20-poly1305") {
        "chacha20-poly1305"
    } else if offered.iter().any(|c| c == "hmac-plain") {
        "hmac-plain"
    } else {
        "null"
    }
}

/// Load the family seed from the environment for handshake key re-derivation.
fn load_family_seed() -> Result<Vec<u8>, String> {
    let seed_str = beardog_errors::process_env::var("FAMILY_SEED")
        .or_else(|_| beardog_errors::process_env::var("BEARDOG_FAMILY_SEED"))
        .map_err(|_| {
            "BTSP Phase 3 requires FAMILY_SEED or BEARDOG_FAMILY_SEED environment variable"
                .to_string()
        })?;

    if seed_str.len() >= 16 {
        Ok(seed_str.as_bytes().to_vec())
    } else {
        Err("FAMILY_SEED too short (minimum 16 bytes)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_best_cipher_prefers_chacha20() {
        let offered = vec![
            "null".to_string(),
            "chacha20-poly1305".to_string(),
            "hmac-plain".to_string(),
        ];
        assert_eq!(select_best_cipher(&offered), "chacha20-poly1305");
    }

    #[test]
    fn select_best_cipher_falls_back_to_hmac() {
        let offered = vec!["hmac-plain".to_string(), "null".to_string()];
        assert_eq!(select_best_cipher(&offered), "hmac-plain");
    }

    #[test]
    fn select_best_cipher_falls_back_to_null() {
        let offered = vec!["aes-256-gcm".to_string()];
        assert_eq!(select_best_cipher(&offered), "null");
    }

    #[test]
    fn select_best_cipher_empty_returns_null() {
        assert_eq!(select_best_cipher(&[]), "null");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn phase3_negotiate_returns_cipher_and_nonce() {
        beardog_errors::process_env::set_var(
            "FAMILY_SEED",
            "test-family-seed-for-phase3-negotiate",
        );

        let handler = BtspHandler::new();
        let client_nonce = [0xAA; 32];
        let client_nonce_b64 = BASE64.encode(client_nonce);

        let params = serde_json::json!({
            "session_id": "test-session-123",
            "ciphers": ["chacha20-poly1305"],
            "client_nonce": client_nonce_b64,
        });

        let result = handler
            .handle_phase3_negotiate(Some(&params))
            .await
            .expect("negotiate should succeed");

        assert_eq!(result["cipher"], "chacha20-poly1305");
        assert!(result["server_nonce"].as_str().is_some());
        let server_nonce = BASE64
            .decode(result["server_nonce"].as_str().unwrap())
            .expect("server_nonce should be valid base64");
        assert_eq!(server_nonce.len(), 32);

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn phase3_negotiate_null_when_unsupported_cipher() {
        beardog_errors::process_env::set_var(
            "FAMILY_SEED",
            "test-family-seed-for-phase3-negotiate",
        );

        let handler = BtspHandler::new();
        let params = serde_json::json!({
            "session_id": "test-session-456",
            "ciphers": ["aes-256-gcm"],
            "client_nonce": BASE64.encode([0xBB; 32]),
        });

        let result = handler
            .handle_phase3_negotiate(Some(&params))
            .await
            .expect("negotiate should succeed with null fallback");

        assert_eq!(result["cipher"], "null");

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    async fn phase3_negotiate_fails_without_params() {
        let handler = BtspHandler::new();
        let result = handler.handle_phase3_negotiate(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn phase3_negotiate_fails_missing_session_id() {
        let handler = BtspHandler::new();
        let params = serde_json::json!({
            "ciphers": ["chacha20-poly1305"],
            "client_nonce": "YWJj",
        });
        let result = handler.handle_phase3_negotiate(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("session_id"));
    }

    #[tokio::test]
    async fn phase3_negotiate_fails_missing_client_nonce() {
        let handler = BtspHandler::new();
        let params = serde_json::json!({
            "session_id": "test",
            "ciphers": ["chacha20-poly1305"],
        });
        let result = handler.handle_phase3_negotiate(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("client_nonce"));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn phase3_negotiate_fails_without_family_seed() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");

        let handler = BtspHandler::new();
        let params = serde_json::json!({
            "session_id": "test",
            "ciphers": ["chacha20-poly1305"],
            "client_nonce": BASE64.encode([0xCC; 32]),
        });

        let result = handler.handle_phase3_negotiate(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("FAMILY_SEED"));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn phase3_negotiate_preferred_cipher_alias() {
        beardog_errors::process_env::set_var(
            "FAMILY_SEED",
            "test-family-seed-for-phase3-negotiate",
        );

        let handler = BtspHandler::new();
        let params = serde_json::json!({
            "session_id": "test-session",
            "preferred_cipher": "chacha20-poly1305",
            "client_nonce": BASE64.encode([0xDD; 32]),
        });

        let result = handler
            .handle_phase3_negotiate(Some(&params))
            .await
            .expect("preferred_cipher alias should work");
        assert_eq!(result["cipher"], "chacha20-poly1305");

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn phase3_key_derivation_matches_primalspring() {
        use crate::btsp_handshake::crypto::{derive_handshake_key, derive_phase3_session_keys};

        let seed = b"deterministic-test-seed-for-key-verification";
        let handshake_key = derive_handshake_key(seed).expect("handshake key");

        let client_nonce = [0x01; 32];
        let server_nonce = [0x02; 32];

        let server_keys = derive_phase3_session_keys(&handshake_key, &client_nonce, &server_nonce)
            .expect("server keys");

        assert_ne!(server_keys.encrypt_key, server_keys.decrypt_key);
        assert_ne!(server_keys.encrypt_key, [0u8; 32]);
        assert_ne!(server_keys.decrypt_key, [0u8; 32]);
    }
}
