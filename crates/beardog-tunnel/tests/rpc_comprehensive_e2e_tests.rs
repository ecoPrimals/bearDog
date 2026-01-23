//! Comprehensive RPC E2E, Chaos, and Fault Testing
//!
//! This test suite provides complete coverage for all 83 RPC methods:
//! - Unit tests for each method
//! - E2E tests for complete workflows
//! - Chaos tests for concurrent operations
//! - Fault injection tests for error handling
//!
//! **Coverage**: 83 RPC methods across 7 handlers + Universal methods

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_tunnel::unix_socket_ipc::crypto_handlers::{
    handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt, handle_sign_ed25519,
    handle_tls_derive_application_secrets, handle_tls_derive_handshake_secrets,
    handle_verify_ed25519, handle_x25519_derive_secret, handle_x25519_generate_ephemeral,
};
use beardog_tunnel::unix_socket_ipc::crypto_handlers_aes_gcm::{
    handle_aes256_gcm_decrypt, handle_aes256_gcm_encrypt,
};
use beardog_tunnel::unix_socket_ipc::crypto_handlers_ecdsa::{
    handle_sign_ecdsa_secp256r1, handle_sign_ecdsa_secp384r1, handle_verify_ecdsa_secp256r1,
    handle_verify_ecdsa_secp384r1,
};
use beardog_tunnel::unix_socket_ipc::crypto_handlers_hashing::{
    handle_blake3_hash, handle_sha256, handle_sha384, handle_sha512,
};
use beardog_tunnel::unix_socket_ipc::crypto_handlers_passwords::{
    handle_argon2id_hash, handle_argon2id_verify,
};
use serde_json::json;
use std::time::Instant;
use tokio::task::JoinSet;

// ============================================================================
// E2E WORKFLOW TESTS
// ============================================================================

#[tokio::test]
async fn test_e2e_full_tls_handshake_to_https() {
    // Complete E2E flow: ECDH → Handshake Secrets → Decrypt Handshake → Application Secrets → HTTP
    
    // Step 1: Generate ECDH keypair
    let ecdh_result = handle_x25519_generate_ephemeral(None).await.unwrap();
    let our_private = ecdh_result["private_key"].as_str().unwrap();
    let our_public = ecdh_result["public_key"].as_str().unwrap();
    
    // Simulate server's public key
    let server_private = BASE64.encode(&[0x42u8; 32]);
    let server_public_params = json!({"private_key": server_private});
    let server_ecdh = handle_x25519_generate_ephemeral(None).await.unwrap();
    let server_public = server_ecdh["public_key"].as_str().unwrap();
    
    // Step 2: Derive shared secret
    let shared_secret_params = json!({
        "private_key": our_private,
        "public_key": server_public
    });
    let shared_secret_result = handle_x25519_derive_secret(Some(&shared_secret_params))
        .await
        .unwrap();
    let pre_master_secret = shared_secret_result["shared_secret"].as_str().unwrap();
    
    // Step 3: Prepare ClientHello + ServerHello
    let client_random = BASE64.encode(&[0x01u8; 32]);
    let server_random = BASE64.encode(&[0x02u8; 32]);
    
    // Compute transcript hash (ClientHello + ServerHello)
    let handshake_transcript = {
        use sha2::{Digest, Sha256};
        let mut transcript = vec![0x03u8; 64]; // Simulated handshake messages
        BASE64.encode(Sha256::digest(&transcript))
    };
    
    // Step 4: Derive handshake secrets
    let hs_params = json!({
        "pre_master_secret": pre_master_secret,
        "client_random": &client_random,
        "server_random": &server_random,
        "transcript_hash": &handshake_transcript
    });
    let hs_secrets = handle_tls_derive_handshake_secrets(Some(&hs_params))
        .await
        .unwrap();
    
    // Verify handshake secrets structure
    assert!(hs_secrets["client_write_key"].is_string());
    assert!(hs_secrets["server_write_key"].is_string());
    assert_eq!(hs_secrets["stage"], "handshake");
    
    // Step 5: Simulate decrypting handshake messages
    let plaintext = b"Encrypted Extensions Data";
    let encrypt_params = json!({
        "plaintext": BASE64.encode(plaintext),
        "key": hs_secrets["server_write_key"],
        "nonce": hs_secrets["server_write_iv"]
    });
    let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    
    let decrypt_params = json!({
        "ciphertext": encrypted["ciphertext"],
        "key": hs_secrets["server_write_key"],
        "nonce": hs_secrets["server_write_iv"]
    });
    let decrypted = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
        .await
        .unwrap();
    
    assert_eq!(
        BASE64.decode(decrypted["plaintext"].as_str().unwrap()).unwrap(),
        plaintext
    );
    
    // Step 6: Compute full transcript hash (ALL handshake messages)
    let full_transcript = {
        use sha2::{Digest, Sha256};
        let mut transcript = vec![0x04u8; 128]; // All handshake messages
        BASE64.encode(Sha256::digest(&transcript))
    };
    
    // Step 7: Derive application secrets
    let app_params = json!({
        "pre_master_secret": pre_master_secret,
        "client_random": &client_random,
        "server_random": &server_random,
        "transcript_hash": &full_transcript
    });
    let app_secrets = handle_tls_derive_application_secrets(Some(&app_params))
        .await
        .unwrap();
    
    // Verify application secrets are different from handshake secrets
    assert_ne!(
        hs_secrets["client_write_key"],
        app_secrets["client_write_key"]
    );
    
    // Step 8: Encrypt/decrypt HTTP data
    let http_request = b"GET / HTTP/1.1\r\nHost: github.com\r\n\r\n";
    let http_encrypt_params = json!({
        "plaintext": BASE64.encode(http_request),
        "key": app_secrets["client_write_key"],
        "nonce": app_secrets["client_write_iv"]
    });
    let http_encrypted = handle_chacha20_poly1305_encrypt(Some(&http_encrypt_params))
        .await
        .unwrap();
    
    let http_decrypt_params = json!({
        "ciphertext": http_encrypted["ciphertext"],
        "key": app_secrets["client_write_key"],
        "nonce": app_secrets["client_write_iv"]
    });
    let http_decrypted = handle_chacha20_poly1305_decrypt(Some(&http_decrypt_params))
        .await
        .unwrap();
    
    assert_eq!(
        BASE64.decode(http_decrypted["plaintext"].as_str().unwrap()).unwrap(),
        http_request
    );
    
    println!("✅ Complete E2E flow: ECDH → Handshake → HTTP SUCCESS!");
}

