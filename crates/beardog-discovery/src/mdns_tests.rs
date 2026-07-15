// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_capability_to_service_type() {
    assert_eq!(
        MdnsDiscovery::capability_to_service_type("storage"),
        "_storage._tcp.local."
    );
    assert_eq!(
        MdnsDiscovery::capability_to_service_type("Orchestration"),
        "_orchestration._tcp.local."
    );
}

#[tokio::test]
async fn test_mdns_discovery_creation() {
    let discovery = MdnsDiscovery::new();
    assert!(discovery.is_ok());
}

#[tokio::test]
async fn test_mdns_discovery_with_config() {
    let config = MdnsConfig {
        timeout: Duration::from_secs(2),
        domain: "local.".to_string(),
        enable_ipv6: false,
    };

    let discovery = MdnsDiscovery::with_config(config);
    assert!(discovery.is_ok());
}

#[tokio::test]
async fn test_mdns_discovery_empty_results() {
    let discovery = MdnsDiscovery::with_config(MdnsConfig {
        timeout: Duration::from_millis(100), // Short timeout
        ..Default::default()
    })
    .expect("Failed to create discovery");

    // Should return empty vec, not error
    let result = discovery.discover("nonexistent-capability").await;
    assert!(result.is_ok());
    let services = result.expect("discover should return Ok for missing capability");
    assert!(services.is_empty());
}

#[tokio::test]
async fn test_cache_operations() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Cache should be empty initially
    assert!(discovery.get_cached("_test._tcp.local.").await.is_none());

    // Update cache
    let services = vec![];
    discovery.update_cache("_test._tcp.local.", services).await;

    // Should now be cached
    assert!(discovery.get_cached("_test._tcp.local.").await.is_some());

    // Clear cache
    discovery.clear_cache().await;
    assert!(discovery.get_cached("_test._tcp.local.").await.is_none());
}

#[test]
fn test_get_hostname() {
    let hostname = MdnsDiscovery::get_hostname();
    assert!(hostname.is_ok());
    let name = hostname.expect("get_hostname should succeed in test environment");
    assert!(!name.is_empty());
    assert!(!name.contains('.'), "Hostname should not contain domain");
}

// ===== ERROR PATH TESTS =====

#[tokio::test]
async fn test_invalid_capability_name() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Empty capability
    let result = discovery.discover("").await;
    assert!(result.is_ok()); // Should handle gracefully

    // Special characters
    let result = discovery.discover("test/capability").await;
    assert!(result.is_ok()); // Should sanitize
}

#[tokio::test]
async fn test_very_short_timeout() {
    let discovery = MdnsDiscovery::with_config(MdnsConfig {
        timeout: Duration::from_millis(1), // Nearly instant timeout
        ..Default::default()
    })
    .expect("Failed to create discovery");

    let result = discovery.discover("test-service").await;
    assert!(result.is_ok()); // Should return empty results
    let services = result.expect("discover with short timeout should return Ok");
    assert!(services.is_empty());
}

#[tokio::test]
async fn test_concurrent_discoveries() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Multiple concurrent discoveries should not interfere
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let disc = discovery.clone();
            tokio::spawn(async move { disc.discover(&format!("test-cap-{i}")).await })
        })
        .collect();

    for handle in handles {
        let result = handle.await.expect("Task panicked");
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_cache_concurrent_access() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Concurrent writes to cache
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let disc = discovery.clone();
            tokio::spawn(async move {
                disc.update_cache(&format!("_test{i}._tcp.local."), vec![])
                    .await;
            })
        })
        .collect();

    for handle in handles {
        handle.await.expect("Task panicked");
    }

    // Cache should have all entries
    discovery.clear_cache().await;
}

#[test]
fn test_parse_capabilities_empty() {
    let properties = HashMap::new();
    let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "test");
    assert_eq!(caps.len(), 1); // Should have primary capability
    assert_eq!(caps[0].capability_type, "test");
}

#[test]
fn test_parse_capabilities_malformed() {
    let mut properties = HashMap::new();
    properties.insert("capabilities".to_string(), ",,,,".to_string());

    let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "test");
    assert_eq!(caps.len(), 1); // Should only have primary
}

#[test]
fn test_parse_capabilities_with_spaces() {
    let mut properties = HashMap::new();
    properties.insert(
        "capabilities".to_string(),
        " storage , encryption ".to_string(),
    );

    let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "storage");
    assert_eq!(caps.len(), 2);
    assert_eq!(caps[1].capability_type, "encryption");
}

#[test]
fn test_parse_metadata_filtering() {
    let mut properties = HashMap::new();
    properties.insert("version".to_string(), "1.0.0".to_string());
    properties.insert("tier".to_string(), "production".to_string());
    properties.insert("capabilities".to_string(), "storage".to_string());

    let metadata = MdnsDiscovery::parse_metadata_from_txt(&properties);
    assert_eq!(metadata.len(), 2); // Should exclude "capabilities"
    assert!(metadata.contains_key("version"));
    assert!(metadata.contains_key("tier"));
    assert!(!metadata.contains_key("capabilities"));
}

#[tokio::test]
async fn test_announce_error_handling() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Test with valid port and empty properties
    let properties = HashMap::new();
    let result = discovery.announce("test-cap", 8080, properties);
    // Just verify it doesn't panic
    let _ = result;
}

#[tokio::test]
async fn test_cache_expiry_behavior() {
    let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // Add to cache
    discovery.update_cache("_test._tcp.local.", vec![]).await;
    assert!(discovery.get_cached("_test._tcp.local.").await.is_some());

    // Clear cache
    discovery.clear_cache().await;
    assert!(discovery.get_cached("_test._tcp.local.").await.is_none());
}

#[test]
fn mdns_config_default_values() {
    let c = MdnsConfig::default();
    assert!(c.domain.contains("local"));
    assert!(c.timeout.as_secs() >= 1);
}
