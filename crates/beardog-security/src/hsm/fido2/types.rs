// SPDX-License-Identifier: AGPL-3.0-only

//! FIDO2 HSM Type Definitions

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// FIDO2 transport type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fido2Transport {
    /// USB HID transport
    Usb,
    /// NFC transport
    Nfc,
    /// Bluetooth transport
    Bluetooth,
    /// Internal platform authenticator
    Internal,
}

/// FIDO2 device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2Capabilities {
    /// Supports resident keys (credential storage on device)
    pub resident_keys: bool,

    /// Supports user presence check (button press)
    pub user_presence: bool,

    /// Supports user verification (PIN, biometric)
    pub user_verification: bool,

    /// Supports hmac-secret extension (entropy generation)
    pub hmac_secret: bool,

    /// Supports credProtect extension
    pub cred_protect: bool,

    /// Maximum message size
    pub max_msg_size: usize,

    /// Maximum credential count in list
    pub max_cred_count: usize,

    /// Supported algorithms (COSE algorithm identifiers)
    pub algorithms: Vec<i32>,

    /// PIN protocol versions supported
    pub pin_protocols: Vec<u8>,

    /// Maximum number of resident keys that can be stored (None = unknown/unlimited)
    pub max_resident_keys: Option<usize>,

    /// Maximum entropy bytes per request (None = unlimited)
    pub max_entropy_size: Option<usize>,

    /// Human-readable algorithm names
    pub supported_algorithms: Vec<String>,
}

/// FIDO2 device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2DeviceInfo {
    /// Device path (e.g., /dev/hidraw5)
    pub device_path: PathBuf,

    /// Vendor ID
    pub vendor_id: u16,

    /// Product ID
    pub product_id: u16,

    /// Manufacturer string
    pub manufacturer: String,

    /// Product string
    pub product: String,

    /// Serial number (if available)
    pub serial: Option<String>,

    /// AAGUID (Authenticator Attestation GUID)
    pub aaguid: Option<[u8; 16]>,

    /// Firmware version
    pub firmware_version: Option<String>,

    /// Protocol version (e.g., "FIDO_2_0", "FIDO_2_1_PRE")
    pub protocol_versions: Vec<String>,

    /// Extension strings
    pub extensions: Vec<String>,

    /// Transport type
    pub transport: Fido2Transport,

    /// Device capabilities
    pub capabilities: Fido2Capabilities,
}

/// FIDO2 key handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2KeyHandle {
    /// Credential ID
    pub credential_id: Vec<u8>,

    /// Public key (COSE format)
    pub public_key: Vec<u8>,

    /// Algorithm (COSE identifier)
    pub algorithm: i32,

    /// Associated relying party ID
    pub rp_id: String,

    /// User ID
    pub user_id: Vec<u8>,

    /// Resident key flag
    pub is_resident: bool,
}

/// Human presence proof from FIDO2 device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2PresenceProof {
    /// User presence flag (button was pressed)
    pub user_present: bool,

    /// User verification flag (PIN/biometric used)
    pub user_verified: bool,

    /// Timestamp of the interaction
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Counter value from authenticator
    pub counter: u32,
}

/// FIDO2 attestation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2AttestationData {
    /// Attestation format (e.g., "packed", "fido-u2f", "none")
    pub format: String,

    /// Attestation statement (format-specific)
    pub statement: Vec<u8>,

    /// Authenticator data
    pub authenticator_data: Vec<u8>,

    /// AAGUID from attestation
    pub aaguid: [u8; 16],

    /// Attestation certificates (if any)
    pub certificates: Vec<Vec<u8>>,
}

impl Default for Fido2Capabilities {
    fn default() -> Self {
        Self {
            resident_keys: false,
            user_presence: true, // All FIDO2 devices support this
            user_verification: false,
            hmac_secret: false,
            cred_protect: false,
            max_msg_size: 1024,
            max_cred_count: 1,
            algorithms: vec![-7], // ES256 (COSE -7)
            pin_protocols: vec![1],
            max_resident_keys: Some(50), // Typical for most FIDO2 devices
            max_entropy_size: Some(64),  // Common limit for hmac-secret
            supported_algorithms: vec!["ES256".to_string()], // ES256 is mandatory for FIDO2
        }
    }
}
