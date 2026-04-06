// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM key management and specifications
//!
//! This module contains HSM key types and key management functionality.

use super::security::{SecurityLevel, KeyAlgorithm, KeyPurpose};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HSM key specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeySpec {
    /// Key algorithm
    pub algorithm: KeyAlgorithm,
    /// Key size in bits
    pub key_size: usize,
    /// Key purposes
    pub purposes: Vec<KeyPurpose>,
    /// Security level required
    pub security_level: SecurityLevel,
    /// Key alias/identifier
    pub alias: String,
    /// Additional key properties
    pub properties: HashMap<String, String>,
}

/// HSM key handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    /// Key specification
    pub spec: HsmKeySpec,
    /// Key identifier
    pub key_id: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Key status
    pub status: KeyStatus,
    /// Key metadata
    pub metadata: HashMap<String, String>,
}

/// Key status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStatus {
    /// Key is active and usable
    Active,
    /// Key is inactive but can be reactivated
    Inactive,
    /// Key is compromised and should not be used
    Compromised,
    /// Key is expired
    Expired,
    /// Key is revoked
    Revoked,
}

impl HsmKey {
    /// Create a new HSM key with the given specification
    pub fn new(spec: HsmKeySpec, key_id: String) -> Self {
        Self {
            spec,
            key_id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            status: KeyStatus::Active,
            metadata: HashMap::new(),
        }
    }

    /// Check if the key is usable
    pub fn is_usable(&self) -> bool {
        self.status == KeyStatus::Active
    }

    /// Get key algorithm
    pub fn algorithm(&self) -> &KeyAlgorithm {
        &self.spec.algorithm
    }

    /// Get key size
    pub fn key_size(&self) -> usize {
        self.spec.key_size
    }

    /// Check if key supports a specific purpose
    pub fn supports_purpose(&self, purpose: &KeyPurpose) -> bool {
        self.spec.purposes.contains(purpose)
    }
}

impl Default for HsmKeySpec {
    fn default() -> Self {
        Self {
            algorithm: KeyAlgorithm::Ec,
            key_size: 256,
            purposes: vec![KeyPurpose::Sign, KeyPurpose::Verify],
            security_level: SecurityLevel::HardwareBacked,
            alias: "default_key".to_string(),
            properties: HashMap::new(),
        }
    }
} 