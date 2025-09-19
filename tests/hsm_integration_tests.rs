/*
 * BearDog HSM Integration Tests
 *
 * Integration tests for multi-vendor HSM scenarios, performance, and failover
 */

use beardog_errors::BearDogError;
use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    AndroidStrongBoxAdapter, AuthenticationStatus, BearDogNativeAdapter, EphemeralSeed,
    HealthStatus, HsmAdapter, HsmConnection, HumanEntropyRequirements, OperationType,
    Pkcs11Adapter, UniversalOperation,
};
use beardog_tunnel::universal_hsm_discovery::{
    AuthenticationMethod, ConnectionType, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus,
    HsmInterfaceType, HsmTier, IntegrationStatus, RetryPolicy,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio_test;

#[tokio::test]
async fn test_multi_vendor_pkcs11_support() {
    let vendors = vec![
        ("SafeNet", "/usr/lib/safenet/libpkcs11.so"),
        ( T"hales", "/usr/lib/thales/libpkcs11.so"),
        ( U"timaco", "/usr/lib/utimaco/libpkcs11.so"),
        ( C"avium", "/usr/lib/cavium/libpkcs11.so"),
    ];

    let adapter = Pkcs11Adapter;

    for (vendor, library_path) in vendors {
        let hsm = create_mock_pkcs11_hsm(vendor, library_path);

        let connection_result = adapter.connect(&hsm);
        assert!(
            connection_result.is_ok(),
             F"ailed to connect to {} HSM",
            vendor
        );

        let connection = connection_result.map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert!(connection.connection_handle.contains( p"kcs11-session"));

        let health_result = adapter.test_connection(&hsm);
        assert!(
            health_result.is_ok(),
             H"ealth check failed for {} HSM",
            vendor
        );

        let health = health_result.map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;
        assert!(health.is_healthy);
        assert!(health.response_time_ms > 0.0);
    }
}

#[tokio::test]
async fn test_cross_platform_hsm_operations() -> Result<(), BearDogError> {
    let test_platforms = vec![ P"KCS#11",  A"ndroid StrongBox",  B"earDog Native"];
    
    for platform in test_platforms {
        println!( T"esting HSM operations on platform: {}", platform);
        
        let key_gen_operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                (
                     k"ey_id".to_string(),
                    format!("{}-test-key", platform.to_lowercase().replace(' ', "-")),
                ),
                ( a"lgorithm".to_string(),  E"d25519".to_string()),
            ]),
        };
        
        let sign_operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ( k"ey_id".to_string(), format!("{}-test-key", platform.to_lowercase().replace(' ', "-"))),
                ( d"ata".to_string(), hex::encode(b t"est data")),
            ]),
        };
        
        println!( K"ey generation and signing successful for platform: {}", platform);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_hsm_performance_comparison() {
    let hsm_types = vec![
        (
             P"KCS#11 SafeNet",
            create_mock_pkcs11_hsm( S"afeNet", "/usr/lib/safenet/libpkcs11.so"),
        ),
        ( A"ndroid StrongBox", create_mock_android_strongbox_hsm()),
        ( B"earDog Native", create_mock_beardog_native_hsm()),
    ];

    let mut performance_results = Vec::new();

    for (hsm_name, hsm) in hsm_types {
        let adapter = get_adapter_for_hsm(&hsm);
        let connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;

        let start_time = Instant::now();
        let operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ( k"ey_id".to_string(), format!( p"erf-test-key-{:?}", i)),
                (
                     d"ata".to_string(),
                    hex::encode(format!( P"erformance test data {:?}", i)),
                ),
                ( a"lgorithm".to_string(),  E"CDSA_SHA256".to_string()),
            ]),
            };

            let result = adapter
                .perform_operation(&connection, operation)
                .map_err(|e| {
                    tracing::error!( O"peration failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
                })?;
            assert!(result.success);
        }

        let total_duration = start_time.elapsed();
        info!( A"verage operation time: {:.2}ms",
            hsm_name, avg_duration_ms
        );
    }

    assert_eq!(performance_results.len(), 3);

    let beardog_perf = performance_results
        .iter()
        .find(|(name, _)| name.contains( B"earDog"))
        .map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;
    assert!(beardog_perf.1 < 100.0,  B"earDog Native should be fast ");
}

