//! Request handlers for JSON-RPC and HTTP protocols
//!
//! This module contains the core request handling logic for both JSON-RPC
//! and HTTP protocols. All methods are capability-based and primal-agnostic,
//! discovering identity from environment variables at runtime.

use super::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::btsp_provider::BeardogBtspProvider;
use anyhow::Result;
use base64::Engine;
use chrono::Utc;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Handle a JSON-RPC request
///
/// This function parses the request, validates the JSON-RPC version,
/// routes to the appropriate method handler, and builds the response.
///
/// # Arguments
/// * `request` - The JSON-RPC request to handle
/// * `btsp_provider` - The BTSP provider for accessing capabilities
///
/// # Returns
/// A JSON-RPC response with either a result or an error
pub async fn handle_jsonrpc_request(
    request: &JsonRpcRequest,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> JsonRpcResponse {
    debug!("→ JSON-RPC Request: {}", request.method);

    // Validate JSON-RPC version
    if request.jsonrpc != "2.0" {
        return JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: "Invalid JSON-RPC version (must be 2.0)".to_string(),
                data: None,
            }),
            id: request.id.clone().unwrap_or(serde_json::Value::Null),
        };
    }

    // Route to handler
    let result = handle_method(&request.method, request.params.as_ref(), btsp_provider).await;

    // Build response with proper error codes
    match result {
        Ok(value) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(value),
            error: None,
            id: request.id.clone().unwrap_or(serde_json::Value::Null),
        },
        Err(e) => {
            // Detect error type and use appropriate error code
            let (code, message) =
                if e.contains("Unknown method") || e.contains("Method not found") {
                    (JsonRpcError::METHOD_NOT_FOUND, e)
                } else if e.contains("Invalid params") || e.contains("Missing required") {
                    (JsonRpcError::INVALID_PARAMS, e)
                } else {
                    (JsonRpcError::INTERNAL_ERROR, e)
                };

            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code,
                    message,
                    data: None,
                }),
                id: request.id.clone().unwrap_or(serde_json::Value::Null),
            }
        }
    }
}

/// Handle HTTP request and return response string
///
/// # Arguments
/// * `method` - HTTP method (GET, POST, etc.)
/// * `path` - Request path
/// * `btsp_provider` - The BTSP provider for accessing capabilities
///
/// # Returns
/// Complete HTTP response string including headers and body
pub async fn handle_http_request(
    method: &str,
    path: &str,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> String {
    let result = route_http_request(method, path, btsp_provider).await;

    match result {
        Ok(value) => {
            let response_body = serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string());
            format!(
                "HTTP/1.1 200 OK\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: {}\r\n\
                 X-Protocol-Security: low\r\n\
                 X-Recommended-Protocol: json-rpc\r\n\
                 X-Security-Warning: HTTP is less secure than JSON-RPC\r\n\
                 \r\n\
                 {}",
                response_body.len(),
                response_body
            )
        }
        Err(e) => http_error_response(500, &e.to_string()),
    }
}

