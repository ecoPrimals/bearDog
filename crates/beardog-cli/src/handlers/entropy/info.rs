// SPDX-License-Identifier: AGPL-3.0-only

//! `beardog entropy info` — display seed metadata and sample bytes.

use std::fs;

use beardog_errors::BearDogError;

use super::helpers::base64_decode;
use super::types::EntropySeedMetadata;

/// Handle entropy info command
///
/// # Errors
///
/// Returns an error if the seed file cannot be read, JSON is invalid, base64 decoding fails, or
/// I/O fails.
pub async fn handle_entropy_info(seed_path: &str) -> Result<(), BearDogError> {
    println!("Entropy Seed Information");
    println!("==========================");
    println!();

    // Read seed file
    let json = fs::read_to_string(seed_path)?;
    let seed: EntropySeedMetadata =
        serde_json::from_str(&json).map_err(|e| BearDogError::serialization(&e.to_string()))?;

    // Display info
    println!("Seed Details:");
    println!("   ID: {}", seed.seed_id);
    println!("   Quality Tier: {}", seed.quality_tier);
    println!("   Quality Score: {:.2}%", seed.quality_score * 100.0);
    println!("   Device: {}", seed.device_used);
    println!("   Device Tier: {}", seed.device_tier);
    println!("   Timestamp: {}", seed.timestamp);
    println!(
        "   Human Input: {}",
        if seed.human_input { "Yes" } else { "No" }
    );
    if let Some(id) = seed.identity {
        println!("   Identity: {id}");
    }

    // Decode entropy bytes
    let entropy_bytes = base64_decode(&seed.entropy_bytes_b64)?;
    println!();
    println!("Entropy Data:");
    println!("   Size: {} bytes", entropy_bytes.len());
    println!(
        "   First 32 bytes (hex): {}",
        hex::encode(&entropy_bytes[..32.min(entropy_bytes.len())])
    );

    Ok(())
}
