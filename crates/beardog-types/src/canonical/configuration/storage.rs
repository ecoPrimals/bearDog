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


/// # Canonical Storage Configuration Module
///
/// This module provides canonical storage configuration types including
/// database connections, caching, and SSL settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL STORAGE CONFIGURATION** - Main storage settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct StorageConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Connection pool configuration
    pub pool: ConnectionPoolConfig,
    /// SSL configuration
    pub ssl: SslConfig,
    /// Cache warming configuration
    pub cache_warming: CacheWarmingConfig,
}


/// **CANONICAL DATABASE CONFIGURATION** - Database connection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Connection pool configuration
    pub pool: ConnectionPoolConfig,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://localhost:5432/beardog".to_string(),
            pool: ConnectionPoolConfig::default(),
        }
    }
}

/// **CANONICAL CONNECTION POOL CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Minimum pool size
    pub min_size: u32,
    /// Maximum pool size
    pub max_size: u32,
    /// Connection idle timeout
    pub idle_timeout: Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: 10,
            idle_timeout: Duration::from_secs(600), // 10 minutes
        }
    }
}

/// **CANONICAL CACHE CONFIGURATION** - Re-exported from performance module
pub use super::performance::CacheConfig;

/// **CANONICAL SSL CONFIGURATION** - SSL/TLS settings for database connections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SslConfig {
    /// Whether SSL is enabled
    pub enabled: bool,
    /// SSL certificate path
    pub cert_path: Option<String>,
    /// SSL key path
    pub key_path: Option<String>,
    /// SSL CA certificate path
    pub ca_cert_path: Option<String>,
    /// SSL verification mode
    pub verify_mode: SslVerifyMode,
}


/// SSL verification modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SslVerifyMode {
    None,
    Required,
    VerifyCA,
    VerifyFull,
}

impl Default for SslVerifyMode {
    fn default() -> Self {
        Self::None
    }
}

/// **CANONICAL CACHE WARMING CONFIGURATION** - Cache warming settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheWarmingConfig {
    /// Whether cache warming is enabled
    pub enabled: bool,
    /// Warm-up strategies
    pub strategies: Vec<WarmingStrategy>,
    /// Warm-up schedule
    pub schedule: WarmingSchedule,
}

impl Default for CacheWarmingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategies: vec![WarmingStrategy::PrePopulate],
            schedule: WarmingSchedule::default(),
        }
    }
}

/// Cache warming strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarmingStrategy {
    /// Pre-populate with common queries
    PrePopulate,
    /// Gradual warming
    Gradual,
    /// Background warming
    Background,
}

/// Cache warming schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmingSchedule {
    /// Warming interval
    pub interval: Duration,
    /// Batch size for warming
    pub batch_size: u32,
}

impl Default for WarmingSchedule {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(3600), // 1 hour
            batch_size: 100,
        }
    }
}
