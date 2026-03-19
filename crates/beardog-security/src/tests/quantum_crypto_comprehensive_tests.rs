// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Quantum Cryptography Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security
//! `TEST_PRIORITY`: high
//!
//! Comprehensive test coverage for quantum-resistant cryptographic operations.
//! Part of Week 1 test expansion (Oct 31, 2025).

use crate::quantum_crypto::*;

#[cfg(test)]
mod quantum_crypto_comprehensive_tests {
    use super::*;

    // ========================================================================
    // Quantum Key Generation Tests
    // ========================================================================

    #[test]
    fn test_quantum_key_generation_basic() {
        let result = generate_quantum_resistant_keypair();
        
        assert!(result.is_ok());
        let keypair = result.unwrap();
        
        assert!(!keypair.public_key.is_empty());
        assert!(!keypair.private_key.is_empty());
        assert_ne!(keypair.public_key, keypair.private_key);
    }

    #[test]
    fn test_quantum_key_generation_uniqueness() {
        let keypair1 = generate_quantum_resistant_keypair().unwrap();
        let keypair2 = generate_quantum_resistant_keypair().unwrap();
        
        // Each generation should produce unique keys
        assert_ne!(keypair1.public_key, keypair2.public_key);
        assert_ne!(keypair1.private_key, keypair2.private_key);
    }

