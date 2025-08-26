

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSecurityConfig {
    pub encryption_enabled: bool,
    pub authentication_required: bool,
    pub authorization_enabled: bool,
    pub audit_logging: bool,
}

impl Default for BasicSecurityConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: true,
            authentication_required: true,
            authorization_enabled: true,
            audit_logging: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedSecurityConfig {
    pub basic: BasicSecurityConfig,
    pub crypto: CryptoOptimizationConfig,
    pub authentication: AuthenticationConfig,
    pub authorization: AuthorizationConfig,
    pub hsm: HsmConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoOptimizationConfig {
    pub algorithm: String,
    pub key_size: usize,
    pub enable_hardware_acceleration: bool,
    pub cache_keys: bool,
}

impl Default for CryptoOptimizationConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
            enable_hardware_acceleration: true,
            cache_keys: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub method: String,
    pub token_lifetime: Duration,
    pub max_attempts: u32,
    pub lockout_duration: Duration,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            method: "jwt".to_string(),
            token_lifetime: Duration::from_secs(3600),
            max_attempts: 3,
            lockout_duration: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    pub enabled: bool,
    pub default_policy: String,
    pub cache_decisions: bool,
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_policy: "deny".to_string(),
            cache_decisions: true,
        }
    }
}

pub use crate::canonical::hsm::config::HsmConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration: Duration,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "change-me".to_string(),
            issuer: "beardog".to_string(),
            audience: "beardog-users".to_string(),
            expiration: Duration::from_secs(3600),
        }
    }
} 
