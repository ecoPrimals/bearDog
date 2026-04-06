// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::json;

#[test]
fn test_sha256_empty_string() {
    let params = json!({
        "data": BASE64.encode(b"")
    });

    let result = handle_sha256(&params).expect("SHA-256 handler should succeed for empty input");
    let hash = result
        .get("hash")
        .expect("SHA-256 response should contain 'hash'")
        .as_str()
        .expect("SHA-256 hash field should be a JSON string");

    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_sha256_hello_world() {
    let params = json!({
        "data": BASE64.encode(b"Hello, World!")
    });

    let result = handle_sha256(&params).expect("SHA-256 handler should succeed for hello world");
    let hash = result
        .get("hash")
        .expect("SHA-256 response should contain 'hash'")
        .as_str()
        .expect("SHA-256 hash field should be a JSON string");

    assert_eq!(
        hash,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
    assert_eq!(
        result
            .get("algorithm")
            .expect("SHA-256 response should contain 'algorithm'")
            .as_str()
            .expect("'algorithm' should be a string"),
        "sha256"
    );
    assert_eq!(
        result
            .get("output_bits")
            .expect("SHA-256 response should contain 'output_bits'")
            .as_u64()
            .expect("'output_bits' should be a JSON number"),
        256
    );
}

#[test]
fn test_sha256_bitcoin_genesis() {
    let message = b"The Times 03/Jan/2009 Chancellor";
    let params = json!({
        "data": BASE64.encode(message)
    });

    let result =
        handle_sha256(&params).expect("SHA-256 handler should succeed for genesis message");
    let hash_hex = result
        .get("hash")
        .expect("SHA-256 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a string");
    let hash_b64 = result
        .get("hash_base64")
        .expect("SHA-256 response should contain 'hash_base64'")
        .as_str()
        .expect("'hash_base64' should be a string");

    assert_eq!(hash_hex.len(), 64);
    assert!(!hash_b64.is_empty());

    let decoded_b64 = BASE64
        .decode(hash_b64)
        .expect("handler-produced hash_base64 should decode as valid base64");
    assert_eq!(hex::encode(&decoded_b64), hash_hex);
}

#[test]
fn test_sha384_empty_string() {
    let params = json!({
        "data": BASE64.encode(b"")
    });

    let result = handle_sha384(&params).expect("SHA-384 handler should succeed for empty input");
    let hash = result
        .get("hash")
        .expect("SHA-384 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(
        hash,
        "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"
    );
}

#[test]
fn test_sha384_hello_world() {
    let params = json!({
        "data": BASE64.encode(b"Hello, World!")
    });

    let result = handle_sha384(&params).expect("SHA-384 handler should succeed for hello world");
    let hash = result
        .get("hash")
        .expect("SHA-384 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(
        hash,
        "5485cc9b3365b4305dfb4e8337e0a598a574f8242bf17289e0dd6c20a3cd44a089de16ab4ab308f63e44b1170eb5f515"
    );
    assert_eq!(
        result
            .get("algorithm")
            .expect("SHA-384 response should contain 'algorithm'")
            .as_str()
            .expect("'algorithm' should be a string"),
        "sha384"
    );
    assert_eq!(
        result
            .get("output_bits")
            .expect("SHA-384 response should contain 'output_bits'")
            .as_u64()
            .expect("'output_bits' should be a JSON number"),
        384
    );
}

#[test]
fn test_sha512_empty_string() {
    let params = json!({
        "data": BASE64.encode(b"")
    });

    let result = handle_sha512(&params).expect("SHA-512 handler should succeed for empty input");
    let hash = result
        .get("hash")
        .expect("SHA-512 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(
        hash,
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
    );
}

#[test]
fn test_sha512_hello_world() {
    let params = json!({
        "data": BASE64.encode(b"Hello, World!")
    });

    let result = handle_sha512(&params).expect("SHA-512 handler should succeed for hello world");
    let hash = result
        .get("hash")
        .expect("SHA-512 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(
        hash,
        "374d794a95cdcfd8b35993185fef9ba368f160d8daf432d08ba9f1ed1e5abe6cc69291e0fa2fe0006a52570ef18c19def4e617c33ce52ef0a6e5fbe318cb0387"
    );
    assert_eq!(
        result
            .get("algorithm")
            .expect("SHA-512 response should contain 'algorithm'")
            .as_str()
            .expect("'algorithm' should be a string"),
        "sha512"
    );
    assert_eq!(
        result
            .get("output_bits")
            .expect("SHA-512 response should contain 'output_bits'")
            .as_u64()
            .expect("'output_bits' should be a JSON number"),
        512
    );
}

#[test]
fn test_sha256_invalid_base64() {
    let params = json!({
        "data": "not-valid-base64!!!"
    });

    let result = handle_sha256(&params);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid base64 data")
    );
}

#[test]
fn test_sha256_missing_data() {
    let params = json!({});

    let result = handle_sha256(&params);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Missing 'data'"));
}

#[test]
fn test_sha_family_consistency() {
    let test_data = b"Consistency test across SHA family";
    let params = json!({
        "data": BASE64.encode(test_data)
    });

    let sha256_result =
        handle_sha256(&params).expect("SHA-256 should succeed for consistency data");
    let sha384_result =
        handle_sha384(&params).expect("SHA-384 should succeed for consistency data");
    let sha512_result =
        handle_sha512(&params).expect("SHA-512 should succeed for consistency data");

    assert_eq!(
        sha256_result
            .get("hash")
            .expect("SHA-256 response should contain 'hash'")
            .as_str()
            .expect("'hash' should be a string")
            .len(),
        64
    );
    assert_eq!(
        sha384_result
            .get("hash")
            .expect("SHA-384 response should contain 'hash'")
            .as_str()
            .expect("'hash' should be a string")
            .len(),
        96
    );
    assert_eq!(
        sha512_result
            .get("hash")
            .expect("SHA-512 response should contain 'hash'")
            .as_str()
            .expect("'hash' should be a string")
            .len(),
        128
    );

    assert!(sha256_result.get("hash_base64").is_some());
    assert!(sha384_result.get("hash_base64").is_some());
    assert!(sha512_result.get("hash_base64").is_some());
}

#[test]
fn test_sha1_hello_world() {
    let params = json!({"data": BASE64.encode(b"Hello, World!")});
    let result = handle_sha1(&params).expect("SHA-1 handler should succeed for hello world");
    let hash = result
        .get("hash")
        .expect("SHA-1 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(hash, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
    assert_eq!(hash.len(), 40);
    assert!(result.get("warning").is_some());
}

#[test]
fn test_sha1_empty() {
    let params = json!({"data": BASE64.encode(b"")});
    let result = handle_sha1(&params).expect("SHA-1 handler should succeed for empty input");
    let hash = result
        .get("hash")
        .expect("SHA-1 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");
    assert_eq!(hash, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
}

#[test]
fn test_sha3_256_hello_world() {
    let params = json!({"data": BASE64.encode(b"Hello, World!")});
    let result = handle_sha3_256(&params).expect("SHA3-256 handler should succeed for hello world");
    let hash = result
        .get("hash")
        .expect("SHA3-256 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");

    assert_eq!(
        hash,
        "1af17a664e3fa8e419b8ba05c2a173169df76162a5a286e0c405b460d478f7ef"
    );
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_sha3_256_empty() {
    let params = json!({"data": BASE64.encode(b"")});
    let result = handle_sha3_256(&params).expect("SHA3-256 handler should succeed for empty input");
    let hash = result
        .get("hash")
        .expect("SHA3-256 response should contain 'hash'")
        .as_str()
        .expect("'hash' should be a JSON string");
    assert_eq!(
        hash,
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
}

#[test]
fn test_sha3_vs_sha2_different() {
    let data = BASE64.encode(b"test");
    let sha2 = handle_sha256(&json!({"data": &data}))
        .expect("SHA-256 should succeed for sha2 vs sha3 comparison");
    let sha3 = handle_sha3_256(&json!({"data": &data}))
        .expect("SHA3-256 should succeed for sha2 vs sha3 comparison");

    assert_ne!(
        sha2.get("hash")
            .expect("SHA-256 response should contain 'hash'"),
        sha3.get("hash")
            .expect("SHA3-256 response should contain 'hash'")
    );
}

#[test]
fn test_derive_onion_address_format() {
    let test_pubkey = [0u8; 32];
    let params = json!({
        "public_key": BASE64.encode(&test_pubkey)
    });

    let result = handle_derive_onion_address(&params)
        .expect("onion derivation should succeed for 32-byte test public key");

    let onion_address = result
        .get("onion_address")
        .expect("response should contain 'onion_address'")
        .as_str()
        .expect("'onion_address' should be a string");
    assert!(onion_address.ends_with(".onion"));
    assert_eq!(onion_address.len(), 62);

    assert_eq!(
        result
            .get("version")
            .expect("response should contain 'version'")
            .as_u64()
            .expect("'version' should be a JSON number"),
        3
    );

    assert!(result.get("checksum").is_some());
}

#[test]
fn test_derive_onion_address_consistency() {
    let test_pubkey = [42u8; 32];
    let params = json!({
        "public_key": BASE64.encode(&test_pubkey)
    });

    let result1 = handle_derive_onion_address(&params)
        .expect("first onion derivation should succeed for fixed public key");
    let result2 = handle_derive_onion_address(&params)
        .expect("second onion derivation should succeed for same public key");

    assert_eq!(
        result1
            .get("onion_address")
            .expect("first result should contain 'onion_address'")
            .as_str()
            .expect("'onion_address' should be a string"),
        result2
            .get("onion_address")
            .expect("second result should contain 'onion_address'")
            .as_str()
            .expect("'onion_address' should be a string")
    );
}

#[test]
fn test_derive_onion_address_invalid_key_length() {
    let short_key = [0u8; 16];
    let params = json!({
        "public_key": BASE64.encode(&short_key)
    });

    let result = handle_derive_onion_address(&params);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("32 bytes"));
}

#[test]
fn test_derive_onion_address_missing_params() {
    let params = json!({});
    let result = handle_derive_onion_address(&params);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Missing 'public_key'")
    );
}

#[test]
fn test_derive_onion_address_base32_lowercase() {
    let test_pubkey = [255u8; 32];
    let params = json!({
        "public_key": BASE64.encode(&test_pubkey)
    });

    let result = handle_derive_onion_address(&params)
        .expect("onion derivation should succeed for high-byte test key");
    let onion_address = result
        .get("onion_address")
        .expect("response should contain 'onion_address'")
        .as_str()
        .expect("'onion_address' should be a string");

    let addr_part = &onion_address[..56];
    assert!(
        addr_part
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    );
}

#[test]
fn test_derive_onion_address_invalid_pubkey_base64() {
    let params = json!({
        "public_key": "not!!!b64!!!"
    });
    let e = handle_derive_onion_address(&params).unwrap_err();
    assert!(e.to_string().contains("base64") || e.to_string().contains("public_key"));
}

#[test]
fn test_sha384_missing_data() {
    let e = handle_sha384(&json!({})).unwrap_err();
    assert!(e.to_string().contains("data"));
}

#[test]
fn test_sha512_invalid_base64() {
    let e = handle_sha512(&json!({"data": "bad@@@"})).unwrap_err();
    assert!(e.to_string().contains("base64"));
}

#[test]
fn test_sha1_missing_data() {
    let e = handle_sha1(&json!({})).unwrap_err();
    assert!(e.to_string().contains("data"));
}

#[test]
fn test_sha3_256_missing_data() {
    let e = handle_sha3_256(&json!({})).unwrap_err();
    assert!(e.to_string().contains("data"));
}

#[tokio::test]
async fn test_generate_onion_identity_default_purpose() {
    let out = handle_generate_onion_identity(None).await.expect("ok");
    assert_eq!(
        out.get("purpose").and_then(|v| v.as_str()),
        Some("hidden_service")
    );
}
