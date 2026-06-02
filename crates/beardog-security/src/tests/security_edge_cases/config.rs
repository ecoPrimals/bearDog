// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration validation edge cases

use super::*;

#[test]
fn test_config_with_invalid_key_size() {
    // Test configuration with invalid key size
    let config = create_config_with_key_size(15); // Invalid
    let result = validate_config(&config);

    assert!(result.is_err(), "Should reject invalid key size");
}

#[test]
fn test_config_with_zero_timeout() {
    // Test configuration with zero timeout
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    let config = create_config_with_timeout(0);
    let result = validate_config(&config);

    assert!(result.is_err(), "Should reject zero timeout");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_config_with_negative_values() {
    // Test configuration rejects negative values where inappropriate
    let config = create_config_with_max_attempts(-1);
    let result = validate_config(&config);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert!(result.is_err(), "Should reject negative values");
}
