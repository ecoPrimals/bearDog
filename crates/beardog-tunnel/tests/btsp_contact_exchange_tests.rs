//! Unit tests for BTSP contact exchange
//!
//! Tests the genetic lineage-based NAT traversal contact discovery.

use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;
use beardog_tunnel::btsp_provider::{BeardogBtspProvider, ContactInfo};
use beardog_tunnel::tunnel::hsm::HsmManager;
use std::sync::Arc;

#[tokio::test]
#[ignore] // Requires HSM initialization
async fn test_contact_exchange_same_family() {
    // Initialize components with auto_initialize for proper HSM provider registration
    use std::env;
    env::set_var("BEARDOG_HSM_MODE", "software");
    let hsm = Arc::new(
        HsmManager::auto_initialize()
            .await
            .expect("Failed to initialize HSM"),
    );
    env::remove_var("BEARDOG_HSM_MODE");

    let genetics =
        Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetics engine"));

    // Create BTSP provider
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("Failed to create BTSP provider");

    // Set environment for testing
    std::env::set_var("FAMILY_ID", "test_family");

    // Test contact exchange (will return empty path for unknown peer)
    let result = provider
        .contact_exchange("unknown_peer", "test_lineage", 3)
        .await;

    // Should fail for unknown peer
    assert!(result.is_err(), "Should fail for unknown peer");
}

#[tokio::test]
async fn test_contact_info_serialization() {
    use chrono::Utc;
    use serde_json;

    let contact = ContactInfo {
        peer_id: "tower-b".to_string(),
        addresses: vec![
            "192.168.1.5:10000".to_string(),
            "10.0.0.3:10001".to_string(),
        ],
        lineage_proof: "lineage_proof_abc123".to_string(),
        lineage_path: vec!["genesis".to_string(), "tower-b".to_string()],
        search_depth: 2,
        last_seen: Utc::now(),
    };

    // Test serialization
    let json = serde_json::to_string(&contact).expect("Failed to serialize");
    assert!(json.contains("tower-b"));
    assert!(json.contains("192.168.1.5:10000"));

    // Test deserialization
    let deserialized: ContactInfo = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.peer_id, "tower-b");
    assert_eq!(deserialized.addresses.len(), 2);
    assert_eq!(deserialized.search_depth, 2);
}

#[tokio::test]
async fn test_lineage_path_environment() {
    // Test that environment variables are used for family discovery
    std::env::set_var("BEARDOG_FAMILY_ID", "nat0");
    std::env::set_var("BEARDOG_NODE_ID", "tower1");

    let family = std::env::var("FAMILY_ID")
        .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
        .unwrap();

    assert_eq!(family, "nat0");

    // Clean up
    std::env::remove_var("BEARDOG_FAMILY_ID");
    std::env::remove_var("BEARDOG_NODE_ID");
}

#[tokio::test]
#[ignore] // Requires HSM initialization
async fn test_contact_exchange_max_hops() {
    // Initialize components with auto_initialize for proper HSM provider registration
    use std::env;
    env::set_var("BEARDOG_HSM_MODE", "software");
    let hsm = Arc::new(
        HsmManager::auto_initialize()
            .await
            .expect("Failed to initialize HSM"),
    );
    env::remove_var("BEARDOG_HSM_MODE");

    let genetics =
        Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetics engine"));

    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("Failed to create BTSP provider");

    std::env::set_var("FAMILY_ID", "test_family");

    // Test with different max_hops values
    let result1 = provider
        .contact_exchange("unknown_peer", "test_lineage", 1)
        .await;
    let result2 = provider
        .contact_exchange("unknown_peer", "test_lineage", 10)
        .await;

    // Both should fail for unknown peer, regardless of max_hops
    assert!(result1.is_err());
    assert!(result2.is_err());
}

#[test]
fn test_contact_info_structure() {
    use chrono::Utc;

    // Test that ContactInfo has all required fields
    let contact = ContactInfo {
        peer_id: "test-peer".to_string(),
        addresses: vec!["127.0.0.1:9000".to_string()],
        lineage_proof: "proof123".to_string(),
        lineage_path: vec!["root".to_string(), "test-peer".to_string()],
        search_depth: 2,
        last_seen: Utc::now(),
    };

    assert_eq!(contact.peer_id, "test-peer");
    assert_eq!(contact.addresses.len(), 1);
    assert_eq!(contact.lineage_path.len(), 2);
    assert!(contact.lineage_proof.starts_with("proof"));
}
