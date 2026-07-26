// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use serde::{Deserialize, Serialize};
use serial_test::serial;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: u32,
    nested: NestedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NestedConfig {
    enabled: bool,
    items: Vec<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            value: 42,
            nested: NestedConfig {
                enabled: true,
                items: vec!["item1".to_string(), "item2".to_string()],
            },
        }
    }
}

#[test]
fn test_config_file_operations_toml() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::default();
    let temp_file = NamedTempFile::new()?;

    UnifiedConfigUtils::save_to_file(&config, temp_file.path())?;

    let loaded_config: TestConfig = UnifiedConfigUtils::load_from_file(temp_file.path())?;
    assert_eq!(config, loaded_config);
    assert!(UnifiedConfigUtils::validate_config_file(temp_file.path()));
    Ok(())
}

#[test]
fn test_save_and_load_json_extension() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("cfg.json");
    let config = TestConfig::default();
    UnifiedConfigUtils::save_to_file(&config, &path)?;
    let loaded: TestConfig = UnifiedConfigUtils::load_from_file(&path)?;
    assert_eq!(config, loaded);
    Ok(())
}

#[test]
fn test_yaml_extension_deprecated() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("cfg.yaml");
    let config = TestConfig::default();
    let save_err = UnifiedConfigUtils::save_to_file(&config, &path).unwrap_err();
    assert!(
        save_err.to_string().contains("YAML format deprecated"),
        "unexpected: {save_err}"
    );
    std::fs::write(&path, "key: value").expect("write");
    let load_err = UnifiedConfigUtils::load_from_file::<TestConfig, _>(&path).unwrap_err();
    assert!(
        load_err.to_string().contains("YAML format deprecated"),
        "unexpected: {load_err}"
    );
}

#[test]
fn test_save_unknown_extension_uses_toml() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("cfg.unknown");
    let config = TestConfig::default();
    UnifiedConfigUtils::save_to_file(&config, &path)?;
    let loaded: TestConfig = UnifiedConfigUtils::load_from_file(&path)?;
    assert_eq!(config, loaded);
    Ok(())
}

#[test]
fn test_load_from_file_missing() {
    let err = UnifiedConfigUtils::load_from_file::<TestConfig, _>("/nonexistent/beardog.cfg.toml")
        .unwrap_err();
    assert!(err.to_string().contains("not found") || err.to_string().contains("Config file"));
}

#[test]
fn test_load_from_file_invalid_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = NamedTempFile::new()?;
    writeln!(f, "not valid {{{{ toml")?;
    let err = UnifiedConfigUtils::load_from_file::<TestConfig, _>(f.path()).unwrap_err();
    assert!(err.to_string().contains("TOML") || err.to_string().contains("Invalid"));
    Ok(())
}

#[test]
fn test_load_with_fallback_primary_wins() -> Result<(), Box<dyn std::error::Error>> {
    let primary = NamedTempFile::new()?;
    let fallback = NamedTempFile::new()?;
    let cfg = TestConfig::default();
    UnifiedConfigUtils::save_to_file(&cfg, primary.path())?;
    let loaded: TestConfig = UnifiedConfigUtils::load_with_fallback(
        primary.path().to_str().unwrap(),
        &[fallback.path().to_str().unwrap()],
    )?;
    assert_eq!(loaded.name, cfg.name);
    Ok(())
}

#[test]
fn test_load_with_fallback_uses_second() -> Result<(), Box<dyn std::error::Error>> {
    let primary = tempfile::tempdir()?.path().join("missing.toml");
    let fallback = NamedTempFile::new()?;
    let cfg = TestConfig::default();
    UnifiedConfigUtils::save_to_file(&cfg, fallback.path())?;
    let loaded: TestConfig = UnifiedConfigUtils::load_with_fallback(
        primary.to_str().unwrap(),
        &[fallback.path().to_str().unwrap()],
    )?;
    assert_eq!(loaded, cfg);
    Ok(())
}

#[test]
fn test_load_with_fallback_all_missing() {
    let err =
        UnifiedConfigUtils::load_with_fallback::<TestConfig>("/nope/nope.toml", &["/nope2.toml"])
            .unwrap_err();
    assert!(err.to_string().contains("No valid configuration"));
}

#[test]
fn test_create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let path = NamedTempFile::new()?.into_temp_path();
    let cfg = TestConfig::default();
    UnifiedConfigUtils::create_default_config(cfg.clone(), &path)?;
    let loaded: TestConfig = UnifiedConfigUtils::load_from_file(&path)?;
    assert_eq!(loaded, cfg);
    Ok(())
}

#[test]
fn test_merge_configs_non_object_override_replaces() -> Result<(), Box<dyn std::error::Error>> {
    let merged = UnifiedConfigUtils::merge_configs(
        serde_json::json!({ "a": { "b": 1 } }),
        serde_json::json!(42),
    )?;
    assert_eq!(merged, serde_json::json!(42));
    Ok(())
}

#[test]
fn test_merge_configs_merged_json_rejects_target_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NeedNumber {
        x: u32,
    }
    let merged = UnifiedConfigUtils::merge_configs(
        serde_json::json!({ "x": 1 }),
        serde_json::json!({ "x": "not-a-number" }),
    )
    .expect("merge as Value");
    assert!(serde_json::from_value::<NeedNumber>(merged).is_err());
}

