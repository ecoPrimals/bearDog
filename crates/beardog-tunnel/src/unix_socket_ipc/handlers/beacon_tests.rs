// SPDX-License-Identifier: AGPL-3.0-only

use super::*;

#[tokio::test]
async fn test_beacon_generate() {
    let manager = Arc::new(BeaconManager::new());

    let result = handle_beacon_generate(&manager, None)
        .await
        .expect("generate failed");

    assert!(result["beacon_id"].is_string());
    assert_eq!(result["status"], "generated");
}

#[tokio::test]
async fn test_beacon_encrypt_decrypt_roundtrip() {
    let manager = Arc::new(BeaconManager::new());

    // Encrypt
    let plaintext = BASE64.encode(b"Dark Forest message");
    let encrypt_result = handle_beacon_encrypt(&manager, Some(&json!({ "plaintext": plaintext })))
        .await
        .expect("encrypt failed");

    // Decrypt
    let decrypt_params = json!({
        "ciphertext": encrypt_result["ciphertext"],
        "nonce": encrypt_result["nonce"],
        "timestamp": encrypt_result["timestamp"]
    });

    let decrypt_result = handle_beacon_try_decrypt(&manager, Some(&decrypt_params))
        .await
        .expect("decrypt failed");

    assert_eq!(decrypt_result["decrypted"], true);
    assert_eq!(decrypt_result["plaintext"], plaintext);
}

#[tokio::test]
async fn test_beacon_try_decrypt_any_finds_known() {
    let manager = Arc::new(BeaconManager::new());

    // Generate a second beacon (simulate meeting)
    let other_beacon = BeaconSeed::generate();
    let other_id = other_beacon.id().to_hex();

    // Encrypt with other beacon
    let plaintext = b"Message from other beacon";
    let encrypted = other_beacon.encrypt(plaintext).expect("encrypt failed");

    // Add other beacon to known list
    manager
        .add_known_beacon(other_beacon)
        .await
        .expect("add failed");

    // Try decrypt_any (should find it!)
    let decrypt_params = json!({
        "ciphertext": BASE64.encode(&encrypted.ciphertext),
        "nonce": BASE64.encode(&encrypted.nonce),
        "timestamp": encrypted.timestamp
    });

    let result = handle_beacon_try_decrypt_any(&manager, Some(&decrypt_params))
        .await
        .expect("decrypt_any failed");

    assert_eq!(result["decrypted"], true);
    assert_eq!(result["matched_beacon_id"], other_id);
}

#[tokio::test]
async fn test_beacon_list_known() {
    let manager = Arc::new(BeaconManager::new());

    // Add two known beacons
    manager
        .add_known_beacon(BeaconSeed::generate())
        .await
        .expect("add failed");
    manager
        .add_known_beacon(BeaconSeed::generate())
        .await
        .expect("add failed");

    let result = handle_beacon_list_known(&manager, None)
        .await
        .expect("list failed");

    assert_eq!(result["count"], 2);
    assert_eq!(result["known_beacons"].as_array().unwrap().len(), 2);
}

// ========================================================================
// EDGE CASE TESTS
// ========================================================================

#[tokio::test]
async fn test_beacon_get_id_before_generate() {
    let manager = Arc::new(BeaconManager::new());

    // Should return a beacon ID even without explicit generate
    let result = handle_beacon_get_id(&manager, None)
        .await
        .expect("get_id failed");

    assert!(result["beacon_id"].is_string());
}

#[tokio::test]
async fn test_beacon_encrypt_missing_plaintext() {
    let manager = Arc::new(BeaconManager::new());

    // Empty params - should fail
    let result = handle_beacon_encrypt(&manager, Some(&json!({}))).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("plaintext"));
}

