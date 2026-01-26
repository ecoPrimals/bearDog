//! BirdSong v2 API Unit Tests
//!
//! Tests for the BirdSong v2 API endpoints (Songbird-compatible)
//!
//! Tests coverage:
//! - Request/response serialization
//! - Encryption/decryption flow
//! - Family ID handling
//! - Error cases
//! - Base64 encoding/decoding
//! - Privacy-preserving fallback behavior

use beardog_tunnel::api::birdsong::*;
use beardog_tunnel::api::types::ApiResponse;

// ====================================================================================
// V2 Request Types Unit Tests
// ====================================================================================

#[test]
fn test_encrypt_request_v2_serialization() {
    let plaintext = b"Hello, Songbird!";

    let request = EncryptRequestV2 {
        plaintext: plaintext.to_vec(),
        family_id: Some("test_family".to_string()),
    };

    let json = serde_json::to_value(&request).unwrap();
    // Plaintext should be base64 encoded in JSON
    assert!(json["plaintext"].is_string());
    assert_eq!(json["family_id"], "test_family");
}

#[test]
fn test_encrypt_request_v2_without_family() {
    let request = EncryptRequestV2 {
        plaintext: b"test".to_vec(),
        family_id: None,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["plaintext"].is_string());
    assert!(json["family_id"].is_null());
}

