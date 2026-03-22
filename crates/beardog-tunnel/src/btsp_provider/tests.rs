// SPDX-License-Identifier: AGPL-3.0-only

//! BTSP Provider Tests
//!
//! Comprehensive test suite for BearDog Tunnel Security Protocol provider.
//! Tests cover peer info serialization, tunnel lifecycle, and basic operations.

use super::*;
use crate::tunnel::hsm::SoftwareHsmConfig;
use crate::tunnel::hsm::manager::HsmManager;
use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
use std::sync::Arc;

/// Helper function to create a test HSM for provider testing
async fn create_test_hsm() -> Arc<HsmManager> {
    let mut hsm = HsmManager::new();
    let software_hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Software HSM creation failed");
    hsm.register_hsm_provider(
        crate::tunnel::hsm::types::HsmTier::Software,
        Arc::new(software_hsm),
    )
    .expect("Provider registration failed");
    Arc::new(hsm)
}

#[tokio::test]
async fn test_peer_info_serialization() {
    let peer = PeerInfo {
        id: "test-peer-123".to_string(),
        endpoint: "192.168.1.100:8080".to_string(),
        public_key: Some(vec![1, 2, 3, 4]),
    };

    let json = serde_json::to_string(&peer).expect("Serialize failed");
    let deserialized: PeerInfo = serde_json::from_str(&json).expect("Deserialize failed");

    assert_eq!(peer.id, deserialized.id);
    assert_eq!(peer.endpoint, deserialized.endpoint);
    assert_eq!(peer.public_key, deserialized.public_key);
}

#[tokio::test]
async fn test_tunnel_activity_tracking() {
    let tunnel = Tunnel::new(
        "test-tunnel".to_string(),
        "test-peer".to_string(),
        "192.168.1.1:8080".to_string(),
        vec![0u8; 32],
        TrustLevel::Trusted,
    );

    assert!(tunnel.is_active());
}

#[tokio::test]
async fn test_tunnel_status_transitions() {
    let tunnel = Tunnel::new(
        "test-tunnel".to_string(),
        "test-peer".to_string(),
        "192.168.1.1:8080".to_string(),
        vec![0u8; 32],
        TrustLevel::Verified,
    );

    // Initial state
    assert!(tunnel.is_active());
    assert_eq!(tunnel._trust_level, TrustLevel::Verified);
}

#[tokio::test]
async fn test_security_context_creation() {
    let context = SecurityContext {
        tunnel_id: "test-tunnel-456".to_string(),
        direction: Direction::Outbound,
    };

    assert_eq!(context.tunnel_id, "test-tunnel-456");
    assert_eq!(context.direction, Direction::Outbound);
}

#[tokio::test]
async fn test_trust_level_ordering() {
    // Verify trust level hierarchy
    assert!(TrustLevel::Verified > TrustLevel::Trusted);
    assert!(TrustLevel::Trusted > TrustLevel::Tentative);
    assert!(TrustLevel::Tentative > TrustLevel::Unknown);
}

#[tokio::test]
async fn test_direction_serialization() {
    // Test Direction enum serialization
    let outbound = Direction::Outbound;
    let json = serde_json::to_string(&outbound).expect("Serialize failed");
    let deserialized: Direction = serde_json::from_str(&json).expect("Deserialize failed");
    assert_eq!(outbound, deserialized);

    let inbound = Direction::Inbound;
    let json = serde_json::to_string(&inbound).expect("Serialize failed");
    let deserialized: Direction = serde_json::from_str(&json).expect("Deserialize failed");
    assert_eq!(inbound, deserialized);
}

#[tokio::test]
async fn test_birdsong_master_key_derivation() {
    // Create HSM manager with software provider
    let hsm = create_test_hsm().await;

    // Generate BirdSong master key
    use crate::tunnel::hsm::KeyType;
    let key = hsm
        .generate_key("test_birdsong_master", &KeyType::ChaCha20)
        .await
        .expect("Key generation failed");

    // Verify key has material
    use crate::tunnel::hsm::KeyMaterial;
    match &key.key_material {
        KeyMaterial::Encrypted { encrypted_data, .. } => {
            assert!(!encrypted_data.is_empty(), "Key material must not be empty");
        }
        _ => {
            // Other variants are also valid
        }
    }
}

#[tokio::test]
async fn test_cleanup_session_key_no_op() {
    // Create HSM manager with software provider
    let hsm = create_test_hsm().await;

    // Create genetics engine
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

    // Initialize BTSP provider
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("Provider init failed");

    // Cleanup should succeed (no-op for BirdSong)
    let result = provider.cleanup_session_key("test_peer").await;
    assert!(result.is_ok(), "Cleanup should succeed");
}

#[tokio::test]
async fn test_get_metrics_and_tunnel_queries() {
    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let m = provider.get_metrics();
    assert_eq!(m.tunnels_established, 0);
    assert_eq!(m.tunnels_active, 0);

    assert!(provider.get_tunnel("nope").is_none());
    assert!(provider.get_peer_trust_record("nope").is_none());

    let bs = provider.birdsong_manager();
    assert!(Arc::strong_count(&bs) >= 1);
}

#[tokio::test]
async fn test_contact_exchange_fails_without_trusted_peer() {
    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let err = provider
        .contact_exchange("unknown-peer", "lineage", 3)
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("not found") || msg.contains("genetic") || msg.contains("lineage"),
        "{msg}"
    );
}

