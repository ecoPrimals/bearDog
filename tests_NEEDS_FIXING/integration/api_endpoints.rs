use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_types::canonical::config::BearDogConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_core_initialization() -> BearDogResult<()> {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        // Test that core initializes successfully
        assert!(core.is_initialized());

        Ok(())
    }

    #[tokio::test]
    async fn test_health_check_endpoint() -> BearDogResult<()> {
        let config = BearDogConfig::default();
        let core = BearDogCore::new(config)?;

        let health = core.health_check()?;
        assert!(health.is_healthy());

        Ok(())
    }
}
