// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constant-time operations edge cases

use super::*;

#[test]
fn test_constant_time_compare_same_data() {
    // Test constant-time comparison with identical data
    let data = b"secret_value";
    let result = constant_time_eq(data, data);

    assert!(result, "Identical data should be equal");
}

#[test]
fn test_constant_time_compare_different_data() {
    // Test constant-time comparison with different data
    let data1 = b"secret_value_1";
    let data2 = b"secret_value_2";

    let result = constant_time_eq(data1, data2);
    assert!(!result, "Different data should not be equal");
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_constant_time_compare_different_lengths() {
    // Test constant-time comparison with different lengths
    let data1 = b"short";
    let data2 = b"much_longer_data";

    let result = constant_time_eq(data1, data2);
    assert!(!result, "Different lengths should not be equal");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[test]
fn test_constant_time_compare_prefix() {
    // Test that prefix matching doesn't cause early exit
    let data1 = b"prefix_different";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let data2 = b"prefix_also_different";

    let result = constant_time_eq(data1, data2);
    assert!(!result, "Should compare entire string");
}
