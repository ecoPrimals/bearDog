//! Dark Forest Beacon RPC Handlers
//!
//! Implements beacon.* JSON-RPC methods for Dark Forest discovery.
//!
//! ## Methods
//!
//! - `beacon.generate` - Generate new beacon seed
//! - `beacon.get_id` - Get public beacon ID  
//! - `beacon.encrypt` - Encrypt data with beacon seed
//! - `beacon.try_decrypt` - Try to decrypt with our beacon seed
//! - `beacon.try_decrypt_any` - Try to decrypt with any known beacon
//! - `beacon.list_known` - List known beacon IDs (meetings)
//! - `beacon.add_known` - Add known beacon (meeting exchange)
//!
//! ## Deep Debt Alignment
//!
//! - **Principle #1**: Pure Rust (ChaCha20-Poly1305, HKDF, BLAKE3)
//! - **Principle #2**: Separation of concerns (beacon ≠ lineage)
//! - **Principle #5**: Runtime discovery (meeting exchange)
//! - **Principle #6**: Production crypto (real AEAD, no mocks)

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BeaconCiphertext, BeaconSeed};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Beacon manager for Dark Forest discovery
///
/// Manages beacon seeds and known beacons (meetings).
pub struct BeaconManager {
    /// Our beacon seed (for encrypting broadcasts)
    our_beacon: Arc<RwLock<Option<BeaconSeed>>>,

    /// Known beacon seeds (from meetings)
    /// Key: BeaconId (hex), Value: BeaconSeed
    known_beacons: Arc<RwLock<HashMap<String, BeaconSeed>>>,
}

impl BeaconManager {
    /// Create new beacon manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            our_beacon: Arc::new(RwLock::new(None)),
            known_beacons: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize with beacon seed (from env or derivation)
    pub async fn initialize(&self, beacon: BeaconSeed) -> Result<(), BearDogError> {
        let mut our_beacon = self.our_beacon.write().await;
        *our_beacon = Some(beacon);
        info!("🔒 Beacon seed initialized");
        Ok(())
    }

    /// Get our beacon (or generate if not set)
    async fn get_or_create_beacon(&self) -> Result<BeaconSeed, BearDogError> {
        let mut our_beacon = self.our_beacon.write().await;

        if let Some(ref beacon) = *our_beacon {
            return Ok(beacon.clone());
        }

        // Generate new beacon if none exists
        let beacon = BeaconSeed::generate();
        info!("🌱 Generated new beacon seed: {}", beacon.id().to_hex());
        *our_beacon = Some(beacon.clone());

        Ok(beacon)
    }

    /// Add known beacon from meeting
    pub async fn add_known_beacon(&self, beacon: BeaconSeed) -> Result<(), BearDogError> {
        let id_hex = beacon.id().to_hex();
        let mut known = self.known_beacons.write().await;
        known.insert(id_hex.clone(), beacon);
        info!("🤝 Added known beacon: {}", id_hex);
        Ok(())
    }
}

impl Default for BeaconManager {
    fn default() -> Self {
        Self::new()
    }
}

/// beacon.generate - Generate new beacon seed
///
/// Returns the public beacon ID (safe to share).
pub async fn handle_beacon_generate(
    beacon_manager: Arc<BeaconManager>,
    _params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🌱 RPC: beacon.generate");

    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "beacon_id": beacon.id().to_hex(),
        "status": "generated"
    }))
}

/// beacon.get_id - Get our public beacon ID
pub async fn handle_beacon_get_id(
    beacon_manager: Arc<BeaconManager>,
    _params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔍 RPC: beacon.get_id");

    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "beacon_id": beacon.id().to_hex()
    }))
}

/// beacon.encrypt - Encrypt data with our beacon seed
///
/// # Request
/// ```json
/// {
///   "plaintext": "<base64 data to encrypt>"
/// }
/// ```
///
/// # Response
/// ```json
/// {
///   "ciphertext": "<base64>",
///   "nonce": "<base64>",
///   "timestamp": 1234567890
/// }
/// ```
pub async fn handle_beacon_encrypt(
    beacon_manager: Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔒 RPC: beacon.encrypt");

    let params = params.ok_or("Missing params")?;
    let plaintext_b64 = params["plaintext"]
        .as_str()
        .ok_or("Missing 'plaintext' field")?;

    let plaintext = base64::decode(plaintext_b64).map_err(|e| format!("Invalid base64: {}", e))?;

    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    let encrypted = beacon.encrypt(&plaintext).map_err(|e| e.to_string())?;

    Ok(json!({
        "ciphertext": base64::encode(&encrypted.ciphertext),
        "nonce": base64::encode(&encrypted.nonce),
        "timestamp": encrypted.timestamp
    }))
}

