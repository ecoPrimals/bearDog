// SPDX-License-Identifier: AGPL-3.0-only

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
            rp_id: "beardog.ecoPrimals".to_string(),
            rp_name: "BearDog Security Platform".to_string(),
            require_user_verification: false,
            timeout_ms: 30000,
        }
    }
}
