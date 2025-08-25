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


/// # Canonical Security Configuration Module
///
/// This module provides canonical security configuration types including
/// MFA, authentication, rate limiting, and session management.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL SECURITY CONFIGURATION** - Main security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityConfig {
    /// Multi-factor authentication settings
    pub mfa: MfaConfig,
    /// Password policy settings
    pub password_policy: PasswordPolicyConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Session management configuration
    pub session: SessionConfig,
}


/// **CANONICAL MFA CONFIGURATION** - Multi-factor authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct MfaConfig {
    /// Whether MFA is enabled
    pub enabled: bool,
    /// TOTP configuration
    pub totp: TotpConfig,
    /// Backup codes configuration
    pub backup_codes: BackupCodesConfig,
}


/// TOTP (Time-based One-Time Password) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    /// TOTP window size in seconds
    pub window_size: u32,
    /// Number of previous/future windows to accept
    pub window_tolerance: u32,
    /// Secret key length in bytes
    pub secret_length: usize,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            window_size: 30,
            window_tolerance: 1,
            secret_length: 32,
        }
    }
}

/// Backup codes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {
    /// Number of backup codes to generate
    pub count: usize,
    /// Length of each backup code
    pub length: usize,
}

impl Default for BackupCodesConfig {
    fn default() -> Self {
        Self {
            count: 10,
            length: 8,
        }
    }
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {
    /// Minimum password length
    pub min_length: usize,
    /// Require uppercase letters
    pub require_uppercase: bool,
    /// Require lowercase letters
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special: bool,
}

impl Default for PasswordPolicyConfig {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window for rate limiting
    pub window: Duration,
    /// Burst allowance
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            burst: 10,
        }
    }
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Enable secure cookies
    pub secure_cookies: bool,
    /// Session storage type
    pub storage: SessionStorage,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3600), // 1 hour
            secure_cookies: true,
            storage: SessionStorage::Memory,
        }
    }
}

/// Session storage type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorage {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis,
    /// Database storage
    Database,
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self::Memory
    }
}

// Type aliases for compatibility
pub use MfaConfig as AuthenticationConfig;
pub use SecurityConfig as UnifiedSecurityConfig;