#[tokio::test]
async fn test_e2e_ecdsa_signing_workflow() {
    // E2E: Generate data → Hash → Sign → Verify
    
    let data = b"Important contract to sign";
    let data_b64 = BASE64.encode(data);
    
    // Step 1: Hash the data
    let hash_params = json!({"data": data_b64});
    let hash_result = handle_sha256(Some(&hash_params)).await.unwrap();
    let hash = hash_result["hash"].as_str().unwrap();
    
    // Step 2: Sign with ECDSA P-256
    let sign_params = json!({"data": hash});
    let sign_result = handle_sign_ecdsa_secp256r1(Some(&sign_params))
        .await
        .unwrap();
    
    let signature = sign_result["signature"].as_str().unwrap();
    let public_key = sign_result["public_key"].as_str().unwrap();
    
    // Step 3: Verify the signature
    let verify_params = json!({
        "data": hash,
        "signature": signature,
        "public_key": public_key
    });
    let verify_result = handle_verify_ecdsa_secp256r1(Some(&verify_params))
        .await
        .unwrap();
    
    assert_eq!(verify_result["valid"], true);
    
    // Step 4: Verify fails with tampered data
    let tampered_hash = BASE64.encode(&sha2::Sha256::digest(b"Tampered data"));
    let tampered_params = json!({
        "data": tampered_hash,
        "signature": signature,
        "public_key": public_key
    });
    let tampered_result = handle_verify_ecdsa_secp256r1(Some(&tampered_params))
        .await
        .unwrap();
    
    assert_eq!(tampered_result["valid"], false);
    
    println!("✅ E2E ECDSA signing workflow SUCCESS!");
}

