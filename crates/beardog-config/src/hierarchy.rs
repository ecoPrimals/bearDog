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

use crate::{BearDogConfig, ConfigError, ConfigResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
    pub value: T,
    pub source: ConfigSource,
}

impl<T> ConfigValue<T> {
    pub fn new(value: T, source: ConfigSource) -> Self {
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
    pub fn with_platform_defaults(self) -> Self {
        // Platform-specific path detection happens in PathConfig::default()
        // which is already called in BearDogConfig::default()
        // This is a no-op but kept for API clarity
        self
    }

    /// Load configuration from file
    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> ConfigResult<Self> {
        let path = path.as_ref();

        // Read file contents
        let contents = std::fs::read_to_string(path).map_err(|e| {
            ConfigError::invalid_value("config_file", format!("Failed to read: {}", e))
        })?;

        // Parse based on extension
        let config: BearDogConfig = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&contents).map_err(|e| {
                ConfigError::invalid_value("config_file", format!("Invalid TOML: {}", e))
            })?
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&contents).map_err(|e| {
                ConfigError::invalid_value("config_file", format!("Invalid JSON: {}", e))
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
        // Collect all BEARDOG_* environment variables
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
    pub fn build(self) -> ConfigResult<BearDogConfig> {
        // Start with base (fallback + platform defaults)
        let mut config = self.base;

        // Layer 1: File config (if provided)
        if let Some((file_config, _path)) = self.file_config {
            config = merge_configs(config, file_config, ConfigSource::ConfigFile);
        }

        // Layer 2: Environment variables
        if !self.env_overrides.is_empty() {
            config = apply_env_overrides(config, &self.env_overrides)?;
        }

        // Layer 3: CLI arguments (highest priority)
        if !self.cli_overrides.is_empty() {
            config = apply_cli_overrides(config, &self.cli_overrides)?;
        }

        // Validate the final configuration
        config.validate()?;

        Ok(config)
    }

    /// Auto-discover and load config file from standard locations
    pub fn with_auto_config_file(self) -> ConfigResult<Self> {
        // Try standard locations in order
        let mut locations = vec![
            PathBuf::from("beardog.toml"),
            PathBuf::from("config.toml"),
            PathBuf::from("/etc/beardog/config.toml"),
        ];

        // Add user config dir if available
        if let Some(config_dir) = dirs::config_dir() {
            locations.push(config_dir.join("beardog/config.toml"));
        }

        for location in &locations {
            if location.exists() {
                return self.with_file(location);
            }
        }

        // No config file found - that's OK, continue with other sources
        Ok(self)
    }
}

impl Default for ConfigHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

/// Merge two configurations, preferring values from the override config
///
/// Implements intelligent field-by-field merging:
/// - Fields with PartialEq: Compare and use override only if different
/// - Fields without PartialEq (paths, hsm): Always use override
///
/// This enables partial configuration updates while preserving unchanged values.
fn merge_configs(
    base: BearDogConfig,
    override_cfg: BearDogConfig,
    _source: ConfigSource,
) -> BearDogConfig {
    // Field-by-field merge strategy for partial overrides
    BearDogConfig {
        network: if override_cfg.network != base.network {
            override_cfg.network
        } else {
            base.network
        },
        paths: override_cfg.paths, // No PartialEq - always use override
        hsm: override_cfg.hsm,     // No PartialEq - always use override
        timeouts: if override_cfg.timeouts != base.timeouts {
            override_cfg.timeouts
        } else {
            base.timeouts
        },
        security: if override_cfg.security != base.security {
            override_cfg.security
        } else {
            base.security
        },
        crypto: if override_cfg.crypto != base.crypto {
            override_cfg.crypto
        } else {
            base.crypto
        },
        limits: if override_cfg.limits != base.limits {
            override_cfg.limits
        } else {
            base.limits
        },
        monitoring: if override_cfg.monitoring != base.monitoring {
            override_cfg.monitoring
        } else {
            base.monitoring
        },
        capacity: if override_cfg.capacity != base.capacity {
            override_cfg.capacity
        } else {
            base.capacity
        },
    }
}

/// Apply environment variable overrides
fn apply_env_overrides(
    mut config: BearDogConfig,
    env_vars: &HashMap<String, String>,
) -> ConfigResult<BearDogConfig> {
    // Common environment variable mappings
    // Network
    if let Some(port) = env_vars.get("BEARDOG_API_PORT") {
        config.network.api.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_API_PORT", format!("Invalid port: {}", port))
        })?;
    }
    if let Some(addr) = env_vars.get("BEARDOG_API_BIND_ADDRESS") {
        config.network.api.bind_address = addr.parse().map_err(|_| {
            ConfigError::invalid_value(
                "BEARDOG_API_BIND_ADDRESS",
                format!("Invalid address: {}", addr),
            )
        })?;
    }
    if let Some(port) = env_vars.get("BEARDOG_DISCOVERY_PORT") {
        config.network.discovery.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_DISCOVERY_PORT", format!("Invalid port: {}", port))
        })?;
    }
    if let Some(port) = env_vars.get("BEARDOG_ADMIN_PORT") {
        config.network.admin.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_ADMIN_PORT", format!("Invalid port: {}", port))
        })?;
    }

    // Paths
    if let Some(config_dir) = env_vars.get("BEARDOG_CONFIG_DIR") {
        config.paths.config_dir = PathBuf::from(config_dir);
    }
    if let Some(data_dir) = env_vars.get("BEARDOG_DATA_DIR") {
        config.paths.data_dir = PathBuf::from(data_dir);
    }
    if let Some(log_dir) = env_vars.get("BEARDOG_LOG_DIR") {
        config.paths.log_dir = PathBuf::from(log_dir);
    }

    // Timeouts
    if let Some(timeout) = env_vars.get("BEARDOG_HSM_TIMEOUT") {
        config.timeouts.hsm_operation_secs = timeout.parse().map_err(|_| {
            ConfigError::invalid_value(
                "BEARDOG_HSM_TIMEOUT",
                format!("Invalid timeout: {}", timeout),
            )
        })?;
    }

    // Security
    if let Some(strict) = env_vars.get("BEARDOG_STRICT_MODE") {
        config.security.strict_mode = strict.parse().map_err(|_| {
            ConfigError::invalid_value(
                "BEARDOG_STRICT_MODE",
                format!("Invalid boolean: {}", strict),
            )
        })?;
    }

    // Monitoring
    if let Some(level) = env_vars.get("BEARDOG_LOG_LEVEL") {
        config.monitoring.log_level = level.clone();
    }

    Ok(config)
}

