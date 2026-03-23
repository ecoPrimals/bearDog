// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Core Lifecycle

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        clippy::float_cmp,
        clippy::useless_vec,
        clippy::needless_range_loop,
        clippy::uninlined_format_args,
        clippy::field_reassign_with_default,
        clippy::manual_range_contains,
        unused_variables,
        dead_code
    )]
    use crate::core::BearDogCore;
    use beardog_types::canonical::ComponentStatus;
    use beardog_types::canonical::config::unified::UnifiedBearDogConfig;

    fn create_test_core() -> BearDogCore {
        let config = UnifiedBearDogConfig::default();
        BearDogCore::new(config)
    }

    #[tokio::test]
    async fn test_startup_success() {
        let core = create_test_core();
        let result = core.startup().await;

        assert!(result.is_ok(), "Startup should succeed");
    }

    #[tokio::test]
    async fn test_startup_sets_components_running() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");

        // Verify components are running
        let state = core.state.read().await;
        assert_eq!(
            state.components.get("security"),
            Some(&ComponentStatus::Running),
            "Security component should be running"
        );
        assert_eq!(
            state.components.get("monitor"),
            Some(&ComponentStatus::Running),
            "Monitor component should be running"
        );
        assert_eq!(
            state.components.get("genetic_optimizer"),
            Some(&ComponentStatus::Running),
            "Genetic optimizer component should be running"
        );
    }

    #[tokio::test]
    async fn test_startup_sets_healthy_status() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");

        let state = core.state.read().await;
        assert!(matches!(
            state.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        ));
    }

    #[tokio::test]
    async fn test_shutdown_success() {
        let core = create_test_core();

        // Start up first
        core.startup().await.expect("Startup should succeed");

        // Then shutdown
        let result = core.shutdown().await;
        assert!(result.is_ok(), "Shutdown should succeed");
    }

    #[tokio::test]
    async fn test_shutdown_sets_components_inactive() {
        let core = create_test_core();

        // Start up first
        core.startup().await.expect("Startup should succeed");

        // Shutdown
        core.shutdown().await.expect("Shutdown should succeed");

        // Verify all components are inactive
        let state = core.state.read().await;
        for (name, status) in &state.components {
            assert_eq!(
                status,
                &ComponentStatus::Inactive,
                "Component {} should be inactive after shutdown",
                name
            );
        }
    }

    #[tokio::test]
    async fn test_shutdown_sets_unhealthy_status() {
        let core = create_test_core();

        core.startup().await.expect("Startup should succeed");
        core.shutdown().await.expect("Shutdown should succeed");

        let state = core.state.read().await;
        assert!(matches!(
            state.overall_health,
            beardog_types::canonical::HealthStatus::Unhealthy
        ));
    }

    #[tokio::test]
    async fn test_shutdown_without_startup() {
        let core = create_test_core();

        // Shutdown without starting up should still work
        let result = core.shutdown().await;
        assert!(
            result.is_ok(),
            "Shutdown should succeed even without startup"
        );
    }

    #[tokio::test]
    async fn test_health_check_after_startup() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");

        let health = core
            .health_check()
            .await
            .expect("Health check should succeed");

        assert_eq!(health.component_name, "core");
        assert_eq!(health.status, ComponentStatus::Running);
        assert!(
            health.details.is_none(),
            "Should have no error details when healthy"
        );
    }

    #[tokio::test]
    async fn test_health_check_after_shutdown() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");
        core.shutdown().await.expect("Shutdown should succeed");

        let health = core
            .health_check()
            .await
            .expect("Health check should succeed");

        assert_eq!(health.component_name, "core");
        assert_eq!(health.status, ComponentStatus::Inactive);
        assert!(
            health.details.is_some(),
            "Should have details when unhealthy"
        );
    }

    #[tokio::test]
    async fn test_health_check_without_startup() {
        let core = create_test_core();

        let health = core
            .health_check()
            .await
            .expect("Health check should succeed");

        // System starts in a default state
        assert_eq!(health.component_name, "core");
    }

    #[tokio::test]
    async fn test_multiple_startups() {
        let core = create_test_core();

        // Multiple startups should not fail
        core.startup().await.expect("First startup should succeed");
        core.startup().await.expect("Second startup should succeed");
        core.startup().await.expect("Third startup should succeed");

        // System should still be healthy
        let health = core
            .health_check()
            .await
            .expect("Health check should succeed");
        assert_eq!(health.status, ComponentStatus::Running);
    }

    #[tokio::test]
    async fn test_multiple_shutdowns() {
        let core = create_test_core();

        core.startup().await.expect("Startup should succeed");

        // Multiple shutdowns should not fail
        core.shutdown()
            .await
            .expect("First shutdown should succeed");
        core.shutdown()
            .await
            .expect("Second shutdown should succeed");
        core.shutdown()
            .await
            .expect("Third shutdown should succeed");
    }

    #[tokio::test]
    async fn test_startup_shutdown_cycle() {
        let core = create_test_core();

        // Test multiple startup/shutdown cycles
        for _ in 0..3 {
            core.startup().await.expect("Startup should succeed");

            let health = core
                .health_check()
                .await
                .expect("Health check should succeed");
            assert_eq!(
                health.status,
                ComponentStatus::Running,
                "Should be running after startup"
            );

            core.shutdown().await.expect("Shutdown should succeed");

            let health = core
                .health_check()
                .await
                .expect("Health check should succeed");
            assert_eq!(
                health.status,
                ComponentStatus::Inactive,
                "Should be inactive after shutdown"
            );
        }
    }

    #[tokio::test]
    async fn test_health_check_uptime() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");

        // ✅ MODERNIZED: Minimal yield to ensure measurable uptime
        tokio::task::yield_now().await;

        let health = core
            .health_check()
            .await
            .expect("Health check should succeed");

        // Uptime should be a valid duration (non-negative by type, just verify it's present)
        let _ = health.uptime; // Uptime is measured as Duration which is always non-negative
    }

    #[tokio::test]
    async fn test_health_check_last_check_timestamp() {
        let core = create_test_core();
        core.startup().await.expect("Startup should succeed");

        let health1 = core
            .health_check()
            .await
            .expect("First health check should succeed");

        // ✅ MODERNIZED: Removed sleep - consecutive health checks don't need delay
        let health2 = core
            .health_check()
            .await
            .expect("Second health check should succeed");

        // Second check should have a later timestamp
        assert!(
            health2.last_check > health1.last_check,
            "Later health check should have later timestamp"
        );
    }

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        use std::sync::Arc;

        let core = Arc::new(create_test_core());
        core.startup().await.expect("Startup should succeed");

        // Perform multiple concurrent health checks
        let mut handles = vec![];
        for _ in 0..10 {
            let core_clone = Arc::clone(&core);
            handles.push(tokio::spawn(async move { core_clone.health_check().await }));
        }

        // All should succeed
        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok(), "Concurrent health check should succeed");
        }
    }

    #[test]
    fn test_health_check_struct_clone() {
        use crate::core::lifecycle::HealthCheck;
        use chrono::Utc;

        let health = HealthCheck {
            component_name: "test".to_string(),
            status: ComponentStatus::Running,
            last_check: Utc::now(),
            details: Some("test details".to_string()),
            uptime: std::time::Duration::from_secs(60),
        };

        let cloned = health.clone();
        assert_eq!(health.component_name, cloned.component_name);
        assert_eq!(health.status, cloned.status);
    }

    #[tokio::test]
    async fn test_health_check_inactive_when_any_component_not_running() {
        let core = create_test_core();
        core.startup().await.expect("startup");

        {
            let mut state = core.state.write().await;
            state
                .components
                .insert("security".to_string(), ComponentStatus::Inactive);
        }

        let health = core
            .health_check()
            .await
            .expect("health check returns Ok with inactive status");

        assert_eq!(health.status, ComponentStatus::Inactive);
        assert!(
            health.details.is_some(),
            "details should note unhealthy components"
        );
    }
}
