

/*
 * BearDog Security Provider Bridge Tests
 * 
 * Tests for enhanced security provider bridge with multi-vendor HSM integration,
 * performance monitoring, and failover capabilities
 */

use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    OperationType, PerformanceMetrics, HumanEntropyRequirements
};
use beardog_errors::{BearDogError, BearDogResult};
use tokio_test;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct MockSecurityProviderBridge {
    vendor_integrations: HashMap<String, MockVendorHsmIntegration>,
    metrics_collector: MockSecurityMetricsCollector,
}

struct MockVendorHsmIntegration {
    vendor_name: String,
    is_healthy: bool,
    performance_metrics: MockVendorPerformanceMetrics,
}

struct MockVendorPerformanceMetrics {
    avg_latency_ms: f64,
    operations_per_second: u32,
    error_rate: f64,
}

struct MockSecurityMetricsCollector {
    operation_counters: HashMap<String, u64>,
    latency_metrics: HashMap<String, Vec<f64>>,
    error_counters: HashMap<String, u64>,
}

impl MockSecurityProviderBridge {
    fn new() -> Self {
        Self {
            vendor_integrations: HashMap::with_capacity(16),
            metrics_collector: MockSecurityMetricsCollector::new(),
        }
    }

    fn register_vendor_integration(&mut self, vendor: &str) {
        let integration = MockVendorHsmIntegration {
            vendor_name: vendor.to_string(),
            is_healthy: true,
            performance_metrics: MockVendorPerformanceMetrics {
                avg_latency_ms: match vendor {
                    "SafeNet" => 15.0,
                    "Thales" => 12.0,
                    "BearDog" => 5.0,
                    _ => 20.0,
                },
                operations_per_second: match vendor {
                    "SafeNet" => 1000,
                    "Thales" => 1200,
                    "BearDog" => 2000,
                    _ => 800,
                },
                error_rate: 0.001, // 0.1% error rate
            },
        };
        
        self.vendor_integrations.insert(vendor.to_string(), integration);
    }

    async fn perform_vendor_operation(&mut self, vendor: &str, operation: OperationType) -> BearDogResult<Vec<u8>> {
        let start_time = Instant::now();

        let integration = self.vendor_integrations.get(vendor)
            .ok_or_else(|| BearDogError::not_found(format_args!("Vendor {) not registered", vendor).to_string(),
            })?;

        if !integration.is_healthy {
            return Err(BearDogError::Hsm {
                message: format_args!("Vendor {} is not healthy", vendor).to_string(),
            });
        }

        let latency_ms = integration.performance_metrics.avg_latency_ms;
        tokio::time::sleep(Duration::from_millis(latency_ms as u64)).await;

        if integration.performance_metrics.error_rate > 0.0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() < integration.performance_metrics.error_rate {
                self.metrics_collector.record_error(vendor, &operation);
                return Err(BearDogError::Hsm {
                    message: format_args!("Simulated error for vendor {}", vendor).to_string(),
                });
            }
        }

        let duration = start_time.elapsed();
        self.metrics_collector.record_operation(vendor, &operation, duration.as_secs_f64() * 1000.0);

        let result_data = match operation {
            OperationType::GenerateKey => b"mock_key_data".to_vec(),
            OperationType::Sign => b"mock_signature".to_vec(),
            OperationType::Verify => vec![1], // 1 for success
            OperationType::Encrypt => b"mock_encrypted_data".to_vec(),
            OperationType::Decrypt => b"mock_decrypted_data".to_vec(),
            OperationType::HumanEntropyGeneration => vec![0u8; 32], // 256 bits of entropy
        };

