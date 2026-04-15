use super::*;

#[test]
fn test_log_level_default() {
    assert_eq!(LogLevel::default(), LogLevel::Info);
}

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Trace < LogLevel::Debug);
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();
    assert!(config.enabled);
    assert_eq!(config.level, LogLevel::Info);
    assert_eq!(config.format, LogFormat::Json);
    assert_eq!(config.targets.len(), 1);
    assert!(config.structured);
}

#[test]
fn test_logging_config_stdout_only() {
    let config = LoggingConfig::stdout_only();
    assert_eq!(config.targets.len(), 1);
    assert_eq!(config.targets[0].target_type, LogTargetType::Stdout);
}

#[test]
fn test_logging_config_file_with_rotation() {
    let config = LoggingConfig::file_with_rotation("/var/log/beardog.log");
    assert_eq!(config.targets.len(), 1);
    assert_eq!(config.targets[0].target_type, LogTargetType::File);
    assert!(config.rotation.is_some());
}

#[test]
fn test_logging_config_validate_success() {
    let config = LoggingConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_logging_config_validate_no_targets() {
    let mut config = LoggingConfig::default();
    config.targets.clear();
    assert!(config.validate().is_err());
}

#[test]
fn test_system_domain_config_default() {
    let config = SystemDomainConfig::default();
    assert_eq!(config.application.name, "BearDog");
    assert!(config.logging.enabled);
    assert!(config.threading.worker_threads > 0);
}

#[test]
fn test_system_domain_config_from_env() {
    let result = SystemDomainConfig::from_env();
    assert!(result.is_ok());
}

#[test]
fn test_system_domain_config_validate_success() {
    let config = SystemDomainConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_system_domain_config_validate_zero_worker_threads() {
    let mut config = SystemDomainConfig::default();
    config.threading.worker_threads = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_threading_config_default() {
    let config = ThreadingConfig::default();
    assert!(config.worker_threads > 0);
    assert!(config.enable_tls_optimization);
}

#[test]
fn test_resource_config_default() {
    let config = ResourceConfig::default();
    assert!(config.max_connections > 0);
    assert!(config.monitoring_interval > Duration::from_secs(0));
}
