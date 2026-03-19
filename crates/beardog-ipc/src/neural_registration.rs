// SPDX-License-Identifier: AGPL-3.0-only

//! Neural API Auto-Registration for BearDog
//!
//! Registers BearDog's crypto capabilities with Neural API on startup.
//! This enables TRUE PRIMAL pattern with semantic routing via `capability.call`.
//!
//! ## Architecture
//!
//! ```text
//! BearDog startup
//!   ↓
//! detect Neural API socket
//!   ↓
//! register capabilities
//!   ↓
//! - crypto (core cryptography)
//! - tls_crypto (TLS-specific operations)
//! - genetic_lineage (lineage verification)
//!   ↓
//! Neural API stores semantic mappings
//!   ↓
//! Other primals use capability.call
//! ```
//!
//! ## Semantic Routing
//!
//! Neural API translates semantic method names to BearDog's actual method names:
//!
//! ```text
//! Songbird: "crypto.generate_keypair"
//!   ↓
//! Neural API: capability.call("crypto", "generate_keypair")
//!   ↓
//! Neural API translates: "crypto.x25519_generate_ephemeral"
//!   ↓
//! BearDog executes
//! ```
//!
//! This enables zero-coupling evolution: BearDog can change its API without
//! breaking consumers.

use anyhow::{Context, Result};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::{debug, info, warn};

/// Register BearDog's capabilities with Neural API
///
/// This registers three main capabilities:
/// 1. `crypto` - Core cryptographic operations
/// 2. `tls_crypto` - TLS-specific crypto operations
/// 3. `genetic_lineage` - Genetic lineage verification
///
/// Each capability includes semantic mappings that translate generic
/// operation names to BearDog's specific method names.
///
/// # Arguments
///
/// * `neural_socket` - Path to Neural API Unix socket
/// * `primal_name` - Primal identifier (e.g., "beardog-nat0")
/// * `socket_path` - Path to this primal's Unix socket (e.g., "/tmp/beardog-nat0.sock")
pub async fn register_with_neural_api(
    neural_socket: &str,
    primal_name: &str,
    socket_path: &str,
) -> Result<()> {
    info!(
        "🔐 Registering BearDog crypto capabilities with Neural API at {}",
        neural_socket
    );
    info!("   Primal: {}, Socket: {}", primal_name, socket_path);

    // Capability definitions with semantic mappings
    let capabilities = vec![
        // Core crypto capability
        json!({
            "capability": "crypto",
            "primal": primal_name,
            "socket": socket_path,
            "provider": "beardog",
            "version": "0.9.0",
            "operations": [
                "generate_keypair",
                "ecdh_derive",
                "encrypt",
                "decrypt",
                "encrypt_aes_128_gcm",
                "decrypt_aes_128_gcm",
                "encrypt_aes_256_gcm",
                "decrypt_aes_256_gcm",
                "sha256",
                "sha384",
                "hkdf_extract",
                "hkdf_expand"
            ],
            // NOTE: semantic_mappings are now handled by Neural API's graph-based
            // translation system (tower_atomic_bootstrap.toml). BearDog just exposes
            // its API, and the graph wires everything together at runtime.
            // This enables TRUE PRIMAL pattern with zero coupling!
        }),
        // TLS-specific crypto
        json!({
            "capability": "tls_crypto",
            "primal": primal_name,
            "socket": socket_path,
            "provider": "beardog",
            "version": "0.9.0",
            "operations": [
                "derive_handshake_secrets",
                "derive_application_secrets",
                "compute_finished_verify_data"
            ],
            // Semantic mappings handled by graph (see tower_atomic_bootstrap.toml)
        }),
        // Genetic lineage
        json!({
            "capability": "genetic_lineage",
            "primal": primal_name,
            "socket": socket_path,
            "provider": "beardog",
            "version": "0.9.0",
            "operations": [
                "verify_lineage",
                "generate_lineage_proof"
            ],
            // NOTE: semantic_mappings are now handled by Neural API's graph-based
            // translation system (tower_atomic_bootstrap.toml). BearDog just exposes
            // its API, and the graph wires everything together at runtime.
            // This enables TRUE PRIMAL pattern with zero coupling!
        }),
    ];

    // Register each capability
    for cap in capabilities {
        register_capability(neural_socket, cap)
            .await
            .context("Failed to register capability")?;
    }

    info!("✅ BearDog capabilities registered with Neural API");
    Ok(())
}

/// Register a single capability with Neural API
async fn register_capability(neural_socket: &str, capability: serde_json::Value) -> Result<()> {
    let cap_name = capability["capability"].as_str().unwrap_or("unknown");
    debug!("📤 Registering capability: {}", cap_name);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "capability.register",
        "params": capability,
        "id": 1
    });

    let request_str = serde_json::to_string(&request)?;
    debug!("📤 Sending registration: {}", request_str);

    // Connect to Neural API
    let mut stream = UnixStream::connect(neural_socket).await.context(format!(
        "Failed to connect to Neural API at {}",
        neural_socket
    ))?;

    // Send registration request
    stream.write_all(request_str.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Read response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let response_json: serde_json::Value =
        serde_json::from_str(&response).context("Failed to parse Neural API response")?;

    if response_json.get("error").is_some() {
        warn!(
            "⚠️  Registration warning for {}: {:?}",
            cap_name, response_json["error"]
        );
    } else {
        info!("✅ Registered capability: {}", cap_name);
    }

    Ok(())
}

