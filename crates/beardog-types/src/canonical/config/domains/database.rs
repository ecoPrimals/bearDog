//! Database Domain Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Database domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDomainConfig {
    /// Primary database connection
    pub primary: DatabaseConnectionConfig,
    /// Connection pool settings
    pub pool: DatabasePoolConfig,
    /// Migration settings
    pub migrations: MigrationConfig,
}

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionConfig {
    /// Database URL
    pub url: String,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout
    pub timeout: Duration,
    /// Enable SSL
    pub ssl: bool,
}

/// Database pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePoolConfig {
    /// Minimum idle connections
    pub min_idle: u32,
    /// Maximum connections
    pub max_size: u32,
    /// Connection idle timeout
    pub idle_timeout: Duration,
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Auto-run migrations
    pub auto_migrate: bool,
    /// Migration directory
    pub directory: String,
}

impl Default for DatabaseDomainConfig {
    fn default() -> Self {
        Self {
            primary: DatabaseConnectionConfig::default(),
            pool: DatabasePoolConfig::default(),
            migrations: MigrationConfig::default(),
        }
    }
}

impl Default for DatabaseConnectionConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://beardog.db".to_string(),
            max_connections: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            timeout: Duration::from_secs(30),
            ssl: false,
        }
    }
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            min_idle: 1,
            max_size: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            idle_timeout: Duration::from_secs(600),
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            auto_migrate: true,
            directory: "migrations".to_string(),
        }
    }
}

impl DatabaseDomainConfig {
    /// Load from environment variables
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();
        
        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.primary.url = url;
        }
        
        Ok(config)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.primary.url.is_empty() {
            return Err(BearDogError::validation("Database URL cannot be empty"));
        }
        Ok(())
    }
} 