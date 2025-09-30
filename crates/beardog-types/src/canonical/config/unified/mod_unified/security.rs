//! # Security Configuration Domain
//!
//! Security and cryptographic configuration for `BearDog`.

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    /// Enable security features
    pub enabled: bool,
    /// Encryption algorithm
    pub encryption_algorithm: String,
    /// Key size in bits
    pub key_size: usize,
}

impl Default for SecurityConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
        }
    }
}

impl BearDogConfig for SecurityConfiguration {
    fn validate(&self) -> BearDogResult<()> {
        if self.key_size < 128 {
            return Err(BearDogError::system("Key size must be at least 128 bits".to_string()));
        }
        Ok(())
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize security config: {e}")))
    }

    fn from_env() -> BearDogResult<Self> {
        Ok(Self::default())
    }
    
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { 
        Ok(self.clone()) 
    }
    
    fn domain() -> &'static str { 
        "security" 
    }
} 