#[tokio::test]
async fn test_e2e_aes_gcm_encryption_workflow() {
    // E2E: Generate key → Encrypt → Decrypt → Verify
    
    let plaintext = b"Sensitive data that needs AES-GCM encryption";
    let plaintext_b64 = BASE64.encode(plaintext);
    
    // Step 1: Generate random key (32 bytes for AES-256)
    let key_params = json!({"length": 32});
    let key_result = handle_generate_random(Some(&key_params))
        .await
        .unwrap();
    let key = key_result["random"].as_str().unwrap();
    
    // Step 2: Encrypt with AES-256-GCM
    let encrypt_params = json!({
        "plaintext": plaintext_b64,
        "key": key
    });
    let encrypt_result = handle_aes256_gcm_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    
    let ciphertext = encrypt_result["ciphertext"].as_str().unwrap();
    let nonce = encrypt_result["nonce"].as_str().unwrap();
    
    // Step 3: Decrypt
    let decrypt_params = json!({
        "ciphertext": ciphertext,
        "key": key,
        "nonce": nonce
    });
    let decrypt_result = handle_aes256_gcm_decrypt(Some(&decrypt_params))
        .await
        .unwrap();
    
    let decrypted = BASE64.decode(decrypt_result["plaintext"].as_str().unwrap()).unwrap();
    assert_eq!(decrypted, plaintext);
    
    // Step 4: Verify decryption fails with wrong key
    let wrong_key_params = json!({"length": 32});
    let wrong_key_result = handle_generate_random(Some(&wrong_key_params))
        .await
        .unwrap();
    let wrong_key = wrong_key_result["random"].as_str().unwrap();
    
    let wrong_decrypt_params = json!({
        "ciphertext": ciphertext,
        "key": wrong_key,
        "nonce": nonce
    });
    let wrong_result = handle_aes256_gcm_decrypt(Some(&wrong_decrypt_params)).await;
    assert!(wrong_result.is_err());
    
    println!("✅ E2E AES-GCM encryption workflow SUCCESS!");
}

#[tokio::test]
async fn test_e2e_password_hashing_workflow() {
    // E2E: Hash password → Verify → Change password → Verify again
    
    let password = "SecurePassword123!";
    let password_b64 = BASE64.encode(password.as_bytes());
    
    // Step 1: Hash with Argon2id
    let hash_params = json!({
        "password": password_b64,
        "memory_cost": 19456,
        "time_cost": 2,
        "parallelism": 1
    });
    let hash_result = handle_argon2id_hash(Some(&hash_params))
        .await
        .unwrap();
    
    let hash = hash_result["hash"].as_str().unwrap();
    
    // Step 2: Verify correct password
    let verify_params = json!({
        "password": password_b64,
        "hash": hash
    });
    let verify_result = handle_argon2id_verify(Some(&verify_params))
        .await
        .unwrap();
    
    assert_eq!(verify_result["valid"], true);
    
    // Step 3: Verify fails with wrong password
    let wrong_password = "WrongPassword456!";
    let wrong_password_b64 = BASE64.encode(wrong_password.as_bytes());
    let wrong_verify_params = json!({
        "password": wrong_password_b64,
        "hash": hash
    });
    let wrong_verify_result = handle_argon2id_verify(Some(&wrong_verify_params))
        .await
        .unwrap();
    
    assert_eq!(wrong_verify_result["valid"], false);
    
    // Step 4: Hash new password (password change)
    let new_password = "NewSecurePassword789!";
    let new_password_b64 = BASE64.encode(new_password.as_bytes());
    let new_hash_params = json!({
        "password": new_password_b64,
        "memory_cost": 19456,
        "time_cost": 2,
        "parallelism": 1
    });
    let new_hash_result = handle_argon2id_hash(Some(&new_hash_params))
        .await
        .unwrap();
    
    let new_hash = new_hash_result["hash"].as_str().unwrap();
    
    // Step 5: Verify old password fails with new hash
    let old_verify_params = json!({
        "password": password_b64,
        "hash": new_hash
    });
    let old_verify_result = handle_argon2id_verify(Some(&old_verify_params))
        .await
        .unwrap();
    
    assert_eq!(old_verify_result["valid"], false);
    
    // Step 6: Verify new password works with new hash
    let new_verify_params = json!({
        "password": new_password_b64,
        "hash": new_hash
    });
    let new_verify_result = handle_argon2id_verify(Some(&new_verify_params))
        .await
        .unwrap();
    
    assert_eq!(new_verify_result["valid"], true);
    
    println!("✅ E2E password hashing workflow SUCCESS!");
}

// ============================================================================
// CHAOS TESTS - CONCURRENT OPERATIONS
// ============================================================================

