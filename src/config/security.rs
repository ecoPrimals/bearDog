//! Security configuration types
//! 
//! Contains all security-related configuration structures including encryption,
//! HSM, MFA, password policies, and session management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::core::SecurityLevel;

/// Security-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Overall security level
    pub level: SecurityLevel,
    /// Enable HSM integration
    pub enable_hsm: bool,
    /// Token expiration time in seconds
    pub token_expiration_seconds: u64,
    /// Maximum failed login attempts
    pub max_failed_logins: u32,
    /// Rate limiting settings
    pub rate_limit_requests_per_minute: u32,
    /// Maximum session duration
    pub max_session_duration: Duration,
    /// Password policy requirements
    pub password_policy: PasswordPolicy,
    /// Multi-factor authentication settings
    pub mfa: MfaConfig,
    /// Session management
    pub session: SessionConfig,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key derivation iterations for Argon2
    pub key_derivation_iterations: u32,
    /// Key rotation interval in days
    pub key_rotation_days: u32,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
    /// Hardware Security Module configuration
    pub hsm: HsmConfig,
    /// Key derivation settings
    pub key_derivation: KeyDerivationConfig,
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
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
    /// Password history count
    pub history_count: usize,
    /// Maximum password age
    pub max_age: Duration,
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Require MFA for all users
    pub required: bool,
    /// Supported MFA methods
    pub methods: Vec<MfaMethod>,
    /// TOTP configuration
    pub totp: TotpConfig,
}

/// MFA method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// Time-based One-Time Password
    TOTP,
    /// SMS-based authentication
    SMS,
    /// Email-based authentication
    Email,
    /// Hardware token
    HardwareToken,
}

/// TOTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    /// Issuer name
    pub issuer: String,
    /// Time step in seconds
    pub time_step: u32,
    /// Code length
    pub code_length: usize,
}

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Session cleanup interval
    pub cleanup_interval: Duration,
    /// Secure session cookies
    pub secure_cookies: bool,
}

/// Hardware Security Module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Enable HSM integration
    pub enabled: bool,
    /// HSM provider
    pub provider: String,
    /// HSM configuration parameters
    pub config: HashMap<String, String>,
}

/// Key derivation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// Argon2 parameters
    pub argon2: Argon2Config,
    /// PBKDF2 parameters
    pub pbkdf2: Pbkdf2Config,
}

/// Argon2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Config {
    /// Memory cost
    pub memory_cost: u32,
    /// Time cost
    pub time_cost: u32,
    /// Parallelism
    pub parallelism: u32,
}

/// PBKDF2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pbkdf2Config {
    /// Iteration count
    pub iterations: u32,
    /// Hash algorithm
    pub hash_algorithm: String,
}

/// Threat detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable ML-based threat detection
    pub enable_ml: bool,
    /// Enable threat detection
    pub enabled: bool,
    /// Detection sensitivity
    pub sensitivity: ThreatSensitivity,
    /// Model update interval in hours
    pub model_update_hours: u32,
    /// ML model configuration
    pub ml_models: MlModelConfig,
    /// Behavioral analysis settings
    pub behavioral_analysis: BehavioralAnalysisConfig,
}

/// Threat detection sensitivity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSensitivity {
    /// Low sensitivity
    Low,
    /// Medium sensitivity
    Medium,
    /// High sensitivity
    High,
}

/// Machine learning model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModelConfig {
    /// Model file paths
    pub model_paths: HashMap<String, String>,
    /// Model update interval
    pub update_interval: Duration,
}

