// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::config_management::{
    ApplicationConfig, AutoScalingConfig, ComplianceConfig, DataRetentionConfig, DatabaseConnection,
    SecretValue, SecretsProvider, TlsConfig,
};
use crate::config_management::secrets_backend::{FileVaultBackend, SecretsBackend};
use beardog_errors::process_env;
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Restores the process current directory when dropped.
struct CurrentDirGuard {
    previous: std::path::PathBuf,
}

impl CurrentDirGuard {
    fn new(root: &Path) -> std::io::Result<Self> {
        let previous = std::env::current_dir()?;
        std::env::set_current_dir(root)?;
        Ok(Self { previous })
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.previous);
    }
}

/// `DatabaseConnection::password` is `skip_serializing`; round-trip file tests must restore it.
fn patch_toml_with_db_password(serialized: String) -> String {
    serialized.replacen(
        "[database.primary]\n",
        "[database.primary]\npassword = \"\"\n",
        1,
    )
}

fn patch_yaml_with_db_password(serialized: &str) -> String {
    let mut v: serde_yaml::Value = serde_yaml::from_str(serialized).expect("yaml parse");
    if let Some(db) = v.get_mut("database")
        && let Some(primary) = db.get_mut("primary")
        && let serde_yaml::Value::Mapping(m) = primary
    {
        m.insert(
            serde_yaml::Value::String("password".to_string()),
            serde_yaml::Value::String(String::new()),
        );
    }
    serde_yaml::to_string(&v).expect("yaml emit")
}

fn patch_json_with_db_password(cfg: &ProductionConfig) -> String {
    let mut v: serde_json::Value = serde_json::to_value(cfg).expect("json value");
    if let Some(db) = v.get_mut("database")
        && let Some(prim) = db.get_mut("primary")
        && let Some(obj) = prim.as_object_mut()
    {
        obj.insert("password".to_string(), serde_json::json!(""));
    }
    serde_json::to_string(&v).expect("json string")
}

#[test]
fn secrets_manager_new_development_succeeds() {
    let mgr = SecretsManager::new(&Environment::Development);
    assert!(mgr.is_ok());
}

#[test]
#[serial]
fn secrets_manager_get_secret_from_environment_provider() {
    process_env::set_var("BEARDOG_SECRET_DATABASE_PASSWORD", "from-env");
    let mut mgr = SecretsManager::new(&Environment::Development).expect("manager");
    let v = mgr.get_secret("database/password").expect("secret");
    assert_eq!(v.value, "from-env");
    process_env::remove_var("BEARDOG_SECRET_DATABASE_PASSWORD");
}

#[test]
#[serial]
fn secrets_manager_get_secret_not_found_returns_error() {
    process_env::remove_var("BEARDOG_SECRET_DATABASE_PASSWORD");
    let mut mgr = SecretsManager::new(&Environment::Development).expect("manager");
    let err = mgr.get_secret("database/password").unwrap_err();
    let _ = format!("{err:?}");
}

#[test]
#[serial]
fn secrets_manager_get_secret_tls_key_env_mapping() {
    process_env::set_var("BEARDOG_SECRET_TLS_PRIVATE_KEY", "/keys/k.pem");
    let mut mgr = SecretsManager::new(&Environment::Development).expect("manager");
    let v = mgr.get_secret("tls/private_key").expect("key");
    assert_eq!(v.value, "/keys/k.pem");
    process_env::remove_var("BEARDOG_SECRET_TLS_PRIVATE_KEY");
}

#[test]
#[serial]
fn secrets_manager_production_with_vault_retrieves_stored_secret() {
    let tmp = TempDir::new().expect("tempdir");
    let vault_root = tmp.path().join("vroot");
    fs::create_dir_all(&vault_root).expect("mkdir");
    let master_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let endpoint = vault_root.to_str().expect("utf8");
    process_env::set_var("BEARDOG_VAULT_MASTER_KEY", master_hex);
    process_env::set_var("VAULT_ENDPOINT", endpoint);
    process_env::set_var("VAULT_TOKEN", "unit-test-token");
    process_env::set_var("VAULT_MOUNT_PATH", "secret");

    let backend =
        FileVaultBackend::open_for_vault_provider(endpoint, "unit-test-token", "secret")
            .expect("vault");
    backend
        .store("database/password", "vault-db-secret")
        .expect("store");

    let mut mgr = SecretsManager::new(&Environment::Production).expect("manager");
    let v = mgr.get_secret("database/password").expect("from vault");
    assert_eq!(v.value, "vault-db-secret");

    process_env::remove_var("BEARDOG_VAULT_MASTER_KEY");
    process_env::remove_var("VAULT_ENDPOINT");
    process_env::remove_var("VAULT_TOKEN");
    process_env::remove_var("VAULT_MOUNT_PATH");
}