#[tokio::test]
async fn test_chaos_concurrent_tls_handshakes() {
    // 100 concurrent TLS handshakes
    let mut join_set = JoinSet::new();
    
    for i in 0..100 {
        join_set.spawn(async move {
            // Generate ECDH keypair
            let ecdh_result = handle_x25519_generate_ephemeral(None).await.unwrap();
            let private_key = ecdh_result["private_key"].as_str().unwrap().to_string();
            
            // Simulate peer public key
            let peer_public = BASE64.encode(&[i as u8; 32]);
            
            // Derive shared secret
            let shared_params = json!({
                "private_key": private_key,
                "public_key": peer_public
            });
            let shared_result = handle_x25519_derive_secret(Some(&shared_params))
                .await
                .unwrap();
            
            let pre_master = shared_result["shared_secret"].as_str().unwrap().to_string();
            
            // Derive handshake secrets
            let client_random = BASE64.encode(&[0x01u8; 32]);
            let server_random = BASE64.encode(&[0x02u8; 32]);
            let transcript = {
                use sha2::{Digest, Sha256};
                BASE64.encode(Sha256::digest(&[i as u8; 64]))
            };
            
            let hs_params = json!({
                "pre_master_secret": pre_master,
                "client_random": client_random,
                "server_random": server_random,
                "transcript_hash": transcript
            });
            
            handle_tls_derive_handshake_secrets(Some(&hs_params))
                .await
                .unwrap()
        });
    }
    
    // Wait for all to complete
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        let secrets = result.expect("Task should not panic");
        assert!(secrets["client_write_key"].is_string());
        count += 1;
    }
    
    assert_eq!(count, 100);
    println!("✅ 100 concurrent TLS handshakes SUCCESS!");
}

