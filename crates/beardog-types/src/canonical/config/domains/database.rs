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
        let default_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| std::env::var("BEARDOG_DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://beardog.db".to_string()));
        
        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE);
        
        let timeout_secs = std::env::var("DATABASE_TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);
        
        let ssl = std::env::var("DATABASE_SSL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);
        
        Self {
            url: default_url,
            max_connections,
            timeout: Duration::from_secs(timeout_secs),
            ssl,
        }
    }
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        let min_idle = std::env::var("DATABASE_POOL_MIN_IDLE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        
        let max_size = std::env::var("DATABASE_POOL_MAX_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE);
        
        let idle_timeout_secs = std::env::var("DATABASE_POOL_IDLE_TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(600);
        
        Self {
            min_idle,
            max_size,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        let auto_migrate = std::env::var("DATABASE_AUTO_MIGRATE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(true);
        
        let directory = std::env::var("DATABASE_MIGRATION_DIR")
            .unwrap_or_else(|_| "migrations".to_string());
        
        Self {
            auto_migrate,
            directory,
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