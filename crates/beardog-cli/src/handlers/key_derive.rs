// SPDX-License-Identifier: AGPL-3.0-only

//! Derive child keys from a master key using HKDF-SHA256.

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use std::path::Path;

/// Handle key derivation command
pub async fn handle_key_derive(
    master_key_id: &str,
    purpose: &str,
    output_key_id: &str,
    expires_in: Option<&str>,
) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_derive_with_home(
        master_key_id,
        purpose,
        output_key_id,
        expires_in,
        home.as_path(),
    )
    .await
}

/// Same as [`handle_key_derive`] but keys and receipts live under `home` (tests / DI).
pub async fn handle_key_derive_with_home(
    master_key_id: &str,
    purpose: &str,
    output_key_id: &str,
    expires_in: Option<&str>,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("🔑 BearDog Key Derivation");
    println!("========================\n");

    // Load master key
    println!("📥 Loading master key: {master_key_id}");
    let master_key = key_store::load_key_from_home(master_key_id, home)?;

    println!("✅ Master key loaded");
    println!("   Algorithm: {}", master_key.algorithm);
    println!("   HSM: {}", master_key.hsm_name);
    println!("   Generation: {}", master_key.generation);

    // Decode master key material
    let master_key_material = key_store::base64_decode(&master_key.key_material_b64)?;

    // Derive new key using HKDF
    println!("\n🔐 Deriving new key...");
    println!("   Purpose: {purpose}");

    let derived_material = derive_key_hkdf(&master_key_material, purpose.as_bytes())?;

    // Calculate expiry if specified
    let expires_at = if let Some(duration_str) = expires_in {
        Some(parse_duration(duration_str)?)
    } else {
        None
    };

    if let Some(expiry) = expires_at {
        let duration_label = expires_in.unwrap_or_default();
        println!(
            "   Expires: {} ({duration_label})",
            expiry.format("%Y-%m-%d %H:%M:%S UTC"),
        );
    }

    // Format expiry as string for storage
    let expires_at_str = expires_at.map(|dt| dt.to_rfc3339());

    // Create derived key metadata
    let parent_depth = master_key.lineage.as_ref().map_or(0, |l| l.depth);

    let derived_key = StoredKey {
        key_id: output_key_id.to_string(),
        algorithm: master_key.algorithm.clone(),
        hsm_name: master_key.hsm_name.clone(),
        key_material_b64: key_store::base64_encode(&derived_material),
        created_at: Utc::now().to_rfc3339(),
        generation: master_key.generation + 1,
        parent_key_id: Some(master_key_id.to_string()),
        derivation_purpose: Some(purpose.to_string()),
        children: Vec::new(),
        lineage: Some(key_store::KeyLineageInfo {
            parent_key_id: Some(master_key_id.to_string()),
            depth: parent_depth + 1,
        }),
        expires_at: expires_at_str,
        usage: None,
        purpose: Some(purpose.to_string()),
    };

    // Save derived key
    key_store::save_key_to_home(&derived_key, home)?;

    // Update master key to add child
    let mut updated_master = master_key.clone();
    updated_master.children.push(output_key_id.to_string());
    key_store::save_key_to_home(&updated_master, home)?;

    // Generate operation receipt
    use beardog_types::receipt::{KeyInfo, OperationReceipt, generate_receipt_filename};
    use serde_json::json;

    let receipt = OperationReceipt::new("key-derive")
        .with_key_info(KeyInfo {
            key_id: output_key_id.to_string(),
            algorithm: derived_key.algorithm.clone(),
            generation: derived_key.generation,
            parent_key_id: Some(master_key_id.to_string()),
            expires_at: derived_key.expires_at.clone(),
            usage: None,
            purpose: Some(purpose.to_string()),
        })
        .with_metadata("master_key_id", json!(master_key_id))
        .with_metadata("derivation_purpose", json!(purpose));

    // Save receipt
    let receipt_dir = home.join("receipts");
    std::fs::create_dir_all(&receipt_dir)?;
    let receipt_path = receipt_dir.join(generate_receipt_filename("key-derive"));
    receipt.save_to_file(&receipt_path)?;

    println!("\n✅ Key derived successfully!");
    println!("\n📋 Derived Key Details:");
    println!("   ID: {output_key_id}");
    println!("   Algorithm: {}", derived_key.algorithm);
    println!("   HSM: {}", derived_key.hsm_name);
    println!(
        "   Generation: {} (derived from Gen {})",
        derived_key.generation, master_key.generation
    );
    println!("   Parent: {master_key_id}");
    println!("   Status: Active");

    if let Some(expiry) = expires_at {
        println!("\n⏰ Expiry:");
        println!("   Expires at: {}", expiry.format("%Y-%m-%d %H:%M:%S UTC"));
        let duration_left = expiry.signed_duration_since(Utc::now());
        if duration_left.num_days() > 0 {
            println!("   Time remaining: {} days", duration_left.num_days());
        } else if duration_left.num_hours() > 0 {
            println!("   Time remaining: {} hours", duration_left.num_hours());
        }
    }

    println!("\n📜 Receipt: {}", receipt_path.display());
    println!("   Receipt ID: {}", receipt.receipt_id);

    println!("\n💡 Next steps:");
    println!(
        "   • Encrypt: beardog encrypt --key {output_key_id} --input data.txt --output data.enc"
    );
    println!("   • View info: beardog key info --key-id {output_key_id}");
    println!("   • View lineage: beardog key lineage --key-id {output_key_id}");

    Ok(())
}

