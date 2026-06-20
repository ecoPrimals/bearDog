// SPDX-License-Identifier: AGPL-3.0-or-later

//! Peer trust verification (`btsp.verify_peer`) and bootstrap seeding (`btsp.trust.seed`).

use crate::btsp_provider::BeardogBtspProvider;
use crate::btsp_provider::parse_trusted_peer_pair;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
    /// Handle unified trust verification
    ///
    /// # Architectural Note
    ///
    /// - **Genetic lineage verification**: Handled by `BearDog` (use existing trust evaluation)
    /// - **Certificate verification**: Handled by the transport peer (external mode)
    ///
    /// For certificate trust, use the transport peer's BTSP external mode API.
    pub(super) async fn handle_verify_peer(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("🔍 BTSP Verify Peer requested");

        let params_value = params.ok_or("Missing params for verify_peer")?;

        // Parse parameters
        let verify_params = beardog_types::btsp::VerifyPeerParams::deserialize(params_value)
            .map_err(|e| format!("Invalid verify_peer params: {e}"))?;

        // Route based on trust mode
        match verify_params.trust_mode.as_str() {
            "genetic_lineage" => {
                // Get tunnel to find peer_id
                let (_tunnel_id, peer_id) = btsp_provider
                    .get_tunnel(&verify_params.tunnel_id)
                    .ok_or_else(|| format!("Tunnel not found: {}", verify_params.tunnel_id))?;

                // Get peer trust record
                let trust_record = btsp_provider.get_peer_trust_record(&peer_id);

                if let Some(record) = trust_record {
                    use crate::btsp_provider::types::TrustLevel;
                    let is_trusted = record.trust_level == TrustLevel::Verified
                        || record.trust_level == TrustLevel::Trusted;

                    let trust_level_str = format!("{:?}", record.trust_level);

                    info!(
                        "✅ Peer {} trust evaluation: {} (level: {})",
                        peer_id,
                        if is_trusted { "TRUSTED" } else { "NOT TRUSTED" },
                        trust_level_str
                    );

                    Ok(serde_json::json!({
                        "valid": is_trusted,
                        "trust_level": trust_level_str.to_lowercase(),
                        "peer_id": peer_id,
                        "connection_count": record.connection_count,
                        "first_seen": record.first_seen.to_rfc3339(),
                        "last_seen": record.last_seen.to_rfc3339(),
                    }))
                } else {
                    warn!("⚠️  No trust record found for peer: {}", peer_id);
                    Ok(serde_json::json!({
                        "valid": false,
                        "trust_level": "unknown",
                        "peer_id": peer_id,
                        "error": "No trust record found for peer",
                    }))
                }
            }
            "certificate" => {
                info!(
                    "📡 Certificate verification is part of external mode; handled outside BearDog (requesting primal)"
                );
                Err("btsp.verify_peer (certificate) is part of external mode, not implemented in BearDog.\n\
                     \n\
                     Use the requesting primal's BTSP external mode API for certificate verification.\n\
                     BearDog provides crypto primitives (tls.verify_certificate RPC method).".into())
            }
            _ => Err(format!("Unknown trust_mode: {}", verify_params.trust_mode).into()),
        }
    }

    /// Bootstrap a trusted peer into the BTSP trust database.
    ///
    /// Params: `{ "peer_id": "...", "family_id": "..." }` or `{ "peers": "id:fam,id2:fam2" }`.
    pub(super) async fn handle_trust_seed(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("🌱 BTSP trust seed requested");

        let params = params.ok_or("Missing params for trust seed")?;

        if let Some(peers) = params.get("peers").and_then(|v| v.as_str()) {
            let mut seeded = 0usize;
            for entry in peers.split(',') {
                let entry = entry.trim();
                if entry.is_empty() {
                    continue;
                }
                let (peer_id, family_id) = parse_trusted_peer_pair(entry)
                    .map_err(|e| format!("Invalid peers entry: {e}"))?;
                if btsp_provider
                    .seed_trusted_peer(peer_id, family_id)
                    .await
                    .map_err(|e| format!("Trust seed failed: {e}"))?
                {
                    seeded += 1;
                }
            }
            return Ok(serde_json::json!({
                "seeded": seeded,
                "source": "peers",
            }));
        }

        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing peer_id (or peers batch string)")?;
        let family_id = params
            .get("family_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing family_id")?;

        let inserted = btsp_provider
            .seed_trusted_peer(peer_id, family_id)
            .await
            .map_err(|e| format!("Trust seed failed: {e}"))?;

        Ok(serde_json::json!({
            "peer_id": peer_id,
            "family_id": family_id,
            "seeded": inserted,
        }))
    }
}
