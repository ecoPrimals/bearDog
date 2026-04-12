// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cipher negotiation and server session store status (`btsp.server.negotiate`, `btsp.server.status`).

use serde::Deserialize;
use tracing::{info, warn};

use super::BtspHandler;

impl BtspHandler {
    /// Re-negotiate cipher suite for an active session.
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
