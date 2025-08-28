

use crate::canonical::MfaConfig;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::crypto::{AesMode, CryptoAlgorithm, EcCurve};
pub use beardog_traits::canonical::SecurityProvider;

pub use crate::canonical::AuthenticationConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,};

    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
}

pub struct AuthenticationResult {
    pub success: bool,
    pub user_id: Option<String>,
    pub session_token: Option<String>,
    pub error_message: Option<String>,
    pub requires_mfa: bool,

pub use crate::canonical::configuration::security::EncryptionConfig;

pub use crate::canonical::configuration::security::RateLimitConfig;

pub struct TokenConfig {

    pub jwt_secret: String,

    pub token_expiration_seconds: u64,

    pub enable_refresh: bool,

    pub refresh_expiration_seconds: u64,}

impl Default for TokenConfig {}

    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            token_expiration_seconds: 3600, // 1 hour
            enable_refresh: true,
            refresh_expiration_seconds: 86400, // 24 hours
        }
    }

pub use crate::canonical::configuration::security::UnifiedAuthConfig;

pub use crate::canonical::SecurityAuditEvent;

pub enum SecurityEventType {
    Login,
    Logout,
    AuthenticationFailure,
    PasswordChange,
    MfaSetup,
    MfaVerification,
    TokenGeneration,
    TokenRevocation,
    PermissionGranted,
    PermissionDenied,
    DataAccess,
    DataModification,
    ConfigurationChange,

pub use crate::canonical::providers::ProviderConfig as SecurityProviderConfig;
