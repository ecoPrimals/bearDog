//! Production Performance Monitoring Tests
//!
//! Tests for performance monitoring, metrics collection,
//! performance benchmarks, and optimization validation.

use beardog::production::*;

/// Test production performance monitoring
pub async fn test_performance_monitoring(prod_manager: &mut ProductionManager) {
    println!("📊 Testing performance monitoring systems...");

    // Test performance metrics collection
    let performance_metrics = prod_manager
        .get_performance_metrics()
        .await
        .expect("Performance metrics should be available");

    assert!(
        performance_metrics.response_time_ms > 0.0,
        "Response time should be measured"
    );
    assert!(
        performance_metrics.throughput_ops_per_sec > 0.0,
        "Throughput should be measured"
    );
    assert!(
        performance_metrics.error_rate >= 0.0 && performance_metrics.error_rate <= 1.0,
        "Error rate should be valid percentage"
    );

    // Test performance benchmarks
    let benchmark_results = prod_manager
        .run_performance_benchmarks()
        .await
        .expect("Performance benchmarks should succeed");

    assert!(
        benchmark_results.encryption_performance_acceptable,
        "Encryption performance should meet requirements"
    );
    assert!(
        benchmark_results.database_performance_acceptable,
        "Database performance should be adequate"
    );
    assert!(
        benchmark_results.network_performance_acceptable,
        "Network performance should be sufficient"
    );

    println!("✅ Performance monitoring tests completed");
}

#[tokio::test]
async fn test_performance_monitoring_standalone() {
    use beardog::core::*;
    use std::sync::Arc;
    
    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .await
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_performance_monitoring(&mut production_manager).await;
} 