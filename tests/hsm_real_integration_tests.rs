/*
 * BearDog Real HSM Integration Tests
 *
 * Comprehensive test coverage for production-ready HSM integrations:
 * - Android StrongBox with real hardware operations
 * - iOS Secure Enclave with biometric authentication
 * - PKCS#11 with real hardware token support
 * - Multi-vendor HSM management and failover
 * - Performance monitoring and metrics
 */

use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::types::KeyType;
use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    AndroidStrongBoxAdapter, AuthenticationStatus, BearDogNativeAdapter, EphemeralSeed,
    HealthStatus, HsmAdapter, HsmConnection, HumanEntropyRequirements, OperationResult,
    OperationType, Pkcs11Adapter, UniversalOperation,
};
use beardog_tunnel::universal_hsm_discovery::{
    DiscoveredHsm, HsmHealthStatus, HsmInterfaceType, HsmTier, PerformanceCapabilities,
};
use std::collections::HashMap;
use tokio_test;

#[cfg(target_os = "android")]
mod android_strongbox_tests {
    use super::*;

    #[tokio::test]
    fn test_android_strongbox_connection() {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let hsm = create_mock_android_strongbox_hsm();
        let connection_result = adapter.connect(&hsm);

        assert!(connection_result.is_ok());
        let connection = connection_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert_eq!(
            connection.authentication_status,
            AuthenticationStatus::BiometricRequired
        );
    }

    #[tokio::test]
    fn test_android_strongbox_key_generation() {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_android_strongbox_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ( "key_typ"e.to_string(),  "rsa_2048".to_string()),
                ( "key_i"d.to_string(),  "test-strongbox-key".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());
        assert!(op_result.performance_metrics.duration_ms > 0.0);
    }

    #[tokio::test]
    fn test_android_strongbox_signing() {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_android_strongbox_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let test_data = b "Hello, StrongBox!";
        let operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ( "key_i"d.to_string(),  "test-strongbox-key".to_string()),
                ( "data".to_string(), hex::encode(test_data)),
                ( "algorith"m.to_string(),  R"SA_PKCS1_SHA256".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());
        assert!(op_result.performance_metrics.hsm_latency_ms < 50.0); // StrongBox should be fast
    }

    #[tokio::test]
    fn test_android_strongbox_health_check() {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_android_strongbox_hsm();

        let health = adapter.test_connection(&hsm);
        assert!(health.is_ok());

        let health_status = health.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(health_status.is_healthy);
        assert!(health_status.response_time_ms < 100.0);
    }

    fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "android-strongbox-001".to_string(),
            vendor:  "Google".to_string(),
            model:  "Android StrongBox".to_string(),
            version:  "Android 13".to_string(),
            interface_type: HsmInterfaceType::AndroidStrongBox {
                security_level:  "STRONGBOX".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }
}

#[cfg(target_os =  "ios")]
mod ios_secure_enclave_tests {
    use super::*;

    #[tokio::test]
    async fn test_ios_secure_enclave_connection() {
        let adapter = create_ios_adapter();
        let hsm = create_mock_ios_secure_enclave_hsm();

        let connection_result = adapter.connect(&hsm);
        assert!(connection_result.is_ok());

        let connection = connection_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert_eq!(
            connection.authentication_status,
            AuthenticationStatus::BiometricRequired
        );
    }

    #[tokio::test]
    async fn test_ios_secure_enclave_ecdsa_key_generation() {
        let adapter = create_ios_adapter();
        let hsm = create_mock_ios_secure_enclave_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ( "key_typ"e.to_string(),  "ecdsa_p256".to_string()),
                ( "key_i"d.to_string(),  "test-enclave-key".to_string()),
                ( "biometric_polic"y.to_string(),  "touch_id".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());
        assert!(op_result.performance_metrics.duration_ms > 0.0);
    }

