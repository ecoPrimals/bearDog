// Encrypt Handler
// Algorithm-agnostic: Works with ANY encryption algorithm

use super::key_store;
use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::types::config::SoftwareHsmConfig;
use beardog_tunnel::tunnel::hsm::HsmProvider;
use beardog_tunnel::tunnel::hsm::SoftwareHsm;
use std::fs;
use std::sync::Arc;

/// Handle encryption command
pub async fn handle_encrypt(
    key_id: &str,
    input_path: &str,
    output_path: &str,
    use_genetic: bool,
) -> Result<(), BearDogError> {
    println!("🔒 BearDog Encryption");
    println!("====================");
    println!();

    // Step 1: Load key metadata (vendor-agnostic)
    println!("🔑 Loading key: {}", key_id);
    let stored_key = key_store::load_key(key_id)?;
    println!("✅ Key loaded");
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   HSM: {}", stored_key.hsm_name);
    println!();

    // Step 2: Read input file
    println!("📂 Reading input file: {}", input_path);
    let plaintext = fs::read(input_path)?;
    println!("   Size: {} bytes", plaintext.len());
    println!();

    // Step 3: Initialize HSM provider
    println!("🔐 Initializing encryption...");
    if use_genetic {
        println!("   Mode: Genetic algorithm (adaptive)");
    } else {
        println!("   Mode: Standard algorithm");
    }

    // Initialize Software HSM with default config (vendor-agnostic crypto provider)
    let config = SoftwareHsmConfig::default();
    let software_hsm: Arc<dyn HsmProvider> = Arc::new(SoftwareHsm::new(config).await?);

    // Import the key into HSM for operation
    let key_material = key_store::base64_decode(&stored_key.key_material_b64)?;
    software_hsm.import_key(&key_material, key_id).await?;

    // Encrypt using real HSM provider
    let ciphertext = software_hsm.encrypt(key_id, &plaintext).await?;

    println!("✅ Encryption complete");
    println!("   Input: {} bytes", plaintext.len());
    println!("   Output: {} bytes", ciphertext.len());
    println!("   Overhead: {} bytes", ciphertext.len() - plaintext.len());
    println!();

    // Step 4: Write output file
    println!("💾 Writing encrypted file: {}", output_path);
    fs::write(output_path, ciphertext)?;
    println!("✅ Saved successfully");
    println!();

    println!("💡 Next steps:");
    println!(
        "   • Decrypt: beardog decrypt --key {} --input {} --output data-decrypted.txt",
        key_id, output_path
    );

    Ok(())
}
