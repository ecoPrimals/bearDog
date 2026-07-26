// SPDX-License-Identifier: AGPL-3.0-or-later

/// Default relying party identifier for `BearDog` deployments.
pub const DEFAULT_RP_ID: &str = "beardog.ecoPrimals";

/// Default human-readable relying party name.
pub const DEFAULT_RP_NAME: &str = "BearDog Security Platform";

/// Default CTAP2 operation timeout (milliseconds).
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// Configuration for FIDO2 provider
#[derive(Debug, Clone)]
pub struct Fido2ProviderConfig {
    /// RP ID (Relying Party Identifier) - typically your domain
    pub rp_id: String,

    /// RP name (human-readable)
    pub rp_name: String,

    /// Whether to require user verification by default
    pub require_user_verification: bool,

    /// Default timeout for operations (milliseconds)
    pub timeout_ms: u64,
}

impl Default for Fido2ProviderConfig {
    fn default() -> Self {
        Self {
            rp_id: DEFAULT_RP_ID.to_string(),
            rp_name: DEFAULT_RP_NAME.to_string(),
            require_user_verification: false,
            timeout_ms: DEFAULT_TIMEOUT_MS,
        }
    }
}