    #[test]
    fn test_quantum_key_size() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        
        // Quantum-resistant keys should be appropriately sized
        assert!(keypair.public_key.len() >= 32);
        assert!(keypair.private_key.len() >= 32);
    }

    #[test]
    fn test_multiple_key_generations() {
        for _ in 0..10 {
            let result = generate_quantum_resistant_keypair();
            assert!(result.is_ok());
        }
    }

    // ========================================================================
    // Quantum Signature Tests
    // ========================================================================

    #[test]
    fn test_quantum_signature_creation() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let message = b"test message";
        
        let result = sign_quantum_resistant(message, &keypair.private_key);
        
        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(!signature.is_empty());
    }

    #[test]
    fn test_quantum_signature_verification_valid() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let message = b"test message";
        
        let signature = sign_quantum_resistant(message, &keypair.private_key).unwrap();
        let result = verify_quantum_resistant(message, &signature, &keypair.public_key);
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_quantum_signature_verification_invalid() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let message = b"test message";
        let wrong_message = b"wrong message";
        
        let signature = sign_quantum_resistant(message, &keypair.private_key).unwrap();
        let result = verify_quantum_resistant(wrong_message, &signature, &keypair.public_key);
        
        // Should fail verification with wrong message
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_quantum_signature_deterministic() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let message = b"test message";
        
        let sig1 = sign_quantum_resistant(message, &keypair.private_key).unwrap();
        let sig2 = sign_quantum_resistant(message, &keypair.private_key).unwrap();
        
        // Signatures should be consistent for same message and key
        assert_eq!(sig1, sig2);
    }

    // ========================================================================
    // Hybrid Cryptography Tests
    // ========================================================================

    #[test]
    fn test_hybrid_encryption_basic() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let plaintext = b"sensitive data";
        
        let result = hybrid_encrypt(plaintext, &keypair.public_key);
        
        assert!(result.is_ok());
        let ciphertext = result.unwrap();
        assert!(!ciphertext.is_empty());
        assert_ne!(plaintext.to_vec(), ciphertext);
    }

    #[test]
    fn test_hybrid_encryption_decryption_roundtrip() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let plaintext = b"sensitive data";
        
        let ciphertext = hybrid_encrypt(plaintext, &keypair.public_key).unwrap();
        let result = hybrid_decrypt(&ciphertext, &keypair.private_key);
        
        assert!(result.is_ok());
        assert_eq!(plaintext.to_vec(), result.unwrap());
    }

    #[test]
    fn test_hybrid_encryption_different_keys() {
        let keypair1 = generate_quantum_resistant_keypair().unwrap();
        let keypair2 = generate_quantum_resistant_keypair().unwrap();
        let plaintext = b"sensitive data";
        
        let ciphertext = hybrid_encrypt(plaintext, &keypair1.public_key).unwrap();
        let result = hybrid_decrypt(&ciphertext, &keypair2.private_key);
        
        // Decryption with wrong key should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_hybrid_encryption_empty_data() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let plaintext = b"";
        
        let result = hybrid_encrypt(plaintext, &keypair.public_key);
        
        // Should handle empty data gracefully
        assert!(result.is_ok());
    }

    #[test]
    fn test_hybrid_encryption_large_data() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let plaintext = vec![0u8; 10000]; // 10KB of data
        
        let ciphertext = hybrid_encrypt(&plaintext, &keypair.public_key).unwrap();
        let decrypted = hybrid_decrypt(&ciphertext, &keypair.private_key).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    // ========================================================================
    // Quantum Key Exchange Tests
    // ========================================================================

    #[test]
    fn test_quantum_key_exchange_basic() {
        let alice_keypair = generate_quantum_resistant_keypair().unwrap();
        let bob_keypair = generate_quantum_resistant_keypair().unwrap();
        
        let alice_shared = quantum_key_exchange(&bob_keypair.public_key, &alice_keypair.private_key);
        let bob_shared = quantum_key_exchange(&alice_keypair.public_key, &bob_keypair.private_key);
        
        assert!(alice_shared.is_ok());
        assert!(bob_shared.is_ok());
        
        // Both parties should derive the same shared secret
        assert_eq!(alice_shared.unwrap(), bob_shared.unwrap());
    }

    #[test]
    fn test_quantum_key_exchange_uniqueness() {
        let keypair1 = generate_quantum_resistant_keypair().unwrap();
        let keypair2 = generate_quantum_resistant_keypair().unwrap();
        let keypair3 = generate_quantum_resistant_keypair().unwrap();
        
        let shared_12 = quantum_key_exchange(&keypair2.public_key, &keypair1.private_key).unwrap();
        let shared_13 = quantum_key_exchange(&keypair3.public_key, &keypair1.private_key).unwrap();
        
        // Different key pairs should produce different shared secrets
        assert_ne!(shared_12, shared_13);
    }

    // ========================================================================
    // Quantum Random Generation Tests
    // ========================================================================

    #[test]
    fn test_quantum_random_generation() {
        let result = generate_quantum_random_bytes(32);
        
        assert!(result.is_ok());
        let random_bytes = result.unwrap();
        assert_eq!(random_bytes.len(), 32);
    }

    #[test]
    fn test_quantum_random_uniqueness() {
        let random1 = generate_quantum_random_bytes(32).unwrap();
        let random2 = generate_quantum_random_bytes(32).unwrap();
        
        // Random generations should be unique
        assert_ne!(random1, random2);
    }

    #[test]
    fn test_quantum_random_various_sizes() {
        for size in [16, 32, 64, 128, 256] {
            let result = generate_quantum_random_bytes(size);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), size);
        }
    }

    // ========================================================================
    // Edge Cases and Error Handling
    // ========================================================================

    #[test]
    fn test_verify_with_invalid_signature() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let message = b"test message";
        let invalid_signature = vec![0u8; 64];
        
        let result = verify_quantum_resistant(message, &invalid_signature, &keypair.public_key);
        
        // Should handle invalid signature gracefully
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_decrypt_invalid_ciphertext() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        let invalid_ciphertext = vec![0u8; 100];
        
        let result = hybrid_decrypt(&invalid_ciphertext, &keypair.private_key);
        
        // Should return error for invalid ciphertext
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_operations_with_empty_keys() {
        let message = b"test";
        let empty_key = vec![];
        
        let sig_result = sign_quantum_resistant(message, &empty_key);
        assert!(sig_result.is_err());
        
        let enc_result = hybrid_encrypt(message, &empty_key);
        assert!(enc_result.is_err());
    }

    // ========================================================================
    // Performance and Stress Tests
    // ========================================================================

    #[test]
    fn test_quantum_operations_performance() {
        let start = std::time::Instant::now();
        
        for _ in 0..10 {
            let _ = generate_quantum_resistant_keypair();
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time
        assert!(duration.as_secs() < 5);
    }

    #[test]
    fn test_multiple_signatures() {
        let keypair = generate_quantum_resistant_keypair().unwrap();
        
        for i in 0..20 {
            let message = format!("message {}", i);
            let signature = sign_quantum_resistant(message.as_bytes(), &keypair.private_key);
            assert!(signature.is_ok());
        }
    }

    #[test]
    fn test_concurrent_key_generation() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        let success_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let counter = success_count.clone();
            let handle = std::thread::spawn(move || {
                if generate_quantum_resistant_keypair().is_ok() {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(success_count.load(Ordering::SeqCst), 5);
    }
}

