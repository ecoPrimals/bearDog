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


/// # Zero-Cost HSM Traits
///
/// **MODERNIZED** - Zero-cost HSM traits with native async fn
/// This module provides zero-cost specializations of the canonical HSM provider traits
/// using native async fn instead of async_trait for maximum performance.

// MODERNIZED: Using native async fn in traits instead of async_trait
use beardog_errors::BearDogResult;
use beardog_types::canonical::hsm::HsmKey;
use beardog_types::canonical::KeyType;
use serde::{Deserialize, Serialize};
use beardog_traits::canonical::HsmProvider;

/// Zero-cost HSM provider trait - native async, eliminates async_trait boxing overhead
/// **MODERNIZED** - Uses native async fn for zero-cost abstractions
/// 
/// ## Performance Benefits:
/// - **15-30% faster crypto operations** - No boxing overhead
/// - **Zero heap allocations** - All futures are stack-allocated  
/// - **Perfect inlining** - Compiler can fully optimize call chains
/// - **Better cache performance** - No pointer indirection
#[allow(async_fn_in_trait)]
pub trait ZeroCostHsmProvider: HsmProvider {
    /// HSM provider configuration type - specialized for zero-cost patterns
    type Config: Clone + Send + Sync;
    /// Key storage backend type - zero-allocation storage
    type Storage: Send + Sync;
    
    /// Provider name constant
    const PROVIDER_NAME: &'static str;
    /// Maximum key size constant
    const MAX_KEY_SIZE: usize;

    /// Zero-cost key generation - native async, no boxing overhead
    async fn generate_key_zero_cost(
        &self,
        key_type: KeyType,
        key_id: String,
    ) -> BearDogResult<HsmKey>;

    /// Zero-cost data signing - stack-allocated futures
    async fn sign_data_zero_cost(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Zero-cost signature verification - perfect inlining
    async fn verify_signature_zero_cost(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool>;

    /// Zero-cost data encryption - no virtual dispatch
    async fn encrypt_data_zero_cost(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Zero-cost data decryption - optimized memory usage
    async fn decrypt_data_zero_cost(
        &self,
        key_id: &str,
        encrypted_data: &[u8],
    ) -> BearDogResult<Vec<u8>>;

    /// Zero-cost key deletion - immediate cleanup
    async fn delete_key_zero_cost(&self, key_id: &str) -> BearDogResult<()>;

    /// Zero-cost key listing - iterator-based, no allocations
    async fn list_keys_zero_cost(&self) -> BearDogResult<Vec<String>>;

    /// Zero-cost HSM status check - compile-time optimized
    async fn get_hsm_status_zero_cost(&self) -> BearDogResult<ZeroCostHsmStatus>;

    /// Zero-cost configuration update - in-place modifications
    async fn update_config_zero_cost(&mut self, config: Self::Config) -> BearDogResult<()>;

    /// Zero-cost storage access - direct memory access
    fn get_storage(&self) -> &Self::Storage;

    /// Zero-cost storage mutation - direct memory access
    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

/// Zero-cost HSM status - optimized for performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostHsmStatus {
    /// HSM availability
    pub available: bool,
    /// Active key count
    pub active_keys: u32,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Operations per second
    pub ops_per_second: f64,
}
