//! # Auth Configuration Domain

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Auth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfiguration {
    /// Whether authentication is enabled
    pub enabled: bool,
    /// List of authentication providers (e.g., "oauth", "ldap", "local")
    pub providers: Vec<String>,
}

impl Default for AuthConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            providers: vec!["local".to_string()],
        }
    }
}

impl BearDogConfig for AuthConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "auth" }
} 