/// Route HTTP request to handler
async fn route_http_request(
    method: &str,
    path: &str,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value> {
    match (method, path) {
        ("GET", "/ping") | ("GET", "/health") => Ok(serde_json::json!({
            "pong": true,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "protocol_warning": "HTTP is less secure than JSON-RPC",
            "recommended_protocol": "json-rpc"
        })),
        ("GET", "/capabilities") => {
            handle_method("capabilities", None, btsp_provider)
                .await
                .map_err(|e| anyhow::anyhow!(e))
        }
        ("GET", "/metrics/security") => Ok(serde_json::json!({
            "trust_evaluations": 0,
            "encryption_operations": 0,
            "active_sessions": 0,
            "uptime_seconds": 0,
            "protocol_warning": "Consider using JSON-RPC for better security"
        })),
        _ => Err(anyhow::anyhow!("Not found: {} {}", method, path)),
    }
}

/// Build HTTP error response
fn http_error_response(status: u16, message: &str) -> String {
    let status_text = match status {
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Error",
    };

    let body = format!(r#"{{"error":"{}"}}"#, message);

    format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         X-Protocol-Security: low\r\n\
         X-Recommended-Protocol: json-rpc\r\n\
         \r\n\
         {}",
        status,
        status_text,
        body.len(),
        body
    )
}

/// Handle a specific JSON-RPC method
///
/// This function implements capability-based routing that is primal-agnostic.
/// Identity is discovered from environment variables at runtime.
#[allow(clippy::too_many_lines)]
async fn handle_method(
    method: &str,
    params: Option<&serde_json::Value>,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value, String> {
    debug!("📞 Method: {}", method);

    // Parse method into namespace and action for capability-based routing
    let (namespace, action) = if let Some((ns, act)) = method.split_once('.') {
        (ns, act)
    } else {
        ("beardog", method) // Default namespace
    };

    match (namespace, action) {
        // ========================================================================
        // UNIVERSAL CAPABILITY-BASED METHODS (Primal Agnostic)
        // ========================================================================

        // Ping/Health - Universal across all primals
        (_, "ping") | (_, "health") | (_, "status") | (_, "check") => {
            info!("🏥 Health check requested");
            Ok(serde_json::json!({
                "status": "healthy",
                "primal": "beardog",
                "version": env!("CARGO_PKG_VERSION"),
                "protocol": "JSON-RPC",
                "timestamp": Utc::now().to_rfc3339(),
            }))
        }

        // Capabilities - Self-description (every primal should provide this)
        (_, "capabilities") | (_, "get_capabilities") => {
            // Get identity from environment (primal only knows itself)
            // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
            let family_id = std::env::var("FAMILY_ID")
                .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                .unwrap_or_else(|_| "unknown".to_string());
            let node_id = std::env::var("NODE_ID")
                .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                .unwrap_or_else(|_| "unknown".to_string());

            info!("🎯 Capabilities requested - exposing our capabilities");
            Ok(serde_json::json!({
                "primal": "beardog",
                "family_id": family_id,
                "node_id": node_id,
                "provided_capabilities": [
                    {
                        "type": "security",
                        "version": "1.0",
                        "methods": ["evaluate", "lineage"],
                    },
                    {
                        "type": "encryption",
                        "version": "1.0",
                        "methods": ["encrypt", "decrypt"],
                    },
                    {
                        "type": "trust",
                        "version": "1.0",
                        "methods": ["evaluate", "lineage"],
                    },
                    {
                        "type": "btsp",
                        "version": "1.0",
                        "methods": ["contact_exchange", "tunnel_establish", "tunnel_encrypt", "tunnel_decrypt", "tunnel_status", "tunnel_close"],
                        "description": "BearDog Tunnel Security Protocol - VPN-free P2P mesh via genetic lineage"
                    },
                    {
                        "type": "graph",
                        "version": "1.0",
                        "methods": ["authorize_modification", "validate_template", "audit_origin"],
                        "description": "Collaborative Intelligence graph security - 5-layer authorization, threat detection, and provenance"
                    }
                ],
                "version": env!("CARGO_PKG_VERSION"),
                "protocols": ["tarpc", "json-rpc", "http"],
                "btsp_enabled": true,
                "collaborative_intelligence": true,
            }))
        }

        // Identity - Self-identification (genetic lineage)
        (_, "identity") | (_, "whoami") | (_, "get_identity") => {
            // Get identity from environment (primal only knows itself)
            // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
            let family_id = std::env::var("FAMILY_ID")
                .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                .unwrap_or_else(|_| "unknown".to_string());
            let node_id = std::env::var("NODE_ID")
                .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                .unwrap_or_else(|_| "unknown".to_string());

            // Generate encryption tag for discovery/federation
            // Format: beardog:family:{family_id} for family-based federation
            let encryption_tag = format!("beardog:family:{}", family_id);

            info!("🆔 Identity requested - family: {}, node: {}, encryption_tag: {}",
                family_id, node_id, encryption_tag);

            Ok(serde_json::json!({
                "primal": "beardog",
                "family": family_id,
                "node": node_id,
                "encryption_tag": encryption_tag,
                "version": env!("CARGO_PKG_VERSION"),
            }))
        }

        // ========================================================================
        // SECURITY CAPABILITY METHODS (BearDog's specialty)
        // ========================================================================

        // Trust evaluation - capability-based, not primal-specific
        ("security", "evaluate") | ("trust", "evaluate") | ("security", "evaluate_trust") | ("trust", "evaluate_peer") => {
            let params = params.ok_or("Missing params for trust evaluation")?;

            // Flexible parameter extraction (works with any primal's naming)
            let peer_id = params.get("peer_id")
                .or_else(|| params.get("id"))
                .or_else(|| params.get("peer"))
                .and_then(|v| v.as_str())
                .ok_or("Missing peer identifier")?;

            let peer_family = params.get("peer_family")
                .or_else(|| params.get("family"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Get our identity from environment (primal only knows itself)
            // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
            let our_family = std::env::var("FAMILY_ID")
                .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                .unwrap_or_else(|_| "unknown".to_string());
            let our_node = std::env::var("NODE_ID")
                .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                .unwrap_or_else(|_| "unknown".to_string());

            // Phase 1: Dual representation with capability hints + Songbird decision field (Jan 7, 2026)
            let (trust_level, trust_level_name, decision, reason, allowed_caps, denied_caps) = if peer_family == our_family {
                info!("✅ Trust: SAME FAMILY - level 1 (limited) - peer: {}, family: {}", peer_id, peer_family);
                (
                    1,
                    "limited",
                    "auto_accept",  // Same family = auto-accept for genetic lineage
                    "same_genetic_family",
                    vec!["birdsong/*", "coordination/*", "health", "capabilities", "discovery"],
                    vec!["data/*", "commands/*", "keys/*", "federation/admin"],
                )
            } else if !peer_family.is_empty() {
                info!("⚠️  Trust: DIFFERENT FAMILY - level 0 (none) - peer: {}, their: {}, ours: {}", peer_id, peer_family, our_family);
                (
                    0,
                    "none",
                    "reject",  // Different family = reject
                    "different_family",
                    vec![],
                    vec!["*"],
                )
            } else {
                info!("⚠️  Trust: UNKNOWN FAMILY - level 0 (none) - peer: {}", peer_id);
                (
                    0,
                    "none",
                    "reject",  // Unknown family = reject
                    "unknown_family",
                    vec![],
                    vec!["*"],
                )
            };

            // Phase 1 Response: Dual representation (int + string) with capability hints + decision field
            Ok(serde_json::json!({
                "decision": decision,                  // Songbird requires this field
                "trust_level": trust_level,           // Integer (compact, backward compat)
                "trust_level_name": trust_level_name, // String (Songbird expects this)
                "reason": reason,
                "peer_id": peer_id,
                "peer_family": peer_family,
                "our_family": our_family,
                "our_node": our_node,
                "evaluated_by": "beardog",
                "capabilities": {                      // Capability hints (Phase 1)
                    "allowed": allowed_caps,
                    "denied": denied_caps,
                },
                "metadata": {
                    "policy_version": 1,
                    "evaluation_method": "genetic_family_match",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }
            }))
        }

        // Lineage information
        ("security", "lineage") | ("trust", "lineage") | ("security", "get_lineage") | ("trust", "get_lineage") => {
            // Get identity from environment (primal only knows itself)
            // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
            let family_id = std::env::var("FAMILY_ID")
                .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                .unwrap_or_else(|_| "unknown".to_string());
            let node_id = std::env::var("NODE_ID")
                .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                .unwrap_or_else(|_| "unknown".to_string());

            // Generate encryption tag for consistency
            let encryption_tag = format!("beardog:family:{}", family_id);

            info!("🌳 Lineage info requested - family: {}, node: {}", family_id, node_id);
            Ok(serde_json::json!({
                "primal": "beardog",
                "family": family_id,
                "node": node_id,
                "encryption_tag": encryption_tag,
                "generation": 0,
                "parent": null,
                "capabilities": ["security", "encryption", "trust"],
            }))
        }

        // Note: Legacy beardog.ping is handled by the universal ping handler above

        // BirdSong encryption (used by Songbird for secure discovery)
        ("beardog", "birdsong.encrypt") | ("birdsong", "encrypt") => {
            let params = params.ok_or("Missing params")?;
            let plaintext = params["plaintext"]
                .as_str()
                .ok_or("Missing plaintext")?;
            let family_id = params["family_id"]
                .as_str()
                .ok_or("Missing family_id")?;

            // Decode base64 plaintext
            let plaintext_bytes = base64::engine::general_purpose::STANDARD
                .decode(plaintext)
                .map_err(|e| format!("Invalid base64: {}", e))?;

            // Encrypt using BirdSong
            let ciphertext = btsp_provider
                .birdsong_manager()
                .encrypt_discovery_for_family(&plaintext_bytes, family_id)
                .map_err(|e| format!("Encryption failed: {}", e))?;

            // Encode to base64
            let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);

            Ok(serde_json::json!({
                "ciphertext": ciphertext_b64,
                "family_id": family_id,
            }))
        }

        // BirdSong decryption (used by Songbird for secure discovery)
        ("beardog", "birdsong.decrypt") | ("birdsong", "decrypt") => {
            let params = params.ok_or("Missing params")?;
            let ciphertext = params["ciphertext"]
                .as_str()
                .ok_or("Missing ciphertext")?;
            let family_id = params["family_id"]
                .as_str()
                .ok_or("Missing family_id")?;

            // Decode base64 ciphertext
            let ciphertext_bytes = base64::engine::general_purpose::STANDARD
                .decode(ciphertext)
                .map_err(|e| format!("Invalid base64: {}", e))?;

            // Decrypt using BirdSong
            match btsp_provider
                .birdsong_manager()
                .decrypt_discovery_from_family(&ciphertext_bytes, family_id)
            {
                Ok(plaintext) => {
                    // Encode to base64
                    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);

                    Ok(serde_json::json!({
                        "plaintext": plaintext_b64,
                        "family_id": family_id,
                        "success": true,
                    }))
                }
                Err(e) => {
                    // Graceful privacy: Not an error, just "not for us"
                    debug!("🔇 Decryption failed (different family): {}", e);
                    Ok(serde_json::json!({
                        "plaintext": "",
                        "family_id": family_id,
                        "success": false,
                    }))
                }
            }
        }

        // ========================================================================
        // BTSP (BearDog Tunnel Security Protocol) METHODS
        // ========================================================================

        // BTSP Contact Exchange
        ("beardog", "/btsp/contact/exchange") | ("btsp", "contact_exchange") | ("btsp", "contact/exchange") => {
            info!("🔍 BTSP Contact Exchange requested");

            let params = params.ok_or("Missing params for contact exchange")?;

            let target_peer_id = params.get("target_peer_id")
                .or_else(|| params.get("peer_id"))
                .and_then(|v| v.as_str())
                .ok_or("Missing target_peer_id")?;

            let requester_lineage = params.get("requester_lineage")
                .or_else(|| params.get("lineage"))
                .and_then(|v| v.as_str())
                .ok_or("Missing requester_lineage")?;

            let max_hops = params.get("max_hops")
                .and_then(|v| v.as_u64())
                .unwrap_or(3) as usize;

            match btsp_provider.contact_exchange(target_peer_id, requester_lineage, max_hops).await {
                Ok(contact_info) => {
                    info!("✅ Contact exchange successful for peer: {}", target_peer_id);
                    Ok(serde_json::to_value(contact_info).map_err(|e| format!("Serialization error: {}", e))?)
                }
                Err(e) => {
                    warn!("⚠️  Contact exchange failed: {}", e);
                    Err(format!("Contact exchange failed: {}", e))
                }
            }
        }

        // BTSP Tunnel Establish
        ("beardog", "/btsp/tunnel/establish") | ("btsp", "tunnel_establish") | ("btsp", "tunnel/establish") => {
            info!("🔒 BTSP Tunnel Establish requested");

            let params = params.ok_or("Missing params for tunnel establish")?;

            let peer: beardog_capabilities::traits::PeerEndpoint = serde_json::from_value(params.clone())
                .map_err(|e| format!("Invalid peer endpoint: {}", e))?;

            use beardog_capabilities::traits::SecureTunnelProvider;
            match btsp_provider.establish_tunnel(peer).await {
                Ok(handle) => {
                    info!("✅ BTSP tunnel established: {}", handle.id);
                    Ok(serde_json::to_value(handle).map_err(|e| format!("Serialization error: {}", e))?)
                }
                Err(e) => {
                    warn!("⚠️  Tunnel establish failed: {}", e);
                    Err(format!("Tunnel establish failed: {}", e))
                }
            }
        }

        // BTSP Tunnel Encrypt
        ("beardog", "/btsp/tunnel/encrypt") | ("btsp", "tunnel_encrypt") | ("btsp", "tunnel/encrypt") => {
            info!("🔒 BTSP Tunnel Encrypt requested");

            let params = params.ok_or("Missing params for tunnel encrypt")?;

            let tunnel: beardog_capabilities::traits::TunnelHandle = serde_json::from_value(
                params.get("tunnel")
                    .ok_or("Missing tunnel handle")?
                    .clone()
            ).map_err(|e| format!("Invalid tunnel handle: {}", e))?;

            let data_b64 = params.get("data")
                .and_then(|v| v.as_str())
                .ok_or("Missing data")?;

            let data = base64::engine::general_purpose::STANDARD
                .decode(data_b64)
                .map_err(|e| format!("Invalid base64 data: {}", e))?;

            use beardog_capabilities::traits::SecureTunnelProvider;
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

        // BTSP Tunnel Decrypt
        ("beardog", "/btsp/tunnel/decrypt") | ("btsp", "tunnel_decrypt") | ("btsp", "tunnel/decrypt") => {
            info!("🔓 BTSP Tunnel Decrypt requested");

            let params = params.ok_or("Missing params for tunnel decrypt")?;

            let tunnel: beardog_capabilities::traits::TunnelHandle = serde_json::from_value(
                params.get("tunnel")
                    .ok_or("Missing tunnel handle")?
                    .clone()
            ).map_err(|e| format!("Invalid tunnel handle: {}", e))?;

            let data_b64 = params.get("data")
                .and_then(|v| v.as_str())
                .ok_or("Missing data")?;

            let data = base64::engine::general_purpose::STANDARD
                .decode(data_b64)
                .map_err(|e| format!("Invalid base64 data: {}", e))?;

            use beardog_capabilities::traits::SecureTunnelProvider;
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

        // BTSP Tunnel Status
        ("beardog", "/btsp/tunnel/status") | ("btsp", "tunnel_status") | ("btsp", "tunnel/status") => {
            info!("📊 BTSP Tunnel Status requested");

            let params = params.ok_or("Missing params for tunnel status")?;

            let tunnel_handle = if let Some(tunnel) = params.get("tunnel") {
                serde_json::from_value(tunnel.clone())
                    .map_err(|e| format!("Invalid tunnel handle: {}", e))?
            } else {
                let tunnel_id = params.get("tunnel_id")
                    .or_else(|| params.get("id"))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing tunnel_id or tunnel handle")?;

                beardog_capabilities::traits::TunnelHandle {
                    id: tunnel_id.to_string(),
                    peer_id: "unknown".to_string(),
                    established_at: Utc::now().to_rfc3339(),
                }
            };

            use beardog_capabilities::traits::SecureTunnelProvider;
            match btsp_provider.tunnel_status(&tunnel_handle).await {
                Ok(status) => {
                    info!("✅ Tunnel status retrieved: {}", tunnel_handle.id);
                    Ok(serde_json::to_value(status).map_err(|e| format!("Serialization error: {}", e))?)
                }
                Err(e) => {
                    warn!("⚠️  Tunnel status failed: {}", e);
                    Err(format!("Tunnel status failed: {}", e))
                }
            }
        }

        // BTSP Tunnel Close
        ("beardog", "/btsp/tunnel/close") | ("btsp", "tunnel_close") | ("btsp", "tunnel/close") => {
            info!("🔒 BTSP Tunnel Close requested");

            let params = params.ok_or("Missing params for tunnel close")?;

            let tunnel_handle = if let Some(tunnel) = params.get("tunnel") {
                serde_json::from_value(tunnel.clone())
                    .map_err(|e| format!("Invalid tunnel handle: {}", e))?
            } else {
                let tunnel_id = params.get("tunnel_id")
                    .or_else(|| params.get("id"))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing tunnel_id or tunnel handle")?;

                beardog_capabilities::traits::TunnelHandle {
                    id: tunnel_id.to_string(),
                    peer_id: "unknown".to_string(),
                    established_at: Utc::now().to_rfc3339(),
                }
            };

            use beardog_capabilities::traits::SecureTunnelProvider;
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

        // Unknown method
        _ => {
            warn!("⚠️  Unknown method: {}.{}", namespace, action);
            Err(format!("Method not found: {}.{}", namespace, action))
        }
    }
}
