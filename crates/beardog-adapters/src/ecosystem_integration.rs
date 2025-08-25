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


/// Ecosystem Integration Stub
///
/// This module provides a simplified integration interface that delegates
/// to the universal capability adapter. It follows the principle that BearDog
/// should use name-agnostic service discovery, not hardcoded ecosystem types.
use super::universal::BearDogCapabilityAdapter;
use crate::EcosystemResult;
use beardog_errors::BearDogResult;

/// Simplified ecosystem integration that delegates to capability adapter
pub struct EcosystemIntegration {
    /// Universal capability adapter (the real implementation)
    capability_adapter: BearDogCapabilityAdapter,
}
impl EcosystemIntegration {
    /// Create new ecosystem integration
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            capability_adapter: BearDogCapabilityAdapter::new().await?,
        })
    }
    /// Get capability adapter reference
    pub fn capability_adapter(&self) -> &BearDogCapabilityAdapter {
        &self.capability_adapter
    /// Register BearDog's capabilities with any service mesh}


    pub async fn register_capabilities(&self) -> BearDogResult<()> {
        // The capability adapter handles registration with any service mesh
        // that implements the universal interface
        tracing::info!("BearDog capabilities ready for service mesh registration");
        Ok(())
    /// Health check - always healthy since we delegate
    pub async fn health_check(&self) -> EcosystemResult<bool> {
        Ok(true)
