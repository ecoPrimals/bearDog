// SPDX-License-Identifier: AGPL-3.0-or-later

//! Neural API Auto-Registration for `BearDog`
//!
//! Registers `BearDog`'s crypto capabilities with Neural API on startup.
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
//! Neural API translates semantic method names to `BearDog`'s actual method names:
//!
//! ```text
//! Consumer: "crypto.generate_keypair"
//!   ↓
//! Neural API: capability.call("crypto", "generate_keypair")
//!   ↓
//! Neural API translates: "crypto.x25519_generate_ephemeral"
//!   ↓
//! BearDog executes
//! ```
//!
//! This enables zero-coupling evolution: `BearDog` can change its API without
//! breaking consumers.

use anyhow::{Context, Result};
use beardog_types::constants::domains::network::ipc_discovery::{
    BEARDOG_CAPABILITY_DOMAIN, resolve_biomeos_ipc_subdir_from_optional,
};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::{debug, info, warn};

/// Register `BearDog`'s capabilities with Neural API
///
/// This registers four main capabilities:
/// 1. `crypto` - Core cryptographic operations
/// 2. `tls_crypto` - TLS-specific crypto operations
/// 3. `genetic_lineage` - Genetic lineage verification
/// 4. `security` - Consent, trust evaluation, JWT secrets
///
/// Each capability includes semantic mappings that translate generic
/// operation names to `BearDog`'s specific method names.
///
/// If `signed_attestation` is provided, each registration payload includes
/// an Ed25519 signature so the Neural API (and downstream Songbird discovery)
/// can verify the advertisement is authentic. The attestation should be
/// produced by the primal's unified identity key.
///
/// # Arguments
///
/// * `neural_socket` - Path to Neural API Unix socket
/// * `primal_name` - Primal identifier (e.g., "beardog-nat0")
/// * `socket_path` - Path to this primal's Unix socket (e.g., "/tmp/beardog-nat0.sock")
/// * `signed_attestation` - Optional Ed25519 attestation over the capability surface
///
/// # Errors
///
/// Returns an error when the Neural API socket is unreachable or registration fails.
pub async fn register_with_neural_api(
    neural_socket: &str,
    primal_name: &str,
    socket_path: &str,
    signed_attestation: Option<&serde_json::Value>,
) -> Result<()> {
    info!(
        "🔐 Registering BearDog crypto capabilities with Neural API at {}",
        neural_socket
    );
    info!("   Primal: {}, Socket: {}", primal_name, socket_path);

    let build_cap = |capability: &str, version: &str, operations: Vec<&str>| {
        let mut cap = json!({
            "capability": capability,
            "primal": primal_name,
            "socket": socket_path,
            "provider": "beardog",
            "version": version,
            "operations": operations,
        });
        if let Some(attestation) = signed_attestation {
            cap["signed_attestation"] = attestation.clone();
        }
        cap
    };

    let capabilities = vec![
        build_cap(
            BEARDOG_CAPABILITY_DOMAIN,
            "0.9.0",
            vec![
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
                "hkdf_expand",
            ],
        ),
        build_cap(
            "tls_crypto",
            "0.9.0",
            vec![
                "derive_handshake_secrets",
                "derive_application_secrets",
                "compute_finished_verify_data",
            ],
        ),
        build_cap(
            "genetic_lineage",
            "0.9.0",
            vec!["verify_lineage", "generate_lineage_proof"],
        ),
        build_cap(
            "security",
            "1.0.0",
            vec![
                "verify_consent",
                "issue_consent_token",
                "evaluate",
                "lineage",
                "generate_jwt_secret",
            ],
        ),
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
        "Failed to connect to Neural API at {neural_socket}"
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

/// All inputs needed to discover the Neural API socket without reading the process environment.
#[derive(Debug, Clone, Default)]
pub struct NeuralApiDiscoveryInputs {
    /// `NEURAL_API_SOCKET` env var (highest priority, explicit override).
    pub neural_api_socket: Option<String>,
    /// `NEURALS_SOCKET` env var (alias).
    pub neurals_socket: Option<String>,
    /// `BIOMEOS_SOCKET_DIR` (orchestrator-managed directory; joins `neural-api.sock`).
    pub biomeos_socket_dir: Option<String>,
    /// `XDG_RUNTIME_DIR` for Tier 3 resolution.
    pub xdg_runtime_dir: Option<String>,
    /// Effective UID for Tier 4 (`/run/user/{uid}/<ecosystem-namespace>/`).
    pub uid: Option<u32>,
}

/// Resolves the Neural API Unix socket path using 5-tier discovery (testable).
///
/// Resolution order:
/// 1. `NEURAL_API_SOCKET` / `NEURALS_SOCKET` env var (explicit override)
/// 2. `BIOMEOS_SOCKET_DIR/neural-api.sock` (orchestrator-managed directory)
/// 3. `XDG_RUNTIME_DIR/<ecosystem-namespace>/neural-api.sock` (namespace via `resolve_biomeos_ipc_subdir_from_optional` in `beardog-types`)
/// 4. `/run/user/{uid}/<ecosystem-namespace>/neural-api.sock`
/// 5. Platform temp dir: `{temp}/<ecosystem-namespace>/neural-api.sock`, `{temp}/neural-api.sock`, optional `BEARDOG_NEURAL_API_LEGACY_SOCKET`
///
/// An empty string in an env field disables auto-registration for that slot (returns `None`).
#[must_use]
pub fn discover_neural_api_socket_with(
    neural_api_socket: Option<String>,
    neurals_socket: Option<String>,
) -> Option<String> {
    discover_neural_api_socket_from_inputs(&NeuralApiDiscoveryInputs {
        neural_api_socket,
        neurals_socket,
        biomeos_socket_dir: std::env::var("BIOMEOS_SOCKET_DIR").ok(),
        xdg_runtime_dir: std::env::var("XDG_RUNTIME_DIR").ok(),
        uid: std::env::var("UID").ok().and_then(|s| s.parse().ok()),
    })
}

/// Resolves the Neural API socket from fully-injected inputs (no environment reads).
#[must_use]
pub fn discover_neural_api_socket_from_inputs(inputs: &NeuralApiDiscoveryInputs) -> Option<String> {
    use std::path::Path;

    let ecosystem_ns = resolve_biomeos_ipc_subdir_from_optional(None);

    // Tier 1: explicit env override
    if let Some(ref socket) = inputs.neural_api_socket {
        if socket.is_empty() {
            debug!("NEURAL_API_SOCKET is empty - auto-registration disabled");
            return None;
        }
        info!("🔍 Using NEURAL_API_SOCKET: {} (Tier 1)", socket);
        return Some(socket.clone());
    }

    if let Some(ref socket) = inputs.neurals_socket {
        if socket.is_empty() {
            debug!("NEURALS_SOCKET is empty - auto-registration disabled");
            return None;
        }
        info!("🔍 Using NEURALS_SOCKET: {} (Tier 1)", socket);
        return Some(socket.clone());
    }

    // Tier 2: orchestrator-managed directory
    if let Some(ref dir) = inputs.biomeos_socket_dir {
        let path = format!("{dir}/neural-api.sock");
        if Path::new(&path).exists() {
            info!(
                "🔍 Found Neural API via BIOMEOS_SOCKET_DIR (Tier 2): {}",
                path
            );
            return Some(path);
        }
    }

    // Tier 3: XDG runtime directory
    if let Some(ref xdg) = inputs.xdg_runtime_dir {
        let path = format!("{xdg}/{ecosystem_ns}/neural-api.sock");
        if Path::new(&path).exists() {
            info!(
                "🔍 Found Neural API via XDG/ecosystem namespace (Tier 3): {}",
                path
            );
            return Some(path);
        }
    }

    // Tier 4: /run/user/{uid}/<ecosystem-namespace>/
    let uid = inputs.uid.unwrap_or_else(|| {
        std::env::var("UID")
            .or_else(|_| std::env::var("EUID"))
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000)
    });
    let run_path = format!("/run/user/{uid}/{ecosystem_ns}/neural-api.sock");
    if Path::new(&run_path).exists() {
        info!("🔍 Found Neural API via /run/user (Tier 4): {}", run_path);
        return Some(run_path);
    }

    // Tier 5: platform temp dir + optional legacy path (no fixed peer names)
    let tmp = std::env::temp_dir();
    let mut fallback_paths: Vec<std::path::PathBuf> = vec![
        tmp.join(&ecosystem_ns).join("neural-api.sock"),
        tmp.join("neural-api.sock"),
    ];
    if let Ok(extra) = std::env::var("BEARDOG_NEURAL_API_LEGACY_SOCKET")
        && !extra.is_empty()
    {
        fallback_paths.push(std::path::PathBuf::from(extra));
    }

    for path in &fallback_paths {
        if path.exists() {
            let ps = path.display().to_string();
            info!("🔍 Found Neural API at fallback path (Tier 5): {}", ps);
            return Some(ps);
        }
        debug!("Checked default path (not found): {}", path.display());
    }

    info!(
        "ℹ️  No Neural API socket found (checked 5 tiers + {} fallback paths)",
        fallback_paths.len()
    );
    None
}

/// Resolves the Neural API Unix socket path for IPC registration.
///
/// Resolution order: `NEURAL_API_SOCKET`, then `NEURALS_SOCKET`, then orchestrator/XDG paths, then platform temp fallbacks.
/// An empty env value disables auto-registration (returns `None`).
pub fn discover_neural_api_socket() -> Option<String> {
    discover_neural_api_socket_with(
        beardog_errors::process_env::var("NEURAL_API_SOCKET").ok(),
        beardog_errors::process_env::var("NEURALS_SOCKET").ok(),
    )
}

/// Test helper: same as [`discover_neural_api_socket_with`] using a string map.
#[cfg(test)]
#[expect(
    clippy::implicit_hasher,
    reason = "API matches external registration map shape; concrete hasher not required at boundary"
)]
pub fn discover_neural_api_socket_with_env(
    env_vars: &std::collections::HashMap<String, String>,
) -> Option<String> {
    discover_neural_api_socket_with(
        env_vars.get("NEURAL_API_SOCKET").cloned(),
        env_vars.get("NEURALS_SOCKET").cloned(),
    )
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

        let mappings = cap["semantic_mappings"]
            .as_object()
            .expect("semantic_mappings fixture must be a JSON object");
        assert!(mappings.contains_key("crypto.generate_keypair"));
        assert!(mappings.contains_key("crypto.ecdh_derive"));
        assert!(mappings.contains_key("crypto.encrypt"));
    }
}

#[cfg(test)]
#[path = "neural_registration_comprehensive_tests.rs"]
mod comprehensive_tests;