/// beacon.try_decrypt - Try to decrypt with our beacon seed
///
/// # Request
/// ```json
/// {
///   "ciphertext": "<base64>",
///   "nonce": "<base64>",
///   "timestamp": 1234567890
/// }
/// ```
///
/// # Response (Success)
/// ```json
/// {
///   "decrypted": true,
///   "plaintext": "<base64>"
/// }
/// ```
///
/// # Response (Different beacon family)
/// ```json
/// {
///   "decrypted": false
/// }
/// ```
pub async fn handle_beacon_try_decrypt(
    beacon_manager: Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔓 RPC: beacon.try_decrypt");

    let params = params.ok_or("Missing params")?;

    let ciphertext = base64::decode(
        params["ciphertext"]
            .as_str()
            .ok_or("Missing 'ciphertext' field")?,
    )
    .map_err(|e| format!("Invalid base64: {}", e))?;

    let nonce_vec = base64::decode(params["nonce"].as_str().ok_or("Missing 'nonce' field")?)
        .map_err(|e| format!("Invalid nonce base64: {}", e))?;

    if nonce_vec.len() != 12 {
        return Err(format!("Nonce must be 12 bytes, got {}", nonce_vec.len()));
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&nonce_vec);

    let timestamp = params["timestamp"]
        .as_u64()
        .ok_or("Missing 'timestamp' field")?;

    let encrypted = BeaconCiphertext {
        ciphertext,
        nonce,
        timestamp,
    };

    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    match beacon.try_decrypt(&encrypted).map_err(|e| e.to_string())? {
        Some(plaintext) => Ok(json!({
            "decrypted": true,
            "plaintext": base64::encode(&plaintext)
        })),
        None => Ok(json!({
            "decrypted": false
        })),
    }
}

/// beacon.try_decrypt_any - Try to decrypt with any known beacon seed
///
/// Iterates through all known beacons (from meetings) and tries to decrypt.
/// Returns the first successful decryption with the matching beacon ID.
///
/// # Response (Success)
/// ```json
/// {
///   "decrypted": true,
///   "plaintext": "<base64>",
///   "matched_beacon_id": "<hex>"
/// }
/// ```
pub async fn handle_beacon_try_decrypt_any(
    beacon_manager: Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔍 RPC: beacon.try_decrypt_any");

    let params = params.ok_or("Missing params")?;

    let ciphertext = base64::decode(
        params["ciphertext"]
            .as_str()
            .ok_or("Missing 'ciphertext' field")?,
    )
    .map_err(|e| format!("Invalid base64: {}", e))?;

    let nonce_vec = base64::decode(params["nonce"].as_str().ok_or("Missing 'nonce' field")?)
        .map_err(|e| format!("Invalid nonce base64: {}", e))?;

    if nonce_vec.len() != 12 {
        return Err(format!("Nonce must be 12 bytes, got {}", nonce_vec.len()));
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&nonce_vec);

    let timestamp = params["timestamp"]
        .as_u64()
        .ok_or("Missing 'timestamp' field")?;

    let encrypted = BeaconCiphertext {
        ciphertext,
        nonce,
        timestamp,
    };

    // Try our own beacon first
    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(plaintext) = beacon.try_decrypt(&encrypted).map_err(|e| e.to_string())? {
        return Ok(json!({
            "decrypted": true,
            "plaintext": base64::encode(&plaintext),
            "matched_beacon_id": beacon.id().to_hex()
        }));
    }

    // Try all known beacons (from meetings)
    let known = beacon_manager.known_beacons.read().await;

    for (id_hex, known_beacon) in known.iter() {
        if let Some(plaintext) = known_beacon
            .try_decrypt(&encrypted)
            .map_err(|e| e.to_string())?
        {
            return Ok(json!({
                "decrypted": true,
                "plaintext": base64::encode(&plaintext),
                "matched_beacon_id": id_hex
            }));
        }
    }

    // None of our beacons could decrypt (TRUE Dark Forest!)
    Ok(json!({
        "decrypted": false
    }))
}

