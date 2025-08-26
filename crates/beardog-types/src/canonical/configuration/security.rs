

use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::config::UnifiedAuthConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityConfig {

    pub mfa: MfaConfig,

    pub password_policy: PasswordPolicyConfig,

    pub rate_limiting: RateLimitConfig,

    pub session: UnifiedAuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct MfaConfig {

    pub enabled: bool,

    pub totp: TotpConfig,

    pub backup_codes: BackupCodesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {

    pub window_size: u32,

    pub window_tolerance: u32,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {

    pub count: usize,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {

    pub min_length: usize,

    pub require_uppercase: bool,

    pub require_lowercase: bool,

    pub require_numbers: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {

    pub max_requests: u32,

    pub window: Duration,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
pub struct SessionConfig {

    pub timeout: Duration,

    pub secure_cookies: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorage {

    Memory,

    Redis,

    Database,
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self::Memory
    }
}

pub use MfaConfig as AuthenticationConfig;
pub use SecurityConfig as UnifiedSecurityConfig;
