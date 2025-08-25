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


/// # Configuration Manager - Canonical
///
/// **UNIFIED CONFIGURATION MANAGER** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// **CANONICAL** Unified Configuration Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedConfigManager {
    pub sources: ConfigSourcesConfig,
    pub validation: ConfigValidationConfig,
    pub caching: ConfigCachingConfig,
    pub reloading: ConfigReloadingConfig,
}


/// **CANONICAL** Configuration Sources Configuration
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

/// **CANONICAL** Configuration Validation Configuration
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

/// **CANONICAL** Configuration Caching Configuration
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

/// **CANONICAL** Configuration Reloading Configuration
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
