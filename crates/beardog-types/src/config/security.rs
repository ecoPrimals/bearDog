

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::configuration::security::{EncryptionConfig, MfaConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {

    pub jwt_secret: Option<String>,

    pub force_https: bool,

    pub hsm_provider: Option<String>,

    pub enable_mfa: bool,

    pub mfa_config: Option<MfaConfig>,

    pub session_timeout_secs: u64,

    pub max_failed_attempts: u32,

    pub lockout_duration_secs: u64,

    pub password_policy: Option<PasswordPolicyConfig>,

    pub rate_limiting: Option<SecurityRateLimitConfig>,

    pub encryption: Option<EncryptionConfig>,

    pub auth_providers: Vec<AuthProviderConfig>,

    pub security_headers: Option<SecurityHeadersConfig>,
}

pub use crate::canonical::configuration::security::PasswordPolicyConfig;

pub struct SecurityRateLimitConfig {

    pub enabled: bool,

    pub requests_per_minute: u32,

    pub burst_size: u32,

    pub block_duration_secs: u64,

pub struct EncryptionProfile {

    pub name: String,

    pub algorithm: String,

    pub key_size: u32,

    pub parameters: HashMap<String, serde_json::Value>,

pub struct AuthProviderConfig {

    pub provider_type: String,

    pub config: HashMap<String, serde_json::Value>,

    pub priority: u32,

pub struct SecurityHeadersConfig {

    pub content_security_policy: Option<String>,

    pub x_frame_options: Option<String>,

    pub x_content_type_options: bool,

    pub x_xss_protection: bool,

    pub strict_transport_security: Option<String>,}

impl Default for SecurityConfig {}

    fn default() -> Self {
        Self {
            jwt_secret: None,
            force_https: false,
            hsm_provider: None,
            enable_mfa: false,
            mfa_config: Some(MfaConfig::default()),
            session_timeout_secs: 3600, // 1 hour
            max_failed_attempts: 5,
            lockout_duration_secs: 1800, // 30 minutes
            password_policy: Some(PasswordPolicyConfig::default()),
            rate_limiting: Some(SecurityRateLimitConfig::default()),
            encryption: Some(EncryptionConfig::default()),
            auth_providers: vec![AuthProviderConfig::default()],
            security_headers: Some(SecurityHeadersConfig::default()),
        }
    }
impl Default for SecurityRateLimitConfig {
            enabled: true,
            requests_per_minute: 60,
            burst_size: 10,
            block_duration_secs: 300, // 5 minutes}

impl Default for AuthProviderConfig {
        let mut config = HashMap::with_capacity(16);
        config.insert(
            "realm".to_string(),
            serde_json::Value::String("beardog".to_string()),
        );
            name: "local".to_string(),
            provider_type: "local".to_string(),
            config,
            priority: 1,
impl Default for SecurityHeadersConfig {
            content_security_policy: Some("default-src 'self'".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: true,
            x_xss_protection: true,
            strict_transport_security: Some("max-age=31536000; includeSubDomains".to_string()),

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MfaMethod {

    Totp,

    Sms,

    Email,

    HardwareToken,

    Biometric,

    Push,

    BackupCodes,}

impl std::fmt::Display for MfaMethod {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Totp => write!(f, "totp"),
            Self::Sms => write!(f, "sms"),
            Self::Email => write!(f, "email"),
            Self::HardwareToken => write!(f, "hardware_token"),
            Self::Biometric => write!(f, "biometric"),
            Self::Push => write!(f, "push"),
            Self::BackupCodes => write!(f, "backup_codes"),

pub enum AuthFactor {

    Knowledge,

    Possession,

    Inherence,

pub enum MfaCombinationStrategy {

    All,

    AnyN(u32),

    Specific(Vec<AuthFactor>),

    OneFromEach,

pub use crate::canonical::configuration::security::TotpConfig;

pub struct SmsConfig {

    pub provider: String,

    pub message_template: String,

    pub token_length: usize,

    pub rate_limit_per_hour: u32,

pub struct EmailConfig {

    pub template: String,

pub struct HardwareTokenConfig {

    pub supported_types: Vec<String>,

    pub challenge_timeout: Duration,

    pub validation_config: HashMap<String, serde_json::Value>,

pub struct BiometricConfig {

    pub match_threshold: f64,

    pub fallback_methods: Vec<MfaMethod>,

pub struct MfaToken {

    pub id: String,

    pub method: MfaMethod,

    pub token: String,

    pub expires_at: DateTime<Utc>,

    pub used: bool,

    pub user_id: String,

    pub attempts: u32,

