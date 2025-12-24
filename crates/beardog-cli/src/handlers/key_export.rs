// Key Export/Import Handler
// Enables inter-primal key sharing (ToadStool integration!)

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::fs;

/// Exported key format for inter-primal sharing
/// This format is designed to be compatible with ToadStool and other primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedKey {
    /// Key identifier
    pub key_id: String,

    /// Algorithm (aes-256-gcm, chacha20-poly1305, ed25519, etc.)
    pub algorithm: String,

    /// Parent key ID (for genetic lineage tracking)
    pub parent: Option<String>,

    /// Generation number (0 = root, 1+ = derived)
    pub generation: u32,

    /// Creation timestamp (RFC3339)
    pub created_at: String,

    /// Derivation context/purpose
    pub context: Option<String>,

    /// Expiry timestamp (RFC3339)
    pub expires_at: Option<String>,

    /// Usage restrictions (encrypt-only, decrypt-only, sign-only, all)
    pub usage: Option<String>,

    /// Key purpose/description
    pub purpose: Option<String>,

    /// Metadata (custom fields)
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,

    /// Key material (base64 encoded, optionally encrypted)
    /// ⚠️ WARNING: Contains sensitive key material!
    /// Should be encrypted when transmitted across networks
    pub key_material: String,

    /// Whether the key_material is encrypted
    #[serde(default)]
    pub encrypted: bool,

    /// Export format version (for future compatibility)
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Handle key export command
pub async fn handle_key_export(
    key_id: &str,
    output_path: &str,
    encrypt: bool,
) -> Result<(), BearDogError> {
    println!("📤 BearDog Key Export");
    println!("====================");
    println!();

    // Load the key from storage
    println!("🔍 Loading key: {}", key_id);
    let stored_key = key_store::load_key(key_id)?;

    println!("✅ Key found");
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   Generation: {}", stored_key.generation);
    if let Some(parent) = &stored_key.parent_key_id {
        println!("   Parent: {}", parent);
    }
    println!();

    // Convert to export format
    let mut exported = ExportedKey {
        key_id: stored_key.key_id.clone(),
        algorithm: stored_key.algorithm.clone(),
        parent: stored_key.parent_key_id.clone(),
        generation: stored_key.generation,
        created_at: stored_key.created_at.clone(),
        context: stored_key.derivation_purpose.clone(),
        expires_at: stored_key.expires_at.clone(),
        usage: stored_key.usage.clone(),
        purpose: stored_key.purpose.clone(),
        metadata: std::collections::HashMap::new(),
        key_material: stored_key.key_material_b64.clone(),
        encrypted: false,
        version: "1.0".to_string(),
    };

    // Add metadata
    exported
        .metadata
        .insert("hsm_name".to_string(), stored_key.hsm_name.clone());
    exported
        .metadata
        .insert("exported_at".to_string(), chrono::Utc::now().to_rfc3339());
    exported
        .metadata
        .insert("exported_by".to_string(), "beardog-cli".to_string());

    // Optionally encrypt the key material
    if encrypt {
        println!("🔐 Encrypting key material...");
        println!("⚠️  You will need to provide a password");
        println!();

        // Get password from user
        let password = rpassword::prompt_password("Enter encryption password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {}", e)))?;

        if password.is_empty() {
            return Err(BearDogError::validation("Password cannot be empty"));
        }

        // Confirm password
        let password_confirm = rpassword::prompt_password("Confirm password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {}", e)))?;

        if password != password_confirm {
            return Err(BearDogError::validation("Passwords do not match"));
        }

        // Encrypt the key material using Argon2 + ChaCha20-Poly1305
        let encrypted_material = encrypt_key_material(&exported.key_material, &password)?;
        exported.key_material = encrypted_material;
        exported.encrypted = true;

        println!("✅ Key material encrypted");
        println!();
    } else {
        println!("⚠️  WARNING: Key material will be exported in plaintext!");
        println!("   Consider using --encrypt flag for secure transmission");
        println!();
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&exported)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;

    // Write to file
    fs::write(output_path, json)?;

    println!("✅ Key exported successfully!");
    println!();
    println!("📋 Export Details:");
    println!("   File: {}", output_path);
    println!("   Key ID: {}", exported.key_id);
    println!("   Algorithm: {}", exported.algorithm);
    println!(
        "   Encrypted: {}",
        if exported.encrypted {
            "Yes"
        } else {
            "No ⚠️"
        }
    );
    println!("   Version: {}", exported.version);
    println!();

    if !exported.encrypted {
        println!("⚠️  SECURITY WARNING:");
        println!("   This key is exported in plaintext!");
        println!("   Protect this file and delete it after transmission.");
        println!("   Use --encrypt flag for secure inter-primal sharing.");
        println!();
    }

    println!("💡 Next steps:");
    println!(
        "   1. Securely transmit {} to the destination tower",
        output_path
    );
    println!(
        "   2. Import on the other tower: beardog key import --input {}",
        output_path
    );
    if exported.encrypted {
        println!("   3. Provide the same password during import");
    }
    println!();

    Ok(())
}

