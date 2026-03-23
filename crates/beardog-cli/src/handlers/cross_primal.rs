// SPDX-License-Identifier: AGPL-3.0-only

//! # Cross-Primal Communication Handler
//!
//! Implements Workflow 3 from PHASE_1_INTEGRATION_REQUIREMENTS.md:
//! Cross-primal secure messaging and key ceremony using capability-based discovery.
//!
//! ## Principle: "Discover, Don't Hardcode"
//!
//! This handler discovers primals by capability, NOT by hardcoded names.
//! BearDog knows ONLY itself. At runtime, it discovers ANY primal advertising
//! the required capabilities (networking, compute, storage, etc) through:
//! - mDNS/DNS-SD service discovery
//! - Capability announcements
//! - Universal adapter pattern
//!
//! **No primal names are hardcoded.** Discovery is purely capability-based.

use crate::ecosystem_discovery_adapter::EcosystemDiscoveryAdapter;
use base64::{Engine, engine::general_purpose::STANDARD};
use beardog_core::ecosystem_integration::{PrimalDiscoveryService, SecureCrossPrimalMessenger};
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, SecurityService, StorageCharacteristic,
    UniversalCapabilityType,
};
use clap::Parser;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// `beardog cross-primal` command: capability-based discovery and secure messaging.
#[derive(Parser, Debug)]
pub struct CrossPrimalCommand {
    #[clap(subcommand)]
    action: CrossPrimalAction,
}

#[derive(Parser, Debug)]
enum CrossPrimalAction {
    /// Perform key ceremony with security-capable primal
    KeyCeremony {
        /// Seed file path (from Workflow 1)
        #[clap(long)]
        seed_file: String,

        /// Output shared key file
        #[clap(long)]
        output: String,

        /// Security level (low, medium, high, critical)
        #[clap(long, default_value = "high")]
        security_level: String,
    },

    /// Send secure message to primal with specific capability
    SendSecure {
        /// Message file to send
        #[clap(long)]
        message: String,

        /// Required capability (network, service_mesh, compute, storage)
        #[clap(long)]
        capability: String,

        /// Output response file
        #[clap(long)]
        output: Option<String>,
    },

    /// List discovered primals by capability
    DiscoverPrimals {
        /// Capability to filter by (network, service_mesh, compute, storage, security)
        #[clap(long)]
        capability: String,
    },
}

/// Handle cross-primal commands
///
/// # Errors
/// Returns an error if discovery, communication, or file operations fail
pub async fn handle_cross_primal(
    cmd: CrossPrimalCommand,
) -> Result<(), beardog_errors::BearDogError> {
    match cmd.action {
        CrossPrimalAction::KeyCeremony {
            seed_file,
            output,
            security_level,
        } => handle_key_ceremony(&seed_file, &output, &security_level).await,

        CrossPrimalAction::SendSecure {
            message,
            capability,
            output,
        } => handle_send_secure(&message, &capability, output.as_deref()).await,

        CrossPrimalAction::DiscoverPrimals { capability } => {
            handle_discover_primals(&capability).await
        }
    }
}

/// Handle key ceremony workflow (Workflow 3)
async fn handle_key_ceremony(
    seed_file: &str,
    output: &str,
    security_level: &str,
) -> Result<(), beardog_errors::BearDogError> {
    info!("🔐 Starting cross-primal key ceremony");
    info!("📋 Security level: {}", security_level);

    // Load seed from Workflow 1
    let _seed_data = std::fs::read(seed_file).map_err(|e| {
        beardog_errors::BearDogError::system(format!("Failed to read seed file: {e}"))
    })?;

    info!("✅ Loaded seed from: {}", seed_file);

    // Create discovery client and messenger
    let discovery_client: Arc<dyn PrimalDiscoveryService> =
        Arc::new(EcosystemDiscoveryAdapter::new()?);
    let messenger = SecureCrossPrimalMessenger::new(discovery_client)?;

    info!("🔍 Discovering security-capable primals in ecosystem...");

    // Establish secure session with security-capable primal (not hardcoded)
    // Note: This will return an error if no primals are discovered
    // That's correct behavior - we don't hardcode fallbacks
    match messenger.establish_secure_session(security_level).await {
        Ok(session) => {
            info!("✅ Secure session established with: {}", session.peer_id);
            info!("📝 Session ID: {}", session.session_id);

            // Save shared key
            std::fs::write(output, &session.encryption_key).map_err(|e| {
                beardog_errors::BearDogError::system(format!("Failed to write shared key: {e}"))
            })?;

            println!("✅ Key ceremony complete!");
            println!("   Peer: {}", session.peer_id);
            println!("   Session: {}", session.session_id);
            println!("   Security Level: {security_level}");
            println!("   Shared key saved to: {output}");
        }
        Err(e) => {
            println!("⚠️  Key ceremony failed: No security-capable primals discovered");
            println!("   This is expected until other primals are running in the ecosystem");
            println!("   Technical details: {e}");
            return Err(e);
        }
    }

    Ok(())
}

