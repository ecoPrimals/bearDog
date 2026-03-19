// SPDX-License-Identifier: AGPL-3.0-only

//! # E2E Cross-Primal Integration Tests
//!
//! End-to-end tests for cross-primal secure messaging.
//! Tests the complete workflow from discovery to secure communication.

use crate::ecosystem_integration::{PrimalDiscoveryService, SecureCrossPrimalMessenger};
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, SecurityService, StorageCharacteristic,
    UniversalCapabilityType, UniversalServiceDescriptor,
};
use std::sync::Arc;

/// Simplified mock primal discovery service for E2E testing
#[derive(Debug, Clone)]
struct MockPrimalEcosystem {
    mock_primals: Vec<UniversalServiceDescriptor>,
}

impl MockPrimalEcosystem {
    fn new() -> Self {
        // Create simplified mock primals
        let network_primal = UniversalServiceDescriptor {
            service_id: "network-primal-001".to_string(),
            capabilities: vec![UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::TrafficRouting],
            }],
            endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                protocol: "http".to_string(),
                host: "network.local".to_string(),
                port: 8080,
                path: Some("/api/network".to_string()),
                parameters: std::collections::HashMap::new(),
            },
            auth_method: beardog_types::canonical::discovery::AuthenticationMethod::ApiKey {
                key_location: "network-key".to_string(),
            },
            performance_profile: Default::default(),
            trust_score: 0.95,
        };

        let security_primal = UniversalServiceDescriptor {
            service_id: "security-primal-002".to_string(),
            capabilities: vec![UniversalCapabilityType::Security {
                services: vec![SecurityService::KeyManagement],
            }],
            endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                protocol: "https".to_string(),
                host: "security.local".to_string(),
                port: 8443,
                path: Some("/api/security".to_string()),
                parameters: std::collections::HashMap::new(),
            },
            auth_method: beardog_types::canonical::discovery::AuthenticationMethod::MutualTls {
                cert_path: "security-cert.pem".to_string(),
                key_path: "security-key.pem".to_string(),
            },
            performance_profile: Default::default(),
            trust_score: 0.98,
        };

        let compute_primal = UniversalServiceDescriptor {
            service_id: "compute-primal-003".to_string(),
            capabilities: vec![UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            }],
            endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                protocol: "grpc".to_string(),
                host: "compute.local".to_string(),
                port: 50051,
                path: Some("/compute".to_string()),
                parameters: std::collections::HashMap::new(),
            },
            auth_method: beardog_types::canonical::discovery::AuthenticationMethod::ApiKey {
                key_location: "compute-key".to_string(),
            },
            performance_profile: Default::default(),
            trust_score: 0.95,
        };

        let storage_primal = UniversalServiceDescriptor {
            service_id: "storage-primal-004".to_string(),
            capabilities: vec![UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::Encrypted],
            }],
            endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                protocol: "https".to_string(),
                host: "storage.local".to_string(),
                port: 9000,
                path: Some("/storage".to_string()),
                parameters: std::collections::HashMap::new(),
            },
            auth_method: beardog_types::canonical::discovery::AuthenticationMethod::MutualTls {
                cert_path: "storage-cert.pem".to_string(),
                key_path: "storage-key.pem".to_string(),
            },
            performance_profile: Default::default(),
            trust_score: 0.99,
        };

        Self {
            mock_primals: vec![
                network_primal,
                security_primal,
                compute_primal,
                storage_primal,
            ],
        }
    }
}

#[async_trait::async_trait]
impl PrimalDiscoveryService for MockPrimalEcosystem {
    async fn discover_by_capability(
        &self,
        capability: UniversalCapabilityType,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        // Filter primals by capability type
        let matching_primals: Vec<UniversalServiceDescriptor> =
            self.mock_primals
                .iter()
                .filter(|primal| {
                    primal.capabilities.iter().any(|cap| {
                        std::mem::discriminant(cap) == std::mem::discriminant(&capability)
                    })
                })
                .cloned()
                .collect();

        Ok(matching_primals)
    }

    async fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        // Simulate successful request with proper SecurePrimalResponse format
        Ok(serde_json::json!({
            "ciphertext": vec![1u8, 2, 3, 4],
            "responder_id": service.service_id,
            "capability_used": serde_json::json!({"Network": {"functions": []}}),
            "processing_time_ms": 10,
        }))
    }
}

#[cfg(test)]
mod e2e_tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_e2e_messenger_creation() {
        // Test: Create messenger with mock ecosystem
        let ecosystem = Arc::new(MockPrimalEcosystem::new());
        let result = SecureCrossPrimalMessenger::new(ecosystem);

