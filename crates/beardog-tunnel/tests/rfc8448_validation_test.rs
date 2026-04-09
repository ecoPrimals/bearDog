// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]
//! RFC 8448 Validation Test
//!
//! This test validates `BearDog`'s TLS 1.3 key derivation against known values
//! from RFC 8448 Section 3 (Simple 1-RTT Handshake).
//!
//! RFC 8448: "Example Handshake Traces for TLS 1.3"
//! <https://www.rfc-editor.org/rfc/rfc8448.html>
//!
//! This provides a reference implementation test to ensure our HKDF-based
//! key derivation exactly matches the RFC specification.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::handle_tls_derive_handshake_secrets;
use serde_json::json;

/// RFC 8448 Section 3: Simple 1-RTT Handshake
///
/// This test uses the exact values from RFC 8448 to validate our implementation.
#[tokio::test]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "RFC 8448 fixture bytes parsed from spec tables; casts match reference vectors"
)]
async fn test_rfc8448_handshake_key_derivation() {
    // ========================================================================
    // RFC 8448 Known Values
    // ========================================================================

    // ClientHello (handshake message body, 196 bytes)
    let client_hello_hex = "\
        01 00 00 c0 03 03 cb 34 ec b1 e7 81 63 \
        ba 1c 38 c6 da cb 19 6a 6d ff a2 1a 8d 99 12 \
        ec 18 a2 ef 62 83 02 4d ec e7 00 00 06 13 01 \
        13 03 13 02 01 00 00 91 00 00 00 0b 00 09 00 \
        00 06 73 65 72 76 65 72 ff 01 00 01 00 00 0a \
        00 14 00 12 00 1d 00 17 00 18 00 19 01 00 01 \
        01 01 02 01 03 01 04 00 23 00 00 00 33 00 26 \
        00 24 00 1d 00 20 99 38 1d e5 60 e4 bd 43 d2 \
        3d 8e 43 5a 7d ba fe b3 c0 6e 51 c1 3c ae 4d \
        54 13 69 1e 52 9a af 2c 00 2b 00 03 02 03 04 \
        00 0d 00 20 00 1e 04 03 05 03 06 03 02 03 08 \
        04 08 05 08 06 04 01 05 01 06 01 02 01 04 02 \
        05 02 06 02 02 02 00 2d 00 02 01 01 00 1c 00 \
        02 40 01";

    // ServerHello (handshake message body, 90 bytes)
    let server_hello_hex = "\
        02 00 00 56 03 03 a6 af 06 a4 12 18 60 dc 5e \
        6e 60 24 9c d3 4c 95 93 0c 8a c5 cb 14 34 da \
        c1 55 77 2e d3 e2 69 28 00 13 01 00 00 2e 00 \
        33 00 24 00 1d 00 20 c9 82 88 76 11 20 95 fe \
        66 76 2b db f7 c6 72 e1 56 d6 cc 25 3b 83 3d \
        f1 dd 69 b1 b0 4e 75 1f 0f 00 2b 00 02 03 04";

    // ECDH Shared Secret (X25519, 32 bytes)
    let ecdh_secret_hex = "\
        8b d4 05 4f b5 5b 9d 63 fd fb ac f9 f0 4b 9f 0d \
        35 e6 d6 3f 53 75 63 ef d4 62 72 90 0f 89 49 2d";

    // Client Random (from ClientHello.random, 32 bytes)
    let client_random_hex = "\
        cb 34 ec b1 e7 81 63 ba 1c 38 c6 da cb 19 6a 6d \
        ff a2 1a 8d 99 12 ec 18 a2 ef 62 83 02 4d ec e7";

    // Server Random (from ServerHello.random, 32 bytes)
    let server_random_hex = "\
        a6 af 06 a4 12 18 60 dc 5e 6e 60 24 9c d3 4c 95 \
        93 0c 8a c5 cb 14 34 da c1 55 77 2e d3 e2 69 28";

    // Expected Transcript Hash (SHA-256 of ClientHello || ServerHello, 32 bytes)
    let expected_transcript_hash_hex = "\
        86 0c 06 ed c0 78 58 ee 8e 78 f0 e7 42 8c 58 ed \
        d6 b4 3f 2c a3 e6 e9 5f 02 ed 06 3c f0 e1 ca d8";

    // Expected Client Handshake Traffic Secret (32 bytes)
    let expected_client_secret_hex = "\
        b3 ed db 12 6e 06 7f 35 a7 80 b3 ab f4 5e 2d 8f \
        3b 1a 95 07 38 f5 2e 96 00 74 6a 0e 27 a5 5a 21";

    // Expected Server Handshake Traffic Secret (32 bytes)
    let expected_server_secret_hex = "\
        b6 7b 7d 69 0c c1 6c 4e 75 e5 42 13 cb 2d 37 b4 \
        e9 c9 12 bc de d9 10 5d 42 be fd 59 d3 91 ad 38";

    // ========================================================================
    // Parse Hex Strings
    // ========================================================================

    fn parse_hex(hex_str: &str) -> Vec<u8> {
        hex_str
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect()
    }

    let client_hello = parse_hex(client_hello_hex);
    let server_hello = parse_hex(server_hello_hex);
    let ecdh_secret = parse_hex(ecdh_secret_hex);
    let client_random = parse_hex(client_random_hex);
    let server_random = parse_hex(server_random_hex);
    let expected_transcript_hash = parse_hex(expected_transcript_hash_hex);
    let expected_client_secret = parse_hex(expected_client_secret_hex);
    let expected_server_secret = parse_hex(expected_server_secret_hex);

    // ========================================================================
    // Verify Transcript Hash
    // ========================================================================

    use sha2::{Digest, Sha256};
    let transcript = [client_hello, server_hello].concat();
    let computed_transcript_hash = Sha256::digest(&transcript);

    assert_eq!(
        computed_transcript_hash.as_slice(),
        expected_transcript_hash.as_slice(),
        "Transcript hash mismatch! Expected: {:02x?}, Got: {:02x?}",
        expected_transcript_hash,
        computed_transcript_hash.as_slice()
    );

    println!("✅ Transcript hash matches RFC 8448!");

    // ========================================================================
    // Call BearDog's Key Derivation
    // ========================================================================

    let params = json!({
        "pre_master_secret": BASE64.encode(&ecdh_secret),
        "client_random": BASE64.encode(&client_random),
        "server_random": BASE64.encode(&server_random),
        "transcript_hash": BASE64.encode(computed_transcript_hash),
        "cipher_suite": 0x1301  // TLS_AES_128_GCM_SHA256 (RFC 8446)
    });

    let result = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("Key derivation failed");

    // ========================================================================
    // Extract and Decode Keys
    // ========================================================================

    let client_write_key_b64 = result["client_write_key"]
        .as_str()
        .expect("Missing client_write_key");
    let server_write_key_b64 = result["server_write_key"]
        .as_str()
        .expect("Missing server_write_key");

    let client_write_key = BASE64
        .decode(client_write_key_b64)
        .expect("Failed to decode client_write_key");
    let server_write_key = BASE64
        .decode(server_write_key_b64)
        .expect("Failed to decode server_write_key");

    // ========================================================================
    // Derive Traffic Secrets from Keys (Reverse Engineering)
    // ========================================================================
    //
    // NOTE: RFC 8448 provides the handshake traffic secrets, but our API
    // surfaces derived keys (HKDF-Expand-Label(secret, "key", "", 32)).
    //
    // To validate, we need to derive the keys from the expected secrets
    // and compare them with our output.

    use hkdf::Hkdf;

    fn hkdf_expand_label(secret: &[u8], label: &str, context: &[u8], length: usize) -> Vec<u8> {
        let mut hkdf_label = Vec::new();
        hkdf_label.extend_from_slice(&(length as u16).to_be_bytes());

        let tls13_label = format!("tls13 {label}");
        hkdf_label.push(tls13_label.len() as u8);
        hkdf_label.extend_from_slice(tls13_label.as_bytes());

        hkdf_label.push(context.len() as u8);
        hkdf_label.extend_from_slice(context);

        let hkdf = Hkdf::<Sha256>::from_prk(secret).unwrap();
        let mut okm = vec![0u8; length];
        hkdf.expand(&hkdf_label, &mut okm).unwrap();
        okm
    }

    // Derive expected keys from RFC 8448 secrets
    // Key length is 16 for AES-128-GCM (cipher suite 0x1301)
    let expected_client_key = hkdf_expand_label(&expected_client_secret, "key", &[], 16);
    let expected_server_key = hkdf_expand_label(&expected_server_secret, "key", &[], 16);

    // ========================================================================
    // Validate Keys
    // ========================================================================

    assert_eq!(
        client_write_key, expected_client_key,
        "Client write key mismatch!\nExpected: {expected_client_key:02x?}\nGot:      {client_write_key:02x?}"
    );

    assert_eq!(
        server_write_key, expected_server_key,
        "Server write key mismatch!\nExpected: {expected_server_key:02x?}\nGot:      {server_write_key:02x?}"
    );

    println!("✅ Client write key matches RFC 8448!");
    println!("✅ Server write key matches RFC 8448!");

    // ========================================================================
    // Validate IVs
    // ========================================================================

    let client_write_iv_b64 = result["client_write_iv"]
        .as_str()
        .expect("Missing client_write_iv");
    let server_write_iv_b64 = result["server_write_iv"]
        .as_str()
        .expect("Missing server_write_iv");

    let client_write_iv = BASE64
        .decode(client_write_iv_b64)
        .expect("Failed to decode client_write_iv");
    let server_write_iv = BASE64
        .decode(server_write_iv_b64)
        .expect("Failed to decode server_write_iv");

    // Derive expected IVs from RFC 8448 secrets
    let expected_client_iv = hkdf_expand_label(&expected_client_secret, "iv", &[], 12);
    let expected_server_iv = hkdf_expand_label(&expected_server_secret, "iv", &[], 12);

    assert_eq!(
        client_write_iv, expected_client_iv,
        "Client write IV mismatch!\nExpected: {expected_client_iv:02x?}\nGot:      {client_write_iv:02x?}"
    );

    assert_eq!(
        server_write_iv, expected_server_iv,
        "Server write IV mismatch!\nExpected: {expected_server_iv:02x?}\nGot:      {server_write_iv:02x?}"
    );

    println!("✅ Client write IV matches RFC 8448!");
    println!("✅ Server write IV matches RFC 8448!");

    // ========================================================================
    // Final Validation
    // ========================================================================

    println!("\n🎉 RFC 8448 VALIDATION COMPLETE!");
    println!("   ✅ Transcript hash: CORRECT");
    println!("   ✅ Client handshake key: CORRECT");
    println!("   ✅ Server handshake key: CORRECT");
    println!("   ✅ Client handshake IV: CORRECT");
    println!("   ✅ Server handshake IV: CORRECT");
    println!("\n🦀 BearDog's TLS 1.3 key derivation is RFC 8448 compliant! ✨");
}

