// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Types Module
//!
//! Core type definitions for Hardware Security Module (HSM) functionality.

pub mod algorithm;
pub mod android;
pub mod canonical;
pub mod capability;
pub mod config;
pub mod ios;
pub mod key;
pub mod ops;
pub mod security_level;
pub mod status;
pub mod tier;

pub use algorithm::Algorithm;
pub use android::{
    AndroidAttestationService, AndroidDeviceCapabilities, AndroidHealthMonitor, AndroidHsmConfig,
    AndroidKeyParams, AndroidKeyPurpose, AndroidKeystore, AndroidKeystoreConfig,
    AttestationTransport, HealthMetricsTransport, KeystoreTransport, StubAttestationTransport,
    StubHealthMetricsTransport, StubKeystoreTransport,
};

#[cfg(target_os = "android")]
pub use android::AndroidJniHealthMetricsTransport;
pub use capability::CapabilityRequirements;
pub use capability::HsmCapabilities;
pub use capability::HsmCapability;
pub use config::SoftwareHsmConfig;
pub use config::{AuthMethod as AuthenticationMethod, HsmConnectionConfig as HsmConnectionInfo};
pub use ios::{IOSDeviceCapabilities, IOSHsmConfig};
pub use key::HsmKeyMetadata as HsmKeyMeta;
pub use key::KeyType;
pub use key::{
    HsmKey, HsmKeyInfo, HsmKeyMetadata, KeyAttestation, KeyHealthStatus, KeyMaterial, KeyMetadata,
    KeyType as HsmKeyType, UniversalKey,
};
pub use ops::{HsmAuditEntry, HsmCache, HsmOperation, HsmOperationResult};
pub use security_level::SecurityLevel;
pub use status::{HsmHealthStatus, PerformanceMetrics};
pub use tier::{
    AndroidKeyAlgorithm, AttestationLevel, HsmTier, KeyStorageType, MemoryProtectionLevel,
    SecureEnclaveType, SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;
