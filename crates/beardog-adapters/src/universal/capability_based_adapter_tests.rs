//! Tests for UniversalCapabilityAdapter
//!
//! Comprehensive test suite for capability-based adapter including
//! discovery, connection management, and sovereignty validation.

use super::*;

// Test constants to eliminate hardcoding
const TEST_PORT: u16 = 8080;

    #[tokio::test]
    fn test_universal_adapter_creation() {
        let adapter = UniversalCapabilityAdapter::new()
            ?;

        assert_eq!(adapter.metrics.capabilities_discovered, 0);
        assert_eq!(adapter.metrics.primals_discovered, 0);
    }

    #[tokio::test]
    fn test_capability_registration() {
        let mut adapter = UniversalCapabilityAdapter::new()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            ?;

        let capability = UniversalCapability {
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: UniversalEndpoint {
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: adapters
                // TEST_PRIORITY: normal
                url: format!("http://test:{}", TEST_PORT),
                protocols: vec!["HTTP".to_string()],
                auth_requirements: AuthRequirements::default(),
                security_config: EndpointSecurityConfig::default(),
            },
            metadata: HashMap::new(),
            health_status: HealthStatus::Healthy,
            performance_metrics: PerformanceMetrics::default(),
        };

        adapter
            .register_capability_provider(capability)
            ?;

        let capabilities = adapter.get_available_capabilities()?;
        assert!(capabilities.contains_key(&ServiceCapabilityType::Security));
        assert_eq!(capabilities[&ServiceCapabilityType::Security].len(), 1);
    }

    #[tokio::test]
    fn test_capability_discovery() {
        let mut adapter = UniversalCapabilityAdapter::new()?;

        // Register a test capability
        let capability = UniversalCapability {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            capability_type: ServiceCapabilityType::ComputeIntelligence,
            provider_id: "compute-provider".to_string(),
            endpoint: UniversalEndpoint {
                url: "http://compute:8081".to_string(),
                protocols: vec!["HTTP".to_string()],
                auth_requirements: AuthRequirements::default(),
                security_config: EndpointSecurityConfig::default(),
            },
            metadata: HashMap::new(),
            health_status: HealthStatus::Healthy,
            performance_metrics: PerformanceMetrics::default(),
        };

        adapter
            .register_capability_provider(capability)
            ?;

        // Discover compute intelligence capability
        let request = CapabilityDiscoveryRequest::compute_intelligence();
        let result = adapter.discover_capability(request)?;

        assert!(!result.discovered_providers.is_empty());
        assert_eq!(
            result.discovered_providers[0].provider.capability_type,
            ServiceCapabilityType::ComputeIntelligence
        );
    }

    #[tokio::test]
    fn test_no_hardcoded_vendor_preferences() {
        // Test that capability requests don't contain hardcoded vendor preferences
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let compute_request = CapabilityDiscoveryRequest::compute_intelligence();
        let key_mgmt_request = CapabilityDiscoveryRequest::key_management();
        let mesh_request = CapabilityDiscoveryRequest::service_mesh();

        // All requests should have empty vendor preferences (true sovereignty)
        assert!(compute_request.preferences.vendor_preferences.is_empty());
        assert!(key_mgmt_request.preferences.vendor_preferences.is_empty());
        assert!(mesh_request.preferences.vendor_preferences.is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_connection_management() {
        let mut adapter = UniversalCapabilityAdapter::new()?;

        // Register and connect to a capability
        let capability = UniversalCapability {
            capability_type: ServiceCapabilityType::Security,
            provider_id: "security-provider".to_string(),
            endpoint: UniversalEndpoint {
                url: format!("http://security:{}", TEST_PORT),
                protocols: vec!["HTTPS".to_string()],
                auth_requirements: AuthRequirements::default(),
                security_config: EndpointSecurityConfig::default(),
            },
            metadata: HashMap::new(),
            health_status: HealthStatus::Healthy,
            performance_metrics: PerformanceMetrics::default(),
        };

        let connection_id = adapter.connect_to_capability(&capability)?;
        assert!(!connection_id.is_empty());

        // Test health check
        let health_statuses = adapter.health_check_all_connections()?;
        assert!(health_statuses.contains_key(&connection_id));
    }
}
