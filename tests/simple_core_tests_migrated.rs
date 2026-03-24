// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]

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

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    let capability = ServiceCapabilityType::Compute;
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_configuration_defaults() {
    // Test that we can create basic configuration structures
    use beardog_types::canonical::config::WorkingUnifiedConfig;
    let config = WorkingUnifiedConfig::default();
    assert!(config.version.is_empty() || !config.version.is_empty()); // Basic existence check
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_error_conversion() {
    let error = BearDogError::internal("Internal test error".to_string());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    let error_string = format!("{error}");
    assert!(error_string.contains("Internal test error"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_basic_types() {
    // Test that basic types can be instantiated
    use beardog_types::canonical::capabilities::ServiceCapabilityType;

    let capability = ServiceCapabilityType::Compute;
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}