        Ok(result_data)
    }

    async fn perform_multi_vendor_operation(&mut self, preferred_vendors: Vec<&str>, operation: OperationType) -> BearDogResult<Vec<u8>> {
        let mut last_error = None;
        
        for vendor in preferred_vendors {
            match self.perform_vendor_operation(vendor, operation.clone()).await {
                Ok(result) => {
                    println!("✅ Operation succeeded with vendor: {}", vendor);
                    return Ok(result);
                }
                Err(err) => {
                    println!("❌ Operation failed with vendor {}: {:?}", vendor, err);
                    last_error = Some(err);

                }
            }
        }

        Err(last_error.unwrap_or_else(|| BearDogError::not_found("No vendors available".to_string(),
        )))
    }

    fn get_security_metrics(&self) -> SecurityMetrics {
        SecurityMetrics {
            total_operations: self.metrics_collector.operation_counters.values().sum(),
            avg_latency_ms: self.metrics_collector.calculate_avg_latency(),
            total_errors: self.metrics_collector.error_counters.values().sum(),
            vendor_metrics: self.get_vendor_metrics(),
        }
    }

    fn get_vendor_metrics(&self) -> HashMap<String, VendorMetrics> {
        let mut vendor_metrics = HashMap::with_capacity(16);
        
        for (vendor_name, integration) in &self.vendor_integrations {
            let operations = self.metrics_collector.operation_counters.get(vendor_name).unwrap_or(&0);
            let errors = self.metrics_collector.error_counters.get(vendor_name).unwrap_or(&0);
            let error_rate = if *operations > 0 { *errors as f64 / *operations as f64 } else { 0.0 };
            
            vendor_metrics.insert(vendor_name.clone(), VendorMetrics {
                operations_count: *operations,
                avg_latency_ms: integration.performance_metrics.avg_latency_ms,
                error_rate,
                is_healthy: integration.is_healthy,
            });
        }
        
        vendor_metrics
    }

    fn set_vendor_health(&mut self, vendor: &str, is_healthy: bool) {
        if let Some(integration) = self.vendor_integrations.get_mut(vendor) {
            integration.is_healthy = is_healthy;
        }
    }
}

impl MockSecurityMetricsCollector {
    fn new() -> Self {
        Self {
            operation_counters: HashMap::with_capacity(16),
            latency_metrics: HashMap::with_capacity(16),
            error_counters: HashMap::with_capacity(16),
        }
    }

    fn record_operation(&mut self, vendor: &str, operation: &OperationType, latency_ms: f64) {
        let key = format_args!("{}_{:?}", vendor, operation).to_string();
        *self.operation_counters.entry(key.clone()).or_insert(0) += 1;
        self.latency_metrics.entry(key).or_insert_with(Vec::new).push(latency_ms);
    }

    fn record_error(&mut self, vendor: &str, operation: &OperationType) {
        let key = format_args!("{}_{:?}", vendor, operation).to_string();
        *self.error_counters.entry(key).or_insert(0) += 1;
    }

    fn calculate_avg_latency(&self) -> f64 {
        let all_latencies: Vec<f64> = self.latency_metrics.values().flatten().copied().collect();
        if all_latencies.is_empty() {
            0.0
        } else {
            all_latencies.iter().sum::<f64>() / all_latencies.len() as f64
        }
    }
}

#[derive(Debug)]
struct SecurityMetrics {
    total_operations: u64,
    avg_latency_ms: f64,
    total_errors: u64,
    vendor_metrics: HashMap<String, VendorMetrics>,
}

#[derive(Debug)]
struct VendorMetrics {
    operations_count: u64,
    avg_latency_ms: f64,
    error_rate: f64,
    is_healthy: bool,
}

#[tokio::test]
async fn test_vendor_registration() {
    let mut bridge = MockSecurityProviderBridge::new();

    let vendors = vec!["SafeNet", "Thales", "BearDog", "Utimaco"];
    for vendor in &vendors {
        bridge.register_vendor_integration(vendor);
    }

    assert_eq!(bridge.vendor_integrations.len(), 4);
    
    for vendor in &vendors {
        assert!(bridge.vendor_integrations.contains_key(*vendor));
        let integration = bridge.vendor_integrations.get(*vendor).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert_eq!(integration.vendor_name, *vendor);
        assert!(integration.is_healthy);
    }
    
    println!("✅ Successfully registered {} HSM vendors", vendors.len());
}

