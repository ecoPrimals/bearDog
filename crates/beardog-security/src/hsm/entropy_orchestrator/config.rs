// SPDX-License-Identifier: AGPL-3.0-or-later

//! Orchestrator configuration and OS RNG fallback constants.

use super::types::SecurityLevel;

/// Canonical identifier for OS RNG fallback entropy.
pub(super) const OS_RNG_SOURCE: &str = "os_rng";

/// Human-readable label when OS RNG fallback is used instead of hardware HSM RNG.
pub(super) const OS_RNG_FALLBACK_DEVICE: &str = "os_rng_fallback";

/// Lowest quality tier reserved for software-only (OS RNG) entropy.
pub(super) const OS_RNG_FALLBACK_TIER: u8 = 0;

/// Orchestrator configuration
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// Prefer biometric-capable devices
    pub prefer_biometric: bool,

    /// Minimum security level required
    pub min_security_level: SecurityLevel,

    /// Enable entropy mixing across multiple devices
    pub enable_multi_device_mixing: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            prefer_biometric: true,
            min_security_level: SecurityLevel::Hardware,
            enable_multi_device_mixing: false,
        }
    }
}
