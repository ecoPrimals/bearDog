// SPDX-License-Identifier: AGPL-3.0-only

//! JSON [`ExportedKey`] wire format for inter-primal key exchange.

use serde::{Deserialize, Serialize};

/// Exported key format for inter-primal sharing
/// This format is designed to be compatible with `ToadStool` and other primals
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

    /// Whether the `key_material` is encrypted
    #[serde(default)]
    pub encrypted: bool,

    /// Export format version (for future compatibility)
    #[serde(default = "default_version")]
    pub version: String,
}

pub(super) fn default_version() -> String {
    "1.0".to_string()
}
