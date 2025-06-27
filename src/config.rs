//! Configuration management for BearDog
//! 
//! Provides secure-by-default configuration loading and validation.

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{BearDogResult};
use std::env;
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

/// Security levels for BearDog operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SecurityLevel {
    /// Standard security - good for most applications
    Standard,
    /// High security - recommended for sensitive data
    #[default]
    High,
    /// Maximum security - for highly sensitive environments
    Maximum,
}

/// Main configuration structure for BearDog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    /// Security level for operations
    pub security: SecurityConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Network and API configuration
    pub network: NetworkConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Threat detection settings
    pub threat_detection: ThreatDetectionConfig,
    /// Compliance settings
    pub compliance: ComplianceConfig,
    /// Audit settings
    pub audit: AuditConfig,
    /// API server configuration
    pub api: ApiConfig,
    /// Workflow configuration
    pub workflows: WorkflowConfig,
    /// Adapter configurations
    pub adapters: AdapterConfigs,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
}

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

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL (supports PostgreSQL and SQLite)
    pub url: String,
    /// Maximum database connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
}

/// Network and API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Enable TLS
    pub enable_tls: bool,
    /// TLS certificate path (if TLS enabled)
    pub tls_cert_path: Option<String>,
    /// TLS private key path (if TLS enabled)
    pub tls_key_path: Option<String>,
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

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address for the API server
    pub bind_address: String,
    /// TLS configuration
    pub tls: TlsConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Request timeout
    pub timeout: Duration,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_path: Option<PathBuf>,
    /// Private key file path
    pub key_path: Option<PathBuf>,
    /// Minimum TLS version
    pub min_version: String,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// JWT expiration time
    pub jwt_expiration: Duration,
    /// API key configuration
    pub api_keys: ApiKeyConfig,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Enable API key authentication
    pub enabled: bool,
    /// API key header name
    pub header_name: String,
    /// API key length
    pub key_length: usize,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
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

/// MFA method enumeration
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

/// Session configuration
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

/// Threat detection sensitivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSensitivity {
    /// Low sensitivity
    Low,
    /// Medium sensitivity
    Medium,
    /// High sensitivity
    High,
}

/// ML model configuration
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

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Default approval timeout
    pub default_approval_timeout: Duration,
    /// Maximum concurrent workflows
    pub max_concurrent_workflows: usize,
    /// Workflow storage configuration
    pub storage: WorkflowStorageConfig,
    /// Notification configuration
    pub notifications: crate::workflows::NotificationConfig,
    /// Policy configuration
    pub policies: crate::workflows::PolicyConfig,
}

/// Workflow storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStorageConfig {
    /// Storage type
    pub storage_type: String,
    /// Storage configuration
    pub config: HashMap<String, String>,
}

/// Adapter configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfigs {
    /// Adapter configuration for external system integrations
    pub external_systems: AdapterConfig,
}

/// Adapter configuration for external system integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    /// Rust ecosystem integrations (always free/enabled)
    pub rust_ecosystem: RustEcosystemConfig,
    /// Licensed external system integrations
    pub external_systems: ExternalSystemsConfig,
}

/// Configuration for Rust ecosystem project integrations (AGPL - always free)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustEcosystemConfig {
    /// NestGate secure file transfer (if part of your Rust ecosystem)
    pub nestgate: Option<RustProjectConfig>,
    /// SongBird communication platform (if part of your Rust ecosystem)
    pub songbird: Option<RustProjectConfig>,
    /// Other Rust projects in your ecosystem
    pub additional_projects: HashMap<String, RustProjectConfig>,
}

/// Configuration for a Rust ecosystem project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustProjectConfig {
    /// Always enabled for Rust ecosystem
    pub enabled: bool,
    /// Project endpoint URL
    pub endpoint: String,
    /// Connection timeout
    pub timeout_ms: u32,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
    /// Optional authentication (for inter-service auth)
    pub auth: Option<InterServiceAuth>,
}

/// Configuration for licensed external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSystemsConfig {
    /// Enterprise HSM integrations
    pub hsm_systems: HashMap<String, ExternalSystemConfig>,
    /// SIEM system integrations
    pub siem_systems: HashMap<String, ExternalSystemConfig>,
    /// Enterprise database integrations
    pub database_systems: HashMap<String, ExternalSystemConfig>,
    /// Cloud service integrations
    pub cloud_services: HashMap<String, ExternalSystemConfig>,
    /// Authentication system integrations
    pub auth_systems: HashMap<String, ExternalSystemConfig>,
    /// Backup system integrations
    pub backup_systems: HashMap<String, ExternalSystemConfig>,
    /// Messaging system integrations
    pub messaging_systems: HashMap<String, ExternalSystemConfig>,
}