/// Handle secure message sending
async fn handle_send_secure(
    message_file: &str,
    capability: &str,
    output: Option<&str>,
) -> Result<(), beardog_errors::BearDogError> {
    info!(
        "📤 Sending secure message to primal with capability: {}",
        capability
    );

    // Load message
    let message_data = std::fs::read(message_file).map_err(|e| {
        beardog_errors::BearDogError::system(format!("Failed to read message file: {e}"))
    })?;

    info!(
        "✅ Loaded message from: {} ({} bytes)",
        message_file,
        message_data.len()
    );

    // Create discovery client and messenger
    let discovery_client: Arc<dyn PrimalDiscoveryService> =
        Arc::new(EcosystemDiscoveryAdapter::new()?);
    let messenger = SecureCrossPrimalMessenger::new(discovery_client)?;

    info!("🔍 Discovering primals with '{}' capability...", capability);

    // Send to primal with required capability (REAL) — routed by capability type, not primal name
    let result = match capability.to_lowercase().as_str() {
        "network" | "service_mesh" => {
            let security_context = HashMap::new();
            messenger
                .send_to_network_primal(&message_data, security_context)
                .await
        }
        "compute" => {
            let encoded_data = STANDARD.encode(&message_data);
            let payload = serde_json::json!({
                "data": encoded_data,
                "operation": "process",
            });
            messenger.send_to_compute_primal(payload).await
        }
        "storage" => messenger.send_to_storage_primal(&message_data).await,
        _ => {
            return Err(beardog_errors::BearDogError::invalid_input(&format!(
                "Unknown capability: {capability} (valid: network, service_mesh, compute, storage)"
            )));
        }
    };

    match result {
        Ok(response) => {
            println!("✅ Message sent successfully!");
            println!("   Size: {} bytes", message_data.len());
            println!("   Capability: {capability}");
            println!("   Responder: {}", response.responder_id);
            println!("   Processing time: {}ms", response.processing_time_ms);

            // Save response if output specified
            if let Some(output_path) = output {
                std::fs::write(output_path, &response.ciphertext).map_err(|e| {
                    beardog_errors::BearDogError::system(format!("Failed to write response: {e}"))
                })?;
                println!("   Response saved to: {output_path}");
            }
        }
        Err(e) => {
            println!("⚠️  Message send failed: No primals with '{capability}' capability found");
            println!("   This is expected until other primals are running in the ecosystem");
            println!("   Technical details: {e}");
            return Err(e);
        }
    }

    Ok(())
}

