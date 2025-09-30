//! # Unified Configuration System - Modular Architecture
//!
//! This module provides the **single source of truth** for all `BearDog` configuration,
//! split into manageable domain modules for better maintainability.
//!
//! ## Architecture Principles
//!
//! - **Single Source of Truth**: All configuration in one logical place
//! - **Domain Organization**: Logical grouping by functional area  
//! - **Type Safety**: Comprehensive validation and error handling
//! - **Environment Awareness**: Dynamic configuration based on deployment
//! - **Modular Design**: Each domain under 300 lines for maintainability

use crate::canonical::config::r#trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Import domain-specific configurations from sibling modules
use super::{
    adapters::AdapterConfiguration,
    auth::AuthConfiguration,
    cache::CacheConfiguration,
    compliance::ComplianceConfiguration,
    database::DatabaseConfiguration,
    genetics::GeneticsConfiguration,
    hsm::HsmConfiguration,
    network::NetworkConfiguration,
    performance::PerformanceConfiguration,
    production::ProductionConfiguration,
    security::SecurityConfiguration,
    system::SystemConfiguration,
};

/// **Unified `BearDog` Configuration** - Single source of truth for all configuration
///
/// This struct consolidates all domain-specific configurations into a single,
/// comprehensive configuration system that eliminates fragmentation and provides
/// type-safe access to all `BearDog` settings.
///
/// # Architecture Benefits
///
/// - **Single Source of Truth**: All configuration in one place
/// - **Type Safety**: Comprehensive validation and error handling
/// - **Domain Organization**: Logical grouping by functional area
/// - **Environment Awareness**: Dynamic configuration based on deployment
/// - **Zero Fragmentation**: Eliminates scattered config types
///
/// # Usage Example
///
/// ```rust
/// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
///
/// // Load from environment
/// let config = UnifiedBearDogConfig::from_env()?;
///
/// // Validate configuration
/// config.validate()?;
///
/// // Access domain-specific settings
/// let api_port = config.network.port;
/// let log_level = &config.system.log_level;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedBearDogConfig {
    /// System-level configuration (versions, limits, defaults)
    pub system: SystemConfiguration,
    
    /// Network and communication configuration
    pub network: NetworkConfiguration,
    
    /// Security and cryptographic configuration
    pub security: SecurityConfiguration,
    
    /// Hardware Security Module configuration
    pub hsm: HsmConfiguration,
    
    /// Authentication and authorization configuration
    pub auth: AuthConfiguration,
    
    /// Universal adapter configuration
    pub adapters: AdapterConfiguration,
    
    /// Database connectivity configuration
    pub database: DatabaseConfiguration,
    
    /// Caching system configuration
    pub cache: CacheConfiguration,
    
    /// Genetics and evolution configuration
    pub genetics: GeneticsConfiguration,
    
    /// Production deployment configuration
    pub production: ProductionConfiguration,
    
    /// Performance optimization configuration
    pub performance: PerformanceConfiguration,
    
    /// Compliance and audit configuration
    pub compliance: ComplianceConfiguration,
}


impl UnifiedBearDogConfig {
    /// Create a new unified configuration with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Load configuration from a specific file
    ///
    /// Supports multiple formats:
    /// - TOML (.toml)
    /// - JSON (.json)
    /// - YAML (.yaml, .yml)
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if the file cannot be read or parsed
    pub fn from_file<P: Into<PathBuf>>(path: P) -> BearDogResult<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)
            .map_err(|e| BearDogError::system(format!("Failed to read config file {}: {}", path.display(), e)))?;
            
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => {
                toml::from_str(&content)
                    .map_err(|e| BearDogError::configuration(&format!("TOML parse error: {e}")))
            }
            Some("json") => {
                serde_json::from_str(&content)
                    .map_err(|e| BearDogError::configuration(&format!("JSON parse error: {e}")))
            }
            Some("yaml" | "yml") => {
                serde_yaml::from_str(&content)
                    .map_err(|e| BearDogError::configuration(&format!("YAML parse error: {e}")))
            }
            _ => Err(BearDogError::configuration("Unsupported config file format. Use .toml, .json, or .yaml"))
        }
    }
    
    /// Get configuration for a specific environment
    ///
    /// Applies environment-specific overrides and validation rules
    pub fn for_environment(env: &str) -> BearDogResult<Self> {
        let mut config = Self::from_env()?;
        
        // Apply environment-specific overrides
        match env {
            "production" => {
                config.system.environment = "production".to_string();
                config.system.log_level = "info".to_string();
                config.system.debug = false;
            }
            "development" => {
                config.system.environment = "development".to_string();
                config.system.log_level = "debug".to_string();
                config.system.debug = true;
            }
            "testing" => {
                config.system.environment = "testing".to_string();
                config.system.log_level = "warn".to_string();
                config.system.debug = false;
            }
            _ => {
                return Err(BearDogError::configuration(&format!("Unknown environment: {env}. Use 'production', 'development', or 'testing'")));
            }
        }
        
        config.validate()?;
        Ok(config)
    }
}

