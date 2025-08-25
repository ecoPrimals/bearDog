// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Security Configuration - Canonical
///
/// **UNIFIED SECURITY CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Basic Security Configuration
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

/// **CANONICAL** Unified Security Configuration
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


/// **CANONICAL** Crypto Optimization Configuration
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

/// **CANONICAL** Authentication Configuration
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

/// **CANONICAL** Authorization Configuration
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

/// **CANONICAL** JWT Configuration
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