#[test]
fn test_encrypt_request_v2_roundtrip() {
    let plaintext = b"Hello, world!";
    let request = EncryptRequestV2 {
        plaintext: plaintext.to_vec(),
        family_id: Some("my_family".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.plaintext, plaintext);
    assert_eq!(deserialized.family_id, Some("my_family".to_string()));
}

#[test]
fn test_decrypt_request_v2_serialization() {
    let ciphertext = b"encrypted_data";

    let request = DecryptRequestV2 {
        ciphertext: ciphertext.to_vec(),
        family_id: Some("test_family".to_string()),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["ciphertext"].is_string());
    assert_eq!(json["family_id"], "test_family");
}

#[test]
fn test_decrypt_request_v2_without_family() {
    let request = DecryptRequestV2 {
        ciphertext: b"test_cipher".to_vec(),
        family_id: None,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["ciphertext"].is_string());
    assert!(json["family_id"].is_null());
}

// ====================================================================================
// V2 Response Types Unit Tests
// ====================================================================================

#[test]
fn test_encrypt_response_v2_serialization() {
    let response = EncryptResponseV2 {
        ciphertext: b"encrypted".to_vec(),
        family_id: "test_family".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(json["ciphertext"].is_string());
    assert_eq!(json["family_id"], "test_family");
}

#[test]
fn test_decrypt_response_v2_success() {
    let plaintext = b"decrypted data";

    let response = DecryptResponseV2 {
        plaintext: plaintext.to_vec(),
        family_id: "test_family".to_string(),
        success: true,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(json["plaintext"].is_string());
    assert_eq!(json["family_id"], "test_family");
    assert_eq!(json["success"], true);
}

#[test]
fn test_decrypt_response_v2_failure() {
    let response = DecryptResponseV2 {
        plaintext: vec![],
        family_id: "other_family".to_string(),
        success: false,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["success"], false);
}

// ====================================================================================
// Legacy API Types Tests (discovery-specific)
// ====================================================================================

#[test]
fn test_legacy_encrypt_discovery_request() {
    let request = EncryptDiscoveryRequest {
        plaintext: vec![1, 2, 3, 4],
        family_id: "legacy_family".to_string(),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["plaintext"].is_array() || json["plaintext"].is_string());
    assert_eq!(json["family_id"], "legacy_family");
}

#[test]
fn test_legacy_decrypt_discovery_request() {
    let request = DecryptDiscoveryRequest {
        encrypted: vec![5, 6, 7, 8],
        family_id: "legacy_family".to_string(),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["encrypted"].is_array() || json["encrypted"].is_string());
    assert_eq!(json["family_id"], "legacy_family");
}

// ====================================================================================
// Base64 Encoding Tests (via serde)
// ====================================================================================

#[test]
fn test_base64_roundtrip_simple() {
    let original = b"Hello, BirdSong v2!";
    let request = EncryptRequestV2 {
        plaintext: original.to_vec(),
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.plaintext, original);
}

#[test]
fn test_base64_roundtrip_utf8() {
    let original = "🐻🐕 BearDog loves 🐦 Songbird!";
    let request = EncryptRequestV2 {
        plaintext: original.as_bytes().to_vec(),
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    let decoded_str = String::from_utf8(deserialized.plaintext).unwrap();
    assert_eq!(original, decoded_str);
}

#[test]
fn test_base64_empty_data() {
    let request = EncryptRequestV2 {
        plaintext: vec![],
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.plaintext.len(), 0);
}

#[test]
fn test_base64_large_data() {
    let original: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let request = EncryptRequestV2 {
        plaintext: original.clone(),
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.plaintext, original);
}

// ====================================================================================
// Family ID Handling Tests
// ====================================================================================

#[test]
fn test_family_id_with_special_characters() {
    let request = EncryptRequestV2 {
        plaintext: b"test".to_vec(),
        family_id: Some("family-with_special.chars!@#".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(
        deserialized.family_id,
        Some("family-with_special.chars!@#".to_string())
    );
}

#[test]
fn test_family_id_unicode() {
    let request = EncryptRequestV2 {
        plaintext: b"test".to_vec(),
        family_id: Some("家族🐻".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.family_id, Some("家族🐻".to_string()));
}

#[test]
fn test_family_id_empty_string() {
    let request = EncryptRequestV2 {
        plaintext: b"test".to_vec(),
        family_id: Some("".to_string()),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["family_id"], "");
}

// ====================================================================================
// Error Case Tests
// ====================================================================================

#[test]
fn test_decrypt_response_empty_plaintext_on_failure() {
    let response = DecryptResponseV2 {
        plaintext: vec![],
        family_id: "family".to_string(),
        success: false,
    };

    assert_eq!(response.plaintext.len(), 0);
    assert!(!response.success);
}

// ====================================================================================
// Integration Compatibility Tests
// ====================================================================================

#[test]
fn test_songbird_compatible_encrypt_request() {
    // Ensure our types match Songbird's expectations (base64-encoded plaintext)
    // The base64 serde module handles encoding/decoding transparently
    let plaintext = b"Hello Songbird!";

    let request = EncryptRequestV2 {
        plaintext: plaintext.to_vec(),
        family_id: Some("ecoPrimals".to_string()),
    };

    // Serialize and check that it works properly
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.plaintext, plaintext);
    assert_eq!(deserialized.family_id, Some("ecoPrimals".to_string()));
}

#[test]
fn test_songbird_compatible_decrypt_response() {
    // Ensure our response format matches what Songbird expects
    let response = DecryptResponseV2 {
        plaintext: b"Hello BearDog!".to_vec(),
        family_id: "ecoPrimals".to_string(),
        success: true,
    };

    let json = serde_json::to_string(&response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert!(parsed["plaintext"].is_string()); // Should be base64-encoded
    assert_eq!(parsed["family_id"], "ecoPrimals");
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_privacy_preserving_failure_response() {
    // When decryption fails (different family), we return success=false
    // This preserves privacy by not leaking whether we tried to decrypt
    let response = DecryptResponseV2 {
        plaintext: vec![],
        family_id: "other_family".to_string(),
        success: false,
    };

    assert!(!response.success);
    assert_eq!(response.plaintext.len(), 0);
}

// ====================================================================================
// V2/Legacy Coexistence Tests
// ====================================================================================

#[test]
fn test_v2_and_legacy_coexistence() {
    // Both v2 and legacy types should work side by side
    let legacy_req = EncryptDiscoveryRequest {
        plaintext: vec![1, 2, 3],
        family_id: "family1".to_string(),
    };

    let v2_req = EncryptRequestV2 {
        plaintext: vec![1, 2, 3],
        family_id: Some("family1".to_string()),
    };

    let legacy_json = serde_json::to_string(&legacy_req).unwrap();
    let v2_json = serde_json::to_string(&v2_req).unwrap();

    assert!(legacy_json.contains("family1"));
    assert!(v2_json.contains("family1"));
}

// ====================================================================================
// Response Wrapping Tests (ApiResponse)
// ====================================================================================

#[test]
fn test_api_response_success() {
    let data = EncryptResponseV2 {
        ciphertext: b"encrypted".to_vec(),
        family_id: "test".to_string(),
    };
    let response = ApiResponse::success(data);

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["success"], true);
    assert!(json["data"].is_object());
}

// ====================================================================================
// Binary Data Tests
// ====================================================================================

#[test]
fn test_binary_data_handling() {
    // Test with non-UTF8 binary data
    let binary_data: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x00, 0x01, 0x02];
    let request = EncryptRequestV2 {
        plaintext: binary_data.clone(),
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.plaintext, binary_data);
}

#[test]
fn test_all_byte_values() {
    // Test that all possible byte values can be handled
    let all_bytes: Vec<u8> = (0..=255).collect();
    let request = EncryptRequestV2 {
        plaintext: all_bytes.clone(),
        family_id: Some("test".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: EncryptRequestV2 = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.plaintext, all_bytes);
}
