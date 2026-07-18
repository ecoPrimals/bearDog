// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy quality classification and source reporting.

use super::config::{OS_RNG_FALLBACK_DEVICE, OS_RNG_FALLBACK_TIER, OS_RNG_SOURCE};
use super::discovery::HsmSource;
use super::orchestrator::HsmEntropyOrchestrator;

/// Metadata describing how entropy was actually produced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct EntropySourceReport {
    pub(super) hardware_backed: bool,
    pub(super) source: &'static str,
    pub(super) device_used: &'static str,
    pub(super) quality_tier: u8,
    pub(super) quality_score: f64,
}

pub(super) const fn os_rng_fallback_report() -> EntropySourceReport {
    EntropySourceReport {
        hardware_backed: false,
        source: OS_RNG_SOURCE,
        device_used: OS_RNG_FALLBACK_DEVICE,
        quality_tier: OS_RNG_FALLBACK_TIER,
        quality_score: 0.35,
    }
}

impl HsmEntropyOrchestrator {
    /// Calculate quality tier based on HSM source and data
    pub(super) const fn calculate_quality_tier(
        &self,
        source: &HsmSource,
        _data_length: usize,
    ) -> u8 {
        match source {
            #[cfg(all(feature = "mobile", target_os = "android"))]
            HsmSource::Android => 3, // StrongBox = Tier 3

            #[cfg(all(feature = "mobile", target_os = "ios"))]
            HsmSource::IOS => 3, // Secure Enclave = Tier 3

            #[cfg(feature = "fido2")]
            HsmSource::Fido2(_) => 2, // FIDO2 = Tier 2 (can be Tier 3 with human input)

            #[allow(
                unreachable_patterns,
                reason = "Fallback when optional HsmSource variants are cfg-disabled; expect unfulfilled when all arms active"
            )]
            _ => 1,
        }
    }

    /// Calculate quality score from tier
    pub(super) const fn calculate_quality_score(&self, tier: u8) -> f64 {
        match tier {
            3 => 0.95,
            2 => 0.75,
            1 => 0.50,
            0 => 0.35, // OS RNG fallback tier
            _ => 0.40,
        }
    }

    /// Get hardware device name from source (only valid when hardware RNG is used)
    pub(super) fn get_device_name(&self, source: &HsmSource) -> String {
        match source {
            #[cfg(feature = "fido2")]
            HsmSource::Fido2(idx) => format!("FIDO2 Device #{}", idx + 1),

            #[cfg(all(feature = "mobile", target_os = "android"))]
            HsmSource::Android => "Android StrongBox".to_string(),

            #[cfg(all(feature = "mobile", target_os = "ios"))]
            HsmSource::IOS => "iOS Secure Enclave".to_string(),

            #[allow(
                unreachable_patterns,
                reason = "Fallback when optional HsmSource variants are cfg-disabled; expect unfulfilled when all arms active"
            )]
            _ => "Unknown Device".to_string(),
        }
    }
}
