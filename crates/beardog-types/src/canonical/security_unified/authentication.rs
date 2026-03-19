// SPDX-License-Identifier: AGPL-3.0-only

// Authentication Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Methods
    /// Collection of methods
    pub methods: Vec<String>,
    /// Tokens
    /// The tokens value
    pub tokens: TokenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenConfig {
    /// Jwt Secret
    /// Optional jwt secret
    pub jwt_secret: Option<String>,
    /// Expiration Seconds
    /// Number of expiration_seconds
    pub expiration_seconds: u64,
}

impl AuthenticationConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
