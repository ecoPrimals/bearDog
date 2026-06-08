// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tunnel operations, unified external-mode stubs, and legacy tunnel crypto (`btsp.tunnel.*`, `btsp.configure_tls`, `btsp.tunnel_send_http`).

use crate::btsp_provider::BeardogBtspProvider;
use base64::Engine;
use beardog_capabilities::traits::SecureTunnelProvider;
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
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
    pub(super) async fn handle_tunnel_establish(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
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
                Err(format!("Tunnel establish failed: {e}").into())
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
    ) -> Result<serde_json::Value, HandlerError> {
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
    ) -> Result<serde_json::Value, HandlerError> {
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
                Err(format!("Internal tunnel establish failed: {e}").into())
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
    ) -> Result<serde_json::Value, HandlerError> {
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
        ).into())
    }

    /// Handle BTSP tunnel encryption request
    ///
    /// Encrypts data for transmission through the tunnel.
    pub(super) async fn handle_tunnel_encrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
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
                Err(format!("Tunnel encrypt failed: {e}").into())
            }
        }
    }

    /// Handle BTSP tunnel decryption request
    ///
    /// Decrypts data received through the tunnel.
    pub(super) async fn handle_tunnel_decrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
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
                Err(format!("Tunnel decrypt failed: {e}").into())
            }
        }
    }

    /// Resolve a `TunnelHandle` from params that may carry a full handle object or
    /// just a `tunnel_id` / `id` string field.
    fn resolve_tunnel_handle(
        params: &serde_json::Value,
    ) -> Result<beardog_capabilities::traits::TunnelHandle, String> {
        if let Some(tunnel) = params.get("tunnel") {
            beardog_capabilities::traits::TunnelHandle::deserialize(tunnel)
                .map_err(|e| format!("Invalid tunnel handle: {e}"))
        } else {
            let tunnel_id = params
                .get("tunnel_id")
                .or_else(|| params.get("id"))
                .and_then(|v| v.as_str())
                .ok_or("Missing tunnel_id or tunnel handle")?;
            Ok(beardog_capabilities::traits::TunnelHandle {
                id: tunnel_id.to_string(),
                peer_id: "unknown".to_string(),
                established_at: Utc::now().to_rfc3339(),
            })
        }
    }

    /// Handle BTSP tunnel status request
    pub(super) async fn handle_tunnel_status(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("📊 BTSP Tunnel Status requested");
        let params = params.ok_or("Missing params for tunnel status")?;
        let tunnel_handle = Self::resolve_tunnel_handle(params)?;

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
                Err(format!("Tunnel status failed: {e}").into())
            }
        }
    }

    /// Handle BTSP tunnel close request
    pub(super) async fn handle_tunnel_close(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("🔒 BTSP Tunnel Close requested");
        let params = params.ok_or("Missing params for tunnel close")?;
        let tunnel_handle = Self::resolve_tunnel_handle(params)?;

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
                Err(format!("Tunnel close failed: {e}").into())
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
    pub(super) async fn handle_configure_tls(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
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

    /// Handle HTTP request through external mode tunnel
    ///
    /// # Architectural Note
    ///
    /// HTTP operations are part of external mode, handled by the transport peer.
    /// The transport peer implements the HTTP/2 client and uses `BearDog` for TLS crypto.
    pub(super) async fn handle_tunnel_send_http(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
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
