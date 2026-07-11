// SPDX-License-Identifier: AGPL-3.0-or-later

//! Environment variable and CLI argument override application.

use crate::{BearDogConfig, ConfigError, ConfigResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Apply environment variable overrides
pub fn apply_env_overrides(
    mut config: BearDogConfig,
    env_vars: &HashMap<String, String>,
) -> ConfigResult<BearDogConfig> {
    // Network
    if let Some(port) = env_vars.get("BEARDOG_API_PORT") {
        config.network.api.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_API_PORT", format!("Invalid port: {port}"))
        })?;
    }
    if let Some(addr) = env_vars.get("BEARDOG_API_BIND_ADDRESS") {
        config.network.api.bind_address = addr.parse().map_err(|_| {
            ConfigError::invalid_value(
                "BEARDOG_API_BIND_ADDRESS",
                format!("Invalid address: {addr}"),
            )
        })?;
    }
    if let Some(port) = env_vars.get("BEARDOG_DISCOVERY_PORT") {
        config.network.discovery.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_DISCOVERY_PORT", format!("Invalid port: {port}"))
        })?;
    }
    if let Some(port) = env_vars.get("BEARDOG_ADMIN_PORT") {
        config.network.admin.port = port.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_ADMIN_PORT", format!("Invalid port: {port}"))
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
            ConfigError::invalid_value("BEARDOG_HSM_TIMEOUT", format!("Invalid timeout: {timeout}"))
        })?;
    }

    // Security
    if let Some(strict) = env_vars.get("BEARDOG_STRICT_MODE") {
        config.security.strict_mode = strict.parse().map_err(|_| {
            ConfigError::invalid_value("BEARDOG_STRICT_MODE", format!("Invalid boolean: {strict}"))
        })?;
    }

    // Monitoring
    if let Some(level) = env_vars.get("BEARDOG_LOG_LEVEL") {
        config.monitoring.log_level.clone_from(level);
    }

    Ok(config)
}

/// Apply CLI argument overrides (highest priority layer).
pub fn apply_cli_overrides(
    mut config: BearDogConfig,
    cli_args: &HashMap<String, String>,
) -> ConfigResult<BearDogConfig> {
    if let Some(port) = cli_args.get("port").or_else(|| cli_args.get("api-port")) {
        config.network.api.port = port
            .parse()
            .map_err(|_| ConfigError::invalid_value("port", format!("Invalid port: {port}")))?;
    }
    if let Some(addr) = cli_args.get("bind-address") {
        config.network.api.bind_address = addr.parse().map_err(|_| {
            ConfigError::invalid_value("bind-address", format!("Invalid address: {addr}"))
        })?;
    }
    if let Some(config_file) = cli_args.get("config") {
        config.paths.config_dir = PathBuf::from(config_file)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
    }
    if let Some(level) = cli_args.get("log-level") {
        config.monitoring.log_level.clone_from(level);
    }

    Ok(config)
}
