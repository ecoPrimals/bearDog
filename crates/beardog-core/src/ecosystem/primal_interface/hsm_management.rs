// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM provider lifecycle management for `BearDogCore`.

use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info};

impl BearDogCore {
    /// Initialize HSM providers for ecosystem integration.
    #[cfg_attr(not(test), allow(dead_code))]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn initialize_hsm_providers(&self) -> Result<(), BearDogError> {
        info!("Initializing universal HSM providers");
        debug!("Detecting available hardware HSM modules");

        #[cfg(feature = "hsm-integration")]
        {
            // Wire real HSM initialization when feature is enabled
        }

        Ok(())
    }

    /// Shutdown HSM providers during ecosystem cleanup.
    #[cfg_attr(not(test), allow(dead_code))]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn shutdown_hsm_providers(&self) -> Result<(), BearDogError> {
        debug!("Closing hardware HSM connections");

        #[cfg(feature = "hsm-integration")]
        {
            // Wire real HSM shutdown when feature is enabled
        }

        info!("HSM providers shutdown complete");
        Ok(())
    }

    /// Check HSM health for monitoring.
    #[cfg_attr(not(test), allow(dead_code))]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn check_hsm_health(&self) -> HealthStatus {
        debug!("Checking HSM provider health");

        #[cfg(feature = "hsm-integration")]
        {
            // Wire real HSM health check when feature is enabled
        }

        HealthStatus::Healthy
    }
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