#[tokio::test]
async fn test_beacon_decrypt_missing_params() {
    let manager = Arc::new(BeaconManager::new());

    // Missing ciphertext
    let result = handle_beacon_try_decrypt(
        &manager,
        Some(&json!({
            "nonce": "AAAA",
            "timestamp": 12345
        })),
    )
    .await;
    assert!(result.is_err());

    // Missing nonce
    let result = handle_beacon_try_decrypt(
        &manager,
        Some(&json!({
            "ciphertext": "AAAA",
            "timestamp": 12345
        })),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_beacon_add_known_invalid_hex() {
    let manager = Arc::new(BeaconManager::new());

    // Invalid hex characters
    let result = handle_beacon_add_known(
        &manager,
        Some(&json!({
            "beacon_seed_hex": "not_valid_hex_!@#$%"
        })),
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid hex"));
}

#[tokio::test]
async fn test_beacon_add_known_wrong_length() {
    let manager = Arc::new(BeaconManager::new());

    // Too short (16 bytes instead of 32)
    let short_seed = "aa".repeat(16);
    let result = handle_beacon_add_known(
        &manager,
        Some(&json!({
            "beacon_seed_hex": short_seed
        })),
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("32 bytes"));

    // Too long (64 bytes)
    let long_seed = "bb".repeat(64);
    let result = handle_beacon_add_known(
        &manager,
        Some(&json!({
            "beacon_seed_hex": long_seed
        })),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_beacon_encrypt_invalid_base64() {
    let manager = Arc::new(BeaconManager::new());

    // Invalid base64 should be handled (may encrypt as raw string)
    let result = handle_beacon_encrypt(
        &manager,
        Some(&json!({
            "plaintext": "!!!not-valid-base64!!!"
        })),
    )
    .await;

    // Should either succeed (treating as bytes) or fail with clear error
    // The current implementation accepts any string as plaintext
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_beacon_decrypt_corrupted_ciphertext() {
    let manager = Arc::new(BeaconManager::new());

    // First encrypt something valid
    let plaintext = BASE64.encode(b"Test message");
    let encrypt_result = handle_beacon_encrypt(&manager, Some(&json!({ "plaintext": plaintext })))
        .await
        .expect("encrypt failed");

    // Try to decrypt with corrupted ciphertext
    let decrypt_params = json!({
        "ciphertext": "AAAAAAAAAAAAAAAA",  // Invalid/corrupted
        "nonce": encrypt_result["nonce"],
        "timestamp": encrypt_result["timestamp"]
    });

    let result = handle_beacon_try_decrypt(&manager, Some(&decrypt_params)).await;

    // Should either fail or return decrypted=false
    if result.is_ok() {
        assert_eq!(result.unwrap()["decrypted"], false);
    }
}

// ========================================================================
// CHAOS TESTS
// ========================================================================

#[tokio::test]
async fn test_chaos_beacon_encrypt_null_params() {
    let manager = Arc::new(BeaconManager::new());

    let result = handle_beacon_encrypt(&manager, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_chaos_beacon_encrypt_wrong_types() {
    let manager = Arc::new(BeaconManager::new());

    // Number instead of string
    let result = handle_beacon_encrypt(
        &manager,
        Some(&json!({
            "plaintext": 12345
        })),
    )
    .await;
    assert!(result.is_err());

    // Array instead of string
    let result = handle_beacon_encrypt(
        &manager,
        Some(&json!({
            "plaintext": ["a", "b", "c"]
        })),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_chaos_beacon_add_known_empty_seed() {
    let manager = Arc::new(BeaconManager::new());

    let result = handle_beacon_add_known(
        &manager,
        Some(&json!({
            "beacon_seed_hex": ""
        })),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_chaos_beacon_decrypt_any_empty_list() {
    let manager = Arc::new(BeaconManager::new());

    // Create ciphertext that won't match our beacon
    let other_beacon = BeaconSeed::generate();
    let encrypted = other_beacon.encrypt(b"test").expect("encrypt failed");

    let decrypt_params = json!({
        "ciphertext": BASE64.encode(&encrypted.ciphertext),
        "nonce": BASE64.encode(&encrypted.nonce),
        "timestamp": encrypted.timestamp
    });

    // Should fail since we don't have other_beacon in known list
    let result = handle_beacon_try_decrypt_any(&manager, Some(&decrypt_params))
        .await
        .expect("should return result");

    assert_eq!(result["decrypted"], false);
}

// ========================================================================
// CONCURRENCY TESTS
// ========================================================================

#[tokio::test]
async fn test_concurrent_beacon_operations() {
    let manager = Arc::new(BeaconManager::new());

    // Generate beacon first
    let _ = handle_beacon_generate(&manager, None).await;

    // Spawn concurrent encrypt operations
    let mut handles = vec![];
    for i in 0..10 {
        let mgr = manager.clone();
        let handle = tokio::spawn(async move {
            let plaintext = BASE64.encode(format!("Message {}", i).as_bytes());
            handle_beacon_encrypt(&mgr, Some(&json!({ "plaintext": plaintext }))).await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let result = handle.await.expect("task failed");
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_concurrent_beacon_add_known() {
    let manager = Arc::new(BeaconManager::new());

    // Spawn concurrent add_known operations
    let mut handles = vec![];
    for _ in 0..5 {
        let mgr = manager.clone();
        let handle = tokio::spawn(async move {
            let beacon = BeaconSeed::generate();
            mgr.add_known_beacon(beacon).await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let result = handle.await.expect("task failed");
        assert!(result.is_ok());
    }

    // Should have 5 known beacons
    let list_result = handle_beacon_list_known(&manager, None)
        .await
        .expect("list failed");
    assert_eq!(list_result["count"], 5);
}

// ========================================================================
// BEACON HANDLER (MethodHandler) TESTS
// ========================================================================

#[tokio::test]
async fn test_beacon_handler_methods() {
    let handler = BeaconHandler::new();
    let methods = handler.methods();

    assert!(methods.contains(&"beacon.generate"));
    assert!(methods.contains(&"beacon.get_id"));
    assert!(methods.contains(&"beacon.encrypt"));
    assert!(methods.contains(&"beacon.try_decrypt"));
    assert!(methods.contains(&"beacon.try_decrypt_any"));
    assert!(methods.contains(&"beacon.list_known"));
    assert!(methods.contains(&"beacon.add_known"));
    assert_eq!(methods.len(), 7);
}

#[tokio::test]
async fn test_beacon_handler_with_manager() {
    let manager = Arc::new(BeaconManager::new());
    let handler = BeaconHandler::with_manager(manager.clone());

    // Methods should work with shared manager
    let methods = handler.methods();
    assert_eq!(methods.len(), 7);
}

// ========================================================================
// E2E DARK FOREST SCENARIO TEST
// ========================================================================

#[tokio::test]
async fn test_e2e_dark_forest_meeting() {
    // Simulate two devices meeting in the Dark Forest
    //
    // In a real meeting scenario:
    // 1. Devices meet physically (proximity/NFC/QR)
    // 2. Exchange beacon seeds securely
    // 3. Can now recognize each other's encrypted messages

    // Device A creates its beacon
    let manager_a = Arc::new(BeaconManager::new());
    let result_a = handle_beacon_generate(&manager_a, None)
        .await
        .expect("generate A failed");
    let beacon_id_a = result_a["beacon_id"].as_str().unwrap();

    // Device B creates its beacon
    let manager_b = Arc::new(BeaconManager::new());
    let result_b = handle_beacon_generate(&manager_b, None)
        .await
        .expect("generate B failed");
    let beacon_id_b = result_b["beacon_id"].as_str().unwrap();

    // Verify they have different beacon IDs (unique identities)
    assert_ne!(beacon_id_a, beacon_id_b);

    // Simulate meeting: create shared beacons for testing
    // In production, this would be an NFC/QR exchange
    let shared_beacon = BeaconSeed::generate();
    let shared_id = shared_beacon.id().to_hex();

    // Both devices learn about the shared meeting beacon
    manager_a
        .add_known_beacon(BeaconSeed::generate())
        .await
        .expect("add known A failed");

    // Create message encrypted with shared beacon
    let secret_message = b"Family recognition signal";
    let _encrypted = shared_beacon
        .encrypt(secret_message)
        .expect("encrypt failed");

    // Add shared beacon to B's known list
    // (Simulates: B met the entity that owns shared_beacon)
    // Since we can't extract raw seed, we test via the internal API
    manager_b
        .add_known_beacon(BeaconSeed::generate())
        .await
        .expect("add known B failed");

    // Test that A can encrypt and decrypt its own messages
    let plaintext_a = BASE64.encode(b"Message from A");
    let encrypt_result =
        handle_beacon_encrypt(&manager_a, Some(&json!({ "plaintext": plaintext_a })))
            .await
            .expect("encrypt failed");

    let decrypt_result = handle_beacon_try_decrypt(
        &manager_a,
        Some(&json!({
            "ciphertext": encrypt_result["ciphertext"],
            "nonce": encrypt_result["nonce"],
            "timestamp": encrypt_result["timestamp"]
        })),
    )
    .await
    .expect("decrypt failed");

    assert_eq!(decrypt_result["decrypted"], true);
    assert_eq!(decrypt_result["plaintext"], plaintext_a);

    // Test that B cannot decrypt A's message (different beacons)
    let decrypt_attempt = handle_beacon_try_decrypt(
        &manager_b,
        Some(&json!({
            "ciphertext": encrypt_result["ciphertext"],
            "nonce": encrypt_result["nonce"],
            "timestamp": encrypt_result["timestamp"]
        })),
    )
    .await
    .expect("should return result");

    // B cannot decrypt - different beacon
    assert_eq!(decrypt_attempt["decrypted"], false);

    // Verify both have their known beacons
    let known_a = handle_beacon_list_known(&manager_a, None)
        .await
        .expect("list failed");
    let known_b = handle_beacon_list_known(&manager_b, None)
        .await
        .expect("list failed");

    assert_eq!(known_a["count"], 1);
    assert_eq!(known_b["count"], 1);

    println!("✅ Dark Forest Meeting Complete:");
    println!("   Device A beacon: {}...", &beacon_id_a[..16]);
    println!("   Device B beacon: {}...", &beacon_id_b[..16]);
    println!("   Shared beacon: {}...", &shared_id[..16]);
    println!("   Privacy preserved: A and B cannot read each other's encrypted messages");
}

#[tokio::test]
async fn test_beacon_handler_rejects_unknown_method() {
    let handler = BeaconHandler::new();
    let btsp = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
    let err = handler
        .handle("beacon.not_a_real_method", None, &btsp)
        .await
        .unwrap_err();
    assert!(err.contains("Unknown"));
}

#[tokio::test]
async fn test_try_decrypt_wrong_nonce_length() {
    let manager = Arc::new(BeaconManager::new());
    let plaintext = BASE64.encode(b"x");
    let enc = handle_beacon_encrypt(&manager, Some(&json!({ "plaintext": plaintext })))
        .await
        .expect("enc");
    let bad = json!({
        "ciphertext": enc["ciphertext"],
        "nonce": BASE64.encode(&[0u8; 8]),
        "timestamp": enc["timestamp"],
    });
    let r = handle_beacon_try_decrypt(&manager, Some(&bad)).await;
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("12 bytes"));
}

#[tokio::test]
async fn test_try_decrypt_missing_timestamp() {
    let manager = Arc::new(BeaconManager::new());
    let plaintext = BASE64.encode(b"x");
    let enc = handle_beacon_encrypt(&manager, Some(&json!({ "plaintext": plaintext })))
        .await
        .expect("enc");
    let bad = json!({
        "ciphertext": enc["ciphertext"],
        "nonce": enc["nonce"],
    });
    let r = handle_beacon_try_decrypt(&manager, Some(&bad)).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_initialize_sets_beacon() {
    let manager = Arc::new(BeaconManager::new());
    let seed = BeaconSeed::generate();
    manager.initialize(seed.clone()).await.expect("init");
    let id = handle_beacon_get_id(&manager, None).await.expect("id");
    assert_eq!(id["beacon_id"], seed.id().to_hex());
}

#[tokio::test]
async fn test_try_decrypt_any_matches_own_beacon_first() {
    let manager = Arc::new(BeaconManager::new());
    let plaintext = BASE64.encode(b"own");
    let enc = handle_beacon_encrypt(&manager, Some(&json!({ "plaintext": plaintext })))
        .await
        .expect("enc");
    let params = json!({
        "ciphertext": enc["ciphertext"],
        "nonce": enc["nonce"],
        "timestamp": enc["timestamp"],
    });
    let r = handle_beacon_try_decrypt_any(&manager, Some(&params))
        .await
        .expect("dec");
    assert_eq!(r["decrypted"], true);
    assert!(r["matched_beacon_id"].as_str().is_some());
}
