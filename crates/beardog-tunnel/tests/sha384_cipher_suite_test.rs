//! SHA-384 Cipher Suite Test (0x1302)
//!
//! Smoke test to verify TLS 1.3 cipher suite 0x1302 (TLS_AES_256_GCM_SHA384)
//! works correctly with SHA-384 HKDF.
//!
//! This test validates the SHA-384 evolution that enables 100% TLS validation.

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::{
    handle_hash_for_cipher, handle_tls_derive_application_secrets,
    handle_tls_derive_handshake_secrets,
};
use serde_json::json;

/// Test crypto.hash_for_cipher with all 3 TLS 1.3 cipher suites
#[tokio::test]
async fn test_hash_for_cipher_all_suites() {
    let test_data = b"Hello, TLS 1.3!";
    let data_b64 = BASE64.encode(test_data);

    // Test 0x1301 (TLS_AES_128_GCM_SHA256) - SHA-256
    let result_1301 = handle_hash_for_cipher(Some(&json!({
        "data": data_b64,
        "cipher_suite": 0x1301
    })))
    .await
    .expect("hash_for_cipher failed for 0x1301");

    assert_eq!(result_1301["algorithm"], "SHA-256");
    assert_eq!(result_1301["hash_length"], 32);
    assert_eq!(result_1301["cipher_suite"], 0x1301);
    let hash_1301 = BASE64
        .decode(result_1301["hash"].as_str().unwrap())
        .unwrap();
    assert_eq!(hash_1301.len(), 32);

    // Test 0x1302 (TLS_AES_256_GCM_SHA384) - SHA-384
    let result_1302 = handle_hash_for_cipher(Some(&json!({
        "data": data_b64,
        "cipher_suite": 0x1302
    })))
    .await
    .expect("hash_for_cipher failed for 0x1302");

    assert_eq!(result_1302["algorithm"], "SHA-384");
    assert_eq!(result_1302["hash_length"], 48);
    assert_eq!(result_1302["cipher_suite"], 0x1302);
    let hash_1302 = BASE64
        .decode(result_1302["hash"].as_str().unwrap())
        .unwrap();
    assert_eq!(hash_1302.len(), 48);

    // Test 0x1303 (TLS_CHACHA20_POLY1305_SHA256) - SHA-256
    let result_1303 = handle_hash_for_cipher(Some(&json!({
        "data": data_b64,
        "cipher_suite": 0x1303
    })))
    .await
    .expect("hash_for_cipher failed for 0x1303");

    assert_eq!(result_1303["algorithm"], "SHA-256");
    assert_eq!(result_1303["hash_length"], 32);
    assert_eq!(result_1303["cipher_suite"], 0x1303);

    // Verify SHA-256 hashes are identical for 0x1301 and 0x1303
    assert_eq!(
        hash_1301,
        BASE64
            .decode(result_1303["hash"].as_str().unwrap())
            .unwrap()
    );

    // Verify SHA-384 hash is different from SHA-256
    assert_ne!(hash_1301.len(), hash_1302.len());

    println!("✅ hash_for_cipher works for all 3 cipher suites!");
}

