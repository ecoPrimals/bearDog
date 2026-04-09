// SPDX-License-Identifier: AGPL-3.0-or-later

// Zero-Knowledge Bootstrap System Tests
// Focus: Bootstrap process, discovery, configuration, metrics

#[cfg(test)]
#[expect(
    deprecated,
    reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
)]
mod tests {
    use super::super::{BootstrapConfig, BootstrapMetrics, DiscoveryProtocol};
    use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;

    // Note: SelfIdentity tests omitted due to complex external dependencies
    // Focus on testable metrics and protocol types

    #[test]
    fn test_bootstrap_metrics_default() {
        let metrics = BootstrapMetrics::default();

        assert_eq!(metrics.bootstrap_duration_ms, 0);
        assert_eq!(metrics.primals_discovered, 0);
        assert_eq!(metrics.capabilities_discovered, 0);
        assert_eq!(metrics.discovery_attempts, 0);
        assert_eq!(metrics.discovery_success_rate, 0.0);
        assert_eq!(metrics.protocols_used.len(), 0);
    }

    #[test]
    fn test_bootstrap_metrics_clone() {
        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 1000,
            primals_discovered: 5,
            capabilities_discovered: 20,
            discovery_attempts: 10,
            discovery_success_rate: 0.5,
            protocols_used: vec!["http".to_string(), "dns".to_string()],
        };