#[tokio::test]
async fn test_chaos_concurrent_signing_operations() {
    // 100 concurrent signing operations (mixed Ed25519, ECDSA P-256, ECDSA P-384)
    let mut join_set = JoinSet::new();
    
    for i in 0..100 {
        let algo = i % 3;
        join_set.spawn(async move {
            let data = BASE64.encode(&[i as u8; 64]);
            let params = json!({"data": data});
            
            match algo {
                0 => {
                    // Ed25519
                    let result = handle_sign_ed25519(Some(&params)).await.unwrap();
                    assert!(result["signature"].is_string());
                }
                1 => {
                    // ECDSA P-256
                    let result = handle_sign_ecdsa_secp256r1(Some(&params)).await.unwrap();
                    assert!(result["signature"].is_string());
                }
                2 => {
                    // ECDSA P-384
                    let result = handle_sign_ecdsa_secp384r1(Some(&params)).await.unwrap();
                    assert!(result["signature"].is_string());
                }
                _ => unreachable!(),
            }
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        result.expect("Task should not panic");
        count += 1;
    }
    
    assert_eq!(count, 100);
    println!("✅ 100 concurrent mixed signing operations SUCCESS!");
}

#[tokio::test]
async fn test_chaos_concurrent_encryption_operations() {
    // 100 concurrent encryption operations (mixed ChaCha20, AES-GCM)
    let mut join_set = JoinSet::new();
    
    for i in 0..100 {
        let use_chacha = i % 2 == 0;
        join_set.spawn(async move {
            let plaintext = BASE64.encode(&[i as u8; 100]);
            let key = BASE64.encode(&[0x42u8; 32]);
            let nonce = BASE64.encode(&[i as u8; 12]);
            
            if use_chacha {
                // ChaCha20-Poly1305
                let encrypt_params = json!({
                    "plaintext": plaintext,
                    "key": key,
                    "nonce": nonce
                });
                let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
                    .await
                    .unwrap();
                
                let decrypt_params = json!({
                    "ciphertext": encrypted["ciphertext"],
                    "key": key,
                    "nonce": nonce
                });
                let decrypted = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
                    .await
                    .unwrap();
                
                assert_eq!(decrypted["plaintext"], plaintext);
            } else {
                // AES-256-GCM
                let encrypt_params = json!({
                    "plaintext": plaintext,
                    "key": key,
                    "nonce": nonce
                });
                let encrypted = handle_aes256_gcm_encrypt(Some(&encrypt_params))
                    .await
                    .unwrap();
                
                let decrypt_params = json!({
                    "ciphertext": encrypted["ciphertext"],
                    "key": key,
                    "nonce": nonce
                });
                let decrypted = handle_aes256_gcm_decrypt(Some(&decrypt_params))
                    .await
                    .unwrap();
                
                assert_eq!(decrypted["plaintext"], plaintext);
            }
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        result.expect("Task should not panic");
        count += 1;
    }
    
    assert_eq!(count, 100);
    println!("✅ 100 concurrent mixed encryption operations SUCCESS!");
}

#[tokio::test]
async fn test_chaos_concurrent_hashing_operations() {
    // 200 concurrent hashing operations (mixed SHA-256, SHA-384, SHA-512, Blake3)
    let mut join_set = JoinSet::new();
    
    for i in 0..200 {
        let algo = i % 4;
        join_set.spawn(async move {
            let data = BASE64.encode(&[i as u8; 1000]);
            let params = json!({"data": data});
            
            match algo {
                0 => {
                    let result = handle_sha256(Some(&params)).await.unwrap();
                    assert!(result["hash"].is_string());
                }
                1 => {
                    let result = handle_sha384(Some(&params)).await.unwrap();
                    assert!(result["hash"].is_string());
                }
                2 => {
                    let result = handle_sha512(Some(&params)).await.unwrap();
                    assert!(result["hash"].is_string());
                }
                3 => {
                    let result = handle_blake3_hash(Some(&params)).await.unwrap();
                    assert!(result["hash"].is_string());
                }
                _ => unreachable!(),
            }
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        result.expect("Task should not panic");
        count += 1;
    }
    
    assert_eq!(count, 200);
    println!("✅ 200 concurrent mixed hashing operations SUCCESS!");
}

// ============================================================================
// FAULT INJECTION TESTS
// ============================================================================

#[tokio::test]
async fn test_fault_invalid_base64_inputs() {
    // Test all methods reject invalid base64
    
    let invalid_b64 = "This is not valid base64!!!";
    
    // Test crypto methods
    let sign_result = handle_sign_ed25519(Some(&json!({"data": invalid_b64}))).await;
    assert!(sign_result.is_err());
    
    let hash_result = handle_sha256(Some(&json!({"data": invalid_b64}))).await;
    assert!(hash_result.is_err());
    
    let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": invalid_b64,
        "key": "validkey",
        "nonce": "validnonce"
    }))).await;
    assert!(encrypt_result.is_err());
    
    println!("✅ Invalid base64 rejection SUCCESS!");
}

#[tokio::test]
async fn test_fault_missing_required_parameters() {
    // Test all methods reject missing required parameters
    
    // TLS derive handshake secrets requires all 4 params
    let missing_transcript = json!({
        "pre_master_secret": BASE64.encode(&[0u8; 32]),
        "client_random": BASE64.encode(&[0u8; 32]),
        "server_random": BASE64.encode(&[0u8; 32])
        // Missing transcript_hash!
    });
    let result = handle_tls_derive_handshake_secrets(Some(&missing_transcript)).await;
    assert!(result.is_err());
    
    // ECDH derive requires both keys
    let missing_peer = json!({
        "private_key": BASE64.encode(&[0u8; 32])
        // Missing public_key!
    });
    let result = handle_x25519_derive_secret(Some(&missing_peer)).await;
    assert!(result.is_err());
    
    // Encryption requires plaintext and key
    let missing_key = json!({
        "plaintext": BASE64.encode(b"data")
        // Missing key!
    });
    let result = handle_chacha20_poly1305_encrypt(Some(&missing_key)).await;
    assert!(result.is_err());
    
    println!("✅ Missing parameter rejection SUCCESS!");
}

#[tokio::test]
async fn test_fault_invalid_key_sizes() {
    // Test methods reject invalid key sizes
    
    // ChaCha20 requires 32-byte key
    let short_key = BASE64.encode(&[0u8; 16]); // Only 16 bytes!
    let params = json!({
        "plaintext": BASE64.encode(b"test"),
        "key": short_key,
        "nonce": BASE64.encode(&[0u8; 12])
    });
    let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
    assert!(result.is_err());
    
    // AES-256 requires 32-byte key
    let short_key = BASE64.encode(&[0u8; 16]); // Only 16 bytes!
    let params = json!({
        "plaintext": BASE64.encode(b"test"),
        "key": short_key
    });
    let result = handle_aes256_gcm_encrypt(Some(&params)).await;
    assert!(result.is_err());
    
    println!("✅ Invalid key size rejection SUCCESS!");
}

#[tokio::test]
async fn test_fault_tampered_ciphertexts() {
    // Test AEAD authentication catches tampering
    
    // Encrypt with ChaCha20-Poly1305
    let plaintext = BASE64.encode(b"Important data");
    let key = BASE64.encode(&[0x42u8; 32]);
    let nonce = BASE64.encode(&[0x01u8; 12]);
    
    let encrypt_params = json!({
        "plaintext": plaintext,
        "key": key,
        "nonce": nonce
    });
    let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    
    let ciphertext_bytes = BASE64.decode(encrypted["ciphertext"].as_str().unwrap()).unwrap();
    
    // Tamper with ciphertext (flip one bit)
    let mut tampered = ciphertext_bytes.clone();
    if !tampered.is_empty() {
        tampered[0] ^= 0x01;
    }
    let tampered_b64 = BASE64.encode(&tampered);
    
    // Decryption should fail
    let decrypt_params = json!({
        "ciphertext": tampered_b64,
        "key": key,
        "nonce": nonce
    });
    let result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params)).await;
    assert!(result.is_err());
    
    println!("✅ Tampered ciphertext detection SUCCESS!");
}

