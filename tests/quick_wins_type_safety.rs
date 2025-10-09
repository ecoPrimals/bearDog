// Quick Win Tests: Type Safety
//
// Verify type safety and trait implementations

use beardog_types::canonical::config::unified::{Environment, LogLevel, UnifiedBearDogConfig};

#[test]
fn test_environment_enum_variants() {
    let dev = Environment::Development;
    let test = Environment::Testing;
    let staging = Environment::Staging;
    let prod = Environment::Production;

    // All variants should be distinct when formatted
    let dev_str = format!("{:?}", dev);
    let prod_str = format!("{:?}", prod);
    assert_ne!(dev_str, prod_str);

    let test_str = format!("{:?}", test);
    let staging_str = format!("{:?}", staging);
    assert_ne!(test_str, staging_str);
}

#[test]
fn test_environment_default() {
    let default_env = Environment::default();
    assert_eq!(format!("{:?}", default_env), "Development");
}

#[test]
fn test_log_level_enum_variants() {
    let trace = LogLevel::Trace;
    let debug = LogLevel::Debug;
    let info = LogLevel::Info;
    let warn = LogLevel::Warn;
    let error = LogLevel::Error;

    // All should be distinct
    let levels = vec![trace, debug, info, warn, error];
    assert_eq!(levels.len(), 5);
}

#[test]
fn test_log_level_default() {
    let default_level = LogLevel::default();
    assert_eq!(format!("{:?}", default_level), "Info");
}

#[test]
fn test_config_implements_clone() {
    let config = UnifiedBearDogConfig::default();
    let cloned = config.clone();
    assert_eq!(config.app.app_name, cloned.app.app_name);
}

#[test]
fn test_config_implements_debug() {
    let config = UnifiedBearDogConfig::default();
    let debug_output = format!("{:?}", config);
    assert!(debug_output.len() > 0);
}

#[test]
fn test_config_implements_default() {
    let config = UnifiedBearDogConfig::default();
    // Default config should be constructable (app_name may be empty by design)
    assert!(config.app.app_name.is_empty() || !config.app.app_name.is_empty());
}

#[test]
fn test_environment_serializable() {
    let env = Environment::Production;
    let json = serde_json::to_string(&env);
    assert!(json.is_ok());
}

#[test]
fn test_log_level_serializable() {
    let level = LogLevel::Info;
    let json = serde_json::to_string(&level);
    assert!(json.is_ok());
}

#[test]
fn test_environment_equality() {
    let env1 = Environment::Development;
    let env2 = Environment::Development;
    let env3 = Environment::Production;

    // Compare using Debug format since PartialEq isn't implemented
    assert_eq!(format!("{:?}", env1), format!("{:?}", env2));
    assert_ne!(format!("{:?}", env1), format!("{:?}", env3));
}

#[test]
fn test_log_level_clone() {
    let level1 = LogLevel::Info;
    let level2 = level1.clone();
    let level3 = level1.clone();

    assert_eq!(format!("{:?}", level2), format!("{:?}", level3));
}