/// Behavioral analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralAnalysisConfig {
    /// Enable behavioral analysis
    pub enabled: bool,
    /// Analysis window
    pub analysis_window: Duration,
    /// Anomaly threshold
    pub anomaly_threshold: f64,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Enable GDPR compliance checks
    pub enable_gdpr: bool,
    /// Enable SOX compliance checks
    pub enable_sox: bool,
    /// Enable HIPAA compliance checks
    pub enable_hipaa: bool,
    /// Compliance report generation interval in days
    pub report_interval_days: u32,
    /// Enabled compliance standards
    pub enabled_standards: Vec<String>,
    /// Compliance monitoring interval
    pub monitoring_interval: Duration,
    /// Audit log retention
    pub audit_retention: Duration,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enable_logging: bool,
    /// Audit log retention days
    pub retention_days: u32,
    /// Enable real-time audit alerts
    pub enable_alerts: bool,
    /// Audit log level
    pub log_level: String,
    /// Audit storage configuration
    pub storage: AuditStorageConfig,
}

/// Audit storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorageConfig {
    /// Storage type
    pub storage_type: String,
    /// Storage configuration
    pub config: HashMap<String, String>,
}

// Default implementations
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            level: SecurityLevel::High,
            enable_hsm: false,
            token_expiration_seconds: 3600,
            max_failed_logins: 3,
            rate_limit_requests_per_minute: 60,
            max_session_duration: Duration::from_secs(8 * 3600), // 8 hours
            password_policy: PasswordPolicy::default(),
            mfa: MfaConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            key_derivation_iterations: 100_000,
            key_rotation_days: 30,
            key_rotation_interval: Duration::from_secs(30 * 24 * 3600), // 30 days
            hsm: HsmConfig::default(),
            key_derivation: KeyDerivationConfig::default(),
        }
    }
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enable_ml: true,
            enabled: true,
            sensitivity: ThreatSensitivity::Medium,
            model_update_hours: 24,
            ml_models: MlModelConfig::default(),
            behavioral_analysis: BehavioralAnalysisConfig::default(),
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enable_gdpr: true,
            enable_sox: false,
            enable_hipaa: false,
            report_interval_days: 30,
            enabled_standards: vec!["GDPR".to_string()],
            monitoring_interval: Duration::from_secs(5 * 60), // 5 minutes
            audit_retention: Duration::from_secs(365 * 24 * 3600), // 1 year
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            retention_days: 365,
            enable_alerts: true,
            log_level: "INFO".to_string(),
            storage: AuditStorageConfig::default(),
        }
    }
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
            history_count: 10,
            max_age: Duration::from_secs(90 * 24 * 3600), // 90 days
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            required: true,
            methods: vec![MfaMethod::TOTP],
            totp: TotpConfig::default(),
        }
    }
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            issuer: "BearDog".to_string(),
            time_step: 30,
            code_length: 6,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(4 * 3600), // 4 hours
            cleanup_interval: Duration::from_secs(15 * 60), // 15 minutes
            secure_cookies: true,
        }
    }
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "SoftHSM".to_string(),
            config: HashMap::new(),
        }
    }
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            argon2: Argon2Config::default(),
            pbkdf2: Pbkdf2Config::default(),
        }
    }
}

impl Default for Argon2Config {
    fn default() -> Self {
        Self {
            memory_cost: 65536, // 64 MB
            time_cost: 3,
            parallelism: 4,
        }
    }
}

impl Default for Pbkdf2Config {
    fn default() -> Self {
        Self {
            iterations: 100_000,
            hash_algorithm: "SHA-256".to_string(),
        }
    }
}

impl Default for MlModelConfig {
    fn default() -> Self {
        Self {
            model_paths: HashMap::new(),
            update_interval: Duration::from_secs(24 * 3600), // 24 hours
        }
    }
}

impl Default for BehavioralAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_window: Duration::from_secs(3600), // 1 hour
            anomaly_threshold: 0.7,
        }
    }
}

impl Default for AuditStorageConfig {
    fn default() -> Self {
        Self {
            storage_type: "file".to_string(),
            config: {
                let mut config = HashMap::new();
                config.insert("path".to_string(), "./audit.log".to_string());
                config
            },
        }
    }
} 