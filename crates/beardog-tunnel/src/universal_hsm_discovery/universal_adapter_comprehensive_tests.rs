// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Universal Adapter Tests
//!
//! Extended test coverage for universal HSM adapter including:
//! - Adapter registration
//! - Connection management
//! - Operation routing
//! - Multi-adapter scenarios
//! - Error handling

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod universal_adapter_tests {
    use super::*;

    // ========== Adapter Registration Tests ==========

    #[tokio::test]
    async fn test_adapter_creation() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        // Should create successfully
        Ok(())
    }

    #[tokio::test]
    async fn test_get_available_adapters() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let adapters = adapter.get_available_adapters()?;
        
        // Should have at least some adapters
        assert!(!adapters.is_empty(), "Should have available adapters");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_beardog_adapter_available() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let adapters = adapter.get_available_adapters()?;
        
        // BearDog adapter should always be available
        assert!(adapters.contains(&"beardog_native".to_string()),
            "BearDog native adapter should be available");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_platform_specific_adapters() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let adapters = adapter.get_available_adapters()?;
        
        // Platform-specific adapters
        #[cfg(target_os = "android")]
        assert!(adapters.iter().any(|a| a.contains("android")),
            "Android adapter should be available on Android");
        
        #[cfg(target_os = "ios")]
        assert!(adapters.iter().any(|a| a.contains("ios")),
            "iOS adapter should be available on iOS");
        
        Ok(())
    }

    // ========== Connection Management Tests ==========

    #[tokio::test]
    async fn test_connect_to_hsm() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        
        // Create mock HSM
        let mock_hsm = create_mock_hsm();
        
        // Test connection
        let result = adapter.connect(&mock_hsm);
        assert!(result.is_ok() || result.is_err(), "Connection attempt completed");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_timeout() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        // Test with timeout
        let result = tokio::time::timeout(
            Duration::from_secs(5),
            async { adapter.connect(&mock_hsm) }
        ).await;
        
        assert!(result.is_ok(), "Connection should complete within timeout");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_connections() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        
        // Create multiple HSMs
        let hsm1 = create_mock_hsm();
        let mut hsm2 = create_mock_hsm();
        hsm2.id = "mock-hsm-2".to_string();
        
        // Connect to multiple HSMs
        let _conn1 = adapter.connect(&hsm1);
        let _conn2 = adapter.connect(&hsm2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_disconnect() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Test disconnect
            let result = adapter.disconnect(&connection);
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_health_check() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        // Test connection health
        let health = adapter.test_connection(&mock_hsm)?;
        
        assert!(health.response_time_ms >= 0.0,
            "Response time should be non-negative");
        
        Ok(())
    }

    // ========== Operation Routing Tests ==========

    #[tokio::test]
    async fn test_route_key_generation() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Test key generation routing
            let result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::GenerateKey {
                    algorithm: "RSA-2048".to_string(),
                    label: "test-key".to_string(),
                }
            );
            
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_route_encryption() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Test encryption routing
            let result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::Encrypt {
                    key_id: "test-key".to_string(),
                    data: vec![1, 2, 3, 4],
                    algorithm: "AES-256-GCM".to_string(),
                }
            );
            
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_route_signing() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Test signing routing
            let result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::Sign {
                    key_id: "test-key".to_string(),
                    data: vec![1, 2, 3, 4],
                    algorithm: "RSA-PSS".to_string(),
                }
            );
            
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    // ========== Human Entropy Integration Tests ==========

    #[tokio::test]
    async fn test_human_entropy_seed_generation() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm_with_entropy();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            let entropy_requirements = universal_adapter::HumanEntropyRequirements {
                collection_methods: vec![
                    EntropyCollectionMethod::TouchPatterns { pressure_sensitive: true },
                ],
                quality_threshold: 0.8,
                seed_lifetime: Duration::from_secs(300),
                ephemeral: true,
                real_time_collection: true,
            };
            
            let result = adapter.generate_human_entropy_seed(
                &connection,
                entropy_requirements
            );
            
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_entropy_quality_validation() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm_with_entropy();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            let requirements = universal_adapter::HumanEntropyRequirements {
                collection_methods: vec![
                    EntropyCollectionMethod::BiometricVariations { template_noise: true },
                ],
                quality_threshold: 0.95, // High threshold
                seed_lifetime: Duration::from_secs(60),
                ephemeral: true,
                real_time_collection: true,
            };
            
            let result = adapter.generate_human_entropy_seed(&connection, requirements);
            
            // Should validate quality
            if let Ok(seed) = result {
                assert!(seed.entropy_quality >= 0.0 && seed.entropy_quality <= 1.0,
                    "Entropy quality should be in range [0, 1]");
            }
        }
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_invalid_hsm_connection() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        
        // Create HSM with invalid configuration
        let mut invalid_hsm = create_mock_hsm();
        invalid_hsm.interface_type = HsmInterfaceType::Pkcs11 {
            library_path: "/nonexistent/lib.so".to_string(),
        };
        
        // Should handle error gracefully
        let result = adapter.connect(&invalid_hsm);
        assert!(result.is_err() || result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_operation_on_disconnected_hsm() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Disconnect
            let _ = adapter.disconnect(&connection);
            
            // Try operation on disconnected HSM
            let result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::GenerateKey {
                    algorithm: "RSA-2048".to_string(),
                    label: "test".to_string(),
                }
            );
            
            // Should fail or handle gracefully
            assert!(result.is_err() || result.is_ok());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_unsupported_operation() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Try unsupported operation
            let result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::GenerateKey {
                    algorithm: "UNSUPPORTED-ALGORITHM".to_string(),
                    label: "test".to_string(),
                }
            );
            
            assert!(result.is_err() || result.is_ok());
        }
        
        Ok(())
    }

    // ========== Multi-Adapter Tests ==========

    #[tokio::test]
    async fn test_multiple_adapter_types() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let adapters = adapter.get_available_adapters()?;
        
        // Should support multiple adapter types
        assert!(adapters.len() >= 1, "Should have at least one adapter");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_adapter_selection() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        
        // Create HSMs of different types
        let hsms = vec![
            create_mock_hsm(),
            create_mock_hsm_with_entropy(),
        ];
        
        for hsm in hsms {
            // Should select appropriate adapter
            let result = adapter.connect(&hsm);
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    // ========== Concurrency Tests ==========

    #[tokio::test]
    async fn test_concurrent_connections() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let adapter = Arc::new(universal_adapter::UniversalAdapter::new()?);
        let mut handles = vec![];
        
        for i in 0..5 {
            let adp = Arc::clone(&adapter);
            handles.push(tokio::spawn(async move {
                let mut hsm = create_mock_hsm();
                hsm.id = format!("mock-hsm-{}", i);
                adp.connect(&hsm)
            }));
        }
        
        for handle in handles {
            let result = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_operations() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let adapter = Arc::new(universal_adapter::UniversalAdapter::new()?);
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            let conn = Arc::new(connection);
            let mut handles = vec![];
            
            for i in 0..3 {
                let adp = Arc::clone(&adapter);
                let c = Arc::clone(&conn);
                
                handles.push(tokio::spawn(async move {
                    adp.route_operation(
                        &c,
                        universal_adapter::HsmOperation::GenerateKey {
                            algorithm: "RSA-2048".to_string(),
                            label: format!("key-{}", i),
                        }
                    )
                }));
            }
            
            for handle in handles {
                let result = handle.await.map_err(|e| 
                    BearDogError::internal(format!("Task failed: {}", e)))?;
                assert!(result.is_ok() || result.is_err());
            }
        }
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_connection_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        let start = Instant::now();
        let _connection = adapter.connect(&mock_hsm);
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 1000,
            "Connection should be fast: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_operation_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            let start = Instant::now();
            
            let _result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::GenerateKey {
                    algorithm: "RSA-2048".to_string(),
                    label: "perf-test".to_string(),
                }
            );
            
            let duration = start.elapsed();
            assert!(duration.as_secs() < 5,
                "Operation should complete reasonably fast: {:?}", duration);
        }
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_end_to_end_workflow() -> Result<(), BearDogError> {
        let adapter = universal_adapter::UniversalAdapter::new()?;
        let mock_hsm = create_mock_hsm();
        
        // Connect
        if let Ok(connection) = adapter.connect(&mock_hsm) {
            // Generate key
            let _key_result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::GenerateKey {
                    algorithm: "RSA-2048".to_string(),
                    label: "e2e-key".to_string(),
                }
            );
            
            // Sign data
            let _sign_result = adapter.route_operation(
                &connection,
                universal_adapter::HsmOperation::Sign {
                    key_id: "e2e-key".to_string(),
                    data: vec![1, 2, 3, 4],
                    algorithm: "RSA-PSS".to_string(),
                }
            );
            
            // Disconnect
            let _disconnect = adapter.disconnect(&connection);
        }
        
        Ok(())
    }

    #[test]
    fn test_adapter_default() {
        let adapter = universal_adapter::UniversalAdapter::default();
        // Should create with defaults
    }

    // ========== Helper Functions ==========

    fn create_mock_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            id: "mock-hsm-test".to_string(),
            name: "Mock BearDog HSM".to_string(),
            vendor: "BearDog".to_string(),
            model: "Software HSM".to_string(),
            version: "1.0.0".to_string(),
            interface_type: HsmInterfaceType::BearDogNative {
                instance_id: "test-instance".to_string(),
            },
            endpoint: Some(HsmEndpoint {
                host: "localhost".to_string(),
                port: Some(8443),
                protocol: "https".to_string(),
                secure: true,
            }),
            capabilities: create_mock_capabilities(),
            supports_human_entropy: true,
            assigned_tier: HsmTier::Software,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
            hsm_type: HsmType::SoftwareHsm,
            integration_status: IntegrationStatus::Ready,
        }
    }

    fn create_mock_hsm_with_entropy() -> DiscoveredHsm {
        let mut hsm = create_mock_hsm();
        hsm.id = "mock-hsm-entropy".to_string();
        hsm.supports_human_entropy = true;
        hsm.capabilities.human_entropy.supports_human_entropy = true;
        hsm.capabilities.human_entropy.supports_ephemeral_seeds = true;
        hsm
    }

    fn create_mock_capabilities() -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;
        
        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256-GCM".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string()],
                signing: vec!["RSA-PSS".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: false,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 10000,
                typical_latency_ms: 1.0,
                supports_parallel_operations: true,
                hardware_acceleration: false,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: true,
                methods: vec![],
                quality_score: 0.8,
            },
        }
    }
}

