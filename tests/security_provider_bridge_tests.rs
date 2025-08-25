// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


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

// Mock security provider bridge functionality for testing
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
            vendor_integrations: HashMap::new(),
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
        
        // Check if vendor is registered
        let integration = self.vendor_integrations.get(vendor)
            .ok_or_else(|| BearDogError::not_found(format!("Vendor {) not registered", vendor),
            })?;

        if !integration.is_healthy {
            return Err(BearDogError::Hsm {
                message: format!("Vendor {} is not healthy", vendor),
            });
        }

        // Simulate operation latency
        let latency_ms = integration.performance_metrics.avg_latency_ms;
        tokio::time::sleep(Duration::from_millis(latency_ms as u64)).await;

        // Simulate potential errors
        if integration.performance_metrics.error_rate > 0.0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() < integration.performance_metrics.error_rate {
                self.metrics_collector.record_error(vendor, &operation);
                return Err(BearDogError::Hsm {
                    message: format!("Simulated error for vendor {}", vendor),
                });
            }
        }

        let duration = start_time.elapsed();
        self.metrics_collector.record_operation(vendor, &operation, duration.as_secs_f64() * 1000.0);

        // Return mock result data
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
                    // Continue to next vendor
                }
            }
        }
        
        // All vendors failed
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
        let mut vendor_metrics = HashMap::new();
        
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
            operation_counters: HashMap::new(),
            latency_metrics: HashMap::new(),
            error_counters: HashMap::new(),
        }
    }

    fn record_operation(&mut self, vendor: &str, operation: &OperationType, latency_ms: f64) {
        let key = format!("{}_{:?}", vendor, operation);
        *self.operation_counters.entry(key.clone()).or_insert(0) += 1;
        self.latency_metrics.entry(key).or_insert_with(Vec::new).push(latency_ms);
    }

    fn record_error(&mut self, vendor: &str, operation: &OperationType) {
        let key = format!("{}_{:?}", vendor, operation);
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

/// Test vendor registration and management
#[tokio::test]
async fn test_vendor_registration() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register multiple vendors
    let vendors = vec!["SafeNet", "Thales", "BearDog", "Utimaco"];
    for vendor in &vendors {
        bridge.register_vendor_integration(vendor);
    }
    
    // Verify all vendors are registered
    assert_eq!(bridge.vendor_integrations.len(), 4);
    
    for vendor in &vendors {
        assert!(bridge.vendor_integrations.contains_key(*vendor));
        let integration = bridge.vendor_integrations.get(*vendor).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert_eq!(integration.vendor_name, *vendor);
        assert!(integration.is_healthy);
    }
    
    println!("✅ Successfully registered {} HSM vendors", vendors.len());
}

/// Test performance monitoring across vendors
#[tokio::test]
async fn test_performance_monitoring() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register vendors with different performance characteristics
    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("Thales");
    bridge.register_vendor_integration("BearDog");
    
    // Perform operations on each vendor
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
    
    // Check performance metrics
    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, 9); // 3 vendors × 3 operations
    assert!(metrics.avg_latency_ms > 0.0);
    assert_eq!(metrics.total_errors, 0); // No errors expected in normal operation
    
    // Verify vendor-specific metrics
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

/// Test multi-vendor failover functionality
#[tokio::test]
async fn test_multi_vendor_failover() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register vendors
    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("Thales");
    bridge.register_vendor_integration("BearDog");
    
    // Test normal operation with preferred vendor list
    let preferred_vendors = vec!["SafeNet", "Thales", "BearDog"];
    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::GenerateKey).await;
    assert!(result.is_ok());
    
    // Simulate primary vendor failure
    bridge.set_vendor_health("SafeNet", false);
    
    // Test failover to secondary vendor
    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::Sign).await;
    assert!(result.is_ok());
    
    // Simulate secondary vendor failure
    bridge.set_vendor_health("Thales", false);
    
    // Test failover to tertiary vendor (BearDog)
    let result = bridge.perform_multi_vendor_operation(preferred_vendors.clone(), OperationType::Verify).await;
    assert!(result.is_ok());
    
    // Simulate all vendors failing
    bridge.set_vendor_health("BearDog", false);
    
    // Test that operation fails when all vendors are down
    let result = bridge.perform_multi_vendor_operation(preferred_vendors, OperationType::Encrypt).await;
    assert!(result.is_err());
    
    println!("✅ Multi-vendor failover test completed successfully");
}

