/*
 * BearDog HSM Integration Tests
 * 
 * Integration tests for multi-vendor HSM scenarios, performance, and failover
 */

use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    HsmAdapter, Pkcs11Adapter, AndroidStrongBoxAdapter, BearDogNativeAdapter,
    HsmConnection, UniversalOperation, OperationType, AuthenticationStatus,
    HumanEntropyRequirements, EphemeralSeed, HealthStatus
};
use beardog_tunnel::universal_hsm_discovery::{
    DiscoveredHsm, HsmInterfaceType, HsmTier, HsmHealthStatus,
    HsmConnectionInfo, ConnectionType, AuthenticationMethod, RetryPolicy,
    IntegrationStatus
};
use beardog_errors::{BearDogError, BearDogResult};
use tokio_test;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test multi-vendor PKCS#11 HSM support
#[tokio::test]
async fn test_multi_vendor_pkcs11_support() {
    let vendors = vec![
        ("SafeNet", "/usr/lib/safenet/libpkcs11.so"),
        ("Thales", "/usr/lib/thales/libpkcs11.so"),
        ("Utimaco", "/usr/lib/utimaco/libpkcs11.so"),
        ("Cavium", "/usr/lib/cavium/libpkcs11.so"),
    ];

    let adapter = Pkcs11Adapter;

    for (vendor, library_path) in vendors {
        let hsm = create_mock_pkcs11_hsm(vendor, library_path);
        
        // Test connection
        let connection_result = adapter.connect(&hsm).await;
        assert!(connection_result.is_ok(), "Failed to connect to {} HSM", vendor);
        
        let connection = connection_result.unwrap();
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert!(connection.connection_handle.contains("pkcs11-session"));
        
        // Test health check
        let health_result = adapter.test_connection(&hsm).await;
        assert!(health_result.is_ok(), "Health check failed for {} HSM", vendor);
        
        let health = health_result.unwrap();
        assert!(health.is_healthy);
        assert!(health.response_time_ms > 0.0);
    }
}

/// Test cross-platform HSM operations
#[tokio::test]
async fn test_cross_platform_hsm_operations() {
    let test_scenarios = vec![
        ("PKCS#11", create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so")),
        ("Android StrongBox", create_mock_android_strongbox_hsm()),
        ("BearDog Native", create_mock_beardog_native_hsm()),
    ];

    for (platform, hsm) in test_scenarios {
        let adapter = get_adapter_for_hsm(&hsm);
        
        // Test connection
        let connection = adapter.connect(&hsm).await
            .expect(&format!("Failed to connect to {} HSM", platform));
        
        // Test key generation
        let gen_operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ("key_type".to_string(), "ecdsa_p256".to_string()),
                ("key_id".to_string(), format!("{}-test-key", platform.to_lowercase().replace(' ', "-"))),
            ]),
        };
        
        let gen_result = adapter.perform_operation(&connection, gen_operation).await
            .expect(&format!("Key generation failed for {} HSM", platform));
        
        assert!(gen_result.success);
        assert!(!gen_result.result_data.is_empty());
        
        // Test signing operation
        let sign_operation = UniversalOperation {
            operation_type: OperationType::Sign,
            parameters: HashMap::from([
                ("key_id".to_string(), format!("{}-test-key", platform.to_lowercase().replace(' ', "-"))),
                ("data".to_string(), hex::encode(b"Cross-platform test data")),
                ("algorithm".to_string(), "ECDSA_SHA256".to_string()),
            ]),
        };
        
        let sign_result = adapter.perform_operation(&connection, sign_operation).await
            .expect(&format!("Signing failed for {} HSM", platform));
        
        assert!(sign_result.success);
        assert!(!sign_result.result_data.is_empty());
        
        println!("✅ {} HSM: Key generation and signing successful", platform);
    }
}

