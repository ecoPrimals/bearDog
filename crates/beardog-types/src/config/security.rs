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


/// Security Configuration Types
///
/// Contains all security-related configuration structures.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Import canonical types to avoid conflicts
use crate::canonical::configuration::security::{EncryptionConfig, MfaConfig};
/// Security configuration with all required fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// JWT secret for token signing
    pub jwt_secret: Option<String>,
    /// Force HTTPS for all connections
    pub force_https: bool,
    /// HSM provider identifier
    pub hsm_provider: Option<String>,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Comprehensive MFA configuration
    pub mfa_config: Option<MfaConfig>,
    /// Session timeout in seconds
    pub session_timeout_secs: u64,
    /// Maximum failed login attempts
    pub max_failed_attempts: u32,
    /// Account lockout duration in seconds
    pub lockout_duration_secs: u64,
    /// Password policy configuration
    pub password_policy: Option<PasswordPolicyConfig>,
    /// Rate limiting configuration
    pub rate_limiting: Option<SecurityRateLimitConfig>,
    /// Encryption configuration
    pub encryption: Option<EncryptionConfig>,
    /// Authentication providers
    pub auth_providers: Vec<AuthProviderConfig>,
    /// Security headers configuration
    pub security_headers: Option<SecurityHeadersConfig>,
}
/// Password policy configuration
pub use crate::canonical::configuration::security::PasswordPolicyConfig;
/// Security rate limiting configuration
pub struct SecurityRateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
    /// Blocked duration in seconds
    pub block_duration_secs: u64,
/// **COMPREHENSIVE CANONICAL ENCRYPTION CONFIGURATION**
/// **CANONICAL ENCRYPTION CONFIGURATION** - Single source of truth
/// This consolidates features from 5+ duplicate encryption configurations:
/// - Network TLS/certificates, key exchange
/// - Security algorithms, key derivation, rotation  
/// - Secret protection encryption
/// - HSM-specific encryption
/// - Basic config with HSM support
/// ## Comprehensive Feature Set
/// - **Algorithm Support**: AES-256-GCM, ChaCha20-Poly1305, RSA, Ed25519
/// - **Key Management**: Generation, rotation, derivation, storage
/// - **Transport Security**: TLS, certificates, key exchange methods
/// - **Hardware Integration**: HSM support, hardware acceleration
/// - **Secret Protection**: At-rest and in-transit encryption for sensitive data
/// - **Enterprise Features**: Compliance, auditing, key escrow
/// Use `beardog_types::canonical::EncryptionConfig` directly to avoid import conflicts.
// Re-export removed to eliminate namespace conflicts - use canonical::EncryptionConfig directly
/// Encryption profile for specific use cases};