/// Handle key import command
pub async fn handle_key_import(
    input_path: &str,
    key_id_override: Option<&str>,
    decrypt: bool,
) -> Result<(), BearDogError> {
    println!("📥 BearDog Key Import");
    println!("====================");
    println!();

    // Read the exported key file
    println!("🔍 Reading key file: {}", input_path);
    let json = fs::read_to_string(input_path)?;

    let mut exported: ExportedKey = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("Invalid key file format: {}", e)))?;

    println!("✅ Key file loaded");
    println!("   Key ID: {}", exported.key_id);
    println!("   Algorithm: {}", exported.algorithm);
    println!("   Generation: {}", exported.generation);
    println!(
        "   Encrypted: {}",
        if exported.encrypted { "Yes" } else { "No" }
    );
    println!("   Version: {}", exported.version);
    println!();

    // Check version compatibility
    if exported.version != "1.0" {
        println!(
            "⚠️  WARNING: Key export version {} may not be fully compatible",
            exported.version
        );
        println!("   Current version: 1.0");
        println!();
    }

    // Decrypt if needed
    if exported.encrypted {
        if !decrypt {
            return Err(BearDogError::validation(
                "Key is encrypted but --decrypt flag not provided. Use --decrypt and provide password."
            ));
        }

        println!("🔐 Decrypting key material...");
        let password = rpassword::prompt_password("Enter decryption password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {}", e)))?;

        if password.is_empty() {
            return Err(BearDogError::validation("Password cannot be empty"));
        }

        // Decrypt the key material
        let decrypted_material = decrypt_key_material(&exported.key_material, &password)?;
        exported.key_material = decrypted_material;
        exported.encrypted = false;

        println!("✅ Key material decrypted");
        println!();
    }

    // Use override key_id if provided
    let final_key_id = key_id_override.unwrap_or(&exported.key_id);

    // Check if key already exists
    if key_store::load_key(final_key_id).is_ok() {
        println!("⚠️  WARNING: Key '{}' already exists!", final_key_id);
        println!("   Import will overwrite the existing key.");
        println!();

        print!("Continue? [y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("❌ Import cancelled");
            return Ok(());
        }
        println!();
    }

    // Convert to StoredKey format
    let stored_key = StoredKey {
        key_id: final_key_id.to_string(),
        algorithm: exported.algorithm.clone(),
        hsm_name: exported
            .metadata
            .get("hsm_name")
            .cloned()
            .unwrap_or_else(|| "imported".to_string()),
        created_at: exported.created_at.clone(),
        key_material_b64: exported.key_material.clone(),
        generation: exported.generation,
        parent_key_id: exported.parent.clone(),
        derivation_purpose: exported.context.clone(),
        children: Vec::new(), // Will be rebuilt as keys are derived
        lineage: None,        // Imported keys don't have lineage info initially
        expires_at: exported.expires_at.clone(),
        usage: exported.usage.clone(),
        purpose: exported.purpose.clone(),
    };

    // Save to key store
    key_store::save_key(&stored_key)?;

    println!("✅ Key imported successfully!");
    println!();
    println!("📋 Import Details:");
    println!("   Key ID: {}", final_key_id);
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   Generation: {}", stored_key.generation);
    if let Some(parent) = &stored_key.parent_key_id {
        println!("   Parent: {}", parent);
    }
    if let Some(expires) = &stored_key.expires_at {
        println!("   Expires: {}", expires);
    }
    println!();

    println!("💡 Next steps:");
    println!(
        "   1. Verify key: beardog key info --key-id {}",
        final_key_id
    );
    println!(
        "   2. Use key: beardog encrypt --key {} --input data.txt --output data.enc",
        final_key_id
    );
    if exported.parent.is_some() {
        println!(
            "   3. Check lineage: beardog key lineage --key-id {}",
            final_key_id
        );
    }
    println!();

    Ok(())
}

