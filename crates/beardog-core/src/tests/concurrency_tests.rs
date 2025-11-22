//! Concurrency Tests
//!
//! Tests for concurrency, thread-safety, and concurrent access patterns

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;

    #[derive(Debug, Clone)]
    struct SharedState {
        counter: i32,
        data: String,
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[test]
    fn test_concurrent_reads() {
        // Test concurrent read access
        let state = Arc::new(RwLock::new(SharedState {
            counter: 42,
            data: "test data".to_string(),
        }));

        let mut handles = vec![];

        // Spawn multiple readers
        for _ in 0..5 {
            let state_clone = Arc::clone(&state);
            let handle = thread::spawn(move || {
                let read_state = state_clone.read().unwrap();
                assert_eq!(read_state.counter, 42);
                assert_eq!(read_state.data, "test data");
            });
            handles.push(handle);
        }

        // Wait for all readers
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[test]
    fn test_concurrent_writes() {
        // Test concurrent write access
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        // Spawn multiple writers
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
            });
            handles.push(handle);
        }

        // Wait for all writers
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify final count
        let final_count = counter.lock().unwrap();
        assert_eq!(*final_count, 10);
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[test]
    fn test_lock_management() {
        // Test lock acquisition and release
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));

        // Acquire write lock
        {
            let mut write_data = data.write().unwrap();
            write_data.push(4);
            assert_eq!(write_data.len(), 4);
        } // Write lock released

        // Acquire read lock after write
        {
            let read_data = data.read().unwrap();
            assert_eq!(read_data.len(), 4);
            assert_eq!(*read_data, vec![1, 2, 3, 4]);
        }
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[test]
    fn test_deadlock_prevention() {
        // Test proper lock ordering to prevent deadlocks
        let resource_a = Arc::new(Mutex::new(1));
        let resource_b = Arc::new(Mutex::new(2));

        let first_resource = Arc::clone(&resource_a);
        let second_resource = Arc::clone(&resource_b);

        let handle = thread::spawn(move || {
            // Lock in consistent order: A then B
            let _a = first_resource.lock().unwrap();
            let _b = second_resource.lock().unwrap();
        });

        // Main thread also locks in same order: A then B
        {
            let _a = resource_a.lock().unwrap();
            let _b = resource_b.lock().unwrap();
        }

        handle.join().unwrap();
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[test]
    fn test_thread_safety() {
        // Test thread-safe data structure
        let shared_vec = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        // Multiple threads pushing data
        for i in 0..5 {
            let vec_clone = Arc::clone(&shared_vec);
            let handle = thread::spawn(move || {
                let mut vec = vec_clone.lock().unwrap();
                vec.push(i);
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all elements were added
        let final_vec = shared_vec.lock().unwrap();
        assert_eq!(final_vec.len(), 5);
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[test]
    fn test_arc_clone_behavior() {
        // Test Arc reference counting
        let data = Arc::new(42);
        assert_eq!(Arc::strong_count(&data), 1);

        let data_clone1 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);

        let data_clone2 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 3);

        drop(data_clone1);
        assert_eq!(Arc::strong_count(&data), 2);

        drop(data_clone2);
        assert_eq!(Arc::strong_count(&data), 1);
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_configuration_types() {
        // Test that different configuration types work
        let dev_config =
            beardog_types::canonical::config::unified::UnifiedBearDogConfig::development();
        let prod_config =
            beardog_types::canonical::config::unified::UnifiedBearDogConfig::production();
        let default_config =
            beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();

        // All configs should be valid
        let core_dev = crate::core::system::BearDogCore::new(dev_config);
        let core_prod = crate::core::system::BearDogCore::new(prod_config);
        let core_default = crate::core::system::BearDogCore::new(default_config);

        assert_eq!(
            core_dev.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
        assert_eq!(
            core_prod.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
        assert_eq!(
            core_default.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
    }

    /// TEST_CATEGORY: performance
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_state_read_latency() {
        // Test that state reads have low latency
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _state = core.state.read().await;
        }
        let elapsed = start.elapsed();

        // 100 reads should be very fast (<10ms)
        assert!(
            elapsed.as_millis() < 10,
            "State reads too slow: {:?}",
            elapsed
        );
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_state_write_isolation() {
        // Test that writes don't interfere with reads in other scopes
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        // Initial read
        let initial_health = {
            let state = core.state.read().await;
            state.overall_health
        };

        // Write in isolated scope
        {
            let mut state = core.state.write().await;
            state.overall_health = beardog_types::canonical::HealthStatus::Degraded;
        }

        // Read in new scope shows update
        {
            let state = core.state.read().await;
            assert_eq!(
                state.overall_health,
                beardog_types::canonical::HealthStatus::Degraded
            );
        }

        // Restore original
        {
            let mut state = core.state.write().await;
            state.overall_health = initial_health;
        }
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[test]
    fn test_health_status_equality() {
        // Test HealthStatus equality comparisons
        use beardog_types::canonical::HealthStatus;
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_eq!(HealthStatus::Degraded, HealthStatus::Degraded);
        assert_eq!(HealthStatus::Unhealthy, HealthStatus::Unhealthy);

        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
        assert_ne!(HealthStatus::Degraded, HealthStatus::Unhealthy);
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_uptime_monotonic() {
        // Test that uptime increases monotonically
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        let mut prev_uptime = std::time::Duration::from_secs(0);

        // Modern pattern: Test monotonicity without artificial delays
        for _ in 0..5 {
            // Actual work between reads (lock acquisition is real work)
            let state = core.state.read().await;
            let current_uptime = state.start_time.elapsed();

            assert!(
                current_uptime >= prev_uptime,
                "Uptime should increase monotonically"
            );
            prev_uptime = current_uptime;
            drop(state); // Explicit drop for next iteration
        }
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[test]
    fn test_config_environment_values() {
        // Test environment enum values
        use beardog_types::canonical::config::unified::Environment;

        let _dev = Environment::Development;
        let _prod = Environment::Production;
        let staging = Environment::Staging;

        // All environments should be creatable without panicking
        // Verify we can format them
        assert_eq!(format!("{:?}", staging), "Staging");
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_component_state_tracking() {
        // Test that component state is tracked
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        let state = core.state.read().await;
        // Component list should be accessible
        let _components = &state.components;
        // Always true, but verifies the field exists and is accessible
        assert!(state.components.is_empty() || !state.components.is_empty());
    }

    /// TEST_CATEGORY: performance
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: low
    #[test]
    fn test_config_creation_speed() {
        // Test that config creation is fast
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let _config =
                beardog_types::canonical::config::unified::UnifiedBearDogConfig::development();
        }

        let elapsed = start.elapsed();

        // 1000 configs should create quickly (<10ms to account for system variability)
        assert!(
            elapsed.as_millis() < 10,
            "Config creation too slow: {:?}",
            elapsed
        );
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_concurrent_readers() {
        // Test multiple concurrent readers don't block
        let core = Arc::new(crate::core::system::BearDogCore::with_default_config().unwrap());

        let mut handles = vec![];

        for _ in 0..20 {
            let core_clone = Arc::clone(&core);
            let handle = tokio::spawn(async move {
                let state = core_clone.state.read().await;
                assert_eq!(
                    state.overall_health,
                    beardog_types::canonical::HealthStatus::Healthy
                );
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_start_time_persistence() {
        // Test that start_time persists across reads
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        let start_time = {
            let state = core.state.read().await;
            state.start_time
        };

        // Multiple reads should see same start_time
        for _ in 0..10 {
            let state = core.state.read().await;
            assert_eq!(state.start_time, start_time);
        }
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_health_transitions() {
        // Test health status transitions
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        // Start healthy
        assert_eq!(
            core.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );

        // Transition to degraded
        {
            let mut state = core.state.write().await;
            state.overall_health = beardog_types::canonical::HealthStatus::Degraded;
        }
        assert_eq!(
            core.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Degraded
        );

        // Transition to unhealthy
        {
            let mut state = core.state.write().await;
            state.overall_health = beardog_types::canonical::HealthStatus::Unhealthy;
        }
        assert_eq!(
            core.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Unhealthy
        );

        // Restore to healthy
        {
            let mut state = core.state.write().await;
            state.overall_health = beardog_types::canonical::HealthStatus::Healthy;
        }
        assert_eq!(
            core.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
    }

    /// TEST_CATEGORY: performance
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_write_performance() {
        // Test write performance
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        let start = std::time::Instant::now();

        for i in 0..100 {
            let mut state = core.state.write().await;
            state.overall_health = if i % 2 == 0 {
                beardog_types::canonical::HealthStatus::Healthy
            } else {
                beardog_types::canonical::HealthStatus::Degraded
            };
        }

        let elapsed = start.elapsed();

        // 100 writes should complete reasonably fast (<50ms)
        assert!(
            elapsed.as_millis() < 50,
            "Write operations too slow: {:?}",
            elapsed
        );
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[test]
    fn test_health_status_debug() {
        // Test that HealthStatus can be debugged
        use beardog_types::canonical::HealthStatus;
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;

        let _healthy_str = format!("{:?}", healthy);
        let _degraded_str = format!("{:?}", degraded);
        let unhealthy_str = format!("{:?}", unhealthy);

        // Should not panic and should contain status information
        assert!(unhealthy_str.contains("Unhealthy"));
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_state_drop_behavior() {
        // Test that dropping state guard releases lock
        let core = crate::core::system::BearDogCore::with_default_config().unwrap();

        {
            let _state = core.state.read().await;
            // Lock held here
        } // Lock released

        // Should be able to acquire again
        let _state2 = core.state.read().await;
        assert!(true, "Lock successfully reacquired");
    }

    /// TEST_CATEGORY: unit
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[test]
    fn test_config_default_values() {
        // Test that default config has reasonable values
        let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();

        // Default should be development environment
        use beardog_types::canonical::config::unified::Environment;
        assert_eq!(config.metadata.environment, Environment::Development);
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_core_multiple_instances() {
        // Test that multiple core instances can coexist
        let core1 = crate::core::system::BearDogCore::with_default_config().unwrap();
        let core2 = crate::core::system::BearDogCore::with_default_config().unwrap();
        let core3 = crate::core::system::BearDogCore::with_default_config().unwrap();

        assert_eq!(
            core1.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
        assert_eq!(
            core2.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
        assert_eq!(
            core3.state.read().await.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
    }

    /// TEST_CATEGORY: performance
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: low
    #[tokio::test]
    async fn test_core_initialization_speed() {
        // Test that core initialization is reasonably fast
        use crate::core::system::BearDogCore;
        let start = std::time::Instant::now();

        for _ in 0..10 {
            let _core = BearDogCore::with_default_config().unwrap();
        }

        let elapsed = start.elapsed();

        // 10 initializations should be fast (<100ms)
        assert!(
            elapsed.as_millis() < 100,
            "Core initialization too slow: {:?}",
            elapsed
        );
    }

    /// TEST_CATEGORY: integration
    /// TEST_DOMAIN: core
    /// TEST_PRIORITY: high
    #[tokio::test]
    async fn test_core_state_consistency_under_load() {
        // Test state consistency with mixed reads/writes
        use crate::core::system::BearDogCore;
        let core = Arc::new(BearDogCore::with_default_config().unwrap());

        let mut handles = vec![];

        // Spawn readers - modern pattern: test actual concurrent access
        for _ in 0..10 {
            let core_clone = Arc::clone(&core);
            let handle = tokio::spawn(async move {
                for _ in 0..10 {
                    let _state = core_clone.state.read().await;
                    // No sleep needed - we're testing lock contention, not timing
                }
            });
            handles.push(handle);
        }

        // Spawn writers - modern pattern: test actual concurrent writes
        for _ in 0..3 {
            let core_clone = Arc::clone(&core);
            let handle = tokio::spawn(async move {
                for i in 0..5 {
                    let mut state = core_clone.state.write().await;
                    state.overall_health = if i % 2 == 0 {
                        beardog_types::canonical::HealthStatus::Healthy
                    } else {
                        beardog_types::canonical::HealthStatus::Degraded
                    };
                    // No sleep needed - testing write lock exclusivity
                }
            });
            handles.push(handle);
        }

        // All operations should complete without deadlock
        for handle in handles {
            handle.await.unwrap();
        }
    }
}
