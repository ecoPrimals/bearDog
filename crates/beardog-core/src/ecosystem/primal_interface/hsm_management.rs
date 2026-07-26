// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM provider lifecycle management for `BearDogCore`.

use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info};

impl BearDogCore {
    /// Initialize HSM providers for ecosystem integration.
    ///
    /// Without `hsm-integration` feature, this is a no-op (software-only crypto).
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "called only from ecosystem orchestration tests")
    )]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn initialize_hsm_providers(&self) -> Result<(), BearDogError> {
        info!("Initializing universal HSM providers (software-only mode)");
        debug!("No hardware HSM modules configured — using software crypto");
        Ok(())
    }

    /// Shutdown HSM providers during ecosystem cleanup.
    ///
    /// Without `hsm-integration` feature, this is a no-op.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "called only from ecosystem orchestration tests")
    )]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) fn shutdown_hsm_providers(&self) -> Result<(), BearDogError> {
        debug!("HSM providers shutdown (software-only — no connections to close)");
        Ok(())
    }

    /// Check HSM health for monitoring.
    ///
    /// Without `hsm-integration` feature, returns `Healthy` because the
    /// software-only crypto path has no hardware dependencies to fail.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "called only from ecosystem orchestration tests")
    )]
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
    pub(crate) const fn check_hsm_health(&self) -> HealthStatus {
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