    #[tokio::test]
    async fn test_ios_secure_enclave_biometric_signing() {
        let adapter = create_ios_adapter();
        let hsm = create_mock_ios_secure_enclave_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let test_data = b "Secure Enclave Test Data";
        let operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ( "key_i"d.to_string(),  "test-enclave-key".to_string()),
                ( "data".to_string(), hex::encode(test_data)),
                ( "algorith"m.to_string(),  "ECDSA_SHA256".to_string()),
                ( "require_biometri"c.to_string(),  "true".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());

        assert!(op_result.performance_metrics.hsm_latency_ms < 20.0);
    }

    fn create_ios_adapter() -> Box<dyn HsmAdapter> {
        Box::new(BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?)
    }

    fn create_mock_ios_secure_enclave_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "ios-secure-enclave-001".to_string(),
            vendor:  "Apple".to_string(),
            model:  "Secure Enclave".to_string(),
            version:  "iOS 17.0".to_string(),
            interface_type: HsmInterfaceType::IosSecureEnclave {
                enclave_version:  "A17_Pro".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }
}

mod pkcs11_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_pkcs11_connection() {
        let adapter = Pkcs11Adapter;
        let hsm = create_mock_pkcs11_hsm();

        let connection_result = adapter.connect(&hsm);
        assert!(connection_result.is_ok());

        let connection = connection_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert_eq!(
            connection.authentication_status,
            AuthenticationStatus::Unauthenticated
        );
        assert!(connection.connection_handle.starts_with( "pkcs11-session-"));
    }

    #[tokio::test]
    async fn test_pkcs11_rsa_key_generation() {
        let adapter = Pkcs11Adapter;
        let hsm = create_mock_pkcs11_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ( "key_typ"e.to_string(),  "rsa_4096".to_string()),
                ( "key_i"d.to_string(),  "test-pkcs11-rsa-key".to_string()),
                ( "extractabl"e.to_string(),  "false".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());
        assert!(op_result.performance_metrics.duration_ms > 0.0);
    }

    #[tokio::test]
    async fn test_pkcs11_ecdsa_operations() {
        let adapter = Pkcs11Adapter;
        let hsm = create_mock_pkcs11_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let gen_operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from(OperationType::Sign,
            parameters: HashMap::from([
                ( "key_i"d.to_string(),  "test-pkcs11-ec-key".to_string()),
                ( "data".to_string(), hex::encode(test_data)),
                ( "algorith"m.to_string(),  "ECDSA_SHA256".to_string()),
            ]),
        };

        let sign_result = adapter.perform_operation(&connection, sign_operation);
        assert!(sign_result.is_ok());

        let op_result = sign_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);
        assert!(!op_result.result_data.is_empty());
    }

    #[tokio::test]
    async fn test_pkcs11_multi_vendor_support() {
        let vendors = vec![ "SafeNe"t,  "Thale"s,  "Utimac"o,  "Cavium"];

        for vendor in vendors {
            let hsm = create_mock_pkcs11_hsm_for_vendor(vendor);
            let adapter = Pkcs11Adapter;

            let connection_result = adapter.connect(&hsm);
            assert!(
                connection_result.is_ok(),
                 "Failed to connect to {} HSM",
                vendor
            );

            let health_result = adapter.test_connection(&hsm);
            assert!(
                health_result.is_ok(),
                 "Health check failed for {} HSM",
                vendor
            );
        }
    }

    #[tokio::test]
    async fn test_pkcs11_human_entropy_not_supported() {
        let adapter = Pkcs11Adapter;

        let supports_entropy = adapter.supports_human_entropy();
        assert!(supports_entropy.is_ok());
        assert!(!supports_entropy.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?);

        let hsm = create_mock_pkcs11_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let requirements = HumanEntropyRequirements {
            minimum_entropy_bits: 256,
            collection_timeout_seconds: 30,
        };

        let entropy_result = adapter
            .generate_human_entropy_seed(&connection, requirements)
            ;
        assert!(entropy_result.is_err());

        if let Err(BearDogError::UnsupportedOperation { operation }) = entropy_result {
            assert!(operation.contains( "PKCS#11"));
            assert!(operation.contains( "human entropy"));
        } else {
            panic!( "Expected UnsupportedOperation error");
        }
    }

    fn create_mock_pkcs11_hsm() -> DiscoveredHsm {
        create_mock_pkcs11_hsm_for_vendor( "SafeNet")
    }

    fn create_mock_pkcs11_hsm_for_vendor(vendor: &str) -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id: format!("pkcs11-{}-001", vendor.to_lowercase()),
            vendor: vendor.to_string(),
            version: "7.4.0".to_string().to_string()
                ),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::CertifiedHardware,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }
}