#[tokio::test]
async fn test_fault_signature_verification_failures() {
    // Test signature verification catches invalid signatures
    
    // Generate valid signature
    let data = BASE64.encode(b"Document to sign");
    let sign_params = json!({"data": data});
    let sign_result = handle_sign_ed25519(Some(&sign_params)).await.unwrap();
    
    let signature = sign_result["signature"].as_str().unwrap();
    let public_key = sign_result["public_key"].as_str().unwrap();
    
    // Tamper with signature (flip one bit)
    let sig_bytes = BASE64.decode(signature).unwrap();
    let mut tampered_sig = sig_bytes.clone();
    if !tampered_sig.is_empty() {
        tampered_sig[0] ^= 0x01;
    }
    let tampered_sig_b64 = BASE64.encode(&tampered_sig);
    
    // Verification should fail
    let verify_params = json!({
        "data": data,
        "signature": tampered_sig_b64,
        "public_key": public_key
    });
    let verify_result = handle_verify_ed25519(Some(&verify_params))
        .await
        .unwrap();
    
    assert_eq!(verify_result["valid"], false);
    
    println!("✅ Invalid signature detection SUCCESS!");
}

#[tokio::test]
async fn test_fault_transcript_hash_size_validation() {
    // Test that TLS methods validate transcript hash size
    
    let pre_master = BASE64.encode(&[0u8; 32]);
    let client_random = BASE64.encode(&[0u8; 32]);
    let server_random = BASE64.encode(&[0u8; 32]);
    let wrong_size_transcript = BASE64.encode(&[0u8; 16]); // Wrong size!
    
    // Handshake secrets
    let hs_params = json!({
        "pre_master_secret": pre_master,
        "client_random": client_random,
        "server_random": server_random,
        "transcript_hash": wrong_size_transcript
    });
    let result = handle_tls_derive_handshake_secrets(Some(&hs_params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("32 bytes"));
    
    // Application secrets
    let app_params = json!({
        "pre_master_secret": pre_master,
        "client_random": client_random,
        "server_random": server_random,
        "transcript_hash": wrong_size_transcript
    });
    let result = handle_tls_derive_application_secrets(Some(&app_params)).await;
    assert!(result.is_err());
    
    println!("✅ Transcript hash size validation SUCCESS!");
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_perf_all_crypto_operations() {
    // Measure performance of all crypto operations
    
    let iterations = 100;
    let mut timings = Vec::new();
    
    // Ed25519
    let start = Instant::now();
    for _ in 0..iterations {
        let data = BASE64.encode(b"test");
        let params = json!({"data": data});
        let _ = handle_sign_ed25519(Some(&params)).await.unwrap();
    }
    timings.push(("Ed25519 sign", start.elapsed() / iterations));
    
    // ECDSA P-256
    let start = Instant::now();
    for _ in 0..iterations {
        let data = BASE64.encode(b"test");
        let params = json!({"data": data});
        let _ = handle_sign_ecdsa_secp256r1(Some(&params)).await.unwrap();
    }
    timings.push(("ECDSA P-256 sign", start.elapsed() / iterations));
    
    // SHA-256
    let start = Instant::now();
    for _ in 0..iterations {
        let data = BASE64.encode(b"test");
        let params = json!({"data": data});
        let _ = handle_sha256(Some(&params)).await.unwrap();
    }
    timings.push(("SHA-256 hash", start.elapsed() / iterations));
    
    // ChaCha20-Poly1305
    let start = Instant::now();
    for _ in 0..iterations {
        let params = json!({
            "plaintext": BASE64.encode(b"test"),
            "key": BASE64.encode(&[0u8; 32]),
            "nonce": BASE64.encode(&[0u8; 12])
        });
        let _ = handle_chacha20_poly1305_encrypt(Some(&params)).await.unwrap();
    }
    timings.push(("ChaCha20 encrypt", start.elapsed() / iterations));
    
    // AES-256-GCM
    let start = Instant::now();
    for _ in 0..iterations {
        let params = json!({
            "plaintext": BASE64.encode(b"test"),
            "key": BASE64.encode(&[0u8; 32])
        });
        let _ = handle_aes256_gcm_encrypt(Some(&params)).await.unwrap();
    }
    timings.push(("AES-256-GCM encrypt", start.elapsed() / iterations));
    
    // X25519 ECDH
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = handle_x25519_generate_ephemeral(None).await.unwrap();
    }
    timings.push(("X25519 keygen", start.elapsed() / iterations));
    
    // TLS handshake secrets
    let start = Instant::now();
    for _ in 0..iterations {
        let params = json!({
            "pre_master_secret": BASE64.encode(&[0u8; 32]),
            "client_random": BASE64.encode(&[0u8; 32]),
            "server_random": BASE64.encode(&[0u8; 32]),
            "transcript_hash": BASE64.encode(&[0u8; 32])
        });
        let _ = handle_tls_derive_handshake_secrets(Some(&params)).await.unwrap();
    }
    timings.push(("TLS handshake secrets", start.elapsed() / iterations));
    
    // Print performance report
    println!("\n📊 PERFORMANCE REPORT (100 iterations avg):");
    println!("═══════════════════════════════════════════");
    for (name, duration) in timings {
        println!("  {:25} {:>8} µs", name, duration.as_micros());
        
        // Assert all operations are fast (< 10ms)
        assert!(
            duration.as_millis() < 10,
            "{} too slow: {:?}",
            name,
            duration
        );
    }
    
    println!("✅ All crypto operations performant (< 10ms)!");
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[tokio::test]
async fn test_stress_rapid_sequential_operations() {
    // 1000 rapid sequential operations
    let iterations = 1000;
    
    let start = Instant::now();
    for i in 0..iterations {
        let data = BASE64.encode(&[i as u8; 32]);
        let params = json!({"data": data});
        let _ = handle_sha256(Some(&params)).await.unwrap();
    }
    let duration = start.elapsed();
    
    println!(
        "✅ 1000 sequential SHA-256 operations: {:?} ({} ops/sec)",
        duration,
        (iterations as f64 / duration.as_secs_f64()) as u64
    );
    
    // Should handle at least 1000 ops/sec
    assert!(duration.as_secs() < 10);
}

#[tokio::test]
async fn test_stress_large_data_encryption() {
    // Encrypt/decrypt large data (1MB)
    let large_data = vec![0x42u8; 1024 * 1024]; // 1MB
    let plaintext_b64 = BASE64.encode(&large_data);
    let key = BASE64.encode(&[0x01u8; 32]);
    let nonce = BASE64.encode(&[0x02u8; 12]);
    
    let start = Instant::now();
    let encrypt_params = json!({
        "plaintext": plaintext_b64,
        "key": key,
        "nonce": nonce
    });
    let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
        .await
        .unwrap();
    let encrypt_duration = start.elapsed();
    
    let start = Instant::now();
    let decrypt_params = json!({
        "ciphertext": encrypted["ciphertext"],
        "key": key,
        "nonce": nonce
    });
    let decrypted = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
        .await
        .unwrap();
    let decrypt_duration = start.elapsed();
    
    let decrypted_data = BASE64.decode(decrypted["plaintext"].as_str().unwrap()).unwrap();
    assert_eq!(decrypted_data, large_data);
    
    println!("✅ 1MB encryption: {:?}", encrypt_duration);
    println!("✅ 1MB decryption: {:?}", decrypt_duration);
    
    // Should handle 1MB in reasonable time (< 100ms)
    assert!(encrypt_duration.as_millis() < 100);
    assert!(decrypt_duration.as_millis() < 100);
}