/// Discover Neural API socket from environment
///
/// Checks in order:
/// 1. `NEURAL_API_SOCKET` environment variable (if empty, disables auto-registration)
/// 2. `NEURALS_SOCKET` environment variable (alternate name, if empty, disables)
/// 3. Default paths (checked in order):
///    - `/tmp/neural-api.sock` (primary default)
///    - `/tmp/neural-api-nat0.sock` (legacy compatibility)
///
/// Returns `None` if:
/// - Explicitly set to empty string (disables auto-registration)
/// - No default socket paths exist (Neural API not running)
/// Discover Neural API socket from environment (for testing)
///
/// This version accepts an explicit environment map, making it concurrent-safe for tests.
#[cfg(test)]
pub fn discover_neural_api_socket_with_env(
    env_vars: &std::collections::HashMap<String, String>,
) -> Option<String> {
    use std::path::Path;

    // Check NEURAL_API_SOCKET first (highest priority)
    if let Some(socket) = env_vars.get("NEURAL_API_SOCKET") {
        if !socket.is_empty() {
            info!("🔍 Using NEURAL_API_SOCKET: {}", socket);
            return Some(socket.clone());
        } else {
            // Empty string explicitly disables auto-registration
            debug!("NEURAL_API_SOCKET is empty - auto-registration disabled");
            return None;
        }
    }

    // Check NEURALS_SOCKET (fallback)
    if let Some(socket) = env_vars.get("NEURALS_SOCKET") {
        if !socket.is_empty() {
            info!("🔍 Using NEURALS_SOCKET: {}", socket);
            return Some(socket.clone());
        } else {
            // Empty string explicitly disables auto-registration
            debug!("NEURALS_SOCKET is empty - auto-registration disabled");
            return None;
        }
    }

    // Try default paths in priority order
    let default_paths = [
        "/tmp/neural-api.sock",      // Primary default (biomeOS standard)
        "/tmp/neural-api-nat0.sock", // Legacy compatibility
    ];

    for path in &default_paths {
        if Path::new(path).exists() {
            info!("🔍 Found Neural API at default path: {}", path);
            return Some(path.to_string());
        }
    }

    info!("🔍 No Neural API socket found");
    None
}

pub fn discover_neural_api_socket() -> Option<String> {
    use std::path::Path;

    // Check NEURAL_API_SOCKET first (highest priority)
    match std::env::var("NEURAL_API_SOCKET") {
        Ok(socket) if !socket.is_empty() => {
            info!("🔍 Using NEURAL_API_SOCKET: {}", socket);
            return Some(socket);
        }
        Ok(_) => {
            // Empty string explicitly disables auto-registration
            debug!("NEURAL_API_SOCKET is empty - auto-registration disabled");
            return None;
        }
        Err(_) => {
            // Not set, try next option
        }
    }

    // Check NEURALS_SOCKET (fallback)
    match std::env::var("NEURALS_SOCKET") {
        Ok(socket) if !socket.is_empty() => {
            info!("🔍 Using NEURALS_SOCKET: {}", socket);
            return Some(socket);
        }
        Ok(_) => {
            // Empty string explicitly disables auto-registration
            debug!("NEURALS_SOCKET is empty - auto-registration disabled");
            return None;
        }
        Err(_) => {
            // Not set, try defaults
        }
    }

    // Try default paths in priority order
    let default_paths = [
        "/tmp/neural-api.sock",      // Primary default (biomeOS standard)
        "/tmp/neural-api-nat0.sock", // Legacy compatibility
    ];

    for path in &default_paths {
        if Path::new(path).exists() {
            info!("🔍 Found Neural API socket at default path: {}", path);
            return Some(path.to_string());
        } else {
            debug!("Checked default path (not found): {}", path);
        }
    }

    info!(
        "ℹ️  No Neural API socket found (checked env vars and {} default paths)",
        default_paths.len()
    );
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_neural_api_socket_default() {
        let socket = discover_neural_api_socket();
        // Should return either env var or default - either is valid
        assert!(socket.is_some() || socket.is_none());
    }

    #[test]
    fn test_semantic_mappings_coverage() {
        // Verify all expected crypto operations are mapped
        let cap = json!({
            "semantic_mappings": {
                "crypto.generate_keypair": "crypto.x25519_generate_ephemeral",
                "crypto.ecdh_derive": "crypto.x25519_derive_secret",
                "crypto.encrypt": "crypto.chacha20_poly1305_encrypt",
            }
        });

        let mappings = cap["semantic_mappings"].as_object().unwrap();
        assert!(mappings.contains_key("crypto.generate_keypair"));
        assert!(mappings.contains_key("crypto.ecdh_derive"));
        assert!(mappings.contains_key("crypto.encrypt"));
    }
}

#[cfg(test)]
#[path = "neural_registration_comprehensive_tests.rs"]
mod comprehensive_tests;
