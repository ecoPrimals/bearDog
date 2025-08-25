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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Performance metrics should be available", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Performance metrics should be available", e))
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

    // Test performance benchmarks
    let benchmark_results = prod_manager
        .run_performance_benchmarks()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Performance benchmarks should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Performance benchmarks should succeed", e))
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
            .await
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Production manager creation failed", e))
})?;
        
    test_performance_monitoring(&mut production_manager).await;
} 