use beardog_errors::BearDogError;

use beardog::production::*;

pub async fn test_performance_monitoring(prod_manager: &mut ProductionManager) {
    println!("📊 Testing performance monitoring systems...");

    let performance_metrics = prod_manager
        .get_performance_metrics()
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Performance metrics should be available", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Performance metrics should be available", e))
})?;

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

    let benchmark_results = prod_manager
        .run_performance_benchmarks()
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Performance benchmarks should succeed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Performance benchmarks should succeed", e))
})?;

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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Production manager creation failed", e))
})?;
        
    test_performance_monitoring(&mut production_manager);
} 