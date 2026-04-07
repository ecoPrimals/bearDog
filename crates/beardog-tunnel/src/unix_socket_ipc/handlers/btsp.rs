// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP (`BearDog` Tunnel Security Protocol) handlers - UNIFIED
//!
//! Provides secure communication for both:
//! - **Internal Mode**: P2P mesh tunneling via genetic lineage (primals)
//! - **External Mode**: HTTPS communication via certificate trust (APIs)
//!
//! # Architecture
//!
//! BTSP Unified consolidates two communication patterns into a single API:
//! - Trust mode (genetic lineage vs. certificate) is the fundamental difference
//! - Same crypto foundation (X25519, ChaCha20-Poly1305, Ed25519) for both
//! - Backward compatible with existing BTSP internal mode calls

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use base64::Engine;
use beardog_capabilities::traits::SecureTunnelProvider;
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

/// Handler for BTSP Unified methods
///
/// # Core Operations (6 methods, internal mode)
///
/// - `btsp.contact.exchange` (semantic) / `btsp.contact_exchange` — Exchange contact info via genetic lineage
/// - `btsp.tunnel.establish` / `btsp.tunnel_establish` — **UNIFIED**: Establish secure tunnel (internal OR external)
/// - `btsp.tunnel.encrypt` / `btsp.tunnel_encrypt` — Encrypt data through tunnel
/// - `btsp.tunnel.decrypt` / `btsp.tunnel_decrypt` — Decrypt data from tunnel
/// - `btsp.tunnel.status` / `btsp.tunnel_status` — Get tunnel status
/// - `btsp.tunnel.close` / `btsp.tunnel_close` — Close tunnel gracefully
///
/// # Unified Operations (3 new methods, Phase 2+)
///
/// - `btsp.configure_tls` - Configure TLS for external mode tunnel (Phase 3)
/// - `btsp.verify_peer` - Unified trust verification (lineage or certificate) (Phase 3)
/// - `btsp.tunnel_send_http` - Send HTTP request through external tunnel (Phase 3)
///
/// # Backward Compatibility
///
/// All existing BTSP calls work unchanged! Old-style calls automatically default
/// to internal mode (genetic lineage + `btsp_native`).
pub struct BtspHandler;

#[async_trait]
impl MethodHandler for BtspHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Contact exchange (semantic `domain.operation` first; path-like = backward compat)
            "btsp.contact.exchange",
            "beardog./btsp/contact/exchange",
            "btsp.contact_exchange",
            "btsp.contact/exchange",
            // Tunnel establishment (UNIFIED - supports internal + external)
            "btsp.tunnel.establish",
            "beardog./btsp/tunnel/establish",
            "btsp.tunnel_establish",
            "btsp.tunnel/establish",
            // Tunnel encryption
            "btsp.tunnel.encrypt",
            "beardog./btsp/tunnel/encrypt",
            "btsp.tunnel_encrypt",
            "btsp.tunnel/encrypt",
            // Tunnel decryption
            "btsp.tunnel.decrypt",
            "beardog./btsp/tunnel/decrypt",
            "btsp.tunnel_decrypt",
            "btsp.tunnel/decrypt",
            // Tunnel status
            "btsp.tunnel.status",
            "beardog./btsp/tunnel/status",
            "btsp.tunnel_status",
            "btsp.tunnel/status",
            // Tunnel close
            "btsp.tunnel.close",
            "beardog./btsp/tunnel/close",
            "btsp.tunnel_close",
            "btsp.tunnel/close",
            // NEW: Unified BTSP methods (Phase 2+)
            "btsp.configure_tls",    // TLS-specific config (external mode)
            "btsp.verify_peer",      // Unified trust verification
            "btsp.tunnel_send_http", // HTTP request wrapper (external mode)
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        // Match on method (semantic `btsp.*` dot names, legacy underscores, path-like beardog.*)
        if method == "btsp.contact.exchange"
            || method.ends_with("contact_exchange")
            || method.contains("/contact/exchange")
        {
            self.handle_contact_exchange(params, btsp_provider).await
        } else if method == "btsp.tunnel.establish"
            || method.ends_with("tunnel_establish")
            || method.contains("/tunnel/establish")
        {
            self.handle_tunnel_establish(params, btsp_provider).await
        } else if method == "btsp.tunnel.encrypt"
            || method.ends_with("tunnel_encrypt")
            || method.contains("/tunnel/encrypt")
        {
            self.handle_tunnel_encrypt(params, btsp_provider).await
        } else if method == "btsp.tunnel.decrypt"
            || method.ends_with("tunnel_decrypt")
            || method.contains("/tunnel/decrypt")
        {
            self.handle_tunnel_decrypt(params, btsp_provider).await
        } else if method == "btsp.tunnel.status"
            || method.ends_with("tunnel_status")
            || method.contains("/tunnel/status")
        {
            self.handle_tunnel_status(params, btsp_provider).await
        } else if method == "btsp.tunnel.close"
            || method.ends_with("tunnel_close")
            || method.contains("/tunnel/close")
        {
            self.handle_tunnel_close(params, btsp_provider).await
        } else if method == "btsp.configure_tls" {
            self.handle_configure_tls(params, btsp_provider).await
        } else if method == "btsp.verify_peer" {
            self.handle_verify_peer(params, btsp_provider).await
        } else if method == "btsp.tunnel_send_http" {
            self.handle_tunnel_send_http(params, btsp_provider).await
        } else {
            Err(format!("Unknown BTSP method: {method}"))
        }
    }
}