#[test]
fn determine_config_sources_development_includes_env_and_file() {
    let sources = ProductionConfigManager::determine_config_sources(&Environment::Development)
        .expect("sources");
    assert!(
        sources
            .iter()
            .any(|s| matches!(s, ConfigSource::Environment))
    );
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::File { path } if path.ends_with("development.toml")
    )));
}

#[test]
fn determine_config_sources_staging_uses_staging_file() {
    let sources = ProductionConfigManager::determine_config_sources(&Environment::Staging)
        .expect("sources");
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::File { path } if path.contains("staging.toml")
    )));
}

#[test]
#[serial]
fn determine_config_sources_production_adds_vault_and_k8s_when_env_set() {
    process_env::set_var("VAULT_ENDPOINT", "https://vault.example");
    process_env::set_var("VAULT_TOKEN", "tok");
    process_env::set_var("KUBERNETES_SERVICE_HOST", "10.0.0.1");
    process_env::set_var("KUBERNETES_NAMESPACE", "bear-ns");

    let sources = ProductionConfigManager::determine_config_sources(&Environment::Production)
        .expect("sources");
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::UniversalSecretsManagement { provider_type, .. }
            if provider_type == "BearDogLocalVault"
    )));
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::UniversalContainerSecrets { namespace, provider_type, .. }
            if namespace == "bear-ns" && provider_type == "kubernetes"
    )));

    process_env::remove_var("VAULT_ENDPOINT");
    process_env::remove_var("VAULT_TOKEN");
    process_env::remove_var("KUBERNETES_SERVICE_HOST");
    process_env::remove_var("KUBERNETES_NAMESPACE");
}

#[test]
fn has_secrets_capability_detects_vault_kms_and_secrets() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    assert!(mgr.has_secrets_capability("MyVaultProvider"));
    assert!(mgr.has_secrets_capability("AWS_SECRETS"));
    assert!(mgr.has_secrets_capability("cloud-kms"));
    assert!(mgr.has_secrets_capability("master-key-ring"));
    assert!(!mgr.has_secrets_capability("plain"));
}

#[test]
fn load_from_universal_secrets_capability_vs_fallback() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.load_from_universal_secrets(
        &mut cfg,
        "https://u",
        "BearDogSecretsVault",
        &HashMap::new(),
    )
    .expect("ok");
    mgr.load_from_universal_secrets(&mut cfg, "https://u", "noop", &HashMap::new())
        .expect("ok");
}

#[test]
fn load_from_universal_container_branches() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    for provider in ["kubernetes", "docker_swarm", "nomad", "unknown"] {
        mgr.load_from_universal_container_secrets(&mut cfg, "ns", provider)
            .expect("ok");
    }
}

#[test]
fn validate_configuration_rejects_invalid_application_and_database() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    cfg.application.port = 0;
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.application.port = 8080;
    cfg.application.worker_threads = 0;
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.application.worker_threads = 4;
    cfg.database.primary.host = String::new();
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.database.primary.host = "localhost".to_string();
    cfg.database.primary.database = String::new();
    assert!(mgr.validate_configuration(&cfg).is_err());
}

#[test]
fn validate_configuration_production_requires_tls_and_compliance() {
    let mgr = ProductionConfigManager::new(Environment::Production).expect("mgr");
    let mut cfg = ProductionConfig::default();
    cfg.networking.tls.enabled = false;
    cfg.compliance.encryption_at_rest = true;
    cfg.compliance.access_logging = true;
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.networking.tls.enabled = true;
    cfg.compliance.encryption_at_rest = false;
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.compliance.encryption_at_rest = true;
    cfg.compliance.access_logging = false;
    assert!(mgr.validate_configuration(&cfg).is_err());

    cfg.compliance.access_logging = true;
    assert!(mgr.validate_configuration(&cfg).is_ok());
}