/// beacon.list_known - List known beacon IDs (from meetings)
pub async fn handle_beacon_list_known(
    beacon_manager: Arc<BeaconManager>,
    _params: Option<&Value>,
) -> Result<Value, String> {
    debug!("📋 RPC: beacon.list_known");

    let known = beacon_manager.known_beacons.read().await;

    let beacon_ids: Vec<String> = known.keys().cloned().collect();

    Ok(json!({
        "known_beacons": beacon_ids,
        "count": beacon_ids.len()
    }))
}

/// beacon.add_known - Add a known beacon (meeting exchange)
///
/// # Request
/// ```json
/// {
///   "beacon_seed_hex": "<hex 64 chars (32 bytes)>"
/// }
/// ```
pub async fn handle_beacon_add_known(
    beacon_manager: Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🤝 RPC: beacon.add_known");

    let params = params.ok_or("Missing params")?;
    let seed_hex = params["beacon_seed_hex"]
        .as_str()
        .ok_or("Missing 'beacon_seed_hex' field")?;

    let seed_bytes = hex::decode(seed_hex).map_err(|e| format!("Invalid hex: {}", e))?;

    if seed_bytes.len() != 32 {
        return Err(format!("Beacon seed must be 32 bytes, got {}", seed_bytes.len()));
    }

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&seed_bytes);

    // Create BeaconSeed from raw bytes (for meeting exchange)
    let beacon_id = BeaconSeed::derive_beacon_id_static(&seed);
    let beacon = BeaconSeed::from_raw_seed(seed, beacon_id);

    let id_hex = beacon.id().to_hex();

    beacon_manager
        .add_known_beacon(beacon)
        .await
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "beacon_id": id_hex,
        "status": "added"
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_beacon_generate() {
        let manager = Arc::new(BeaconManager::new());

        let result = handle_beacon_generate(manager.clone(), None)
            .await
            .expect("generate failed");

        assert!(result["beacon_id"].is_string());
        assert_eq!(result["status"], "generated");
    }

    #[tokio::test]
    async fn test_beacon_encrypt_decrypt_roundtrip() {
        let manager = Arc::new(BeaconManager::new());

        // Encrypt
        let plaintext = base64::encode(b"Dark Forest message");
        let encrypt_result = handle_beacon_encrypt(
            manager.clone(),
            Some(&json!({ "plaintext": plaintext })),
        )
        .await
        .expect("encrypt failed");

        // Decrypt
        let decrypt_params = json!({
            "ciphertext": encrypt_result["ciphertext"],
            "nonce": encrypt_result["nonce"],
            "timestamp": encrypt_result["timestamp"]
        });

        let decrypt_result = handle_beacon_try_decrypt(manager.clone(), Some(&decrypt_params))
            .await
            .expect("decrypt failed");

        assert_eq!(decrypt_result["decrypted"], true);
        assert_eq!(decrypt_result["plaintext"], plaintext);
    }

    #[tokio::test]
    async fn test_beacon_try_decrypt_any_finds_known() {
        let manager = Arc::new(BeaconManager::new());

        // Generate a second beacon (simulate meeting)
        let other_beacon = BeaconSeed::generate();
        let other_id = other_beacon.id().to_hex();

        // Encrypt with other beacon
        let plaintext = b"Message from other beacon";
        let encrypted = other_beacon.encrypt(plaintext).expect("encrypt failed");

        // Add other beacon to known list
        manager
            .add_known_beacon(other_beacon)
            .await
            .expect("add failed");

        // Try decrypt_any (should find it!)
        let decrypt_params = json!({
            "ciphertext": base64::encode(&encrypted.ciphertext),
            "nonce": base64::encode(&encrypted.nonce),
            "timestamp": encrypted.timestamp
        });

        let result = handle_beacon_try_decrypt_any(manager.clone(), Some(&decrypt_params))
            .await
            .expect("decrypt_any failed");

        assert_eq!(result["decrypted"], true);
        assert_eq!(result["matched_beacon_id"], other_id);
    }

    #[tokio::test]
    async fn test_beacon_list_known() {
        let manager = Arc::new(BeaconManager::new());

        // Add two known beacons
        manager
            .add_known_beacon(BeaconSeed::generate())
            .await
            .expect("add failed");
        manager
            .add_known_beacon(BeaconSeed::generate())
            .await
            .expect("add failed");

        let result = handle_beacon_list_known(manager.clone(), None)
            .await
            .expect("list failed");

        assert_eq!(result["count"], 2);
        assert_eq!(result["known_beacons"].as_array().unwrap().len(), 2);
    }
}