/// Configuration for a licensed external system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSystemConfig {
    /// Requires valid license to enable
    pub enabled: bool,
    /// System endpoint URL
    pub endpoint: String,
    /// Connection timeout
    pub timeout_ms: u32,
    /// License file path
    pub license_file: Option<String>,
    /// System-specific configuration
    pub system_config: HashMap<String, String>,
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
}

/// Inter-service authentication for Rust ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterServiceAuth {
    /// Shared secret or key
    pub secret: String,
    /// Authentication method
    pub method: AuthMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    SharedSecret,
    JWT,
    Mutual,
}

/// Retry configuration for external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u32,
    pub max_delay_ms: u32,
    pub backoff_multiplier: f32,
}

/// Audit storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorageConfig {
    /// Storage type
    pub storage_type: String,
    /// Storage configuration
    pub config: HashMap<String, String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output configuration
    pub output: LoggingOutputConfig,
}

/// Logging output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingOutputConfig {
    /// Console output
    pub console: bool,
    /// File output
    pub file: Option<String>,
    /// Syslog output
    pub syslog: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Prometheus metrics configuration
    pub prometheus: PrometheusConfig,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Metrics port
    pub port: u16,
}

impl Default for BearDogConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            database: DatabaseConfig::default(),
            network: NetworkConfig::default(),
            api: ApiConfig::default(),
            encryption: EncryptionConfig::default(),
            compliance: ComplianceConfig::default(),
            threat_detection: ThreatDetectionConfig::default(),
            workflows: WorkflowConfig::default(),
            adapters: AdapterConfigs::default(),
            audit: AuditConfig::default(),
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind_address: std::env::var("BEARDOG_API_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8443".to_string()),
            tls: TlsConfig::default(),
            auth: AuthConfig::default(),
            rate_limiting: RateLimitConfig::default(),
            cors: CorsConfig::default(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: None,
            key_path: None,
            min_version: "1.2".to_string(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiration: Duration::from_secs(3600),
            api_keys: ApiKeyConfig::default(),
        }
    }
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_name: "X-API-Key".to_string(),
            key_length: 32,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 100,
            burst_size: 10,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["*".to_string()],
        }
    }
}

impl BearDogConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> BearDogResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: BearDogConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables and default file
    pub fn from_env() -> BearDogResult<Self> {
        // Try to load from file first
        let mut config = if Path::new("beardog.toml").exists() {
            Self::from_file("beardog.toml")?
        } else if Path::new("/etc/beardog/config.toml").exists() {
            Self::from_file("/etc/beardog/config.toml")?
        } else {
            Self::default()
        };

        // Apply environment variable overrides
        if let Ok(level) = env::var("BEARDOG_SECURITY_LEVEL") {
            config.security.level = match level.to_lowercase().as_str() {
                "standard" => SecurityLevel::Standard,
                "high" => SecurityLevel::High,
                "maximum" => SecurityLevel::Maximum,
                _ => config.security.level, // Keep default if invalid
            };
        }

        if let Ok(port) = env::var("BEARDOG_API_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.network.port = port_num;
            }
        }

        if let Ok(host) = env::var("BEARDOG_API_HOST") {
            config.network.host = host;
        }

        if let Ok(url) = env::var("BEARDOG_DATABASE_URL") {
            config.database.url = url;
        }

        if let Ok(level) = env::var("BEARDOG_LOG_LEVEL") {
            config.audit.enable_logging = level.to_lowercase() == "debug";
        }

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> BearDogResult<()> {
        // Validate network configuration
        if self.network.port == 0 {
            return Err(crate::error::BearDogError::Configuration { 
                message: "Network port cannot be 0".to_string() 
            });
        }

        // Validate database configuration
        if self.database.url.is_empty() {
            return Err(crate::error::BearDogError::Configuration { 
                message: "Database URL cannot be empty".to_string() 
            });
        }

        // Validate threat detection configuration
        if self.threat_detection.enabled && self.threat_detection.ml_models.model_paths.is_empty() {
            return Err(crate::error::BearDogError::Configuration { 
                message: "Threat detection enabled but no ML models configured".to_string() 
            });
        }

        // Validate TLS configuration
        if self.network.enable_tls && (self.network.tls_cert_path.is_none() || self.network.tls_key_path.is_none()) {
            return Err(crate::error::BearDogError::Configuration { 
                message: "TLS certificate and key paths must be provided when TLS is enabled".to_string() 
            });
        }

        // Validate API configuration
        if self.api.bind_address.is_empty() {
            return Err(crate::error::BearDogError::Configuration { 
                message: "API bind address cannot be empty".to_string() 
            });
        }
        
        // Validate JWT secret
        if self.api.auth.jwt_secret == "changeme" {
            return Err(crate::error::BearDogError::Configuration { 
                message: "Default JWT secret must be changed in production".to_string() 
            });
        }
        
        // Validate password policy
        if self.security.password_policy.min_length < 8 {
            return Err(crate::error::BearDogError::Configuration { 
                message: "Minimum password length must be at least 8".to_string() 
            });
        }

        Ok(())
    }

    /// Generate a default configuration file
    pub fn generate_default_file<P: AsRef<Path>>(path: P) -> BearDogResult<()> {
        let config = Self::default();
        let toml_content = toml::to_string_pretty(&config)?;
        std::fs::write(path, toml_content)?;
        Ok(())
    }
}