/// Test handshake secret derivation with SHA-384 (cipher suite 0x1302)
#[tokio::test]
async fn test_handshake_secrets_sha384() {
    // Test inputs (32-byte ECDH secret, 32-byte randoms, 48-byte transcript for SHA-384)
    let pre_master_secret = vec![42u8; 32];
    let client_random = vec![1u8; 32];
    let server_random = vec![2u8; 32];

    // SHA-384 produces 48-byte hashes
    let transcript_hash = vec![0xAAu8; 48];

    let params = json!({
        "pre_master_secret": BASE64.encode(&pre_master_secret),
        "client_random": BASE64.encode(&client_random),
        "server_random": BASE64.encode(&server_random),
        "transcript_hash": BASE64.encode(&transcript_hash),
        "cipher_suite": 0x1302  // TLS_AES_256_GCM_SHA384
    });

    let result = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("Handshake secret derivation failed for SHA-384");

    // Verify response structure
    assert!(result["client_write_key"].is_string());
    assert!(result["server_write_key"].is_string());
    assert!(result["client_write_iv"].is_string());
    assert!(result["server_write_iv"].is_string());
    assert!(result["client_handshake_secret"].is_string());
    assert!(result["server_handshake_secret"].is_string());
    assert!(result["handshake_secret"].is_string());

    // Verify algorithm and hash info
    assert_eq!(result["algorithm"], "HKDF-SHA-384");
    assert_eq!(result["hash_algorithm"], "SHA-384");
    assert_eq!(result["hash_length"], 48);
    assert_eq!(result["key_length"], 32); // AES-256-GCM uses 32-byte keys
    assert_eq!(result["cipher_suite"], 0x1302);

    // Decode and verify key sizes
    let client_write_key = BASE64
        .decode(result["client_write_key"].as_str().unwrap())
        .unwrap();
    let server_write_key = BASE64
        .decode(result["server_write_key"].as_str().unwrap())
        .unwrap();
    let client_write_iv = BASE64
        .decode(result["client_write_iv"].as_str().unwrap())
        .unwrap();
    let server_write_iv = BASE64
        .decode(result["server_write_iv"].as_str().unwrap())
        .unwrap();

    // AES-256-GCM: 32-byte keys, 12-byte IVs
    assert_eq!(client_write_key.len(), 32);
    assert_eq!(server_write_key.len(), 32);
    assert_eq!(client_write_iv.len(), 12);
    assert_eq!(server_write_iv.len(), 12);

    // Keys should be different
    assert_ne!(client_write_key, server_write_key);
    assert_ne!(client_write_iv, server_write_iv);

    // Handshake secrets should be 48 bytes (SHA-384 output size)
    let client_handshake_secret = BASE64
        .decode(result["client_handshake_secret"].as_str().unwrap())
        .unwrap();
    let server_handshake_secret = BASE64
        .decode(result["server_handshake_secret"].as_str().unwrap())
        .unwrap();
    let handshake_secret = BASE64
        .decode(result["handshake_secret"].as_str().unwrap())
        .unwrap();

    assert_eq!(client_handshake_secret.len(), 48);
    assert_eq!(server_handshake_secret.len(), 48);
    assert_eq!(handshake_secret.len(), 48);

    println!("✅ SHA-384 handshake secret derivation works!");
}

/// Test application secret derivation with SHA-384 (cipher suite 0x1302)
#[tokio::test]
async fn test_application_secrets_sha384() {
    // Test inputs (48-byte handshake secret, 48-byte transcript for SHA-384)
    let handshake_secret = vec![0x42u8; 48]; // From handshake stage
    let transcript_hash = vec![0xAAu8; 48]; // SHA-384 of all handshake messages

    let params = json!({
        "handshake_secret": BASE64.encode(&handshake_secret),
        "transcript_hash": BASE64.encode(&transcript_hash),
        "cipher_suite": 0x1302  // TLS_AES_256_GCM_SHA384
    });

    let result = handle_tls_derive_application_secrets(Some(&params))
        .await
        .expect("Application secret derivation failed for SHA-384");

    // Verify response structure
    assert!(result["client_write_key"].is_string());
    assert!(result["server_write_key"].is_string());
    assert!(result["client_write_iv"].is_string());
    assert!(result["server_write_iv"].is_string());
    assert!(result["client_application_secret"].is_string());
    assert!(result["server_application_secret"].is_string());

    // Verify algorithm and hash info
    assert_eq!(result["algorithm"], "HKDF-SHA-384");
    assert_eq!(result["hash_algorithm"], "SHA-384");
    assert_eq!(result["hash_length"], 48);
    assert_eq!(result["key_length"], 32); // AES-256-GCM uses 32-byte keys
    assert_eq!(result["cipher_suite"], 0x1302);

    // Decode and verify key sizes
    let client_write_key = BASE64
        .decode(result["client_write_key"].as_str().unwrap())
        .unwrap();
    let server_write_key = BASE64
        .decode(result["server_write_key"].as_str().unwrap())
        .unwrap();
    let client_write_iv = BASE64
        .decode(result["client_write_iv"].as_str().unwrap())
        .unwrap();
    let server_write_iv = BASE64
        .decode(result["server_write_iv"].as_str().unwrap())
        .unwrap();

    // AES-256-GCM: 32-byte keys, 12-byte IVs
    assert_eq!(client_write_key.len(), 32);
    assert_eq!(server_write_key.len(), 32);
    assert_eq!(client_write_iv.len(), 12);
    assert_eq!(server_write_iv.len(), 12);

    // Keys should be different
    assert_ne!(client_write_key, server_write_key);
    assert_ne!(client_write_iv, server_write_iv);

    // Application secrets should be 48 bytes (SHA-384 output size)
    let client_app_secret = BASE64
        .decode(result["client_application_secret"].as_str().unwrap())
        .unwrap();
    let server_app_secret = BASE64
        .decode(result["server_application_secret"].as_str().unwrap())
        .unwrap();

    assert_eq!(client_app_secret.len(), 48);
    assert_eq!(server_app_secret.len(), 48);
    assert_ne!(client_app_secret, server_app_secret);

    println!("✅ SHA-384 application secret derivation works!");
}