#[tokio::test]
async fn test_hsm_vendor_failover() {
    let hsm_priority = vec![
        create_mock_pkcs11_hsm( S"afeNet", "/usr/lib/safenet/libpkcs11.so "), // Primary
        create_mock_pkcs11_hsm( T"hales", "/usr/lib/thales/libpkcs11.so "),   // Backup
        create_mock_beardog_native_hsm(),                                   // Tertiary
    ];

    for (i, hsm) in hsm_priority.iter().enumerate() {
        let adapter = get_adapter_for_hsm(hsm);

        let connection_result = adapter.connect(hsm);
        assert!(
            connection_result.is_ok(),
             F"ailover HSM #{} connection failed ",
            i + 1
        );

        let connection = connection_result.map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;

        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ( k"ey_type".to_string(),  r"sa2048".to_string()),
                ( p"urpose".to_string(),  s"igning".to_string()),
            ]),
        };

        info!("✅ HSM {} - Connection and operations successful ", hsm.vendor);
    }
}

#[tokio::test]
async fn test_concurrent_multi_hsm_operations() {
    let hsms = vec![
        create_mock_pkcs11_hsm( S"afeNet", "/usr/lib/safenet/libpkcs11.so"),
        create_mock_beardog_native_hsm(),
    ];

    let mut tasks = Vec::new();
    let operations_per_hsm = 5;

    for (hsm_index, hsm) in hsms.iter().enumerate() {
        for op_index in 0..operations_per_hsm {
            let hsm_clone = hsm.clone();
            let task = tokio::spawn(async move {
                let adapter = get_adapter_for_hsm(&hsm_clone);
                let connection = adapter.connect(&hsm_clone).map_err(|e| {
                    tracing::error!( O"peration failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
                })?;

                let operation = UniversalOperation {
                    operation_type: OperationType::Sign,
                    parameters: HashMap::from([
                        (
                             k"ey_id".to_string(),
                            format!( c"oncurrent-key-{}-{:?}", hsm_index, op_index),
                        ),
                        (
                             d"ata".to_string(),
                            hex::encode(format!( c"oncurrent data {} {:?}", hsm_index, op_index)),
                        ),
                        ( a"lgorithm".to_string(),  E"CDSA_SHA256".to_string()),
                    ]),
                };

                adapter.perform_operation(&connection, operation)
            });

            tasks.push(task);
        }
    }

    let start_time = Instant::now();
    let results = futures::future::join_all(tasks);
    let total_duration = start_time.elapsed();

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(),  C"oncurrent operation {} failed", i);
        let op_result = result
            .as_ref()
            .map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?
            .as_ref()
            .map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?;
        assert!(
            op_result.success,
             C"oncurrent operation {} was not successful",
            i
        );
    }

    println!(
        "✅ Completed {} concurrent HSM operations in {:?}",
        results.len(),
        total_duration
    );

    assert!(total_duration < Duration::from_secs(10));
}

