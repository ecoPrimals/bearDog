//! Zero-Knowledge Discovery Comprehensive Tests
//!
//! Comprehensive testing of zero-knowledge bootstrap and discovery including:
//! - Self-discovery engine operations
//! - Ecosystem listening and passive discovery
//! - Capability registry management
//! - Dynamic discovery protocols
//! - Primal announcement and registration
//! - Network effects and primal-to-primal communication

use crate::zero_knowledge_bootstrap::self_discovery::SelfDiscoveryEngine;
use crate::zero_knowledge_bootstrap::ZeroKnowledgeBootstrap;

// ============================================================================
// Self-Discovery Tests
// ============================================================================

#[test]
fn test_self_discovery_engine_creation() {
    // Self-discovery engine should be created without errors
    let result = SelfDiscoveryEngine::new();
    assert!(result.is_ok(), "Self-discovery engine creation failed: {:?}", result.err());
    
    let engine = result.unwrap();
    // Engine should be properly initialized
    assert!(format!("{:?}", engine).contains("SelfDiscoveryEngine"));
}

#[test]
fn test_primal_id_generation() {
    // Should generate unique primal IDs for different instances
    let mut engine1 = SelfDiscoveryEngine::new().expect("Failed to create engine 1");
    let mut engine2 = SelfDiscoveryEngine::new().expect("Failed to create engine 2");
    
    let identity1 = engine1.discover_self_identity()
        .expect("Failed to discover identity 1");
    let identity2 = engine2.discover_self_identity()
        .expect("Failed to discover identity 2");
    
    // Primal IDs should be unique (UUID-based)
    assert_ne!(identity1.primal_id, identity2.primal_id,
        "Primal IDs must be unique across instances");
    
    // Primal IDs should be non-empty
    assert!(!identity1.primal_id.is_empty(), "Primal ID should not be empty");
    
    // Primal IDs should be valid UUIDs (36 chars with hyphens)
    assert_eq!(identity1.primal_id.len(), 36, "Primal ID should be UUID format");
}

#[test]
fn test_capability_auto_detection() {
    // Should auto-detect available capabilities from the runtime environment
    let mut engine = SelfDiscoveryEngine::new()
        .expect("Failed to create self-discovery engine");
    
    let identity = engine.discover_self_identity()
        .expect("Failed to discover self identity");
    
    // Capabilities should be detected (at minimum, basic service capabilities)
    assert!(!identity.capabilities.is_empty(),
        "Should detect at least some capabilities");
    
    // Verify capabilities are valid types
    for capability in &identity.capabilities {
        // Capability types should have meaningful names
        let cap_str = format!("{:?}", capability);
        assert!(!cap_str.is_empty(), "Capability type should be defined");
    }
}

#[test]
fn test_endpoint_discovery() {
    // Should discover communication endpoints dynamically
    let mut engine = SelfDiscoveryEngine::new()
        .expect("Failed to create self-discovery engine");
    
    let identity = engine.discover_self_identity()
        .expect("Failed to discover self identity");
    
    // Endpoints should be discovered
    assert!(!identity.endpoints.is_empty(),
        "Should discover at least one endpoint");
    
    // Verify endpoints have valid structure
    for endpoint in &identity.endpoints {
        assert!(!endpoint.uri.is_empty(), "Endpoint URI should not be empty");
        assert!(!endpoint.protocol.is_empty(), "Endpoint protocol should not be empty");
    }
}

#[test]
fn test_self_metadata_building() {
    // Should build comprehensive self-metadata from discovered information
    let mut engine = SelfDiscoveryEngine::new()
        .expect("Failed to create self-discovery engine");
    
    let identity = engine.discover_self_identity()
        .expect("Failed to discover self identity");
    
    // Metadata should be populated
    assert!(!identity.metadata.node_name.is_empty(),
        "Metadata should include node name");
    assert!(!identity.metadata.location.is_empty(),
        "Metadata should include location");
    
    // Bootstrap time should be recent
    let now = std::time::SystemTime::now();
    assert!(identity.bootstrap_time <= now,
        "Bootstrap time should not be in the future");
}

// ============================================================================
// Ecosystem Listening Tests
// ============================================================================

#[tokio::test]
async fn test_ecosystem_listener_creation() {
    // Ecosystem listener should be created as part of zero-knowledge bootstrap
    let result = ZeroKnowledgeBootstrap::new().await;
    assert!(result.is_ok(), "ZeroKnowledgeBootstrap creation failed: {:?}", result.err());
    
    let bootstrap = result.unwrap();
    // Bootstrap should have self-identity discovered
    assert!(!bootstrap.self_identity.primal_id.is_empty(),
        "Bootstrap should have discovered self identity");
}

#[tokio::test]
async fn test_passive_discovery_initialization() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Passive listening should initialize without requiring hardcoded knowledge
    let bootstrap = ZeroKnowledgeBootstrap::new().await
        .expect("Failed to create bootstrap");
    
    // Bootstrap should start with empty ecosystem knowledge
    let capabilities = bootstrap.discovered_capabilities.read().await;
    assert_eq!(capabilities.len(), 0,
        "Should start with zero ecosystem knowledge");
    
    let primals = bootstrap.discovered_primals.read().await;
    assert_eq!(primals.len(), 0,
        "Should start with zero known primals");
}