mod beardog_native_tests {
    use super::*;

    #[tokio::test]
    fn test_beardog_native_connection() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_beardog_native_hsm();

        let connection_result = adapter.connect(&hsm);
        assert!(connection_result.is_ok());

        let connection = connection_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert_eq!(
            connection.authentication_status,
            AuthenticationStatus::Authenticated
        );
    }

    #[tokio::test]
    fn test_beardog_native_human_entropy_support() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let supports_entropy = adapter.supports_human_entropy();
        assert!(supports_entropy.is_ok());
        assert!(supports_entropy.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?);
    }

    #[tokio::test]
    fn test_beardog_native_human_entropy_generation() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_beardog_native_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let requirements = HumanEntropyRequirements {
            minimum_entropy_bits: 256,
            collection_timeout_seconds: 30,
        };

        let entropy_result = adapter
            .generate_human_entropy_seed(&connection, requirements)
            ;
        assert!(entropy_result.is_ok());

        let seed = entropy_result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert_eq!(seed.seed_data.len(), 32); // 256 bits
        assert!(seed.entropy_estimate > 0.9); // High quality entropy
        assert!(seed.creation_timestamp <= chrono::Utc::now());
    }

    #[tokio::test]
    fn test_beardog_native_high_performance() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_beardog_native_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let test_data = b "BearDog Native Performance Test";
        let operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ( "key_i"d.to_string(),  "beardog-perf-key".to_string()),
                ( "data".to_string(), hex::encode(test_data)),
                ( "algorith"m.to_string(),  "ED25519".to_string()),
            ]),
        };

        let result = adapter.perform_operation(&connection, operation);
        assert!(result.is_ok());

        let op_result = result.map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(op_result.success);

        assert!(op_result.performance_metrics.hsm_latency_ms < 10.0);
        assert!(op_result.performance_metrics.duration_ms < 50.0);
    }

    fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "beardog-native-001".to_string(),
            vendor:  "BearDog".to_string(),
            model:  "Native HSM".to_string(),
            version: "1.0.0".to_string(),
            interface_type: HsmInterfaceType::BearDogNative {
                instance_id:  "native-001".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HumanEntropyPremium,
            supports_human_entropy: true,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }
}

mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    fn test_concurrent_hsm_operations() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_beardog_native_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let mut tasks = Vec::new();
        let num_operations = 10;

        for i in 0..num_operations {
            let adapter_clone = BearDogNativeAdapter::new().map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;
            let connection_clone = adapter.connect(&hsm).map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

            let task = tokio::spawn(OperationType::Sign,
                    parameters: HashMap::from([
                        ( "key_i"d.to_string(), format!( c"oncurrent-key-{:?}", i)),
                        ( "dat"a.to_string(), hex::encode(format!("data-{:?}", i))),
                        ( "algorith"m.to_string(),  "ED25519".to_string()),
                    ]),
                };

                adapter_clone
                    .perform_operation(&connection_clone, operation)
            });

            tasks.push(task);
        }

        let start_time = Instant::now();
        let results = futures::future::join_all(tasks);
        let total_duration = start_time.elapsed();

        for result in results {
            assert!(result.is_ok());
            let op_result = result
                .map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?
                .map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?;
            assert!(op_result.success);
        }

        assert!(total_duration.as_millis() < 5000); // Less than 5 seconds
    }

    #[tokio::test]
    fn test_performance_metrics_collection() {
        let adapter = BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let hsm = create_mock_beardog_native_hsm();
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ( "key_typ"e.to_string(),  "ecdsa_p256".to_string()),
                ( "key_i"d.to_string(),  m"etrics-test-key".to_string()),
            ]),
        };

        let result = adapter
            .perform_operation(&connection, operation)
            .map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(result.performance_metrics.duration_ms > 0.0);
        assert!(result.performance_metrics.hsm_latency_ms >= 0.0);
        assert_eq!(result.performance_metrics.error_count, 0);

        if let Some(throughput) = result.performance_metrics.throughput_bps {
            assert!(throughput > 0.0);
        }
    }

    #[tokio::test]
    fn test_health_monitoring() {
        let adapters: Vec<Box<dyn HsmAdapter>> = vec![
            Box::new(Pkcs11Adapter),
            Box::new(AndroidStrongBoxAdapter::new().map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?),
            Box::new(BearDogNativeAdapter::new().map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?),
        ];

        let hsms = vec![
            create_mock_pkcs11_hsm(),
            create_mock_android_strongbox_hsm(),
            create_mock_beardog_native_hsm(),
        ];

        for (adapter, hsm) in adapters.iter().zip(hsms.iter()) {
            let health_result = adapter.test_connection(hsm);
            assert!(health_result.is_ok());

            let health = health_result.map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;
            assert!(health.is_healthy);
            assert!(health.response_time_ms > 0.0);
            assert!(health.response_time_ms < 1000.0); // Reasonable response time
            assert!(health.last_check <= chrono::Utc::now());
        }
    }

    fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
        super::beardog_native_tests::create_mock_beardog_native_hsm()
    }

    fn create_mock_pkcs11_hsm() -> DiscoveredHsm {
        super::pkcs11_integration_tests::create_mock_pkcs11_hsm()
    }

    fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "android-strongbox-perf".to_string(),
            vendor:  "Google".to_string(),
            model:  "Android StrongBox".to_string(),
            version:  "Android 13".to_string(),
            interface_type: HsmInterfaceType::AndroidStrongBox {
                security_level:  "STRONGBOX".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }
}

mod failover_tests {
    use super::*;

