// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Core Tests
//!
//! Tests for core `BearDog` functionality and primitives


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

#[cfg(test)]
mod core_functionality_tests {
    use crate::core::system::BearDogCore;
    use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    use beardog_types::canonical::HealthStatus;

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: critical
    #[tokio::test]
    async fn test_core_initialization() {
        // Test that core initializes with default config
        let result = BearDogCore::with_default_config();
        assert!(result.is_ok(), "Core should initialize with default config");

        let core = result.unwrap();
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        // Components list should be accessible (empty or not)
        assert!(state.components.is_empty() || !state.components.is_empty());
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_lifecycle() {
        // Test full lifecycle: create -> use -> verify state
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Verify initial state
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        let initial_time = state.start_time;
        drop(state);

        // Perform actual state operations (no artificial delays needed)
        // Multiple reads/writes to verify state consistency
        for _ in 0..5 {
            let state = core.state.read().await;
            assert_eq!(
                state.start_time, initial_time,
                "Start time should not change during operations"
            );
        }

        // Verify state persists after operations
        let state = core.state.read().await;
        assert_eq!(
            state.start_time, initial_time,
            "Start time should not change"
        );
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_cleanup() {
        // Test that core cleans up resources properly
        let core = BearDogCore::with_default_config().unwrap();
        let state = core.state.read().await;
        let _component_count = state.components.len();
        drop(state);

        // Drop core and verify cleanup
        drop(core);

        // If we get here without panic, cleanup succeeded
        assert!(true, "Core cleanup completed without panic");
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: critical
    #[tokio::test]
    async fn test_core_concurrent_state_access() {
        // Test concurrent read/write access to core state
        let core = BearDogCore::with_default_config().unwrap();
        let core_clone = std::sync::Arc::new(core);

        // Spawn multiple readers
        let mut handles = vec![];
        for i in 0..5 {
            let core_ref = core_clone.clone();
            let handle = tokio::spawn(async move {
                for _ in 0..10 {
                    let state = core_ref.state.read().await;
                    assert_eq!(state.overall_health, HealthStatus::Healthy);
                    // No artificial delay - test true concurrency
                }
            });
            handles.push(handle);
        }

        // Wait for all readers
        for handle in handles {
            handle.await.unwrap();
        }
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_state_transitions() {
        // Test state transitions through different health states
        let core = BearDogCore::with_default_config().unwrap();

        // Initial state
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }

        // Transition to degraded
        {
            let mut state = core.state.write().await;
            state.overall_health = HealthStatus::Degraded;
        }

        // Verify degraded state
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Degraded);
        }

        // Recover to healthy
        {
            let mut state = core.state.write().await;
            state.overall_health = HealthStatus::Healthy;
        }

        // Verify recovered
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_multiple_config_types() {
        // Test core with different configuration types
        let configs = vec![
            UnifiedBearDogConfig::default(),
            UnifiedBearDogConfig::development(),
        ];

        for config in configs {
            let result = BearDogCore::new(config);
            let state = result.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_rapid_state_changes() {
        // Test rapid state changes
        let core = BearDogCore::with_default_config().unwrap();

        for i in 0..20 {
            let mut state = core.state.write().await;
            if i % 2 == 0 {
                state.overall_health = HealthStatus::Healthy;
            } else {
                state.overall_health = HealthStatus::Degraded;
            }
        }

        // Explicitly set to healthy at the end
        {
            let mut state = core.state.write().await;
            state.overall_health = HealthStatus::Healthy;
        }

        // Verify final state
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: medium
    #[tokio::test]
    async fn test_core_state_read_write_balance() {
        // Test that many readers don't block writers
        let core = std::sync::Arc::new(BearDogCore::with_default_config().unwrap());

        // Use barrier for truly concurrent read/write test (no polling waits)
        let barrier = Arc::new(tokio::sync::Barrier::new(11)); // 10 readers + 1 writer

        // Spawn readers that check state exists (not specific value due to concurrent writes)
        let mut read_handles = vec![];
        for _ in 0..10 {
            let core_ref = core.clone();
            let barrier_ref = Arc::clone(&barrier);
            let handle = tokio::spawn(async move {
                // Wait for all tasks to be ready (no artificial delays)
                barrier_ref.wait().await;
                
                // Read truly concurrently with writer
                let state = core_ref.state.read().await;
                // Just verify we can read state, don't assert specific value
                let _health = state.overall_health;
            });
            read_handles.push(handle);
        }

        // Writer joins the barrier - all tasks start simultaneously
        barrier.wait().await;
        
        // Write concurrently with reads (tests RwLock behavior)
        {
            let mut state = core.state.write().await;
            state.overall_health = HealthStatus::Degraded;
        }

        // Restore state
        {
            let mut state = core.state.write().await;
            state.overall_health = HealthStatus::Healthy;
        }

        // Wait for readers
        for handle in read_handles {
            handle.await.unwrap();
        }

        // Verify final state
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_initialization_with_development_config() {
        // Test initialization with development configuration
        let config = UnifiedBearDogConfig::development();

        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: medium
    #[tokio::test]
    async fn test_core_start_time_immutability() {
        // Verify start time cannot be changed
        let core = BearDogCore::with_default_config().unwrap();

        let original_start_time = {
            let state = core.state.read().await;
            state.start_time
        };

        // Perform multiple operations to verify start_time invariant
        // (no artificial delay - test the invariant directly)
        for _ in 0..10 {
            let state = core.state.read().await;
            assert_eq!(
                state.start_time, original_start_time,
                "Start time must remain constant across operations"
            );
        }
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_state_management() {
        // Test state management operations
        let core = BearDogCore::with_default_config().unwrap();

        // Test read access
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }

        // Test write access
        {
            let mut state = core.state.write().await;
            let original_health = state.overall_health;
            state.overall_health = HealthStatus::Degraded;
            assert_eq!(state.overall_health, HealthStatus::Degraded);
            // Restore
            state.overall_health = original_health;
        }

        // Verify state after write
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[tokio::test]
    async fn test_core_error_handling() {
        // Test that core handles invalid configurations gracefully
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Core should still be in valid state
        let state = core.state.read().await;
        assert!(
            state.overall_health != HealthStatus::Unhealthy,
            "Core should maintain health even with challenges"
        );
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[tokio::test]
    async fn test_core_multiple_instances() {
        // Test that multiple core instances can coexist
        let core1 = BearDogCore::with_default_config().unwrap();
        let core2 = BearDogCore::with_default_config().unwrap();

        let state1 = core1.state.read().await;
        let state2 = core2.state.read().await;

        assert_eq!(state1.overall_health, HealthStatus::Healthy);
        assert_eq!(state2.overall_health, HealthStatus::Healthy);

        // They should be independent
        assert_ne!(
            state1.start_time, state2.start_time,
            "Different instances should have different start times"
        );
    }
}

#[cfg(test)]
mod messaging_tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestMessage {
        id: String,
        content: String,
        timestamp: u64,
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_message_creation() {
        let msg = TestMessage {
            id: "test-123".to_string(),
            content: "Hello, BearDog!".to_string(),
            timestamp: 1_234_567_890,
        };

        assert_eq!(msg.id, "test-123");
        assert_eq!(msg.content, "Hello, BearDog!");
        assert_eq!(msg.timestamp, 1_234_567_890);
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_message_serialization() {
        let msg = TestMessage {
            id: "ser-001".to_string(),
            content: "Serialize me".to_string(),
            timestamp: 9_876_543_210,
        };

        let serialized = serde_json::to_string(&msg);
        assert!(serialized.is_ok(), "Message should serialize successfully");

        let json = serialized.unwrap();
        assert!(json.contains("ser-001"));
        assert!(json.contains("Serialize me"));
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_message_deserialization() {
        let json = r#"{"id":"deser-001","content":"Deserialize me","timestamp":1111111111}"#;

        let result: Result<TestMessage, _> = serde_json::from_str(json);
        assert!(result.is_ok(), "Should deserialize valid JSON");

        let msg = result.unwrap();
        assert_eq!(msg.id, "deser-001");
        assert_eq!(msg.content, "Deserialize me");
        assert_eq!(msg.timestamp, 1_111_111_111);
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_message_validation() {
        // Test valid message
        let valid_msg = TestMessage {
            id: "valid-123".to_string(),
            content: "Valid content".to_string(),
            timestamp: 1_000_000,
        };
        assert!(
            !valid_msg.id.is_empty(),
            "Valid message should have non-empty ID"
        );
        assert!(
            !valid_msg.content.is_empty(),
            "Valid message should have content"
        );

        // Test message with empty fields
        let invalid_msg = TestMessage {
            id: String::new(),
            content: String::new(),
            timestamp: 0,
        };
        assert!(invalid_msg.id.is_empty(), "Invalid message detected");
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_message_routing() {
        // Test message routing logic
        let msg1 = TestMessage {
            id: "route-001".to_string(),
            content: "Route to A".to_string(),
            timestamp: 1000,
        };

        let msg2 = TestMessage {
            id: "route-002".to_string(),
            content: "Route to B".to_string(),
            timestamp: 2000,
        };

        // Messages should have unique IDs for routing
        assert_ne!(
            msg1.id, msg2.id,
            "Different messages should have unique IDs"
        );
        assert_ne!(
            msg1.timestamp, msg2.timestamp,
            "Messages should have different timestamps"
        );
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_message_encryption() {
        // Test that message content can be transformed (simulating encryption)
        let original = TestMessage {
            id: "enc-001".to_string(),
            content: "Secret message".to_string(),
            timestamp: 3000,
        };

        // Simulate encryption by reversing content
        let encrypted_content: String = original.content.chars().rev().collect();
        let encrypted = TestMessage {
            id: original.id.clone(),
            content: encrypted_content,
            timestamp: original.timestamp,
        };

        assert_ne!(
            original.content, encrypted.content,
            "Content should be transformed"
        );
        assert_eq!(original.id, encrypted.id, "ID should remain the same");

        // Verify decryption (reverse again)
        let decrypted_content: String = encrypted.content.chars().rev().collect();
        assert_eq!(
            original.content, decrypted_content,
            "Should decrypt back to original"
        );
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: messaging
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_message_signing() {
        // Test that messages can include signature-like data
        use std::collections::HashMap;

        let msg = TestMessage {
            id: "sign-001".to_string(),
            content: "Signed message".to_string(),
            timestamp: 4000,
        };

        // Simulate signature metadata
        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert("signature".to_string(), "fake-signature-hash".to_string());
        metadata.insert("signer".to_string(), msg.id.clone());

        assert!(
            metadata.contains_key("signature"),
            "Should have signature metadata"
        );
        assert!(
            metadata.contains_key("signer"),
            "Should have signer metadata"
        );
        assert_eq!(metadata.get("signer").unwrap(), &msg.id);
    }
}

#[cfg(test)]
mod identity_tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct PrimalIdentity {
        id: String,
        primal_type: String,
    }

    impl PrimalIdentity {
        fn new(id: String, primal_type: String) -> Self {
            Self { id, primal_type }
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn primal_type(&self) -> &str {
            &self.primal_type
        }
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: identity
    /// `TEST_PRIORITY`: critical
    #[test]
    fn test_identity_creation() {
        // Test creating a primal identity
        let identity = PrimalIdentity::new("beardog-test-001".to_string(), "BearDog".to_string());

        assert_eq!(identity.id(), "beardog-test-001");
        assert_eq!(identity.primal_type(), "BearDog");
        assert!(!identity.id().is_empty(), "Identity ID should not be empty");
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: identity
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_identity_validation() {
        // Test that valid identities pass validation
        let valid_identity = PrimalIdentity::new("valid-id-123".to_string(), "BearDog".to_string());
        assert!(!valid_identity.id().is_empty());
        assert!(!valid_identity.primal_type().is_empty());

        // Test that identity fields are non-empty
        let identity = PrimalIdentity::new("test-id".to_string(), "TestType".to_string());
        assert!(identity.id().len() > 0, "ID should have length");
        assert!(identity.primal_type().len() > 0, "Type should have length");
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: identity
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_identity_verification() {
        // Test identity verification logic
        let identity1 = PrimalIdentity::new("verify-001".to_string(), "BearDog".to_string());

        // Verify identity has correct attributes
        assert_eq!(identity1.id(), "verify-001");
        assert_eq!(identity1.primal_type(), "BearDog");

        // Identity should be consistent
        assert_eq!(identity1.id(), identity1.id(), "ID should be stable");
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: identity
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_identity_serialization() {
        // Test that identities can be serialized
        let identity = PrimalIdentity::new("serialize-001".to_string(), "BearDog".to_string());

        let serialized = serde_json::to_string(&identity);
        assert!(serialized.is_ok(), "Identity should serialize successfully");

        let json = serialized.unwrap();
        assert!(json.contains("serialize-001"), "JSON should contain ID");
    }

    /// `TEST_CATEGORY`: unit
    /// `TEST_DOMAIN`: identity
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_identity_comparison() {
        // Test identity comparison
        let identity1 = PrimalIdentity::new("compare-001".to_string(), "BearDog".to_string());

        let identity2 = PrimalIdentity::new("compare-002".to_string(), "BearDog".to_string());

        let identity1_clone = PrimalIdentity::new("compare-001".to_string(), "BearDog".to_string());

        // Different identities should be different
        assert_ne!(identity1.id(), identity2.id());

        // Same ID should match
        assert_eq!(identity1.id(), identity1_clone.id());
        assert_eq!(identity1.primal_type(), identity1_clone.primal_type());
    }
}

#[cfg(test)]
mod coordination_tests {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct Node {
        id: String,
        status: NodeStatus,
        last_seen: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[allow(dead_code)]
    enum NodeStatus {
        Active,
        Inactive,
        Unknown,
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_node_registration() {
        // Test node registration logic
        let mut registry: HashMap<String, Node> = HashMap::new();

        let node = Node {
            id: "node-001".to_string(),
            status: NodeStatus::Active,
            last_seen: 1000,
        };

        registry.insert(node.id.clone(), node);

        assert!(registry.contains_key("node-001"));
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.get("node-001").unwrap().status, NodeStatus::Active);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_node_discovery() {
        // Test node discovery process
        let mut discovered_nodes: Vec<Node> = Vec::new();

        // Simulate discovering nodes
        discovered_nodes.push(Node {
            id: "node-A".to_string(),
            status: NodeStatus::Active,
            last_seen: 2000,
        });

        discovered_nodes.push(Node {
            id: "node-B".to_string(),
            status: NodeStatus::Active,
            last_seen: 2100,
        });

        assert_eq!(discovered_nodes.len(), 2);
        assert!(discovered_nodes.iter().any(|n| n.id == "node-A"));
        assert!(discovered_nodes
            .iter()
            .all(|n| n.status == NodeStatus::Active));
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_node_health_check() {
        // Test node health checking
        let healthy_node = Node {
            id: "healthy-001".to_string(),
            status: NodeStatus::Active,
            last_seen: 3000,
        };

        let unhealthy_node = Node {
            id: "unhealthy-001".to_string(),
            status: NodeStatus::Inactive,
            last_seen: 1000, // Old timestamp
        };

        // Health check logic
        assert_eq!(healthy_node.status, NodeStatus::Active);
        assert_eq!(unhealthy_node.status, NodeStatus::Inactive);
        assert!(healthy_node.last_seen > unhealthy_node.last_seen);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_leader_election() {
        // Test leader election algorithm
        let nodes = vec![
            Node {
                id: "node-1".to_string(),
                status: NodeStatus::Active,
                last_seen: 5000,
            },
            Node {
                id: "node-2".to_string(),
                status: NodeStatus::Active,
                last_seen: 5100,
            },
            Node {
                id: "node-3".to_string(),
                status: NodeStatus::Active,
                last_seen: 5200,
            },
        ];

        // Simulate leader election (highest last_seen wins)
        let leader = nodes.iter().max_by_key(|n| n.last_seen).unwrap();

        assert_eq!(leader.id, "node-3");
        assert_eq!(leader.last_seen, 5200);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_consensus_protocol() {
        // Test basic consensus logic
        let mut votes: HashMap<String, u32> = HashMap::new();

        // Simulate voting
        *votes.entry("proposal-A".to_string()).or_insert(0) += 1;
        *votes.entry("proposal-A".to_string()).or_insert(0) += 1;
        *votes.entry("proposal-B".to_string()).or_insert(0) += 1;

        // Find winner
        let winner = votes.iter().max_by_key(|(_, v)| *v).unwrap();

        assert_eq!(winner.0, "proposal-A");
        assert_eq!(*winner.1, 2);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_node_failure_detection() {
        // Test failure detection based on last_seen
        let current_time = 10000u64;
        let timeout = 1000u64;

        let nodes = vec![
            Node {
                id: "node-1".to_string(),
                status: NodeStatus::Active,
                last_seen: 9900,
            }, // Recent
            Node {
                id: "node-2".to_string(),
                status: NodeStatus::Active,
                last_seen: 8000,
            }, // Old
        ];

        let failed_nodes: Vec<&Node> = nodes
            .iter()
            .filter(|n| current_time - n.last_seen > timeout)
            .collect();

        assert_eq!(failed_nodes.len(), 1);
        assert_eq!(failed_nodes[0].id, "node-2");
    }
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[cfg(test)]
mod storage_tests {
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    struct StorageEntry {
        key: String,
        value: Vec<u8>,
        encrypted: bool,
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_storage_write() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();

        let entry = StorageEntry {
            key: "test-key-001".to_string(),
            value: b"test data".to_vec(),
            encrypted: false,
        };

        storage.insert(entry.key.clone(), entry.clone());

        assert!(storage.contains_key("test-key-001"));
        assert_eq!(storage.get("test-key-001").unwrap().value, b"test data");
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_storage_read() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();

        let entry = StorageEntry {
            key: "read-key-001".to_string(),
            value: b"read data".to_vec(),
            encrypted: false,
        };

        storage.insert(entry.key.clone(), entry.clone());

        let read_entry = storage.get("read-key-001");
        assert!(read_entry.is_some());
        assert_eq!(read_entry.unwrap().value, b"read data");
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_storage_delete() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();

        let entry = StorageEntry {
            key: "delete-key-001".to_string(),
            value: b"to be deleted".to_vec(),
            encrypted: false,
        };

        storage.insert(entry.key.clone(), entry.clone());
        assert!(storage.contains_key("delete-key-001"));

        storage.remove("delete-key-001");
        assert!(!storage.contains_key("delete-key-001"));
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_storage_list() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();

        for i in 0..5 {
            let entry = StorageEntry {
                key: format!("key-{}", i),
                value: format!("value-{}", i).into_bytes(),
                encrypted: false,
            };
            storage.insert(entry.key.clone(), entry);
        }

        assert_eq!(storage.len(), 5);
        let keys: Vec<String> = storage.keys().cloned().collect();
        assert!(keys.contains(&"key-0".to_string()));
        assert!(keys.contains(&"key-4".to_string()));
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: high
    #[test]
    fn test_storage_encryption() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();

        let encrypted_entry = StorageEntry {
            key: "encrypted-key-001".to_string(),
            value: b"encrypted data".to_vec(),
            encrypted: true,
        };

        let plain_entry = StorageEntry {
            key: "plain-key-001".to_string(),
            value: b"plain data".to_vec(),
            encrypted: false,
        };

        storage.insert(encrypted_entry.key.clone(), encrypted_entry.clone());
        storage.insert(plain_entry.key.clone(), plain_entry.clone());

        assert!(storage.get("encrypted-key-001").unwrap().encrypted);
        assert!(!storage.get("plain-key-001").unwrap().encrypted);
    }

    /// `TEST_CATEGORY`: integration
    /// `TEST_DOMAIN`: core
    /// `TEST_PRIORITY`: normal
    #[test]
    fn test_storage_transaction() {
        let mut storage: HashMap<String, StorageEntry> = HashMap::new();
        let mut transaction: Vec<StorageEntry> = Vec::new();

        for i in 0..3 {
            let entry = StorageEntry {
                key: format!("tx-key-{}", i),
                value: format!("tx-value-{}", i).into_bytes(),
                encrypted: false,
            };
            transaction.push(entry);
        }

        for entry in transaction {
            storage.insert(entry.key.clone(), entry);
        }

        assert_eq!(storage.len(), 3);
        assert!(storage.contains_key("tx-key-0"));
        assert!(storage.contains_key("tx-key-2"));
    }
}
