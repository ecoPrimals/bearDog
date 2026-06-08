// SPDX-License-Identifier: AGPL-3.0-or-later

//! Contact exchange (`btsp.contact.*`).

use crate::btsp_provider::BeardogBtspProvider;
use std::sync::Arc;
use tracing::{info, warn};

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
    /// Handle BTSP contact exchange request
    ///
    /// Exchanges contact information with a peer using genetic lineage for routing.
    pub(super) async fn handle_contact_exchange(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("🔍 BTSP Contact Exchange requested");

        let params = params.ok_or("Missing params for contact exchange")?;

        let target_peer_id = params
            .get("target_peer_id")
            .or_else(|| params.get("peer_id"))
            .and_then(|v| v.as_str())
            .ok_or("Missing target_peer_id")?;

        let requester_lineage = params
            .get("requester_lineage")
            .or_else(|| params.get("lineage"))
            .and_then(|v| v.as_str())
            .ok_or("Missing requester_lineage")?;

        #[expect(
            clippy::cast_possible_truncation,
            reason = "BTSP hop limit from request"
        )]
        let max_hops = params
            .get("max_hops")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(3) as usize;

        match btsp_provider
            .contact_exchange(target_peer_id, requester_lineage, max_hops)
            .await
        {
            Ok(contact_info) => {
                info!(
                    "✅ Contact exchange successful for peer: {}",
                    target_peer_id
                );
                Ok(serde_json::to_value(contact_info)
                    .map_err(|e| format!("Serialization error: {e}"))?)
            }
            Err(e) => {
                warn!("⚠️  Contact exchange failed: {}", e);
                Err(format!("Contact exchange failed: {e}").into())
            }
        }
    }
}