// Default implementations for all configuration structs

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            level: SecurityLevel::High,
            enable_hsm: false,
            token_expiration_seconds: 3600,
            max_failed_logins: 5,
            rate_limit_requests_per_minute: 100,
            max_session_duration: Duration::from_secs(28800), // 8 hours
            password_policy: PasswordPolicy::default(),
            mfa: MfaConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:beardog.db".to_string(),
            max_connections: 10,
            connection_timeout_seconds: 30,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("BEARDOG_NETWORK_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("BEARDOG_NETWORK_PORT")
                .unwrap_or_else(|_| "8443".to_string())
                .parse()
                .unwrap_or(8443),
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            key_derivation_iterations: 100000,
            key_rotation_days: 90,
            key_rotation_interval: Duration::from_secs(90 * 24 * 3600), // 90 days
            hsm: HsmConfig::default(),
            key_derivation: KeyDerivationConfig::default(),
        }
    }
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enable_ml: false,
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
            monitoring_interval: Duration::from_secs(3600), // 1 hour
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
            log_level: "info".to_string(),
            storage: AuditStorageConfig::default(),
        }
    }
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            default_approval_timeout: Duration::from_secs(3 * 24 * 3600), // 3 days
            max_concurrent_workflows: 1000,
            storage: WorkflowStorageConfig::default(),
            notifications: crate::workflows::NotificationConfig::default(),
            policies: crate::workflows::PolicyConfig::default(),
        }
    }
}

impl Default for WorkflowStorageConfig {
    fn default() -> Self {
        let mut config = HashMap::new();
        config.insert("max_items".to_string(), "10000".to_string());
        
        Self {
            storage_type: "memory".to_string(),
            config,
        }
    }
}

impl Default for AdapterConfigs {
    fn default() -> Self {
        Self {
            external_systems: AdapterConfig::default(),
        }
    }
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            rust_ecosystem: RustEcosystemConfig::default(),
            external_systems: ExternalSystemsConfig::default(),
        }
    }
}

impl Default for RustEcosystemConfig {
    fn default() -> Self {
        Self {
            nestgate: None,
            songbird: None,
            additional_projects: HashMap::new(),
        }
    }
}

impl Default for ExternalSystemsConfig {
    fn default() -> Self {
        Self {
            hsm_systems: HashMap::new(),
            siem_systems: HashMap::new(),
            database_systems: HashMap::new(),
            cloud_services: HashMap::new(),
            auth_systems: HashMap::new(),
            backup_systems: HashMap::new(),
            messaging_systems: HashMap::new(),
        }
    }
}

impl Default for AuditStorageConfig {
    fn default() -> Self {
        let mut config = HashMap::new();
        config.insert("max_items".to_string(), "100000".to_string());
        
        Self {
            storage_type: "memory".to_string(),
            config,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: LoggingOutputConfig::default(),
        }
    }
}

impl Default for LoggingOutputConfig {
    fn default() -> Self {
        Self {
            console: true,
            file: None,
            syslog: false,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            prometheus: PrometheusConfig::default(),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "/metrics".to_string(),
            port: 9090,
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
            history_count: 5,
            max_age: Duration::from_secs(90 * 24 * 3600), // 90 days
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            required: false,
            methods: vec![MfaMethod::TOTP],
            totp: TotpConfig::default(),
        }
    }
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            issuer: "BearDog Security".to_string(),
            time_step: 30,
            code_length: 6,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(300), // 5 minutes
            secure_cookies: true,
        }
    }
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "software".to_string(),
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
            memory_cost: 65536, // 64MB
            time_cost: 3,
            parallelism: 4,
        }
    }
}

impl Default for Pbkdf2Config {
    fn default() -> Self {
        Self {
            iterations: 100000,
            hash_algorithm: "SHA256".to_string(),
        }
    }
}

impl Default for MlModelConfig {
    fn default() -> Self {
        let mut model_paths = HashMap::new();
        model_paths.insert("anomaly_detection".to_string(), "/tmp/models/anomaly.model".to_string());
        
        Self {
            model_paths,
            update_interval: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl Default for BehavioralAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_window: Duration::from_secs(24 * 3600), // 24 hours
            anomaly_threshold: 0.8,
        }
    }
} 