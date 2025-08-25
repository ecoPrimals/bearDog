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


/// # Clean HSM Foundation
///
/// This module provides a unified, agnostic foundation for all HSM operations,
/// eliminating the architectural debt in the previous implementation.
/// ## Core Principles:
/// - Single source of truth for all types
/// - Consistent error handling
/// - Clean trait boundaries
/// - Zero unsafe code
/// - Platform agnostic design

pub mod error;
pub mod providers;
pub mod traits;
pub mod types;
// Re-export core types for easy access
pub use error::*;
pub use providers::*;
pub use traits::*;
pub use types::*;
/// HSM Foundation version for compatibility tracking
// Use canonical version constants
pub use beardog_types::constants::api::versions::HSM_FOUNDATION_VERSION;
/// Core HSM capabilities that all providers must support
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CoreCapabilities {
    /// Can generate cryptographic keys
    pub key_generation: bool,
    /// Can sign data with stored keys
    pub signing: bool,
    /// Can encrypt/decrypt data
    pub encryption: bool,
    /// Supports key derivation functions
    pub key_derivation: bool,
    /// Hardware-backed security available
    pub hardware_backed: bool,
    /// Supports attestation
    pub attestation: bool,
}
impl Default for CoreCapabilities {}


    fn default() -> Self {
        Self {
            key_generation: true,
            signing: true,
            encryption: true,
            key_derivation: false,
            hardware_backed: false,
            attestation: false,
        }
    }
/// HSM Foundation health check
#[derive(Debug, Clone)]
pub struct FoundationHealth {
    /// Overall health status
    pub is_healthy: bool,
    /// Available providers count
    pub provider_count: usize,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Any critical issues
    pub issues: Vec<String>,}


impl FoundationHealth {}


    #[must_use] pub fn healthy() -> Self {
            is_healthy: true,
            provider_count: 0,
            last_check: chrono::Utc::now(),
            issues: Vec::new(),
    #[must_use] pub fn unhealthy(reason: String) -> Self {
            is_healthy: false,
            issues: vec![reason],
