// SPDX-License-Identifier: AGPL-3.0-only

//! Derive child keys from a master key using HKDF-SHA256.

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::{Duration, Utc};

/// Handle key derivation command
pub async fn handle_key_derive(
    master_key_id: &str,
    purpose: &str,
    output_key_id: &str,
    expires_in: Option<&str>,
) -> Result<(), BearDogError> {
    println!("🔑 BearDog Key Derivation");
    println!("========================\n");

    // Load master key
    println!("📥 Loading master key: {master_key_id}");
    let master_key = key_store::load_key(master_key_id)?;

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
        println!(
            "   Expires: {} ({})",
            expiry.format("%Y-%m-%d %H:%M:%S UTC"),
            expires_in.unwrap_or("")
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
    key_store::save_key(&derived_key)?;

    // Update master key to add child
    let mut updated_master = master_key.clone();
    updated_master.children.push(output_key_id.to_string());
    key_store::save_key(&updated_master)?;

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
    let receipt_dir = std::path::Path::new("receipts");
    std::fs::create_dir_all(receipt_dir)?;
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
    use super::*;

    #[test]
    fn test_derive_key_hkdf() {
        let master_key = b"test_master_key_material_32bytes";
        let context = b"test_purpose";

        let derived1 = derive_key_hkdf(master_key, context).unwrap();
        let derived2 = derive_key_hkdf(master_key, context).unwrap();

        // Should be deterministic
        assert_eq!(derived1, derived2);
        assert_eq!(derived1.len(), 32);

        // Different context should produce different key
        let derived3 = derive_key_hkdf(master_key, b"different_purpose").unwrap();
        assert_ne!(derived1, derived3);
    }

    #[test]
    fn test_parse_duration() {
        // Hours
        let result = parse_duration("24h").unwrap();
        // Allow 2-hour tolerance for test timing variations
        assert!((result.signed_duration_since(Utc::now()).num_hours() - 24).abs() < 2);

        // Days
        let result = parse_duration("30d").unwrap();
        // Allow 2-day tolerance for test timing variations
        assert!((result.signed_duration_since(Utc::now()).num_days() - 30).abs() < 2);

        // Weeks
        let result = parse_duration("2w").unwrap();
        assert!((result.signed_duration_since(Utc::now()).num_days() - 14).abs() < 2);

        // Invalid format
        assert!(parse_duration("invalid").is_err());
        assert!(parse_duration("24").is_err());
    }
}