#[tokio::test]
async fn test_performance_monitoring() {
    let mut bridge = MockSecurityProviderBridge::new();

    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("Thales");
    bridge.register_vendor_integration("BearDog");

    let operations = vec![
        OperationType::GenerateKey,
        OperationType::Sign,
        OperationType::Verify,
    ];
    
    for vendor in ["SafeNet", "Thales", "BearDog"] {
        for operation in &operations {
            let result = bridge.perform_vendor_operation(vendor, operation.clone()).await;
            assert!(result.is_ok(), "Operation {:?} failed for vendor {}", operation, vendor);
        }
    }

    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, 9); // 3 vendors × 3 operations
    assert!(metrics.avg_latency_ms > 0.0);
    assert_eq!(metrics.total_errors, 0); // No errors expected in normal operation

    assert_eq!(metrics.vendor_metrics.len(), 3);
    
    for (vendor_name, vendor_metrics) in &metrics.vendor_metrics {
        assert_eq!(vendor_metrics.operations_count, 3);
        assert!(vendor_metrics.avg_latency_ms > 0.0);
        assert!(vendor_metrics.error_rate < 0.01); // Less than 1% error rate
        assert!(vendor_metrics.is_healthy);
        
        println!("📊 {} HSM: {} operations, {:.2}ms avg latency, {:.3}% error rate",
                 vendor_name, vendor_metrics.operations_count, 
                 vendor_metrics.avg_latency_ms, vendor_metrics.error_rate * 100.0);
    }
}

#[tokio::test]
async fn test_multi_vendor_failover() {
    let mut bridge = MockSecurityProviderBridge::new();

    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("Thales");
    bridge.register_vendor_integration("BearDog");

    let preferred_vendors = vec!["SafeNet", "Thales", "BearDog"];
    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::GenerateKey).await;
    assert!(result.is_ok());

    bridge.set_vendor_health("SafeNet", false);

    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::Sign).await;
    assert!(result.is_ok());

    bridge.set_vendor_health("Thales", false);

    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::Verify).await;
    assert!(result.is_ok());

    bridge.set_vendor_health("BearDog", false);

    let result = bridge.perform_multi_vendor_operation(preferred_vendors, OperationType::Encrypt).await;
    assert!(result.is_err());
    
    println!("✅ Multi-vendor failover test completed successfully");
}

#[tokio::test]
async fn test_concurrent_vendor_operations() {
    let bridge = std::sync::Arc::new(tokio::sync::Mutex::new(MockSecurityProviderBridge::new()));

    {
        let mut bridge_lock = bridge.lock().await;
        bridge_lock.register_vendor_integration("SafeNet");
        bridge_lock.register_vendor_integration("Thales");
        bridge_lock.register_vendor_integration("BearDog");
    }

    let mut tasks = Vec::new();
    let vendors = vec!["SafeNet", "Thales", "BearDog"];
    let operations_per_vendor = 5;
    
    for vendor in vendors {
        for i in 0..operations_per_vendor {
            let bridge_clone = bridge.clone();
            let vendor_name = vendor.to_string();
            
            let task = tokio::spawn(async move {
                let mut bridge_lock = bridge_clone.lock().await;
                let operation = match i % 3 {
                    0 => OperationType::GenerateKey,
                    1 => OperationType::Sign,
                    _ => OperationType::Verify,
                };
                
                bridge_lock.perform_vendor_operation(&vendor_name, operation).await
            });
            
            tasks.push(task);
        }
    }

    let start_time = Instant::now();
    let results = futures::future::join_all(tasks).await;
    let total_duration = start_time.elapsed();

    let mut success_count = 0;
    for result in results {
        assert!(result.is_ok()); // Task should not panic
        if result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?.is_ok() {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, 15); // 3 vendors × 5 operations each

    {
        let bridge_lock = bridge.lock().await;
        let metrics = bridge_lock.get_security_metrics();
        assert_eq!(metrics.total_operations, 15);
        assert!(metrics.avg_latency_ms > 0.0);
    }
    
    println!("✅ Completed {} concurrent vendor operations in {:?}", 
             success_count, total_duration);

    assert!(total_duration < Duration::from_secs(5));
}

#[tokio::test]
async fn test_security_provider_performance_load() {
    let mut bridge = MockSecurityProviderBridge::new();

    bridge.register_vendor_integration("BearDog");

    let num_operations = 100;
    let start_time = Instant::now();
    
    for i in 0..num_operations {
        let operation = match i % 4 {
            0 => OperationType::GenerateKey,
            1 => OperationType::Sign,
            2 => OperationType::Verify,
            _ => OperationType::Encrypt,
        };
        
        let result = bridge.perform_vendor_operation("BearDog", operation).await;
        assert!(result.is_ok(), "Operation {} failed", i);
    }
    
    let total_duration = start_time.elapsed();
    let avg_duration_per_op = total_duration.as_millis() as f64 / num_operations as f64;
    
    println!("📊 Load test: {} operations in {:?} (avg: {:.2}ms per operation)", 
             num_operations, total_duration, avg_duration_per_op);

    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, num_operations);
    assert!(metrics.avg_latency_ms > 0.0);
    assert_eq!(metrics.total_errors, 0);

    assert!(avg_duration_per_op < 20.0, "Average operation time should be under 20ms");
    assert!(total_duration < Duration::from_secs(10), "Total load test should complete within 10 seconds");
}

