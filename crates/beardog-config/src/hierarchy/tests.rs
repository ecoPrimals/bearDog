// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::BearDogConfig;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_hierarchy_defaults() {
    let config = ConfigHierarchy::new()
        .build()
        .expect("ConfigHierarchy::build should succeed in test");
    assert_eq!(config.network.api.port, 8080);
}

#[test]
fn test_env_var_override() {
    let config = ConfigHierarchy::new()
        .with_cli_arg("port".to_string(), "9000".to_string())
        .build()
        .expect("ConfigHierarchy::build with CLI port override");

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

    let result = fallback.merge(cli);
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
    let config = h
        .build()
        .expect("ConfigHierarchy::build should succeed in test");
    assert!(config.network.api.port > 0);
}

#[test]
fn test_with_platform_defaults() {
    let config = ConfigHierarchy::new()
        .with_platform_defaults()
        .build()
        .expect("ConfigHierarchy::build with platform defaults");
    assert!(config.network.api.port > 0);
}

#[test]
fn test_with_cli_args_multiple() {
    let mut args = HashMap::new();
    args.insert("port".to_string(), "7777".to_string());
    args.insert("log-level".to_string(), "debug".to_string());

    let config = ConfigHierarchy::new()
        .with_cli_args(args)
        .build()
        .expect("ConfigHierarchy::build should succeed in test");

    assert_eq!(config.network.api.port, 7777);
    assert_eq!(config.monitoring.log_level, "debug");
}

#[test]
fn test_with_cli_arg_bind_address() {
    let config = ConfigHierarchy::new()
        .with_cli_arg("bind-address".to_string(), "0.0.0.0".to_string())
        .build()
        .expect("ConfigHierarchy::build with bind-address CLI override");

    assert_eq!(
        config.network.api.bind_address,
        std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)
    );
}

#[test]
fn test_with_cli_arg_api_port_alias() {
    let config = ConfigHierarchy::new()
        .with_cli_arg("api-port".to_string(), "5555".to_string())
        .build()
        .expect("ConfigHierarchy::build with api-port CLI alias");
    assert_eq!(config.network.api.port, 5555);
}

#[test]
fn test_with_cli_arg_config_path() {
    let config = ConfigHierarchy::new()
        .with_cli_arg("config".to_string(), "/tmp/beardog/config.toml".to_string())
        .build()
        .expect("ConfigHierarchy::build with config path CLI override");
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
    let config = ConfigHierarchy::new()
        .with_env_vars()
        .build()
        .expect("ConfigHierarchy::build should succeed in test");
    assert!(config.network.api.port > 0);
}

#[test]
fn test_with_auto_config_file_no_files() {
    let config = ConfigHierarchy::new()
        .with_auto_config_file()
        .expect("with_auto_config_file should succeed when no files exist")
        .build()
        .expect("ConfigHierarchy::build after auto config discovery");
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

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides ports");
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

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides paths");
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

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides bind address");
    assert_eq!(
        config.network.api.bind_address,
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1))
    );
}

#[test]
fn test_apply_env_overrides_invalid_port() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_API_PORT".to_string(), "invalid".to_string());

    let result = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(result.is_err());
}

#[test]
fn test_apply_env_overrides_hsm_timeout() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_HSM_TIMEOUT".to_string(), "120".to_string());

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides HSM timeout");
    assert_eq!(config.timeouts.hsm_operation_secs, 120);
}

#[test]
fn test_apply_env_overrides_strict_mode() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_STRICT_MODE".to_string(), "true".to_string());

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides strict mode");
    assert!(config.security.strict_mode);
}

#[test]
fn test_apply_env_overrides_log_level() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_LOG_LEVEL".to_string(), "trace".to_string());

    let config = overrides::apply_env_overrides(BearDogConfig::default(), &env)
        .expect("apply_env_overrides log level");
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
        .expect("ConfigHierarchy::build full stack with CLI port");

    assert_eq!(config.network.api.port, 6000);
}

#[test]
fn test_with_file_toml_loads() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir.path().join("cfg.toml");
    let cfg = BearDogConfig::default();
    let toml = toml::to_string(&cfg).expect("serialize");
    std::fs::write(&path, toml).expect("write");
    let built = ConfigHierarchy::new().with_file(&path).expect("with_file");
    let out = built.build().expect("build");
    assert_eq!(out.network.api.port, cfg.network.api.port);
}

#[test]
fn test_with_file_json_loads() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir.path().join("cfg.json");
    let cfg = BearDogConfig::default();
    let j = serde_json::to_string_pretty(&cfg).expect("json");
    std::fs::write(&path, j).expect("write");
    let built = ConfigHierarchy::new().with_file(&path).expect("with_file");
    let out = built.build().expect("build");
    assert_eq!(out.security.strict_mode, cfg.security.strict_mode);
}

#[test]
fn test_merge_configs_paths_always_take_override() {
    let base = BearDogConfig::default();
    let mut o = BearDogConfig::default();
    o.paths.config_dir = PathBuf::from("/overridden/config");
    let merged = merge_configs(base, o, ConfigSource::ConfigFile);
    assert_eq!(merged.paths.config_dir, PathBuf::from("/overridden/config"));
}

#[test]
fn test_merge_configs_hsm_always_take_override() {
    let base = BearDogConfig::default();
    let mut o = BearDogConfig::default();
    o.hsm.enable_yubihsm = true;
    let merged = merge_configs(base, o, ConfigSource::ConfigFile);
    assert!(merged.hsm.enable_yubihsm);
}

#[test]
fn test_apply_env_overrides_invalid_bind_address() {
    let mut env = HashMap::new();
    env.insert(
        "BEARDOG_API_BIND_ADDRESS".to_string(),
        "not-an-ip-address".to_string(),
    );
    let r = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(r.is_err());
}

#[test]
fn test_apply_env_overrides_invalid_discovery_port() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_DISCOVERY_PORT".to_string(), "xyz".to_string());
    let r = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(r.is_err());
}

#[test]
fn test_apply_env_overrides_invalid_admin_port() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_ADMIN_PORT".to_string(), "bad".to_string());
    let r = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(r.is_err());
}

#[test]
fn test_apply_env_overrides_invalid_hsm_timeout() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_HSM_TIMEOUT".to_string(), "nan".to_string());
    let r = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(r.is_err());
}

#[test]
fn test_apply_env_overrides_invalid_strict_mode() {
    let mut env = HashMap::new();
    env.insert("BEARDOG_STRICT_MODE".to_string(), "maybe".to_string());
    let r = overrides::apply_env_overrides(BearDogConfig::default(), &env);
    assert!(r.is_err());
}
