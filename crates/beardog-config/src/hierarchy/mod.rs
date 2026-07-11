// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration hierarchy resolution
//!
//! Implements the zero-hardcoding configuration hierarchy:
//! 1. CLI arguments (highest priority)
//! 2. Environment variables
//! 3. Configuration file
//! 4. Platform-specific defaults
//! 5. Secure fallback defaults (lowest priority)
//!
//! ## Design
//!
//! The hierarchy uses a layered approach where each layer can override
//! values from lower layers. This enables maximum flexibility while
//! maintaining secure defaults.
//!
//! ```text
//! Priority: High → Low
//! ┌──────────────────────┐
//! │  CLI Arguments       │ ← --port 9000
//! ├──────────────────────┤
//! │  Environment Vars    │ ← BEARDOG_PORT=8080
//! ├──────────────────────┤
//! │  Config File         │ ← config.toml
//! ├──────────────────────┤
//! │  Platform Defaults   │ ← OS-specific
//! ├──────────────────────┤
//! │  Fallback Defaults   │ ← Secure defaults
//! └──────────────────────┘
//! ```

mod overrides;
#[cfg(test)]
mod tests;

use crate::{BearDogConfig, ConfigError, ConfigResult};
use overrides::{apply_cli_overrides, apply_env_overrides};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration source with priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigSource {
    /// Secure fallback defaults (lowest priority)
    FallbackDefaults = 0,
    /// Platform-specific defaults
    PlatformDefaults = 1,
    /// Configuration file
    ConfigFile = 2,
    /// Environment variables
    Environment = 3,
    /// CLI arguments (highest priority)
    CliArgs = 4,
}

/// Configuration value with source tracking
#[derive(Debug, Clone)]
pub struct ConfigValue<T> {
    /// Resolved value after applying the configuration hierarchy up to this layer.
    pub value: T,
    /// Which layer supplied `value`, used by [`Self::merge`] to pick the higher-precedence source.
    pub source: ConfigSource,
}

impl<T> ConfigValue<T> {
    /// Wraps `value` and records which configuration layer it came from.
    pub const fn new(value: T, source: ConfigSource) -> Self {
        Self { value, source }
    }

    /// Merge with another value, keeping the higher priority one
    pub fn merge(self, other: Self) -> Self {
        if other.source >= self.source {
            other
        } else {
            self
        }
    }
}

/// Configuration hierarchy builder
#[derive(Debug)]
pub struct ConfigHierarchy {
    /// Base configuration (defaults)
    base: BearDogConfig,
    /// File-based overrides
    file_config: Option<(BearDogConfig, PathBuf)>,
    /// Environment variable overrides
    env_overrides: HashMap<String, String>,
    /// CLI argument overrides
    cli_overrides: HashMap<String, String>,
}

impl ConfigHierarchy {
    /// Create a new hierarchy with fallback defaults
    pub fn new() -> Self {
        Self {
            base: BearDogConfig::default(),
            file_config: None,
            env_overrides: HashMap::new(),
            cli_overrides: HashMap::new(),
        }
    }

    /// Apply platform-specific defaults
    pub const fn with_platform_defaults(self) -> Self {
        self
    }

    /// Load configuration from file
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when the file cannot be read or the format is invalid.
    pub fn with_file<P: AsRef<std::path::Path>>(mut self, path: P) -> ConfigResult<Self> {
        let path = path.as_ref();

        let contents = std::fs::read_to_string(path).map_err(|e| {
            ConfigError::invalid_value("config_file", format!("Failed to read: {e}"))
        })?;

        let config: BearDogConfig = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&contents).map_err(|e| {
                ConfigError::invalid_value("config_file", format!("Invalid TOML: {e}"))
            })?
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&contents).map_err(|e| {
                ConfigError::invalid_value("config_file", format!("Invalid JSON: {e}"))
            })?
        } else {
            return Err(ConfigError::invalid_value(
                "config_file",
                "Unsupported format. Use .toml or .json",
            ));
        };

        self.file_config = Some((config, path.to_path_buf()));
        Ok(self)
    }

    /// Load from environment variables
    pub fn with_env_vars(mut self) -> Self {
        for (key, value) in std::env::vars() {
            if key.starts_with("BEARDOG_") {
                self.env_overrides.insert(key, value);
            }
        }
        self
    }

    /// Add CLI argument override
    pub fn with_cli_arg(mut self, key: String, value: String) -> Self {
        self.cli_overrides.insert(key, value);
        self
    }

    /// Add multiple CLI arguments
    pub fn with_cli_args(mut self, args: HashMap<String, String>) -> Self {
        self.cli_overrides.extend(args);
        self
    }

    /// Build the final configuration by merging all layers
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when environment/CLI overrides cannot be applied or final validation fails.
    pub fn build(self) -> ConfigResult<BearDogConfig> {
        let mut config = self.base;

        if let Some((file_config, _path)) = self.file_config {
            config = merge_configs(config, file_config, ConfigSource::ConfigFile);
        }

        if !self.env_overrides.is_empty() {
            config = apply_env_overrides(config, &self.env_overrides)?;
        }

        if !self.cli_overrides.is_empty() {
            config = apply_cli_overrides(config, &self.cli_overrides)?;
        }

        config.validate()?;

        Ok(config)
    }

    /// Auto-discover and load config file from standard locations
    ///
    /// # Errors
    ///
    /// Propagates errors from [`Self::with_file`] when a discovered file cannot be loaded.
    pub fn with_auto_config_file(self) -> ConfigResult<Self> {
        let mut locations = vec![
            PathBuf::from("beardog.toml"),
            PathBuf::from("config.toml"),
            PathBuf::from("/etc/beardog/config.toml"),
        ];

        if let Some(base_dirs) = directories::BaseDirs::new() {
            locations.push(base_dirs.config_dir().join("beardog/config.toml"));
        }

        for location in &locations {
            if location.exists() {
                return self.with_file(location);
            }
        }

        Ok(self)
    }
}

impl Default for ConfigHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

/// Merge two configurations, preferring values from the override config.
///
/// Fields with `PartialEq` are compared and only overridden when different.
/// Fields without `PartialEq` (paths, hsm) always take the override.
fn merge_configs(
    base: BearDogConfig,
    override_cfg: BearDogConfig,
    _source: ConfigSource,
) -> BearDogConfig {
    BearDogConfig {
        network: if override_cfg.network == base.network {
            base.network
        } else {
            override_cfg.network
        },
        paths: override_cfg.paths,
        hsm: override_cfg.hsm,
        timeouts: if override_cfg.timeouts == base.timeouts {
            base.timeouts
        } else {
            override_cfg.timeouts
        },
        security: if override_cfg.security == base.security {
            base.security
        } else {
            override_cfg.security
        },
        crypto: if override_cfg.crypto == base.crypto {
            base.crypto
        } else {
            override_cfg.crypto
        },
        limits: if override_cfg.limits == base.limits {
            base.limits
        } else {
            override_cfg.limits
        },
        monitoring: if override_cfg.monitoring == base.monitoring {
            base.monitoring
        } else {
            override_cfg.monitoring
        },
        capacity: if override_cfg.capacity == base.capacity {
            base.capacity
        } else {
            override_cfg.capacity
        },
    }
}
