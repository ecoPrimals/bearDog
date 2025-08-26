

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct StorageConfig {

    pub database: DatabaseConfig,

    pub pool: ConnectionPoolConfig,

    pub ssl: SslConfig,

    pub cache_warming: CacheWarmingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {

    pub url: String,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {

    pub min_size: u32,

    pub max_size: u32,

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

pub use super::performance::CacheConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SslConfig {

    pub enabled: bool,

    pub cert_path: Option<String>,

    pub key_path: Option<String>,

    pub ca_cert_path: Option<String>,

    pub verify_mode: SslVerifyMode,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheWarmingConfig {

    pub enabled: bool,

    pub strategies: Vec<WarmingStrategy>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarmingStrategy {

    PrePopulate,

    Gradual,

    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmingSchedule {

    pub interval: Duration,

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
