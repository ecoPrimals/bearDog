// SPDX-License-Identifier: AGPL-3.0-only

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

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BeaconCiphertext, BeaconSeed};
use serde_json::{Value, json};
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
    beacon_manager: &Arc<BeaconManager>,
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
    beacon_manager: &Arc<BeaconManager>,
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
    beacon_manager: &Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔒 RPC: beacon.encrypt");

    let params = params.ok_or("Missing params")?;
    let plaintext_b64 = params["plaintext"]
        .as_str()
        .ok_or("Missing 'plaintext' field")?;

    let plaintext = BASE64
        .decode(plaintext_b64)
        .map_err(|e| format!("Invalid base64: {e}"))?;

    let beacon = beacon_manager
        .get_or_create_beacon()
        .await
        .map_err(|e| e.to_string())?;

    let encrypted = beacon.encrypt(&plaintext).map_err(|e| e.to_string())?;

    Ok(json!({
        "ciphertext": BASE64.encode(&encrypted.ciphertext),
        "nonce": BASE64.encode(encrypted.nonce),
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
    beacon_manager: &Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔓 RPC: beacon.try_decrypt");

    let params = params.ok_or("Missing params")?;

    let ciphertext = BASE64
        .decode(
            params["ciphertext"]
                .as_str()
                .ok_or("Missing 'ciphertext' field")?,
        )
        .map_err(|e| format!("Invalid base64: {e}"))?;

    let nonce_vec = BASE64
        .decode(params["nonce"].as_str().ok_or("Missing 'nonce' field")?)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

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
            "plaintext": BASE64.encode(&plaintext)
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
    beacon_manager: &Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🔍 RPC: beacon.try_decrypt_any");

    let params = params.ok_or("Missing params")?;

    let ciphertext = BASE64
        .decode(
            params["ciphertext"]
                .as_str()
                .ok_or("Missing 'ciphertext' field")?,
        )
        .map_err(|e| format!("Invalid base64: {e}"))?;

    let nonce_vec = BASE64
        .decode(params["nonce"].as_str().ok_or("Missing 'nonce' field")?)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

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
            "plaintext": BASE64.encode(&plaintext),
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
                "plaintext": BASE64.encode(&plaintext),
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
    beacon_manager: &Arc<BeaconManager>,
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
    beacon_manager: &Arc<BeaconManager>,
    params: Option<&Value>,
) -> Result<Value, String> {
    debug!("🤝 RPC: beacon.add_known");

    let params = params.ok_or("Missing params")?;
    let seed_hex = params["beacon_seed_hex"]
        .as_str()
        .ok_or("Missing 'beacon_seed_hex' field")?;

    let seed_bytes = hex::decode(seed_hex).map_err(|e| format!("Invalid hex: {e}"))?;

    if seed_bytes.len() != 32 {
        return Err(format!(
            "Beacon seed must be 32 bytes, got {}",
            seed_bytes.len()
        ));
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

// ============================================================================
// MethodHandler Implementation for HandlerRegistry Integration
// ============================================================================

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;

/// BeaconHandler wraps BeaconManager for HandlerRegistry integration
///
/// This handler exposes Dark Forest beacon methods via JSON-RPC.
pub struct BeaconHandler {
    beacon_manager: Arc<BeaconManager>,
}

impl BeaconHandler {
    /// Create new beacon handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            beacon_manager: Arc::new(BeaconManager::new()),
        }
    }

    /// Create beacon handler with existing manager
    #[must_use]
    pub const fn with_manager(manager: Arc<BeaconManager>) -> Self {
        Self {
            beacon_manager: manager,
        }
    }
}

impl Default for BeaconHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MethodHandler for BeaconHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "beacon.generate",
            "beacon.get_id",
            "beacon.encrypt",
            "beacon.try_decrypt",
            "beacon.try_decrypt_any",
            "beacon.list_known",
            "beacon.add_known",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        // Zero-copy: pass Arc by reference instead of cloning
        match method {
            "beacon.generate" => handle_beacon_generate(&self.beacon_manager, params).await,
            "beacon.get_id" => handle_beacon_get_id(&self.beacon_manager, params).await,
            "beacon.encrypt" => handle_beacon_encrypt(&self.beacon_manager, params).await,
            "beacon.try_decrypt" => handle_beacon_try_decrypt(&self.beacon_manager, params).await,
            "beacon.try_decrypt_any" => {
                handle_beacon_try_decrypt_any(&self.beacon_manager, params).await
            }
            "beacon.list_known" => handle_beacon_list_known(&self.beacon_manager, params).await,
            "beacon.add_known" => handle_beacon_add_known(&self.beacon_manager, params).await,
            _ => Err(format!("Unknown beacon method: {method}")),
        }
    }
}

#[cfg(test)]
#[path = "beacon_tests.rs"]
mod tests;
