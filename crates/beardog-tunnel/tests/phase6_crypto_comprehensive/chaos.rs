// SPDX-License-Identifier: AGPL-3.0-only

//! Chaos tests — concurrent operations and rapid cycles.

use crate::common::*;

#[test]
fn test_chaos_concurrent_sha256_operations() {
    // Run 100 concurrent SHA-256 operations
    use std::thread;

    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                let data = format!("Message number {}", i);
                let result = handle_sha256(&json!({
                    "data": BASE64.encode(data.as_bytes())
                }));
                result.is_ok()
            })
        })
        .collect();

    // All should succeed
    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

#[test]
fn test_chaos_concurrent_key_generation() {
    // Generate 50 P-256 keypairs concurrently
    use std::thread;

    let handles: Vec<_> = (0..50)
        .map(|_| thread::spawn(|| handle_ecdh_p256_generate(&json!({})).is_ok()))
        .collect();

    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

#[test]
fn test_chaos_rapid_encrypt_decrypt_cycles() {
    // Perform 100 encrypt/decrypt cycles rapidly
    let key = vec![0x42u8; 32];

    for i in 0..100 {
        let plaintext = format!("Message {}", i);

        let encrypt_result = handle_aes256_gcm_encrypt(&json!({
            "plaintext": BASE64.encode(plaintext.as_bytes()),
            "key": BASE64.encode(&key)
        }))
        .unwrap();

        let ciphertext = encrypt_result.get("ciphertext").unwrap().as_str().unwrap();
        let nonce = encrypt_result.get("nonce").unwrap().as_str().unwrap();

        let decrypt_result = handle_aes256_gcm_decrypt(&json!({
            "ciphertext": ciphertext,
            "key": BASE64.encode(&key),
            "nonce": nonce
        }));

        assert!(decrypt_result.is_ok());
    }
}

#[test]
fn test_chaos_concurrent_password_hashing() {
    // Hash 20 passwords concurrently
    use std::thread;

    let handles: Vec<_> = (0..20)
        .map(|i| {
            thread::spawn(move || {
                let password = format!("Password{}", i);
                handle_argon2id_hash(&json!({"password": password})).is_ok()
            })
        })
        .collect();

    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

#[test]
fn test_chaos_mixed_operations_concurrent() {
    // Mix different crypto operations concurrently
    use std::thread;

    let mut handles = vec![];

    // SHA operations
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            handle_sha256(&json!({"data": BASE64.encode(format!("data{}", i).as_bytes())})).is_ok()
        }));
    }

    // ECDH operations
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            handle_ecdh_p256_generate(&json!({})).is_ok()
        }));
    }

    // AES operations
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let key = vec![0x42u8; 32];
            handle_aes256_gcm_encrypt(&json!({
                "plaintext": BASE64.encode(format!("msg{}", i).as_bytes()),
                "key": BASE64.encode(&key)
            }))
            .is_ok()
        }));
    }

    // All should succeed
    for handle in handles {
        assert!(handle.join().unwrap());
    }
}
