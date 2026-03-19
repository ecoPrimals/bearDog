// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Self-Discovery

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        clippy::float_cmp,
        clippy::useless_vec,
        clippy::needless_range_loop,
        clippy::uninlined_format_args,
        clippy::field_reassign_with_default,
        clippy::manual_range_contains,
        unused_variables,
        dead_code
    )]

    use crate::ecosystem::primal_types::UniversalIntegrationConfig;
    use crate::ecosystem::self_discovery::{
        DiscoveredService, HealthStatus, SelfDiscoveryManager, SelfIdentity,
        UniversalCapabilityDiscovery,
    };
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    use std::collections::HashMap;

    /// Create a test self-identity
    fn create_test_identity() -> SelfIdentity {
        SelfIdentity {
            id: "test-id-123".to_string(),
            name: "TestService".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![ServiceCapabilityType::Security],
            endpoint: "https://test.example.com".to_string(),
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }

    /// Create a test integration config
    fn create_test_config() -> UniversalIntegrationConfig {
        UniversalIntegrationConfig {
            enable_capability_discovery: true,
            required_capabilities: vec![ServiceCapabilityType::Security],
            optional_capabilities: vec![],
            discovery_endpoints: vec!["https://discovery.test.com".to_string()],
            custom_config: HashMap::new(),
            enable_environment_discovery: false,
        }
    }

    #[test]
    fn test_self_identity_new() {
        let identity = SelfIdentity::new(
            "TestService",
            vec![ServiceCapabilityType::Security],
            "https://test.com",
        );

        assert_eq!(identity.name, "TestService");
        assert_eq!(identity.version, "1.0.0");
        assert_eq!(identity.capabilities.len(), 1);
        assert_eq!(identity.endpoint, "https://test.com");
        assert_eq!(identity.health_status, HealthStatus::Healthy);
        assert!(!identity.id.is_empty());
    }

    #[test]
    fn test_self_identity_beardog() {
        let identity = SelfIdentity::beardog();

        assert_eq!(identity.name, "BearDog");
        assert!(!identity.version.is_empty());
        assert!(identity.capabilities.len() >= 4);
        assert_eq!(identity.health_status, HealthStatus::Healthy);
        assert!(!identity.id.is_empty());
    }

    #[test]
    fn test_health_status_values() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unknown);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus::Healthy;
        let json = serde_json::to_string(&status).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: HealthStatus = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized, status);
    }

    #[test]
    fn test_universal_capability_discovery_new() {
        let discovery = UniversalCapabilityDiscovery::new();
        assert!(discovery.is_ok());
    }

    #[test]
    fn test_discover_by_capability_empty() {
        let discovery = UniversalCapabilityDiscovery::new().expect("Should create");
        let capability = ServiceCapabilityType::Security;

        let result = discovery.discover_by_capability(&capability);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_self_discovery_manager_new() {
        let identity = create_test_identity();
        let config = create_test_config();

        let manager = SelfDiscoveryManager::new(identity, config);
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        assert_eq!(manager.identity().name, "TestService");
    }

    #[tokio::test]
    async fn test_manager_identity_access() {
        let identity = create_test_identity();
        let config = create_test_config();

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let identity = manager.identity();
        assert_eq!(identity.name, "TestService");
        assert_eq!(identity.version, "1.0.0");
        assert_eq!(identity.health_status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_manager_update_health_status() {
        let identity = create_test_identity();
        let config = create_test_config();

        let mut manager =
            SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        manager.update_health_status(HealthStatus::Degraded);
        assert_eq!(manager.identity().health_status, HealthStatus::Degraded);

        manager.update_health_status(HealthStatus::Unhealthy);
        assert_eq!(manager.identity().health_status, HealthStatus::Unhealthy);

        manager.update_health_status(HealthStatus::Healthy);
        assert_eq!(manager.identity().health_status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_manager_register_self() {
        let identity = create_test_identity();
        let config = create_test_config();

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let result = manager.register_self();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_discover_required_capabilities_empty() {
        let identity = create_test_identity();
        let mut config = create_test_config();
        config.required_capabilities = vec![ServiceCapabilityType::Security];

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let result = manager.discover_required_capabilities();
        // Should fail because no services are found
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discover_optional_capabilities() {
        let identity = create_test_identity();
        let mut config = create_test_config();
        config.optional_capabilities =
            vec![ServiceCapabilityType::Custom("OptionalFeature".to_string())];

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let result = manager.discover_optional_capabilities();
        assert!(result.is_ok());
        // Optional capabilities should not fail if not found
        let discovered = result.unwrap();
        assert!(discovered.is_empty());
    }

    #[test]
    fn test_discovered_service_default() {
        let service = DiscoveredService::default();

        assert_eq!(service.name, "BearDog Security Provider");
        assert_eq!(service.service_id, "beardog-default");
        assert_eq!(service.capabilities.len(), 1);
        assert_eq!(service.health_status, HealthStatus::Healthy);
        assert!(!service.endpoint.is_empty());
    }

    #[test]
    fn test_discovered_service_serialization() {
        let service = DiscoveredService::default();
        let json = serde_json::to_string(&service).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: DiscoveredService =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.name, service.name);
    }

    #[test]
    fn test_self_identity_serialization() {
        let identity = create_test_identity();
        let json = serde_json::to_string(&identity).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: SelfIdentity = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.name, identity.name);
        assert_eq!(deserialized.version, identity.version);
    }

    #[test]
    fn test_self_identity_clone() {
        let identity1 = create_test_identity();
        let identity2 = identity1.clone();

        assert_eq!(identity1.name, identity2.name);
        assert_eq!(identity1.id, identity2.id);
        assert_eq!(identity1.endpoint, identity2.endpoint);
    }

    #[test]
    fn test_health_status_clone() {
        let status1 = HealthStatus::Healthy;
        let status2 = status1.clone();

        assert_eq!(status1, status2);
    }

    #[tokio::test]
    async fn test_manager_debug_format() {
        let identity = create_test_identity();
        let config = create_test_config();

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("SelfDiscoveryManager"));
    }

    #[tokio::test]
    async fn test_multiple_capabilities() {
        let mut identity = create_test_identity();
        identity.capabilities = vec![
            ServiceCapabilityType::Security,
            ServiceCapabilityType::Custom("Feature1".to_string()),
            ServiceCapabilityType::Custom("Feature2".to_string()),
        ];

        let config = create_test_config();

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        assert_eq!(manager.identity().capabilities.len(), 3);
    }

    #[tokio::test]
    async fn test_multiple_discovery_endpoints() {
        let identity = create_test_identity();
        let mut config = create_test_config();
        config.discovery_endpoints = vec![
            "https://discovery1.test.com".to_string(),
            "https://discovery2.test.com".to_string(),
            "https://discovery3.test.com".to_string(),
        ];

        let manager = SelfDiscoveryManager::new(identity, config).expect("Should create manager");

        let result = manager.register_self();
        assert!(result.is_ok());
    }

    #[test]
    fn test_discovered_service_clone() {
        let service1 = DiscoveredService::default();
        let service2 = service1.clone();

        assert_eq!(service1.name, service2.name);
        assert_eq!(service1.service_id, service2.service_id);
    }
}
