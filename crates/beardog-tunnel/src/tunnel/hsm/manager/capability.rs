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


/// # HSM Capability Detection
///
/// This module provides capability detection and recommendation for HSM providers,
/// including tier recommendation based on security requirements.

use super::{HsmCapabilityDetector, SecurityLevel, SecurityRequirements};
use crate::tunnel::hsm::types::HsmCapability;
use async_trait::async_trait;
use beardog_core::HsmTier; // Use the core HsmTier instead of local one
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
/// Default HSM capability detector
pub struct DefaultHsmCapabilityDetector {
    pub(crate) provider_capabilities: Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}
impl DefaultHsmCapabilityDetector {
    /// Create a new HSM capability detector with default settings
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::new())),
        })
    }

impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapability>> {
        // Return common capabilities that most HSMs support
        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
        ];
        Ok(capabilities)}


    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool> {
        match hsm_type {
            HsmTier::Software => Ok(true), // Software HSM always available
            HsmTier::Hardware => {
                // Check if hardware HSM is available
                Ok(self
                    .check_hardware_hsm_availability()
                    .await
                    .unwrap_or(false))
            }
            HsmTier::SmartCard => {
                // Check if smart card HSM is available
                Ok(self.check_smartcard_availability().await.unwrap_or(false))
            HsmTier::CloudHsm => {
                // Check if cloud HSM is available
                Ok(self.check_cloud_hsm_availability().await.unwrap_or(false))
        }
    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<HsmTier> {
        // Recommend HSM tier based on security requirements
        match requirements.security_level {
            SecurityLevel::Basic => {
                // Basic security - software HSM is sufficient
                Ok(HsmTier::Software)
            SecurityLevel::Medium => {
                // Medium security - smart card HSM recommended
                Ok(HsmTier::SmartCard)
            SecurityLevel::High => {
                // High security - hardware HSM recommended
                Ok(HsmTier::Hardware)
            SecurityLevel::Critical => {
                // Critical security - cloud HSM with hardware backing
                Ok(HsmTier::CloudHsm)
    /// Get cached provider capabilities}


    pub async fn get_provider_capabilities(
        provider_id: &str,
    ) -> BearDogResult<Vec<HsmCapability>> {
        let capabilities = self.provider_capabilities.read().await;
        Ok(capabilities.get(provider_id).cloned().unwrap_or_default())
    /// Update provider capabilities cache
    pub async fn update_provider_capabilities(
        provider_id: String,
        capabilities: Vec<HsmCapability>,
    ) -> BearDogResult<()> {
        let mut provider_capabilities = self.provider_capabilities.write().await;
        provider_capabilities.insert(provider_id, capabilities);
        Ok(())
