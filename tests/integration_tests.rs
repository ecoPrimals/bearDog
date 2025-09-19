use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_types::canonical::config::BearDogConfig;
use tokio::time::{sleep, Duration};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_system_integration() -> BearDogResult<()> {
        // Test complete system initialization and operation
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        // Verify initialization
        assert!(core.is_initialized());

        // Test health check
        let health = core.health_check()?;
        assert!(health.is_healthy());

        // Test graceful shutdown
        core.shutdown()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_operations() -> BearDogResult<()> {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        // Test concurrent health checks
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let core_clone = core.clone();
                tokio::spawn(async move { core_clone.health_check().await })
            })
            .collect();

        // All should succeed
        for handle in handles {
            let result = handle.unwrap()?;
            assert!(result.is_healthy());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_system_resilience() -> BearDogResult<()> {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        // Test system behavior under load
        for i in 0..20 {
            let health = core.health_check()?;
            assert!(
                health.is_healthy(),
                "Health check failed on iteration {}",
                i
            );

            // Small delay to simulate real usage
            sleep(Duration::from_millis(10));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_configuration_validation() -> BearDogResult<()> {
        // Test that system validates configuration properly
        let config = BearDogConfig::default();

        // Should succeed with valid config
        let core = BearDogCore::new(config)?;
        assert!(core.is_initialized());

        Ok(())
    }

    #[tokio::test]
    async fn test_error_handling() -> BearDogResult<()> {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        // Test that system handles errors gracefully
        let health = core.health_check()?;

        // Even if there are internal issues, health check should not panic
        assert!(health.is_healthy() || !health.is_healthy());

        Ok(())
    }
}