#[test]
#[serial]
fn load_from_environment_parses_log_levels_and_invalid_numbers() {
    process_env::set_var("BEARDOG_PORT", "9090");
    process_env::set_var("BEARDOG_BIND_ADDRESS", "0.0.0.0");
    process_env::set_var("BEARDOG_WORKER_THREADS", "8");
    process_env::set_var("BEARDOG_DB_HOST", "db.example");
    process_env::set_var("BEARDOG_DB_PORT", "5433");
    process_env::set_var("BEARDOG_LOG_LEVEL", "WaRn");

    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.load_from_environment(&mut cfg).expect("env");

    assert_eq!(cfg.application.port, 9090);
    assert_eq!(cfg.application.bind_address, "0.0.0.0");
    assert_eq!(cfg.application.worker_threads, 8);
    assert_eq!(cfg.database.primary.host, "db.example");
    assert_eq!(cfg.database.primary.port, 5433);
    assert!(matches!(cfg.logging.level, LogLevel::Warn));

    process_env::set_var("BEARDOG_LOG_LEVEL", "trace");
    mgr.load_from_environment(&mut cfg).expect("trace");
    assert!(matches!(cfg.logging.level, LogLevel::Trace));

    process_env::set_var("BEARDOG_LOG_LEVEL", "bogus");
    mgr.load_from_environment(&mut cfg).expect("info default");
    assert!(matches!(cfg.logging.level, LogLevel::Info));

    process_env::set_var("BEARDOG_PORT", "not-valid");
    let mut cfg_bad = ProductionConfig::default();
    assert!(mgr.load_from_environment(&mut cfg_bad).is_err());
    process_env::remove_var("BEARDOG_PORT");
    process_env::remove_var("BEARDOG_BIND_ADDRESS");
    process_env::remove_var("BEARDOG_WORKER_THREADS");
    process_env::remove_var("BEARDOG_DB_HOST");
    process_env::remove_var("BEARDOG_DB_PORT");
    process_env::remove_var("BEARDOG_LOG_LEVEL");
}

#[test]
#[serial]
fn load_from_environment_invalid_port_errors() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    process_env::set_var("BEARDOG_PORT", "not-a-u16");
    let mut cfg = ProductionConfig::default();
    let r = mgr.load_from_environment(&mut cfg);
    process_env::remove_var("BEARDOG_PORT");
    assert!(r.is_err());
}

#[test]
#[serial]
fn load_from_environment_invalid_worker_threads_errors() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    process_env::set_var("BEARDOG_WORKER_THREADS", "xyz");
    let mut cfg = ProductionConfig::default();
    let r = mgr.load_from_environment(&mut cfg);
    process_env::remove_var("BEARDOG_WORKER_THREADS");
    assert!(r.is_err());
}

#[test]
#[serial]
fn load_from_environment_invalid_db_port_errors() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    process_env::set_var("BEARDOG_DB_PORT", "nan");
    let mut cfg = ProductionConfig::default();
    let r = mgr.load_from_environment(&mut cfg);
    process_env::remove_var("BEARDOG_DB_PORT");
    assert!(r.is_err());
}

#[test]
#[serial]
fn load_from_file_missing_path_is_noop() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.load_from_file(&mut cfg, "nope/not-here.toml")
        .expect("noop");
}

#[test]
#[serial]
fn load_from_file_toml_yaml_and_json() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");

    let mut base = ProductionConfig::default();
    base.application.port = 7777;
    let toml_str = patch_toml_with_db_password(toml::to_string(&base).expect("toml"));
    fs::write("cfg.toml", toml_str).expect("write");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.load_from_file(&mut cfg, "cfg.toml").expect("toml load");
    assert_eq!(cfg.application.port, 7777);

    let yaml_raw = serde_yaml::to_string(&base).expect("yaml");
    let yaml_str = patch_yaml_with_db_password(&yaml_raw);
    fs::write("cfg.yaml", yaml_str).expect("write");
    mgr.load_from_file(&mut cfg, "cfg.yaml").expect("yaml");
    assert_eq!(cfg.application.port, 7777);

    let json_str = patch_json_with_db_password(&base);
    fs::write("cfg.json", json_str).expect("write");
    mgr.load_from_file(&mut cfg, "cfg.json").expect("json");
    assert_eq!(cfg.application.port, 7777);

    let yml_str = patch_yaml_with_db_password(&serde_yaml::to_string(&base).expect("yml"));
    fs::write("cfg.yml", yml_str).expect("write");
    mgr.load_from_file(&mut cfg, "cfg.yml").expect("yml ext");
    assert_eq!(cfg.application.port, 7777);
}