/// Derive key using HKDF (HMAC-based Key Derivation Function)
fn derive_key_hkdf(master_key: &[u8], context: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    // Use HKDF with SHA-256
    let hk = Hkdf::<Sha256>::new(Some(b"beardog_key_derivation_v1"), master_key);

    // Expand to 32 bytes (256 bits)
    let mut okm = vec![0u8; 32];
    hk.expand(context, &mut okm)
        .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

    Ok(okm)
}

/// Parse a duration like `24h`, `30d`, or `1y` into an absolute UTC expiry.
pub fn parse_duration(duration_str: &str) -> Result<chrono::DateTime<Utc>, BearDogError> {
    let duration_str = duration_str.trim().to_lowercase();

    // Parse number and unit
    let (num_str, unit) = if let Some(pos) = duration_str.find(|c: char| c.is_alphabetic()) {
        (&duration_str[..pos], &duration_str[pos..])
    } else {
        return Err(BearDogError::validation(
            "Invalid duration format. Use format like '24h', '30d', '1y'",
        ));
    };

    let num: i64 = num_str
        .trim()
        .parse()
        .map_err(|_| BearDogError::validation("Invalid duration number"))?;

    let duration = match unit {
        "h" | "hour" | "hours" => Duration::hours(num),
        "d" | "day" | "days" => Duration::days(num),
        "w" | "week" | "weeks" => Duration::weeks(num),
        "m" | "month" | "months" => Duration::days(num * 30), // Approximate
        "y" | "year" | "years" => Duration::days(num * 365),  // Approximate
        _ => {
            return Err(BearDogError::validation(&format!(
                "Unknown duration unit '{unit}'. Use h (hours), d (days), w (weeks), m (months), or y (years)"
            )));
        }
    };

    Ok(Utc::now() + duration)
}

