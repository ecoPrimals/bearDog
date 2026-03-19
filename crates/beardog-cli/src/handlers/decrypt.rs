// SPDX-License-Identifier: AGPL-3.0-only

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
    // Check for stdin/stdout mode
    let use_stdin = input_path == "-";
    let use_stdout = output_path == "-";

    if !use_stdin && !use_stdout {
        println!("🔓 BearDog Decryption");
        println!("====================");
        println!();
    }

    // Step 1: Load key metadata (vendor-agnostic)
    if !use_stdin && !use_stdout {
        println!("🔑 Loading key: {}", key_id);
    }
    let stored_key = key_store::load_key(key_id)?;
    if !use_stdin && !use_stdout {
        println!("✅ Key loaded");
        println!("   Algorithm: {}", stored_key.algorithm);
        println!("   HSM: {}", stored_key.hsm_name);
        println!();
    }

    // Step 2: Read encrypted input (file or stdin)
    let ciphertext = if use_stdin {
        // Read from stdin
        use std::io::{self, Read};
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        if !use_stdout {
            println!("📂 Reading encrypted file: {}", input_path);
        }
        let data = fs::read(input_path)?;
        if !use_stdout {
            println!("   Size: {} bytes", data.len());
            println!();
        }
        data
    };

    // Step 3: Initialize HSM provider and decrypt
    if !use_stdin && !use_stdout {
        println!("🔓 Decrypting...");
    }

    // Initialize Software HSM with default config (vendor-agnostic crypto provider)
    let config = SoftwareHsmConfig::default();
    let software_hsm: Arc<dyn HsmProvider> = Arc::new(SoftwareHsm::new(config).await?);

    // Import the key into HSM for operation
    let key_material = key_store::base64_decode(&stored_key.key_material_b64)?;
    software_hsm.import_key(&key_material, key_id).await?;

    // Decrypt using real HSM provider
    let plaintext = software_hsm.decrypt(key_id, &ciphertext).await?;

    if !use_stdin && !use_stdout {
        println!("✅ Decryption complete");
        println!("   Input: {} bytes", ciphertext.len());
        println!("   Output: {} bytes", plaintext.len());
        println!();
    }

    // Step 4: Write output (file or stdout)
    if use_stdout {
        // Write to stdout
        use std::io::{self, Write};
        io::stdout().write_all(&plaintext)?;
        io::stdout().flush()?;
    } else {
        println!("💾 Writing decrypted file: {}", output_path);
        fs::write(output_path, plaintext)?;
        println!("✅ Saved successfully");
    }

    Ok(())
}
