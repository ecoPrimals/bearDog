//! Core configuration types
//!
//! Contains the main configuration structures for BearDog.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

use super::integration::{AdapterConfigs, WorkflowConfig};
use super::monitoring::{MetricsConfig, MonitoringConfig};
use super::network::NetworkConfig;
use super::security::{
    AuditConfig, ComplianceConfig, EncryptionConfig as SecurityEncryptionConfig, SecurityConfig,
    ThreatDetectionConfig,
};

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

/// Licensing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensingConfig {
    /// License key
    pub key: String,
    /// License server URL
    pub server_url: String,
    /// Check interval in seconds
    pub check_interval: u64,
    /// Enable license validation
    pub validation_enabled: bool,
}

/// Node registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRegistryConfig {
    /// Registry server URL
    pub url: String,
    /// Registration interval in seconds
    pub registration_interval: u64,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Enable automatic registration
    pub auto_register: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Log format (json, plain)
    pub format: String,
    /// Log output (stdout, stderr, file)
    pub output: String,
    /// Log file path (if output is file)
    pub file_path: Option<PathBuf>,
    /// Enable log rotation
    pub rotation: bool,
    /// Max log file size in MB
    pub max_file_size: u64,
    /// Max number of log files to keep
    pub max_files: u32,
}

/// Main BearDog configuration structure
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
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://beardog.db".to_string(),
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

impl Default for LicensingConfig {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            server_url: "".to_string(),
            check_interval: 3600,
            validation_enabled: true,
        }
    }
}

impl Default for NodeRegistryConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            registration_interval: 300,
            health_check_interval: 60,
            auto_register: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "plain".to_string(),
            output: "stdout".to_string(),
            file_path: None,
            rotation: true,
            max_file_size: 100,
            max_files: 5,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            standalone_mode: false,
            environment: "development".to_string(),
            debug: false,
        }
    }
}

impl BearDogConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> BearDogResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| BearDogError::Configuration {
            message: format!("Failed to read config file: {e}"),
        })?;

        let config: Self = toml::from_str(&content).map_err(|e| BearDogError::Configuration {
            message: format!("Failed to parse config file: {e}"),
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();

        // Override with environment variables
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database.url = db_url;
        }

        if let Ok(bind_addr) = std::env::var("BIND_ADDRESS") {
            config.api.http.bind_address = bind_addr;
        }

        if let Ok(port) = std::env::var("PORT") {
            config.api.http.port = port.parse().unwrap_or(8080);
        }

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration for security and correctness
    pub fn validate(&self) -> BearDogResult<()> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(BearDogError::Configuration {
                message: "Database URL cannot be empty".to_string(),
            });
        }

        // Validate API bind address
        if self.api.http.bind_address.is_empty() {
            return Err(BearDogError::Configuration {
                message: "API bind address cannot be empty".to_string(),
            });
        }

        // Validate encryption settings
        if self.encryption.key_derivation_iterations < 10000 {
            return Err(BearDogError::Configuration {
                message: "Key derivation iterations must be at least 10,000 for security"
                    .to_string(),
            });
        }

        // Validate security level constraints
        match self.security.level {
            SecurityLevel::Maximum => {
                if !self.encryption.hsm.enabled {
                    return Err(BearDogError::Configuration {
                        message: "Maximum security level requires HSM to be enabled".to_string(),
                    });
                }
                if !self.security.mfa.required {
                    return Err(BearDogError::Configuration {
                        message: "Maximum security level requires MFA to be enabled".to_string(),
                    });
                }
            }
            SecurityLevel::High => {
                if !self.security.mfa.required {
                    return Err(BearDogError::Configuration {
                        message: "High security level requires MFA to be enabled".to_string(),
                    });
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Generate a default configuration file
    pub fn generate_default_file<P: AsRef<Path>>(path: P) -> BearDogResult<()> {
        let config = Self::default();
        let toml_content =
            toml::to_string_pretty(&config).map_err(|e| BearDogError::Configuration {
                message: format!("Failed to serialize default config: {e}"),
            })?;

        std::fs::write(path, toml_content).map_err(|e| BearDogError::Configuration {
            message: format!("Failed to write config file: {e}"),
        })?;

        Ok(())
    }
}