#[test]
#[serial]
fn load_from_file_invalid_toml_errors() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::write("bad.toml", "not valid toml [[[").expect("write");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    assert!(mgr.load_from_file(&mut cfg, "bad.toml").is_err());
}

#[test]
#[serial]
fn load_from_file_read_error_surfaces() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::create_dir_all("is_dir.toml").expect("mkdir");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    assert!(mgr.load_from_file(&mut cfg, "is_dir.toml").is_err());
}

#[test]
#[serial]
fn load_configuration_development_end_to_end() {
    process_env::remove_var("BEARDOG_PORT");
    process_env::remove_var("BEARDOG_WORKER_THREADS");
    process_env::remove_var("BEARDOG_DB_PORT");
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::create_dir_all("configs/environments").expect("mkdir");
    let mut mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let cfg = mgr.load_configuration().expect("loaded");
    assert_ne!(cfg.application.port, 0);
    assert!(!cfg.database.primary.host.is_empty());
}

#[test]
#[serial]
fn load_configuration_production_fails_without_compliance_defaults() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::create_dir_all("configs/environments").expect("mkdir");
    let mut mgr = ProductionConfigManager::new(Environment::Production).expect("mgr");
    assert!(mgr.load_configuration().is_err());
}

#[test]
#[serial]
fn load_configuration_production_succeeds_with_valid_file() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::create_dir_all("configs/environments").expect("mkdir");
    let mut valid = ProductionConfig::default();
    valid.networking.tls.enabled = true;
    valid.compliance.encryption_at_rest = true;
    valid.compliance.access_logging = true;
    let body = patch_toml_with_db_password(toml::to_string(&valid).expect("toml"));
    fs::write("configs/environments/production.toml", body).expect("write");

    let mut mgr = ProductionConfigManager::new(Environment::Production).expect("mgr");
    let cfg = mgr.load_configuration().expect("production ok");
    assert!(cfg.networking.tls.enabled);
}

#[test]
fn default_configs_are_constructible() {
    let _ = ApplicationConfig::default();
    let _ = DatabaseConnection::default();
    let _ = TlsConfig::default();
    let _ = AutoScalingConfig::default();
    let _ = ComplianceConfig::default();
    let _ = DataRetentionConfig::default();
}

#[test]
#[serial]
fn inject_secrets_updates_database_and_tls_paths() {
    process_env::set_var("BEARDOG_SECRET_DATABASE_PASSWORD", "dbpw");
    process_env::set_var("BEARDOG_SECRET_TLS_CERTIFICATE", "/c/cert.pem");
    process_env::set_var("BEARDOG_SECRET_TLS_PRIVATE_KEY", "/c/key.pem");
    let mut mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.inject_secrets(&mut cfg).expect("inject");
    assert_eq!(cfg.database.primary.password, "dbpw");
    assert_eq!(cfg.networking.tls.cert_path, "/c/cert.pem");
    assert_eq!(cfg.networking.tls.key_path, "/c/key.pem");
    process_env::remove_var("BEARDOG_SECRET_DATABASE_PASSWORD");
    process_env::remove_var("BEARDOG_SECRET_TLS_CERTIFICATE");
    process_env::remove_var("BEARDOG_SECRET_TLS_PRIVATE_KEY");
}

#[test]
fn determine_config_sources_testing_matches_development_file() {
    let sources = ProductionConfigManager::determine_config_sources(&Environment::Testing)
        .expect("sources");
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::File { path } if path.ends_with("development.toml")
    )));
}

#[test]
fn determine_config_sources_custom_uses_development_file() {
    let sources = ProductionConfigManager::determine_config_sources(&Environment::Custom(
        "edge".to_string(),
    ))
    .expect("sources");
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::File { path } if path.ends_with("development.toml")
    )));
}

#[test]
#[serial]
fn determine_config_sources_production_k8s_default_namespace_when_unset() {
    process_env::remove_var("VAULT_ENDPOINT");
    process_env::remove_var("VAULT_TOKEN");
    process_env::set_var("KUBERNETES_SERVICE_HOST", "10.96.0.1");
    process_env::remove_var("KUBERNETES_NAMESPACE");

    let sources = ProductionConfigManager::determine_config_sources(&Environment::Production)
        .expect("sources");
    assert!(sources.iter().any(|s| matches!(
        s,
        ConfigSource::UniversalContainerSecrets { namespace, provider_type, .. }
            if namespace == "default" && provider_type == "kubernetes"
    )));

    process_env::remove_var("KUBERNETES_SERVICE_HOST");
}

