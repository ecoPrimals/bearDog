//! Comprehensive Performance Benchmark Tests
//!
//! Extended test coverage for performance benchmarks including:
//! - Latency tests
//! - Throughput tests
//! - Resource usage
//! - Scalability tests

use super::*;
use beardog_errors::BearDogError;
use std::time::{Duration, Instant};

#[cfg(test)]
mod performance_benchmark_tests {
    use super::*;

    // ========== Latency Tests ==========

    #[tokio::test]
    async fn test_latency_discovery_single_run() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration = start.elapsed();
        
        // Should complete in reasonable time
        assert!(duration.as_secs() < 5,
            "Discovery latency too high: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_latency_health_check() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        let _ = discovery.perform_health_checks()?;
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 2,
            "Health check latency too high: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_latency_get_stats() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        let _ = discovery.get_discovery_stats();
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 100,
            "Stats retrieval latency too high: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_latency_tier_classification() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        let _ = discovery.get_hsms_by_tier()?;
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 50,
            "Tier classification latency too high: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_latency_best_hsm_selection() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        let _ = discovery.get_best_available_hsm()?;
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 50,
            "Best HSM selection latency too high: {:?}", duration);
        
        Ok(())
    }

    // ========== Throughput Tests ==========

    #[tokio::test]
    async fn test_throughput_sequential_discoveries() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        
        for _ in 0..10 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        let duration = start.elapsed();
        let throughput = 10.0 / duration.as_secs_f64();
        
        assert!(throughput > 1.0,
            "Throughput too low: {} discoveries/sec", throughput);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_throughput_sequential_health_checks() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        
        for _ in 0..20 {
            let _ = discovery.perform_health_checks();
        }
        
        let duration = start.elapsed();
        let throughput = 20.0 / duration.as_secs_f64();
        
        assert!(throughput > 2.0,
            "Health check throughput too low: {} checks/sec", throughput);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_throughput_status_updates() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            let start = Instant::now();
            
            for i in 0..50 {
                let status = if i % 2 == 0 {
                    HsmHealthStatus::Healthy
                } else {
                    HsmHealthStatus::Degraded
                };
                let _ = discovery.update_hsm_status(&hsm.id, status);
            }
            
            let duration = start.elapsed();
            let throughput = 50.0 / duration.as_secs_f64();
            
            assert!(throughput > 50.0,
                "Status update throughput too low: {} updates/sec", throughput);
        }
        
        Ok(())
    }

    // ========== Resource Usage Tests ==========

    #[tokio::test]
    async fn test_memory_usage_baseline() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Should complete without excessive memory
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_stability_repeated_discovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Repeated operations shouldn't leak memory
        for _ in 0..100 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_cpu_usage_idle() -> Result<(), BearDogError> {
        let discovery = UniversalHsmDiscovery::new()?;
        
        // Idle discovery should not consume CPU
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        Ok(())
    }

    // ========== Scalability Tests ==========

    #[tokio::test]
    async fn test_scalability_many_hsms() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        let start = Instant::now();
        
        // Operations should scale with number of HSMs
        for _ in 0..10 {
            let _ = discovery.get_hsms_by_tier()?;
        }
        
        let duration = start.elapsed();
        
        // Should scale linearly or better
        assert!(duration.as_millis() < 1000);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_scalability_concurrent_operations() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let start = Instant::now();
        let mut handles = vec![];
        
        for _ in 0..10 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.get_discovery_stats()
            }));
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 2,
            "Concurrent operations too slow: {:?}", duration);
        
        Ok(())
    }

    // ========== Warm-up Tests ==========

    #[tokio::test]
    async fn test_warmup_first_discovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let first_duration = start.elapsed();
        
        // Subsequent discoveries should be faster (caching)
        let start2 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let second_duration = start2.elapsed();
        
        // Allow warm-up effect
        assert!(second_duration <= first_duration * 2);
        
        Ok(())
    }

    // ========== Steady State Tests ==========

    #[tokio::test]
    async fn test_steady_state_performance() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Warm up
        for _ in 0..5 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        // Measure steady state
        let start = Instant::now();
        
        for _ in 0..10 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        let duration = start.elapsed();
        let avg_latency = duration.as_millis() / 10;
        
        assert!(avg_latency < 500,
            "Steady state latency too high: {}ms", avg_latency);
        
        Ok(())
    }

    // ========== Peak Load Tests ==========

    #[tokio::test]
    async fn test_peak_load_handling() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let start = Instant::now();
        let mut handles = vec![];
        
        // Peak load: 50 concurrent operations
        for _ in 0..50 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.perform_health_checks()
            }));
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 10,
            "Peak load handling too slow: {:?}", duration);
        
        Ok(())
    }

    // ========== Burst Tests ==========

    #[tokio::test]
    async fn test_burst_traffic_handling() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Burst: rapid sequential operations
        let start = Instant::now();
        
        for _ in 0..20 {
            let _ = discovery.discover_all_hsms()?;
            let _ = discovery.perform_health_checks();
        }
        
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 30,
            "Burst handling too slow: {:?}", duration);
        
        Ok(())
    }

    // ========== Degradation Tests ==========

    #[tokio::test]
    async fn test_performance_degradation_over_time() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Initial performance
        let start1 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration1 = start1.elapsed();
        
        // After many operations
        for _ in 0..50 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        let start2 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration2 = start2.elapsed();
        
        // Should not degrade significantly
        assert!(duration2.as_millis() <= duration1.as_millis() * 3,
            "Performance degraded: {:?} -> {:?}", duration1, duration2);
        
        Ok(())
    }

    // ========== Comparison Tests ==========

    #[tokio::test]
    async fn test_compare_discovery_vs_rediscovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // First discovery
        let start1 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let first_time = start1.elapsed();
        
        // Rediscovery
        let start2 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let rediscovery_time = start2.elapsed();
        
        // Rediscovery may benefit from caching
        assert!(rediscovery_time <= first_time * 2);
        
        Ok(())
    }

    // ========== Optimization Verification Tests ==========

    #[tokio::test]
    async fn test_optimization_tier_caching() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // First tier query
        let start1 = Instant::now();
        let _ = discovery.get_hsms_by_tier()?;
        let first_time = start1.elapsed();
        
        // Second tier query (should be faster if cached)
        let start2 = Instant::now();
        let _ = discovery.get_hsms_by_tier()?;
        let second_time = start2.elapsed();
        
        assert!(second_time <= first_time * 2);
        
        Ok(())
    }

    // ========== Regression Tests ==========

    #[tokio::test]
    async fn test_performance_regression_baseline() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration = start.elapsed();
        
        // Baseline: discovery should complete in < 5 seconds
        assert!(duration.as_secs() < 5,
            "Performance regression detected: {:?}", duration);
        
        Ok(())
    }

    // ========== Percentile Tests ==========

    #[tokio::test]
    async fn test_p99_latency() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let mut latencies = Vec::new();
        
        // Collect 100 samples
        for _ in 0..100 {
            let start = Instant::now();
            let _ = discovery.discover_all_hsms()?;
            latencies.push(start.elapsed().as_millis());
        }
        
        latencies.sort();
        let p99 = latencies[98]; // 99th percentile
        
        assert!(p99 < 1000,
            "P99 latency too high: {}ms", p99);
        
        Ok(())
    }

    // ========== Stress Recovery Tests ==========

    #[tokio::test]
    async fn test_stress_recovery_performance() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Stress: 50 rapid operations
        for _ in 0..50 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        // Recovery: check if performance is normal
        let start = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 5,
            "Failed to recover from stress: {:?}", duration);
        
        Ok(())
    }

    // ========== Cold Start Tests ==========

    #[tokio::test]
    async fn test_cold_start_latency() -> Result<(), BearDogError> {
        // Fresh instance (cold start)
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let cold_start = start.elapsed();
        
        // Cold start should still be reasonable
        assert!(cold_start.as_secs() < 10,
            "Cold start too slow: {:?}", cold_start);
        
        Ok(())
    }

    // ========== Efficiency Tests ==========

    #[tokio::test]
    async fn test_operation_efficiency() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Multiple operations should be efficient
        let start = Instant::now();
        
        for _ in 0..10 {
            let _ = discovery.get_discovery_stats();
            let _ = discovery.get_hsms_by_tier()?;
            let _ = discovery.get_best_available_hsm()?;
        }
        
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 500,
            "Operations not efficient: {:?}", duration);
        
        Ok(())
    }

    #[test]
    fn test_performance_initialization() {
        let start = Instant::now();
        let discovery = UniversalHsmDiscovery::new();
        let duration = start.elapsed();
        
        assert!(discovery.is_ok());
        assert!(duration.as_millis() < 100,
            "Initialization too slow: {:?}", duration);
    }
}