impl BearDogConfig for UnifiedBearDogConfig {
    fn validate(&self) -> BearDogResult<()> {
        // Validate all domain configurations
        self.system.validate()
            .map_err(|e| BearDogError::configuration(&format!("System validation error: {e}")))?;
            
        self.network.validate()
            .map_err(|e| BearDogError::configuration(&format!("Network validation error: {e}")))?;
            
        self.security.validate()
            .map_err(|e| BearDogError::configuration(&format!("Security validation error: {e}")))?;
            
        self.hsm.validate()
            .map_err(|e| BearDogError::configuration(&format!("HSM validation error: {e}")))?;
            
        self.auth.validate()
            .map_err(|e| BearDogError::configuration(&format!("Auth validation error: {e}")))?;
            
        self.adapters.validate()
            .map_err(|e| BearDogError::configuration(&format!("Adapters validation error: {e}")))?;
            
        self.database.validate()
            .map_err(|e| BearDogError::configuration(&format!("Database validation error: {e}")))?;
            
        self.cache.validate()
            .map_err(|e| BearDogError::configuration(&format!("Cache validation error: {e}")))?;
            
        self.genetics.validate()
            .map_err(|e| BearDogError::configuration(&format!("Genetics validation error: {e}")))?;
            
        self.production.validate()
            .map_err(|e| BearDogError::configuration(&format!("Production validation error: {e}")))?;
            
        self.performance.validate()
            .map_err(|e| BearDogError::configuration(&format!("Performance validation error: {e}")))?;
            
        self.compliance.validate()
            .map_err(|e| BearDogError::configuration(&format!("Compliance validation error: {e}")))?;
        
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        Ok(Self {
            system: self.system.merge(&other.system)?,
            network: self.network.merge(&other.network)?,
            security: self.security.merge(&other.security)?,
            hsm: self.hsm.merge(&other.hsm)?,
            auth: self.auth.merge(&other.auth)?,
            adapters: self.adapters.merge(&other.adapters)?,
            database: self.database.merge(&other.database)?,
            cache: self.cache.merge(&other.cache)?,
            genetics: self.genetics.merge(&other.genetics)?,
            production: self.production.merge(&other.production)?,
            performance: self.performance.merge(&other.performance)?,
            compliance: self.compliance.merge(&other.compliance)?,
        })
    }
    
    fn from_env() -> BearDogResult<Self> {
        Ok(Self {
            system: SystemConfiguration::from_env()?,
            network: NetworkConfiguration::from_env()?,
            security: SecurityConfiguration::from_env()?,
            hsm: HsmConfiguration::from_env()?,
            auth: AuthConfiguration::from_env()?,
            adapters: AdapterConfiguration::from_env()?,
            database: DatabaseConfiguration::from_env()?,
            cache: CacheConfiguration::from_env()?,
            genetics: GeneticsConfiguration::from_env()?,
            production: ProductionConfiguration::from_env()?,
            performance: PerformanceConfiguration::from_env()?,
            compliance: ComplianceConfiguration::from_env()?,
        })
    }
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::configuration(&format!("Failed to serialize config to TOML: {e}")))
    }
    
    fn domain() -> &'static str {
        "unified"
    }
} 