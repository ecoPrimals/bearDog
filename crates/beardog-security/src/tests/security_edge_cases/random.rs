// SPDX-License-Identifier: AGPL-3.0-or-later

//! Random number generation edge cases

use super::*;

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: important

#[test]
fn test_random_bytes_length() {
    // Test random generation produces correct length
    let sizes = vec![16, 32, 64, 128, 256];

    for size in sizes {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let random = generate_random_bytes(size);
        assert!(random.is_ok(), "Should generate {size} bytes");

        if let Ok(bytes) = random {
            assert_eq!(bytes.len(), size, "Should have correct length");
        }
    }
}

#[test]
fn test_random_bytes_not_all_same() {
    // Test that random bytes are not all the same value
    let random = generate_random_bytes(256).expect("Should generate");

    let first = random[0];
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let all_same = random.iter().all(|&b| b == first);

    assert!(!all_same, "Random bytes should vary");
}

#[test]
fn test_random_generation_entropy() {
    // Test that random generation has good entropy
    let random = generate_random_bytes(256).expect("Should generate");

    // Count unique bytes
    let mut seen = std::collections::HashSet::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    for &byte in &random {
        seen.insert(byte);
    }

    // Should have reasonable variety (at least 50% of possible values)
    assert!(
        seen.len() >= 128,
        "Should have good entropy: {} unique bytes",
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        seen.len()
    );
}
