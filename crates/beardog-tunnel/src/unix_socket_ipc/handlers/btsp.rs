//! BTSP (BearDog Tunnel Security Protocol) handlers
//!
//! Provides secure P2P mesh tunneling via genetic lineage.
//! No VPN required - uses family-based trust for contact exchange and tunnel establishment.

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use base64::Engine;
use beardog_capabilities::traits::SecureTunnelProvider;
use chrono::Utc;
use std::sync::Arc;
use tracing::{info, warn};

/// Handler for BTSP tunnel methods
///
/// Supports all BTSP operations:
/// - `btsp.contact_exchange` - Exchange contact info via genetic lineage
/// - `btsp.tunnel_establish` - Establish secure P2P tunnel
/// - `btsp.tunnel_encrypt` - Encrypt data through tunnel
/// - `btsp.tunnel_decrypt` - Decrypt data from tunnel
/// - `btsp.tunnel_status` - Get tunnel status
/// - `btsp.tunnel_close` - Close tunnel gracefully
pub struct BtspHandler;

#[async_trait]
impl MethodHandler for BtspHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Contact exchange
            "beardog./btsp/contact/exchange",
            "btsp.contact_exchange",
            "btsp.contact/exchange",
            // Tunnel establishment
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
                info!("✅ Contact exchange successful for peer: {}", target_peer_id);
                Ok(serde_json::to_value(contact_info)
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Contact exchange failed: {}", e);
                Err(format!("Contact exchange failed: {}", e))
            }
        }
    }

    /// Handle BTSP tunnel establishment request
    ///
    /// Establishes a secure P2P tunnel to a peer endpoint.
    async fn handle_tunnel_establish(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🔒 BTSP Tunnel Establish requested");

        let params = params.ok_or("Missing params for tunnel establish")?;

        let peer: beardog_capabilities::traits::PeerEndpoint =
            serde_json::from_value(params.clone())
                .map_err(|e| format!("Invalid peer endpoint: {}", e))?;

        match btsp_provider.establish_tunnel(peer).await {
            Ok(handle) => {
                info!("✅ BTSP tunnel established: {}", handle.id);
                Ok(serde_json::to_value(handle)
                    .map_err(|e| format!("Serialization error: {}", e))?)
            }
            Err(e) => {
                warn!("⚠️  Tunnel establish failed: {}", e);
                Err(format!("Tunnel establish failed: {}", e))
            }
        }
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

        let tunnel: beardog_capabilities::traits::TunnelHandle = serde_json::from_value(
            params.get("tunnel").ok_or("Missing tunnel handle")?.clone(),
        )
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

        let tunnel: beardog_capabilities::traits::TunnelHandle = serde_json::from_value(
            params.get("tunnel").ok_or("Missing tunnel handle")?.clone(),
        )
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_btsp_handler_methods() {
        let handler = BtspHandler;
        let methods = handler.methods();

        // Should have 6 operations × 3 aliases each = 18 methods
        assert_eq!(methods.len(), 18);

        // Check each operation has its aliases
        assert!(methods.contains(&"btsp.contact_exchange"));
        assert!(methods.contains(&"btsp.tunnel_establish"));
        assert!(methods.contains(&"btsp.tunnel_encrypt"));
        assert!(methods.contains(&"btsp.tunnel_decrypt"));
        assert!(methods.contains(&"btsp.tunnel_status"));
        assert!(methods.contains(&"btsp.tunnel_close"));
    }

    // Note: Full integration tests require a working BTSP provider
    // These would be added as integration tests in the tunnel crate
}

