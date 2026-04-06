// SPDX-License-Identifier: AGPL-3.0-or-later

// Security Migration Utilities

use super::CanonicalSecurityConfig;
use beardog_errors::BearDogError;

/// Migrate From Legacy
pub fn migrate_from_legacy() -> Result<CanonicalSecurityConfig, BearDogError> {
    Ok(CanonicalSecurityConfig::default())
}