/// Apply CLI argument overrides
fn apply_cli_overrides(
    mut config: BearDogConfig,
    cli_args: &HashMap<String, String>,
) -> ConfigResult<BearDogConfig> {
    // CLI arguments have the same mappings as env vars
    // but with different key names (use -- prefix convention)

    if let Some(port) = cli_args.get("port").or_else(|| cli_args.get("api-port")) {
        config.network.api.port = port
            .parse()
            .map_err(|_| ConfigError::invalid_value("port", format!("Invalid port: {}", port)))?;
    }
    if let Some(addr) = cli_args.get("bind-address") {
        config.network.api.bind_address = addr.parse().map_err(|_| {
            ConfigError::invalid_value("bind-address", format!("Invalid address: {}", addr))
        })?;
    }
    if let Some(config_file) = cli_args.get("config") {
        config.paths.config_dir = PathBuf::from(config_file)
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();
    }
    if let Some(level) = cli_args.get("log-level") {
        config.monitoring.log_level = level.clone();
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_defaults() {
        let config = ConfigHierarchy::new().build().unwrap();
        // Should have secure defaults
        assert_eq!(config.network.api.port, 8080);
    }

    #[test]
    fn test_env_var_override() {
        let config = ConfigHierarchy::new()
            .with_cli_arg("port".to_string(), "9000".to_string())
            .build()
            .unwrap();

        // CLI args should override defaults
        assert_eq!(config.network.api.port, 9000);
    }

    #[test]
    fn test_config_source_priority() {
        let fallback = ConfigSource::FallbackDefaults;
        let cli = ConfigSource::CliArgs;

        assert!(cli > fallback);
    }

    #[test]
    fn test_config_value_merge() {
        let fallback = ConfigValue::new(8080, ConfigSource::FallbackDefaults);
        let cli = ConfigValue::new(9000, ConfigSource::CliArgs);

        let result = fallback.merge(cli.clone());
        assert_eq!(result.value, 9000);
        assert_eq!(result.source, ConfigSource::CliArgs);
    }

    #[test]
    fn test_config_value_merge_lower_priority_keeps_self() {
        let cli = ConfigValue::new(9000, ConfigSource::CliArgs);
        let fallback = ConfigValue::new(8080, ConfigSource::FallbackDefaults);

        let result = cli.merge(fallback);
        assert_eq!(result.value, 9000);
        assert_eq!(result.source, ConfigSource::CliArgs);
    }

    #[test]
    fn test_config_value_merge_equal_priority_takes_other() {
        let a = ConfigValue::new(1000, ConfigSource::Environment);
        let b = ConfigValue::new(2000, ConfigSource::Environment);

        let result = a.merge(b);
        assert_eq!(result.value, 2000);
    }

    #[test]
    fn test_hierarchy_default() {
        let h = ConfigHierarchy::default();
        let config = h.build().unwrap();
        assert!(config.network.api.port > 0);
    }

    #[test]
    fn test_with_platform_defaults() {
        let config = ConfigHierarchy::new()
            .with_platform_defaults()
            .build()
            .unwrap();
        assert!(config.network.api.port > 0);
    }

    #[test]
    fn test_with_cli_args_multiple() {
        let mut args = HashMap::new();
        args.insert("port".to_string(), "7777".to_string());
        args.insert("log-level".to_string(), "debug".to_string());

        let config = ConfigHierarchy::new().with_cli_args(args).build().unwrap();

        assert_eq!(config.network.api.port, 7777);
        assert_eq!(config.monitoring.log_level, "debug");
    }

    #[test]
    fn test_with_cli_arg_bind_address() {
        let config = ConfigHierarchy::new()
            .with_cli_arg("bind-address".to_string(), "0.0.0.0".to_string())
            .build()
            .unwrap();

        assert_eq!(
            config.network.api.bind_address,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))
        );
    }

    #[test]
    fn test_with_cli_arg_api_port_alias() {
        let config = ConfigHierarchy::new()
            .with_cli_arg("api-port".to_string(), "5555".to_string())
            .build()
            .unwrap();
        assert_eq!(config.network.api.port, 5555);
    }

    #[test]
    fn test_with_cli_arg_config_path() {
        let config = ConfigHierarchy::new()
            .with_cli_arg("config".to_string(), "/tmp/beardog/config.toml".to_string())
            .build()
            .unwrap();
        assert_eq!(
            config.paths.config_dir,
            std::path::PathBuf::from("/tmp/beardog")
        );
    }

    #[test]
    fn test_invalid_cli_port() {
        let result = ConfigHierarchy::new()
            .with_cli_arg("port".to_string(), "not_a_number".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_cli_bind_address() {
        let result = ConfigHierarchy::new()
            .with_cli_arg("bind-address".to_string(), "not_an_ip".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_with_env_vars() {
        // Just test that it doesn't panic (env vars are unpredictable)
        let config = ConfigHierarchy::new().with_env_vars().build().unwrap();
        assert!(config.network.api.port > 0);
    }

    #[test]
    fn test_with_auto_config_file_no_files() {
        // In test env, no config files exist, so it should fall through
        let config = ConfigHierarchy::new()
            .with_auto_config_file()
            .unwrap()
            .build()
            .unwrap();
        assert!(config.network.api.port > 0);
    }

    #[test]
    fn test_with_file_unsupported_format() {
        let result = ConfigHierarchy::new().with_file("/tmp/beardog_test.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_file_nonexistent() {
        let result = ConfigHierarchy::new().with_file("/nonexistent/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_env_overrides_ports() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_API_PORT".to_string(), "3000".to_string());
        env.insert("BEARDOG_DISCOVERY_PORT".to_string(), "3001".to_string());
        env.insert("BEARDOG_ADMIN_PORT".to_string(), "3002".to_string());

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert_eq!(config.network.api.port, 3000);
        assert_eq!(config.network.discovery.port, 3001);
        assert_eq!(config.network.admin.port, 3002);
    }

    #[test]
    fn test_apply_env_overrides_paths() {
        let mut env = HashMap::new();
        env.insert(
            "BEARDOG_CONFIG_DIR".to_string(),
            "/custom/config".to_string(),
        );
        env.insert("BEARDOG_DATA_DIR".to_string(), "/custom/data".to_string());
        env.insert("BEARDOG_LOG_DIR".to_string(), "/custom/logs".to_string());

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert_eq!(config.paths.config_dir, PathBuf::from("/custom/config"));
        assert_eq!(config.paths.data_dir, PathBuf::from("/custom/data"));
        assert_eq!(config.paths.log_dir, PathBuf::from("/custom/logs"));
    }

    #[test]
    fn test_apply_env_overrides_api_bind_address() {
        let mut env = HashMap::new();
        env.insert(
            "BEARDOG_API_BIND_ADDRESS".to_string(),
            "192.168.1.1".to_string(),
        );

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert_eq!(
            config.network.api.bind_address,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1))
        );
    }

    #[test]
    fn test_apply_env_overrides_invalid_port() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_API_PORT".to_string(), "invalid".to_string());

        let result = apply_env_overrides(BearDogConfig::default(), &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_env_overrides_hsm_timeout() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_HSM_TIMEOUT".to_string(), "120".to_string());

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert_eq!(config.timeouts.hsm_operation_secs, 120);
    }

    #[test]
    fn test_apply_env_overrides_strict_mode() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_STRICT_MODE".to_string(), "true".to_string());

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert!(config.security.strict_mode);
    }

    #[test]
    fn test_apply_env_overrides_log_level() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_LOG_LEVEL".to_string(), "trace".to_string());

        let config = apply_env_overrides(BearDogConfig::default(), &env).unwrap();
        assert_eq!(config.monitoring.log_level, "trace");
    }

    #[test]
    fn test_merge_configs_with_different_values() {
        let base = BearDogConfig::default();
        let mut override_cfg = BearDogConfig::default();
        override_cfg.network.api.port = 9999;

        let merged = merge_configs(base, override_cfg, ConfigSource::ConfigFile);
        assert_eq!(merged.network.api.port, 9999);
    }

    #[test]
    fn test_merge_configs_identical() {
        let base = BearDogConfig::default();
        let override_cfg = BearDogConfig::default();

        let merged = merge_configs(base.clone(), override_cfg, ConfigSource::ConfigFile);
        assert_eq!(merged.network.api.port, base.network.api.port);
    }

    #[test]
    fn test_config_source_ordering() {
        assert!(ConfigSource::CliArgs > ConfigSource::Environment);
        assert!(ConfigSource::Environment > ConfigSource::ConfigFile);
        assert!(ConfigSource::ConfigFile > ConfigSource::PlatformDefaults);
        assert!(ConfigSource::PlatformDefaults > ConfigSource::FallbackDefaults);
    }

    #[test]
    fn test_full_hierarchy_build() {
        let config = ConfigHierarchy::new()
            .with_platform_defaults()
            .with_env_vars()
            .with_cli_arg("port".to_string(), "6000".to_string())
            .build()
            .unwrap();

        // CLI arg should win
        assert_eq!(config.network.api.port, 6000);
    }
}
