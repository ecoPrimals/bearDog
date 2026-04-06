// SPDX-License-Identifier: AGPL-3.0-or-later

//! Encrypt plaintext files (or stdin) using a stored key and the software HSM tunnel.

use super::key_store;
use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::HsmProvider;
use beardog_tunnel::tunnel::hsm::SoftwareHsm;
use beardog_tunnel::tunnel::hsm::types::config::SoftwareHsmConfig;
use std::fs;
use std::sync::Arc;

/// Handle encryption command
///
/// # Errors
///
/// Returns an error if the key cannot be loaded, I/O on stdin/file fails, encryption via the HSM
/// fails, or the output cannot be written.
pub async fn handle_encrypt(
    key_id: &str,
    input_path: &str,
    output_path: &str,
    use_genetic: bool,
) -> Result<(), BearDogError> {
    // Check for stdin/stdout mode
    let use_stdin = input_path == "-";
    let use_stdout = output_path == "-";

    if !use_stdin && !use_stdout {
        println!("🔒 BearDog Encryption");
        println!("====================");
        println!();
    }

    // Step 1: Load key metadata (vendor-agnostic)
    if !use_stdin && !use_stdout {
        println!("🔑 Loading key: {key_id}");
    }
    let stored_key = key_store::load_key(key_id)?;
    if !use_stdin && !use_stdout {
        println!("✅ Key loaded");
        println!("   Algorithm: {}", stored_key.algorithm);
        println!("   HSM: {}", stored_key.hsm_name);
        println!();
    }

    // Step 2: Read input (file or stdin)
    let plaintext = if use_stdin {
        // Read from stdin
        use std::io::{self, Read};
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        if !use_stdout {
            println!("📂 Reading input file: {input_path}");
        }
        let data = fs::read(input_path)?;
        if !use_stdout {
            println!("   Size: {} bytes", data.len());
            println!();
        }
        data
    };

    // Step 3: Initialize HSM provider
    if !use_stdin && !use_stdout {
        println!("🔐 Initializing encryption...");
        if use_genetic {
            println!("   Mode: Genetic algorithm (adaptive)");
        } else {
            println!("   Mode: Standard algorithm");
        }
    }

    // Initialize Software HSM with default config (vendor-agnostic crypto provider)
    let config = SoftwareHsmConfig::default();
    let software_hsm: Arc<dyn HsmProvider> = Arc::new(SoftwareHsm::new(config).await?);

    // Import the key into HSM for operation
    let key_material = key_store::base64_decode(&stored_key.key_material_b64)?;
    software_hsm.import_key(&key_material, key_id).await?;

    // Encrypt using real HSM provider
    let ciphertext = software_hsm.encrypt(key_id, &plaintext).await?;

    if !use_stdin && !use_stdout {
        println!("✅ Encryption complete");
        println!("   Input: {} bytes", plaintext.len());
        println!("   Output: {} bytes", ciphertext.len());
        println!("   Overhead: {} bytes", ciphertext.len() - plaintext.len());
        println!();
    }

    // Step 4: Write output (file or stdout)
    if use_stdout {
        // Write to stdout
        use std::io::{self, Write};
        io::stdout().write_all(&ciphertext)?;
        io::stdout().flush()?;
    } else {
        println!("💾 Writing encrypted file: {output_path}");
        fs::write(output_path, ciphertext)?;
        println!("✅ Saved successfully");
        println!();

        println!("💡 Next steps:");
        println!(
            "   • Decrypt: beardog decrypt --key {key_id} --input {output_path} --output data-decrypted.txt"
        );
    }

    Ok(())
}
