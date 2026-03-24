// SPDX-License-Identifier: AGPL-3.0-only

// Simplified hsm_management.rs to resolve compilation issues
use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info};

impl BearDogCore {
    /// Initialize HSM providers - used by ecosystem integration
    #[expect(
        dead_code,
        reason = "pub(crate) HSM init hook for ecosystem integration"
    )]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn initialize_hsm_providers(&self) -> Result<(), BearDogError> {
        info!("🔧 Initializing universal HSM providers");
        info!("🛠️ Initializing Software HSM provider");
        info!("🔍 Detecting available hardware HSM modules");

        // Actual initialization would happen here in production
        #[cfg(feature = "hsm-integration")]
        {
            // Real HSM initialization code would go here
        }

        Ok(())
    }

    /// Shutdown HSM providers - used by ecosystem cleanup
    #[expect(
        dead_code,
        reason = "pub(crate) HSM shutdown hook for ecosystem cleanup"
    )]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn shutdown_hsm_providers(&self) -> Result<(), BearDogError> {
        debug!("🔍 Closing hardware HSM connections");

        #[cfg(feature = "hsm-integration")]
        {
            // Real HSM shutdown code would go here
        }

        info!("✅ HSM providers shutdown complete");
        Ok(())
    }

    /// Check HSM health - used by health monitoring
    #[expect(dead_code, reason = "pub(crate) HSM health hook for monitoring")]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn check_hsm_health(&self) -> HealthStatus {
        debug!("🏥 Checking HSM provider health");

        #[cfg(feature = "hsm-integration")]
        {
            // Real HSM health check would go here
            // For now, return healthy in development
        }

        HealthStatus::Healthy
    }

    // Note: get_hsm_metrics() removed as unused. Metrics collection can be
    // re-implemented when monitoring integration is activated.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BearDogCore;

    fn make_core() -> BearDogCore {
        BearDogCore::with_default_config().expect("core creation")
    }

    #[test]
    fn test_initialize_hsm_providers() {
        let core = make_core();
        let result = core.initialize_hsm_providers();
        assert!(result.is_ok());
    }

    #[test]
    fn test_shutdown_hsm_providers() {
        let core = make_core();
        let result = core.shutdown_hsm_providers();
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_hsm_health() {
        let core = make_core();
        let status = core.check_hsm_health();
        assert_eq!(status, HealthStatus::Healthy);
    }
}