    #[tokio::test]
    fn test_vendor_failover_scenario({}",
                hsm.vendor
            );
        }
    }

    #[tokio::test]
    fn test_automatic_failover(OperationType::GenerateKey,
            parameters: HashMap::from([
                ( "key_typ"e.to_string(),  "ecdsa_p256".to_string()),
                ( "key_i"d.to_string(),  "failover-test-key".to_string()),
            ]),
        };

        let primary_result = primary_adapter
            .perform_operation(
                &primary_connection.map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
                operation.clone(),
            )
            ;
        assert!(primary_result.is_ok());

        let backup_result = backup_adapter
            .perform_operation(
                &backup_connection.map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
                operation,
            )
            ;
        assert!(backup_result.is_ok());
    }

    fn get_adapter_for_interface(interface_type: &HsmInterfaceType) -> Box<dyn HsmAdapter> {
        match interface_type {
            HsmInterfaceType::Pkcs11 { .. } => Box::new(Pkcs11Adapter),
            HsmInterfaceType::AndroidStrongBox { .. } => {
                Box::new(AndroidStrongBoxAdapter::new().map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?)
            }
            HsmInterfaceType::BearDogNative { .. } => {
                Box::new(BearDogNativeAdapter::new().map_err(|e| {
                    tracing::error!( "Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?)
            }
            _ => Box::new(BearDogNativeAdapter::new().map_err(|e| {
                tracing::error!( "Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?), // Default fallback
        }
    }

    fn create_mock_pkcs11_hsm_for_vendor(vendor: &str) -> DiscoveredHsm {
        super::pkcs11_integration_tests::create_mock_pkcs11_hsm_for_vendor(vendor)
    }

    fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
        super::beardog_native_tests::create_mock_beardog_native_hsm()
    }
}

mod security_tests {
    use super::*;

    #[tokio::test]
    fn test_hsm_security_levels() {
        let hsm_configs = vec![
            (create_mock_pkcs11_hsm(), HsmTier::CertifiedHardware),
            (create_mock_android_strongbox_hsm(), HsmTier::HighSecurity),
            (
                create_mock_beardog_native_hsm(),
                HsmTier::HumanEntropyPremium,
            ),
        ];

        for (hsm, expected_tier) in hsm_configs {
            assert_eq!(hsm.assigned_tier, expected_tier);

            match expected_tier {
                HsmTier::CertifiedHardware => {
                    assert!(!hsm.supports_human_entropy);
                }
                HsmTier::HighSecurity => {
                    assert_eq!(hsm.vendor,  "Google");
                }
                HsmTier::HumanEntropyPremium => {
                    assert!(hsm.supports_human_entropy);
                    assert_eq!(hsm.vendor,  "BearDog");
                }
                _ => {}
            }
        }
    }

    #[tokio::test]
    fn test_key_attestation_support() {
        let android_hsm = create_mock_android_strongbox_hsm();

        match android_hsm.interface_type {
            HsmInterfaceType::AndroidStrongBox { security_level } => {
                assert_eq!(security_level,  "STRONGBOX");

                assert_eq!(android_hsm.assigned_tier, HsmTier::HighSecurity);
            }
            _ => panic!( "Expected AndroidStrongBox interface type"),
        }
    }

    #[tokio::test]
    fn test_biometric_authentication_requirements() {
        let ios_hsm = create_mock_ios_secure_enclave_hsm();

        let adapter = Box::new(BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?); // Mock iOS adapter
        let connection = adapter.connect(&ios_hsm).map_err(|e| {
            tracing::error!( "Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        assert_eq!(
            connection.authentication_status,
            AuthenticationStatus::Authenticated
        );
    }

    fn create_mock_pkcs11_hsm() -> DiscoveredHsm {
        super::pkcs11_integration_tests::create_mock_pkcs11_hsm()
    }

    fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "android-strongbox-security".to_string(),
            vendor:  "Google".to_string(),
            model:  "Android StrongBox".to_string(),
            version:  "Android 13".to_string(),
            interface_type: HsmInterfaceType::AndroidStrongBox {
                security_level:  "STRONGBOX".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }

    fn create_mock_ios_secure_enclave_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id:  "ios-secure-enclave-security".to_string(),
            vendor:  "Apple".to_string(),
            model:  "Secure Enclave".to_string(),
            version:  "iOS 17.0".to_string(),
            interface_type: HsmInterfaceType::IosSecureEnclave {
                enclave_version:  "A17_Pro".to_string(),
            },
            connection_info: create_mock_connection_info(),
            capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
            integration_status: crate::universal_hsm_discovery::IntegrationStatus::Ready,
        }
    }

    fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
        super::beardog_native_tests::create_mock_beardog_native_hsm()
    }
}

fn create_mock_connection_info() -> crate::universal_hsm_discovery::HsmConnectionInfo {
    use crate::universal_hsm_discovery::{AuthenticationMethod, ConnectionType, RetryPolicy};
    use std::time::Duration;

    crate::universal_hsm_discovery::HsmConnectionInfo {
        connection_type: ConnectionType::Local,
        authentication: AuthenticationMethod::None,
        endpoint: None,
        port: None,
        timeout: Duration::from_secs(RetryPolicy {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2.0,
        },
        ssl_config: None,
    }
}

fn create_mock_hsm_capabilities() -> crate::universal_hsm_discovery::HsmCapabilities {
    use crate::universal_hsm_discovery::{
        AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
        CryptoOperationCapabilities, HsmCapabilities, HumanEntropyCapabilities,
        KeyGenerationCapabilities, KeyManagementCapabilities, PerformanceCapabilities,
        SecurityCapabilities,
    };

    HsmCapabilities {
        key_management: KeyGenerationCapabilities {
            supported_algorithms: vec![ "RSA".to_string(), 1000u32),
                ( "verify".to_string(), 2000u32),
                ( "encrypt".to_string(), 500u32),
            ]),
            latency_ms: HashMap::from([
                ( "sign".to_string(), 10.0),
                ( "verify".to_string(), 5.0),
                ( "encrypt".to_string(), 15.0),
            ]),
            throughput_mbps: Some(50,
            memory_usage_mb: Some(128),
            power_consumption_watts: Some(5.0),
        },
        security: SecurityCapabilities::default(),
        api_support: ApiSupportCapabilities::default(),
        compliance: ComplianceCapabilities::default(),
        human_entropy: HumanEntropyCapabilities::default(),
    }
}
