//! # Compliance Configuration Domain

use crate::canonical::config::r#trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ComplianceConfiguration {
    /// Whether compliance monitoring is enabled
    pub enabled: bool,
    /// Whether audit logging for compliance is enabled
    pub audit_enabled: bool,
}


impl BearDogConfig for ComplianceConfiguration {
    fn validate(&self) -> BearDogResult<()> { Ok(()) }
    fn to_toml(&self) -> BearDogResult<String> { 
        toml::to_string(self).map_err(|e| BearDogError::system(e.to_string()))
    }
    fn from_env() -> BearDogResult<Self> { Ok(Self::default()) }
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { Ok(self.clone()) }
    fn domain() -> &'static str { "compliance" }
} 