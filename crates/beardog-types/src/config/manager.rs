

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedConfigManager {
    pub sources: ConfigSourcesConfig,
    pub validation: ConfigValidationConfig,
    pub caching: ConfigCachingConfig,
    pub reloading: ConfigReloadingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSourcesConfig {
    pub file_sources: Vec<PathBuf>,
    pub environment_prefix: String,
    pub remote_sources: Vec<String>,
    pub priority_order: Vec<String>,
}

impl Default for ConfigSourcesConfig {
    fn default() -> Self {
        Self {
            file_sources: vec![PathBuf::from("config.toml")],
            environment_prefix: "BEARDOG_".to_string(),
            remote_sources: Vec::new(),
            priority_order: vec!["environment".to_string(), "file".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationConfig {
    pub enabled: bool,
    pub strict_mode: bool,
    pub schema_validation: bool,
    pub custom_validators: Vec<String>,
}

impl Default for ConfigValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strict_mode: false,
            schema_validation: true,
            custom_validators: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCachingConfig {
    pub enabled: bool,
    pub cache_ttl: Duration,
    pub max_cache_size: usize,
    pub cache_strategy: String,
}

impl Default for ConfigCachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            max_cache_size: 1000,
            cache_strategy: "lru".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigReloadingConfig {
    pub enabled: bool,
    pub watch_files: bool,
    pub reload_interval: Duration,
    pub graceful_reload: bool,
}

impl Default for ConfigReloadingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_files: true,
            reload_interval: Duration::from_secs(60),
            graceful_reload: true,
        }
    }
}