#[tokio::test]
async fn test_establish_tunnel_registers_with_get_tunnel() {
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider};

    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let handle = provider
        .establish_tunnel(PeerEndpoint {
            id: "registered-peer".to_string(),
            endpoint: "unix:///tmp/registered.sock".to_string(),
            public_key: Some(vec![3u8; 32]),
        })
        .await
        .expect("establish");

    let found = provider.get_tunnel(&handle.id);
    assert!(found.is_some());
    let (_, peer_id) = found.expect("tunnel");
    assert_eq!(peer_id, "registered-peer");
}

#[tokio::test]
async fn test_establish_tunnel_rejects_empty_peer_endpoint() {
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider};

    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let err = provider
        .establish_tunnel(PeerEndpoint {
            id: "empty-endpoint-peer".to_string(),
            endpoint: String::new(),
            public_key: Some(vec![9u8; 32]),
        })
        .await
        .expect_err("empty endpoint must be rejected");
    let s = format!("{err}");
    assert!(
        s.to_lowercase().contains("endpoint") || s.contains("empty"),
        "{s}"
    );
}

#[tokio::test]
async fn test_secure_tunnel_ops_fail_for_unknown_tunnel_handle() {
    use beardog_capabilities::traits::{SecureTunnelProvider, TunnelHandle};

    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let fake = TunnelHandle {
        id: "no-such-tunnel-uuid".to_string(),
        peer_id: "ghost".to_string(),
        established_at: chrono::Utc::now().to_rfc3339(),
    };

    assert!(provider.tunnel_encrypt(&fake, b"data").await.is_err());
    assert!(
        provider
            .tunnel_decrypt(&fake, b"xxxxxxxxxxxx")
            .await
            .is_err()
    );
    assert!(provider.tunnel_status(&fake).await.is_err());
}

#[tokio::test]
async fn test_tunnel_decrypt_rejects_ciphertext_shorter_than_nonce() {
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider};

    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let handle = provider
        .establish_tunnel(PeerEndpoint {
            id: "short-cipher-peer".to_string(),
            endpoint: "unix:///tmp/short.sock".to_string(),
            public_key: Some(vec![7u8; 32]),
        })
        .await
        .expect("establish");

    let err = provider
        .tunnel_decrypt(&handle, b"tooshort")
        .await
        .expect_err("ciphertext must include 12-byte nonce");
    assert!(format!("{err}").to_lowercase().contains("short"));
}

#[tokio::test]
async fn test_close_tunnel_returns_not_found_for_unknown_handle() {
    use beardog_capabilities::traits::{SecureTunnelProvider, TunnelHandle};

    let hsm = create_test_hsm().await;
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("provider");

    let fake = TunnelHandle {
        id: "missing-close".to_string(),
        peer_id: "x".to_string(),
        established_at: chrono::Utc::now().to_rfc3339(),
    };

    let err = provider
        .close_tunnel(&fake)
        .await
        .expect_err("unknown tunnel");
    let s = format!("{err}");
    assert!(
        s.to_lowercase().contains("not found") || s.contains("Tunnel"),
        "{s}"
    );
}

#[test]
fn test_get_discovery_socket_paths_non_empty_and_ordered_fallbacks() {
    let paths = BeardogBtspProvider::get_discovery_socket_paths();
    assert!(!paths.is_empty());
    let joined = paths.join(",");
    assert!(
        joined.contains("primal") || joined.contains("discovery") || joined.contains("beardog"),
        "{joined}"
    );
}

#[test]
fn test_get_discovery_socket_paths_prefers_ipc_socket_env() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("ipc.sock");
    let path_str = sock.to_string_lossy().to_string();
    beardog_errors::process_env::set_var("IPC_SOCKET", &path_str);
    beardog_errors::process_env::remove_var("DISCOVERY_SOCKET");
    let paths = BeardogBtspProvider::get_discovery_socket_paths();
    beardog_errors::process_env::remove_var("IPC_SOCKET");
    assert_eq!(paths.first().map(String::as_str), Some(path_str.as_str()));
}

#[test]
fn test_get_discovery_socket_paths_skips_empty_ipc_socket() {
    beardog_errors::process_env::set_var("IPC_SOCKET", "");
    beardog_errors::process_env::remove_var("DISCOVERY_SOCKET");
    let paths = BeardogBtspProvider::get_discovery_socket_paths();
    beardog_errors::process_env::remove_var("IPC_SOCKET");
    assert!(
        !paths.is_empty(),
        "fallback paths must remain when IPC_SOCKET is empty"
    );
    assert!(!paths.iter().any(|p| p.is_empty()));
}

#[test]
fn test_get_discovery_socket_paths_dedupes_dev_override() {
    let dir = tempfile::tempdir().expect("tempdir");
    let dev = dir.path().join("dev.sock").to_string_lossy().to_string();
    beardog_errors::process_env::remove_var("IPC_SOCKET");
    beardog_errors::process_env::remove_var("DISCOVERY_SOCKET");
    beardog_errors::process_env::set_var("BEARDOG_DEV_DISCOVERY_SOCKET", &dev);
    let paths = BeardogBtspProvider::get_discovery_socket_paths();
    beardog_errors::process_env::remove_var("BEARDOG_DEV_DISCOVERY_SOCKET");
    let count = paths.iter().filter(|p| p.as_str() == dev.as_str()).count();
    assert_eq!(count, 1, "duplicate dev path should appear once: {paths:?}");
}
