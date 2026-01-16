//! BTSP Unix Socket Client Example
//!
//! Demonstrates how to connect to BearDog BTSP via Unix socket and use tunnel features.
//!
//! # Prerequisites
//!
//! 1. Start beardog-server in another terminal:
//!    ```bash
//!    cargo run --bin beardog-server
//!    ```
//!
//! 2. Run this example:
//!    ```bash
//!    cargo run --example btsp_unix_socket_client
//!    ```
//!
//! # What This Demonstrates
//!
//! - Unix socket connection to BearDog
//! - JSON-RPC 2.0 protocol usage
//! - BTSP capability discovery
//! - Tunnel establishment
//! - Data encryption/decryption
//!
//! # Modern Patterns
//!
//! - Async/await throughout
//! - JSON-RPC 2.0 standard protocol
//! - Environment-based socket discovery
//! - Error handling with `?` operator
//!
//! # Concentrated Gap Strategy
//!
//! This example shows how Songbird (or any primal) should communicate with BearDog:
//! - Unix sockets for inter-primal communication (fast, secure)
//! - NO HTTP for local communication (HTTP deprecated for BTSP)
//! - Songbird remains the single HTTP gateway for external access

use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         🐻 BTSP Unix Socket Client Example 🐻                     ║");
    info!("║                                                                    ║");
    info!("║  Demonstrates TRUE PRIMAL inter-primal communication:             ║");
    info!("║  • Unix socket (fast, secure)                                      ║");
    info!("║  • JSON-RPC 2.0 (standard protocol)                                ║");
    info!("║  • NO HTTP (deprecated for inter-primal)                           ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    // Step 1: Discover socket path (4-tier fallback)
    let socket_path = discover_beardog_socket()?;
    info!("📡 Step 1: Discovered BearDog socket");
    info!("   Socket: {}", socket_path.display());
    info!("");

    // Step 2: Connect to BearDog
    info!("🔌 Step 2: Connecting to BearDog...");
    let mut stream = UnixStream::connect(&socket_path).await?;
    info!("✅ Connected!");
    info!("");

    // Step 3: Health check
    info!("🏥 Step 3: Health check...");
    let response = send_jsonrpc_request(
        &mut stream,
        json!({
            "jsonrpc": "2.0",
            "method": "ping",
            "id": 1
        }),
    )
    .await?;

    if let Some(result) = response.get("result") {
        info!("   Status: {}", result.get("status").and_then(|s| s.as_str()).unwrap_or("unknown"));
        info!("   Version: {}", result.get("version").and_then(|v| v.as_str()).unwrap_or("unknown"));
        info!("✅ BearDog is healthy!");
    } else {
        info!("⚠️  Unexpected response: {:?}", response);
    }
    info!("");

    // Step 4: Discover capabilities
    info!("🔍 Step 4: Discovering BTSP capabilities...");
    let response = send_jsonrpc_request(
        &mut stream,
        json!({
            "jsonrpc": "2.0",
            "method": "capabilities",
            "id": 2
        }),
    )
    .await?;

    if let Some(capabilities) = response.get("result").and_then(|r| r.get("provided_capabilities")) {
        if let Some(caps_array) = capabilities.as_array() {
            for cap in caps_array {
                if cap.get("type").and_then(|t| t.as_str()) == Some("btsp") {
                    info!("✅ Found BTSP capability!");
                    info!("   Version: {}", cap.get("version").and_then(|v| v.as_str()).unwrap_or("unknown"));
                    if let Some(methods) = cap.get("methods").and_then(|m| m.as_array()) {
                        info!("   Methods:");
                        for method in methods {
                            if let Some(method_name) = method.as_str() {
                                info!("     • {}", method_name);
                            }
                        }
                    }
                }
            }
        }
    }
    info!("");

    // Step 5: Demonstrate tunnel establishment (mock peer)
    info!("🔒 Step 5: Demonstrating tunnel establishment...");
    info!("   (Using mock peer for demonstration)");

    let response = send_jsonrpc_request(
        &mut stream,
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_establish",
            "params": {
                "peer": {
                    "id": "demo-songbird-node",
                    "endpoint": "10.0.1.100:9000",
                    "public_key": "demo-public-key-base64",
                    "capabilities": ["federation", "coordination"]
                }
            },
            "id": 3
        }),
    )
    .await?;

    let tunnel_handle = if let Some(result) = response.get("result") {
        info!("✅ Tunnel established!");
        if let Some(tunnel_id) = result.get("id").and_then(|id| id.as_str()) {
            info!("   Tunnel ID: {}", tunnel_id);
        }
        if let Some(peer_id) = result.get("peer_id").and_then(|id| id.as_str()) {
            info!("   Peer ID: {}", peer_id);
        }
        result.clone()
    } else if let Some(error) = response.get("error") {
        info!("⚠️  Tunnel establishment failed (expected for mock peer):");
        info!("   Error: {}", error.get("message").and_then(|m| m.as_str()).unwrap_or("unknown"));
        info!("   This is normal without a real peer running!");
        return Ok(());
    } else {
        info!("⚠️  Unexpected response: {:?}", response);
        return Ok(());
    };
    info!("");

    // Step 6: Demonstrate encryption (if tunnel succeeded)
    info!("🔐 Step 6: Demonstrating data encryption...");
    let plaintext = "Hello from BTSP Unix socket client!";
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(plaintext);

    let response = send_jsonrpc_request(
        &mut stream,
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_encrypt",
            "params": {
                "tunnel": tunnel_handle,
                "direction": "outbound",
                "data": plaintext_b64
            },
            "id": 4
        }),
    )
    .await?;

    if let Some(result) = response.get("result") {
        info!("✅ Data encrypted!");
        if let Some(ciphertext_b64) = result.get("ciphertext").and_then(|c| c.as_str()) {
            info!("   Ciphertext (base64): {}...", &ciphertext_b64[..20.min(ciphertext_b64.len())]);
        }
    } else if let Some(error) = response.get("error") {
        info!("⚠️  Encryption failed:");
        info!("   Error: {}", error.get("message").and_then(|m| m.as_str()).unwrap_or("unknown"));
    }
    info!("");

    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         ✅ BTSP Unix Socket Demo Complete! ✅                     ║");
    info!("║                                                                    ║");
    info!("║  Key Takeaways:                                                    ║");
    info!("║  • Unix sockets are FAST (no HTTP overhead)                        ║");
    info!("║  • JSON-RPC 2.0 is standard and well-supported                     ║");
    info!("║  • BearDog's BTSP is fully functional via Unix sockets             ║");
    info!("║  • Concentrated Gap: Songbird = single HTTP gateway                ║");
    info!("║                                                                    ║");
    info!("║  Next Steps:                                                       ║");
    info!("║  1. Songbird: Migrate from HTTP client to Unix socket client      ║");
    info!("║  2. Use this code as reference implementation                      ║");
    info!("║  3. See SONGBIRD_BTSP_HANDOFF_JAN_16_2026.md for details           ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");

    Ok(())
}