/// Test with RFC 8448 values via base64 (for easy copy-paste testing)
#[tokio::test]
async fn test_rfc8448_base64_inputs() {
    // These are the RFC 8448 values encoded in base64 for easy RPC testing

    let ecdh_secret_b64 = "i9QFT7Vbnf39uyz5T7kNNeY2P1N1Y+/UYnKQD4lJLQ==";
    let client_random_b64 = "yzTsseeBY7ocOMbcyxlqbf+iGo2ZEuwYou9iggLTeuc=";
    let server_random_b64 = "pq8GpBIYYNxeblAkmM00yZMwyKxcsUDawVV3LtPeaSg=";
    let transcript_hash_b64 = "hgwG7cB4WO6OePDnQoxY7da0PyyWO656XwLtBjzw4dg=";

    let params = json!({
        "pre_master_secret": ecdh_secret_b64,
        "client_random": client_random_b64,
        "server_random": server_random_b64,
        "transcript_hash": transcript_hash_b64,
        "cipher_suite": 0x1301  // TLS_AES_128_GCM_SHA256 (RFC 8446)
    });

    let result = handle_tls_derive_handshake_secrets(Some(&params))
        .await
        .expect("Key derivation failed");

    // Just verify it succeeds and returns expected structure
    assert!(result["client_write_key"].is_string());
    assert!(result["client_write_iv"].is_string());
    assert!(result["server_write_key"].is_string());
    assert!(result["server_write_iv"].is_string());

    println!("✅ RFC 8448 base64 test passed!");
    println!("   Client key: {}", result["client_write_key"]);
    println!("   Server key: {}", result["server_write_key"]);
}