pub struct EncryptionProfile {
    /// Profile name
    pub name: String,
    /// Algorithm for this profile
    pub algorithm: String,
    /// Key size for this profile
    pub key_size: u32,
    /// Profile-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
// Default implementation removed - using canonical implementation from crate::canonical
/// Authentication provider configuration
pub struct AuthProviderConfig {
    /// Provider name
    /// Provider type (local, ldap, oauth, etc.)
    pub provider_type: String,
    /// Provider-specific configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Whether this provider is enabled
    /// Priority order (lower numbers = higher priority)
    pub priority: u32,
/// Security headers configuration}


pub struct SecurityHeadersConfig {
    /// Enable security headers
    /// Content Security Policy
    pub content_security_policy: Option<String>,
    /// X-Frame-Options
    pub x_frame_options: Option<String>,
    /// X-Content-Type-Options
    pub x_content_type_options: bool,
    /// X-XSS-Protection
    pub x_xss_protection: bool,
    /// Strict-Transport-Security
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
        let mut config = HashMap::new();
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
/// **CANONICAL MFA CONFIGURATION** - Single source of truth
/// This consolidates 4+ duplicate MFA configurations:
/// - auth_types: method enums, grace period, token management
/// - security types: basic enabled/methods structure
/// - hsm config: security factors, combination strategies
/// - config/security: TOTP config, hardware tokens
/// - **Method Management**: Support for TOTP, SMS, Email, Hardware Tokens, Biometric
/// - **Security Factors**: Knowledge, Possession, Inherence factor classification
/// - **Combination Strategies**: Flexible factor combination requirements
/// - **Configuration Details**: Detailed settings for each MFA method
/// - **Token Management**: Token lifecycle and validation settings
/// - **Enterprise Features**: Grace periods, backup methods, audit logging
/// Use `beardog_types::canonical::MfaConfig` directly to avoid import conflicts.
// Re-export removed to eliminate namespace conflicts - use canonical::MfaConfig directly
/// **UNIFIED MFA METHOD ENUM** - Consolidates all method variants
/// Combines MfaMethod variants from:
/// - auth_types: Totp, Sms, Email
/// - config/security: TOTP, SMS, Email, HardwareToken
/// - Plus enterprise additions: Biometric, Push}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP)
    Totp,
    /// SMS-based token authentication
    Sms,
    /// Email-based token authentication
    Email,
    /// Hardware token (YubiKey, RSA SecurID, etc.)
    HardwareToken,
    /// Biometric authentication (fingerprint, face, etc.)
    Biometric,
    /// Push notification authentication
    Push,
    /// Backup recovery codes
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
/// **AUTHENTICATION FACTORS** - From HSM implementation
/// Security-focused classification of authentication factors}


pub enum AuthFactor {
    /// Something you know (PIN/password/knowledge)
    Knowledge,
    /// Something you have (device/token/possession)
    Possession,
    /// Something you are (biometric/inherence)
    Inherence,
/// **MFA COMBINATION STRATEGIES** - From HSM implementation
/// Advanced strategies for combining multiple authentication factors}


pub enum MfaCombinationStrategy {
    /// All specified factors required
    All,
    /// Any N factors required (e.g., any 2 of 3)
    AnyN(u32),
    /// Specific combination of factors required
    Specific(Vec<AuthFactor>),
    /// At least one from each category
    OneFromEach,
/// **TOTP CONFIGURATION** - Use canonical implementation  
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TotpConfig {
//     /// Issuer name for TOTP apps
//     pub issuer: String,
//     /// Time step in seconds (usually 30)
//     pub time_step: u32,
//     /// Code length (usually 6)
//     pub code_length: usize,
//     /// Algorithm (SHA1, SHA256, SHA512)
//     pub algorithm: String,
//     /// Window size for clock skew tolerance
//     pub window_size: u32,
// }
pub use crate::canonical::configuration::security::TotpConfig;
/// **SMS CONFIGURATION** - Enhanced for enterprise use
pub struct SmsConfig {
    /// SMS provider (twilio, aws_sns, etc.)
    pub provider: String,
    /// SMS template for tokens
    pub message_template: String,
    /// Token length for SMS codes
    pub token_length: usize,
    /// Rate limiting for SMS sends
    pub rate_limit_per_hour: u32,
/// **EMAIL CONFIGURATION** - Enhanced for enterprise use};


pub struct EmailConfig {
    /// Email provider configuration
    /// Email template for tokens
    pub template: String,
    /// Token length for email codes
    /// Rate limiting for email sends
/// **HARDWARE TOKEN CONFIGURATION** - From config/security
pub struct HardwareTokenConfig {
    /// Supported token types (yubikey, rsa_securid, etc.)
    pub supported_types: Vec<String>,
    /// Challenge-response timeout
    pub challenge_timeout: Duration,
    /// Token validation settings
    pub validation_config: HashMap<String, serde_json::Value>,
/// **BIOMETRIC CONFIGURATION** - New enterprise feature
pub struct BiometricConfig {
    /// Supported biometric types (fingerprint, face, voice, etc.)
    /// Biometric matching threshold (0.0-1.0)
    pub match_threshold: f64,
    /// Fallback methods when biometric fails
    pub fallback_methods: Vec<MfaMethod>,
/// **MFA TOKEN INFORMATION** - From auth_types implementation
pub struct MfaToken {
    /// Token identifier
    pub id: String,
    /// MFA method used to generate this token
    pub method: MfaMethod,
    /// Encrypted token value
    pub token: String,
    /// Token expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Whether the token has been used
    pub used: bool,
    /// User ID associated with this token
    pub user_id: String,
    /// Number of verification attempts
    pub attempts: u32,
// impl Default for TotpConfig {
//     fn default() -> Self {
//         Self {
//             issuer: "`BearDog` Security".to_string(),
//             time_step: 30,
//             code_length: 6,
//             algorithm: "SHA1".to_string(),
//             window_size: 1,
//         }
//     }