#[cfg(test)]
mod tests {
    use super::key_store;
    use super::*;
    use chrono::Utc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_handle_key_derive_with_home_roundtrip() {
        let dir = TempDir::new().expect("create temp directory for key derive roundtrip test");
        let home = dir.path();
        let master = StoredKey {
            key_id: "master-1".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
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
        key_store::save_key_to_home(&master, home).expect("save master key for derive test");

        handle_key_derive_with_home("master-1", "student-1", "child-1", None, home)
            .await
            .expect("derive child key from master");

        let child = key_store::load_key_from_home("child-1", home).expect("load derived child key");
        assert_eq!(child.parent_key_id.as_deref(), Some("master-1"));
        assert_eq!(child.generation, 1);
        assert_eq!(child.derivation_purpose.as_deref(), Some("student-1"));

        let master_again =
            key_store::load_key_from_home("master-1", home).expect("reload master after derive");
        assert!(master_again.children.contains(&"child-1".to_string()));

        let receipt_dir = home.join("receipts");
        let entries: Vec<_> = std::fs::read_dir(&receipt_dir)
            .expect("read receipts directory")
            .filter_map(|e| e.ok())
            .collect();
        assert!(
            !entries.is_empty(),
            "receipt file should exist under home/receipts"
        );
    }

    #[tokio::test]
    async fn test_handle_key_derive_with_home_expiry() {
        let dir = TempDir::new().expect("create temp directory for derive with expiry test");
        let home = dir.path();
        let master = StoredKey {
            key_id: "master-exp".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
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
        key_store::save_key_to_home(&master, home).expect("save master-exp key");

        handle_key_derive_with_home("master-exp", "purpose-x", "child-exp", Some("48h"), home)
            .await
            .expect("derive key with 48h expiry");

        let child =
            key_store::load_key_from_home("child-exp", home).expect("load child-exp with expiry");
        assert!(child.expires_at.is_some());
    }

    #[test]
    fn test_derive_key_hkdf() {
        let master_key = b"test_master_key_material_32bytes";
        let context = b"test_purpose";

        let derived1 = derive_key_hkdf(master_key, context).expect("HKDF derive first");
        let derived2 = derive_key_hkdf(master_key, context).expect("HKDF derive second");

        // Should be deterministic
        assert_eq!(derived1, derived2);
        assert_eq!(derived1.len(), 32);

        // Different context should produce different key
        let derived3 =
            derive_key_hkdf(master_key, b"different_purpose").expect("HKDF different context");
        assert_ne!(derived1, derived3);
    }

    #[test]
    fn test_parse_duration() {
        // Hours
        let result = parse_duration("24h").expect("parse 24h");
        // Allow 2-hour tolerance for test timing variations
        assert!((result.signed_duration_since(Utc::now()).num_hours() - 24).abs() < 2);

        // Days
        let result = parse_duration("30d").expect("parse 30d");
        // Allow 2-day tolerance for test timing variations
        assert!((result.signed_duration_since(Utc::now()).num_days() - 30).abs() < 2);

        // Weeks
        let result = parse_duration("2w").expect("parse 2w");
        assert!((result.signed_duration_since(Utc::now()).num_days() - 14).abs() < 2);

        // Invalid format
        assert!(parse_duration("invalid").is_err());
        assert!(parse_duration("24").is_err());

        // Months / years (approximate)
        let m = parse_duration("2m").expect("parse 2m");
        assert!((m.signed_duration_since(Utc::now()).num_days() - 60).abs() < 3);
        let y = parse_duration("1y").expect("parse 1y");
        assert!((y.signed_duration_since(Utc::now()).num_days() - 365).abs() < 3);

        assert!(parse_duration("5x").is_err());
    }

    #[test]
    fn test_parse_duration_trims_and_unit_aliases() {
        let h = parse_duration("  6 hour ").expect("parse 6 hour with spaces");
        assert!((h.signed_duration_since(Utc::now()).num_hours() - 6).abs() < 2);

        let d = parse_duration("3 days").expect("parse 3 days");
        assert!((d.signed_duration_since(Utc::now()).num_days() - 3).abs() < 2);

        let w = parse_duration("1 week").expect("parse 1 week");
        assert!((w.signed_duration_since(Utc::now()).num_days() - 7).abs() < 2);
    }

    #[test]
    fn test_parse_duration_invalid_number() {
        assert!(parse_duration("xxh").is_err());
    }

    #[tokio::test]
    async fn test_handle_key_derive_with_home_missing_master_fails() {
        let dir = TempDir::new().expect("create temp directory for missing master test");
        let home = dir.path();
        let r = handle_key_derive_with_home("no-such-key", "p", "out", None, home).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_handle_key_derive_with_home_invalid_master_material_fails() {
        let dir = TempDir::new().expect("create temp directory for invalid b64 master test");
        let home = dir.path();
        let master = StoredKey {
            key_id: "bad-b64".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            key_material_b64: "@@@not-valid-base64@@@".to_string(),
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
        key_store::save_key_to_home(&master, home).expect("save bad-b64 master key");
        let r = handle_key_derive_with_home("bad-b64", "p", "child-x", None, home).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_handle_key_derive_with_home_lineage_depth_increments() {
        let dir = TempDir::new().expect("create temp directory for lineage depth test");
        let home = dir.path();
        let master = StoredKey {
            key_id: "lineage-root".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
            created_at: Utc::now().to_rfc3339(),
            generation: 2,
            parent_key_id: Some("parent".to_string()),
            derivation_purpose: None,
            children: vec![],
            lineage: Some(key_store::KeyLineageInfo {
                parent_key_id: Some("parent".to_string()),
                depth: 3,
            }),
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&master, home).expect("save lineage-root master");

        handle_key_derive_with_home("lineage-root", "next", "child-depth", None, home)
            .await
            .expect("derive child-depth from lineage-root");

        let child =
            key_store::load_key_from_home("child-depth", home).expect("load child-depth key");
        assert_eq!(
            child
                .lineage
                .as_ref()
                .expect("derived key must have lineage metadata")
                .depth,
            4
        );
    }

    #[tokio::test]
    async fn test_handle_key_derive_with_home_short_expiry_prints_hours_branch() {
        let dir = TempDir::new().expect("create temp directory for short expiry branch test");
        let home = dir.path();
        let master = StoredKey {
            key_id: "m-hours".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
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
        key_store::save_key_to_home(&master, home).expect("save m-hours master");

        handle_key_derive_with_home("m-hours", "p", "c-hours", Some("3h"), home)
            .await
            .expect("derive c-hours with 3h expiry");
        let child = key_store::load_key_from_home("c-hours", home).expect("load c-hours");
        assert!(child.expires_at.is_some());
    }
}
