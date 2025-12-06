// Decrypt Handler
// Algorithm-agnostic: Automatically detects encryption algorithm from metadata

use super::key_store;
use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::types::config::SoftwareHsmConfig;
use beardog_tunnel::tunnel::hsm::HsmProvider;
use beardog_tunnel::tunnel::hsm::SoftwareHsm;
use std::fs;
use std::sync::Arc;

/// Handle decryption command
pub async fn handle_decrypt(
    key_id: &str,
    input_path: &str,
    output_path: &str,
) -> Result<(), BearDogError> {
    println!("🔓 BearDog Decryption");
    println!("====================");
    println!();

    // Step 1: Load key metadata (vendor-agnostic)
    println!("🔑 Loading key: {}", key_id);
    let stored_key = key_store::load_key(key_id)?;
    println!("✅ Key loaded");
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   HSM: {}", stored_key.hsm_name);
    println!();

    // Step 2: Read encrypted file
    println!("📂 Reading encrypted file: {}", input_path);
    let ciphertext = fs::read(input_path)?;
    println!("   Size: {} bytes", ciphertext.len());
    println!();

    // Step 3: Initialize HSM provider and decrypt
    println!("🔓 Decrypting...");

    // Initialize Software HSM with default config (vendor-agnostic crypto provider)
    let config = SoftwareHsmConfig::default();
    let software_hsm: Arc<dyn HsmProvider> = Arc::new(SoftwareHsm::new(config).await?);

    // Import the key into HSM for operation
    let key_material = key_store::base64_decode(&stored_key.key_material_b64)?;
    software_hsm.import_key(&key_material, key_id).await?;

    // Decrypt using real HSM provider
    let plaintext = software_hsm.decrypt(key_id, &ciphertext).await?;

    println!("✅ Decryption complete");
    println!("   Input: {} bytes", ciphertext.len());
    println!("   Output: {} bytes", plaintext.len());
    println!();

    // Step 4: Write output file
    println!("💾 Writing decrypted file: {}", output_path);
    fs::write(output_path, plaintext)?;
    println!("✅ Saved successfully");

    Ok(())
}
