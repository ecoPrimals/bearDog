// SPDX-License-Identifier: AGPL-3.0-or-later

//! Session creation and verification (`btsp.server.create_session`, `btsp.server.verify`).

use base64::Engine;
use serde::Deserialize;
use tracing::{info, warn};

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
    ) -> Result<serde_json::Value, String> {
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
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    /// Verify a client's challenge response and derive session keys.
    ///
    /// Consumes the pending handshake (single-use token). On success, the
    /// session is promoted to active with derived `ChaCha20-Poly1305` keys.
    pub(super) async fn handle_server_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
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
                serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
            }
            Err(e) => {
                warn!(error = %e, "BTSP server session verification failed");
                let resp = beardog_types::btsp::SessionVerifyResponse {
                    verified: false,
                    session_id: None,
                    cipher: None,
                    error: Some(e.to_string()),
                };
                serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
            }
        }
    }
}
