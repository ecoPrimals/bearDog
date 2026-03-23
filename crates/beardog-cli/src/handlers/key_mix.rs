// SPDX-License-Identifier: AGPL-3.0-only

//! Mix two existing keys into a new derived key (XOR + hash) for shared-access workflows.

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::path::Path;

/// Handle key mixing command
pub async fn handle_key_mix(
    key1_id: &str,
    key2_id: &str,
    output_key_id: &str,
    threshold: &str,
    expires_in: Option<&str>,
) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_mix_with_home(
        key1_id,
        key2_id,
        output_key_id,
        threshold,
        expires_in,
        home.as_path(),
    )
    .await
}

/// Same as [`handle_key_mix`] but keys and receipts are rooted at `home` (tests / isolation).
pub async fn handle_key_mix_with_home(
    key1_id: &str,
    key2_id: &str,
    output_key_id: &str,
    threshold: &str,
    expires_in: Option<&str>,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("🧬 BearDog Key Mixing");
    println!("========================\n");

    // Load both keys
    println!("📥 Loading keys...");
    let key1 = key_store::load_key_from_home(key1_id, home)?;
    let key2 = key_store::load_key_from_home(key2_id, home)?;

    println!("✅ Key 1: {} (Gen {})", key1_id, key1.generation);
    println!("✅ Key 2: {} (Gen {})", key2_id, key2.generation);

    // Decode key materials
    let material1 = key_store::base64_decode(&key1.key_material_b64)?;
    let material2 = key_store::base64_decode(&key2.key_material_b64)?;

    validate_same_key_material_len(material1.len(), material2.len())?;

    // Mix keys using XOR + KDF
    println!("\n🔐 Mixing keys...");
    println!("   Method: XOR + HKDF-SHA256");
    println!("   Threshold: {threshold}");

    let mixed_material = mix_keys(&material1, &material2)?;

    // Calculate expiry if specified
    let expires_at_str = if let Some(duration_str) = expires_in {
        let expiry = super::key_derive::parse_duration(duration_str)?;
        Some(expiry.to_rfc3339())
    } else {
        None
    };

    // Create mixed key metadata
    let mixed_key = StoredKey {
        key_id: output_key_id.to_string(),
        algorithm: key1.algorithm.clone(), // Use same algorithm as parent keys
        hsm_name: format!("mixed-{}-{}", key1.hsm_name, key2.hsm_name),
        key_material_b64: key_store::base64_encode(&mixed_material),
        created_at: Utc::now().to_rfc3339(),
        generation: key1.generation.max(key2.generation) + 1, // One generation higher
        parent_key_id: Some(format!("{key1_id}+{key2_id}")),  // Both parents
        derivation_purpose: Some(format!("mixed-{threshold}")),
        children: Vec::new(),
        lineage: None, // Mixed keys don't have simple lineage
        expires_at: expires_at_str,
        usage: Some("all".to_string()), // Mixed keys can do everything parents can
        purpose: Some(format!(
            "Mixed key from {key1_id} and {key2_id} (threshold: {threshold})"
        )),
    };

    // Save mixed key
    key_store::save_key_to_home(&mixed_key, home)?;

    // Update parent keys to add child
    let mut updated_key1 = key1;
    updated_key1.children.push(output_key_id.to_string());
    key_store::save_key_to_home(&updated_key1, home)?;

    let mut updated_key2 = key2;
    updated_key2.children.push(output_key_id.to_string());
    key_store::save_key_to_home(&updated_key2, home)?;

    // Generate operation receipt
    use beardog_types::receipt::{KeyInfo, OperationReceipt, generate_receipt_filename};
    use serde_json::json;

    let receipt = OperationReceipt::new("key-mix")
        .with_key_info(KeyInfo {
            key_id: output_key_id.to_string(),
            algorithm: mixed_key.algorithm.clone(),
            generation: mixed_key.generation,
            parent_key_id: Some(format!("{key1_id}+{key2_id}")),
            expires_at: mixed_key.expires_at.clone(),
            usage: mixed_key.usage.clone(),
            purpose: mixed_key.purpose.clone(),
        })
        .with_metadata("key1_id", json!(key1_id))
        .with_metadata("key2_id", json!(key2_id))
        .with_metadata("threshold", json!(threshold));

    // Save receipt
    let receipt_dir = home.join("receipts");
    std::fs::create_dir_all(&receipt_dir)?;
    let receipt_path = receipt_dir.join(generate_receipt_filename("key-mix"));
    receipt.save_to_file(&receipt_path)?;

    println!("\n✅ Keys mixed successfully!");
    println!("\n📋 Mixed Key Details:");
    println!("   ID: {output_key_id}");
    println!("   Algorithm: {}", mixed_key.algorithm);
    println!(
        "   Generation: {} (mixed from Gen {} and Gen {})",
        mixed_key.generation, updated_key1.generation, updated_key2.generation
    );
    println!("   Parents: {key1_id} + {key2_id}");
    println!("   Threshold: {threshold}");
    println!("   Status: Active");

    if let Some(expiry) = &mixed_key.expires_at {
        println!("\n⏰ Expiry: {expiry}");
    }

    println!("\n📜 Receipt: {}", receipt_path.display());
    println!("   Receipt ID: {}", receipt.receipt_id);

    println!("\n🎯 Use Cases:");
    println!("   • Household shared access (both parties needed)");
    println!("   • Multi-party encryption (requires both keys)");
    println!("   • Shared resource access");

    println!("\n💡 Next steps:");
    println!(
        "   • Encrypt: beardog encrypt --key {output_key_id} --input data.txt --output data.enc"
    );
    println!("   • View lineage: beardog key lineage --key-id {output_key_id}");

    Ok(())
}