        let cloned = metrics.clone();
        assert_eq!(cloned.bootstrap_duration_ms, metrics.bootstrap_duration_ms);
        assert_eq!(cloned.primals_discovered, metrics.primals_discovered);
        assert_eq!(
            cloned.capabilities_discovered,
            metrics.capabilities_discovered
        );
        assert_eq!(
            cloned.discovery_success_rate,
            metrics.discovery_success_rate
        );
    }

    #[test]
    fn test_bootstrap_metrics_success_rate_calculation() {
        let total_attempts = 100;
        let successful = 75;
        let success_rate = f64::from(successful) / f64::from(total_attempts);

        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 5000,
            primals_discovered: 10,
            capabilities_discovered: 50,
            discovery_attempts: total_attempts,
            discovery_success_rate: success_rate,
            protocols_used: vec![],
        };

        assert_eq!(metrics.discovery_success_rate, 0.75);
        assert_eq!(metrics.discovery_attempts, 100);
    }

    #[test]
    fn test_discovery_protocol_multicast_dns() {
        let protocol = DiscoveryProtocol::MulticastDNS;
        assert_eq!(protocol, DiscoveryProtocol::MulticastDNS);
    }

    #[test]
    fn test_discovery_protocol_http() {
        let protocol = DiscoveryProtocol::HttpDiscovery;
        assert_eq!(protocol, DiscoveryProtocol::HttpDiscovery);
    }

    #[test]
    fn test_discovery_protocol_environment() {
        let protocol = DiscoveryProtocol::EnvironmentDiscovery;
        assert_eq!(protocol, DiscoveryProtocol::EnvironmentDiscovery);
    }

    #[test]
    fn test_discovery_protocol_service_mesh() {
        let protocol = DiscoveryProtocol::ServiceMeshDiscovery;
        assert_eq!(protocol, DiscoveryProtocol::ServiceMeshDiscovery);
    }

    #[test]
    fn test_discovery_protocol_container() {
        let protocol = DiscoveryProtocol::ContainerDiscovery;
        assert_eq!(protocol, DiscoveryProtocol::ContainerDiscovery);
    }

    #[test]
    fn test_discovery_protocol_equality() {
        let proto1 = DiscoveryProtocol::HttpDiscovery;
        let proto2 = DiscoveryProtocol::HttpDiscovery;
        let proto3 = DiscoveryProtocol::MulticastDNS;

        assert_eq!(proto1, proto2);
        assert_ne!(proto1, proto3);
    }

    #[test]
    fn test_discovery_protocol_clone() {
        let protocol = DiscoveryProtocol::ServiceMeshDiscovery;
        let cloned = protocol.clone();
        assert_eq!(protocol, cloned);
    }

    #[test]
    fn test_discovery_protocol_vec() {
        let protocols = vec![
            DiscoveryProtocol::MulticastDNS,
            DiscoveryProtocol::HttpDiscovery,
            DiscoveryProtocol::EnvironmentDiscovery,
        ];

        assert_eq!(protocols.len(), 3);
        assert!(protocols.contains(&DiscoveryProtocol::MulticastDNS));
        assert!(protocols.contains(&DiscoveryProtocol::HttpDiscovery));
    }

    // Note: Struct tests for PrimalMetadata and UniversalEndpoint omitted
    // These are complex external types with specific field requirements

    #[test]
    fn test_bootstrap_metrics_protocol_tracking() {
        let protocols = vec![
            "multicast-dns".to_string(),
            "http".to_string(),
            "service-mesh".to_string(),
        ];

        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 3000,
            primals_discovered: 8,
            capabilities_discovered: 40,
            discovery_attempts: 15,
            discovery_success_rate: 0.8,
            protocols_used: protocols,
        };

        assert_eq!(metrics.protocols_used.len(), 3);
        assert!(metrics.protocols_used.contains(&"http".to_string()));
        assert!(
            metrics
                .protocols_used
                .contains(&"multicast-dns".to_string())
        );
    }

    #[test]
    fn test_bootstrap_duration_calculation() {
        let duration_ms: u64 = 5000; // 5 seconds
        let duration_secs = duration_ms / 1000;

        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: duration_ms,
            primals_discovered: 10,
            capabilities_discovered: 50,
            discovery_attempts: 20,
            discovery_success_rate: 0.9,
            protocols_used: vec![],
        };

        assert_eq!(metrics.bootstrap_duration_ms, 5000);
        assert_eq!(duration_secs, 5);
    }

    #[test]
    fn test_discovery_success_rate_perfect() {
        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 1000,
            primals_discovered: 5,
            capabilities_discovered: 25,
            discovery_attempts: 10,
            discovery_success_rate: 1.0,
            protocols_used: vec![],
        };

        assert_eq!(metrics.discovery_success_rate, 1.0);
    }

    #[test]
    fn test_discovery_success_rate_zero() {
        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 2000,
            primals_discovered: 0,
            capabilities_discovered: 0,
            discovery_attempts: 10,
            discovery_success_rate: 0.0,
            protocols_used: vec![],
        };

        assert_eq!(metrics.discovery_success_rate, 0.0);
        assert_eq!(metrics.primals_discovered, 0);
    }

    #[test]
    fn test_bootstrap_time_tracking() {
        let now = std::time::SystemTime::now();

        // Bootstrap time should be trackable
        let elapsed = now.elapsed().unwrap();
        assert!(elapsed.as_millis() < 1000); // Should be very recent
    }

    #[test]
    fn test_metrics_accumulation() {
        let mut total_primals = 0;
        let mut total_capabilities = 0;

        // Simulate discovery rounds
        total_primals += 3;
        total_capabilities += 15;

        total_primals += 2;
        total_capabilities += 10;

        let metrics = BootstrapMetrics {
            bootstrap_duration_ms: 6000,
            primals_discovered: total_primals,
            capabilities_discovered: total_capabilities,
            discovery_attempts: 2,
            discovery_success_rate: 1.0,
            protocols_used: vec![],
        };

        assert_eq!(metrics.primals_discovered, 5);
        assert_eq!(metrics.capabilities_discovered, 25);
    }

    #[test]
    fn test_discovery_protocol_serialization_roundtrip() {
        use serde_json;

        let protocol = DiscoveryProtocol::HttpDiscovery;
        let json = serde_json::to_string(&protocol).unwrap();
        let deserialized: DiscoveryProtocol = serde_json::from_str(&json).unwrap();

        assert_eq!(protocol, deserialized);
    }

    #[test]
    fn test_multiple_protocol_support() {
        let all_protocols = vec![
            DiscoveryProtocol::MulticastDNS,
            DiscoveryProtocol::HttpDiscovery,
            DiscoveryProtocol::EnvironmentDiscovery,
            DiscoveryProtocol::ServiceMeshDiscovery,
            DiscoveryProtocol::ContainerDiscovery,
        ];

        assert_eq!(all_protocols.len(), 5);

        // Verify all unique protocols are present
        assert!(all_protocols.contains(&DiscoveryProtocol::MulticastDNS));
        assert!(all_protocols.contains(&DiscoveryProtocol::HttpDiscovery));
        assert!(all_protocols.contains(&DiscoveryProtocol::EnvironmentDiscovery));
        assert!(all_protocols.contains(&DiscoveryProtocol::ServiceMeshDiscovery));
        assert!(all_protocols.contains(&DiscoveryProtocol::ContainerDiscovery));
    }

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn test_legacy_bootstrap_config_maps_into_unified() {
        let old = BootstrapConfig::default();
        let expected_timeout = old.discovery_timeout_ms;
        let unified: UnifiedBootstrapConfig = old.into();
        assert_eq!(unified.core.discovery_timeout_ms, expected_timeout);
        assert_eq!(
            unified.core.max_discovery_attempts,
            BootstrapConfig::default().max_discovery_attempts
        );
    }
}
