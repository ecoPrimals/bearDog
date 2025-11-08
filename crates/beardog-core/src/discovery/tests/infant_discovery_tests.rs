//! Infant Discovery System Tests
//!
//! Comprehensive tests for zero-knowledge bootstrap discovery

use crate::discovery::infant_discovery::*;

#[cfg(test)]
mod infant_discovery_tests {
    use super::*;

    #[test]
    fn test_discovered_capability_creation() {
        let capability = DiscoveredCapability {
            capability_id: "test-capability".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://localhost:8080".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec!["compute".to_string(), "storage".to_string()],
            trust_level: 0.8,
            performance_profile: PerformanceProfile::default(),
        };

        assert_eq!(capability.capability_id, "test-capability");
        assert_eq!(capability.abilities.len(), 2);
        assert_eq!(capability.trust_level, 0.8);
    }

    #[test]
    fn test_communication_protocol_http() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let protocol = CommunicationProtocol::Http {
            endpoint: "https://api.example.com".to_string(),
            headers: headers.clone(),
        };

        match protocol {
            CommunicationProtocol::Http { endpoint, headers: h } => {
                assert_eq!(endpoint, "https://api.example.com");
                assert_eq!(h.len(), 1);
                assert_eq!(h.get("Authorization").unwrap(), "Bearer token");
            }
            _ => panic!("Expected Http protocol"),
        }
    }

    #[test]
    fn test_communication_protocol_grpc() {
        let protocol = CommunicationProtocol::Grpc {
            endpoint: "localhost:50051".to_string(),
            service_name: "MyService".to_string(),
        };

        match protocol {
            CommunicationProtocol::Grpc {
                endpoint,
                service_name,
            } => {
                assert_eq!(endpoint, "localhost:50051");
                assert_eq!(service_name, "MyService");
            }
            _ => panic!("Expected Grpc protocol"),
        }
    }

    #[test]
    fn test_communication_protocol_unix_socket() {
        let protocol = CommunicationProtocol::UnixSocket {
            path: "/var/run/beardog.sock".to_string(),
        };

        match protocol {
            CommunicationProtocol::UnixSocket { path } => {
                assert_eq!(path, "/var/run/beardog.sock");
            }
            _ => panic!("Expected UnixSocket protocol"),
        }
    }

    #[test]
    fn test_communication_protocol_environment() {
        let protocol = CommunicationProtocol::Environment {
            variables: vec!["API_KEY".to_string(), "API_SECRET".to_string()],
        };

        match protocol {
            CommunicationProtocol::Environment { variables } => {
                assert_eq!(variables.len(), 2);
                assert!(variables.contains(&"API_KEY".to_string()));
            }
            _ => panic!("Expected Environment protocol"),
        }
    }

    #[test]
    fn test_communication_protocol_filesystem() {
        let protocol = CommunicationProtocol::FileSystem {
            paths: vec!["/etc/config".to_string(), "/var/data".to_string()],
        };

        match protocol {
            CommunicationProtocol::FileSystem { paths } => {
                assert_eq!(paths.len(), 2);
                assert_eq!(paths[0], "/etc/config");
            }
            _ => panic!("Expected FileSystem protocol"),
        }
    }

    #[test]
    fn test_performance_profile_default() {
        let profile = PerformanceProfile::default();

        assert_eq!(profile.avg_response_time_ms, 0.0);
        assert_eq!(profile.success_rate, 0.0);
        assert_eq!(profile.throughput_ops_per_sec, 0.0);
    }

    #[test]
    fn test_performance_profile_custom() {
        let profile = PerformanceProfile {
            avg_response_time_ms: 15.5,
            success_rate: 0.99,
            throughput_ops_per_sec: 1000.0,
        };

        assert_eq!(profile.avg_response_time_ms, 15.5);
        assert_eq!(profile.success_rate, 0.99);
        assert_eq!(profile.throughput_ops_per_sec, 1000.0);
    }

    #[test]
    fn test_capability_trust_level_range() {
        let low_trust = DiscoveredCapability {
            capability_id: "untrusted".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec![],
            trust_level: 0.1,
            performance_profile: PerformanceProfile::default(),
        };

        let high_trust = DiscoveredCapability {
            capability_id: "trusted".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec![],
            trust_level: 0.95,
            performance_profile: PerformanceProfile::default(),
        };

        assert!(low_trust.trust_level < 0.5);
        assert!(high_trust.trust_level > 0.9);
    }

    #[test]
    fn test_capability_multiple_abilities() {
        let capability = DiscoveredCapability {
            capability_id: "multi-cap".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec![
                "compute".to_string(),
                "storage".to_string(),
                "networking".to_string(),
                "security".to_string(),
            ],
            trust_level: 0.8,
            performance_profile: PerformanceProfile::default(),
        };

        assert_eq!(capability.abilities.len(), 4);
        assert!(capability.abilities.contains(&"compute".to_string()));
        assert!(capability.abilities.contains(&"security".to_string()));
    }

    #[test]
    fn test_capability_serialization() {
        let capability = DiscoveredCapability {
            capability_id: "serialize-test".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec!["test".to_string()],
            trust_level: 0.7,
            performance_profile: PerformanceProfile::default(),
        };

        // Test that it can be serialized (validates derive macros)
        let serialized = serde_json::to_string(&capability);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_capability_clone() {
        let capability1 = DiscoveredCapability {
            capability_id: "clone-test".to_string(),
            communication_protocol: CommunicationProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: std::collections::HashMap::new(),
            },
            abilities: vec!["ability1".to_string()],
            trust_level: 0.6,
            performance_profile: PerformanceProfile::default(),
        };

        let capability2 = capability1.clone();

        assert_eq!(capability1.capability_id, capability2.capability_id);
        assert_eq!(capability1.trust_level, capability2.trust_level);
        assert_eq!(capability1.abilities.len(), capability2.abilities.len());
    }

    #[test]
    fn test_learning_pattern_creation() {
        let pattern = LearningPattern {
            name: "http-detection".to_string(),
            detection_method: DetectionMethod::NetworkEndpoint {
                host: "localhost".to_string(),
                port: 8080,
            },
            validation_method: ValidationMethod::HttpPing,
        };

        assert_eq!(pattern.name, "http-detection");
    }

    #[test]
    fn test_high_performance_profile() {
        let profile = PerformanceProfile {
            avg_response_time_ms: 5.0,
            success_rate: 0.999,
            throughput_ops_per_sec: 10000.0,
        };

        assert!(profile.avg_response_time_ms < 10.0);
        assert!(profile.success_rate > 0.99);
        assert!(profile.throughput_ops_per_sec > 5000.0);
    }

    #[test]
    fn test_low_performance_profile() {
        let profile = PerformanceProfile {
            avg_response_time_ms: 500.0,
            success_rate: 0.80,
            throughput_ops_per_sec: 10.0,
        };

        assert!(profile.avg_response_time_ms > 100.0);
        assert!(profile.success_rate < 0.90);
        assert!(profile.throughput_ops_per_sec < 100.0);
    }
}