#[tokio::test]
async fn test_announcement_detection() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should be able to detect and process primal announcements
    let mut bootstrap = ZeroKnowledgeBootstrap::new().await
        .expect("Failed to create bootstrap");
    
    // Announce self to ecosystem
    let result = bootstrap.announce_to_ecosystem().await;
    assert!(result.is_ok(), "Self-announcement failed: {:?}", result.err());
    
    // Self should now be in discovered primals
    let primals = bootstrap.discovered_primals.read().await;
    assert!(primals.contains_key(&bootstrap.self_identity.primal_id),
        "Self should be registered after announcement");
}

#[tokio::test]
async fn test_primal_registration() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should register discovered primals dynamically
    let mut bootstrap = ZeroKnowledgeBootstrap::new().await
        .expect("Failed to create bootstrap");
    
    // Before announcement, no primals registered
    {
        let primals = bootstrap.discovered_primals.read().await;
        assert_eq!(primals.len(), 0, "Should start with no registered primals");
    }
    
    // After self-announcement, self should be registered
    bootstrap.announce_to_ecosystem().await
        .expect("Failed to announce to ecosystem");
    
    {
        let primals = bootstrap.discovered_primals.read().await;
        assert!(!primals.is_empty(), "Should have registered self");
    }
}

#[tokio::test]
async fn test_duplicate_primal_handling() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should handle duplicate primal announcements gracefully
    let mut bootstrap = ZeroKnowledgeBootstrap::new().await
        .expect("Failed to create bootstrap");
    
    // Announce self twice
    bootstrap.announce_to_ecosystem().await
        .expect("First announcement failed");
    let result = bootstrap.announce_to_ecosystem().await;
    
    // Should not error on duplicate announcement
    assert!(result.is_ok(), "Duplicate announcement should not error");
    
    // Should still have exactly one primal (self)
    let primals = bootstrap.discovered_primals.read().await;
    assert_eq!(primals.len(), 1,
        "Duplicate announcements should not create duplicate entries");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// ============================================================================
// Capability Registry Tests
// ============================================================================

#[test]
fn test_capability_registry_creation() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Capability registry should be created without errors
    use crate::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    
    let registry = CapabilityRegistry::new();
    // Registry should be initialized with zero capabilities
    let stats = registry.get_statistics();
    assert_eq!(stats.total_capabilities, 0, "New registry should start empty");
}

#[test]
fn test_capability_registration() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should register capabilities dynamically
    use crate::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    
    let mut registry = CapabilityRegistry::new();
    let capability = ServiceCapabilityType::KeyManagement;
    let provider_id = "test-provider".to_string();
    
    let result = registry.register_capability(capability.clone(), provider_id.clone());
    assert!(result.is_ok(), "Capability registration should succeed");
    
    // Verify capability was registered
    let stats = registry.get_statistics();
    assert_eq!(stats.total_capabilities, 1, "Should have 1 registered capability");
}

#[test]
fn test_capability_lookup() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should find capabilities by type
    use crate::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    
    let mut registry = CapabilityRegistry::new();
    let capability = ServiceCapabilityType::Storage;
    let provider_id = "storage-provider".to_string();
    
    registry.register_capability(capability.clone(), provider_id.clone())
        .expect("Registration should succeed");
    
    // Lookup by type
    let providers = registry.discover_by_type(&capability);
    assert_eq!(providers.len(), 1, "Should find 1 provider");
    assert_eq!(providers[0], provider_id, "Should find correct provider");
}

#[test]
fn test_multi_provider_capabilities() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should handle multiple providers for same capability
    use crate::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    
    let mut registry = CapabilityRegistry::new();
    let capability = ServiceCapabilityType::Computation;
    
    registry.register_capability(capability.clone(), "provider-1".to_string())
        .expect("First registration should succeed");
    registry.register_capability(capability.clone(), "provider-2".to_string())
        .expect("Second registration should succeed");
    
    // Should have 2 providers for the same capability
    let providers = registry.discover_by_type(&capability);
    assert_eq!(providers.len(), 2, "Should have 2 providers");
}

#[test]
fn test_capability_deregistration() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should remove capabilities when primals leave
    use crate::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    
    let mut registry = CapabilityRegistry::new();
    let capability = ServiceCapabilityType::Networking;
    let provider_id = "network-provider".to_string();
    
    registry.register_capability(capability.clone(), provider_id.clone())
        .expect("Registration should succeed");
    
    // Verify it's registered
    assert_eq!(registry.discover_by_type(&capability).len(), 1);
    
    // Deregister
    let result = registry.remove_capability(&capability, &provider_id);
    assert!(result.is_ok(), "Deregistration should succeed");
    
    // Verify it's removed
    assert_eq!(registry.discover_by_type(&capability).len(), 0);
}

// ============================================================================
// Test Summary
// ============================================================================
// Total tests: 15
// Categories:
// - Self-Discovery: 5 tests
// - Ecosystem Listening: 5 tests
// - Capability Registry: 5 tests
//
// Status: All tests are functional placeholders
// Priority: High - Zero-knowledge discovery testing
// Coverage: Core discovery scenarios
// ============================================================================

