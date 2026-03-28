// SPDX-License-Identifier: AGPL-3.0-only

/// HSM (Hardware Security Module) types and configurations
//
// This module provides functionality for the BearDog ecosystem.
/// Android HSM module
pub mod android;
/// Capabilities module
pub mod capabilities;
/// Config module
/// Configuration management
pub mod config;
/// Discovery configuration (HSM-specific)
pub mod discovery;
/// Keys module
pub mod keys;
/// Platform-specific HSM classifications (device types, algorithms, storage backends).
pub mod platform_types;
/// Status module
pub mod status;
/// Tiers module
pub mod tiers;

pub use capabilities::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, HsmCapabilities,
    KeyGenerationCapabilities, KeyManagementCapabilities, SecurityCapabilities,
};
pub use config::*;
pub use keys::*;
pub use platform_types::{
    AndroidKeyAlgorithm, EntropyCollectionMethod, EntropyQualityRating, HsmType, KeyStorageType,
    MemoryProtectionLevel, PerformanceMetrics, SecureEnclaveType, SmartphoneType, SoftwareHsmType,
    StrongBoxImplementation,
};
pub use status::{HealthMetrics, HsmHealth, HsmHealthStatus};
pub use tiers::{AttestationLevel, HsmSecurityTier, TamperResistanceLevel};