#[tokio::test]
async fn test_vendor_health_monitoring() {
    let mut bridge = MockSecurityProviderBridge::new();

    let vendors = vec!["SafeNet", "Thales", "BearDog"];
    for vendor in &vendors {
        bridge.register_vendor_integration(vendor);
    }

    let metrics = bridge.get_security_metrics();
    for (vendor_name, vendor_metrics) in &metrics.vendor_metrics {
        assert!(vendor_metrics.is_healthy, "Vendor {} should be healthy", vendor_name);
    }

    bridge.set_vendor_health("SafeNet", false);

    let result = bridge.perform_vendor_operation("SafeNet", OperationType::GenerateKey).await;
    assert!(result.is_err(), "Operation should fail for unhealthy vendor");

    let result = bridge.perform_vendor_operation("Thales", OperationType::GenerateKey).await;
    assert!(result.is_ok(), "Operation should succeed for healthy vendor");

    bridge.set_vendor_health("SafeNet", true);

    let result = bridge.perform_vendor_operation("SafeNet", OperationType::GenerateKey).await;
    assert!(result.is_ok(), "Operation should succeed for recovered vendor");
    
    println!("✅ Vendor health monitoring test completed successfully");
}

#[tokio::test]
async fn test_security_metrics_collection() {
    let mut bridge = MockSecurityProviderBridge::new();

    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("BearDog");

    let test_operations = vec![
        ("SafeNet", OperationType::GenerateKey),
        ("SafeNet", OperationType::Sign),
        ("BearDog", OperationType::GenerateKey),
        ("BearDog", OperationType::Sign),
        ("BearDog", OperationType::HumanEntropyGeneration),
    ];
    
    for (vendor, operation) in test_operations {
        let result = bridge.perform_vendor_operation(vendor, operation).await;
        assert!(result.is_ok(), "Operation {:?} failed for vendor {}", operation, vendor);
    }

    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, 5);
    assert!(metrics.avg_latency_ms > 0.0);

    let safenet_metrics = metrics.vendor_metrics.get("SafeNet").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert_eq!(safenet_metrics.operations_count, 2);
    
    let beardog_metrics = metrics.vendor_metrics.get("BearDog").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert_eq!(beardog_metrics.operations_count, 3);

    assert!(beardog_metrics.avg_latency_ms < safenet_metrics.avg_latency_ms,
            "BearDog should be faster than SafeNet");
    
    println!("📊 Security Metrics Summary:");
    println!("  Total Operations: {}", metrics.total_operations);
    println!("  Average Latency: {:.2}ms", metrics.avg_latency_ms);
    println!("  Total Errors: {}", metrics.total_errors);
    
    for (vendor, vendor_metrics) in &metrics.vendor_metrics {
        println!("  {} HSM: {} ops, {:.2}ms avg, {:.3}% errors",
                 vendor, vendor_metrics.operations_count,
                 vendor_metrics.avg_latency_ms, vendor_metrics.error_rate * 100.0);
    }
} 