/// Test transcript hash computation (helper for debugging)
#[test]
fn test_transcript_hash_computation() {
    use sha2::{Digest, Sha256};

    // Example: Simple ClientHello + ServerHello
    let client_hello = vec![
        0x01, 0x00, 0x00, 0x05, // Handshake type (ClientHello) + length
        0x03, 0x03, // Version (TLS 1.2 compat)
        0xAA, 0xBB, 0xCC, // Random (partial, for demo)
    ];

    let server_hello = vec![
        0x02, 0x00, 0x00, 0x05, // Handshake type (ServerHello) + length
        0x03, 0x03, // Version
        0xDD, 0xEE, 0xFF, // Random (partial)
    ];

    let transcript = [client_hello, server_hello].concat();
    let transcript_hash = Sha256::digest(&transcript);

    println!(
        "Transcript ({} bytes): {:02x?}",
        transcript.len(),
        transcript
    );
    println!(
        "Transcript hash ({} bytes): {:02x?}",
        transcript_hash.len(),
        transcript_hash.as_slice()
    );

    assert_eq!(transcript_hash.len(), 32, "SHA-256 should produce 32 bytes");
}

/// Test that TLS record headers should NOT be in transcript
#[test]
fn test_transcript_without_record_headers() {
    use sha2::{Digest, Sha256};

    // WRONG: Including TLS record header
    let with_header = vec![
        0x16, 0x03, 0x03, 0x00, 0x05, // TLS record header (5 bytes)
        0x01, 0x00, 0x00, 0x01, 0xAA, // Handshake message (5 bytes)
    ];

    // CORRECT: Only handshake message
    let without_header = vec![
        0x01, 0x00, 0x00, 0x01, 0xAA, // Handshake message (5 bytes)
    ];

    let hash_with = Sha256::digest(&with_header);
    let hash_without = Sha256::digest(&without_header);

    println!("Hash with TLS header:    {:02x?}", hash_with.as_slice());
    println!("Hash without TLS header: {:02x?}", hash_without.as_slice());

    // These should be DIFFERENT!
    assert_ne!(
        hash_with.as_slice(),
        hash_without.as_slice(),
        "Including TLS record header changes the transcript hash!"
    );

    println!("\n⚠️  IMPORTANT: TLS record headers MUST NOT be included in transcript!");
    println!("   Transcript = Handshake messages ONLY (no [16 03 03 LL LL] headers)");
}
