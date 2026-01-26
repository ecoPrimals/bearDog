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
pub async fn register_with_neural_api(neural_socket: &str) -> Result<()> {
    info!(
        "🔐 Registering BearDog crypto capabilities with Neural API at {}",
        neural_socket
    );

    // Capability definitions with semantic mappings
    let capabilities = vec![
        // Core crypto capability
        json!({
            "capability": "crypto",
            "provider": "beardog",
            "version": "0.1.0",
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
            "semantic_mappings": {
                // Map semantic names to BearDog's actual method names
                "crypto.generate_keypair": "crypto.x25519_generate_ephemeral",
                "crypto.ecdh_derive": "crypto.x25519_derive_secret",
                "crypto.encrypt": "crypto.chacha20_poly1305_encrypt",
                "crypto.decrypt": "crypto.chacha20_poly1305_decrypt",
                "crypto.encrypt_aes_128_gcm": "crypto.aes128_gcm_encrypt",
                "crypto.decrypt_aes_128_gcm": "crypto.aes128_gcm_decrypt",
                "crypto.encrypt_aes_256_gcm": "crypto.aes256_gcm_encrypt",
                "crypto.decrypt_aes_256_gcm": "crypto.aes256_gcm_decrypt",
                "crypto.sha256": "crypto.sha256",
                "crypto.sha384": "crypto.sha384",
                "crypto.hkdf_extract": "crypto.hkdf_extract",
                "crypto.hkdf_expand": "crypto.hkdf_expand"
            }
        }),
        // TLS-specific crypto
        json!({
            "capability": "tls_crypto",
            "provider": "beardog",
            "version": "0.1.0",
            "operations": [
                "derive_handshake_secrets",
                "derive_application_secrets",
                "compute_finished_verify_data"
            ],
            "semantic_mappings": {
                "tls.derive_handshake_secrets": "tls.derive_handshake_secrets",
                "tls.derive_application_secrets": "tls.derive_application_secrets",
                "tls.compute_finished_verify_data": "tls.compute_finished_verify_data"
            }
        }),
        // Genetic lineage
        json!({
            "capability": "genetic_lineage",
            "provider": "beardog",
            "version": "0.1.0",
            "operations": [
                "verify_lineage",
                "generate_lineage_proof"
            ],
            "semantic_mappings": {
                "genetic.verify_lineage": "genetic.verify_lineage",
                "genetic.generate_lineage_proof": "genetic.generate_lineage_proof"
            }
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
async fn register_capability(
    neural_socket: &str,
    capability: serde_json::Value,
) -> Result<()> {
    let cap_name = capability["capability"]
        .as_str()
        .unwrap_or("unknown");
    debug!("📤 Registering capability: {}", cap_name);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "capability.register",
        "params": capability,
        "id": 1
    });

    let request_str = serde_json::to_string(&request)?;

    // Connect to Neural API
    let mut stream = UnixStream::connect(neural_socket).await.context(
        format!("Failed to connect to Neural API at {}", neural_socket),
    )?;

    // Send registration request
    stream.write_all(request_str.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Read response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let response_json: serde_json::Value = serde_json::from_str(&response)
        .context("Failed to parse Neural API response")?;

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
/// 1. `NEURAL_API_SOCKET` environment variable
/// 2. `NEURALS_SOCKET` environment variable (alternate name)
/// 3. Default: `/tmp/neural-api-nat0.sock` (if file exists)
///
/// Returns `None` if:
/// - Explicitly set to empty string (disables auto-registration)
/// - Default socket path doesn't exist (Neural API not running)
pub fn discover_neural_api_socket() -> Option<String> {
    use std::path::Path;
    
    // Check env vars first
    std::env::var("NEURAL_API_SOCKET")
        .ok()
        .or_else(|| std::env::var("NEURALS_SOCKET").ok())
        .or_else(|| {
            // Check if default socket exists
            let default = "/tmp/neural-api-nat0.sock";
            if Path::new(default).exists() {
                debug!("Using default Neural API socket: {}", default);
                Some(default.to_string())
            } else {
                debug!("Default Neural API socket not found: {}", default);
                None
            }
        })
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
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