impl BtspHandler {
    /// Handle BTSP contact exchange request
    ///
    /// Exchanges contact information with a peer using genetic lineage for routing.
    async fn handle_contact_exchange(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
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
                Err(format!("Contact exchange failed: {e}"))
            }
        }
    }

    /// Handle BTSP tunnel establishment request (UNIFIED)
    ///
    /// Supports both internal (primal-to-primal) and external (HTTPS API) modes.
    ///
    /// # Backward Compatibility
    ///
    /// Old-style BTSP calls (without `trust_mode/protocol`) automatically default
    /// to internal mode (genetic lineage + `btsp_native`), ensuring 100% compatibility.
    ///
    /// # New Unified Format
    ///
    /// External mode requires explicit `trust_mode` and protocol:
    /// - `trust_mode`: "certificate" (for external HTTPS)
    /// - protocol: "`tls_http`" (for TLS 1.3 + HTTP/2)
    async fn handle_tunnel_establish(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔒 BTSP Tunnel Establish requested (Unified)");

        let params_value = params.ok_or("Missing params for tunnel establish")?;

        // Try parsing as new unified format first (deserialize from `&Value` — no extra `Value` clone)
        if let Ok(unified_params) =
            beardog_types::btsp::TunnelEstablishParams::deserialize(params_value)
        {
            info!("📋 Parsed as unified BTSP parameters");
            return self
                .handle_tunnel_establish_unified(unified_params, btsp_provider)
                .await;
        }

        // Fall back to legacy format for backward compatibility
        info!("📋 Falling back to legacy BTSP format (backward compat)");
        let peer: beardog_capabilities::traits::PeerEndpoint =
            beardog_capabilities::traits::PeerEndpoint::deserialize(params_value)
                .map_err(|e| format!("Invalid peer endpoint (legacy format): {e}"))?;

        match btsp_provider.establish_tunnel(peer).await {
            Ok(handle) => {
                info!("✅ BTSP tunnel established (legacy): {}", handle.id);
                Ok(
                    serde_json::to_value(handle)
                        .map_err(|e| format!("Serialization error: {e}"))?,
                )
            }
            Err(e) => {
                warn!("⚠️  Tunnel establish failed: {}", e);
                Err(format!("Tunnel establish failed: {e}"))
            }
        }
    }

    /// Handle unified BTSP tunnel establishment
    ///
    /// Routes to internal or external mode based on `trust_mode` and protocol.
    async fn handle_tunnel_establish_unified(
        &self,
        params: beardog_types::btsp::TunnelEstablishParams,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        // Detect mode
        let is_internal = params.is_internal();
        let is_external = params.is_external();

        if is_internal {
            info!("🔹 Internal mode: Genetic lineage trust");
            self.handle_tunnel_establish_internal(params, btsp_provider)
                .await
        } else if is_external {
            info!("🔸 External mode: Certificate trust + TLS 1.3");
            self.handle_tunnel_establish_external(params, btsp_provider)
                .await
        } else {
            // Mixed mode (shouldn't happen with proper types, but handle gracefully)
            warn!("⚠️  Mixed mode detected (trust_mode and protocol mismatch)");
            Err("Invalid mode: trust_mode and protocol must both be internal or external".into())
        }
    }

    /// Handle internal mode tunnel establishment (genetic lineage)
    async fn handle_tunnel_establish_internal(
        &self,
        params: beardog_types::btsp::TunnelEstablishParams,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!(
            "🧬 Establishing internal tunnel: {} → {}",
            params.peer_id, params.peer_endpoint
        );

        // Convert to legacy PeerEndpoint for now (existing implementation)
        let peer = beardog_capabilities::traits::PeerEndpoint {
            id: params.peer_id.clone(),
            endpoint: params.peer_endpoint.clone(),
            public_key: None, // Will be discovered during handshake
        };

        match btsp_provider.establish_tunnel(peer).await {
            Ok(handle) => {
                info!("✅ Internal tunnel established: {}", handle.id);

                // Return unified response format
                let response = beardog_types::btsp::TunnelEstablishResponse {
                    tunnel_id: handle.id.clone(),
                    peer_id: handle.peer_id.clone(),
                    mode: "internal".into(),
                    protocol: "btsp_native".into(),
                    established_at: handle.established_at,
                };

                Ok(serde_json::to_value(response)
                    .map_err(|e| format!("Serialization error: {e}"))?)
            }
            Err(e) => {
                warn!("⚠️  Internal tunnel establish failed: {}", e);
                Err(format!("Internal tunnel establish failed: {e}"))
            }
        }
    }

    /// Handle external mode tunnel establishment (TLS 1.3 + certificate trust)
    ///
    /// # Architectural Note
    ///
    /// External mode (HTTPS) is implemented by the **HTTP/TLS-capable peer**, not `BearDog`.
    /// `BearDog` provides the crypto primitives; the transport peer implements TLS/HTTP.
    ///
    /// This follows the **Tower Atomic pattern**: transport peer + `BearDog` = Secure HTTPS
    async fn handle_tunnel_establish_external(
        &self,
        params: beardog_types::btsp::TunnelEstablishParams,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!(
            "🌐 External tunnel requested: {} → {}",
            params.peer_id, params.peer_endpoint
        );
        info!(
            "External mode (HTTPS) is not implemented in BearDog; use the calling primal's BTSP external mode API (Tower Atomic pattern)"
        );

        Err(format!(
            "External mode (HTTPS) is not implemented in BearDog.\n\
             \n\
             BearDog provides crypto primitives via RPC (11 methods already implemented).\n\
             The HTTP/TLS layer runs in the requesting primal using BearDog's crypto.\n\
             \n\
             Tower Atomic Pattern: HTTP (calling primal) + BearDog (crypto) = Secure HTTPS\n\
             \n\
             To use external HTTPS:\n\
             1. Connect to an HTTP-capable primal in your stack\n\
             2. Use that primal's BTSP external mode API\n\
             3. That stack calls BearDog's crypto RPC methods\n\
             \n\
             Requested: {} ({})\n\
             Primal responsibility: BearDog = internal mode + crypto; external HTTPS = calling primal.",
            params.peer_id, params.peer_endpoint
        ))
    }

    /// Handle BTSP tunnel encryption request
    ///
    /// Encrypts data for transmission through the tunnel.
    async fn handle_tunnel_encrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔒 BTSP Tunnel Encrypt requested");

        let params = params.ok_or("Missing params for tunnel encrypt")?;

        let tunnel_val = params.get("tunnel").ok_or("Missing tunnel handle")?;
        let tunnel: beardog_capabilities::traits::TunnelHandle =
            beardog_capabilities::traits::TunnelHandle::deserialize(tunnel_val)
                .map_err(|e| format!("Invalid tunnel handle: {e}"))?;

        let data_b64 = params
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or("Missing data")?;

        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| format!("Invalid base64 data: {e}"))?;

        match btsp_provider.tunnel_encrypt(&tunnel, &data).await {
            Ok(ciphertext) => {
                info!("✅ Data encrypted for tunnel: {}", tunnel.id);
                let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
                Ok(serde_json::json!({
                    "ciphertext": ciphertext_b64
                }))
            }
            Err(e) => {
                warn!("⚠️  Tunnel encrypt failed: {}", e);
                Err(format!("Tunnel encrypt failed: {e}"))
            }
        }
    }

    /// Handle BTSP tunnel decryption request
    ///
    /// Decrypts data received through the tunnel.
    async fn handle_tunnel_decrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔓 BTSP Tunnel Decrypt requested");

        let params = params.ok_or("Missing params for tunnel decrypt")?;

        let tunnel_val = params.get("tunnel").ok_or("Missing tunnel handle")?;
        let tunnel: beardog_capabilities::traits::TunnelHandle =
            beardog_capabilities::traits::TunnelHandle::deserialize(tunnel_val)
                .map_err(|e| format!("Invalid tunnel handle: {e}"))?;

        let data_b64 = params
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or("Missing data")?;

        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| format!("Invalid base64 data: {e}"))?;

        match btsp_provider.tunnel_decrypt(&tunnel, &data).await {
            Ok(plaintext) => {
                info!("✅ Data decrypted for tunnel: {}", tunnel.id);
                let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);
                Ok(serde_json::json!({
                    "plaintext": plaintext_b64
                }))
            }
            Err(e) => {
                warn!("⚠️  Tunnel decrypt failed: {}", e);
                Err(format!("Tunnel decrypt failed: {e}"))
            }
        }
    }

    /// Handle BTSP tunnel status request
    ///
    /// Retrieves the current status of a tunnel.
    async fn handle_tunnel_status(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("📊 BTSP Tunnel Status requested");

        let params = params.ok_or("Missing params for tunnel status")?;

        let tunnel_handle = if let Some(tunnel) = params.get("tunnel") {
            beardog_capabilities::traits::TunnelHandle::deserialize(tunnel)
                .map_err(|e| format!("Invalid tunnel handle: {e}"))?
        } else {
            let tunnel_id = params
                .get("tunnel_id")
                .or_else(|| params.get("id"))
                .and_then(|v| v.as_str())
                .ok_or("Missing tunnel_id or tunnel handle")?;

            beardog_capabilities::traits::TunnelHandle {
                id: tunnel_id.to_string(),
                peer_id: "unknown".to_string(),
                established_at: Utc::now().to_rfc3339(),
            }
        };

        match btsp_provider.tunnel_status(&tunnel_handle).await {
            Ok(status) => {
                info!("✅ Tunnel status retrieved: {}", tunnel_handle.id);
                Ok(
                    serde_json::to_value(status)
                        .map_err(|e| format!("Serialization error: {e}"))?,
                )
            }
            Err(e) => {
                warn!("⚠️  Tunnel status failed: {}", e);
                Err(format!("Tunnel status failed: {e}"))
            }
        }
    }

    /// Handle BTSP tunnel close request
    ///
    /// Gracefully closes a tunnel.
    async fn handle_tunnel_close(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔒 BTSP Tunnel Close requested");

        let params = params.ok_or("Missing params for tunnel close")?;

        let tunnel_handle = if let Some(tunnel) = params.get("tunnel") {
            beardog_capabilities::traits::TunnelHandle::deserialize(tunnel)
                .map_err(|e| format!("Invalid tunnel handle: {e}"))?
        } else {
            let tunnel_id = params
                .get("tunnel_id")
                .or_else(|| params.get("id"))
                .and_then(|v| v.as_str())
                .ok_or("Missing tunnel_id or tunnel handle")?;

            beardog_capabilities::traits::TunnelHandle {
                id: tunnel_id.to_string(),
                peer_id: "unknown".to_string(),
                established_at: Utc::now().to_rfc3339(),
            }
        };

        match btsp_provider.close_tunnel(&tunnel_handle).await {
            Ok(()) => {
                info!("✅ Tunnel closed: {}", tunnel_handle.id);
                Ok(serde_json::json!({
                    "success": true,
                    "tunnel_id": tunnel_handle.id,
                    "message": "Tunnel closed successfully"
                }))
            }
            Err(e) => {
                warn!("⚠️  Tunnel close failed: {}", e);
                Err(format!("Tunnel close failed: {e}"))
            }
        }
    }

    // =========================================================================
    // NEW: Unified BTSP Methods (Phase 2+)
    // =========================================================================

    /// Handle TLS configuration for external mode tunnels
    ///
    /// # Architectural Note
    ///
    /// TLS configuration is part of external mode, handled by the transport peer.
    /// Use the transport peer's BTSP external mode API for TLS configuration.
    async fn handle_configure_tls(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔐 BTSP Configure TLS requested");
        info!(
            "📡 TLS configuration is part of external mode; handled outside BearDog (requesting primal's BTSP external mode API)"
        );

        let params_value = params.ok_or("Missing params for configure_tls")?;

        // Parse parameters for validation
        let _config_params = beardog_types::btsp::ConfigureTlsParams::deserialize(params_value)
            .map_err(|e| format!("Invalid configure_tls params: {e}"))?;

        Err(
            "btsp.configure_tls is part of external mode (HTTPS), not implemented in BearDog.\n\
             \n\
             Use the requesting primal's BTSP external mode API for TLS configuration.\n\
             BearDog provides the crypto primitives via RPC."
                .into(),
        )
    }

    /// Handle unified trust verification
    ///
    /// # Architectural Note
    ///
    /// - **Genetic lineage verification**: Handled by `BearDog` (use existing trust evaluation)
    /// - **Certificate verification**: Handled by the transport peer (external mode)
    ///
    /// For certificate trust, use the transport peer's BTSP external mode API.
    async fn handle_verify_peer(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
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
            _ => Err(format!("Unknown trust_mode: {}", verify_params.trust_mode)),
        }
    }

    /// Handle HTTP request through external mode tunnel
    ///
    /// # Architectural Note
    ///
    /// HTTP operations are part of external mode, handled by the transport peer.
    /// The transport peer implements the HTTP/2 client and uses `BearDog` for TLS crypto.
    async fn handle_tunnel_send_http(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🌐 BTSP Tunnel Send HTTP requested");
        info!(
            "📡 HTTP operations are part of external mode; handled outside BearDog (requesting primal)"
        );

        let params_value = params.ok_or("Missing params for tunnel_send_http")?;

        // Parse parameters for validation
        let _http_params = beardog_types::btsp::TunnelSendHttpParams::deserialize(params_value)
            .map_err(|e| format!("Invalid tunnel_send_http params: {e}"))?;

        Err(
            "btsp.tunnel_send_http is part of external mode (HTTPS), not implemented in BearDog.\n\
             \n\
             The requesting primal typically implements:\n\
             - HTTP/2 client\n\
             - TLS 1.3 handshake (using BearDog crypto RPC)\n\
             - BTSP external mode API\n\
             \n\
             BearDog provides crypto primitives only.\n\
             Use the calling primal for external HTTPS communication."
                .into(),
        )
    }
}

#[cfg(test)]
#[path = "btsp_tests.rs"]
mod tests;
