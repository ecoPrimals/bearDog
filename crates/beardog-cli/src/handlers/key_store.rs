// Key Storage Module
// Simple JSON-based key persistence for CLI operations
// NOTE: This is a simplified version for testing. Production should use HSM-backed storage.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Key metadata stored in ~/.beardog/keys/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredKey {
    pub key_id: String,
    pub algorithm: String,
    pub hsm_name: String,
    pub created_at: String,
    /// Base64-encoded key material (simplified for testing)
    /// In production, this would be an HSM reference/handle
    pub key_material_b64: String,
}

/// Get the keys directory path
pub fn get_keys_dir() -> Result<PathBuf, BearDogError> {
    let home = std::env::var("HOME")
        .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))?;

    let keys_dir = PathBuf::from(home).join(".beardog").join("keys");

    // Create directory if it doesn't exist
    if !keys_dir.exists() {
        fs::create_dir_all(&keys_dir)?;
    }

    Ok(keys_dir)
}

/// Save a key to storage
pub fn save_key(key: &StoredKey) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir()?;
    let key_file = keys_dir.join(format!("{}.json", key.key_id));

    let json = serde_json::to_string_pretty(key)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;

    fs::write(key_file, json)?;
    Ok(())
}

/// Load a key from storage
pub fn load_key(key_id: &str) -> Result<StoredKey, BearDogError> {
    let keys_dir = get_keys_dir()?;
    let key_file = keys_dir.join(format!("{}.json", key_id));

    if !key_file.exists() {
        return Err(BearDogError::not_found(format!(
            "Key '{}' not found. Use 'beardog key generate' to create it.",
            key_id
        )));
    }

    let json = fs::read_to_string(key_file)?;
    let key: StoredKey =
        serde_json::from_str(&json).map_err(|e| BearDogError::serialization(&e.to_string()))?;

    Ok(key)
}

/// List all stored keys
pub fn list_keys() -> Result<Vec<StoredKey>, BearDogError> {
    let keys_dir = get_keys_dir()?;

    if !keys_dir.exists() {
        return Ok(Vec::new());
    }

    let mut keys = Vec::new();

    for entry in fs::read_dir(keys_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(key) = serde_json::from_str::<StoredKey>(&json) {
                    keys.push(key);
                }
            }
        }
    }

    Ok(keys)
}

/// Delete a key from storage
pub fn delete_key(key_id: &str) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir()?;
    let key_file = keys_dir.join(format!("{}.json", key_id));

    if !key_file.exists() {
        return Err(BearDogError::not_found(format!(
            "Key '{}' not found",
            key_id
        )));
    }

    fs::remove_file(key_file)?;
    Ok(())
}

/// Encode bytes to base64
pub fn base64_encode(data: &[u8]) -> String {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    STANDARD.encode(data)
}

/// Decode base64 to bytes
pub fn base64_decode(data: &str) -> Result<Vec<u8>, BearDogError> {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    STANDARD
        .decode(data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))
}