#[test]
#[serial]
fn secrets_manager_get_secret_returns_from_cache_when_not_expired() {
    process_env::remove_var("BEARDOG_SECRET_DATABASE_PASSWORD");
    let mut mgr = SecretsManager::new(&Environment::Development).expect("manager");
    let future = std::time::SystemTime::now() + std::time::Duration::from_secs(3_600);
    mgr.insert_cache_entry_for_test(
        "database/password",
        SecretValue {
            value: "cached-only".to_string(),
            expires_at: Some(future),
            metadata: HashMap::new(),
        },
    );
    let v = mgr.get_secret("database/password").expect("from cache");
    assert_eq!(v.value, "cached-only");
}

#[test]
#[serial]
fn secrets_manager_get_secret_ignores_expired_cache_and_queries_providers() {
    process_env::set_var("BEARDOG_SECRET_DATABASE_PASSWORD", "from-env-after-expiry");
    let mut mgr = SecretsManager::new(&Environment::Development).expect("manager");
    mgr.insert_cache_entry_for_test(
        "database/password",
        SecretValue {
            value: "stale".to_string(),
            expires_at: Some(std::time::SystemTime::UNIX_EPOCH),
            metadata: HashMap::new(),
        },
    );
    let v = mgr.get_secret("database/password").expect("from env");
    assert_eq!(v.value, "from-env-after-expiry");
    process_env::remove_var("BEARDOG_SECRET_DATABASE_PASSWORD");
}

#[test]
#[serial]
fn secrets_manager_universal_secrets_management_provider_retrieves_from_file_vault() {
    let tmp = TempDir::new().expect("tempdir");
    let vault_root = tmp.path();
    let endpoint = vault_root.to_str().expect("utf8");
    let master_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    process_env::set_var("BEARDOG_VAULT_MASTER_KEY", master_hex);

    let backend = FileVaultBackend::open_for_usm(endpoint, &HashMap::new()).expect("usm open");
    backend
        .store("database/password", "usm-db-secret")
        .expect("store");

    let mut auth = HashMap::new();
    auth.insert("token".to_string(), "ignored".to_string());
    let mut mgr = SecretsManager::from_providers_for_test(vec![
        SecretsProvider::UniversalSecretsManagement {
            endpoint: endpoint.to_string(),
            provider_type: "file-usm".to_string(),
            auth_config: auth,
        },
    ]);

    let v = mgr.get_secret("database/password").expect("usm");
    assert_eq!(v.value, "usm-db-secret");

    process_env::remove_var("BEARDOG_VAULT_MASTER_KEY");
}

#[test]
#[serial]
fn load_from_environment_log_level_debug_and_error() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();

    process_env::set_var("BEARDOG_LOG_LEVEL", "debug");
    mgr.load_from_environment(&mut cfg).expect("debug");
    assert!(matches!(cfg.logging.level, LogLevel::Debug));

    process_env::set_var("BEARDOG_LOG_LEVEL", "error");
    mgr.load_from_environment(&mut cfg).expect("error");
    assert!(matches!(cfg.logging.level, LogLevel::Error));

    process_env::remove_var("BEARDOG_LOG_LEVEL");
}

#[test]
#[serial]
fn load_from_file_unknown_extension_parses_as_json() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");

    let mut base = ProductionConfig::default();
    base.application.port = 65000;
    let json_str = patch_json_with_db_password(&base);
    fs::write("cfg.data", json_str).expect("write");

    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.load_from_file(&mut cfg, "cfg.data")
        .expect("json fallback ext");
    assert_eq!(cfg.application.port, 65000);
}

#[test]
#[serial]
fn load_from_file_invalid_yaml_errors() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::write("bad.yaml", "{ not: yaml").expect("write");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    assert!(mgr.load_from_file(&mut cfg, "bad.yaml").is_err());
}

#[test]
#[serial]
fn load_from_file_invalid_json_errors() {
    let tmp = TempDir::new().expect("tempdir");
    let _guard = CurrentDirGuard::new(tmp.path()).expect("chdir");
    fs::write("bad.json", "{invalid").expect("write");
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    assert!(mgr.load_from_file(&mut cfg, "bad.json").is_err());
}

#[test]
fn apply_environment_overrides_is_noop_but_callable() {
    let mgr = ProductionConfigManager::new(Environment::Development).expect("mgr");
    let mut cfg = ProductionConfig::default();
    mgr.apply_environment_overrides(&mut cfg)
        .expect("noop overrides");
}