/// Test performance across different HSM types
#[tokio::test]
async fn test_hsm_performance_comparison() {
    let hsm_types = vec![
        ("PKCS#11 SafeNet", create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so")),
        ("Android StrongBox", create_mock_android_strongbox_hsm()),
        ("BearDog Native", create_mock_beardog_native_hsm()),
    ];

    let mut performance_results = Vec::new();

    for (hsm_name, hsm) in hsm_types {
        let adapter = get_adapter_for_hsm(&hsm);
        let connection = adapter.connect(&hsm).await.unwrap();
        
        // Measure signing performance
        let start_time = Instant::now();
        let num_operations = 10;
        
        for i in 0..num_operations {
            let operation = UniversalOperation {
                operation_type: OperationType::Sign,
                parameters: HashMap::from([
                    ("key_id".to_string(), format!("perf-test-key-{}", i)),
                    ("data".to_string(), hex::encode(format!("Performance test data {}", i))),
                    ("algorithm".to_string(), "ECDSA_SHA256".to_string()),
                ]),
            };
            
            let result = adapter.perform_operation(&connection, operation).await.unwrap();
            assert!(result.success);
        }
        
        let total_duration = start_time.elapsed();
        let avg_duration_ms = total_duration.as_millis() as f64 / num_operations as f64;
        
        performance_results.push((hsm_name, avg_duration_ms));
        println!("📊 {} HSM: Average operation time: {:.2}ms", hsm_name, avg_duration_ms);
    }
    
    // Verify all HSMs completed operations
    assert_eq!(performance_results.len(), 3);
    
    // BearDog Native should be fastest (in our mock implementation)
    let beardog_perf = performance_results.iter()
        .find(|(name, _)| name.contains("BearDog"))
        .unwrap();
    assert!(beardog_perf.1 < 100.0, "BearDog Native should be fast");
}

/// Test failover between HSM vendors
#[tokio::test]
async fn test_hsm_vendor_failover() {
    // Simulate HSM priority list
    let hsm_priority = vec![
        create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so"),  // Primary
        create_mock_pkcs11_hsm("Thales", "/usr/lib/thales/libpkcs11.so"),    // Backup
        create_mock_beardog_native_hsm(),                                     // Tertiary
    ];
    
    // Test that we can fail over to any HSM in the priority list
    for (i, hsm) in hsm_priority.iter().enumerate() {
        let adapter = get_adapter_for_hsm(hsm);
        
        // Test connection
        let connection_result = adapter.connect(hsm).await;
        assert!(connection_result.is_ok(), "Failover HSM #{} connection failed", i + 1);
        
        let connection = connection_result.unwrap();
        
        // Test that operations work on failover HSM
        let operation = UniversalOperation {
            operation_type: OperationType::GenerateKey,
            parameters: HashMap::from([
                ("key_type".to_string(), "ecdsa_p256".to_string()),
                ("key_id".to_string(), format!("failover-test-key-{}", i)),
            ]),
        };
        
        let op_result = adapter.perform_operation(&connection, operation).await;
        assert!(op_result.is_ok(), "Operation failed on failover HSM #{}", i + 1);
        
        println!("✅ Failover HSM #{}: {} - Connection and operations successful", 
                 i + 1, hsm.vendor);
    }
}

/// Test concurrent operations across multiple HSMs
#[tokio::test]
async fn test_concurrent_multi_hsm_operations() {
    let hsms = vec![
        create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so"),
        create_mock_beardog_native_hsm(),
    ];
    
    let mut tasks = Vec::new();
    let operations_per_hsm = 5;
    
    for (hsm_index, hsm) in hsms.iter().enumerate() {
        for op_index in 0..operations_per_hsm {
            let hsm_clone = hsm.clone();
            let task = tokio::spawn(async move {
                let adapter = get_adapter_for_hsm(&hsm_clone);
                let connection = adapter.connect(&hsm_clone).await.unwrap();
                
                let operation = UniversalOperation {
                    operation_type: OperationType::Sign,
                    parameters: HashMap::from([
                        ("key_id".to_string(), format!("concurrent-key-{}-{}", hsm_index, op_index)),
                        ("data".to_string(), hex::encode(format!("concurrent data {} {}", hsm_index, op_index))),
                        ("algorithm".to_string(), "ECDSA_SHA256".to_string()),
                    ]),
                };
                
                adapter.perform_operation(&connection, operation).await
            });
            
            tasks.push(task);
        }
    }
    
    // Wait for all concurrent operations to complete
    let start_time = Instant::now();
    let results = futures::future::join_all(tasks).await;
    let total_duration = start_time.elapsed();
    
    // Verify all operations succeeded
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Concurrent operation {} failed", i);
        let op_result = result.as_ref().unwrap().as_ref().unwrap();
        assert!(op_result.success, "Concurrent operation {} was not successful", i);
    }
    
    println!("✅ Completed {} concurrent HSM operations in {:?}", 
             results.len(), total_duration);
    
    // Should complete reasonably quickly with concurrent operations
    assert!(total_duration < Duration::from_secs(10));
}

