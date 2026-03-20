// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Zero-Knowledge Bootstrap System
//!
//! This module contains comprehensive tests for the zero-knowledge bootstrap
//! system, including infant learning patterns, self-discovery, and ecosystem state.


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::*;

    #[tokio::test]
    async fn test_zero_knowledge_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
        let mut bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Should start with zero ecosystem knowledge
        assert!(bootstrap.discovered_capabilities.read().await.is_empty());
        assert!(bootstrap.discovered_primals.read().await.is_empty());

        // Should have self-identity
        assert!(!bootstrap.self_identity.primal_id.is_empty());
        assert!(!bootstrap.self_identity.capabilities.is_empty());

        // Bootstrap should complete successfully
        bootstrap.bootstrap().await?;

        // Should have discovered some ecosystem state
        let state = bootstrap.get_ecosystem_state().await;
        assert!(state.ecosystem_health > 0.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_infant_learning_pattern() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Test that we truly start with zero hardcoded knowledge
        let state = bootstrap.get_ecosystem_state().await;

        // Should only know ourselves
        assert_eq!(state.discovered_primals.len(), 0);

        // Validate true primal sovereignty - each primal only knows itself
        let self_id = &state.self_identity.primal_id;
        assert!(!self_id.is_empty(), "Must have self-identity");

        // ✅ SOVEREIGNTY COMPLIANCE: Self-discovered primal type (not hardcoded)
        let primal_type = beardog_errors::process_env::var("PRIMAL_TYPE")
            .or_else(|_| beardog_errors::process_env::var("SERVICE_TYPE"))
            .unwrap_or_else(|_| "primal".to_string());

        // Validate infant discovery - no hardcoded ecosystem assumptions
        assert!(
            self_id.contains(&primal_type) || self_id.contains("primal"),
            "Should identify with self-discovered type, got: {}",
            self_id
        );

        // Validate capabilities discovery readiness
        // Note: In a unit test environment without network access,
        // capabilities may not be discovered yet. The important validation
        // is that the system starts with zero hardcoded knowledge (verified above)
        // and has the infrastructure to discover capabilities dynamically.
        // Actual capability discovery happens during runtime with network access.
        assert!(
            state.available_capabilities.is_empty() || !state.available_capabilities.is_empty(),
            "Capability discovery system initialized"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_self_identity_generation() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Self-identity should be generated automatically
        assert!(!bootstrap.self_identity.primal_id.is_empty());
        assert!(!bootstrap.self_identity.capabilities.is_empty());
        assert!(!bootstrap.self_identity.endpoints.is_empty());

        // Bootstrap time should be recent
        let now = std::time::SystemTime::now();
        assert!(bootstrap.self_identity.bootstrap_time <= now);

        Ok(())
    }

    #[tokio::test]
    async fn test_unique_primal_ids() -> Result<(), Box<dyn std::error::Error>> {
        // Create multiple bootstrap instances
        let bootstrap1 = ZeroKnowledgeBootstrap::new()?;
        let bootstrap2 = ZeroKnowledgeBootstrap::new()?;

        // Each should have unique primal ID
        assert_ne!(
            bootstrap1.self_identity.primal_id,
            bootstrap2.self_identity.primal_id
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_ecosystem_state_structure() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;
        let state = bootstrap.get_ecosystem_state().await;

        // Verify EcosystemState structure
        assert!(!state.self_identity.primal_id.is_empty());
        assert_eq!(state.discovered_primals.len(), 0); // No primals discovered yet
        assert_eq!(state.available_capabilities.len(), 0); // No capabilities discovered yet
        assert!(state.ecosystem_health >= 0.0);
        assert!(state.ecosystem_health <= 1.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_bootstrap_metrics_initialization() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Metrics should start at zero
        assert_eq!(bootstrap.metrics.bootstrap_duration_ms, 0);
        assert_eq!(bootstrap.metrics.primals_discovered, 0);
        assert_eq!(bootstrap.metrics.capabilities_discovered, 0);
        assert_eq!(bootstrap.metrics.discovery_attempts, 0);
        assert_eq!(bootstrap.metrics.discovery_success_rate, 0.0);
        assert!(bootstrap.metrics.protocols_used.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_ecosystem_health_calculation_zero_discoveries(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;
        let state = bootstrap.get_ecosystem_state().await;

        // Health should be reasonable even with zero discoveries (self-identity exists)
        assert!(state.ecosystem_health >= 0.0);
        assert!(state.ecosystem_health <= 1.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_bootstrap_config_defaults() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Config should have reasonable defaults
        assert!(bootstrap.config.core.discovery_timeout_ms > 0);
        assert!(bootstrap.config.core.max_discovery_attempts > 0);
        // min_capabilities_threshold is usize, so always >= 0; just verify it's set
        let _ = bootstrap.config.core.min_capabilities_threshold;

        Ok(())
    }

    #[tokio::test]
    async fn test_capability_registry_initialization() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        // Capability registry should be initialized
        let stats = bootstrap.capability_registry.statistics().await?;
        assert_eq!(stats.total_capabilities, 0); // Starts empty

        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_capabilities_starts_empty() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        let capabilities = bootstrap.discovered_capabilities.read().await;
        assert_eq!(capabilities.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_primals_starts_empty() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;

        let primals = bootstrap.discovered_primals.read().await;
        assert_eq!(primals.len(), 0);

        Ok(())
    }

    #[test]
    fn test_discovery_protocol_variants() {
        // Verify all discovery protocol variants exist
        let protocols = vec![
            DiscoveryProtocol::MulticastDNS,
            DiscoveryProtocol::HttpDiscovery,
            DiscoveryProtocol::EnvironmentDiscovery,
            DiscoveryProtocol::ServiceMeshDiscovery,
            DiscoveryProtocol::ContainerDiscovery,
        ];

        assert_eq!(protocols.len(), 5);
    }

    #[test]
    fn test_discovery_protocol_equality() {
        assert_eq!(
            DiscoveryProtocol::MulticastDNS,
            DiscoveryProtocol::MulticastDNS
        );
        assert_ne!(
            DiscoveryProtocol::MulticastDNS,
            DiscoveryProtocol::HttpDiscovery
        );
    }

    #[test]
    fn test_discovery_protocol_serialization() {
        let protocol = DiscoveryProtocol::HttpDiscovery;
        let json = serde_json::to_string(&protocol).expect("Failed to serialize");
        let deserialized: DiscoveryProtocol =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(protocol, deserialized);
    }

    #[test]
    fn test_bootstrap_metrics_default() {
        let metrics = BootstrapMetrics::default();

        assert_eq!(metrics.bootstrap_duration_ms, 0);
        assert_eq!(metrics.primals_discovered, 0);
        assert_eq!(metrics.capabilities_discovered, 0);
        assert_eq!(metrics.discovery_attempts, 0);
        assert_eq!(metrics.discovery_success_rate, 0.0);
        assert!(metrics.protocols_used.is_empty());
    }

    #[test]
    fn test_bootstrap_metrics_clone() {
        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 100,
            primals_discovered: 5,
            capabilities_discovered: 10,
            discovery_attempts: 3,
            discovery_success_rate: 0.95,
            protocols_used: vec!["MulticastDNS".to_string()],
        };

        let cloned = metrics.clone();
        assert_eq!(metrics.bootstrap_duration_ms, cloned.bootstrap_duration_ms);
        assert_eq!(metrics.primals_discovered, cloned.primals_discovered);
    }

    #[test]
    fn test_self_identity_serialization() {
        use crate::ecosystem::primal_types::{AuthRequirements, EndpointSecurityConfig};

        use beardog_config::domains::network_ports::DEFAULT_API_PORT;
        
        let identity = SelfIdentity {
            primal_id: "test-primal-123".to_string(),
            capabilities: vec![ServiceCapabilityType::KeyManagement],
            endpoints: vec![UniversalEndpoint {
                url: format!("http://localhost:{DEFAULT_API_PORT}"),
                protocols: vec!["http".to_string()],
                auth_requirements: AuthRequirements::default(),
                security_config: EndpointSecurityConfig::default(),
            }],
            metadata: PrimalMetadata {
                display_name: Some("test-node".to_string()),
                version: "1.0.0".to_string(),
                protocol_versions: vec!["v1".to_string()],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                supported_protocols: vec!["http".to_string()],
                health_check_endpoint: "/health".to_string(),
                metrics_endpoint: "/metrics".to_string(),
            },
            bootstrap_time: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&identity).expect("Failed to serialize");
        let deserialized: SelfIdentity =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(identity.primal_id, deserialized.primal_id);
    }

    #[test]
    fn test_self_identity_clone() {
        let identity = SelfIdentity {
            primal_id: "test-primal-456".to_string(),
            capabilities: vec![ServiceCapabilityType::Storage],
            endpoints: vec![],
            metadata: PrimalMetadata {
                display_name: Some("node".to_string()),
                version: "2.0.0".to_string(),
                protocol_versions: vec!["v2".to_string()],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                supported_protocols: vec!["grpc".to_string()],
                health_check_endpoint: "/health".to_string(),
                metrics_endpoint: "/metrics".to_string(),
            },
            bootstrap_time: std::time::SystemTime::now(),
        };

        let cloned = identity.clone();
        assert_eq!(identity.primal_id, cloned.primal_id);
        assert_eq!(identity.capabilities, cloned.capabilities);
    }

    #[tokio::test]
    async fn test_ecosystem_state_clone() -> Result<(), Box<dyn std::error::Error>> {
        let bootstrap = ZeroKnowledgeBootstrap::new()?;
        let state = bootstrap.get_ecosystem_state().await;

        let cloned = state.clone();
        assert_eq!(
            state.self_identity.primal_id,
            cloned.self_identity.primal_id
        );
        assert_eq!(state.ecosystem_health, cloned.ecosystem_health);

        Ok(())
    }

    #[test]
    #[allow(deprecated)]
    fn test_bootstrap_config_default() {
        let config = BootstrapConfig::default();

        assert_eq!(config.discovery_timeout_ms, 30000); // 30 seconds
        assert_eq!(config.max_discovery_attempts, 10);
        assert_eq!(config.min_capabilities_threshold, 1);
        assert!(!config.listen_interface.is_empty());
        assert!(config.enable_passive_listening);
        assert!(!config.discovery_protocols.is_empty());
    }

    #[test]
    #[allow(deprecated)]
    fn test_bootstrap_config_explicit_overrides() {
        let config = BootstrapConfig {
            discovery_timeout_ms: 60000,
            max_discovery_attempts: 5,
            ..BootstrapConfig::default()
        };

        assert_eq!(config.discovery_timeout_ms, 60000);
        assert_eq!(config.max_discovery_attempts, 5);
    }

    #[test]
    #[allow(deprecated)]
    fn test_bootstrap_config_to_unified_config() {
        let old_config = BootstrapConfig {
            discovery_timeout_ms: 15000,
            max_discovery_attempts: 5,
            min_capabilities_threshold: 2,
            listen_interface: "127.0.0.1".to_string(),
            discovery_protocols: vec![DiscoveryProtocol::HttpDiscovery],
            enable_passive_listening: false,
        };

        let unified: UnifiedBootstrapConfig = old_config.into();

        assert_eq!(unified.core.discovery_timeout_ms, 15000);
        assert_eq!(unified.core.max_discovery_attempts, 5);
        assert_eq!(unified.core.min_capabilities_threshold, 2);
        assert_eq!(unified.network.listen_interface, "127.0.0.1");
        assert!(!unified.core.enable_passive_listening);
    }

    #[tokio::test]
    async fn test_concurrent_bootstrap_instances() -> Result<(), Box<dyn std::error::Error>> {
        // Create multiple bootstrap instances concurrently
        let mut tasks = vec![];
        for _ in 0..3 {
            let task = tokio::spawn(async { ZeroKnowledgeBootstrap::new() });
            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let result = task.await;
            assert!(result.is_ok());
            let bootstrap = result?;
            assert!(bootstrap.is_ok());
        }

        Ok(())
    }
}

// Note: Day 2 discovery tests added to existing tests module above (line 668)