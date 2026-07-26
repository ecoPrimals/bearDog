// SPDX-License-Identifier: AGPL-3.0-or-later

//! Database Domain Configuration

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Database domain configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl DatabaseConnectionConfig {
    /// Default database URL
    pub const DEFAULT_URL: &'static str = "sqlite://beardog.db";

    /// Default timeout in seconds
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Default SSL setting
    pub const DEFAULT_SSL: bool = false;

    /// Create `DatabaseConnectionConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for connection limits"
    )]
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            url: Self::DEFAULT_URL.to_string(),
            max_connections: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            timeout: Duration::from_secs(Self::DEFAULT_TIMEOUT_SECS),
            ssl: Self::DEFAULT_SSL,
        }
    }

    /// Create `DatabaseConnectionConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `DATABASE_URL` or `BEARDOG_DATABASE_URL`: Connection URL (default: "<sqlite://beardog.db>")
    /// - `DATABASE_MAX_CONNECTIONS`: Maximum connections (default: `DEFAULT_POOL_SIZE`)
    /// - `DATABASE_TIMEOUT_SECONDS`: Connection timeout (default: 30)
    /// - `DATABASE_SSL`: Enable SSL/TLS (default: false)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for connection limits"
    )]
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        let default_url = get("DATABASE_URL")
            .or_else(|| get("BEARDOG_DATABASE_URL"))
            .unwrap_or_else(|| Self::DEFAULT_URL.to_string());

        let max_connections = get("DATABASE_MAX_CONNECTIONS")
            .and_then(|v| v.parse().ok())
            .unwrap_or(crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32);

        let timeout_secs = get("DATABASE_TIMEOUT_SECONDS")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_TIMEOUT_SECS);

        let ssl = get("DATABASE_SSL")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_SSL);

        Self {
            url: default_url,
            max_connections,
            timeout: Duration::from_secs(timeout_secs),
            ssl,
        }
    }
}

impl Default for DatabaseConnectionConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl DatabasePoolConfig {
    /// Default minimum idle connections
    pub const DEFAULT_MIN_IDLE: u32 = 1;

    /// Default idle timeout in seconds
    pub const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 600;

    /// Create `DatabasePoolConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for pool sizing"
    )]
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            min_idle: Self::DEFAULT_MIN_IDLE,
            max_size: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            idle_timeout: Duration::from_secs(Self::DEFAULT_IDLE_TIMEOUT_SECS),
        }
    }

    /// Create `DatabasePoolConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `DATABASE_POOL_MIN_IDLE`: Minimum idle connections (default: 1)
    /// - `DATABASE_POOL_MAX_SIZE`: Maximum pool size (default: `DEFAULT_POOL_SIZE`)
    /// - `DATABASE_POOL_IDLE_TIMEOUT_SECONDS`: Idle timeout (default: 600)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for pool sizing"
    )]
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        let min_idle = get("DATABASE_POOL_MIN_IDLE")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_MIN_IDLE);

        let max_size = get("DATABASE_POOL_MAX_SIZE")
            .and_then(|v| v.parse().ok())
            .unwrap_or(crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32);

        let idle_timeout_secs = get("DATABASE_POOL_IDLE_TIMEOUT_SECONDS")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_IDLE_TIMEOUT_SECS);

        Self {
            min_idle,
            max_size,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
        }
    }
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl MigrationConfig {
    /// Default auto-migrate setting
    pub const DEFAULT_AUTO_MIGRATE: bool = true;

    /// Default migration directory
    pub const DEFAULT_DIRECTORY: &'static str = "migrations";

    /// Create `MigrationConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            auto_migrate: Self::DEFAULT_AUTO_MIGRATE,
            directory: Self::DEFAULT_DIRECTORY.to_string(),
        }
    }

    /// Create `MigrationConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `DATABASE_AUTO_MIGRATE`: Enable auto-migration (default: true)
    /// - `DATABASE_MIGRATION_DIR`: Migration directory (default: "migrations")
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        let auto_migrate = get("DATABASE_AUTO_MIGRATE")
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_AUTO_MIGRATE);

        let directory =
            get("DATABASE_MIGRATION_DIR").unwrap_or_else(|| Self::DEFAULT_DIRECTORY.to_string());

        Self {
            auto_migrate,
            directory,
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl DatabaseDomainConfig {
    /// Load from environment variables
    ///
    /// # Errors
    ///
    /// This function currently always returns `Ok`; merges optional `DATABASE_URL` when set.
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Ok(url) = std::env::var(env_keys::ENV_DATABASE_URL) {
            config.primary.url = url;
        }

        Ok(config)
    }

    /// Validate configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the primary database URL is empty.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.primary.url.is_empty() {
            return Err(BearDogError::validation("Database URL cannot be empty"));
        }
        Ok(())
    }
}