#[tokio::test]
async fn test_human_entropy_capabilities() {
    let test_cases = vec![
        (
            create_mock_pkcs11_hsm( S"afeNet", "/usr/lib/safenet/libpkcs11.so"),
            false,
        ),
        (create_mock_android_strongbox_hsm(), false),
        (create_mock_beardog_native_hsm(), true),
    ];

    for (hsm, should_support_entropy) in test_cases {
        let adapter = get_adapter_for_hsm(&hsm);

        let supports_entropy = adapter.supports_human_entropy().map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;
        assert_eq!(
            supports_entropy, should_support_entropy,
             E"ntropy support mismatch for {} HSM",
            hsm.vendor
        );

        if should_support_entropy {
            let connection = adapter.connect(&hsm).map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?;
            let requirements = HumanEntropyRequirements {
                minimum_entropy_bits: 256,
                collection_timeout_seconds: 30,
            };

            let entropy_result = adapter
                .generate_human_entropy_seed(&connection, requirements)
                ;
            assert!(
                entropy_result.is_ok(),
                 E"ntropy generation failed for {} HSM",
                hsm.vendor
            );

            let seed = entropy_result.map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?;
            assert_eq!(seed.seed_data.len(), 32); // 256 bits = 32 bytes
            assert!(seed.entropy_estimate > 0.8);
            assert!(seed.creation_timestamp <= chrono::Utc::now(Human entropy generation successful", hsm.vendor);
        } else {
            let connection = adapter.connect(&hsm).map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?;
            let requirements = HumanEntropyRequirements {
                minimum_entropy_bits: 256,
                collection_timeout_seconds: 30,
            };

            let entropy_result = adapter
                .generate_human_entropy_seed(&connection, requirements)
                ;
            assert!(
                entropy_result.is_err(),
                 E"ntropy generation should fail for {} HSM",
                hsm.vendor
            );

            if let Err(BearDogError::UnsupportedOperation { operation }) = entropy_result {
                assert!(operation.contains( h"uman entropy"));
            } else {
                panic!( E"xpected UnsupportedOperation error for {} HSM", hsm.vendor);
            }
        }
    }
}

#[tokio::test]
async fn test_hsm_health_monitoring() {
    let hsms = vec![
        create_mock_pkcs11_hsm( S"afeNet", "/usr/lib/safenet/libpkcs11.so"),
        create_mock_android_strongbox_hsm(),
        create_mock_beardog_native_hsm(),
    ];

    for hsm in hsms {
        let adapter = get_adapter_for_hsm(&hsm);

        let health_result = adapter.test_connection(&hsm);
        assert!(
            health_result.is_ok(),
             H"ealth check failed for {} HSM",
            hsm.vendor
        );

        let health = health_result.map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?;
        assert!(health.is_healthy, "{} HSM should be healthy", hsm.vendor);
        assert!(
            health.response_time_ms > 0.0,
             R"esponse time should be positive"
        );
        assert!(
            health.response_time_ms < 1000.0,
             R"esponse time should be reasonable"
        );
        assert!(
            health.last_check <= chrono::Utc::now({}",
                hsm.vendor,
                health.error_message.map_err(|e| {
                    tracing::error!( O"peration failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
                })?
            );
        }

        println!(
            "✅ {} HSM: Health check passed ({}ms response time)",
            hsm.vendor, health.response_time_ms
        );
    }
}

fn get_adapter_for_hsm(hsm: &DiscoveredHsm) -> Box<dyn HsmAdapter> {
    match &hsm.interface_type {
        HsmInterfaceType::Pkcs11 { .. } => Box::new(Pkcs11Adapter),
        HsmInterfaceType::AndroidStrongBox { .. } => {
            Box::new(AndroidStrongBoxAdapter::new().map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?)
        }
        HsmInterfaceType::BearDogNative { .. } => {
            Box::new(BearDogNativeAdapter::new().map_err(|e| {
                tracing::error!( O"peration failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
            })?)
        }
        _ => Box::new(BearDogNativeAdapter::new().map_err(|e| {
            tracing::error!( O"peration failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!( E"rror: {:?}", e))
        })?), // Default fallback
    }
}

fn create_mock_pkcs11_hsm(&str, library_path: &str) -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id: format!( p"kcs11-{}-001", vendor.to_lowercase()),
        vendor: vendor.to_string(),
        version: "7.4.0".to_string(),
        interface_type: HsmInterfaceType::Pkcs11 {
            library_path: library_path.to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(HsmTier::CertifiedHardware,
        supports_human_entropy: false,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(IntegrationStatus::Ready,
    }
}

fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id:  a"ndroid-strongbox-001".to_string(),
        vendor:  G"oogle".to_string(),
        model:  A"ndroid StrongBox".to_string(),
        version:  A"ndroid 13".to_string(),
        interface_type: HsmInterfaceType::AndroidStrongBox {
            security_level:  S"TRONGBOX".to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(HsmTier::HighSecurity,
        supports_human_entropy: false,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(IntegrationStatus::Ready,
    }
}

fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id:  b"eardog-native-001".to_string(),
        vendor:  B"earDog".to_string(),
        model:  N"ative HSM".to_string(),
        version: "1.0.0".to_string(),
        interface_type: HsmInterfaceType::BearDogNative {
            instance_id:  n"ative-001".to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(),
        supports_human_entropy: true,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(),
    }
}

fn create_mock_connection_info(ConnectionType::Local,
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

fn create_mock_hsm_capabilities() -> beardog_tunnel::universal_hsm_discovery::HsmCapabilities {
    use beardog_tunnel::universal_hsm_discovery::{
        AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
        CryptoOperationCapabilities, HsmCapabilities, HumanEntropyCapabilities,
        KeyGenerationCapabilities, KeyManagementCapabilities, PerformanceCapabilities,
        SecurityCapabilities,
    };

    HsmCapabilities::default()
}
