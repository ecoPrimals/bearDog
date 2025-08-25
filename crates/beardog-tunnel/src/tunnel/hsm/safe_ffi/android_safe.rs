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


/// Safe Android StrongBox Provider
///
/// This module provides safe Rust interfaces for Android StrongBox operations,
/// eliminating direct unsafe FFI calls.

use super::traits::SafeHardwareProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{info, warn};
/// Safe Android StrongBox provider
pub struct SafeAndroidProvider {
    /// Whether StrongBox is available
    strongbox_available: bool,
}
impl SafeAndroidProvider {
    /// Create new safe Android provider}


    pub fn new() -> BearDogResult<Self> {
        let strongbox_available = Self::check_strongbox_availability();
        Ok(Self {
            strongbox_available,
        })
    }
    /// Check if StrongBox is available without unsafe code
    fn check_strongbox_availability() -> bool {
        if !cfg!(target_os = "android") {
            info!("Not on Android platform, StrongBox not available");
            return false;
        }
        // Placeholder for actual safe Android API check
        info!("Simulating StrongBox availability check on Android");
        true
    /// Generate a key using safe Android operations
    async fn generate_key_safe(&self, _key_id: &str, _key_type: &KeyType) -> BearDogResult<HsmKey> {
        // Placeholder for safe key generation
        Err(BearDogError::NotImplemented {
            message: "Android StrongBox key generation not yet implemented safely".to_string(),
    /// Sign data using safe Android operations}


    async fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Placeholder for safe signing
            message: "Android StrongBox signing not yet implemented safely".to_string(),
    /// Verify signature using safe Android operations
    async fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> BearDogResult<bool> {
        // Placeholder for safe verification
            message: "Android StrongBox verification not yet implemented safely".to_string(),}



impl SafeHardwareProvider for SafeAndroidProvider {
    async fn generate_key(&self, key_id: &str, key_type: &KeyType) -> BearDogResult<HsmKey> {
        self.generate_key_safe(key_id, key_type).await}


    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.sign_data_safe(key_id, data).await
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        self.verify_signature_safe(key_id, data, signature).await}


    fn is_hardware_backed(&self) -> bool {
        self.strongbox_available