#[test]
#[serial]
fn test_clear_shared_configs_and_stats() {
    let key = format!("stats_key_{}", uuid::Uuid::new_v4());
    UnifiedConfigUtils::clear_shared_configs();
    let _ = UnifiedConfigUtils::get_shared_config(&key, || 1u32);
    let stats = UnifiedConfigUtils::get_shared_config_stats();
    assert!(stats.active_configs >= 1);
    UnifiedConfigUtils::clear_shared_configs();
    let stats_after = UnifiedConfigUtils::get_shared_config_stats();
    assert_eq!(stats_after.active_configs, 0);
}

#[test]
fn test_shared_config_overwrites_on_type_mismatch() {
    let key = format!("mixed_{}", uuid::Uuid::new_v4());
    let _a = UnifiedConfigUtils::get_shared_config(&key, || 42i32);
    let _b = UnifiedConfigUtils::get_shared_config(&key, || 99u64);
    let v = UnifiedConfigUtils::get_shared_config(&key, || 0u64);
    assert_eq!(*v, 99u64);
}

#[test]
fn test_standard_paths_beardog_includes_extra() {
    let paths = UnifiedConfigUtils::get_standard_config_paths("beardog");
    let joined: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    assert!(joined.iter().any(|p| p.contains("beardog.toml")));
    assert!(joined.iter().any(|p| p.contains("beardog-config.toml")));
}

#[test]
fn test_find_and_auto_load_with_temp_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("findapp.toml");
    let cfg = TestConfig::default();
    UnifiedConfigUtils::save_to_file(&cfg, &path)?;
    assert!(UnifiedConfigUtils::validate_config_file(&path));
    // find_config_file only checks relative standard paths — exercise auto_load error path
    let err =
        UnifiedConfigUtils::auto_load_config::<TestConfig>("unlikely_app_xyz_12345").unwrap_err();
    assert!(err.to_string().contains("No configuration file found"));
    Ok(())
}

#[test]
fn test_load_with_env_overrides() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = NamedTempFile::new()?;
    writeln!(
        f,
        r#"
name = "base"
value = 1

[nested]
enabled = false
items = ["x"]
"#
    )?;
    let prefix = "BEARDOG_TYPES_CFG_UT";
    let cfg: TestConfig = UnifiedConfigUtils::load_with_env_overrides_from_vars(
        f.path().to_str().unwrap(),
        prefix,
        [(format!("{prefix}_NAME"), "from_env".to_string())],
    )?;
    assert_eq!(cfg.name, "from_env");
    assert_eq!(cfg.value, 1);
    Ok(())
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_config_merging() -> Result<(), Box<dyn std::error::Error>> {
    let base = TestConfig {
        name: "base".to_string(),
        value: 10,
        nested: NestedConfig {
            enabled: false,
            items: vec!["base_item".to_string()],
        },
    };

    let override_config = TestConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        name: "override".to_string(),
        value: 20,
        nested: NestedConfig {
            enabled: true,
            items: vec!["override_item".to_string()],
        },
    };

    let merged = UnifiedConfigUtils::merge_configs(base, override_config)?;

    // Override should take precedence
    assert_eq!(merged.name, "override");
    assert_eq!(merged.value, 20);
    assert!(merged.nested.enabled);
    Ok(())
}

#[test]
fn test_shared_config_management() {
    let config1 = UnifiedConfigUtils::get_shared_config("test_config", TestConfig::default);
    let config2 = UnifiedConfigUtils::get_shared_config("test_config", TestConfig::default);

    // Should be the same instance
    assert!(std::sync::Arc::ptr_eq(&config1, &config2));

    // Test removal
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(UnifiedConfigUtils::remove_shared_config("test_config"));
    assert!(!UnifiedConfigUtils::remove_shared_config("nonexistent"));
}

#[test]
fn test_standard_config_paths() {
    let paths = UnifiedConfigUtils::get_standard_config_paths("myapp");
    assert!(!paths.is_empty());

    // Should include current directory
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(
        paths
            .iter()
            .any(|p| p.to_string_lossy().contains("./myapp.toml"))
    );

    // Should include config directory
    assert!(
        paths
            .iter()
            .any(|p| p.to_string_lossy().contains("./config/myapp.toml"))
    );
}

#[test]
fn test_performance_metrics() {
    let metrics = UnifiedConfigUtils::get_performance_metrics();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(metrics.consolidation_benefit > 0.0);
    assert!(metrics.memory_reduction_mb > 0.0);
    assert!(metrics.config_operations_per_second > 0.0);
}

// test_legacy_compatibility removed with legacy config module (see git history).

#[test]
fn test_arc_str_serialization() {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(
            serialize_with = "serialize_arc_str",
            deserialize_with = "deserialize_arc_str"
        )]
        value: Arc<str>,
    }

    let original = TestStruct {
        value: Arc::from("test value"),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("Failed to serialize");
    assert_eq!(json, r#"{"value":"test value"}"#);

    // Deserialize back
    let deserialized: TestStruct = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(original.value, deserialized.value);

    // Verify Arc property: cloning is cheap
    let cloned = deserialized.value.clone();
    assert!(Arc::ptr_eq(&deserialized.value, &cloned));
}
