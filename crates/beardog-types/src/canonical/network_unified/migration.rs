// Network Migration Utilities

use super::CanonicalNetworkConfig;
use beardog_errors::BearDogError;

/// Migrate From Legacy
pub fn migrate_from_legacy() -> Result<CanonicalNetworkConfig, BearDogError> {
    Ok(CanonicalNetworkConfig::default())
}