        assert!(result.is_ok(), "Messenger creation should succeed");
    }

    #[tokio::test]
    async fn test_e2e_discover_network_primal() {
        // Test: Discover network-capable primals
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        let network_cap = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };

        let primals = ecosystem.discover_by_capability(network_cap).await.unwrap();

        assert_eq!(primals.len(), 1, "Should discover 1 network primal");
        assert_eq!(primals[0].service_id, "network-primal-001");
    }

    #[tokio::test]
    async fn test_e2e_discover_security_primal() {
        // Test: Discover security-capable primals
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        let security_cap = UniversalCapabilityType::Security {
            services: vec![SecurityService::KeyManagement],
        };

        let primals = ecosystem
            .discover_by_capability(security_cap)
            .await
            .unwrap();

        assert_eq!(primals.len(), 1, "Should discover 1 security primal");
        assert_eq!(primals[0].service_id, "security-primal-002");
    }

    #[tokio::test]
    async fn test_e2e_send_to_network_primal() {
        // Test: Send message to network primal
        let ecosystem = Arc::new(MockPrimalEcosystem::new());
        let messenger = SecureCrossPrimalMessenger::new(ecosystem.clone()).unwrap();

        let payload = b"test data";
        let security_context = HashMap::new();

        let result = messenger
            .send_to_network_primal(payload, security_context)
            .await;

        assert!(
            result.is_ok(),
            "Should successfully send to network primal: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_e2e_send_to_compute_primal() {
        // Test: Send computation request
        let ecosystem = Arc::new(MockPrimalEcosystem::new());
        let messenger = SecureCrossPrimalMessenger::new(ecosystem.clone()).unwrap();

        let computation = serde_json::json!({"algorithm": "test"});
        let result = messenger.send_to_compute_primal(computation).await;

        assert!(
            result.is_ok(),
            "Should successfully send to compute primal: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_e2e_send_to_storage_primal() {
        // Test: Send data to storage
        let ecosystem = Arc::new(MockPrimalEcosystem::new());
        let messenger = SecureCrossPrimalMessenger::new(ecosystem.clone()).unwrap();

        let data = vec![0u8; 100];
        let result = messenger.send_to_storage_primal(&data).await;

        assert!(
            result.is_ok(),
            "Should successfully send to storage primal: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_e2e_no_hardcoded_names() {
        // Test: Verify discovery is capability-based, not name-based
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        let network_cap = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };

        let primals = ecosystem.discover_by_capability(network_cap).await.unwrap();

        // Should discover based on capability, not hardcoded name
        assert!(
            primals.iter().any(|p| p
                .capabilities
                .iter()
                .any(|c| matches!(c, UniversalCapabilityType::Network { .. }))),
            "Should discover by capability"
        );
    }

    #[tokio::test]
    async fn test_e2e_ecosystem_health() {
        // Test: Ecosystem has expected primals
        let ecosystem = MockPrimalEcosystem::new();

        assert_eq!(
            ecosystem.mock_primals.len(),
            4,
            "Ecosystem should have 4 mock primals (network, security, compute, storage)"
        );

        // Verify trust scores
        for primal in &ecosystem.mock_primals {
            assert!(
                primal.trust_score > 0.0 && primal.trust_score <= 1.0,
                "Trust score should be in valid range"
            );
        }
    }

    #[tokio::test]
    async fn test_e2e_concurrent_discovery() {
        // Test: Concurrent discovery operations
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let eco = ecosystem.clone();
                tokio::spawn(async move {
                    let cap = UniversalCapabilityType::Network {
                        functions: vec![NetworkFunction::TrafficRouting],
                    };
                    eco.discover_by_capability(cap).await
                })
            })
            .collect();

        for handle in handles {
            let result = handle.await.expect("Task should complete").unwrap();
            assert!(!result.is_empty(), "Should discover primals concurrently");
        }
    }

    #[tokio::test]
    async fn test_e2e_trust_scores() {
        // Test: Primals have appropriate trust scores
        let ecosystem = MockPrimalEcosystem::new();

        let security_primal = ecosystem
            .mock_primals
            .iter()
            .find(|p| p.service_id == "security-primal-002")
            .expect("Security primal should exist");

        assert!(
            security_primal.trust_score >= 0.95,
            "Security primal should have high trust score"
        );
    }

    #[tokio::test]
    async fn test_e2e_capability_filtering() {
        // Test: Capability filtering works correctly
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        let security_cap = UniversalCapabilityType::Security {
            services: vec![SecurityService::KeyManagement],
        };

        let security_primals = ecosystem
            .discover_by_capability(security_cap)
            .await
            .unwrap();

        assert_eq!(security_primals.len(), 1);
        assert!(security_primals[0]
            .capabilities
            .iter()
            .any(|c| matches!(c, UniversalCapabilityType::Security { .. })));
    }

    #[tokio::test]
    async fn test_e2e_discover_nonexistent_capability() {
        // Test: Discovering non-existent capability returns empty
        let ecosystem = Arc::new(MockPrimalEcosystem::new());

        // Use a capability variant that no primal has
        let rare_cap = UniversalCapabilityType::Orchestration { features: vec![] };

        let primals = ecosystem.discover_by_capability(rare_cap).await.unwrap();

        assert_eq!(
            primals.len(),
            0,
            "Should return empty for non-existent capability"
        );
    }
}
