// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software key representation and accessors.

use crate::tunnel::hsm::software_hsm::KeyMetadata;
use crate::tunnel::hsm::types::KeyType;
use chrono::{DateTime, Utc};

use super::protected_memory::ProtectedMemory;

/// Software key representation
#[derive(Debug, Clone)]
pub struct SoftwareKey {
    /// Unique key identifier
    pub id: String,
    /// Key type
    pub key_type: KeyType,
    /// Protected key material
    pub key_material: ProtectedMemory,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Key metadata
    pub metadata: KeyMetadata,
}

impl SoftwareKey {
    /// Create a new software key
    pub fn new(
        id: String,
        key_type: KeyType,
        key_material: ProtectedMemory,
        metadata: KeyMetadata,
    ) -> Self {
        Self {
            id,
            key_type,
            key_material,
            created_at: Utc::now(),
            metadata,
        }
    }

    /// Get key ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get key type
    pub const fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Get key material
    pub const fn key_material(&self) -> &ProtectedMemory {
        &self.key_material
    }

    /// Get metadata
    pub const fn metadata(&self) -> &KeyMetadata {
        &self.metadata
    }

    /// Get creation timestamp
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
