//! Core configuration types and validation
//! 
//! Contains the main BearDogConfig struct and core configuration logic.

use crate::error::BearDogResult;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

use super::security::*;
use super::network::*;
use super::monitoring::*;
use super::integration::*;

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://beardog.db".to_string(),
            max_connections: 10,
            connection_timeout_seconds: 30,
        }
    }
}

impl BearDogConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> BearDogResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::error::BearDogError::Configuration {
                message: format!("Failed to read config file: {}", e),
            })?;
        
        let config: Self = toml::from_str(&content)
            .map_err(|e| crate::error::BearDogError::Configuration {
                message: format!("Failed to parse config file: {}", e),
            })?;
        
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables with secure defaults
    pub fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();

        // Database configuration
        if let Ok(db_url) = env::var("DATABASE_URL") {
            config.database.url = db_url;
        }

        // API configuration
        if let Ok(bind_addr) = env::var("BEARDOG_BIND_ADDRESS") {
            config.api.bind_address = bind_addr;
        }

        // Security configuration
        if let Ok(security_level) = env::var("BEARDOG_SECURITY_LEVEL") {
            match security_level.to_lowercase().as_str() {
                "standard" => config.security.level = SecurityLevel::Standard,
                "high" => config.security.level = SecurityLevel::High,
                "maximum" => config.security.level = SecurityLevel::Maximum,
                _ => {} // Keep default
            }
        }

        // Validate the configuration
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration for security and correctness
    pub fn validate(&self) -> BearDogResult<()> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(crate::error::BearDogError::Configuration {
                message: "Database URL cannot be empty".to_string(),
            });
        }

        // Validate API bind address
        if self.api.bind_address.is_empty() {
            return Err(crate::error::BearDogError::Configuration {
                message: "API bind address cannot be empty".to_string(),
            });
        }

        // Validate encryption settings
        if self.encryption.key_derivation_iterations < 10000 {
            return Err(crate::error::BearDogError::Configuration {
                message: "Key derivation iterations must be at least 10,000 for security".to_string(),
            });
        }

        // Validate security level constraints
        match self.security.level {
            SecurityLevel::Maximum => {
                if !self.encryption.hsm.enabled {
                    return Err(crate::error::BearDogError::Configuration {
                        message: "Maximum security level requires HSM to be enabled".to_string(),
                    });
                }
                if !self.security.mfa.required {
                    return Err(crate::error::BearDogError::Configuration {
                        message: "Maximum security level requires MFA to be enabled".to_string(),
                    });
                }
            }
            SecurityLevel::High => {
                if self.security.max_failed_logins > 5 {
                    return Err(crate::error::BearDogError::Configuration {
                        message: "High security level requires max failed logins ≤ 5".to_string(),
                    });
                }
            }
            SecurityLevel::Standard => {
                // Standard level has more lenient requirements
            }
        }

        Ok(())
    }

    /// Generate a default configuration file
    pub fn generate_default_file<P: AsRef<Path>>(path: P) -> BearDogResult<()> {
        let config = Self::default();
        let toml_content = toml::to_string_pretty(&config)
            .map_err(|e| crate::error::BearDogError::Configuration {
                message: format!("Failed to serialize default config: {}", e),
            })?;
        
        std::fs::write(path, toml_content)
            .map_err(|e| crate::error::BearDogError::Configuration {
                message: format!("Failed to write config file: {}", e),
            })?;
        
        Ok(())
    }
} 