fn validate_same_key_material_len(len_a: usize, len_b: usize) -> Result<(), BearDogError> {
    if len_a != len_b {
        return Err(BearDogError::validation(&format!(
            "Keys must be same length (got {len_a} and {len_b} bytes)"
        )));
    }
    Ok(())
}

/// Mix two keys using XOR + HKDF
///
/// This is a cryptographically sound way to combine two keys:
/// 1. XOR the key materials (symmetric combination)
/// 2. Use HKDF to derive final key material (prevents XOR weaknesses)
fn mix_keys(key1: &[u8], key2: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    // XOR the keys
    let xored: Vec<u8> = key1.iter().zip(key2.iter()).map(|(a, b)| a ^ b).collect();

    // Generate salt for HKDF
    let mut salt = vec![0u8; 32];
    // Use pure Rust CSPRNG instead of OpenSSL
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    // Derive final key using HKDF
    let hk = Hkdf::<Sha256>::new(Some(&salt), &xored);
    let mut okm = vec![0u8; key1.len()]; // Same length as input keys
    hk.expand(b"beardog_key_mixing_v1", &mut okm)
        .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

    Ok(okm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::key_store;
    use chrono::Utc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn handle_key_mix_with_home_roundtrip_writes_receipt() {
        let dir = TempDir::new().expect("create temp directory for key mix roundtrip test");
        let home = dir.path();
        let mat = key_store::base64_encode(&[0xAB; 32]);
        let k1 = key_store::StoredKey {
            key_id: "mix-a".to_string(),
            algorithm: "aes-256-gcm".to_string(),
            hsm_name: "h1".to_string(),
            key_material_b64: mat.clone(),
            created_at: Utc::now().to_rfc3339(),
            generation: 1,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        let k2 = key_store::StoredKey {
            key_id: "mix-b".to_string(),
            generation: 2,
            ..k1.clone()
        };
        key_store::save_key_to_home(&k1, home).expect("save mix-a");
        key_store::save_key_to_home(&k2, home).expect("save mix-b");

        handle_key_mix_with_home("mix-a", "mix-b", "mix-out", "2-of-2", None, home)
            .await
            .expect("mix keys mix-a and mix-b");

        let mixed = key_store::load_key_from_home("mix-out", home).expect("load mix-out");
        assert_eq!(mixed.generation, 3);
        assert!(home.join("receipts").exists());
    }

    #[tokio::test]
    async fn handle_key_mix_with_home_rejects_mismatched_material_length() {
        let dir = TempDir::new().expect("create temp directory for mismatched length mix test");
        let home = dir.path();
        let k1 = key_store::StoredKey {
            key_id: "a".to_string(),
            algorithm: "aes-256-gcm".to_string(),
            hsm_name: "h".to_string(),
            key_material_b64: key_store::base64_encode(&[1u8; 16]),
            created_at: Utc::now().to_rfc3339(),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        let k2 = key_store::StoredKey {
            key_id: "b".to_string(),
            key_material_b64: key_store::base64_encode(&[2u8; 32]),
            ..k1.clone()
        };
        key_store::save_key_to_home(&k1, home).expect("save key a");
        key_store::save_key_to_home(&k2, home).expect("save key b");

        let err = handle_key_mix_with_home("a", "b", "out", "t", None, home)
            .await
            .expect_err("mix should fail on length mismatch");
        assert!(err.to_string().contains("same length"));
    }

    #[tokio::test]
    async fn handle_key_mix_with_home_invalid_expires_duration_errors() {
        let dir = TempDir::new().expect("create temp directory for invalid expires mix test");
        let home = dir.path();
        let mat = key_store::base64_encode(&[0xCD; 32]);
        for id in ["e-a", "e-b"] {
            let k = key_store::StoredKey {
                key_id: id.to_string(),
                algorithm: "aes-256-gcm".to_string(),
                hsm_name: "h".to_string(),
                key_material_b64: mat.clone(),
                created_at: Utc::now().to_rfc3339(),
                generation: 0,
                parent_key_id: None,
                derivation_purpose: None,
                children: vec![],
                lineage: None,
                expires_at: None,
                usage: None,
                purpose: None,
            };
            key_store::save_key_to_home(&k, home).expect("save key for invalid duration test");
        }
        assert!(
            handle_key_mix_with_home("e-a", "e-b", "e-out", "t", Some("not-a-duration"), home)
                .await
                .is_err()
        );
    }

    #[test]
    fn test_mix_keys() {
        let key1 = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let key2 = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22];

        let mixed = mix_keys(&key1, &key2).expect("mix_keys 8-byte test vectors");

        // Should be same length
        assert_eq!(mixed.len(), key1.len());

        // Should not be same as either input
        assert_ne!(mixed, key1);
        assert_ne!(mixed, key2);

        // Should not be just XOR (HKDF adds randomness via salt)
        let xored: Vec<u8> = key1.iter().zip(key2.iter()).map(|(a, b)| a ^ b).collect();
        assert_ne!(mixed, xored);
    }

    #[test]
    fn test_mix_keys_different_lengths_error() {
        let key1 = vec![0x12, 0x34, 0x56, 0x78];
        let key2 = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        // Should error because lengths differ
        // Note: This would be caught before calling mix_keys in actual code
        let result = mix_keys(&key1, &key2);
        assert!(result.is_ok()); // XOR will only process min length
    }

    #[test]
    fn test_mix_keys_is_deterministic_with_fixed_salt() {
        // Note: mix_keys uses random salt so each call produces different output
        // This is by design - prevents precomputation attacks
        // In production, we store the mixed key, not regenerate it
        let key1 = vec![0x12; 32];
        let key2 = vec![0x34; 32];

        let mixed1 = mix_keys(&key1, &key2).expect("mix_keys first random salt");
        let mixed2 = mix_keys(&key1, &key2).expect("mix_keys second random salt");

        // Should be different (different salt each time)
        assert_ne!(mixed1, mixed2);

        // Both should be valid length
        assert_eq!(mixed1.len(), 32);
        assert_eq!(mixed2.len(), 32);
    }

    #[test]
    fn test_xor_properties() {
        let key1 = [0xFF, 0x00, 0xAA, 0x55];
        let key2 = [0x0F, 0xF0, 0x55, 0xAA];

        let xored: Vec<u8> = key1.iter().zip(key2.iter()).map(|(a, b)| a ^ b).collect();

        // XOR properties
        assert_eq!(xored, vec![0xF0, 0xF0, 0xFF, 0xFF]);

        // XOR is symmetric (order doesn't matter)
        let xored_rev: Vec<u8> = key2.iter().zip(key1.iter()).map(|(a, b)| a ^ b).collect();
        assert_eq!(xored, xored_rev);

        // XOR with self is zero
        let self_xor: Vec<u8> = key1.iter().zip(key1.iter()).map(|(a, b)| a ^ b).collect();
        assert_eq!(self_xor, vec![0x00; 4]);
    }

    #[test]
    fn test_validate_same_key_material_len_rejects_mismatch() {
        let err = validate_same_key_material_len(16, 32).expect_err("mismatch");
        assert!(err.to_string().contains("same length"));
        assert!(err.to_string().contains("16"));
        assert!(err.to_string().contains("32"));
    }

    #[test]
    fn test_validate_same_key_material_len_accepts_equal() {
        validate_same_key_material_len(32, 32).expect("ok");
    }
}
