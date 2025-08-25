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


/// Configuration types for in-memory key management
///
/// **EXTRACTED FROM LARGE FILE** - Part of modularization effort

use chrono::Duration;
use serde::{Deserialize, Serialize};
/// Configuration for in-memory key management
#[derive(Debug, Clone)]
pub struct MemoryKeyConfig {
    /// Maximum number of keys to store in memory
    pub max_keys: usize,
    /// Key expiry duration (None = no expiry)
    pub key_expiry: Option<Duration>,
    /// Enable key derivation caching
    pub cache_derivations: bool,
    /// Enable automatic key rotation
    pub auto_rotation: bool,
    /// Rotation interval
    pub rotation_interval: Duration,
    /// Enable vault sharing
    pub enable_vault_sharing: bool,
    /// Maximum shared vaults
    pub max_shared_vaults: usize,
}
impl Default for MemoryKeyConfig {}


    fn default() -> Self {
        Self {
            max_keys: 10_000,
            key_expiry: Some(Duration::hours(24)),
            cache_derivations: true,
            auto_rotation: false,
            rotation_interval: Duration::hours(1),
            enable_vault_sharing: false,
            max_shared_vaults: 5,
        }
    }
/// Key storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStorageConfig {
    /// Memory protection level
    pub memory_protection: bool,
    /// Encryption at rest
    pub encrypt_at_rest: bool,
    /// Key backup enabled
    pub backup_enabled: bool,}


impl Default for KeyStorageConfig {
            memory_protection: true,
            encrypt_at_rest: true,
            backup_enabled: true,
} 
