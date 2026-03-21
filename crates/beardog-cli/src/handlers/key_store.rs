// SPDX-License-Identifier: AGPL-3.0-only

//! Local JSON key store under `~/.beardog/keys` (CLI testing and development).

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Lineage information for a key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyLineageInfo {
    /// Immediate parent key id, if any
    pub parent_key_id: Option<String>,
    /// Depth from the lineage root (0 = root)
    pub depth: u32,
}

/// Key metadata stored in ~/.beardog/keys/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredKey {
    /// User-defined key identifier (filename stem)
    pub key_id: String,
    /// Algorithm name (e.g. `aes256-gcm`)
    pub algorithm: String,
    /// Label of the HSM used when the key was created
    pub hsm_name: String,
    /// Creation timestamp (RFC 3339)
    pub created_at: String,
    /// Base64-encoded key material (simplified for testing)
    /// In production, this would be an HSM reference/handle
    pub key_material_b64: String,

    // Lineage tracking
    /// Generation number (0 = root, 1 = derived, etc.)
    #[serde(default)]
    pub generation: u32,
    /// Parent key ID if this is a derived key
    #[serde(default)]
    pub parent_key_id: Option<String>,
    /// Purpose of derivation
    #[serde(default)]
    pub derivation_purpose: Option<String>,
    /// Child key IDs derived from this key
    #[serde(default)]
    pub children: Vec<String>,
    /// Lineage information (for BirdSong encryption)
    #[serde(default)]
    pub lineage: Option<KeyLineageInfo>,

    // Usage restrictions
    /// Expiry timestamp
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Usage restrictions (e.g., "encrypt-only", "decrypt-only")
    #[serde(default)]
    pub usage: Option<String>,
    /// Purpose/description of the key
    #[serde(default)]
    pub purpose: Option<String>,
}

/// Keys directory under a given home (testable).
pub fn get_keys_dir_for_home(home: impl AsRef<std::path::Path>) -> Result<PathBuf, BearDogError> {
    let keys_dir = home.as_ref().join(".beardog").join("keys");
    if !keys_dir.exists() {
        fs::create_dir_all(&keys_dir)?;
    }
    Ok(keys_dir)
}

/// Home directory used for `get_keys_dir()` (same as `HOME` env).
pub fn home_dir_for_keys() -> Result<PathBuf, BearDogError> {
    std::env::var("HOME")
        .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))
        .map(PathBuf::from)
}

/// Get the keys directory path
pub fn get_keys_dir() -> Result<PathBuf, BearDogError> {
    let home = home_dir_for_keys()?;
    get_keys_dir_for_home(home)
}

/// Save a key to storage under a specific home directory (tests).
pub fn save_key_to_home(
    key: &StoredKey,
    home: impl AsRef<std::path::Path>,
) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir_for_home(home)?;
    save_key_to_dir(key, &keys_dir)
}

fn save_key_to_dir(key: &StoredKey, keys_dir: &std::path::Path) -> Result<(), BearDogError> {
    let key_file = keys_dir.join(format!("{}.json", key.key_id));

    let json = serde_json::to_string_pretty(key)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;

    fs::write(key_file, json)?;
    Ok(())
}

/// Save a key to storage
pub fn save_key(key: &StoredKey) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir()?;
    save_key_to_dir(key, &keys_dir)
}

/// Load a key from storage under a specific home directory (tests).
pub fn load_key_from_home(
    key_id: &str,
    home: impl AsRef<std::path::Path>,
) -> Result<StoredKey, BearDogError> {
    let keys_dir = get_keys_dir_for_home(home)?;
    load_key_from_dir(key_id, &keys_dir)
}

fn load_key_from_dir(key_id: &str, keys_dir: &std::path::Path) -> Result<StoredKey, BearDogError> {
    let key_file = keys_dir.join(format!("{key_id}.json"));

    if !key_file.exists() {
        return Err(BearDogError::not_found(format!(
            "Key '{key_id}' not found. Use 'beardog key generate' to create it."
        )));
    }

    let json = fs::read_to_string(key_file)?;
    let key: StoredKey =
        serde_json::from_str(&json).map_err(|e| BearDogError::serialization(&e.to_string()))?;

    Ok(key)
}

/// Load a key from storage
pub fn load_key(key_id: &str) -> Result<StoredKey, BearDogError> {
    let keys_dir = get_keys_dir()?;
    load_key_from_dir(key_id, &keys_dir)
}

/// List all stored keys
// Public API for callers using default HOME; CLI handlers use `list_keys_from_home` for DI.
#[allow(
    dead_code,
    reason = "Stable HOME-based API; handlers use list_keys_from_home for DI."
)]
pub fn list_keys() -> Result<Vec<StoredKey>, BearDogError> {
    let keys_dir = get_keys_dir()?;
    list_keys_in_dir(&keys_dir)
}

/// List keys under a specific home directory (tests / DI).
pub fn list_keys_from_home(
    home: impl AsRef<std::path::Path>,
) -> Result<Vec<StoredKey>, BearDogError> {
    let keys_dir = get_keys_dir_for_home(home)?;
    list_keys_in_dir(&keys_dir)
}

fn list_keys_in_dir(keys_dir: &std::path::Path) -> Result<Vec<StoredKey>, BearDogError> {
    if !keys_dir.exists() {
        return Ok(Vec::new());
    }

    let mut keys = Vec::new();

    for entry in fs::read_dir(keys_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json")
            && let Ok(json) = fs::read_to_string(&path)
            && let Ok(key) = serde_json::from_str::<StoredKey>(&json)
        {
            keys.push(key);
        }
    }

    Ok(keys)
}

/// Delete a key from storage
// Public API for callers using default HOME; CLI handlers use `delete_key_from_home` for DI.
#[allow(
    dead_code,
    reason = "Stable HOME-based API; handlers use delete_key_from_home for DI."
)]
pub fn delete_key(key_id: &str) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir()?;
    delete_key_in_dir(key_id, &keys_dir)
}

/// Delete a key under a specific home directory (tests / DI).
pub fn delete_key_from_home(
    key_id: &str,
    home: impl AsRef<std::path::Path>,
) -> Result<(), BearDogError> {
    let keys_dir = get_keys_dir_for_home(home)?;
    delete_key_in_dir(key_id, &keys_dir)
}

fn delete_key_in_dir(key_id: &str, keys_dir: &std::path::Path) -> Result<(), BearDogError> {
    let key_file = keys_dir.join(format!("{key_id}.json"));

    if !key_file.exists() {
        return Err(BearDogError::not_found(format!("Key '{key_id}' not found")));
    }

    fs::remove_file(key_file)?;
    Ok(())
}

/// Encode bytes to base64
pub fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    STANDARD.encode(data)
}

/// Decode base64 to bytes
pub fn base64_decode(data: &str) -> Result<Vec<u8>, BearDogError> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    STANDARD
        .decode(data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))
}
