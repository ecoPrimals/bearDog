// SPDX-License-Identifier: AGPL-3.0-only

//! Entropy seed metadata and HSM display types.

use serde::{Deserialize, Serialize};

/// Entropy seed metadata (saved to file)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntropySeedMetadata {
    /// Unique seed identifier (UUID)
    pub seed_id: String,
    /// User-selected quality tier (1–5)
    pub quality_tier: u8,
    /// Measured quality score (0.0–1.0)
    pub quality_score: f64,
    /// HSM or device label used during collection
    pub device_used: String,
    /// Tier label of the selected device (e.g. Hardware, Software)
    pub device_tier: String,
    /// Creation time (RFC 3339)
    pub timestamp: String,
    /// Whether interactive human entropy was used
    pub human_input: bool,
    /// Optional human identity string for sovereign seeds
    pub identity: Option<String>,
    /// Raw entropy bytes, standard Base64-encoded
    pub entropy_bytes_b64: String,
}

/// HSM information for CLI display
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HsmInfo {
    pub(crate) name: String,
    pub(crate) tier: String,
    pub(crate) hsm_type: String,
}
