// SPDX-License-Identifier: AGPL-3.0-or-later

//! Targeted coverage tests for low-coverage beardog-types modules:
//! kubernetes discovery, factory, production config, and universal types.

// -- KubernetesDiscovery: runtime detection --

mod kubernetes_discovery_tests {
    use crate::canonical::discovery::service_discovery_capability::KubernetesDiscovery;

    #[tokio::test]
    async fn kubernetes_try_create_returns_result() {
        let _ = KubernetesDiscovery::try_create().await;
    }
}

// -- Factory: create_service_discovery Kubernetes or DNS fallback --

mod factory_tests {
    use crate::canonical::discovery::service_discovery_capability::ServiceDiscoveryCapability;
    use crate::canonical::discovery::service_discovery_capability::create_service_discovery;

    #[tokio::test]
    async fn create_service_discovery_returns_kubernetes_or_dns() {
        let discovery = create_service_discovery()
            .await
            .expect("factory should always succeed with a fallback");
        let name = discovery.provider_name();
        assert!(
            name.contains("kubernetes") || name.contains("dns"),
            "expected kubernetes or dns-http-fallback, got: {name}"
        );
    }
}

// -- UnifiedProductionConfig --

mod production_config_tests {
    use crate::canonical::config::production::UnifiedProductionConfig;
    use crate::canonical::config::production::core::EnvironmentLevel;

    #[test]
    fn new_returns_development_level() {
        let config = UnifiedProductionConfig::new();
        assert!(!config.is_production());
    }

    #[test]
    fn validate_default_config() {
        let config = UnifiedProductionConfig::default();
        config.validate().expect("default config should validate");
    }

    #[test]
    fn is_production_default_is_false() {
        let config = UnifiedProductionConfig::default();
        assert!(
            !config.is_production(),
            "default is development, not production"
        );
    }

    #[test]
    fn environment_level_default_is_development() {
        let config = UnifiedProductionConfig::default();
        assert_eq!(*config.environment_level(), EnvironmentLevel::Development);
    }

    #[test]
    fn service_info_returns_non_empty_tuple() {
        let config = UnifiedProductionConfig::default();
        let (name, version, deployment_id) = config.service_info();
        assert!(!name.is_empty());
        assert!(!version.is_empty());
        assert!(!deployment_id.is_empty());
    }
}

// -- Universal discovery types: Default impls and serde roundtrip --

mod universal_types_tests {
    use crate::canonical::discovery::universal::{
        PerformanceProfile, PerformanceRequirements, SecurityLevel, SecurityRequirements,
        UniversalCapabilityType, UniversalDiscoveryRequest,
    };

    #[test]
    fn performance_requirements_default_values() {
        let pr = PerformanceRequirements::default();
        assert_eq!(pr.max_latency_ms, Some(5000));
        assert_eq!(pr.min_throughput_ops_per_sec, Some(10.0));
        assert_eq!(pr.required_availability, Some(0.95));
    }

    #[test]
    fn security_requirements_default_values() {
        let sr = SecurityRequirements::default();
        assert!(sr.require_encryption);
        assert!(sr.require_mutual_auth);
        assert_eq!(sr.min_security_level, SecurityLevel::High);
    }

    #[test]
    fn performance_profile_default_values() {
        let pp = PerformanceProfile::default();
        assert_eq!(pp.avg_response_time_ms, 0.0);
        assert_eq!(pp.success_rate, 1.0);
        assert_eq!(pp.availability, 1.0);
    }

    #[test]
    fn universal_discovery_request_serde_roundtrip() {
        let req = UniversalDiscoveryRequest {
            required_capabilities: vec![UniversalCapabilityType::Compute { abilities: vec![] }],
            optional_capabilities: vec![],
            performance_requirements: PerformanceRequirements::default(),
            security_requirements: SecurityRequirements::default(),
        };

        let json = serde_json::to_string(&req).expect("serialize");
        let roundtrip: UniversalDiscoveryRequest =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(roundtrip.required_capabilities.len(), 1);
    }
}