/// Test full TLS 1.3 key schedule with SHA-384
#[tokio::test]
async fn test_e2e_tls_sha384_key_schedule() {
    // Simulate full TLS 1.3 key schedule for cipher suite 0x1302

    // Step 1: Handshake secrets
    let pre_master_secret = vec![42u8; 32];
    let client_random = vec![1u8; 32];
    let server_random = vec![2u8; 32];
    let handshake_transcript = vec![0xAAu8; 48]; // SHA-384

    let handshake_params = json!({
        "pre_master_secret": BASE64.encode(&pre_master_secret),
        "client_random": BASE64.encode(&client_random),
        "server_random": BASE64.encode(&server_random),
        "transcript_hash": BASE64.encode(&handshake_transcript),
        "cipher_suite": 0x1302
    });

    let handshake_result = handle_tls_derive_handshake_secrets(Some(&handshake_params))
        .await
        .expect("Handshake derivation failed");

    // Step 2: Application secrets (using handshake_secret from step 1)
    let handshake_secret = handshake_result["handshake_secret"].as_str().unwrap();
    let full_transcript = vec![0xBBu8; 48]; // SHA-384 of all messages

    let app_params = json!({
        "handshake_secret": handshake_secret,
        "transcript_hash": BASE64.encode(&full_transcript),
        "cipher_suite": 0x1302
    });

    let app_result = handle_tls_derive_application_secrets(Some(&app_params))
        .await
        .expect("Application derivation failed");

    // Verify end-to-end key schedule worked
    assert_eq!(handshake_result["hash_algorithm"], "SHA-384");
    assert_eq!(app_result["hash_algorithm"], "SHA-384");

    // Verify we got different keys at each stage
    let handshake_client_key = handshake_result["client_write_key"].as_str().unwrap();
    let app_client_key = app_result["client_write_key"].as_str().unwrap();
    assert_ne!(handshake_client_key, app_client_key);

    println!("✅ Full TLS 1.3 key schedule with SHA-384 works end-to-end!");
}

/// Test that SHA-384 keys differ from SHA-256 keys for same inputs
#[tokio::test]
async fn test_sha256_vs_sha384_produces_different_keys() {
    let pre_master_secret = vec![42u8; 32];
    let client_random = vec![1u8; 32];
    let server_random = vec![2u8; 32];

    // SHA-256 transcript (32 bytes)
    let transcript_sha256 = vec![0xAAu8; 32];

    // SHA-384 transcript (48 bytes)
    let transcript_sha384 = vec![0xAAu8; 48];

    // Derive with SHA-256 (cipher 0x1301)
    let params_sha256 = json!({
        "pre_master_secret": BASE64.encode(&pre_master_secret),
        "client_random": BASE64.encode(&client_random),
        "server_random": BASE64.encode(&server_random),
        "transcript_hash": BASE64.encode(&transcript_sha256),
        "cipher_suite": 0x1301
    });

    let result_sha256 = handle_tls_derive_handshake_secrets(Some(&params_sha256))
        .await
        .expect("SHA-256 derivation failed");

    // Derive with SHA-384 (cipher 0x1302)
    let params_sha384 = json!({
        "pre_master_secret": BASE64.encode(&pre_master_secret),
        "client_random": BASE64.encode(&client_random),
        "server_random": BASE64.encode(&server_random),
        "transcript_hash": BASE64.encode(&transcript_sha384),
        "cipher_suite": 0x1302
    });

    let result_sha384 = handle_tls_derive_handshake_secrets(Some(&params_sha384))
        .await
        .expect("SHA-384 derivation failed");

    // Keys should be different due to different hash algorithms
    assert_ne!(
        result_sha256["client_write_key"],
        result_sha384["client_write_key"]
    );
    assert_ne!(
        result_sha256["server_write_key"],
        result_sha384["server_write_key"]
    );

    // Verify algorithms are different
    assert_eq!(result_sha256["hash_algorithm"], "SHA-256");
    assert_eq!(result_sha384["hash_algorithm"], "SHA-384");

    // Verify hash lengths are different
    assert_eq!(result_sha256["hash_length"], 32);
    assert_eq!(result_sha384["hash_length"], 48);

    println!("✅ SHA-256 and SHA-384 produce cryptographically independent keys!");
}
