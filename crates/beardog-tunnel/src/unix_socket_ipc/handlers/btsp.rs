//! BTSP (BearDog Tunnel Security Protocol) handlers - UNIFIED
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
use std::sync::Arc;
use tracing::{info, warn};

/// Handler for BTSP Unified methods
///
/// # Core Operations (6 methods, internal mode)
///
/// - `btsp.contact_exchange` - Exchange contact info via genetic lineage
/// - `btsp.tunnel_establish` - **UNIFIED**: Establish secure tunnel (internal OR external)
/// - `btsp.tunnel_encrypt` - Encrypt data through tunnel
/// - `btsp.tunnel_decrypt` - Decrypt data from tunnel
/// - `btsp.tunnel_status` - Get tunnel status
/// - `btsp.tunnel_close` - Close tunnel gracefully
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
/// to internal mode (genetic lineage + btsp_native).
pub struct BtspHandler;

#[async_trait]
impl MethodHandler for BtspHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Contact exchange
            "beardog./btsp/contact/exchange",
            "btsp.contact_exchange",
            "btsp.contact/exchange",
            // Tunnel establishment (UNIFIED - supports internal + external)
            "beardog./btsp/tunnel/establish",
            "btsp.tunnel_establish",
            "btsp.tunnel/establish",
            // Tunnel encryption
            "beardog./btsp/tunnel/encrypt",
            "btsp.tunnel_encrypt",
            "btsp.tunnel/encrypt",
            // Tunnel decryption
            "beardog./btsp/tunnel/decrypt",
            "btsp.tunnel_decrypt",
            "btsp.tunnel/decrypt",
            // Tunnel status
            "beardog./btsp/tunnel/status",
            "btsp.tunnel_status",
            "btsp.tunnel/status",
            // Tunnel close
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
        // Match on method (ignoring namespace variations)
        if method.ends_with("contact_exchange") || method.contains("/contact/exchange") {
            self.handle_contact_exchange(params, btsp_provider).await
        } else if method.ends_with("tunnel_establish") || method.contains("/tunnel/establish") {
            self.handle_tunnel_establish(params, btsp_provider).await
        } else if method.ends_with("tunnel_encrypt") || method.contains("/tunnel/encrypt") {
            self.handle_tunnel_encrypt(params, btsp_provider).await
        } else if method.ends_with("tunnel_decrypt") || method.contains("/tunnel/decrypt") {
            self.handle_tunnel_decrypt(params, btsp_provider).await
        } else if method.ends_with("tunnel_status") || method.contains("/tunnel/status") {
            self.handle_tunnel_status(params, btsp_provider).await
        } else if method.ends_with("tunnel_close") || method.contains("/tunnel/close") {
            self.handle_tunnel_close(params, btsp_provider).await
        } else if method == "btsp.configure_tls" {
            self.handle_configure_tls(params, btsp_provider).await
        } else if method == "btsp.verify_peer" {
            self.handle_verify_peer(params, btsp_provider).await
        } else if method == "btsp.tunnel_send_http" {
            self.handle_tunnel_send_http(params, btsp_provider).await
        } else {
            Err(format!("Unknown BTSP method: {}", method))
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

        let max_hops = params.get("max_hops").and_then(|v| v.as_u64()).unwrap_or(3) as usize;

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
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Contact exchange failed: {}", e);
                Err(format!("Contact exchange failed: {}", e))
            }
        }
    }

    /// Handle BTSP tunnel establishment request (UNIFIED)
    ///
    /// Supports both internal (primal-to-primal) and external (HTTPS API) modes.
    ///
    /// # Backward Compatibility
    ///
    /// Old-style BTSP calls (without trust_mode/protocol) automatically default
    /// to internal mode (genetic lineage + btsp_native), ensuring 100% compatibility.
    ///
    /// # New Unified Format
    ///
    /// External mode requires explicit trust_mode and protocol:
    /// - trust_mode: "certificate" (for external HTTPS)
    /// - protocol: "tls_http" (for TLS 1.3 + HTTP/2)
    async fn handle_tunnel_establish(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔒 BTSP Tunnel Establish requested (Unified)");

        let params_value = params.ok_or("Missing params for tunnel establish")?;

        // Try parsing as new unified format first
        if let Ok(unified_params) = serde_json::from_value::<
            beardog_types::btsp::TunnelEstablishParams,
        >(params_value.clone())
        {
            info!("📋 Parsed as unified BTSP parameters");
            return self
                .handle_tunnel_establish_unified(unified_params, btsp_provider)
                .await;
        }

        // Fall back to legacy format for backward compatibility
        info!("📋 Falling back to legacy BTSP format (backward compat)");
        let peer: beardog_capabilities::traits::PeerEndpoint =
            serde_json::from_value(params_value.clone())
                .map_err(|e| format!("Invalid peer endpoint (legacy format): {}", e))?;

        match btsp_provider.establish_tunnel(peer).await {
            Ok(handle) => {
                info!("✅ BTSP tunnel established (legacy): {}", handle.id);
                Ok(serde_json::to_value(handle)
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Tunnel establish failed: {}", e);
                Err(format!("Tunnel establish failed: {}", e))
            }
        }
    }

    /// Handle unified BTSP tunnel establishment
    ///
    /// Routes to internal or external mode based on trust_mode and protocol.
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
                    established_at: handle.established_at.clone(),
                };

                Ok(serde_json::to_value(response)
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Internal tunnel establish failed: {}", e);
                Err(format!("Internal tunnel establish failed: {}", e))
            }
        }
    }

    /// Handle external mode tunnel establishment (TLS 1.3 + certificate trust)
    ///
    /// # Architectural Note
    ///
    /// External mode (HTTPS) is implemented by **Songbird**, not BearDog.
    /// BearDog provides the crypto primitives, Songbird implements the TLS/HTTP layer.
    ///
    /// This follows the **Tower Atomic pattern**: Songbird + BearDog = Secure HTTPS
    async fn handle_tunnel_establish_external(
        &self,
        params: beardog_types::btsp::TunnelEstablishParams,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!(
            "🌐 External tunnel requested: {} → {}",
            params.peer_id, params.peer_endpoint
        );
        info!("📡 External mode is handled by Songbird (Tower Atomic pattern)");

        Err(format!(
            "External mode (HTTPS) is handled by Songbird, not BearDog.\n\
             \n\
             BearDog provides crypto primitives via RPC (11 methods already implemented).\n\
             Songbird implements TLS 1.3 + HTTP/2 using BearDog's crypto.\n\
             \n\
             Tower Atomic Pattern: Songbird (HTTP) + BearDog (Crypto) = Secure HTTPS\n\
             \n\
             To use external HTTPS:\n\
             1. Connect to Songbird (HTTP-capable primal)\n\
             2. Use Songbird's BTSP external mode API\n\
             3. Songbird will call BearDog's crypto RPC methods\n\
             \n\
             Requested: {} ({})\n\
             Primal Responsibility: BearDog = Internal Mode + Crypto | Songbird = External Mode + HTTP",
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

        let tunnel: beardog_capabilities::traits::TunnelHandle =
            serde_json::from_value(params.get("tunnel").ok_or("Missing tunnel handle")?.clone())
                .map_err(|e| format!("Invalid tunnel handle: {}", e))?;

        let data_b64 = params
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or("Missing data")?;

        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| format!("Invalid base64 data: {}", e))?;

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
                Err(format!("Tunnel encrypt failed: {}", e))
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

        let tunnel: beardog_capabilities::traits::TunnelHandle =
            serde_json::from_value(params.get("tunnel").ok_or("Missing tunnel handle")?.clone())
                .map_err(|e| format!("Invalid tunnel handle: {}", e))?;

        let data_b64 = params
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or("Missing data")?;

        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| format!("Invalid base64 data: {}", e))?;

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
                Err(format!("Tunnel decrypt failed: {}", e))
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
            serde_json::from_value(tunnel.clone())
                .map_err(|e| format!("Invalid tunnel handle: {}", e))?
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
                Ok(serde_json::to_value(status)
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Tunnel status failed: {}", e);
                Err(format!("Tunnel status failed: {}", e))
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
            serde_json::from_value(tunnel.clone())
                .map_err(|e| format!("Invalid tunnel handle: {}", e))?
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
            Ok(_) => {
                info!("✅ Tunnel closed: {}", tunnel_handle.id);
                Ok(serde_json::json!({
                    "success": true,
                    "tunnel_id": tunnel_handle.id,
                    "message": "Tunnel closed successfully"
                }))
            }
            Err(e) => {
                warn!("⚠️  Tunnel close failed: {}", e);
                Err(format!("Tunnel close failed: {}", e))
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
    /// TLS configuration is part of external mode, which is handled by **Songbird**.
    /// Use Songbird's BTSP external mode API for TLS configuration.
    async fn handle_configure_tls(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔐 BTSP Configure TLS requested");
        info!("📡 TLS configuration is handled by Songbird (external mode)");

        let params_value = params.ok_or("Missing params for configure_tls")?;

        // Parse parameters for validation
        let _config_params =
            serde_json::from_value::<beardog_types::btsp::ConfigureTlsParams>(params_value.clone())
                .map_err(|e| format!("Invalid configure_tls params: {}", e))?;

        Err(
            "btsp.configure_tls is part of external mode (HTTPS), handled by Songbird.\n\
             \n\
             Use Songbird's BTSP external mode API for TLS configuration.\n\
             BearDog provides the crypto primitives via RPC."
                .into(),
        )
    }

    /// Handle unified trust verification
    ///
    /// # Architectural Note
    ///
    /// - **Genetic lineage verification**: Handled by BearDog (use existing trust evaluation)
    /// - **Certificate verification**: Handled by Songbird (external mode)
    ///
    /// For certificate trust, use Songbird's BTSP external mode API.
    async fn handle_verify_peer(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔍 BTSP Verify Peer requested");

        let params_value = params.ok_or("Missing params for verify_peer")?;

        // Parse parameters
        let verify_params =
            serde_json::from_value::<beardog_types::btsp::VerifyPeerParams>(params_value.clone())
                .map_err(|e| format!("Invalid verify_peer params: {}", e))?;

        // Route based on trust mode
        match verify_params.trust_mode.as_str() {
            "genetic_lineage" => {
                // TODO: Implement using existing BTSP trust evaluation
                Err("btsp.verify_peer (genetic_lineage) not yet implemented.\n\
                     Use existing BTSP internal mode trust evaluation."
                    .into())
            }
            "certificate" => {
                info!("📡 Certificate verification is handled by Songbird (external mode)");
                Err("btsp.verify_peer (certificate) is part of external mode, handled by Songbird.\n\
                     \n\
                     Use Songbird's BTSP external mode API for certificate verification.\n\
                     BearDog provides crypto primitives (tls.verify_certificate RPC method).".into())
            }
            _ => Err(format!("Unknown trust_mode: {}", verify_params.trust_mode)),
        }
    }

    /// Handle HTTP request through external mode tunnel
    ///
    /// # Architectural Note
    ///
    /// HTTP operations are part of external mode, which is handled by **Songbird**.
    /// Songbird implements HTTP/2 client and uses BearDog for TLS crypto.
    async fn handle_tunnel_send_http(
        &self,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🌐 BTSP Tunnel Send HTTP requested");
        info!("📡 HTTP operations are handled by Songbird (external mode)");

        let params_value = params.ok_or("Missing params for tunnel_send_http")?;

        // Parse parameters for validation
        let _http_params = serde_json::from_value::<beardog_types::btsp::TunnelSendHttpParams>(
            params_value.clone(),
        )
        .map_err(|e| format!("Invalid tunnel_send_http params: {}", e))?;

        Err(
            "btsp.tunnel_send_http is part of external mode (HTTPS), handled by Songbird.\n\
             \n\
             Songbird implements:\n\
             - HTTP/2 client\n\
             - TLS 1.3 handshake (using BearDog crypto RPC)\n\
             - BTSP external mode API\n\
             \n\
             BearDog provides crypto primitives only.\n\
             Use Songbird for all external HTTPS communication."
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_btsp_handler_methods() {
        let handler = BtspHandler;
        let methods = handler.methods();

        // Should have:
        // - 6 core operations × 3 aliases each = 18 methods
        // - 3 new unified methods = 3 methods
        // Total = 21 methods
        assert_eq!(methods.len(), 21);

        // Check core operations have their aliases
        assert!(methods.contains(&"btsp.contact_exchange"));
        assert!(methods.contains(&"btsp.tunnel_establish"));
        assert!(methods.contains(&"btsp.tunnel_encrypt"));
        assert!(methods.contains(&"btsp.tunnel_decrypt"));
        assert!(methods.contains(&"btsp.tunnel_status"));
        assert!(methods.contains(&"btsp.tunnel_close"));

        // Check new unified methods
        assert!(methods.contains(&"btsp.configure_tls"));
        assert!(methods.contains(&"btsp.verify_peer"));
        assert!(methods.contains(&"btsp.tunnel_send_http"));
    }

    // Note: Full integration tests require a working BTSP provider
    // These would be added as integration tests in the tunnel crate
}
