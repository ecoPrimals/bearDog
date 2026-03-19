// SPDX-License-Identifier: AGPL-3.0-only

//! Solo V2 type definitions

use serde::{Deserialize, Serialize};

/// Solo V2 device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoloV2DeviceInfo {
    /// Device serial number or identifier
    pub device_id: String,
    /// Product name
    pub product_name: String,
    /// Firmware version
    pub firmware_version: String,
    /// Whether device is currently connected
    pub is_connected: bool,
    /// USB vendor ID
    pub vendor_id: u16,
    /// USB product ID
    pub product_id: u16,
}

/// Solo V2 key handle
///
/// References a key stored on the Solo V2 device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoloV2KeyHandle {
    /// Unique identifier for this key
    pub key_id: String,
    /// CTAP2 credential ID
    pub credential_id: Vec<u8>,
    /// Key type
    pub key_type: KeyType,
    /// Whether this is a resident key (stored on device)
    pub is_resident: bool,
    /// User ID associated with this key
    pub user_id: Option<Vec<u8>>,
    /// Public key bytes (for verification)
    pub public_key: Vec<u8>,
}

/// Key type supported by Solo V2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    /// ECDSA with P-256 curve (NIST P-256)
    EcdsaP256,
    /// EdDSA with Ed25519 curve
    Ed25519,
}

/// PIN configuration for Solo V2
#[derive(Debug, Clone)]
pub struct PinConfig {
    /// Whether PIN is required for operations
    pub pin_required: bool,
    /// Cached PIN (in memory only, never persisted)
    pub cached_pin: Option<String>,
    /// PIN retry attempts remaining
    pub retries_remaining: Option<u8>,
}

impl Default for PinConfig {
    fn default() -> Self {
        Self {
            pin_required: true,
            cached_pin: None,
            retries_remaining: None,
        }
    }
}

/// Solo V2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoloV2Config {
    /// Device to use (if multiple connected)
    pub device_id: Option<String>,
    /// Relying party ID for FIDO2 operations (typically domain name)
    pub relying_party_id: String,
    /// Whether to require user presence verification
    pub require_user_presence: bool,
    /// Whether to require user verification (PIN/biometric)
    pub require_user_verification: bool,
    /// Timeout for user interaction (seconds)
    pub user_interaction_timeout: u64,
}

impl Default for SoloV2Config {
    fn default() -> Self {
        Self {
            device_id: None,
            relying_party_id: "beardog.dev".to_string(), // Default RP ID
            require_user_presence: true,
            require_user_verification: true,
            user_interaction_timeout: 30,
        }
    }
}
