

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedDatabaseConfig {
    pub connection: DatabaseConnectionConfig,
    pub pool: DatabasePoolConfig,
    pub ssl: DatabaseSslConfig,
    pub performance: DatabasePerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionConfig {
    pub url: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Default for DatabaseConnectionConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://localhost:5432/beardog".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "beardog".to_string(),
            username: "beardog".to_string(),
            password: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSslConfig {
    pub enabled: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
    pub verify_mode: String,
}

impl Default for DatabaseSslConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: None,
            key_path: None,
            ca_path: None,
            verify_mode: "none".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePerformanceConfig {
    pub statement_cache_size: usize,
    pub query_timeout: Duration,
    pub enable_prepared_statements: bool,
    pub enable_query_logging: bool,
}

impl Default for DatabasePerformanceConfig {
    fn default() -> Self {
        Self {
            statement_cache_size: 100,
            query_timeout: Duration::from_secs(30),
            enable_prepared_statements: true,
            enable_query_logging: false,
        }
    }
}
