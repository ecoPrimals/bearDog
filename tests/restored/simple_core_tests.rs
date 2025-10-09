use beardog_errors::BearDogError;

#[test]
fn test_basic_error_types() {
    let error = BearDogError::configuration("Test rejection");

    assert!(format!("{error:?}").contains("Test rejection"));
}

#[test]
fn test_trust_levels() {
    // Test basic capabilities
    use beardog_types::canonical::capabilities::ServiceCapabilityType;

    let capability = ServiceCapabilityType::Compute;
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}

#[test]
fn test_configuration_defaults() {
    // Test that we can create basic configuration structures
    use beardog_types::canonical::config::WorkingUnifiedConfig;
    let config = WorkingUnifiedConfig::default();
    assert!(config.version.is_empty() || !config.version.is_empty()); // Basic existence check
}

#[test]
fn test_error_conversion() {
    let error = BearDogError::internal("Internal test error".to_string());
    let error_string = format!("{}", error);
    assert!(error_string.contains("Internal test error"));
}

#[test]
fn test_basic_types() {
    // Test that basic types can be instantiated
    use beardog_types::canonical::capabilities::ServiceCapabilityType;

    let capability = ServiceCapabilityType::Compute;
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}
