use crate::universal_discovery::DiscoveryProtocol;
use std::collections::HashMap;

#[cfg(test)]
mod discovery_protocol_tests {
    use super::*;

    // ============================================================================
    // HTTP Discovery Protocol Tests
    // ============================================================================

    #[test]
    fn test_http_protocol_creation() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        
        let protocol = DiscoveryProtocol::Http {
            endpoint: "http://localhost:8080/discovery".to_string(),
            headers: headers.clone(),
        };

        match protocol {
            DiscoveryProtocol::Http { endpoint, headers: h } => {
                assert_eq!(endpoint, "http://localhost:8080/discovery");
                assert_eq!(h.get("Authorization"), Some(&"Bearer token123".to_string()));
            }
            _ => panic!("Expected HTTP protocol"),
        }
    }

    #[test]
    fn test_http_protocol_clone() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let protocol = DiscoveryProtocol::Http {
            endpoint: "http://api.example.com".to_string(),
            headers,
        };

        let cloned = protocol.clone();
        assert_eq!(format!("{:?}", protocol), format!("{:?}", cloned));
    }

    #[test]
    fn test_http_protocol_serialization() {
        let protocol = DiscoveryProtocol::Http {
            endpoint: "http://localhost:9090".to_string(),
            headers: HashMap::new(),
        };

        let json = serde_json::to_string(&protocol).expect("Serialization failed");
        let deserialized: DiscoveryProtocol = serde_json::from_str(&json).expect("Deserialization failed");
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        match deserialized {
            DiscoveryProtocol::Http { endpoint, .. } => {
                assert_eq!(endpoint, "http://localhost:9090");
            }
            _ => panic!("Deserialization produced wrong variant"),
        }
    }

    // ============================================================================
    // DNS Discovery Protocol Tests
    // ============================================================================

    #[test]
    fn test_dns_protocol_creation() {
        let protocol = DiscoveryProtocol::Dns {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            domain: "services.example.com".to_string(),
            servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
        };

        match protocol {
            DiscoveryProtocol::Dns { domain, servers } => {
                assert_eq!(domain, "services.example.com");
                assert_eq!(servers.len(), 2);
                assert_eq!(servers[0], "8.8.8.8");
            }
            _ => panic!("Expected DNS protocol"),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        }
    }

    #[test]
    fn test_dns_protocol_empty_servers() {
        let protocol = DiscoveryProtocol::Dns {
            domain: "test.local".to_string(),
            servers: vec![],
        };

        match protocol {
            DiscoveryProtocol::Dns { servers, .. } => {
                assert!(servers.is_empty());
            }
            _ => panic!("Expected DNS protocol"),
        }
    }

    // ============================================================================
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // mDNS Discovery Protocol Tests
    // ============================================================================

    #[test]
    fn test_mdns_protocol_creation() {
        let protocol = DiscoveryProtocol::Mdns {
            service_type: "_http._tcp".to_string(),
            interface: "eth0".to_string(),
            timeout_ms: 5000,
            continuous_monitoring: true,
        };

        match protocol {
            DiscoveryProtocol::Mdns { service_type, interface, timeout_ms, continuous_monitoring } => {
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                assert_eq!(service_type, "_http._tcp");
                assert_eq!(interface, "eth0");
                assert_eq!(timeout_ms, 5000);
                assert!(continuous_monitoring);
            }
            _ => panic!("Expected mDNS protocol"),
        }
    }

    #[test]
    fn test_mdns_protocol_no_monitoring() {
        let protocol = DiscoveryProtocol::Mdns {
            service_type: "_beardog._tcp".to_string(),
            interface: "lo".to_string(),
            timeout_ms: 1000,
            continuous_monitoring: false,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        };

        match protocol {
            DiscoveryProtocol::Mdns { continuous_monitoring, .. } => {
                assert!(!continuous_monitoring);
            }
            _ => panic!("Expected mDNS protocol"),
        }
    }

    // ============================================================================
    // Consul Discovery Protocol Tests
    // ============================================================================

    #[test]
    fn test_consul_protocol_creation() {
        let protocol = DiscoveryProtocol::Consul {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            address: "localhost:8500".to_string(),
            datacenter: "dc1".to_string(),
        };

        match protocol {
            DiscoveryProtocol::Consul { address, datacenter } => {
                assert_eq!(address, "localhost:8500");
                assert_eq!(datacenter, "dc1");
            }
            _ => panic!("Expected Consul protocol"),
        }
    }

    #[test]
    fn test_consul_protocol_remote() {
        let protocol = DiscoveryProtocol::Consul {
            address: "consul.example.com:8500".to_string(),
            datacenter: "us-east-1".to_string(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        };

        match protocol {
            DiscoveryProtocol::Consul { address, datacenter } => {
                assert!(address.contains("consul.example.com"));
                assert_eq!(datacenter, "us-east-1");
            }
            _ => panic!("Expected Consul protocol"),
        }
    }

    // ============================================================================
    // etcd Discovery Protocol Tests
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // ============================================================================

    #[test]
    fn test_etcd_protocol_creation() {
        let protocol = DiscoveryProtocol::Etcd {
            endpoints: vec!["http://localhost:2379".to_string()],
            key_prefix: "/services".to_string(),
            timeout_ms: 3000,
        };

        match protocol {
            DiscoveryProtocol::Etcd { endpoints, key_prefix, timeout_ms } => {
                assert_eq!(endpoints.len(), 1);
                assert_eq!(key_prefix, "/services");
                assert_eq!(timeout_ms, 3000);
            }
            _ => panic!("Expected etcd protocol"),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        }
    }

    #[test]
    fn test_etcd_protocol_cluster() {
        let protocol = DiscoveryProtocol::Etcd {
            endpoints: vec![
                "http://etcd1:2379".to_string(),
                "http://etcd2:2379".to_string(),
                "http://etcd3:2379".to_string(),
            ],
            key_prefix: "/beardog/services".to_string(),
            timeout_ms: 5000,
        };

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        match protocol {
            DiscoveryProtocol::Etcd { endpoints, .. } => {
                assert_eq!(endpoints.len(), 3);
            }
            _ => panic!("Expected etcd protocol"),
        }
    }

    // ============================================================================
    // Protocol Comparison Tests
    // ============================================================================

    #[test]
    fn test_protocol_equality() {
        let http1 = DiscoveryProtocol::Http {
            endpoint: "http://localhost:8080".to_string(),
            headers: HashMap::new(),
        };

        let http2 = DiscoveryProtocol::Http {
            endpoint: "http://localhost:8080".to_string(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            headers: HashMap::new(),
        };

        assert_eq!(http1, http2);
    }

    #[test]
    fn test_protocol_inequality() {
        let http = DiscoveryProtocol::Http {
            endpoint: "http://localhost:8080".to_string(),
            headers: HashMap::new(),
        };
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        let dns = DiscoveryProtocol::Dns {
            domain: "localhost".to_string(),
            servers: vec![],
        };

        assert_ne!(http, dns);
    }

    // ============================================================================
    // Hash Tests
    // ============================================================================

    #[test]
    fn test_protocol_hashable() {
        use std::collections::HashSet;
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        let protocols = vec![
            DiscoveryProtocol::Http {
                endpoint: "http://localhost:8080".to_string(),
                headers: HashMap::new(),
            },
            DiscoveryProtocol::Dns {
                domain: "example.com".to_string(),
                servers: vec!["8.8.8.8".to_string()],
            },
        ];

        let set: HashSet<_> = protocols.into_iter().collect();
        assert_eq!(set.len(), 2);
    }

    // ============================================================================
    // Serialization Round-Trip Tests
    // ============================================================================

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_all_protocols_serializable() {
        let protocols = vec![
            DiscoveryProtocol::Http {
                endpoint: "http://test".to_string(),
                headers: HashMap::new(),
            },
            DiscoveryProtocol::Dns {
                domain: "test.com".to_string(),
                servers: vec![],
            },
            DiscoveryProtocol::Mdns {
                service_type: "_test._tcp".to_string(),
                interface: "eth0".to_string(),
                timeout_ms: 1000,
                continuous_monitoring: false,
            },
            DiscoveryProtocol::Consul {
                address: "localhost:8500".to_string(),
                datacenter: "dc1".to_string(),
            },
            DiscoveryProtocol::Etcd {
                endpoints: vec!["http://localhost:2379".to_string()],
                key_prefix: "/test".to_string(),
                timeout_ms: 3000,
            },
        ];

        for protocol in protocols {
            let json = serde_json::to_string(&protocol).expect("Serialization failed");
            let deserialized: DiscoveryProtocol = serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(protocol, deserialized);
        }
    }
}