/// Test concurrent operations across multiple vendors
#[tokio::test]
async fn test_concurrent_vendor_operations() {
    let bridge = std::sync::Arc::new(tokio::sync::Mutex::new(MockSecurityProviderBridge::new()));
    
    // Register vendors
    {
        let mut bridge_lock = bridge.lock().await;
        bridge_lock.register_vendor_integration("SafeNet");
        bridge_lock.register_vendor_integration("Thales");
        bridge_lock.register_vendor_integration("BearDog");
    }
    
    // Create concurrent tasks
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
    
    // Wait for all concurrent operations
    let start_time = Instant::now();
    let results = futures::future::join_all(tasks).await;
    let total_duration = start_time.elapsed();
    
    // Verify all operations succeeded
    let mut success_count = 0;
    for result in results {
        assert!(result.is_ok()); // Task should not panic
        if result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?.is_ok() {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, 15); // 3 vendors × 5 operations each
    
    // Verify performance metrics
    {
        let bridge_lock = bridge.lock().await;
        let metrics = bridge_lock.get_security_metrics();
        assert_eq!(metrics.total_operations, 15);
        assert!(metrics.avg_latency_ms > 0.0);
    }
    
    println!("✅ Completed {} concurrent vendor operations in {:?}", 
             success_count, total_duration);
    
    // Should complete within reasonable time for concurrent operations
    assert!(total_duration < Duration::from_secs(5));
}

/// Test security provider bridge performance under load
#[tokio::test]
async fn test_security_provider_performance_load() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register high-performance vendor
    bridge.register_vendor_integration("BearDog");
    
    // Perform load test
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
    
    // Verify performance metrics
    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, num_operations);
    assert!(metrics.avg_latency_ms > 0.0);
    assert_eq!(metrics.total_errors, 0);
    
    // Performance should be reasonable
    assert!(avg_duration_per_op < 20.0, "Average operation time should be under 20ms");
    assert!(total_duration < Duration::from_secs(10), "Total load test should complete within 10 seconds");
}

/// Test vendor health monitoring and recovery
#[tokio::test]
async fn test_vendor_health_monitoring() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register vendors
    let vendors = vec!["SafeNet", "Thales", "BearDog"];
    for vendor in &vendors {
        bridge.register_vendor_integration(vendor);
    }
    
    // All vendors should initially be healthy
    let metrics = bridge.get_security_metrics();
    for (vendor_name, vendor_metrics) in &metrics.vendor_metrics {
        assert!(vendor_metrics.is_healthy, "Vendor {} should be healthy", vendor_name);
    }
    
    // Simulate vendor failure
    bridge.set_vendor_health("SafeNet", false);
    
    // Test operation with failed vendor
    let result = bridge.perform_vendor_operation("SafeNet", OperationType::GenerateKey).await;
    assert!(result.is_err(), "Operation should fail for unhealthy vendor");
    
    // Verify healthy vendors still work
    let result = bridge.perform_vendor_operation("Thales", OperationType::GenerateKey).await;
    assert!(result.is_ok(), "Operation should succeed for healthy vendor");
    
    // Simulate vendor recovery
    bridge.set_vendor_health("SafeNet", true);
    
    // Test operation with recovered vendor
    let result = bridge.perform_vendor_operation("SafeNet", OperationType::GenerateKey).await;
    assert!(result.is_ok(), "Operation should succeed for recovered vendor");
    
    println!("✅ Vendor health monitoring test completed successfully");
}

/// Test security metrics collection and reporting
#[tokio::test]
async fn test_security_metrics_collection() {
    let mut bridge = MockSecurityProviderBridge::new();
    
    // Register vendors
    bridge.register_vendor_integration("SafeNet");
    bridge.register_vendor_integration("BearDog");
    
    // Perform various operations
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
    
    // Analyze collected metrics
    let metrics = bridge.get_security_metrics();
    assert_eq!(metrics.total_operations, 5);
    assert!(metrics.avg_latency_ms > 0.0);
    
    // Verify vendor-specific metrics
    let safenet_metrics = metrics.vendor_metrics.get("SafeNet").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
    assert_eq!(safenet_metrics.operations_count, 2);
    
    let beardog_metrics = metrics.vendor_metrics.get("BearDog").map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
    assert_eq!(beardog_metrics.operations_count, 3);
    
    // BearDog should be faster than SafeNet (based on mock implementation)
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