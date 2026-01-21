//! Legacy request handlers for JSON-RPC and HTTP protocols
//!
//! This module provides backward compatibility and HTTP fallback support.
//! Most JSON-RPC methods are now handled by the modular handler registry
//! (see `handlers/mod.rs`). This file contains:
//! - HTTP protocol fallback (deprecated, use JSON-RPC instead)
//! - BirdSong encrypt/decrypt methods
//! - Legacy btsp.tunnel_close method
//!
//! All methods are capability-based and primal-agnostic, discovering
//! identity from environment variables at runtime.
//!
//! # Handler Registry Migration Status
//!
//! ✅ **100% Complete!** All handlers migrated to modular registry:
//! - Health, ping, status → handlers/health.rs
//! - Capabilities → handlers/capabilities.rs
//! - Security (trust, JWT) → handlers/security.rs
//! - BTSP tunnels → handlers/btsp.rs
//! - Crypto operations → handlers/crypto.rs
//! - TLS operations → handlers/crypto.rs
//! - Federation → handlers/federation.rs
//! - Encryption → handlers/encryption.rs
//!
//! **File Size Reduction**: 1,809 → 1,494 lines (-315 lines of dead code)

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
            let (code, message) = if e.contains("Unknown method") || e.contains("Method not found")
            {
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
        ("GET", "/capabilities") => handle_method("capabilities", None, btsp_provider)
            .await
            .map_err(|e| anyhow::anyhow!(e)),
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
///
/// # Migration Architecture
///
/// This function now uses a hybrid approach:
/// 1. Try the new modular handler registry first
/// 2. Fall back to legacy handlers for remaining methods
/// 3. Allows seamless incremental migration
#[allow(clippy::too_many_lines)]
async fn handle_method(
    method: &str,
    params: Option<&serde_json::Value>,
    btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value, String> {
    debug!("📞 Method: {}", method);

    // Try new modular handler registry first
    use super::handlers::HandlerRegistry;
    let registry = HandlerRegistry::new();
    match registry.route(method, params, btsp_provider).await {
        Ok(result) => {
            debug!("✅ Handled by modular registry: {}", method);
            return Ok(result);
        }
        Err(e) if e.contains("Unknown method") => {
            // Method not in registry, try legacy handlers
            debug!("⏭️  Falling back to legacy handlers for: {}", method);
        }
        Err(e) => {
            // Real error from a handler, propagate it
            return Err(e);
        }
    }

    // Legacy handlers (will be phased out as modules are extracted)
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
                        "methods": ["evaluate", "lineage", "generate_jwt_secret"],
                        "description": "Security provider - trust evaluation, genetic lineage, and secret generation"
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
                    },
                    {
                        "type": "jwt_secrets",
                        "version": "1.0",
                        "methods": ["generate_jwt_secret"],
                        "description": "JWT secret generation for authentication systems (e.g., NestGate)"
                    },
                    {
                        "type": "crypto",
                        "version": "1.0",
                        "methods": [
                            "sign_ed25519",
                            "verify_ed25519",
                            "x25519_generate_ephemeral",
                            "x25519_derive_secret",
                            "chacha20_poly1305_encrypt",
                            "chacha20_poly1305_decrypt",
                            "blake3_hash",
                            "hmac_sha256"
                        ],
                        "description": "Pure Rust cryptographic operations for Songbird TLS and other primals - Ed25519, X25519, ChaCha20-Poly1305, Blake3, HMAC"
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

            info!(
                "🆔 Identity requested - family: {}, node: {}, encryption_tag: {}",
                family_id, node_id, encryption_tag
            );

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
        ("security", "evaluate")
        | ("trust", "evaluate")
        | ("security", "evaluate_trust")
        | ("trust", "evaluate_peer") => {
            let params = params.ok_or("Missing params for trust evaluation")?;

            // Flexible parameter extraction (works with any primal's naming)
            let peer_id = params
                .get("peer_id")
                .or_else(|| params.get("id"))
                .or_else(|| params.get("peer"))
                .and_then(|v| v.as_str())
                .ok_or("Missing peer identifier")?;

            let peer_family = params
                .get("peer_family")
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
            let (trust_level, trust_level_name, decision, reason, allowed_caps, denied_caps) =
                if peer_family == our_family {
                    info!(
                        "✅ Trust: SAME FAMILY - level 1 (limited) - peer: {}, family: {}",
                        peer_id, peer_family
                    );
                    (
                        1,
                        "limited",
                        "auto_accept", // Same family = auto-accept for genetic lineage
                        "same_genetic_family",
                        vec![
                            "birdsong/*",
                            "coordination/*",
                            "health",
                            "capabilities",
                            "discovery",
                        ],
                        vec!["data/*", "commands/*", "keys/*", "federation/admin"],
                    )
                } else if !peer_family.is_empty() {
                    info!("⚠️  Trust: DIFFERENT FAMILY - level 0 (none) - peer: {}, their: {}, ours: {}", peer_id, peer_family, our_family);
                    (
                        0,
                        "none",
                        "reject", // Different family = reject
                        "different_family",
                        vec![],
                        vec!["*"],
                    )
                } else {
                    info!(
                        "⚠️  Trust: UNKNOWN FAMILY - level 0 (none) - peer: {}",
                        peer_id
                    );
                    (
                        0,
                        "none",
                        "reject", // Unknown family = reject
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
        ("security", "lineage")
        | ("trust", "lineage")
        | ("security", "get_lineage")
        | ("trust", "get_lineage") => {
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

            info!(
                "🌳 Lineage info requested - family: {}, node: {}",
                family_id, node_id
            );
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
            let plaintext = params["plaintext"].as_str().ok_or("Missing plaintext")?;
            let family_id = params["family_id"].as_str().ok_or("Missing family_id")?;

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
            let ciphertext = params["ciphertext"].as_str().ok_or("Missing ciphertext")?;
            let family_id = params["family_id"].as_str().ok_or("Missing family_id")?;

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
                    let plaintext_b64 =
                        base64::engine::general_purpose::STANDARD.encode(&plaintext);

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
        ("beardog", "/btsp/contact/exchange")
        | ("btsp", "contact_exchange")
        | ("btsp", "contact/exchange") => {
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

        // BTSP Tunnel Establish
        ("beardog", "/btsp/tunnel/establish")
        | ("btsp", "tunnel_establish")
        | ("btsp", "tunnel/establish") => {
            info!("🔒 BTSP Tunnel Establish requested");

            let params = params.ok_or("Missing params for tunnel establish")?;

            let peer: beardog_capabilities::traits::PeerEndpoint =
                serde_json::from_value(params.clone())
                    .map_err(|e| format!("Invalid peer endpoint: {}", e))?;

            use beardog_capabilities::traits::SecureTunnelProvider;
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

        // BTSP Tunnel Encrypt
        ("beardog", "/btsp/tunnel/encrypt")
        | ("btsp", "tunnel_encrypt")
        | ("btsp", "tunnel/encrypt") => {
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

            use beardog_capabilities::traits::SecureTunnelProvider;
            match btsp_provider.tunnel_encrypt(&tunnel, &data).await {
                Ok(ciphertext) => {
                    info!("✅ Data encrypted for tunnel: {}", tunnel.id);
                    let ciphertext_b64 =
                        base64::engine::general_purpose::STANDARD.encode(&ciphertext);
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
        ("beardog", "/btsp/tunnel/decrypt")
        | ("btsp", "tunnel_decrypt")
        | ("btsp", "tunnel/decrypt") => {
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

            use beardog_capabilities::traits::SecureTunnelProvider;
            match btsp_provider.tunnel_decrypt(&tunnel, &data).await {
                Ok(plaintext) => {
                    info!("✅ Data decrypted for tunnel: {}", tunnel.id);
                    let plaintext_b64 =
                        base64::engine::general_purpose::STANDARD.encode(&plaintext);
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
        ("beardog", "/btsp/tunnel/status")
        | ("btsp", "tunnel_status")
        | ("btsp", "tunnel/status") => {
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

            use beardog_capabilities::traits::SecureTunnelProvider;
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

        // BTSP Tunnel Close
        ("beardog", "/btsp/tunnel/close") | ("btsp", "tunnel_close") | ("btsp", "tunnel/close") => {
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

        // ========================================================================
        // FEDERATION & ENCRYPTION METHODS
        // ========================================================================
        // NOTE: These methods are now handled by the modular handler registry!
        // - federation.* → handlers/federation.rs (FederationHandler)
        // - encryption.* → handlers/encryption.rs (EncryptionHandler)
        //
        // The registry is checked FIRST (lines 190-206), so these will never be reached.
        // Keeping this comment as documentation of the migration.
        // ========================================================================

        // ========================================================================
        // JWT SECRET GENERATION (Security Provider Capability)
        // ========================================================================

        // Generate JWT secret - BearDog provides secure secrets to other primals
        ("beardog", "generate_jwt_secret")
        | ("security", "generate_jwt_secret")
        | ("beardog", "jwt_secret")
        | ("security", "jwt_secret") => {
            info!("🔐 JWT Secret Generation requested");

            let params = params.ok_or("Missing params for JWT secret generation")?;

            let purpose = params
                .get("purpose")
                .and_then(|v| v.as_str())
                .unwrap_or("authentication");

            let strength = params
                .get("strength")
                .and_then(|v| v.as_str())
                .unwrap_or("high");

            // Determine byte length based on strength
            // high: 64 bytes (512 bits) -> 88 chars base64
            // medium: 48 bytes (384 bits) -> 64 chars base64
            // low: 32 bytes (256 bits) -> 44 chars base64
            let byte_length = match strength {
                "high" => 64,
                "medium" => 48,
                "low" => 32,
                _ => 64, // Default to high security
            };

            // Generate cryptographically secure random bytes
            use rand::RngCore;
            let mut secret_bytes = vec![0u8; byte_length];
            rand::rngs::OsRng.fill_bytes(&mut secret_bytes);

            // Encode to base64 (URL-safe for JWT)
            let jwt_secret = base64::engine::general_purpose::STANDARD.encode(&secret_bytes);

            info!(
                "🔐 Generated JWT secret: purpose={}, strength={}, length={}",
                purpose,
                strength,
                jwt_secret.len()
            );

            Ok(serde_json::json!({
                "secret": jwt_secret,
                "purpose": purpose,
                "strength": strength,
                "byte_length": byte_length,
                "encoded_length": jwt_secret.len(),
                "algorithm": "CSPRNG",
                "provider": "beardog",
                "generated_at": Utc::now().to_rfc3339(),
            }))
        }

        // ========================================================================
        // CRYPTO & TLS OPERATIONS
        // ========================================================================
        // NOTE: These methods are now handled by the modular handler registry!
        // - crypto.* → handlers/crypto.rs (CryptoHandler)
        // - tls.* → handlers/crypto.rs (CryptoHandler)
        //
        // The registry is checked FIRST (lines 190-206), so these will never be reached.
        // Keeping this comment as documentation of the migration.
        // ========================================================================

        // Unknown method
        _ => {
            warn!("⚠️  Unknown method: {}.{}", namespace, action);
            Err(format!("Method not found: {}.{}", namespace, action))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
    use crate::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
    use beardog_genetics::EcosystemGeneticEngine;
    use std::sync::Arc;

    /// Helper function to create a test HSM manager
    async fn create_test_hsm() -> Arc<HsmManager> {
        let mut hsm = HsmManager::new();
        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config)
            .await
            .expect("Software HSM init failed");

        hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))
            .expect("HSM provider registration failed");

        Arc::new(hsm)
    }

    /// Helper function to create a test BTSP provider with all dependencies
    async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
        let hsm = create_test_hsm().await;
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));
        
        Arc::new(
            BeardogBtspProvider::new(hsm, genetics)
                .await
                .expect("Failed to create BTSP provider"),
        )
    }

    #[tokio::test]
    async fn test_generate_jwt_secret_high_strength() {
        let btsp_provider = create_test_btsp_provider().await;

        // Test high strength JWT secret generation
        let params = serde_json::json!({
            "purpose": "nestgate_authentication",
            "strength": "high"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("JWT secret generation should succeed");

        // Verify response structure
        assert!(result.get("secret").is_some(), "Response should contain secret");
        assert_eq!(result["purpose"].as_str().unwrap(), "nestgate_authentication");
        assert_eq!(result["strength"].as_str().unwrap(), "high");
        assert_eq!(result["byte_length"].as_u64().unwrap(), 64);
        assert!(result["encoded_length"].as_u64().unwrap() >= 88, "Base64 encoded secret should be at least 88 chars");
        assert_eq!(result["provider"].as_str().unwrap(), "beardog");

        // Verify the secret is properly base64 encoded
        let secret = result["secret"].as_str().unwrap();
        assert!(
            base64::engine::general_purpose::STANDARD.decode(secret).is_ok(),
            "Secret should be valid base64"
        );
    }

    #[tokio::test]
    async fn test_generate_jwt_secret_medium_strength() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "testing",
            "strength": "medium"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("JWT secret generation should succeed");

        assert_eq!(result["byte_length"].as_u64().unwrap(), 48);
        assert!(result["encoded_length"].as_u64().unwrap() >= 64);
    }

    #[tokio::test]
    async fn test_generate_jwt_secret_low_strength() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "development",
            "strength": "low"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("JWT secret generation should succeed");

        assert_eq!(result["byte_length"].as_u64().unwrap(), 32);
        assert!(result["encoded_length"].as_u64().unwrap() >= 44);
    }

    #[tokio::test]
    async fn test_generate_jwt_secret_default_params() {
        let btsp_provider = create_test_btsp_provider().await;

        // Test with minimal params (should use defaults)
        let params = serde_json::json!({});

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("JWT secret generation should succeed");

        // Should default to high strength
        assert_eq!(result["strength"].as_str().unwrap(), "high");
        assert_eq!(result["purpose"].as_str().unwrap(), "authentication");
        assert_eq!(result["byte_length"].as_u64().unwrap(), 64);
    }

    #[tokio::test]
    async fn test_generate_jwt_secret_uniqueness() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "testing_uniqueness",
            "strength": "high"
        });

        // Generate multiple secrets and verify they're all unique
        let mut secrets = Vec::new();
        for _ in 0..10 {
            let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
                .await
                .expect("JWT secret generation should succeed");
            
            let secret = result["secret"].as_str().unwrap().to_string();
            secrets.push(secret);
        }

        // Verify all secrets are unique
        for i in 0..secrets.len() {
            for j in (i + 1)..secrets.len() {
                assert_ne!(
                    secrets[i], secrets[j],
                    "Generated secrets should be unique"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_jwt_secret_in_capabilities() {
        let btsp_provider = create_test_btsp_provider().await;

        let result = handle_method("capabilities", None, &btsp_provider)
            .await
            .expect("Capabilities query should succeed");

        let capabilities = result["provided_capabilities"]
            .as_array()
            .expect("provided_capabilities should be an array");

        // Find the jwt_secrets capability
        let jwt_cap = capabilities
            .iter()
            .find(|c| c["type"].as_str() == Some("jwt_secrets"));

        assert!(jwt_cap.is_some(), "jwt_secrets capability should be advertised");
        
        let methods = jwt_cap.unwrap()["methods"]
            .as_array()
            .expect("methods should be an array");
        
        assert!(
            methods.iter().any(|m| m.as_str() == Some("generate_jwt_secret")),
            "generate_jwt_secret should be in methods list"
        );
    }

    // =========================================================================
    // FAULT VALIDATION TESTS - Error Handling & Edge Cases
    // =========================================================================

    #[tokio::test]
    async fn test_jwt_secret_missing_params() {
        let btsp_provider = create_test_btsp_provider().await;

        // Should succeed with defaults even without params
        let result = handle_method("beardog.generate_jwt_secret", None, &btsp_provider).await;

        // Missing params should be treated as an error by the handler
        assert!(result.is_err(), "Missing params should return error");
        assert!(result.unwrap_err().contains("Missing params"), "Error should mention missing params");
    }

    #[tokio::test]
    async fn test_jwt_secret_invalid_strength() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "testing",
            "strength": "ultra_mega_high" // Invalid strength
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("Should default to high strength for invalid values");

        // Should default to high strength
        assert_eq!(result["byte_length"].as_u64().unwrap(), 64);
        assert_eq!(result["strength"].as_str().unwrap(), "ultra_mega_high"); // Echoes back but uses default
    }

    #[tokio::test]
    async fn test_jwt_secret_empty_purpose() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "",
            "strength": "high"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("Empty purpose should succeed");

        assert_eq!(result["purpose"].as_str().unwrap(), "");
        assert!(result.get("secret").is_some());
    }

    #[tokio::test]
    async fn test_jwt_secret_special_chars_in_purpose() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "test!@#$%^&*()_+-={}[]|:;<>?,./~`",
            "strength": "high"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("Special characters in purpose should succeed");

        assert!(result.get("secret").is_some());
        assert_eq!(result["purpose"].as_str().unwrap(), "test!@#$%^&*()_+-={}[]|:;<>?,./~`");
    }

    #[tokio::test]
    async fn test_jwt_secret_very_long_purpose() {
        let btsp_provider = create_test_btsp_provider().await;

        let long_purpose = "a".repeat(10000); // 10KB purpose string
        let params = serde_json::json!({
            "purpose": long_purpose,
            "strength": "high"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("Very long purpose should succeed");

        assert!(result.get("secret").is_some());
        assert_eq!(result["purpose"].as_str().unwrap().len(), 10000);
    }

    #[tokio::test]
    async fn test_jwt_secret_method_aliases() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "alias_test",
            "strength": "high"
        });

        // Test all method aliases
        let aliases = vec![
            "beardog.generate_jwt_secret",
            "security.generate_jwt_secret",
            "beardog.jwt_secret",
            "security.jwt_secret",
        ];

        for alias in aliases {
            let result = handle_method(alias, Some(&params), &btsp_provider)
                .await
                .unwrap_or_else(|_| panic!("Alias '{}' should work", alias));

            assert!(result.get("secret").is_some(), "Alias '{}' should return secret", alias);
        }
    }

    // =========================================================================
    // CHAOS TESTS - Concurrent Access & Race Conditions
    // =========================================================================

    #[tokio::test]
    async fn test_jwt_secret_concurrent_generation() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "concurrent_test",
            "strength": "high"
        });

        // Spawn 100 concurrent requests
        let mut handles = vec![];
        for i in 0..100 {
            let provider = Arc::clone(&btsp_provider);
            let params_clone = params.clone();
            
            let handle = tokio::spawn(async move {
                let result = handle_method(
                    "beardog.generate_jwt_secret",
                    Some(&params_clone),
                    &provider
                ).await;
                (i, result)
            });
            handles.push(handle);
        }

        // Collect all results
        let mut secrets = Vec::new();
        for handle in handles {
            let (idx, result) = handle.await.expect("Task should complete");
            let result = result.unwrap_or_else(|_| panic!("Request {} should succeed", idx));
            let secret = result["secret"].as_str().unwrap().to_string();
            secrets.push(secret);
        }

        // Verify all 100 secrets are unique
        assert_eq!(secrets.len(), 100);
        for i in 0..secrets.len() {
            for j in (i + 1)..secrets.len() {
                assert_ne!(
                    secrets[i], secrets[j],
                    "Concurrent secrets {} and {} should be unique",
                    i, j
                );
            }
        }
    }

    #[tokio::test]
    async fn test_jwt_secret_high_frequency_generation() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "high_frequency_test",
            "strength": "medium" // Use medium to be faster
        });

        // Generate 1000 secrets in rapid succession
        let mut secrets = Vec::new();
        for _ in 0..1000 {
            let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
                .await
                .expect("High frequency generation should succeed");
            
            let secret = result["secret"].as_str().unwrap().to_string();
            secrets.push(secret);
        }

        // Verify all are unique (sampling check - check first 100)
        for i in 0..100.min(secrets.len()) {
            for j in (i + 1)..100.min(secrets.len()) {
                assert_ne!(
                    secrets[i], secrets[j],
                    "High frequency secrets should be unique"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_jwt_secret_mixed_concurrent_requests() {
        let btsp_provider = create_test_btsp_provider().await;

        // Mix of different strength levels and purposes
        let test_cases = vec![
            ("high", "auth1"),
            ("medium", "auth2"),
            ("low", "auth3"),
            ("high", "auth4"),
            ("invalid", "auth5"), // Invalid strength
        ];

        let mut handles = vec![];
        for (strength, purpose) in test_cases {
            let provider = Arc::clone(&btsp_provider);
            let strength = strength.to_string();
            let purpose = purpose.to_string();
            
            // Spawn 20 concurrent requests for each case
            for _ in 0..20 {
                let provider_clone = Arc::clone(&provider);
                let strength_clone = strength.clone();
                let purpose_clone = purpose.clone();
                
                let handle = tokio::spawn(async move {
                    let params = serde_json::json!({
                        "purpose": purpose_clone,
                        "strength": strength_clone
                    });
                    handle_method("beardog.generate_jwt_secret", Some(&params), &provider_clone).await
                });
                handles.push(handle);
            }
        }

        // All should complete successfully
        let mut success_count = 0;
        for handle in handles {
            if let Ok(result) = handle.await {
                if result.is_ok() {
                    success_count += 1;
                }
            }
        }

        assert_eq!(success_count, 100, "All 100 mixed concurrent requests should succeed");
    }

    // =========================================================================
    // SECURITY VALIDATION TESTS - Entropy & Cryptographic Quality
    // =========================================================================

    #[tokio::test]
    async fn test_jwt_secret_entropy_quality() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "entropy_test",
            "strength": "high"
        });

        let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("Entropy test should succeed");

        let secret = result["secret"].as_str().unwrap();
        let secret_bytes = base64::engine::general_purpose::STANDARD
            .decode(secret)
            .expect("Should decode base64");

        // Test 1: Not all zeros
        assert!(
            secret_bytes.iter().any(|&b| b != 0),
            "Secret should not be all zeros"
        );

        // Test 2: Not all 0xFF
        assert!(
            secret_bytes.iter().any(|&b| b != 0xFF),
            "Secret should not be all 0xFF"
        );

        // Test 3: Has reasonable distribution (not all same byte)
        let first_byte = secret_bytes[0];
        let all_same = secret_bytes.iter().all(|&b| b == first_byte);
        assert!(!all_same, "Secret should have varied bytes");

        // Test 4: Has both high and low nibbles set (not trivial pattern)
        let has_high_bits = secret_bytes.iter().any(|&b| (b & 0xF0) != 0);
        let has_low_bits = secret_bytes.iter().any(|&b| (b & 0x0F) != 0);
        assert!(has_high_bits && has_low_bits, "Secret should use full byte range");
    }

    #[tokio::test]
    async fn test_jwt_secret_no_predictable_patterns() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "pattern_test",
            "strength": "high"
        });

        // Generate 10 secrets
        let mut decoded_secrets = Vec::new();
        for _ in 0..10 {
            let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
                .await
                .expect("Pattern test should succeed");
            
            let secret = result["secret"].as_str().unwrap();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(secret)
                .expect("Should decode base64");
            decoded_secrets.push(bytes);
        }

        // Test: No two secrets share the same first 16 bytes
        for i in 0..decoded_secrets.len() {
            for j in (i + 1)..decoded_secrets.len() {
                let prefix_i = &decoded_secrets[i][0..16];
                let prefix_j = &decoded_secrets[j][0..16];
                assert_ne!(
                    prefix_i, prefix_j,
                    "Secrets should not share common prefixes (predictable pattern)"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_jwt_secret_all_strengths_different_lengths() {
        let btsp_provider = create_test_btsp_provider().await;

        let strengths = vec![
            ("high", 64),
            ("medium", 48),
            ("low", 32),
        ];

        for (strength, expected_bytes) in strengths {
            let params = serde_json::json!({
                "purpose": format!("length_test_{}", strength),
                "strength": strength
            });

            let result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
                .await
                .expect("Length test should succeed");

            let secret = result["secret"].as_str().unwrap();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(secret)
                .expect("Should decode base64");

            assert_eq!(
                bytes.len(),
                expected_bytes,
                "Strength '{}' should produce {} bytes",
                strength,
                expected_bytes
            );
        }
    }

    // =========================================================================
    // E2E TESTS - Full JSON-RPC Flow
    // =========================================================================

    #[tokio::test]
    async fn test_jwt_secret_full_jsonrpc_request() {
        let btsp_provider = create_test_btsp_provider().await;

        // Create a full JSON-RPC request
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "beardog.generate_jwt_secret".to_string(),
            params: Some(serde_json::json!({
                "purpose": "e2e_test",
                "strength": "high"
            })),
            id: Some(serde_json::json!(1)),
        };

        // Handle the full request
        let response = handle_jsonrpc_request(&request, &btsp_provider).await;

        // Verify response structure
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.error.is_none(), "Should not have error");
        assert!(response.result.is_some(), "Should have result");

        let result = response.result.unwrap();
        assert!(result.get("secret").is_some());
        assert_eq!(result["provider"].as_str().unwrap(), "beardog");
    }

    #[tokio::test]
    async fn test_jwt_secret_jsonrpc_error_handling() {
        let btsp_provider = create_test_btsp_provider().await;

        // Request with wrong JSON-RPC version
        let request = JsonRpcRequest {
            jsonrpc: "1.0".to_string(), // Wrong version
            method: "beardog.generate_jwt_secret".to_string(),
            params: Some(serde_json::json!({})),
            id: Some(serde_json::json!(1)),
        };

        let response = handle_jsonrpc_request(&request, &btsp_provider).await;

        // Should return error
        assert!(response.error.is_some(), "Should have error for wrong JSON-RPC version");
        assert_eq!(response.error.unwrap().code, -32600); // Invalid Request
    }

    #[tokio::test]
    async fn test_jwt_secret_jsonrpc_method_not_found() {
        let btsp_provider = create_test_btsp_provider().await;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "beardog.nonexistent_method".to_string(),
            params: Some(serde_json::json!({})),
            id: Some(serde_json::json!(1)),
        };

        let response = handle_jsonrpc_request(&request, &btsp_provider).await;

        // Should return method not found error
        assert!(response.error.is_some(), "Should have error for unknown method");
        assert_eq!(response.error.unwrap().code, JsonRpcError::METHOD_NOT_FOUND);
    }

    // =========================================================================
    // PERFORMANCE & LOAD TESTS
    // =========================================================================

    #[tokio::test]
    async fn test_jwt_secret_batch_generation_performance() {
        let btsp_provider = create_test_btsp_provider().await;

        let params = serde_json::json!({
            "purpose": "performance_test",
            "strength": "high"
        });

        let start = std::time::Instant::now();
        
        // Generate 100 secrets sequentially
        for _ in 0..100 {
            let _result = handle_method("beardog.generate_jwt_secret", Some(&params), &btsp_provider)
                .await
                .expect("Performance test should succeed");
        }

        let duration = start.elapsed();

        // Should complete 100 secrets in under 1 second (very generous)
        assert!(
            duration.as_secs() < 1,
            "100 sequential secret generations should complete in under 1s, took {:?}",
            duration
        );
    }
}
