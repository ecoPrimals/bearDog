// SPDX-License-Identifier: AGPL-3.0-or-later

//! Memory zeroing edge cases

use super::*;

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_secure_zero_sensitive_data() {
    // Test that sensitive data is properly zeroed
    let mut sensitive = vec![0x42u8; 1024];

    secure_zero(&mut sensitive);

    for byte in &sensitive {
        assert_eq!(*byte, 0, "All bytes should be zeroed");
    }
}

#[test]
fn test_secure_zero_empty_slice() {
    // Test zeroing empty slice doesn't panic
    let mut empty: Vec<u8> = vec![];
    secure_zero(&mut empty);

    assert_eq!(empty.len(), 0, "Empty slice should remain empty");
}

#[test]
fn test_secure_zero_large_buffer() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Test zeroing large buffer
    let mut large = vec![0xFFu8; 10 * 1024 * 1024]; // 10MB

    secure_zero(&mut large);

    // Sample check (checking all would be slow)
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert_eq!(large[0], 0);
    assert_eq!(large[large.len() / 2], 0);
    assert_eq!(large[large.len() - 1], 0);
}