/// Discover BearDog socket path with 4-tier fallback
fn discover_beardog_socket() -> Result<PathBuf> {
    // Tier 1: BEARDOG_SOCKET (highest priority)
    if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
        return Ok(PathBuf::from(socket_path));
    }

    // Tier 2: BIOMEOS_SOCKET_PATH (orchestrator)
    if let Ok(socket_path) = std::env::var("BIOMEOS_SOCKET_PATH") {
        return Ok(PathBuf::from(socket_path));
    }

    // Tier 3: XDG Runtime Directory
    if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let family_id = std::env::var("BEARDOG_FAMILY_ID")
            .or_else(|_| std::env::var("FAMILY_ID"))
            .unwrap_or_else(|_| "default".to_string());

        return Ok(PathBuf::from(format!("{}/beardog-{}.sock", xdg_runtime, family_id)));
    }

    // Tier 4: /tmp fallback
    let family_id = std::env::var("BEARDOG_FAMILY_ID")
        .or_else(|_| std::env::var("FAMILY_ID"))
        .unwrap_or_else(|_| "default".to_string());

    let node_id = std::env::var("BEARDOG_NODE_ID")
        .or_else(|_| std::env::var("NODE_ID"))
        .unwrap_or_else(|_| "default".to_string());

    Ok(PathBuf::from(format!("/tmp/beardog-{}-{}.sock", family_id, node_id)))
}

/// Send JSON-RPC request and receive response
async fn send_jsonrpc_request(
    stream: &mut UnixStream,
    request: serde_json::Value,
) -> Result<serde_json::Value> {
    // Send request
    let request_bytes = serde_json::to_vec(&request)?;
    stream.write_all(&request_bytes).await?;
    stream.write_all(b"\n").await?; // JSON-RPC delimiter

    // Read response
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(stream);
    reader.read_until(b'\n', &mut buffer).await?;

    let response: serde_json::Value = serde_json::from_slice(&buffer)?;
    Ok(response)
}

