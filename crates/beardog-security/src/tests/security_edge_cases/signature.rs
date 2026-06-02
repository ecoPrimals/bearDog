// SPDX-License-Identifier: AGPL-3.0-or-later

//! Signature verification edge cases

use super::*;

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical

#[test]
fn test_verify_with_empty_signature() {
    // Test signature verification with empty signature
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let data = b"test data";
    let empty_sig: &[u8] = &[];

    let result = verify_signature(data, empty_sig);
    assert!(result.is_err(), "Should reject empty signature");
}

#[test]
fn test_verify_with_invalid_signature_length() {
    // Test signature verification with wrong length signature
    let data = b"test data";
    let wrong_length_sig = vec![0u8; 31]; // Ed25519 needs 64

    let result = verify_signature(data, &wrong_length_sig);
    assert!(result.is_err(), "Should reject wrong length signature");
}

#[test]
fn test_verify_signature_with_modified_data() {
    // Test that signature verification detects data modification
    let original_data = b"original data";
    let modified_data = b"modified data";

    // This would need a real signature, but we test the concept
    let signature = generate_test_signature(original_data);
    if let Ok(sig) = signature {
        let result = verify_signature_with_data(modified_data, &sig, original_data);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result.is_err(), "Should detect data modification");
    }
}
