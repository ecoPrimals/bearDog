// Canonical Database Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical database configuration
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