/// Test human entropy generation capabilities
#[tokio::test]
async fn test_human_entropy_capabilities() {
    let test_cases = vec![
        (create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so"), false),
        (create_mock_android_strongbox_hsm(), false),
        (create_mock_beardog_native_hsm(), true),
    ];
    
    for (hsm, should_support_entropy) in test_cases {
        let adapter = get_adapter_for_hsm(&hsm);
        
        // Test entropy support detection
        let supports_entropy = adapter.supports_human_entropy().await.unwrap();
        assert_eq!(supports_entropy, should_support_entropy, 
                   "Entropy support mismatch for {} HSM", hsm.vendor);
        
        if should_support_entropy {
            // Test entropy generation
            let connection = adapter.connect(&hsm).await.unwrap();
            let requirements = HumanEntropyRequirements {
                minimum_entropy_bits: 256,
                collection_timeout_seconds: 30,
            };
            
            let entropy_result = adapter.generate_human_entropy_seed(&connection, requirements).await;
            assert!(entropy_result.is_ok(), "Entropy generation failed for {} HSM", hsm.vendor);
            
            let seed = entropy_result.unwrap();
            assert_eq!(seed.seed_data.len(), 32); // 256 bits = 32 bytes
            assert!(seed.entropy_estimate > 0.8);
            assert!(seed.creation_timestamp <= chrono::Utc::now());
            
            println!("✅ {} HSM: Human entropy generation successful", hsm.vendor);
        } else {
            // Test that entropy generation properly fails
            let connection = adapter.connect(&hsm).await.unwrap();
            let requirements = HumanEntropyRequirements {
                minimum_entropy_bits: 256,
                collection_timeout_seconds: 30,
            };
            
            let entropy_result = adapter.generate_human_entropy_seed(&connection, requirements).await;
            assert!(entropy_result.is_err(), "Entropy generation should fail for {} HSM", hsm.vendor);
            
            if let Err(BearDogError::UnsupportedOperation { operation }) = entropy_result {
                assert!(operation.contains("human entropy"));
            } else {
                panic!("Expected UnsupportedOperation error for {} HSM", hsm.vendor);
            }
        }
    }
}

/// Test HSM health monitoring and status reporting
#[tokio::test]
async fn test_hsm_health_monitoring() {
    let hsms = vec![
        create_mock_pkcs11_hsm("SafeNet", "/usr/lib/safenet/libpkcs11.so"),
        create_mock_android_strongbox_hsm(),
        create_mock_beardog_native_hsm(),
    ];
    
    for hsm in hsms {
        let adapter = get_adapter_for_hsm(&hsm);
        
        // Test health check
        let health_result = adapter.test_connection(&hsm).await;
        assert!(health_result.is_ok(), "Health check failed for {} HSM", hsm.vendor);
        
        let health = health_result.unwrap();
        assert!(health.is_healthy, "{} HSM should be healthy", hsm.vendor);
        assert!(health.response_time_ms > 0.0, "Response time should be positive");
        assert!(health.response_time_ms < 1000.0, "Response time should be reasonable");
        assert!(health.last_check <= chrono::Utc::now(), "Last check timestamp should be valid");
        
        // Test that error cases are handled
        if health.error_message.is_some() {
            println!("ℹ️  {} HSM health message: {}", hsm.vendor, health.error_message.unwrap());
        }
        
        println!("✅ {} HSM: Health check passed ({}ms response time)", 
                 hsm.vendor, health.response_time_ms);
    }
}

// Helper functions
fn get_adapter_for_hsm(hsm: &DiscoveredHsm) -> Box<dyn HsmAdapter> {
    match &hsm.interface_type {
        HsmInterfaceType::Pkcs11 { .. } => Box::new(Pkcs11Adapter),
        HsmInterfaceType::AndroidStrongBox { .. } => Box::new(AndroidStrongBoxAdapter::new().unwrap()),
        HsmInterfaceType::BearDogNative { .. } => Box::new(BearDogNativeAdapter::new().unwrap()),
        _ => Box::new(BearDogNativeAdapter::new().unwrap()), // Default fallback
    }
}

fn create_mock_pkcs11_hsm(vendor: &str, library_path: &str) -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id: format!("pkcs11-{}-001", vendor.to_lowercase()),
        vendor: vendor.to_string(),
        model: format!("{} Network HSM", vendor),
        version: "7.4.0".to_string(),
        interface_type: HsmInterfaceType::Pkcs11 {
            library_path: library_path.to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(),
        assigned_tier: HsmTier::CertifiedHardware,
        supports_human_entropy: false,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(),
        integration_status: IntegrationStatus::Ready,
    }
}

fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id: "android-strongbox-001".to_string(),
        vendor: "Google".to_string(),
        model: "Android StrongBox".to_string(),
        version: "Android 13".to_string(),
        interface_type: HsmInterfaceType::AndroidStrongBox {
            security_level: "STRONGBOX".to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(),
        assigned_tier: HsmTier::HighSecurity,
        supports_human_entropy: false,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(),
        integration_status: IntegrationStatus::Ready,
    }
}

fn create_mock_beardog_native_hsm() -> DiscoveredHsm {
    DiscoveredHsm {
        hsm_id: "beardog-native-001".to_string(),
        vendor: "BearDog".to_string(),
        model: "Native HSM".to_string(),
        version: "1.0.0".to_string(),
        interface_type: HsmInterfaceType::BearDogNative {
            instance_id: "native-001".to_string(),
        },
        connection_info: create_mock_connection_info(),
        capabilities: create_mock_hsm_capabilities(),
        assigned_tier: HsmTier::HumanEntropyPremium,
        supports_human_entropy: true,
        health_status: HsmHealthStatus::Healthy,
        discovered_at: chrono::Utc::now(),
        last_health_check: chrono::Utc::now(),
        integration_status: IntegrationStatus::Ready,
    }
}

fn create_mock_connection_info() -> HsmConnectionInfo {
    HsmConnectionInfo {
        connection_type: ConnectionType::Local,
        authentication: AuthenticationMethod::None,
        endpoint: None,
        port: None,
        timeout: Duration::from_secs(30),
        retry_policy: RetryPolicy {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        },
        ssl_config: None,
    }
}

fn create_mock_hsm_capabilities() -> beardog_tunnel::universal_hsm_discovery::HsmCapabilities {
    // For now, create minimal capabilities
    use beardog_tunnel::universal_hsm_discovery::{
        HsmCapabilities, KeyGenerationCapabilities, CryptoOperationCapabilities,
        KeyManagementCapabilities, AdvancedFeatureCapabilities, SecurityCapabilities,
        ApiSupportCapabilities, ComplianceCapabilities, HumanEntropyCapabilities,
        PerformanceCapabilities
    };
    
    // Return default implementation for testing
    HsmCapabilities::default()
} 