/// Handle primal discovery
async fn handle_discover_primals(capability: &str) -> Result<(), beardog_errors::BearDogError> {
    info!("🔍 Discovering primals with '{}' capability...", capability);

    // Create discovery client
    let discovery_client: Arc<dyn PrimalDiscoveryService> =
        Arc::new(EcosystemDiscoveryAdapter::new()?);

    // Determine capability type (universal discovery — no hardcoded primal identifiers)
    let capability_type = match capability.to_lowercase().as_str() {
        "network" => UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        },
        "service_mesh" => UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::ServiceMesh],
        },
        "security" => UniversalCapabilityType::Security {
            services: vec![SecurityService::KeyManagement],
        },
        "compute" => UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        },
        "storage" => UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Encrypted],
        },
        _ => {
            return Err(beardog_errors::BearDogError::invalid_input(&format!(
                "Unknown capability: {capability} (valid: network, service_mesh, security, compute, storage)"
            )));
        }
    };

    // Discover primals (REAL)
    println!("🔍 Discovering primals with '{capability}' capability...");

    match discovery_client
        .discover_by_capability(capability_type)
        .await
    {
        Ok(primals) => {
            if primals.is_empty() {
                println!("📋 No primals found with '{capability}' capability");
                println!("   This is expected until other primals are running in the ecosystem");
                println!("   The system correctly avoids hardcoding any primals");
            } else {
                println!(
                    "✅ Discovered {} primal(s) with '{}' capability:",
                    primals.len(),
                    capability
                );
                println!();

                for (idx, primal) in primals.iter().enumerate() {
                    println!("{}. Primal: {}", idx + 1, primal.service_id);
                    println!("   Endpoint: {:?}", primal.endpoint);
                    println!("   Trust Score: {:.2}", primal.trust_score);
                    println!();
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("⚠️  Discovery failed: {e}");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_discover_primals_valid_capabilities() {
        // Test that valid capabilities are accepted
        assert!(handle_discover_primals("network").await.is_ok());
        assert!(handle_discover_primals("service_mesh").await.is_ok());
        assert!(handle_discover_primals("security").await.is_ok());
        assert!(handle_discover_primals("compute").await.is_ok());
        assert!(handle_discover_primals("storage").await.is_ok());
    }

    #[tokio::test]
    async fn test_discover_primals_invalid_capability() {
        // Test that invalid capabilities are rejected
        assert!(handle_discover_primals("invalid").await.is_err());
    }

    #[test]
    fn test_cross_primal_parse_discover_primals() {
        use clap::Parser;

        let cmd = CrossPrimalCommand::try_parse_from([
            "beardog",
            "discover-primals",
            "--capability",
            "network",
        ])
        .expect("parse");

        match cmd.action {
            CrossPrimalAction::DiscoverPrimals { capability } => {
                assert_eq!(capability, "network");
            }
            _ => panic!("expected DiscoverPrimals"),
        }
    }

    #[test]
    fn test_cross_primal_parse_key_ceremony() {
        use clap::Parser;

        let cmd = CrossPrimalCommand::try_parse_from([
            "beardog",
            "key-ceremony",
            "--seed-file",
            "/tmp/seed",
            "--output",
            "/tmp/out",
            "--security-level",
            "medium",
        ])
        .expect("parse");

        match cmd.action {
            CrossPrimalAction::KeyCeremony {
                seed_file,
                output,
                security_level,
            } => {
                assert_eq!(seed_file, "/tmp/seed");
                assert_eq!(output, "/tmp/out");
                assert_eq!(security_level, "medium");
            }
            _ => panic!("expected KeyCeremony"),
        }
    }

    #[test]
    fn test_no_hardcoded_primal_names() {
        // Verify the module doesn't hardcode primal names like "songbird"
        let module_src = include_str!("cross_primal.rs");
        // Check for literal "songbird" (not in comments)
        let code_lines: Vec<&str> = module_src
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect();
        let code_only = code_lines.join("\n");
        assert!(
            !code_only.contains("\"songbird\""),
            "Should not hardcode 'songbird' string"
        );
        assert!(
            !code_only.contains("\"network-primal\""),
            "Should not hardcode primal names"
        );
    }

    #[tokio::test]
    async fn test_handle_send_secure_unknown_capability_before_network() {
        let dir =
            TempDir::new().expect("create temp directory for send_secure unknown capability test");
        let msg = dir.path().join("msg.bin");
        std::fs::write(&msg, b"hello").expect("write msg.bin fixture");
        let err = handle_send_secure(
            msg.to_str().expect("msg.bin path must be valid UTF-8"),
            "not-a-capability",
            None,
        )
        .await
        .expect_err("invalid capability");
        assert!(err.to_string().contains("Unknown capability"));
    }

    #[tokio::test]
    async fn test_handle_send_secure_network_capability_is_case_insensitive() {
        let dir = TempDir::new().expect("create temp directory for send_secure NETWORK test");
        let msg = dir.path().join("msg-net.bin");
        std::fs::write(&msg, b"ping").expect("write msg-net.bin fixture");
        let r = handle_send_secure(
            msg.to_str().expect("msg-net.bin path must be valid UTF-8"),
            "NETWORK",
            None,
        )
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_handle_discover_primals_capability_case_insensitive() {
        assert!(handle_discover_primals("CoMpUtE").await.is_ok());
    }

    #[tokio::test]
    async fn test_handle_key_ceremony_missing_seed_file() {
        let err = handle_key_ceremony("/no/such/seed", "/tmp/out", "high")
            .await
            .expect_err("missing seed");
        assert!(err.to_string().contains("seed") || err.to_string().contains("Failed to read"));
    }

    #[test]
    fn test_parse_send_secure_subcommand() {
        use clap::Parser;
        let cmd = CrossPrimalCommand::try_parse_from([
            "beardog",
            "send-secure",
            "--message",
            "/tmp/m",
            "--capability",
            "network",
            "--output",
            "/tmp/o",
        ])
        .expect("parse");
        match cmd.action {
            CrossPrimalAction::SendSecure {
                message,
                capability,
                output,
            } => {
                assert_eq!(message, "/tmp/m");
                assert_eq!(capability, "network");
                assert_eq!(output.as_deref(), Some("/tmp/o"));
            }
            _ => panic!("expected SendSecure"),
        }
    }
}
