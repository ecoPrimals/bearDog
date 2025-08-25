// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # HSM Keys - Canonical
///
/// **UNIFIED HSM KEY TYPES** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// **CANONICAL** HSM Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    pub key_id: String,
    pub key_type: KeyType,
    pub material: KeyMaterial,
    pub metadata: KeyMetadata,
    pub health: KeyHealth,
}

impl Default for HsmKey {
    fn default() -> Self {
        Self {
            key_id: "default".to_string(),
            key_type: KeyType::default(),
            material: KeyMaterial::default(),
            metadata: KeyMetadata::default(),
            health: KeyHealth::default(),
        }
    }
}

/// **CANONICAL** HSM Key Storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyStorage {
    pub storage_id: String,
    pub key_material: KeyMaterial,
    pub created_at: SystemTime,
    pub access_count: u64,
    pub health_status: Option<KeyHealth>,
    pub encryption_info: Option<EncryptionInfo>,
    pub backup_info: Option<BackupInfo>,
    pub compliance_info: Option<ComplianceInfo>,
}

impl Default for HsmKeyStorage {
    fn default() -> Self {
        Self {
            storage_id: "default".to_string(),
            key_material: KeyMaterial::default(),
            created_at: SystemTime::now(),
            access_count: 0,
            health_status: None,
            encryption_info: None,
            backup_info: None,
            compliance_info: None,
        }
    }
}

/// **CANONICAL** Key Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    Symmetric,
    Asymmetric,
    Signing,
    Encryption,
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Symmetric
    }
}

/// **CANONICAL** Key Material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMaterial {
    pub algorithm: String,
    pub key_size: u32,
    pub key_data: Vec<u8>,
}

impl Default for KeyMaterial {
    fn default() -> Self {
        Self {
            algorithm: "AES-256".to_string(),
            key_size: 256,
            key_data: Vec::new(),
        }
    }
}

/// **CANONICAL** Key Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub usage_count: u64,
    pub tags: Vec<String>,
}

impl Default for KeyMetadata {
    fn default() -> Self {
        Self {
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            usage_count: 0,
            tags: Vec::new(),
        }
    }
}

/// **CANONICAL** Key Health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyHealth {
    pub status: String,
    pub last_check: SystemTime,
    pub error_count: u32,
}

impl Default for KeyHealth {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            last_check: SystemTime::now(),
            error_count: 0,
        }
    }
}

/// **CANONICAL** Encryption Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub cipher: String,
    pub mode: String,
    pub padding: String,
}

impl Default for EncryptionInfo {
    fn default() -> Self {
        Self {
            cipher: "AES".to_string(),
            mode: "GCM".to_string(),
            padding: "PKCS7".to_string(),
        }
    }
}

/// **CANONICAL** Backup Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub backup_id: String,
    pub backup_date: SystemTime,
    pub backup_location: String,
}

impl Default for BackupInfo {
    fn default() -> Self {
        Self {
            backup_id: "default".to_string(),
            backup_date: SystemTime::now(),
            backup_location: "local".to_string(),
        }
    }
}

/// **CANONICAL** Compliance Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub compliance_level: String,
    pub certifications: Vec<String>,
    pub audit_trail: Vec<String>,
}

impl Default for ComplianceInfo {
    fn default() -> Self {
        Self {
            compliance_level: "standard".to_string(),
            certifications: Vec::new(),
            audit_trail: Vec::new(),
        }
    }
}
