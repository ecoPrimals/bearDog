// SPDX-License-Identifier: AGPL-3.0-only

// Monitoring Migration Utilities

use super::CanonicalMonitoringConfig;
use beardog_errors::BearDogError;

/// Migrate From Legacy
pub fn migrate_from_legacy() -> Result<CanonicalMonitoringConfig, BearDogError> {
    Ok(CanonicalMonitoringConfig::default())
}
