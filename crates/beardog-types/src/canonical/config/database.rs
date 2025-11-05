//! Database Configuration
//!
//! Canonical database configuration for managing database connections and operations.
//!
//! # Overview
//!
//! `CanonicalDatabaseConfig` provides comprehensive database settings including:
//! - Connection management and pooling
//! - Timeout configuration for connections and queries
//! - Enable/disable toggle for testing scenarios
//! - Connection string management
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::database::CanonicalDatabaseConfig;
//! use std::time::Duration;
//!
//! // Create database configuration
//! let config = CanonicalDatabaseConfig {
//!     enabled: true,
//!     connection_string: "postgresql://localhost:5432/beardog".to_string(),
//!     max_connections: 10,
//!     connection_timeout: Duration::from_secs(5),
//!     query_timeout: Duration::from_secs(30),
//! };
//!
//! assert!(config.enabled);
//! assert_eq!(config.max_connections, 10);
//! ```
//!
//! # Production Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::database::CanonicalDatabaseConfig;
//! use std::time::Duration;
//!
//! // Production database with high concurrency
//! let config = CanonicalDatabaseConfig {
//!     enabled: true,
//!     connection_string: "postgresql://prod-db:5432/beardog?sslmode=require".to_string(),
//!     max_connections: 100,  // Higher for production load
//!     connection_timeout: Duration::from_secs(10),
//!     query_timeout: Duration::from_secs(60),
//! };
//! ```
//!
//! # Connection Pooling
//!
//! ```rust
//! use beardog_types::canonical::config::database::CanonicalDatabaseConfig;
//! use std::time::Duration;
//!
//! // Configure connection pool size based on expected load
//! let config = CanonicalDatabaseConfig {
//!     enabled: true,
//!     connection_string: "postgresql://localhost/db".to_string(),
//!     max_connections: 50,  // Pool size
//!     connection_timeout: Duration::from_secs(5),
//!     query_timeout: Duration::from_secs(30),
//! };
//!
//! // Rule of thumb: max_connections = (number of cores) * 2 + effective_spindle_count
//! ```
//!
//! # Testing Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::database::CanonicalDatabaseConfig;
//! use std::time::Duration;
//!
//! // Disable database for unit tests
//! let test_config = CanonicalDatabaseConfig {
//!     enabled: false,  // Skip database operations
//!     ..Default::default()
//! };
//!
//! if !test_config.enabled {
//!     // Use in-memory database instead
//! }
//! ```
//!
//! # Security Considerations
//!
//! - **SSL/TLS**: Always use `sslmode=require` in production connection strings
//! - **Credentials**: Load connection strings from environment variables, not config files
//! - **Timeouts**: Set appropriate timeouts to prevent hung connections
//! - **Connection Limits**: Set `max_connections` to avoid exhausting database resources
//!
//! # Performance Tips
//!
//! - Connection pool size should match expected concurrent queries
//! - Query timeouts prevent long-running queries from blocking connections
//! - Monitor connection pool usage and adjust `max_connections` accordingly
//!
//! # Design Principles
//!
//! - **Configurability**: All parameters exposed for fine-tuning
//! - **Safety**: Timeouts prevent resource exhaustion
//! - **Testability**: Can be disabled for testing
//! - **Security**: Supports secure connection parameters

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical database configuration
///
/// Comprehensive configuration for database connections, pooling, and operations.
///
/// # Fields
///
/// * `enabled` - Whether database functionality is active (disable for testing)
/// * `connection_string` - Database connection URL (e.g., "postgresql://host:port/db")
/// * `max_connections` - Maximum number of concurrent database connections (pool size)
/// * `connection_timeout` - Maximum time to wait for a connection from the pool
/// * `query_timeout` - Maximum time to wait for a query to complete
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::database::CanonicalDatabaseConfig;
/// use std::time::Duration;
///
/// let config = CanonicalDatabaseConfig {
///     enabled: true,
///     connection_string: "postgresql://localhost/mydb".to_string(),
///     max_connections: 20,
///     connection_timeout: Duration::from_secs(5),
///     query_timeout: Duration::from_secs(30),
/// };
/// ```
///
/// # Connection String Formats
///
/// - PostgreSQL: `postgresql://user:pass@host:port/database?sslmode=require`
/// - MySQL: `mysql://user:pass@host:port/database`
/// - SQLite: `sqlite:///path/to/database.db` or `sqlite::memory:` for in-memory
///
/// # Performance Tuning
///
/// - Set `max_connections` based on expected concurrent load
/// - Use connection pooling for better performance under load
/// - Adjust timeouts based on query complexity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalDatabaseConfig {
    /// Whether database functionality is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Database connection string
    /// The connection string value
    pub connection_string: String,
    /// Maximum number of database connections
    /// Number of `max_connections`
    pub max_connections: u32,
    /// Database connection timeout
    pub connection_timeout: Duration,
    /// Database query timeout
    pub query_timeout: Duration,
}

pub type DatabaseConfig = CanonicalDatabaseConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_default() {
        let config = CanonicalDatabaseConfig::default();
        assert!(!config.enabled);
        assert!(config.connection_string.is_empty());
        assert_eq!(config.max_connections, 0);
    }

    #[test]
    fn test_database_config_enabled() {
        let config = CanonicalDatabaseConfig {
            enabled: true,
            connection_string: "postgresql://localhost:5432/beardog".to_string(),
            max_connections: 10,
            connection_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DB_CONNECTION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
            ),
            query_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DB_QUERY_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
        };
        assert!(config.enabled);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.max_connections, 10);
        assert!(config.connection_string.contains("postgresql"));
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_database_config_disabled_for_testing() {
        let config = CanonicalDatabaseConfig {
            enabled: false,
            ..Default::default()
        };
        assert!(!config.enabled);
    }

    #[test]
    fn test_database_config_connection_timeout() {
        let config = CanonicalDatabaseConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            connection_timeout: Duration::from_secs(10),
            ..Default::default()
        };
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_database_config_query_timeout() {
        let config = CanonicalDatabaseConfig {
            query_timeout: Duration::from_secs(60),
            ..Default::default()
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.query_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_database_config_high_concurrency() {
        let config = CanonicalDatabaseConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            enabled: true,
            max_connections: 100,
            ..Default::default()
        };
        assert_eq!(config.max_connections, 100);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_type_alias() {
        let _config: DatabaseConfig = CanonicalDatabaseConfig::default();
    }
}
