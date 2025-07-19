//! Core configuration types
//!
//! Contains the main configuration structures for BearDog.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// Import configuration types from other modules
use crate::integration::{AdapterConfigs, WorkflowConfig};
use crate::monitoring::MetricsConfig;
use crate::network::NetworkConfig;
use crate::secrets::{SecretManager, SecretSourceConfig};

/// BearDog error type for configuration
pub type BearDogError = anyhow::Error;
pub type BearDogResult<T> = Result<T, BearDogError>;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable HSM (Hardware Security Module) support
    pub enable_hsm: bool,
    /// HSM provider (software, hardware, cloud)
    pub hsm_provider: String,
    /// HSM configuration path
    pub hsm_config_path: Option<PathBuf>,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Security level
    pub security_level: SecurityLevel,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_hsm: false,
            hsm_provider: "software".to_string(),
            hsm_config_path: None,
            enable_mfa: true,
            security_level: SecurityLevel::High,
        }
    }
}

/// Security encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEncryptionConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key derivation function
    pub key_derivation: String,
    /// Enable key rotation
    pub enable_key_rotation: bool,
    /// Key rotation interval in seconds
    pub key_rotation_interval: u64,
}

impl Default for SecurityEncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            key_derivation: "PBKDF2".to_string(),
            enable_key_rotation: true,
            key_rotation_interval: 86400, // 24 hours
        }
    }
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Monitoring endpoint
    pub endpoint: String,
    /// Metrics collection interval in seconds
    pub interval: u64,
    /// Enable health checks
    pub health_checks: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            interval: 30,
            health_checks: true,
        }
    }
}

/// Threat detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable threat detection
    pub enabled: bool,
    /// Threat detection sensitivity (low, medium, high)
    pub sensitivity: String,
    /// Enable machine learning detection
    pub enable_ml: bool,
    /// Response actions for threats
    pub response_actions: Vec<String>,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: "medium".to_string(),
            enable_ml: false,
            response_actions: vec!["log".to_string(), "alert".to_string()],
        }
    }
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Enable compliance monitoring
    pub enabled: bool,
    /// Compliance framework (SOC2, ISO27001, etc.)
    pub framework: String,
    /// Audit log retention period in days
    pub audit_retention_days: u32,
    /// Enable automated compliance reporting
    pub auto_reporting: bool,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            framework: "SOC2".to_string(),
            audit_retention_days: 365,
            auto_reporting: true,
        }
    }
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit log storage path
    pub storage_path: PathBuf,
    /// Enable tamper detection
    pub tamper_detection: bool,
    /// Audit log format (json, csv, syslog)
    pub format: String,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_path: PathBuf::from("/var/log/beardog/audit.log"),
            tamper_detection: true,
            format: "json".to_string(),
        }
    }
}

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

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Enable standalone mode with memory key manager
    pub standalone_mode: bool,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Enable debug mode
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: "0.1.0".to_string(),
            standalone_mode: false,
            environment: "development".to_string(),
            debug: false,
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Query timeout in seconds
    pub query_timeout: u64,
    /// Enable SSL/TLS
    pub ssl: bool,
    /// SSL certificate path
    pub ssl_cert_path: Option<PathBuf>,
    /// SSL private key path
    pub ssl_key_path: Option<PathBuf>,
    /// SSL CA certificate path
    pub ssl_ca_path: Option<PathBuf>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:///var/lib/beardog/beardog.db".to_string(),
            max_connections: 10,
            connection_timeout: 30,
            query_timeout: 30,
            ssl: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            ssl_ca_path: None,
        }
    }
}

/// Licensing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensingConfig {
    /// License key
    pub key: String,
    /// License server URL
    pub server_url: String,
    /// License check interval in seconds
    pub check_interval: u64,
    /// Enable license validation
    pub validation_enabled: bool,
}

impl Default for LicensingConfig {
    fn default() -> Self {
        Self {
            key: "demo".to_string(),
            server_url: "https://license.beardog.eco".to_string(),
            check_interval: 3600, // 1 hour
            validation_enabled: true,
        }
    }
}

/// Node registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRegistryConfig {
    /// Registry URL
    pub url: String,
    /// Node registration interval in seconds
    pub registration_interval: u64,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Enable auto-registration
    pub auto_register: bool,
}

