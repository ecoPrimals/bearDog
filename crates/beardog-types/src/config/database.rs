// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Database Configuration - Canonical
///
/// **UNIFIED DATABASE CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Unified Database Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedDatabaseConfig {
    pub connection: DatabaseConnectionConfig,
    pub pool: DatabasePoolConfig,
    pub ssl: DatabaseSslConfig,
    pub performance: DatabasePerformanceConfig,
}


/// **CANONICAL** Database Connection Configuration
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

/// **CANONICAL** Database Pool Configuration
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

/// **CANONICAL** Database SSL Configuration
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

/// **CANONICAL** Database Performance Configuration
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
