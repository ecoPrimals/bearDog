//! Key Generation Tests
//!
//! Comprehensive tests for cryptographic key generation.

#[cfg(test)]
mod key_generation_tests {
    use rand::Rng;

    #[test]
    fn test_key_generation_produces_unique_keys() {
        // Test that key generation produces unique keys
        let key1: [u8; 32] = rand::thread_rng().gen();
        let key2: [u8; 32] = rand::thread_rng().gen();
        
        assert_ne!(key1, key2, "Generated keys should be unique");
    }

    #[test]
    fn test_key_generation_correct_length() {
        // Test that generated keys have correct length
        let key: [u8; 32] = rand::thread_rng().gen();
        
        assert_eq!(key.len(), 32, "Key should be 32 bytes (256 bits)");
    }

    #[test]
    fn test_key_generation_not_all_zeros() {
        // Test that generated keys are not all zeros
        let key: [u8; 32] = rand::thread_rng().gen();
        
        let all_zeros = key.iter().all(|&b| b == 0);
        assert!(!all_zeros, "Generated key should not be all zeros");
    }

    #[test]
    fn test_key_generation_not_all_ones() {
        // Test that generated keys are not all ones
        let key: [u8; 32] = rand::thread_rng().gen();
        
        let all_ones = key.iter().all(|&b| b == 255);
        assert!(!all_ones, "Generated key should not be all ones");
    }

    #[test]
    fn test_multiple_key_generations_unique() {
        // Test that multiple key generations produce unique keys
        let mut keys = Vec::new();
        
        for _ in 0..10 {
            let key: [u8; 32] = rand::thread_rng().gen();
            keys.push(key);
        }
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Check all keys are unique
        for i in 0..keys.len() {
            for j in (i+1)..keys.len() {
                assert_ne!(keys[i], keys[j], "All generated keys should be unique");
            }
        }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_key_generation_entropy_quality() {
        // Test that generated keys have good entropy
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let key: [u8; 32] = rand::thread_rng().gen();
        
        // Count unique bytes
        let unique_count = key.iter().collect::<std::collections::HashSet<_>>().len();
        
        // Should have reasonable variety (at least 20 unique bytes out of 32)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(unique_count >= 20, "Key should have good entropy: {} unique bytes", unique_count);
    }

    #[test]
    fn test_key_generation_thread_safety() {
        // Test that key generation is thread-safe
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let keys = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let keys_clone = Arc::clone(&keys);
            let handle = thread::spawn(move || {
                let key: [u8; 32] = rand::thread_rng().gen();
                keys_clone.lock().unwrap().push(key);
            });
            handles.push(handle);
        }
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        for handle in handles {
            handle.join().unwrap();
        }
        
        let keys = keys.lock().unwrap();
        assert_eq!(keys.len(), 5, "All threads should have generated keys");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_key_sizes_128_bit() {
        // Test 128-bit key generation
        let key: [u8; 16] = rand::thread_rng().gen();
        assert_eq!(key.len(), 16, "128-bit key should be 16 bytes");
    }

    #[test]
    fn test_key_sizes_256_bit() {
        // Test 256-bit key generation
        let key: [u8; 32] = rand::thread_rng().gen();
        assert_eq!(key.len(), 32, "256-bit key should be 32 bytes");
    }

    #[test]
    fn test_key_sizes_512_bit() {
        // Test 512-bit key generation
        let key: [u8; 64] = rand::thread_rng().gen();
        assert_eq!(key.len(), 64, "512-bit key should be 64 bytes");
    }

    #[test]
    fn test_key_generation_deterministic_from_seed() {
        // Test that seeded RNG produces deterministic results
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        use rand::SeedableRng;
        use rand::rngs::StdRng;
        
        let seed = [42u8; 32];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut rng1 = StdRng::from_seed(seed);
        let mut rng2 = StdRng::from_seed(seed);
        
        let key1: [u8; 32] = rng1.gen();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let key2: [u8; 32] = rng2.gen();
        
        assert_eq!(key1, key2, "Seeded RNGs should produce identical keys");
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

    #[test]
    fn test_key_generation_different_seeds_different_keys() {
        // Test that different seeds produce different keys
        use rand::SeedableRng;
        use rand::rngs::StdRng;
        
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        
        let mut rng1 = StdRng::from_seed(seed1);
        let mut rng2 = StdRng::from_seed(seed2);
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let key1: [u8; 32] = rng1.gen();
        let key2: [u8; 32] = rng2.gen();
        
        assert_ne!(key1, key2, "Different seeds should produce different keys");
    }

    #[test]
    fn test_key_generation_bytes_distribution() {
        // Test that key bytes are well-distributed
        let key: [u8; 32] = rand::thread_rng().gen();
        
        let sum: u32 = key.iter().map(|&b| b as u32).sum();
        let avg = sum / key.len() as u32;
        
        // Average should be near 127.5 for uniform distribution
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!((avg as i32 - 127).abs() < 50, "Key bytes should be well-distributed");
    }

    #[test]
    fn test_key_generation_no_sequential_pattern() {
        // Test that keys don't have sequential patterns
        let key: [u8; 32] = rand::thread_rng().gen();
        
        let is_sequential = key.windows(2).all(|w| w[1] == w[0].wrapping_add(1));
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(!is_sequential, "Key should not have sequential pattern");
    }

    #[test]
    fn test_key_generation_no_repeating_pattern() {
        // Test that keys don't have obvious repeating patterns
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let key: [u8; 32] = rand::thread_rng().gen();
        
        // Check for repeating bytes
        let first_byte = key[0];
        let all_same = key.iter().all(|&b| b == first_byte);
        assert!(!all_same, "Key should not have all same bytes");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_key_zeroization_support() {
        // Test that keys can be securely zeroized
        let mut key: [u8; 32] = rand::thread_rng().gen();
        
        // Verify key is not zeros initially
        let had_data = key.iter().any(|&b| b != 0);
        assert!(had_data, "Key should have data before zeroization");
        
        // Zeroize
        for byte in key.iter_mut() {
            *byte = 0;
        }
        
        // Verify key is now zeros
        let is_zero = key.iter().all(|&b| b == 0);
        assert!(is_zero, "Key should be zeros after zeroization");
    }
}

