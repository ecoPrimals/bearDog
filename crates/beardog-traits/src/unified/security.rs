// Unified Security Trait System
//
// This module provides comprehensive security traits that unify authentication,
// authorization, cryptography, and audit functionality.

use super::{BearDogProvider, UnifiedTraitError};
// async_trait no longer needed - using native fn
use beardog_types::canonical::config::r#trait::BearDogConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified security provider trait
pub trait SecurityProvider: BearDogProvider {
    /// Authentication result type
    type AuthResult: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Session type
    type Session: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Credentials type
    type Credentials: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Authenticate user with credentials
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl std::future::Future<Output = Result<Self::AuthResult, Self::Error>> + Send;

    /// Create secure session
    /// Creates session
    fn create_session(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<Self::Session, Self::Error>> + Send;

    /// Validate session
    /// Validates session
    fn validate_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// Revoke session
    fn revoke_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Authorize operation
    fn authorize(
        &self,
        session_id: &str,
        resource: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// Gets `security_requirements`
    fn get_security_requirements(
        &self,
        resource: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send;
}

/// Cryptographic provider trait
pub trait CryptoProvider: BearDogProvider {
    /// Key type
    type Key: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Signature type
    type Signature: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Generate cryptographic key
    fn generate_key(
        &self,
        key_type: &str,
    ) -> impl std::future::Future<Output = Result<Self::Key, Self::Error>> + Send;

    /// Sign data
    fn sign(
        &self,
        key: &Self::Key,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Self::Signature, Self::Error>> + Send;

    /// Verify signature
    fn verify(
        &self,
        key: &Self::Key,
        data: &[u8],
        signature: &Self::Signature,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    /// Encrypt data
    fn encrypt(
        &self,
        key: &Self::Key,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data
    fn decrypt(
        &self,
        key: &Self::Key,
        encrypted_data: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Get supported algorithms
    fn supported_algorithms(&self) -> Vec<String>;
}

/// HSM (Hardware Security Module) provider trait
pub trait HsmProvider: CryptoProvider {
    /// HSM status type
    type HsmStatus: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Key slot type
    type KeySlot: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Get HSM status
    fn hsm_status(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::HsmStatus, Self::Error>> + Send;

    /// List available key slots
    fn list_key_slots(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::KeySlot>, Self::Error>> + Send;

    /// Generate key in HSM
    fn generate_hsm_key(
        &self,
        slot: Self::KeySlot,
        key_type: &str,
    ) -> impl std::future::Future<Output = Result<Self::Key, Self::Error>> + Send;

    /// Import key to HSM
    fn import_key(
        &self,
        slot: Self::KeySlot,
        key: Self::Key,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Delete key from HSM
    /// Removes key
    fn delete_key(
        &self,
        slot: Self::KeySlot,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    fn hsm_operation(
        &self,
        operation: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> impl std::future::Future<Output = Result<serde_json::Value, Self::Error>> + Send;
}

pub trait AuditProvider: BearDogProvider {
    /// Audit event type
    type AuditEvent: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Log security event
    fn log_event(
        &self,
        event: Self::AuditEvent,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Query audit events
    fn query_events(
        &self,
        query: AuditQuery,
    ) -> impl std::future::Future<Output = Result<Vec<Self::AuditEvent>, Self::Error>> + Send;

    /// Get audit statistics
    fn audit_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<AuditStats, Self::Error>> + Send;

    /// Archive old events
    fn archive_events(
        &self,
        before_date: chrono::DateTime<chrono::Utc>,
    ) -> impl std::future::Future<Output = Result<usize, Self::Error>> + Send;
}

pub trait PolicyEngine: Send + Sync {
    /// Policy type
    type Policy: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Decision type
    type Decision: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Evaluate policy
    fn evaluate(
        &self,
        policy: &Self::Policy,
        context: PolicyContext,
    ) -> impl std::future::Future<Output = Result<Self::Decision, UnifiedTraitError>> + Send;

    /// Load policy
    /// Loads policy
    fn load_policy(
        &mut self,
        policy: Self::Policy,
    ) -> impl std::future::Future<Output = Result<(), UnifiedTraitError>> + Send;

    /// List loaded policies
    fn list_policies(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, UnifiedTraitError>> + Send;

    /// Remove policy
    /// Removes policy
    fn remove_policy(
        &mut self,
        policy_id: &str,
    ) -> impl std::future::Future<Output = Result<(), UnifiedTraitError>> + Send;

    /// Validate policy syntax
    /// Validates policy
    fn validate_policy(
        &self,
        policy: &Self::Policy,
    ) -> impl std::future::Future<Output = Result<bool, UnifiedTraitError>> + Send;
}

// Security types

/// Security audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    pub event_id: String,
    /// The event type value
    pub event_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    /// Optional resource
    pub resource: Option<String>,
    /// Optional action
    pub action: Option<String>,
    /// The result value
    pub result: SecurityResult,
    /// Mapping of details
    pub details: HashMap<String, serde_json::Value>,
    /// The severity value
    pub severity: SecuritySeverity,
}

/// Security operation result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityResult {
    /// Successful completion state
    Success,
    /// Error or failure state
    Failure,
    /// State indicating denied
    Denied,
    /// Error or failure state
    Error,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SecuritySeverity {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

/// Audit query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQuery {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub user_id: Option<String>,
    /// Optional event type
    pub event_type: Option<String>,
    /// Optional severity
    pub severity: Option<SecuritySeverity>,
    /// Optional limit
    pub limit: Option<usize>,
    /// Optional offset
    pub offset: Option<usize>,
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    /// Number of `total_events`
    pub total_events: usize,
    /// Mapping of events by type
    pub events_by_type: HashMap<String, usize>,
    /// Mapping of events by severity
    pub events_by_severity: HashMap<SecuritySeverity, usize>,
    /// The success rate value
    pub success_rate: f64,
    /// The average events per day value
    pub average_events_per_day: f64,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Policy evaluation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContext {
    pub user_id: String,
    pub session_id: Option<String>,
    /// The resource value
    pub resource: String,
    /// The action value
    pub action: String,
    /// Mapping of environment
    pub environment: HashMap<String, serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthCredentials {
    /// Represents password variant
    Password {
        username: String,
        password: String,
    },
    Token {
        token: String,
    },
    Certificate {
        certificate: Vec<u8>,
        private_key: Vec<u8>,
    },
    Biometric {
        user_id: String,
        biometric_data: Vec<u8>,
        biometric_type: String,
    },
    MultiFactorAuth {
        primary: Box<AuthCredentials>,
        secondary: Vec<AuthCredentials>,
    },
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether success is enabled
    pub success: bool,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    /// Collection of permissions
    pub permissions: Vec<String>,
    /// Optional expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional error message
    pub error_message: Option<String>,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureSession {
    pub session_id: String,
    pub user_id: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Collection of permissions
    pub permissions: Vec<String>,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Whether `is_active` is enabled
    pub is_active: bool,
}

/// Unified security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSecurityConfig {
    /// Whether `enable_authentication` is enabled
    pub enable_authentication: bool,
    /// Whether `enable_authorization` is enabled
    pub enable_authorization: bool,
    /// Whether `enable_audit` is enabled
    pub enable_audit: bool,
    pub session_timeout_minutes: u64,
    /// Number of `max_failed_attempts`
    pub max_failed_attempts: u32,
    /// Number of `lockout_duration_minutes`
    pub lockout_duration_minutes: u64,
    /// Whether `require_mfa` is enabled
    pub require_mfa: bool,
    /// Collection of supported auth methods
    pub supported_auth_methods: Vec<String>,
    /// Collection of crypto algorithms
    pub crypto_algorithms: Vec<String>,
    /// Whether hsm is enabled
    pub hsm_enabled: bool,
    /// Number of `audit_retention_days`
    pub audit_retention_days: u32,
    /// Number of `policy_refresh_interval_minutes`
    pub policy_refresh_interval_minutes: u64,
}

impl Default for UnifiedSecurityConfig {
    fn default() -> Self {
        Self {
            enable_authentication: true,
            enable_authorization: true,
            enable_audit: true,
            session_timeout_minutes: 60,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
            require_mfa: false,
            supported_auth_methods: vec![
                "password".to_string(),
                "token".to_string(),
                "certificate".to_string(),
            ],
            crypto_algorithms: vec![
                "AES-256-GCM".to_string(),
                "RSA-4096".to_string(),
                "ECDSA-P256".to_string(),
            ],
            hsm_enabled: false,
            audit_retention_days: 365,
            policy_refresh_interval_minutes: 60,
        }
    }
}

impl BearDogConfig for UnifiedSecurityConfig {
    /// Validates input
    fn validate(&self) -> Result<(), beardog_errors::BearDogError> {
        if self.session_timeout_minutes == 0 {
            return Err(beardog_errors::BearDogError::Security {
                message: "Session timeout must be greater than 0".to_string(),
                category: beardog_errors::SecurityErrorCategory::Configuration,
            });
        }

        if self.max_failed_attempts == 0 {
            return Err(beardog_errors::BearDogError::Security {
                message: "Max failed attempts must be greater than 0".to_string(),
                category: beardog_errors::SecurityErrorCategory::Configuration,
            });
        }

        if self.supported_auth_methods.is_empty() {
            return Err(beardog_errors::BearDogError::Security {
                message: "At least one authentication method must be supported".to_string(),
                category: beardog_errors::SecurityErrorCategory::Configuration,
            });
        }

        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, beardog_errors::BearDogError> {
        let mut merged = self.clone();
        merged.enable_authentication = other.enable_authentication;
        merged.enable_authorization = other.enable_authorization;
        merged.enable_audit = other.enable_audit;

        if other.session_timeout_minutes != Self::default().session_timeout_minutes {
            merged.session_timeout_minutes = other.session_timeout_minutes;
        }
        if other.max_failed_attempts != Self::default().max_failed_attempts {
            merged.max_failed_attempts = other.max_failed_attempts;
        }
        if other.lockout_duration_minutes != Self::default().lockout_duration_minutes {
            merged.lockout_duration_minutes = other.lockout_duration_minutes;
        }

        merged.require_mfa = other.require_mfa;
        merged.hsm_enabled = other.hsm_enabled;

        if !other.supported_auth_methods.is_empty() {
            merged.supported_auth_methods = other.supported_auth_methods.clone();
        }
        if !other.crypto_algorithms.is_empty() {
            merged.crypto_algorithms = other.crypto_algorithms.clone();
        }

        Ok(merged)
    }

    fn to_toml(&self) -> Result<String, beardog_errors::BearDogError> {
        serde_json::to_string_pretty(self).map_err(|e| beardog_errors::BearDogError::System {
            message: format!("Failed to serialize security config: {}", e),
            category: beardog_errors::SystemErrorCategory::General,
        })
    }

    fn domain() -> &'static str {
        "security"
    }

    /// Creates instance from env
    fn from_env() -> Result<Self, beardog_errors::BearDogError> {
        let mut config = Self::default();

        if let Ok(enable_auth) = std::env::var("BEARDOG_SECURITY_ENABLE_AUTH") {
            config.enable_authentication =
                enable_auth.parse().unwrap_or(config.enable_authentication);
        }

        if let Ok(session_timeout) = std::env::var("BEARDOG_SECURITY_SESSION_TIMEOUT") {
            config.session_timeout_minutes = session_timeout
                .parse()
                .unwrap_or(config.session_timeout_minutes);
        }

        if let Ok(require_mfa) = std::env::var("BEARDOG_SECURITY_REQUIRE_MFA") {
            config.require_mfa = require_mfa.parse().unwrap_or(config.require_mfa);
        }

        if let Ok(hsm_enabled) = std::env::var("BEARDOG_SECURITY_HSM_ENABLED") {
            config.hsm_enabled = hsm_enabled.parse().unwrap_or(config.hsm_enabled);
        }

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_config_validation() {
        let mut config = UnifiedSecurityConfig::default();
        assert!(config.validate().is_ok());

        config.session_timeout_minutes = 0;
        assert!(config.validate().is_err());

        config.session_timeout_minutes = 60;
        config.supported_auth_methods.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_security_audit_event() {
        let event = SecurityAuditEvent {
            event_id: "test_event".to_string(),
            event_type: "authentication".to_string(),
            timestamp: chrono::Utc::now(),
            user_id: Some("user123".to_string()),
            session_id: None,
            resource: None,
            action: Some("login".to_string()),
            result: SecurityResult::Success,
            details: HashMap::new(),
            severity: SecuritySeverity::Medium,
        };

        assert_eq!(event.event_id, "test_event");
        assert_eq!(event.result, SecurityResult::Success);
        assert_eq!(event.severity, SecuritySeverity::Medium);
    }

    #[test]
    fn test_auth_credentials_variants() {
        let password_creds = AuthCredentials::Password {
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        let token_creds = AuthCredentials::Token {
            token: "abc123".to_string(),
        };

        assert!(matches!(password_creds, AuthCredentials::Password { .. }));
        assert!(matches!(token_creds, AuthCredentials::Token { .. }));
    }
}