/// Encrypt key material using Argon2 + ChaCha20-Poly1305
fn encrypt_key_material(key_material_b64: &str, password: &str) -> Result<String, BearDogError> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};
    use chacha20poly1305::{
        aead::{Aead, KeyInit, OsRng as ChaChaRng},
        ChaCha20Poly1305, Nonce,
    };

    // Generate salt for Argon2
    let salt = SaltString::generate(&mut OsRng);

    // Derive encryption key from password using Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BearDogError::Cryptographic {
            message: format!("Argon2 key derivation failed: {}", e),
        })?;

    // Extract the hash bytes (32 bytes for ChaCha20-Poly1305)
    let hash_bytes = password_hash
        .hash
        .ok_or_else(|| BearDogError::Cryptographic {
            message: "Failed to extract Argon2 hash".to_string(),
        })?;

    let key_bytes: [u8; 32] =
        hash_bytes.as_bytes()[..32]
            .try_into()
            .map_err(|_| BearDogError::Cryptographic {
                message: "Invalid key length from Argon2".to_string(),
            })?;

    // Create ChaCha20-Poly1305 cipher
    let cipher = ChaCha20Poly1305::new(&key_bytes.into());

    // Generate random nonce
    let mut rng = ChaChaRng;
    let nonce_bytes: [u8; 12] = rand::Rng::gen(&mut rng);
    let nonce = Nonce::from(nonce_bytes);

    // Encrypt the key material
    let plaintext = STANDARD
        .decode(key_material_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid base64: {}", e)))?;

    let ciphertext =
        cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|e| BearDogError::Cryptographic {
                message: format!("ChaCha20-Poly1305 encryption failed: {}", e),
            })?;

    // Package: salt || nonce || ciphertext (all base64 encoded)
    let package = serde_json::json!({
        "salt": salt.as_str(),
        "nonce": STANDARD.encode(nonce_bytes),
        "ciphertext": STANDARD.encode(ciphertext),
    });

    Ok(package.to_string())
}

/// Decrypt key material using Argon2 + ChaCha20-Poly1305
fn decrypt_key_material(encrypted_package: &str, password: &str) -> Result<String, BearDogError> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};
    use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit, Nonce};

    // Parse the encrypted package
    let package: serde_json::Value = serde_json::from_str(encrypted_package)
        .map_err(|e| BearDogError::serialization(&format!("Invalid encrypted package: {}", e)))?;

    let salt_str = package["salt"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing salt in encrypted package"))?;
    let nonce_b64 = package["nonce"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing nonce in encrypted package"))?;
    let ciphertext_b64 = package["ciphertext"]
        .as_str()
        .ok_or_else(|| BearDogError::serialization("Missing ciphertext in encrypted package"))?;

    // Reconstruct salt
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| BearDogError::serialization(&format!("Invalid salt: {}", e)))?;

    // Derive encryption key from password using Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BearDogError::Cryptographic {
            message: format!("Argon2 key derivation failed (wrong password?): {}", e),
        })?;

    let hash_bytes = password_hash
        .hash
        .ok_or_else(|| BearDogError::Cryptographic {
            message: "Failed to extract Argon2 hash".to_string(),
        })?;

    let key_bytes: [u8; 32] =
        hash_bytes.as_bytes()[..32]
            .try_into()
            .map_err(|_| BearDogError::Cryptographic {
                message: "Invalid key length from Argon2".to_string(),
            })?;

    // Create ChaCha20-Poly1305 cipher
    let cipher = ChaCha20Poly1305::new(&key_bytes.into());

    // Decode nonce and ciphertext
    let nonce_bytes = STANDARD
        .decode(nonce_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid nonce: {}", e)))?;
    let nonce: [u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| BearDogError::serialization("Invalid nonce length"))?;
    let nonce = Nonce::from(nonce);

    let ciphertext = STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| BearDogError::serialization(&format!("Invalid ciphertext: {}", e)))?;

    // Decrypt
    let plaintext =
        cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| BearDogError::Cryptographic {
                message: "Decryption failed (wrong password or corrupted data)".to_string(),
            })?;

    // Encode back to base64
    Ok(STANDARD.encode(plaintext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_key_material() {
        let original = "SGVsbG8gV29ybGQh"; // "Hello World!" in base64
        let password = "test-password-123";

        // Encrypt
        let encrypted = encrypt_key_material(original, password).unwrap();
        assert_ne!(encrypted, original);
        assert!(encrypted.contains("salt"));
        assert!(encrypted.contains("nonce"));
        assert!(encrypted.contains("ciphertext"));

        // Decrypt
        let decrypted = decrypt_key_material(&encrypted, password).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_decrypt_wrong_password_fails() {
        let original = "SGVsbG8gV29ybGQh";
        let password = "correct-password";
        let wrong_password = "wrong-password";

        let encrypted = encrypt_key_material(original, password).unwrap();
        let result = decrypt_key_material(&encrypted, wrong_password);

        assert!(result.is_err());
    }

    #[test]
    fn test_exported_key_serialization() {
        let exported = ExportedKey {
            key_id: "test-key".to_string(),
            algorithm: "aes-256-gcm".to_string(),
            parent: Some("master-key".to_string()),
            generation: 1,
            created_at: "2025-12-18T00:00:00Z".to_string(),
            context: Some("student-1".to_string()),
            expires_at: None,
            usage: Some("all".to_string()),
            purpose: Some("testing".to_string()),
            metadata: std::collections::HashMap::new(),
            key_material: "base64encodedkey".to_string(),
            encrypted: false,
            version: "1.0".to_string(),
        };

        let json = serde_json::to_string(&exported).unwrap();
        let deserialized: ExportedKey = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.key_id, "test-key");
        assert_eq!(deserialized.algorithm, "aes-256-gcm");
        assert_eq!(deserialized.generation, 1);
    }
}
