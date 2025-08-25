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


/// # Canonical HSM Types
///
/// **SINGLE SOURCE OF TRUTH** for all HSM and key management types in the `BearDog` ecosystem.
/// This module consolidates HSM key types, metadata, and operations into canonical definitions.
pub mod android;
pub mod capabilities;
pub mod config;
pub mod keys;
pub mod platform_types;
pub mod status;
pub mod tiers;

// Re-export commonly used types (specific to avoid conflicts)
pub use capabilities::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, HsmCapabilities,
    KeyGenerationCapabilities, KeyManagementCapabilities, SecurityCapabilities,
};
pub use config::*;
pub use keys::*;
pub use status::{HealthMetrics, HsmHealth, HsmHealthStatus};
pub use tiers::{AttestationLevel, HsmSecurityTier, TamperResistanceLevel};
pub use platform_types::{
    AndroidKeyAlgorithm, EntropyCollectionMethod, EntropyQualityRating, HsmType,
    KeyStorageType, MemoryProtectionLevel, PerformanceMetrics, SecureEnclaveType,
    SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};