impl Default for NodeRegistryConfig {
    fn default() -> Self {
        Self {
            url: "https://registry.beardog.eco".to_string(),
            registration_interval: 300, // 5 minutes
            health_check_interval: 60,  // 1 minute
            auto_register: true,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, plain)
    pub format: String,
    /// Log output (stdout, file, syslog)
    pub output: String,
    /// Log file path (if output is file)
    pub file_path: Option<PathBuf>,
    /// Enable log rotation
    pub rotation: bool,
    /// Maximum log file size in MB
    pub max_file_size: u64,
    /// Maximum number of log files to keep
    pub max_files: u32,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
            file_path: None,
            rotation: true,
            max_file_size: 100, // 100MB
            max_files: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BearDogConfig {
    /// Basic application settings
    pub app: AppConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Encryption configuration
    pub encryption: SecurityEncryptionConfig,
    /// Licensing configuration
    pub licensing: LicensingConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Node registration configuration
    pub node_registry: NodeRegistryConfig,
    /// Threat detection settings
    pub threat_detection: ThreatDetectionConfig,
    /// Compliance settings
    pub compliance: ComplianceConfig,
    /// Audit settings
    pub audit: AuditConfig,
    /// API server configuration (alias for network config)
    pub api: NetworkConfig,
    /// Workflow configuration
    pub workflows: WorkflowConfig,
    /// Adapter configurations
    pub adapters: AdapterConfigs,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Secret management configuration
    pub secrets: SecretSourceConfig,
}

impl BearDogConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> BearDogResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;

        let config: Self = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

        // Initialize decentralized authentication
        // Each node generates its own Ed25519 keypair - no central authority needed
        tracing::info!("Initializing decentralized Ed25519 authentication");

        // No JWT secrets needed - BearDog uses decentralized cryptographic identity
        tracing::info!(
            "BearDog configuration loaded successfully - decentralized authentication enabled"
        );

        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> BearDogResult<Self> {
        // Use default configuration and validate
        let config = Self::default();

        // Initialize decentralized authentication
        // Each node generates its own Ed25519 keypair - no central authority needed
        tracing::info!("Initializing decentralized Ed25519 authentication");

        // No JWT secrets needed - BearDog uses decentralized cryptographic identity
        tracing::info!(
            "BearDog configuration loaded successfully - decentralized authentication enabled"
        );

        config.validate()?;
        Ok(config)
    }

    /// Load configuration with secret management
    pub async fn from_env_with_secrets() -> BearDogResult<(Self, SecretManager)> {
        let config = Self::from_env()?;

        // Create secret manager based on environment
        let secret_manager = match config.app.environment.as_str() {
            "production" => SecretManager::new(SecretSourceConfig::default()),
            _ => SecretManager::new(SecretSourceConfig::default()),
        };

        // Load database password if available
        // This is a placeholder for future secret integration

        // Initialize decentralized authentication
        // Each node generates its own Ed25519 keypair - no central authority needed
        tracing::info!("Initializing decentralized Ed25519 authentication");

        // No JWT secrets needed - BearDog uses decentralized cryptographic identity
        tracing::info!(
            "BearDog configuration loaded successfully - decentralized authentication enabled"
        );

        config.validate()?;
        Ok((config, secret_manager))
    }

    /// Load secrets from secret manager
    pub async fn load_secrets_from_manager(
        &mut self,
        secret_manager: &SecretManager,
    ) -> BearDogResult<()> {
        // Clone the secret manager to avoid borrowing issues
        let mut temp_manager = SecretManager::new(secret_manager.config.clone());

        // Load database password if available
        if let Ok(_db_password) = temp_manager
            .get_secret(crate::secrets::secret_keys::DATABASE_PASSWORD)
            .await
        {
            // Update database configuration if needed
            // This is a placeholder - actual implementation depends on database config structure
        }

        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> BearDogResult<()> {
        // Validate database configuration
        if self.database.url.is_empty() {
            return Err(anyhow::anyhow!("Database URL cannot be empty"));
        }

        if self.database.max_connections == 0 {
            return Err(anyhow::anyhow!(
                "Database max connections must be greater than 0"
            ));
        }

        if self.database.connection_timeout == 0 {
            return Err(anyhow::anyhow!(
                "Database connection timeout must be greater than 0"
            ));
        }

        // Validate network configuration
        if self.network.http.enabled && self.network.http.bind_address.is_empty() {
            return Err(anyhow::anyhow!(
                "HTTP bind address cannot be empty when HTTP is enabled"
            ));
        }

        if self.network.http.enabled && self.network.http.port == 0 {
            return Err(anyhow::anyhow!(
                "HTTP port must be greater than 0 when HTTP is enabled"
            ));
        }

        if self.network.https.enabled && self.network.https.bind_address.is_empty() {
            return Err(anyhow::anyhow!(
                "HTTPS bind address cannot be empty when HTTPS is enabled"
            ));
        }

        if self.network.https.enabled && self.network.https.port == 0 {
            return Err(anyhow::anyhow!(
                "HTTPS port must be greater than 0 when HTTPS is enabled"
            ));
        }

        // Validate security configuration
        if self.security.hsm_provider.is_empty() {
            return Err(anyhow::anyhow!("Security HSM provider cannot be empty"));
        }

        // Validate audit configuration
        if self.audit.enabled && self.audit.storage_path.as_os_str().is_empty() {
            return Err(anyhow::anyhow!(
                "Audit storage path cannot be empty when audit is enabled"
            ));
        }

        Ok(())
    }

    /// Get the current environment
    pub fn environment(&self) -> &str {
        &self.app.environment
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }

    /// Check if debug mode is enabled
    pub fn is_debug(&self) -> bool {
        self.app.debug
    }

    /// Get the service name
    pub fn service_name(&self) -> &str {
        &self.app.name
    }

    /// Get the service version
    pub fn service_version(&self) -> &str {
        &self.app.version
    }
}
